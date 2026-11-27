//! `SelectionBar`, `SelectionBarAction` and `SelectionBarSeparator`: a bar
//! floating at the bottom of the window (or of a container) while items are
//! selected, with their count, its "×" and the actions on all of them. On
//! `nsh/base`'s `Toolbar` (one stop of the keyboard focus whose arrows move
//! between the actions, the toolbar role) and NSH's overlay; the actions are
//! the library's `Button`s. The library adds the surface and where it goes,
//! the count and its announcement, Escape, the motion, the room it covers
//! and the steps it takes in a narrow window. API:
//! `docs/components/selection-bar.md`.

use std::rc::Rc;

use nsh_base::{toolbar as base_toolbar, toolbar_separator as base_separator};
use nsh_core::{
    element::{ScrollEvent, Style, TextPartStyle, safe_area, watch_scroll},
    prelude::*,
};

use super::{
    button::{
        BUTTON_DESTRUCTIVE_RING, BUTTON_GHOST_FOREGROUND, BUTTON_GHOST_HOVER,
        BUTTON_GHOST_HOVER_FOREGROUND, BUTTON_HEIGHT_LG, BUTTON_HEIGHT_MD, BUTTON_HEIGHT_SM,
        BUTTON_HEIGHT_XS, BUTTON_RING, ButtonProps, ButtonVariant, Extras, button, draw_with,
    },
    dialog::Presence,
    tooltip::tooltip,
};
use crate::{
    foundation::{
        icon::{IconName, provide_text_color},
        labels::{self, Label},
        popup, shadow,
        size::Size,
        trigger,
        typography::{font_size_token, line_height, medium, reset_inherited},
    },
    theme::{
        Theme, ThemeAppearance, provide_over,
        seeds::{DURATION, SPACING},
        semantic::{
            BORDER, BORDER_WIDTH, DESTRUCTIVE, POPOVER, POPOVER_FOREGROUND, RADIUS_XL, RING_WIDTH,
        },
        theme,
    },
    tokens,
};

tokens! {
    /// The bar's fill (`popover`, as the library's other floating surfaces).
    SELECTION_BAR: Color = "selection-bar" => |t| t.color(POPOVER),
    /// The bar's text: the count, and what its actions take as theirs.
    SELECTION_BAR_FOREGROUND: Color = "selection-bar-foreground" => |t| t.color(POPOVER_FOREGROUND),
    /// The bar's edge.
    SELECTION_BAR_BORDER: Color = "selection-bar-border" => |t| t.color(BORDER),
    /// The lines between the count and the actions, and between runs of
    /// actions.
    SELECTION_BAR_SEPARATOR: Color = "selection-bar-separator" => |t| t.color(BORDER),
    /// The bar's corner radius (`radius-xl`: a button's corners plus the
    /// bar's padding, so they are concentric).
    SELECTION_BAR_RADIUS: Length = "selection-bar-radius" => |t| t.length(RADIUS_XL),
    /// The padding around the bar's row.
    SELECTION_BAR_PADDING: Length = "selection-bar-padding" => |t| t.length(SPACING) * 1.5,
    /// The gap between the count, the "×", the separators and the actions.
    SELECTION_BAR_GAP: Length = "selection-bar-gap" => |t| t.length(SPACING),
    /// The bar's shadow level (4: `lg`).
    SELECTION_BAR_SHADOW: Number = "selection-bar-shadow" => |t| t.value(4.0),
    /// How far the bar is from the bottom edge (inside the safe area) and,
    /// at least, from the side edges of its window or container.
    SELECTION_BAR_OFFSET: Length = "selection-bar-offset" => |t| t.pointer_touch(t.length(SPACING) * 6.0, t.length(SPACING) * 4.0),
    /// How far the actions fade out at a side where more of them are
    /// scrolled out of the bar.
    SELECTION_BAR_FADE: Length = "selection-bar-fade" => |t| t.length(SPACING) * 6.0,
    /// A destructive action's text and icon.
    SELECTION_BAR_ACTION_DESTRUCTIVE_FOREGROUND: Color = "selection-bar-action-destructive-foreground" => |t| t.color(DESTRUCTIVE),
    /// A destructive action's fill under the pointer: `destructive` at 10 %,
    /// 20 % in the dark.
    SELECTION_BAR_ACTION_DESTRUCTIVE_HOVER: Color = "selection-bar-action-destructive-hover" => |t| t.light_dark(t.color(DESTRUCTIVE).opacity(0.1), t.color(DESTRUCTIVE).opacity(0.2)),
    /// How long the bar takes to come and to go, in milliseconds.
    SELECTION_BAR_DURATION: Number = "selection-bar-duration" => |t| t.number(DURATION) * 4.0 / 3.0,
    /// Whether the bar fades and slides in and out (1) or shows and goes in
    /// one frame (0).
    SELECTION_BAR_ANIMATE: Number = "selection-bar-animate" => |t| t.value(1.0),
}

/// How far the bar slides as it comes and goes.
const SLIDE: f32 = 8.0;

/// Where a [`SelectionBar`] floats.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, PropEnum)]
pub enum SelectionBarPlacement {
    /// At the bottom centre of the window, above the page.
    #[default]
    Window,
    /// At the bottom centre of its parent (the view it is a child of), over
    /// the parent's content.
    Container,
}

nsh_core::static_reactive!(SelectionBarPlacement: SelectionBarPlacement);

/// What an action finds of its bar.
#[derive(Clone, Copy)]
struct BarContext {
    size: Memo<Size>,
    /// The actions at full size do not fit: those with an icon show it
    /// alone.
    compact: Memo<bool>,
}

/// The props of [`SelectionBar`].
#[derive(Debug, Clone, PartialEq, Props)]
pub struct SelectionBarProps {
    /// How many items are selected.
    #[prop(default)]
    pub count: Prop<u32>,
    /// Shown; left out, while `count` is above 0. Set it for a selection
    /// mode that shows the bar with nothing selected yet.
    pub open: Prop<Option<bool>>,
    /// What the count says ("3 tasks selected"); left out, the library's
    /// words ("3 selected": `count-selected`, its `{count}` replaced).
    pub label: Prop<Option<SharedString>>,
    /// Where it floats (`"window"` by default, or `"container"`: its
    /// parent). Read when it is made.
    #[prop(default)]
    pub placement: Prop<SelectionBarPlacement>,
    /// The actions' and the "×"'s size (`xs`, `sm`, `md`, `lg`), and the
    /// count's type.
    #[prop(default)]
    pub size: Prop<Size>,
    /// Draws the bar, and what it holds and opens, in these values whatever
    /// the app's appearance (`"dark"`: a dark bar on a light page); left
    /// out, the app's.
    pub appearance: Prop<Option<ThemeAppearance>>,
    /// What screen readers call the toolbar; left out, the count's words.
    pub accessibility_label: Prop<Option<SharedString>>,
    /// Tailwind classes of the bar (its surface), after its own look.
    pub class: Prop<Option<SharedString>>,
    /// The inline style of the bar, after its own look and classes.
    #[prop(default)]
    pub style: Prop<Style>,
    /// Tailwind classes of the count's text.
    pub count_class: Prop<Option<SharedString>>,
    /// The inline style of the count's text.
    #[prop(default)]
    pub count_style: Prop<TextPartStyle>,
    /// Tailwind classes of the "×".
    pub clear_class: Prop<Option<SharedString>>,
    /// The inline style of the "×".
    #[prop(default)]
    pub clear_style: Prop<Style>,
    /// The actions: `SelectionBarAction`s, `SelectionBarSeparator`s, a
    /// `DropdownMenu` for more, any `Button`.
    pub children: Prop<Children>,
}

/// The events of a [`SelectionBar`].
#[derive(Debug, Clone, PartialEq, Event)]
pub enum SelectionBarEvent {
    /// The user asks to clear the selection: the "×", or Escape while the
    /// keyboard focus is in the bar. The app empties its selection.
    Clear,
    /// The room the bar covers at the bottom of its window or container
    /// changed: from that edge to the bar's top (its height, its offset and,
    /// in the window, the safe area), 0 once it is gone. Give the scrolling
    /// content as much room at its end.
    #[args(inset)]
    InsetChange(f32),
}

/// A bar of actions on the selected items: it floats at the bottom
/// centre of the window (or of its parent) while items are selected, says
/// how many with an "×" that clears them, and holds the actions. It comes
/// and goes with a short fade and slide; the arrow keys move between its
/// actions; Escape in it asks to clear the selection; screen readers hear
/// the count as it changes.
#[component]
pub fn SelectionBar(props: SelectionBarProps, cx: Scope<SelectionBarEvent>) -> impl IntoElement {
    let t = provide_over(Vec::new, move || props.appearance.get());
    provide_text_color();
    let is_open = computed(move || props.open.get().unwrap_or_else(|| props.count.get() > 0));
    // The words of the count, kept while the bar goes (it does not say
    // "0 selected" on its way out).
    let words_now = move || {
        props.label.get().unwrap_or_else(|| {
            labels::text(Label::CountSelected)
                .replace("{count}", &props.count.get().to_string())
                .into()
        })
    };
    let words = signal(untracked(words_now));
    let number = signal(untracked(|| props.count.get()));
    let first = stored(true);
    effect(move || {
        let now = words_now();
        let count = props.count.get();
        let open = is_open.get();
        untracked(|| {
            if !open {
                return;
            }
            if number.get_untracked() != count {
                number.set(count);
            }
            let changed = words.get_untracked() != now;
            if changed {
                words.set(now.clone());
            }
            // Heard as the bar appears and as the count changes; not for
            // a bar already there when the page is made.
            if !first.replace(false) || changed {
                announce_for_accessibility(now.to_string());
            }
        });
    });
    effect(move || {
        if !is_open.get() {
            first.replace(false);
        }
    });

    let presence = Presence::with_timing(
        move || is_open.get(),
        move || t.number(SELECTION_BAR_ANIMATE) != 0.0,
        move || t.number(SELECTION_BAR_DURATION),
    );
    let size = computed(move || props.size.get());

    // The steps of a narrow bar, each taken while the one before does not
    // fit: the actions with an icon show it alone (`compact`), then the
    // count shows its number alone (`tight`), then the actions scroll. What
    // each step needs is measured in the form before it and kept, so a
    // step taken does not undo itself.
    let room = signal(0.0_f32);
    // The count and its "×" as they are, and as they are with the count's
    // words.
    let head = signal(0.0_f32);
    let head_words = signal(0.0_f32);
    // The actions as they are, at full size, and with icons alone.
    let content = signal(0.0_f32);
    let full = signal(0.0_f32);
    let icons = signal(0.0_f32);
    let offset = move || t.length(SELECTION_BAR_OFFSET).0;
    let window_placement = props.placement.get_untracked() == SelectionBarPlacement::Window;
    let room_now = move || {
        if window_placement {
            let inset = safe_area();
            window().width.0 - inset.left.0 - inset.right.0 - 2.0 * offset()
        } else {
            room.get() - 2.0 * offset()
        }
    };
    // What the strip of actions may take beside a head `head` wide: the
    // room less the head, the separator after it, the gaps, the padding
    // and the edge; plus the focus rings' room on each side, which the
    // strip keeps inside it (a scroll view clips) and lends back.
    let ring = move || t.length(RING_WIDTH).0;
    let strip_room = move |head: f32| {
        let fixed = head
            + t.length(BORDER_WIDTH).0
            + 2.0 * t.length(SELECTION_BAR_GAP).0
            + 2.0 * t.length(SELECTION_BAR_PADDING).0
            + 2.0 * t.length(BORDER_WIDTH).0;
        (room_now() - fixed).max(0.0) + 2.0 * ring()
    };
    let room_known = move || window_placement || room.get() > 0.0;
    let compact = computed(move || {
        room_known()
            && head_words.get() > 0.0
            && full.get() > 0.0
            && full.get() > strip_room(head_words.get()) + 0.5
    });
    let tight = computed(move || {
        compact.get() && icons.get() > 0.0 && icons.get() > strip_room(head_words.get()) + 0.5
    });
    provide_context(BarContext { size, compact });

    // Escape is an open menu's of the bar while it is open: its overlay,
    // newer than the bar's, hears it first; this is a guard for one that
    // does not listen.
    let layers = signal(0_u32);
    trigger::provide_open_inside(layers);
    let escape = move || {
        if layers.get_untracked() == 0 && !popup::covered() {
            cx.emit(SelectionBarEvent::Clear);
        }
    };
    // Escape (and Android's back) asks to clear while the keyboard focus is
    // in the bar: an empty overlay of the bar's own listens to `onCancel`
    // then, so it is the top overlay listening unless a menu or popover
    // opened from the bar is over it. Elsewhere (a search field, the list)
    // Escape is the app's.
    let bar_ref = element_ref();
    let focus_in = signal(false);
    bar_ref.on_focus_in(move || focus_in.set(true));
    bar_ref.on_focus_out(move || focus_in.set(false));
    let listening = move || presence.mounted.get() && !presence.closing.get() && focus_in.get();
    let canceller = move || {
        vec![
            overlay()
                .w(px(0.0))
                .h(px(0.0))
                .focusable(false)
                .accessible(false)
                .pointer_events(PointerEvents::None)
                .on_cancel(escape)
                .into_element(),
        ]
    };

    // The actions' strip fades out at a side where more of them are
    // scrolled out of it.
    let strip_ref = element_ref();
    let clipped = signal((false, false));
    let strip_width = signal(0.0_f32);
    watch_scroll(
        move || strip_ref.node(),
        move |e: &ScrollEvent| {
            let start = e.offset.x.0 > 0.5;
            let end = e.offset.x.0 + e.size.width.0 < e.content_size.width.0 - 0.5;
            if clipped.get_untracked() != (start, end) {
                clipped.set((start, end));
            }
            if (strip_width.get_untracked() - e.size.width.0).abs() > 0.25 {
                strip_width.set(e.size.width.0);
            }
        },
    );
    let fade_stops = move || {
        let (start, end) = clipped.get();
        let width = strip_width.get();
        if !(start || end) || width <= 0.0 {
            return Vec::new();
        }
        let f = (t.length(SELECTION_BAR_FADE).0 / width).min(0.5);
        let mut stops: Vec<ColorStop> = Vec::new();
        if start {
            stops.push((transparent_black(), relative(0.0)).into());
            stops.push((black(), relative(f)).into());
        } else {
            stops.push((black(), relative(0.0)).into());
        }
        if end {
            stops.push((black(), relative(1.0 - f)).into());
            stops.push((transparent_black(), relative(1.0)).into());
        } else {
            stops.push((black(), relative(1.0)).into());
        }
        stops
    };

    // The room it covers, reported as it changes.
    let height = signal(0.0_f32);
    let inset = computed(move || {
        if !presence.mounted.get() || height.get() <= 0.0 {
            return 0.0;
        }
        let bottom = if window_placement {
            safe_area().bottom.0
        } else {
            0.0
        };
        height.get() + offset() + bottom
    });
    let reported = stored(0.0_f32);
    effect(move || {
        let now = inset.get();
        untracked(|| {
            if (now - reported.get()).abs() > 0.25 {
                reported.replace(now);
                cx.emit(SelectionBarEvent::InsetChange(now));
            }
        });
    });

    let bar = move || {
        let fade = presence.fade;
        let closing = presence.closing;
        let slide = fade.interpolate(&[0.0, 1.0], &[SLIDE, 0.0]);
        let scale = fade.interpolate(&[0.0, 1.0], &[0.95, 1.0]);
        let font_size = move || t.length(font_size_token(size.get()));
        let name = move || {
            props
                .accessibility_label
                .get()
                .or_else(|| Some(words.get()))
        };
        // Tight on room, the number alone; screen readers still hear the
        // words.
        let count = text(move || {
            if tight.get() {
                SharedString::from(number.get().to_string())
            } else {
                words.get()
            }
        })
        .label(move || words.get())
        .text_size(font_size)
        .line_height(move || line_height(font_size()))
        .font_weight(medium(t))
        .tabular_nums()
        .whitespace_nowrap()
        .flex_shrink_0()
        .px(move || t.length(SPACING) * 2.0)
        .app_class(props.count_class)
        .inline_style(move || Style::from(props.count_style.get()));
        let clear_words = move || labels::text(Label::ClearSelection);
        // The tooltip's anchor: a view around the button (a component's
        // element takes no tooltip in Rust).
        let clear = view()
            .flex_shrink_0()
            .tooltip(tooltip().child(text(clear_words)))
            .child(
                button()
                    .variant(ButtonVariant::Ghost)
                    .size(move || size.get())
                    .icon_only(true)
                    .icon(IconName::X)
                    .accessibility_label(move || Some(clear_words()))
                    .on_press(move || cx.emit(SelectionBarEvent::Clear))
                    .app_class(props.clear_class)
                    .inline_style(move || props.clear_style.get()),
            );
        let head_part = view()
            .flex_row()
            .items_center()
            .flex_shrink_0()
            .on_layout(move |b: &Bounds<Points>| {
                let w = b.size.width.0;
                if (head.get_untracked() - w).abs() > 0.25 {
                    head.set(w);
                }
                if !tight.get_untracked() && (head_words.get_untracked() - w).abs() > 0.25 {
                    head_words.set(w);
                }
            })
            .child(count)
            .child(clear);
        let row = view()
            .flex_row()
            .items_center()
            .gap(move || t.length(SELECTION_BAR_GAP))
            .p(move || px(ring()))
            .on_layout(move |b: &Bounds<Points>| {
                let w = b.size.width.0;
                if (content.get_untracked() - w).abs() > 0.25 {
                    content.set(w);
                }
                if !compact.get_untracked() && (full.get_untracked() - w).abs() > 0.25 {
                    full.set(w);
                }
                if compact.get_untracked() && (icons.get_untracked() - w).abs() > 0.25 {
                    icons.set(w);
                }
            })
            .child(props.children);
        let strip = scroll()
            .horizontal()
            .scrollbar(false)
            .flex_shrink_0()
            .w(move || px(content.get().min(strip_room(head.get()))))
            .m(move || px(-ring()))
            .element_ref(&strip_ref)
            .mask_linear(deg(90.0), fade_stops)
            .child(row);
        let toolbar = base_toolbar()
            .accessibility_label(name)
            .items_center()
            .gap(move || t.length(SELECTION_BAR_GAP))
            .min_w(px(0.0))
            .child(head_part)
            .child(separator(t, move || size.get()))
            .child(strip);
        reset_inherited(view())
            .flex_row()
            .items_center()
            .max_w(Length::Definite(relative(1.0)))
            .p(move || t.length(SELECTION_BAR_PADDING))
            .rounded(move || t.length(SELECTION_BAR_RADIUS))
            .border(move || t.length(BORDER_WIDTH))
            .border_color(move || t.color(SELECTION_BAR_BORDER))
            .bg(move || t.color(SELECTION_BAR))
            .text_color(move || t.color(SELECTION_BAR_FOREGROUND))
            .style_with(move |s| {
                s.shadow(shadow::box_shadows(
                    t,
                    shadow::level(t.number(SELECTION_BAR_SHADOW)),
                ))
            })
            .bind_opacity(&fade)
            .bind_translate_y(&slide)
            .bind_scale(&scale)
            .pointer_events(move || {
                if closing.get() {
                    PointerEvents::None
                } else {
                    PointerEvents::Auto
                }
            })
            .on_layout(move |b: &Bounds<Points>| {
                if (height.get_untracked() - b.size.height.0).abs() > 0.25 {
                    height.set(b.size.height.0);
                }
            })
            .element_ref(&bar_ref)
            .app_class(props.class)
            .inline_style(move || props.style.get())
            .child(toolbar)
            .into_element()
    };
    let shown = move || presence.mounted.get();

    if window_placement {
        let place = move || {
            let inset = safe_area();
            overlay()
                .inset_0()
                .focusable(false)
                .pointer_events(PointerEvents::BoxNone)
                .flex_col()
                .justify_end()
                .items_center()
                .pt(move || px(offset() + inset.top.0))
                .pb(move || px(offset() + inset.bottom.0))
                .pl(move || px(offset() + inset.left.0))
                .pr(move || px(offset() + inset.right.0))
                .child(bar())
                .into_element()
        };
        // An empty view where the app put it: the bar is the window's.
        view()
            .absolute()
            .left(px(0.0))
            .bottom(px(0.0))
            .w(px(0.0))
            .h(px(0.0))
            .child(when(shown, move || vec![place()]))
            .child(when(listening, canceller))
            .into_element()
    } else {
        view()
            .absolute()
            .left(px(0.0))
            .right(px(0.0))
            .bottom(px(0.0))
            .flex_col()
            .items_center()
            .pointer_events(PointerEvents::BoxNone)
            .px(move || px(offset()))
            .pb(move || px(offset()))
            .on_layout(move |b: &Bounds<Points>| {
                if (room.get_untracked() - b.size.width.0).abs() > 0.25 {
                    room.set(b.size.width.0);
                }
            })
            .child(when(shown, move || vec![bar()]))
            .child(when(listening, canceller))
            .into_element()
    }
}

/// A vertical line half the actions' height, never a stop of the focus.
fn separator(t: Theme, size: impl Fn() -> Size + Copy + 'static) -> impl IntoElement {
    base_separator()
        .flex_shrink_0()
        .w(move || t.length(BORDER_WIDTH))
        .h(move || t.length(action_height_token(size())) * 0.5)
        .bg(move || t.color(SELECTION_BAR_SEPARATOR))
}

fn action_height_token(size: Size) -> crate::theme::Token<crate::theme::kind::Length> {
    size.pick(
        BUTTON_HEIGHT_XS,
        BUTTON_HEIGHT_SM,
        BUTTON_HEIGHT_MD,
        BUTTON_HEIGHT_LG,
    )
}

/// The props of [`SelectionBarSeparator`].
#[derive(Debug, Clone, PartialEq, Props)]
pub struct SelectionBarSeparatorProps {}

/// A line between runs of a [`SelectionBar`]'s actions: half their height,
/// in `selection-bar-separator`; never a stop of the focus, skipped by
/// screen readers.
#[component]
pub fn SelectionBarSeparator(
    _props: SelectionBarSeparatorProps,
    _cx: Scope<NoEvent>,
) -> impl IntoElement {
    let t = theme();
    let bar = use_context::<BarContext>();
    separator(t, move || bar.map(|b| b.size.get()).unwrap_or_default())
}

/// A [`SelectionBarAction`]'s look.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, PropEnum)]
pub enum SelectionBarActionVariant {
    /// The bar's text color, a quiet fill under the pointer (a ghost
    /// button).
    #[default]
    Default,
    /// Deletes or destroys: the destructive color, a destructive tint under
    /// the pointer.
    Destructive,
}

nsh_core::static_reactive!(SelectionBarActionVariant: SelectionBarActionVariant);

/// The props of [`SelectionBarAction`].
#[derive(Debug, Clone, PartialEq, Props)]
pub struct SelectionBarActionProps {
    /// Its words: shown beside the icon, or its tooltip and its name when
    /// the icon shows alone.
    #[prop(default)]
    pub label: Prop<SharedString>,
    /// An icon before the label.
    pub icon: Prop<Option<IconName>>,
    /// An icon after the label (a menu's `chevron-down`).
    pub icon_end: Prop<Option<IconName>>,
    /// Always the icon alone, the label in its tooltip and its name.
    #[prop(default)]
    pub icon_only: Prop<bool>,
    /// Its look: `default`, or `destructive`.
    #[prop(default)]
    pub variant: Prop<SelectionBarActionVariant>,
    /// Its size; left out, the bar's. Give the bar's `size` to an action
    /// in a menu's `trigger`, which is made where the app writes it and
    /// does not see the bar.
    pub size: Prop<Option<Size>>,
    /// No presses, at half opacity.
    #[prop(default)]
    pub disabled: Prop<bool>,
    /// Its action is running: a spinner in place of the icon, no presses.
    #[prop(default)]
    pub loading: Prop<bool>,
    /// What screen readers say, when it should differ from `label`.
    pub accessibility_label: Prop<Option<SharedString>>,
    /// What pressing it does, for screen readers.
    pub accessibility_hint: Prop<Option<SharedString>>,
    /// Tailwind classes of the label part (its words).
    pub label_class: Prop<Option<SharedString>>,
    /// The inline style of the label part.
    #[prop(default)]
    pub label_style: Prop<TextPartStyle>,
    /// Tailwind classes of the icons (`icon`, `iconEnd`, the spinner).
    pub icon_class: Prop<Option<SharedString>>,
    /// The inline style of the icons.
    #[prop(default)]
    pub icon_style: Prop<Style>,
}

/// The events of a [`SelectionBarAction`].
#[derive(Debug, Clone, PartialEq, Event)]
pub enum SelectionBarActionEvent {
    /// It was pressed (never while `disabled` or `loading`).
    Press,
}

/// An action of a [`SelectionBar`]: a ghost button of the bar's size with
/// an icon and its label, or the icon alone (`iconOnly`, or while the bar
/// is short of room) with the label as its tooltip and its name.
#[component]
pub fn SelectionBarAction(
    props: SelectionBarActionProps,
    cx: Scope<SelectionBarActionEvent>,
) -> impl IntoElement {
    let bar = use_context::<BarContext>();
    let outer = theme();
    let destructive = move || props.variant.get() == SelectionBarActionVariant::Destructive;
    // The destructive look is the button's ghost look in other colors.
    provide_over(
        move || {
            if !destructive() {
                return Vec::new();
            }
            let fg = outer.color(SELECTION_BAR_ACTION_DESTRUCTIVE_FOREGROUND);
            vec![
                (BUTTON_GHOST_FOREGROUND.name(), fg),
                (BUTTON_GHOST_HOVER_FOREGROUND.name(), fg),
                (
                    BUTTON_GHOST_HOVER.name(),
                    outer.color(SELECTION_BAR_ACTION_DESTRUCTIVE_HOVER),
                ),
                (BUTTON_RING.name(), outer.color(BUTTON_DESTRUCTIVE_RING)),
            ]
        },
        || None,
    );
    let size = computed(move || {
        props
            .size
            .get()
            .unwrap_or_else(|| bar.map(|b| b.size.get()).unwrap_or_default())
    });
    let alone = computed(move || {
        props.icon_only.get()
            || (bar.is_some_and(|b| b.compact.get()) && props.icon.with(Option::is_some))
    });
    let label = props.label;
    let name = computed(move || {
        props
            .accessibility_label
            .get()
            .or_else(|| Some(label.get()))
    });
    let button_props = ButtonProps {
        variant: Prop::from(IntoReactive::<ButtonVariant>::into_reactive(
            ButtonVariant::Ghost,
        )),
        size: Prop::from(IntoReactive::<Size>::into_reactive(size)),
        icon_only: Prop::from(IntoReactive::<bool>::into_reactive(alone)),
        icon: props.icon,
        icon_end: props.icon_end,
        loading: props.loading,
        disabled: props.disabled,
        invalid: Prop::from(IntoReactive::<bool>::into_reactive(false)),
        accessibility_label: Prop::from(IntoReactive::<Option<SharedString>>::into_reactive(name)),
        accessibility_hint: props.accessibility_hint,
        label_class: props.label_class,
        label_style: props.label_style,
        icon_class: props.icon_class,
        icon_style: props.icon_style,
        children: Default::default(),
    };
    // The label is the tooltip of an action showing its icon alone; the
    // tooltip is the button's own hint, so the button stays the root (a
    // menu's trigger is placed and pressed as its slot's root). Mounted
    // only then: a hint that is there takes Escape before the bar.
    let hint = when(
        move || alone.get(),
        move || vec![tooltip().child(text(move || label.get())).into_element()],
    )
    .into_element();
    draw_with(
        button_props,
        None,
        move || cx.emit(SelectionBarActionEvent::Press),
        Extras {
            words: Some(Rc::new(move || label.get())),
            hint: Some(hint),
        },
    )
}
