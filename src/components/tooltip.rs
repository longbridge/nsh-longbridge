//! `Tooltip`: shadcn/ui's tooltip on `nsh/base`'s `Tooltip` (when it shows
//! and hides, where it goes, the keyboard, the events). An element's
//! `tooltip` prop takes it. API: `docs/components/tooltip.md`.

use nsh_base::{OpenChangeReason, TooltipEvent as BaseEvent, tooltip as base_tooltip};
use nsh_core::{
    element::{__hint_tooltip, ElNode, Style, TextPartStyle},
    prelude::*,
};

use crate::{
    foundation::{motion, typography::line_height},
    theme::{
        seeds::SPACING,
        semantic::{BACKGROUND, FOREGROUND, RADIUS_MD},
        theme,
    },
    tokens,
};

tokens! {
    /// The bubble's fill: `foreground`, so it is inverted (dark on a light
    /// theme), as shadcn's.
    TOOLTIP: Color = "tooltip" => |t| t.color(FOREGROUND),
    /// The bubble's text.
    TOOLTIP_FOREGROUND: Color = "tooltip-foreground" => |t| t.color(BACKGROUND),
    /// The bubble's corner radius.
    TOOLTIP_RADIUS: Length = "tooltip-radius" => |t| t.length(RADIUS_MD),
    /// The bubble's horizontal padding (shadcn `px-3`).
    TOOLTIP_PADDING_X: Length = "tooltip-padding-x" => |t| t.length(SPACING) * 3.0,
    /// The bubble's vertical padding (shadcn `py-1.5`).
    TOOLTIP_PADDING_Y: Length = "tooltip-padding-y" => |t| t.length(SPACING) * 1.5,
    /// The text size: Tailwind's `text-xs`, explanatory text rather than a
    /// control's.
    TOOLTIP_FONT_SIZE: Length = "tooltip-font-size" => |t| t.px(12.0),
    /// The widest the bubble gets before its text wraps.
    TOOLTIP_MAX_WIDTH: Length = "tooltip-max-width" => |t| t.px(320.0),
    /// The gap between the element and the bubble.
    TOOLTIP_OFFSET: Length = "tooltip-offset" => |t| t.length(SPACING),
    /// How long the pointer rests on the element before the bubble shows,
    /// in milliseconds (a tooltip's `delay` wins).
    TOOLTIP_DELAY: Number = "tooltip-delay" => |t| t.value(500.0),
    /// Whether the bubble fades and zooms in and out (1) or shows and hides
    /// in one frame (0).
    TOOLTIP_ANIMATE: Number = "tooltip-animate" => |t| t.value(1.0),
}

/// What a `Tooltip` gives what is inside it: a `Kbd` there takes the
/// tooltip's colors.
#[derive(Debug, Clone, Copy)]
pub(crate) struct InTooltip;

/// Whether the current component is inside a `Tooltip`'s bubble.
pub(crate) fn in_tooltip() -> bool {
    use_context::<InTooltip>().is_some()
}

/// The look the bubble starts from when shown and ends at when hidden.
const HIDDEN_LOOK: &str = "opacity-0 scale-95";

/// The props of [`Tooltip`].
#[derive(Debug, Clone, PartialEq, Props)]
pub struct TooltipProps {
    /// The side of the element it goes on (`"top"` by default); the other
    /// side when only that one has room.
    #[prop(default = Placement::Top)]
    pub placement: Prop<Placement>,
    /// How it lines up with the element along that side (`"center"` by
    /// default; `start` and `end` follow the layout direction).
    #[prop(default)]
    pub align: Prop<Align>,
    /// Milliseconds the pointer rests on the element before it shows
    /// (`tooltip-delay`, 500, when left out); `0` at once. `Infinity`:
    /// neither the pointer nor the focus shows it, only `open`. The keyboard
    /// focus shows it at once whatever the delay.
    pub delay: Prop<Option<f64>>,
    /// Shown (controlled): the app owns the value and changes it in
    /// `onOpenChange`. Left out, the tooltip keeps its own state.
    pub open: Prop<Option<bool>>,
    /// Shown at first, while `open` is not set.
    #[prop(default)]
    pub default_open: Prop<bool>,
    /// No tooltip: it neither shows nor asks to.
    #[prop(default)]
    pub disabled: Prop<bool>,
    /// Tailwind classes of the label: the part each child sits in (a
    /// string child's text takes its text classes).
    pub label_class: Prop<Option<SharedString>>,
    /// The inline style of the label part.
    #[prop(default)]
    pub label_style: Prop<TextPartStyle>,
    /// What it says: text, a `Kbd`, other elements.
    pub children: Prop<Children>,
}

/// The events of a [`Tooltip`].
#[derive(Debug, Clone, PartialEq, Event)]
pub enum TooltipEvent {
    /// The user asks to show or hide it: the pointer rested on the element,
    /// or left or pressed it (`reason` `"hover"`); the keyboard moved the
    /// focus to it or away (`"focus"`); Escape (`"cancel"`). Not when the
    /// app changes `open`.
    #[args(open, reason)]
    OpenChange(bool, OpenChangeReason),
    /// It is shown.
    Open,
    /// It is gone, after its exit (whoever hid it).
    Close,
}

/// A hint bubble, shadcn/ui's: a line about an element shown while the
/// pointer rests on it or the keyboard focus is on it. Give it as the
/// element's `tooltip` (`<Button tooltip={<Tooltip>Save</Tooltip>}>`). It
/// is inverted (dark on a light theme), fades and zooms in, never takes the
/// pointer, and hardly ever shows on touch screens: never put what the
/// user must know only in a tooltip.
#[component]
pub fn Tooltip(props: TooltipProps, cx: Scope<TooltipEvent>) -> impl IntoElement {
    let t = theme();
    provide_context(InTooltip);
    let animate = move || t.number(TOOLTIP_ANIMATE) != 0.0;
    let font_size = move || t.length(TOOLTIP_FONT_SIZE);
    let root = base_tooltip()
        .placement(props.placement)
        .align(props.align)
        .delay(move || {
            props
                .delay
                .get()
                .unwrap_or_else(|| f64::from(t.number(TOOLTIP_DELAY)))
        })
        .offset(move || t.length(TOOLTIP_OFFSET))
        .open(props.open)
        .default_open(props.default_open)
        .disabled(props.disabled)
        .starting_class(move || animate().then(|| SharedString::from(HIDDEN_LOOK)))
        .ending_class(move || animate().then(|| SharedString::from(HIDDEN_LOOK)))
        .on_open_change(move |e: &BaseEvent| {
            if let BaseEvent::OpenChange(open, reason) = e {
                cx.emit(TooltipEvent::OpenChange(*open, *reason));
            }
        })
        .on_open(move || cx.emit(TooltipEvent::Open))
        .on_close(move || cx.emit(TooltipEvent::Close))
        .flex_row()
        .items_center()
        .gap(move || t.length(SPACING) * 2.0)
        .max_w(move || t.length(TOOLTIP_MAX_WIDTH))
        .px(move || t.length(TOOLTIP_PADDING_X))
        .py(move || t.length(TOOLTIP_PADDING_Y))
        .rounded(move || t.length(TOOLTIP_RADIUS))
        .bg(move || t.color(TOOLTIP))
        .text_color(move || t.color(TOOLTIP_FOREGROUND))
        .text_size(font_size)
        .line_height(move || line_height(font_size()))
        // The bubble is the trigger's child, so it would inherit the
        // trigger's text styles: a truncated label's tooltip would not wrap,
        // a title's would be bold. shadcn's, portaled, inherits none.
        .whitespace_normal()
        .font_weight(FontWeight::NORMAL)
        .no_underline()
        .normal_case()
        .letter_spacing(px(0.0));
    motion::transition_all(root, t).children_each(props.children, move |child, _| {
        // Shrinkable, so long text wraps at `tooltip-max-width`.
        text("")
            .flex_shrink(1.0)
            .min_w(px(0.0))
            .app_class(props.label_class)
            .inline_style(move || Style::from(props.label_style.get()))
            .child(child)
    })
}

/// As an element's `tooltip`, it is the hint layer itself, as `nsh/base`'s
/// `Tooltip` is (with its placement, delay and look), not an element
/// inside a hint layer of the core's.
impl IntoTooltip for TooltipElement {
    fn apply_tooltip(self, node: &mut ElNode) {
        __hint_tooltip(node, self.into_element());
    }
}
