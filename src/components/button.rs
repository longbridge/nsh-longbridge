//! `Button`: a button on `nsh/base`'s `Button` (presses, the
//! disabled and busy states, its role for screen readers). API:
//! `docs/components/button.md`.

use nsh_base::button as base_button;
use nsh_core::{element::Style, element::TextPartStyle, prelude::*};

use crate::{
    foundation::{
        focus::focus_ring,
        icon::{IconName, icon_size_token, icon_view, provide_icon_size, provide_text_color},
        labels::{self, Label},
        motion,
        press::{BUTTON_SCALE, press},
        shadow,
        size::{Size, hit_slop},
        trigger,
        typography::{font_size_token, line_height, medium},
    },
    theme::{
        Theme, Token, kind,
        seeds::SPACING,
        semantic::{
            ACCENT, ACCENT_FOREGROUND, BACKGROUND, BORDER, BORDER_WIDTH, CONTROL_HEIGHT_LG,
            CONTROL_HEIGHT_MD, CONTROL_HEIGHT_SM, CONTROL_HEIGHT_TOUCH_LG, CONTROL_HEIGHT_TOUCH_MD,
            CONTROL_HEIGHT_TOUCH_SM, CONTROL_HEIGHT_TOUCH_XS, CONTROL_HEIGHT_XS, DESTRUCTIVE,
            DESTRUCTIVE_FOREGROUND, FOREGROUND, INPUT, OPACITY_DISABLED, PRIMARY,
            PRIMARY_FOREGROUND, RADIUS_MD, RING, RING_INVALID, RING_OPACITY, SECONDARY,
            SECONDARY_FOREGROUND,
        },
        theme,
    },
    tokens,
};

tokens! {
    /// The corner radius.
    BUTTON_RADIUS: Length = "button-radius" => |t| t.length(RADIUS_MD),
    /// An extra small button's height (and an icon-only one's side).
    BUTTON_HEIGHT_XS: Length = "button-height-xs" => |t| t.pointer_touch(t.length(CONTROL_HEIGHT_XS), t.length(CONTROL_HEIGHT_TOUCH_XS)),
    /// A small button's height.
    BUTTON_HEIGHT_SM: Length = "button-height-sm" => |t| t.pointer_touch(t.length(CONTROL_HEIGHT_SM), t.length(CONTROL_HEIGHT_TOUCH_SM)),
    /// A medium button's height.
    BUTTON_HEIGHT_MD: Length = "button-height-md" => |t| t.pointer_touch(t.length(CONTROL_HEIGHT_MD), t.length(CONTROL_HEIGHT_TOUCH_MD)),
    /// A large button's height.
    BUTTON_HEIGHT_LG: Length = "button-height-lg" => |t| t.pointer_touch(t.length(CONTROL_HEIGHT_LG), t.length(CONTROL_HEIGHT_TOUCH_LG)),
    /// An extra small button's horizontal padding (`px-2`); with an
    /// icon, `spacing` × 0.5 less on each side.
    BUTTON_PADDING_X_XS: Length = "button-padding-x-xs" => |t| t.length(SPACING) * 2.0,
    /// A small button's horizontal padding (`px-3`); with an icon, `spacing`
    /// × 0.5 less.
    BUTTON_PADDING_X_SM: Length = "button-padding-x-sm" => |t| t.length(SPACING) * 3.0,
    /// A medium button's horizontal padding (`px-4`); with an icon,
    /// `spacing` less.
    BUTTON_PADDING_X_MD: Length = "button-padding-x-md" => |t| t.length(SPACING) * 4.0,
    /// A large button's horizontal padding (`px-6`); with an icon, `spacing`
    /// × 2 less.
    BUTTON_PADDING_X_LG: Length = "button-padding-x-lg" => |t| t.length(SPACING) * 6.0,

    /// The default variant's fill.
    BUTTON_DEFAULT: Color = "button-default" => |t| t.color(PRIMARY),
    /// The default variant's text and icons.
    BUTTON_DEFAULT_FOREGROUND: Color = "button-default-foreground" => |t| t.color(PRIMARY_FOREGROUND),
    /// The default variant's fill under the pointer and while pressed.
    BUTTON_DEFAULT_HOVER: Color = "button-default-hover" => |t| t.color(BUTTON_DEFAULT).opacity(0.9),
    /// The secondary variant's fill.
    BUTTON_SECONDARY: Color = "button-secondary" => |t| t.color(SECONDARY),
    /// The secondary variant's text and icons.
    BUTTON_SECONDARY_FOREGROUND: Color = "button-secondary-foreground" => |t| t.color(SECONDARY_FOREGROUND),
    /// The secondary variant's fill under the pointer and while pressed.
    BUTTON_SECONDARY_HOVER: Color = "button-secondary-hover" => |t| t.color(BUTTON_SECONDARY).opacity(0.8),
    /// The outline variant's fill: `background`, `input` at 30 % in the dark.
    BUTTON_OUTLINE: Color = "button-outline" => |t| t.light_dark(t.color(BACKGROUND), t.color(INPUT).opacity(0.3)),
    /// The outline variant's text and icons.
    BUTTON_OUTLINE_FOREGROUND: Color = "button-outline-foreground" => |t| t.color(FOREGROUND),
    /// The outline variant's edge: `border`, `input` in the dark.
    BUTTON_OUTLINE_BORDER: Color = "button-outline-border" => |t| t.light_dark(t.color(BORDER), t.color(INPUT)),
    /// The outline variant's fill under the pointer: `accent`, `input` at 50 % in the dark.
    BUTTON_OUTLINE_HOVER: Color = "button-outline-hover" => |t| t.light_dark(t.color(ACCENT), t.color(INPUT).opacity(0.5)),
    /// The outline variant's text under the pointer.
    BUTTON_OUTLINE_HOVER_FOREGROUND: Color = "button-outline-hover-foreground" => |t| t.color(ACCENT_FOREGROUND),
    /// The outline variant's shadow level (1: `xs`).
    BUTTON_OUTLINE_SHADOW: Number = "button-outline-shadow" => |t| t.value(1.0),
    /// The ghost variant's text and icons.
    BUTTON_GHOST_FOREGROUND: Color = "button-ghost-foreground" => |t| t.color(FOREGROUND),
    /// The ghost variant's fill under the pointer: `accent`, at 50 % in the dark.
    BUTTON_GHOST_HOVER: Color = "button-ghost-hover" => |t| t.light_dark(t.color(ACCENT), t.color(ACCENT).opacity(0.5)),
    /// The ghost variant's text under the pointer.
    BUTTON_GHOST_HOVER_FOREGROUND: Color = "button-ghost-hover-foreground" => |t| t.color(ACCENT_FOREGROUND),
    /// The destructive variant's fill: `destructive`, at 60 % in the dark.
    BUTTON_DESTRUCTIVE: Color = "button-destructive" => |t| t.light_dark(t.color(DESTRUCTIVE), t.color(DESTRUCTIVE).opacity(0.6)),
    /// The destructive variant's text and icons.
    BUTTON_DESTRUCTIVE_FOREGROUND: Color = "button-destructive-foreground" => |t| t.color(DESTRUCTIVE_FOREGROUND),
    /// The destructive variant's fill under the pointer and while pressed.
    BUTTON_DESTRUCTIVE_HOVER: Color = "button-destructive-hover" => |t| t.color(DESTRUCTIVE).opacity(0.9),
    /// The destructive variant's focus ring: `destructive` at 20 %, 40 % in the dark.
    BUTTON_DESTRUCTIVE_RING: Color = "button-destructive-ring" => |t| t.light_dark(t.color(DESTRUCTIVE).opacity(0.2), t.color(DESTRUCTIVE).opacity(0.4)),
    /// The link variant's text.
    BUTTON_LINK_FOREGROUND: Color = "button-link-foreground" => |t| t.color(PRIMARY),
    /// The edge while focused (the outline variant's, the only one with an edge).
    BUTTON_BORDER_FOCUS: Color = "button-border-focus" => |t| t.color(RING),
    /// The edge while `invalid` (the outline variant's).
    BUTTON_BORDER_INVALID: Color = "button-border-invalid" => |t| t.color(DESTRUCTIVE),
    /// The focus ring.
    BUTTON_RING: Color = "button-ring" => |t| t.color(RING).opacity(t.number(RING_OPACITY)),
    /// The focus ring while `invalid`.
    BUTTON_RING_INVALID: Color = "button-ring-invalid" => |t| t.color(RING_INVALID),
}

/// A button's look.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, PropEnum)]
pub enum ButtonVariant {
    /// The main action: the `primary` fill.
    #[default]
    Default,
    /// A secondary action: the `secondary` fill.
    Secondary,
    /// An edge over the background, a shadow: the ordinary action beside a
    /// main one.
    Outline,
    /// Bare until the pointer is over it: toolbars, inline actions.
    Ghost,
    /// Deletes or destroys something.
    Destructive,
    /// Looks like a link (underlined under the pointer). For an action, not
    /// for opening a URL.
    Link,
}

nsh_core::static_reactive!(ButtonVariant: ButtonVariant);

/// The props of [`Button`].
#[derive(Debug, Clone, PartialEq, Props)]
pub struct ButtonProps {
    /// Its look: `default`, `secondary`, `outline`, `ghost`, `destructive`
    /// or `link`.
    #[prop(default)]
    pub variant: Prop<ButtonVariant>,
    /// Its size (`xs`, `sm`, `md`, `lg`): height, text, icon and padding
    /// change together.
    #[prop(default)]
    pub size: Prop<Size>,
    /// A square button showing its icon alone (`icon`, or the spinner while
    /// loading); give it an `accessibilityLabel`. Its side is the height of
    /// its `size`.
    #[prop(default)]
    pub icon_only: Prop<bool>,
    /// An icon before the content.
    pub icon: Prop<Option<IconName>>,
    /// An icon after the content (a menu's chevron, a next step's arrow).
    pub icon_end: Prop<Option<IconName>>,
    /// Its action is running: a spinner takes the place of `icon` (or comes
    /// before the content), presses do nothing, nothing dims, and screen
    /// readers say it is busy.
    #[prop(default)]
    pub loading: Prop<bool>,
    /// No presses and no hover or press feedback, at half opacity.
    #[prop(default)]
    pub disabled: Prop<bool>,
    /// The app's checks reject it (a form's submit with errors): the
    /// destructive edge (outline) and focus ring.
    #[prop(default)]
    pub invalid: Prop<bool>,
    /// What screen readers say, when the content is not enough (an
    /// icon-only button).
    pub accessibility_label: Prop<Option<SharedString>>,
    /// What pressing it does, for screen readers.
    pub accessibility_hint: Prop<Option<SharedString>>,
    /// Tailwind classes of the label: the part each child sits in (a
    /// string child's text takes its text classes).
    pub label_class: Prop<Option<SharedString>>,
    /// The inline style of the label part.
    #[prop(default)]
    pub label_style: Prop<TextPartStyle>,
    /// Tailwind classes of the icons (`icon`, `iconEnd`, the spinner).
    pub icon_class: Prop<Option<SharedString>>,
    /// The inline style of the icons.
    #[prop(default)]
    pub icon_style: Prop<Style>,
    /// The content: its label (a string), or anything else.
    pub children: Prop<Children>,
}

/// The events of a [`Button`].
#[derive(Debug, Clone, PartialEq, Event)]
pub enum ButtonEvent {
    /// The button was pressed (never while `disabled` or `loading`).
    Press,
}

/// A button: six variants, four sizes, icons before and after
/// its label, an icon-only square, a loading state. Press feedback sinks it
/// slightly (not with reduced motion); the keyboard focus draws a ring
/// around it. On touch screens it is taller and takes presses up to 44
/// points around it.
#[component]
pub fn Button(props: ButtonProps, cx: Scope<ButtonEvent>) -> impl IntoElement {
    draw(props, None, move || cx.emit(ButtonEvent::Press))
}

/// A `Button` with `props` whose presses run `on_press` (never while
/// `disabled` or `loading`): the drawing of `Button` and of the buttons
/// that are one with a role of their own (`DialogClose`,
/// `AlertDialogAction`, a toast's action), its root named by `node` when
/// one is given (to focus it).
pub(crate) fn draw(
    props: ButtonProps,
    node: Option<ElementRef>,
    on_press: impl Fn() + 'static,
) -> impl IntoElement {
    let t = theme();
    let variant = move || props.variant.get();
    let size = move || props.size.get();
    let idle = move || props.disabled.get() || props.loading.get();
    let height = move || t.length(height_token(size()));
    let has_icon =
        move || props.loading.get() || props.icon.get().is_some() || props.icon_end.get().is_some();
    let padding = move || {
        if props.icon_only.get() {
            return px(0.0);
        }
        let base = t.length(padding_token(size()));
        if has_icon() {
            base - t.length(SPACING) * size().pick(0.5, 0.5, 1.0, 2.0)
        } else {
            base
        }
    };
    let font_size = move || t.length(font_size_token(size()));
    let icon_side = move || t.length(icon_size_token(size()));
    provide_icon_size(size);
    provide_text_color();

    let icon_part = move |name: IconName, spin: bool| {
        let icon = icon_view(t, move || name, icon_side);
        let icon = if spin { motion::spin(icon) } else { icon };
        icon.app_class(props.icon_class)
            .inline_style(move || props.icon_style.get())
    };
    let leading = move || {
        if props.loading.get() {
            Some(icon_part(IconName::LoaderCircle, true))
        } else {
            props.icon.get().map(|name| icon_part(name, false))
        }
    };

    let link = props.variant.get_untracked() == ButtonVariant::Link;
    // An overlay's trigger: the press stops here, not at the slot's view,
    // so it opens the overlay from here (`foundation::trigger`).
    let trigger = trigger::current();
    let expanded = trigger.clone();
    // Its overlay is open: lit as under the pointer, until it closes (the
    // slot's view has the trigger's state, not this one).
    let lit = computed(move || expanded.as_ref().is_some_and(|t| t.open()));
    let fill = move || {
        if lit.get() {
            hover_fill(t, variant())
        } else {
            rest_fill(t, variant())
        }
    };
    let foreground = move || {
        if lit.get() {
            hover_foreground(t, variant())
        } else {
            rest_foreground(t, variant())
        }
    };
    let root = base_button()
        .disabled(props.disabled)
        .busy(props.loading)
        .accessibility_label(move || {
            props.accessibility_label.get().or_else(|| {
                (props.icon_only.get() && props.loading.get()).then(|| labels::text(Label::Loading))
            })
        })
        .accessibility_hint(props.accessibility_hint)
        .hit_slop(move || hit_slop(t, height()))
        .on_press(move || {
            if !props.loading.get_untracked() {
                on_press();
                if let Some(trigger) = &trigger {
                    trigger.toggle();
                }
            }
        })
        .flex_row()
        .items_center()
        .justify_center()
        .flex_shrink_0()
        .gap(move || t.length(SPACING) * size().pick(1.0, 1.5, 2.0, 2.0))
        .h(height)
        .style_when(props.icon_only, move |s| s.w(height))
        .px(padding)
        .rounded(move || t.length(BUTTON_RADIUS))
        .border(move || {
            if variant() == ButtonVariant::Outline {
                t.length(BORDER_WIDTH)
            } else {
                px(0.0)
            }
        })
        .border_color(move || {
            if props.invalid.get() {
                t.color(BUTTON_BORDER_INVALID)
            } else {
                t.color(BUTTON_OUTLINE_BORDER)
            }
        })
        .bg(fill)
        .text_color(foreground)
        .text_size(font_size)
        .line_height(move || line_height(font_size()))
        .font_weight(medium(t))
        .whitespace_nowrap()
        .inline_style(move || {
            let level = if variant() == ButtonVariant::Outline {
                shadow::level(t.number(BUTTON_OUTLINE_SHADOW))
            } else {
                0
            };
            shadow::shadow(t, level)
        })
        .hover(move |s| {
            // Not bindable (as the cursor below): read when it is built.
            let s = if link {
                s.underline().underline_offset(px(4.0))
            } else {
                s
            };
            s.bg(move || {
                if idle() {
                    rest_fill(t, variant())
                } else {
                    hover_fill(t, variant())
                }
            })
            .text_color(move || {
                if idle() {
                    rest_foreground(t, variant())
                } else {
                    hover_foreground(t, variant())
                }
            })
        })
        .active(move |s| {
            press(
                s.bg(move || {
                    if idle() {
                        rest_fill(t, variant())
                    } else {
                        hover_fill(t, variant())
                    }
                }),
                BUTTON_SCALE,
                idle,
            )
        })
        // The popup it opens is open (a dialog's, a popover's, a menu's):
        // lit as under the pointer, until it closes. (Its own state when
        // the trigger slot's view is laid out as it; else `lit`.)
        .expanded(move |s| {
            s.bg(move || hover_fill(t, variant()))
                .text_color(move || hover_foreground(t, variant()))
        })
        .focus_visible(move |s| {
            focus_ring(
                s,
                t,
                move || ring(t, variant(), props.invalid.get()),
                Some(BUTTON_BORDER_FOCUS),
            )
        })
        .disabled_style(move |s| {
            let s = s.opacity(move || t.number(OPACITY_DISABLED));
            if link { s.no_underline() } else { s }
        });
    let root = motion::transition_all(root, t);
    let root = match node {
        Some(node) => root.element_ref(&node),
        None => root,
    };
    let root = if link {
        root.cursor_pointer()
    } else {
        root.cursor(Cursor::Default)
    };
    root.child(dynamic(leading))
        .children_each(props.children, move |child, _| {
            text("")
                .app_class(props.label_class)
                .inline_style(move || Style::from(props.label_style.get()))
                .style_when(props.icon_only, |s| s.hidden())
                .child(child)
        })
        .child(dynamic(move || {
            (!props.icon_only.get())
                .then(|| props.icon_end.get())
                .flatten()
                .map(|name| icon_part(name, false))
        }))
}

fn height_token(size: Size) -> Token<kind::Length> {
    size.pick(
        BUTTON_HEIGHT_XS,
        BUTTON_HEIGHT_SM,
        BUTTON_HEIGHT_MD,
        BUTTON_HEIGHT_LG,
    )
}

fn padding_token(size: Size) -> Token<kind::Length> {
    size.pick(
        BUTTON_PADDING_X_XS,
        BUTTON_PADDING_X_SM,
        BUTTON_PADDING_X_MD,
        BUTTON_PADDING_X_LG,
    )
}

fn rest_fill(t: Theme, variant: ButtonVariant) -> AdaptiveColor {
    match variant {
        ButtonVariant::Default => t.color(BUTTON_DEFAULT),
        ButtonVariant::Secondary => t.color(BUTTON_SECONDARY),
        ButtonVariant::Outline => t.color(BUTTON_OUTLINE),
        ButtonVariant::Destructive => t.color(BUTTON_DESTRUCTIVE),
        ButtonVariant::Ghost | ButtonVariant::Link => AdaptiveColor::TRANSPARENT,
    }
}

fn hover_fill(t: Theme, variant: ButtonVariant) -> AdaptiveColor {
    match variant {
        ButtonVariant::Default => t.color(BUTTON_DEFAULT_HOVER),
        ButtonVariant::Secondary => t.color(BUTTON_SECONDARY_HOVER),
        ButtonVariant::Outline => t.color(BUTTON_OUTLINE_HOVER),
        ButtonVariant::Ghost => t.color(BUTTON_GHOST_HOVER),
        ButtonVariant::Destructive => t.color(BUTTON_DESTRUCTIVE_HOVER),
        ButtonVariant::Link => AdaptiveColor::TRANSPARENT,
    }
}

fn rest_foreground(t: Theme, variant: ButtonVariant) -> AdaptiveColor {
    match variant {
        ButtonVariant::Default => t.color(BUTTON_DEFAULT_FOREGROUND),
        ButtonVariant::Secondary => t.color(BUTTON_SECONDARY_FOREGROUND),
        ButtonVariant::Outline => t.color(BUTTON_OUTLINE_FOREGROUND),
        ButtonVariant::Ghost => t.color(BUTTON_GHOST_FOREGROUND),
        ButtonVariant::Destructive => t.color(BUTTON_DESTRUCTIVE_FOREGROUND),
        ButtonVariant::Link => t.color(BUTTON_LINK_FOREGROUND),
    }
}

fn hover_foreground(t: Theme, variant: ButtonVariant) -> AdaptiveColor {
    match variant {
        ButtonVariant::Outline => t.color(BUTTON_OUTLINE_HOVER_FOREGROUND),
        ButtonVariant::Ghost => t.color(BUTTON_GHOST_HOVER_FOREGROUND),
        other => rest_foreground(t, other),
    }
}

fn ring(t: Theme, variant: ButtonVariant, invalid: bool) -> AdaptiveColor {
    if invalid {
        t.color(BUTTON_RING_INVALID)
    } else if variant == ButtonVariant::Destructive {
        t.color(BUTTON_DESTRUCTIVE_RING)
    } else {
        t.color(BUTTON_RING)
    }
}
