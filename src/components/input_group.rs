//! `InputGroup`, `InputGroupAddon`, `InputGroupButton` and
//! `InputGroupPasswordToggle`: icons, words, key caps, spinners and
//! buttons around a text field, inside one field look. The group draws
//! the frame (foundation's field look at rest, its focus while the field
//! has it) and provides a context the library's `Input` and `Textarea`
//! read: inside a group they draw no frame of their own, take its size,
//! and report their focus, `invalid` and `disabled` to it. API:
//! `docs/components/input-group.md`.

use std::rc::Rc;

use nsh_base::{button as base_button, toggle as base_toggle};
use nsh_core::{
    element::{Style, TextPartStyle},
    prelude::*,
};

use super::{
    button::{
        BUTTON_BORDER_FOCUS, BUTTON_GHOST_HOVER_FOREGROUND, BUTTON_OUTLINE_BORDER, ButtonVariant,
        hover_fill, hover_foreground, rest_fill, rest_foreground, ring,
    },
    input::{font_size_token, height_token, padding_token, vertical_hit_slop},
};
use crate::{
    foundation::{
        class::Classes,
        field::{
            self, INPUT_BORDER_WIDTH, field_button_radius, field_button_side, field_focus,
            field_rest,
        },
        focus::focus_ring,
        icon::{IconName, icon_size_token, icon_view, provide_icon_size, provide_text_color},
        labels::{self, Label},
        motion,
        press::{BUTTON_SCALE, press},
        size::Size,
        typography::{self, line_height, medium},
    },
    theme::{
        Theme,
        seeds::SPACING,
        semantic::{BORDER_WIDTH, MUTED_FOREGROUND, OPACITY_DISABLED},
        theme,
    },
    tokens,
};

tokens! {
    /// An addon's words and icons, and a ghost button's at rest.
    INPUT_GROUP_ADDON_FOREGROUND: Color = "input-group-addon-foreground" => |t| t.color(MUTED_FOREGROUND),
    /// Between an addon's children, and between the field's text and an
    /// addon beside it (`md` and `lg`; `xs` and `sm` take three quarters).
    INPUT_GROUP_GAP: Length = "input-group-gap" => |t| t.length(SPACING) * 2.0,
}

/// Which edge of the group an addon sits at.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, PropEnum)]
pub enum InputGroupAlign {
    /// Before the field, in its row (write it before the field).
    #[default]
    InlineStart,
    /// After the field, in its row (write it after the field).
    InlineEnd,
    /// A row over the field: the group becomes a column.
    BlockStart,
    /// A row under the field (a toolbar): the group becomes a column.
    BlockEnd,
}

impl InputGroupAlign {
    fn block(self) -> bool {
        matches!(self, Self::BlockStart | Self::BlockEnd)
    }
}

nsh_core::static_reactive!(InputGroupAlign: InputGroupAlign);

/// What a group gives what is inside it, and what its field tells it.
#[derive(Clone, Copy)]
pub(crate) struct Group {
    size: Memo<Size>,
    blocks: Signal<u32>,
    control: Signal<Option<ElementRef>>,
    focused: Signal<bool>,
    invalid: Signal<bool>,
    disabled: Signal<bool>,
    shown: Signal<bool>,
}

impl Group {
    /// The group's size, which its field, addons and buttons take.
    pub(crate) fn size(&self) -> Size {
        self.size.get()
    }

    /// A block addon is in it: a column (header, field, footer).
    pub(crate) fn column(&self) -> bool {
        self.blocks.get() > 0
    }

    /// Its field is disabled: buttons take no presses.
    pub(crate) fn disabled(&self) -> bool {
        self.disabled.get()
    }

    /// The password toggle shows the password.
    pub(crate) fn shown(&self) -> bool {
        self.shown.get()
    }

    /// Makes the field at `element` the group's field while the current
    /// owner lives, unless the group has one: its `invalid` and `disabled`
    /// draw the frame. Whether it became the group's field.
    pub(crate) fn join(
        &self,
        element: ElementRef,
        invalid: impl Fn() -> bool + 'static,
        disabled: impl Fn() -> bool + 'static,
    ) -> bool {
        if self.control.get_untracked().is_some() {
            return false;
        }
        let g = *self;
        g.control.set(Some(element));
        effect(move || {
            let on = invalid();
            if g.invalid.get_untracked() != on {
                g.invalid.set(on);
            }
        });
        effect(move || {
            let on = disabled();
            if g.disabled.get_untracked() != on {
                g.disabled.set(on);
            }
        });
        cleanup(move || {
            // The group may go first (its owner disposed).
            if untracked(|| g.control.try_get()) == Some(Some(element)) {
                g.control.try_set(None);
                g.focused.try_set(false);
                g.invalid.try_set(false);
                g.disabled.try_set(false);
            }
        });
        true
    }

    /// Its field took (`true`) or lost the keyboard focus.
    pub(crate) fn set_focused(&self, focused: bool) {
        if self.focused.get_untracked() != focused {
            self.focused.set(focused);
        }
    }

    /// Moves the keyboard focus into its field (a press on an addon),
    /// unless the field is disabled.
    fn focus_field(&self) {
        if self.disabled.get_untracked() {
            return;
        }
        if let Some(field) = self.control.get_untracked() {
            field.focus();
        }
    }
}

/// The group the caller is made in, if any.
pub(crate) fn current() -> Option<Group> {
    use_context::<Option<Group>>().flatten()
}

/// The addon a button is made in: its edge.
#[derive(Clone, Copy)]
struct InAddon(InputGroupAlign);

/// The props of [`InputGroup`].
#[derive(Debug, Clone, PartialEq, Props)]
pub struct InputGroupProps {
    /// Its size (`xs`, `sm`, `md`, `lg`): the field's height, text and
    /// padding, and its addons', icons' and buttons' sizes, together. The
    /// field's own `size` is not read inside a group.
    #[prop(default)]
    pub size: Prop<Size>,
    /// The field (the library's `Input` or `Textarea`) and its addons, in
    /// the order they show.
    pub children: Prop<Children>,
}

/// A text field with addons: the field look (edge, fill, shadow, the
/// focus ring while the field has the focus, invalid and disabled from
/// the field) around this library's `Input` or `Textarea` and the
/// `InputGroupAddon`s beside, over or under it.
#[component]
pub fn InputGroup(props: InputGroupProps, _cx: Scope<NoEvent>) -> impl IntoElement {
    let t = theme();
    let group = Group {
        size: computed(move || props.size.get()),
        blocks: signal(0),
        control: signal(None),
        focused: signal(false),
        invalid: signal(false),
        disabled: signal(false),
        shown: signal(false),
    };
    provide_context(Some(group));
    let invalid = move || group.invalid.get();
    let focused = move || group.focused.get();
    let column = move || group.column();
    // Beside a label in a horizontal field: an equal share of the row, as
    // a field alone takes.
    let field_row = field::current();
    let el = view()
        .flex_row()
        .style_when(column, |s| s.flex_col())
        .items_center()
        .style_when(column, |s| s.items_stretch())
        .w_full()
        .min_w(px(0.0))
        .style_when(
            move || field_row.is_some_and(|f| f.in_row()),
            |s| s.flex_1(),
        )
        .min_h(move || t.length(height_token(group.size())))
        .cursor(Cursor::Default)
        .opacity(move || {
            if group.disabled.get() {
                t.number(OPACITY_DISABLED)
            } else {
                1.0
            }
        });
    let el = field_rest(el, t, invalid).style_when(focused, move |s| field_focus(s, t, invalid));
    motion::transition_all(el, t).child(props.children)
}

/// The props of [`InputGroupAddon`].
#[derive(Debug, Clone, PartialEq, Props)]
pub struct InputGroupAddonProps {
    /// Which edge it sits at: `inlineStart` (before the field), `inlineEnd`
    /// (after it), `blockStart` (a row over it), `blockEnd` (a row under
    /// it). Write it where it shows: before or after the field.
    #[prop(default)]
    pub align: Prop<InputGroupAlign>,
    /// Its content: words, an `Icon`, a `Kbd`, a `Spinner`,
    /// `InputGroupButton`s.
    pub children: Prop<Children>,
}

/// Icons, words, key caps, a spinner or buttons at one edge of an
/// `InputGroup`, muted, at the group's text and icon size. A press on it
/// (not on a button in it) moves the focus into the field.
#[component]
pub fn InputGroupAddon(props: InputGroupAddonProps, _cx: Scope<NoEvent>) -> impl IntoElement {
    let t = theme();
    let group = current();
    let size = move || group.map_or(Size::Md, |g| g.size());
    let align = props.align.get_untracked();
    // A block addon makes the group a column while it is mounted.
    if align.block()
        && let Some(g) = group
    {
        g.blocks.update(|n| *n += 1);
        cleanup(move || {
            if let Some(n) = untracked(|| g.blocks.try_get()) {
                g.blocks.try_set(n.saturating_sub(1));
            }
        });
    }
    provide_icon_size(size);
    provide_text_color();
    provide_context(InAddon(align));
    let pad = move || t.length(padding_token(size()));
    let gap = move || gap(t, size());
    let font_size = move || t.length(font_size_token(size()));
    let (start, end, top, bottom) = match align {
        InputGroupAlign::InlineStart => (true, false, false, false),
        InputGroupAlign::InlineEnd => (false, true, false, false),
        InputGroupAlign::BlockStart => (true, true, true, false),
        InputGroupAlign::BlockEnd => (true, true, false, true),
    };
    view()
        // Its words and buttons are read one by one; it is none itself.
        .accessible(false)
        .focusable(false)
        .on_press(move || {
            if let Some(g) = group {
                g.focus_field();
            }
        })
        .flex_row()
        .items_center()
        .flex_shrink_0()
        .gap(gap)
        .style_when(align.block(), |s| s.w_full())
        .ps(move || if start { pad() } else { px(0.0) })
        .pe(move || if end { pad() } else { px(0.0) })
        .pt(move || if top { gap() } else { px(0.0) })
        .pb(move || if bottom { gap() } else { px(0.0) })
        .text_color(move || t.color(INPUT_GROUP_ADDON_FOREGROUND))
        .text_size(font_size)
        .line_height(move || line_height(font_size()))
        .whitespace_nowrap()
        .select_none()
        // A press here edits the field: the I-beam, as over the field.
        .cursor(Cursor::Text)
        .child(props.children)
}

/// The gap of a group of `size`: `input-group-gap`, three quarters of it
/// at `xs` and `sm`.
fn gap(t: Theme, size: Size) -> Points {
    t.length(INPUT_GROUP_GAP) * size.pick(0.75, 0.75, 1.0, 1.0)
}

/// The class that moves a button at the group's edge out toward it: to 4
/// from the frame's inner edge in an inline addon, to the gap in a block
/// one. Only the addon's first child (at the start) or last (at the end)
/// moves, by NSH's `first:` / `last:` variants.
fn edge_class(t: Theme, size: Size, addon: Option<InAddon>) -> SharedString {
    let pad = t.length(padding_token(size));
    let c = Classes::new();
    let c = match addon {
        Some(InAddon(InputGroupAlign::InlineStart)) => {
            c.length("first", "ms", px(0.0) - (pad - px(4.0)).max(px(0.0)))
        }
        Some(InAddon(InputGroupAlign::InlineEnd)) => {
            c.length("last", "me", px(0.0) - (pad - px(4.0)).max(px(0.0)))
        }
        Some(InAddon(InputGroupAlign::BlockStart | InputGroupAlign::BlockEnd)) => {
            let pull = px(0.0) - (pad - gap(t, size)).max(px(0.0));
            c.length("first", "ms", pull).length("last", "me", pull)
        }
        None => c,
    };
    c.build()
}

/// What a group's button draws with.
#[derive(Clone, Copy)]
struct Look {
    t: Theme,
    size: Memo<Size>,
    variant: Memo<ButtonVariant>,
    icon_only: Memo<bool>,
    /// Its own `disabled`, which dims it (the group's dims the group).
    own_disabled: Memo<bool>,
    /// Its own or the group's `disabled`: no hover or press feedback.
    idle: Memo<bool>,
}

/// A group's button: a square or a short row the field's height less 8,
/// the frame's corners inset, `variant`'s colors (a ghost one muted at
/// rest), the hover and press of `Button`, its focus ring.
fn button_look<E: Styled + ComponentBuilder>(el: E, look: Look) -> E {
    let Look {
        t,
        size,
        variant,
        icon_only,
        own_disabled,
        idle,
    } = look;
    let side = move || field_button_side(t, size.get());
    let font_size = move || t.length(typography::font_size_token(size.get()));
    let rest = move || {
        if variant.get() == ButtonVariant::Ghost {
            t.color(INPUT_GROUP_ADDON_FOREGROUND)
        } else {
            rest_foreground(t, variant.get())
        }
    };
    let hover_text = move || {
        if variant.get() == ButtonVariant::Ghost {
            t.color(BUTTON_GHOST_HOVER_FOREGROUND)
        } else {
            hover_foreground(t, variant.get())
        }
    };
    let el = el
        .flex_row()
        .items_center()
        .justify_center()
        .flex_shrink_0()
        .gap(move || t.length(SPACING) * size.get().pick(1.0, 1.0, 1.5, 1.5))
        .h(side)
        .min_w(side)
        .px(move || {
            if icon_only.get() {
                px(0.0)
            } else {
                t.length(SPACING) * size.get().pick(1.5, 1.5, 2.0, 2.0)
            }
        })
        .rounded(move || field_button_radius(t))
        .border(move || {
            if variant.get() == ButtonVariant::Outline {
                t.length(BORDER_WIDTH)
            } else {
                px(0.0)
            }
        })
        .border_color(move || t.color(BUTTON_OUTLINE_BORDER))
        .bg(move || rest_fill(t, variant.get()))
        .text_color(rest)
        .text_size(font_size)
        .line_height(move || line_height(font_size()))
        .font_weight(medium(t))
        .whitespace_nowrap()
        .select_none()
        .cursor(Cursor::Default)
        .opacity(move || {
            if own_disabled.get() {
                t.number(OPACITY_DISABLED)
            } else {
                1.0
            }
        })
        .hover(move |s| {
            s.bg(move || {
                if idle.get() {
                    rest_fill(t, variant.get())
                } else {
                    hover_fill(t, variant.get())
                }
            })
            .text_color(move || if idle.get() { rest() } else { hover_text() })
        })
        .active(move |s| {
            press(
                s.bg(move || {
                    if idle.get() {
                        rest_fill(t, variant.get())
                    } else {
                        hover_fill(t, variant.get())
                    }
                }),
                BUTTON_SCALE,
                move || idle.get(),
            )
        })
        .focus_visible(move |s| {
            focus_ring(
                s,
                t,
                move || ring(t, variant.get(), false),
                Some(BUTTON_BORDER_FOCUS),
            )
        });
    motion::transition_all(el, t)
}

/// The props of [`InputGroupButton`].
#[derive(Debug, Clone, PartialEq, Props)]
pub struct InputGroupButtonProps {
    /// Its look, as `Button`'s; `ghost` (the default) takes the addon's
    /// muted color at rest.
    #[prop(default = ButtonVariant::Ghost)]
    pub variant: Prop<ButtonVariant>,
    /// A square showing its icon alone; give it an `accessibilityLabel`.
    #[prop(default)]
    pub icon_only: Prop<bool>,
    /// An icon before the content.
    pub icon: Prop<Option<IconName>>,
    /// No presses, at half opacity. A disabled field disables it too.
    #[prop(default)]
    pub disabled: Prop<bool>,
    /// What screen readers say (an icon-only button's name).
    pub accessibility_label: Prop<Option<SharedString>>,
    /// What pressing it does, for screen readers.
    pub accessibility_hint: Prop<Option<SharedString>>,
    /// Tailwind classes of the text part around each child.
    pub label_class: Prop<Option<SharedString>>,
    /// The inline style of the text part around each child.
    #[prop(default)]
    pub label_style: Prop<TextPartStyle>,
    /// Tailwind classes of the icon.
    pub icon_class: Prop<Option<SharedString>>,
    /// The inline style of the icon.
    #[prop(default)]
    pub icon_style: Prop<Style>,
    /// Its label ("Copy").
    pub children: Prop<Children>,
}

/// The events of an [`InputGroupButton`].
#[derive(Debug, Clone, PartialEq, Event)]
pub enum InputGroupButtonEvent {
    /// It was pressed (never while disabled).
    Press,
}

/// A small button in an `InputGroupAddon`: as tall as the field less 8,
/// concentric with the frame, ghost by default. A press on it leaves the
/// keyboard focus in the field; Tab reaches it after the field.
#[component]
pub fn InputGroupButton(
    props: InputGroupButtonProps,
    cx: Scope<InputGroupButtonEvent>,
) -> impl IntoElement {
    let t = theme();
    let group = current();
    let addon = use_context::<InAddon>();
    let size = computed(move || group.map_or(Size::Md, |g| g.size()));
    let disabled = move || props.disabled.get() || group.is_some_and(|g| g.disabled());
    provide_icon_size(move || size.get());
    provide_text_color();
    let look = Look {
        t,
        size,
        variant: computed(move || props.variant.get()),
        icon_only: computed(move || props.icon_only.get()),
        own_disabled: computed(move || {
            props.disabled.get() && !group.is_some_and(|g| g.disabled())
        }),
        idle: computed(disabled),
    };
    let el = base_button()
        .disabled(disabled)
        .accessibility_label(props.accessibility_label)
        .accessibility_hint(props.accessibility_hint)
        .hit_slop(move || vertical_hit_slop(t, field_button_side(t, size.get())))
        .on_press(move || cx.emit(InputGroupButtonEvent::Press))
        .app_class(move || Some(edge_class(t, size.get(), addon)));
    let icon_side = move || t.length(icon_size_token(size.get()));
    button_look(el, look)
        .child(dynamic(move || {
            props.icon.get().map(|name| {
                icon_view(t, move || name, icon_side)
                    .app_class(props.icon_class)
                    .inline_style(move || props.icon_style.get())
            })
        }))
        .children_each(props.children, move |child, _| {
            text("")
                .app_class(props.label_class)
                .inline_style(move || Style::from(props.label_style.get()))
                .style_when(props.icon_only, |s| s.hidden())
                .child(child)
        })
}

/// The props of [`InputGroupPasswordToggle`].
#[derive(Debug, Clone, PartialEq, Props)]
pub struct InputGroupPasswordToggleProps {
    /// Whether the password shows (controlled): the app owns it, `onChange`
    /// asks for a change. Unset when it is made, the toggle keeps its own
    /// state, hidden at first.
    pub shown: Prop<Option<bool>>,
    /// No presses, at half opacity. A disabled field disables it too.
    #[prop(default)]
    pub disabled: Prop<bool>,
    /// What screen readers say: "Show password" (translatable,
    /// `show-password`) by default. It does not change with the state,
    /// which screen readers say as pressed.
    pub accessibility_label: Prop<Option<SharedString>>,
    /// Tailwind classes of the eye.
    pub icon_class: Prop<Option<SharedString>>,
    /// The inline style of the eye.
    #[prop(default)]
    pub icon_style: Prop<Style>,
}

/// The events of an [`InputGroupPasswordToggle`].
#[derive(Debug, Clone, PartialEq, Event)]
pub enum InputGroupPasswordToggleEvent {
    /// It was pressed: whether the password is to show. It runs before
    /// anything changes; a controlled toggle waits for the app's `shown`.
    #[args(shown)]
    Change(bool),
}

/// The eye button of a password: shows the group's masked `Input` while
/// pressed, an eye crossed out then. A toggle button for screen readers
/// ("Show password", pressed or not), a ghost square in an
/// `InputGroupAddon`. When it goes away, the password is hidden again.
#[component]
pub fn InputGroupPasswordToggle(
    props: InputGroupPasswordToggleProps,
    cx: Scope<InputGroupPasswordToggleEvent>,
) -> impl IntoElement {
    let t = theme();
    let group = current();
    let addon = use_context::<InAddon>();
    let size = computed(move || group.map_or(Size::Md, |g| g.size()));
    let disabled = move || props.disabled.get() || group.is_some_and(|g| g.disabled());
    let controlled = props.shown.get_untracked().is_some();
    let own = signal(false);
    let shown = computed(move || {
        if controlled {
            props.shown.get().unwrap_or(false)
        } else {
            own.get()
        }
    });
    if let Some(g) = group {
        effect(move || {
            let on = shown.get();
            if g.shown.get_untracked() != on {
                g.shown.set(on);
            }
        });
        cleanup(move || {
            g.shown.try_set(false);
        });
    }
    provide_icon_size(move || size.get());
    provide_text_color();
    let look = Look {
        t,
        size,
        variant: computed(|| ButtonVariant::Ghost),
        icon_only: computed(|| true),
        own_disabled: computed(move || {
            props.disabled.get() && !group.is_some_and(|g| g.disabled())
        }),
        idle: computed(disabled),
    };
    let on_change = Rc::new(move |next: bool| {
        cx.emit(InputGroupPasswordToggleEvent::Change(next));
        if !controlled {
            own.set(next);
        }
    });
    let el = base_toggle()
        .pressed(move || shown.get())
        .disabled(disabled)
        .accessibility_label(move || {
            Some(
                props
                    .accessibility_label
                    .get()
                    .unwrap_or_else(|| labels::text(Label::ShowPassword)),
            )
        })
        .hit_slop(move || vertical_hit_slop(t, field_button_side(t, size.get())))
        .on_change(move |next: &bool| on_change(*next))
        .app_class(move || Some(edge_class(t, size.get(), addon)));
    button_look(el, look).child(
        icon_view(
            t,
            move || {
                if shown.get() {
                    IconName::EyeOff
                } else {
                    IconName::Eye
                }
            },
            move || t.length(icon_size_token(size.get())),
        )
        .app_class(props.icon_class)
        .inline_style(move || props.icon_style.get()),
    )
}

/// The field's frame inside a group: the inner height of the frame (its
/// height less the edges), and its text `input-group-gap` from an addon
/// beside it, the field's padding from the frame's edge (by NSH's
/// `first:` / `last:`, so the group counts nothing); in a column, the
/// field's padding at both sides.
pub(crate) fn field_class(t: Theme, group: Group) -> SharedString {
    let size = group.size();
    let pad = t.length(padding_token(size));
    if group.column() {
        return Classes::new().length("", "px", pad).build();
    }
    let gap = gap(t, size);
    Classes::new()
        .length("", "ps", gap)
        .length("", "pe", gap)
        .length("first", "ps", pad)
        .length("last", "pe", pad)
        .build()
}

/// The field's height inside a group of `size`: the frame's less its
/// edges.
pub(crate) fn field_height(t: Theme, size: Size) -> Points {
    t.length(height_token(size)) - t.length(INPUT_BORDER_WIDTH) * 2.0
}
