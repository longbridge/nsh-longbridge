//! `Popover`: shadcn/ui's popover on `nsh/base`'s `Popover` (the trigger,
//! opening and closing, placement, focus, presence). The popup surface it
//! draws is `foundation::popup`'s, which the menus share. API:
//! `docs/components/popover.md`.

use nsh_base::{OpenChangeReason, PopoverEvent as BaseEvent, popover as base_popover};
use nsh_core::{element::Style, prelude::*};

use crate::{
    foundation::{
        icon::provide_text_color,
        popup::{self, POPOVER_MARGIN, POPOVER_OFFSET},
        trigger,
        typography::{line_height, medium},
    },
    theme::{
        seeds::SPACING,
        semantic::{MUTED_FOREGROUND, TEXT_SM},
        theme,
    },
    tokens,
};

tokens! {
    /// The popup's width (shadcn's `w-72`).
    POPOVER_WIDTH: Length = "popover-width" => |t| t.length(SPACING) * 72.0,
    /// The popup's padding (shadcn's `p-4`).
    POPOVER_PADDING: Length = "popover-padding" => |t| t.length(SPACING) * 4.0,
    /// The gap between a `PopoverHeader`'s title and description.
    POPOVER_HEADER_GAP: Length = "popover-header-gap" => |t| t.length(SPACING),
    /// A `PopoverDescription`'s text.
    POPOVER_DESCRIPTION_FOREGROUND: Color = "popover-description-foreground" => |t| t.color(MUTED_FOREGROUND),
}

/// The props of [`Popover`].
#[derive(Debug, Clone, PartialEq, Props)]
pub struct PopoverProps {
    /// What opens it (a `Button`, usually): pressing it toggles the
    /// popover, which is placed against it. It is `expanded` while open.
    pub trigger: Prop<Option<Slot>>,
    /// Open (controlled): the app owns the value and changes it in
    /// `onOpenChange`. Left out, the popover keeps its own state.
    pub open: Prop<Option<bool>>,
    /// Open at first, while `open` is not set.
    #[prop(default)]
    pub default_open: Prop<bool>,
    /// The side of the trigger it goes on (`"bottom"` by default); the
    /// other side when only that one has room.
    #[prop(default = Placement::Bottom)]
    pub placement: Prop<Placement>,
    /// How it lines up with the trigger along that side (`"center"` by
    /// default; `start` and `end` follow the layout direction).
    #[prop(default)]
    pub align: Prop<Align>,
    /// Escape and the Android back button close it (default true); either
    /// way `onCancel` hears them.
    #[prop(default = true)]
    pub close_on_escape: Prop<bool>,
    /// A press outside closes it (default true); either way `onCancel`
    /// hears it.
    #[prop(default = true)]
    pub close_on_press_outside: Prop<bool>,
    /// Tailwind classes of the panel (the popup's box), after its own look:
    /// `w-80`, `p-0`, `gap-2`.
    pub class: Prop<Option<SharedString>>,
    /// The inline style of the panel, after its own look and classes
    /// (`{ width: 320 }`).
    #[prop(default)]
    pub style: Prop<Style>,
    /// The content, made on each opening.
    #[prop(lazy)]
    pub children: Prop<Children>,
}

/// The events of a [`Popover`]: `nsh/base`'s, passed on.
#[derive(Debug, Clone, PartialEq, Event)]
pub enum PopoverEvent {
    /// The user asks to open or close it (the trigger, a press outside,
    /// Escape or back), with why; set `open` to follow it.
    #[args(open, reason)]
    OpenChange(bool, OpenChangeReason),
    /// It is shown, its content made: refs inside it name their views, so
    /// focusing one here moves the keyboard focus into it.
    Open,
    /// It is gone, after its exit (whoever closed it).
    Close,
    /// The user tried to close it (Escape or back, a press outside),
    /// whether or not `closeOnEscape` / `closeOnPressOutside` let it.
    Cancel,
    /// It was placed: the side of the trigger it went on (the other side
    /// of `placement` when only that one had room), its frame in the root
    /// view and the room on that side.
    #[args(placement, bounds, available)]
    Position(Placement, Bounds<Points>, Size<Points>),
}

/// Rich content in a floating panel next to the element that opened it,
/// shadcn/ui's: a press on the trigger opens it, a press outside or Escape
/// closes it, and the focus goes back to the trigger. It does not block
/// the page. It fades, zooms and slides in from its trigger's side.
#[component]
pub fn Popover(props: PopoverProps, cx: Scope<PopoverEvent>) -> impl IntoElement {
    let t = theme();
    // The side it went on, which its entrance slides from: the requested
    // one until the core places it.
    let side = signal(props.placement.get_untracked());
    let animate = move || popup::animate(t);
    // The panel's look: content `nsh/base`'s popover places takes the
    // context of where it is, under it, not of where it was made.
    provide_context(PanelLook {
        class: props.class,
        style: props.style,
    });
    // Made on each opening: counted among the open popups while it is
    // there.
    let panel = move || {
        untracked(|| {
            popup::hold_open();
            popover_panel().child(props.children)
        })
    };
    // The open state, the app's or its own: kept here (`nsh/base`'s
    // popover is controlled) so a library `Button` as the trigger, which
    // takes the press itself, can open it too.
    let own_open = signal(props.default_open.get_untracked());
    let is_open = move || props.open.get().unwrap_or_else(|| own_open.get());
    let one = trigger::OnePress::new();
    let change = move |open: bool, reason: OpenChangeReason| {
        if reason == OpenChangeReason::TriggerPress && !one.first() {
            return;
        }
        if props.open.get_untracked().is_none() {
            own_open.set(open);
        }
        cx.emit(PopoverEvent::OpenChange(open, reason));
    };
    trigger::provide(
        move || change(!untracked(is_open), OpenChangeReason::TriggerPress),
        is_open,
    );
    base_popover()
        .trigger(props.trigger.place(|trigger| trigger))
        .open(move || Some(is_open()))
        .placement(props.placement)
        .align(props.align)
        .offset(move || t.length(POPOVER_OFFSET))
        .margin(move || t.length(POPOVER_MARGIN))
        .close_on_escape(props.close_on_escape)
        .close_on_press_outside(props.close_on_press_outside)
        .popup_style(move |s| {
            let s = popup::transitions(s, t);
            s.style_with(move |s| popup::origin(s, side.get()))
        })
        .popup_starting_class(move || popup::starting_class(side.get(), animate()))
        .popup_ending_class(move || popup::ending_class(animate()))
        .on_open_change(move |e: &BaseEvent| {
            if let BaseEvent::OpenChange(open, reason) = e {
                change(*open, *reason);
            }
        })
        .on_open(move || cx.emit(PopoverEvent::Open))
        .on_close(move || {
            side.set(props.placement.get_untracked());
            cx.emit(PopoverEvent::Close)
        })
        .on_cancel(move || cx.emit(PopoverEvent::Cancel))
        .on_position(move |e: &BaseEvent| {
            if let BaseEvent::Position(placement, bounds, available) = e {
                side.set(*placement);
                cx.emit(PopoverEvent::Position(*placement, *bounds, *available));
            }
        })
        .child(dynamic(panel))
}

/// The popover's `class` and `style`, for its panel.
#[derive(Clone, Copy)]
struct PanelLook {
    class: Prop<Option<SharedString>>,
    style: Prop<Style>,
}

/// The props of [`PopoverPanel`].
#[derive(Debug, Clone, PartialEq, Props)]
struct PopoverPanelProps {
    /// The popover's content.
    children: Prop<Children>,
}

/// A popover's panel: a component of its own so that the app's content,
/// under it, is not the popover's trigger (`trigger::clear`), as it would
/// be under `nsh/base`'s popover alone (the trigger is under it too).
#[component]
fn PopoverPanel(props: PopoverPanelProps, _cx: Scope<NoEvent>) -> impl IntoElement {
    provide_text_color();
    let t = theme();
    let look = use_context::<PanelLook>().expect("a popover's panel is in a popover");
    trigger::clear();
    popup::surface(view(), t)
        .flex_col()
        .w(move || t.length(POPOVER_WIDTH))
        .p(move || t.length(POPOVER_PADDING))
        .app_class(look.class)
        .inline_style(move || look.style.get())
        .child(props.children)
}

/// The props of [`PopoverHeader`].
#[derive(Debug, Clone, PartialEq, Props)]
pub struct PopoverHeaderProps {
    /// Its parts: a `PopoverTitle` and a `PopoverDescription`.
    pub children: Prop<Children>,
}

/// A column holding a popover's title and description.
#[component]
pub fn PopoverHeader(props: PopoverHeaderProps, _cx: Scope<NoEvent>) -> impl IntoElement {
    let t = theme();
    view()
        .flex_col()
        .gap(move || t.length(POPOVER_HEADER_GAP))
        .text_size(move || t.length(TEXT_SM))
        .line_height(move || line_height(t.length(TEXT_SM)))
        .child(props.children)
}

/// The props of [`PopoverTitle`].
#[derive(Debug, Clone, PartialEq, Props)]
pub struct PopoverTitleProps {
    /// The title: text.
    pub children: Prop<Children>,
}

/// A popover's title: medium weight, in the panel's size and color.
#[component]
pub fn PopoverTitle(props: PopoverTitleProps, _cx: Scope<NoEvent>) -> impl IntoElement {
    let t = theme();
    text("").font_weight(medium(t)).child(props.children)
}

/// The props of [`PopoverDescription`].
#[derive(Debug, Clone, PartialEq, Props)]
pub struct PopoverDescriptionProps {
    /// The description: text.
    pub children: Prop<Children>,
}

/// Muted text under a popover's title.
#[component]
pub fn PopoverDescription(props: PopoverDescriptionProps, _cx: Scope<NoEvent>) -> impl IntoElement {
    let t = theme();
    text("")
        .text_color(move || t.color(POPOVER_DESCRIPTION_FOREGROUND))
        .child(props.children)
}
