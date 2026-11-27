//! `Slider`: a value, or a range of two, picked along a track, on
//! `nsh/base`'s `Slider` (the value and its bounds, steps and scale,
//! pointer, touch and keyboard changes, the thumbs' adjustable role, the
//! terminal's slider). This file adds the look (the `slider-*` tokens) and
//! the press area's size. API: `docs/components/slider.md`.

use nsh_base::slider as base_slider;
pub use nsh_base::{SliderScale, SliderValue};
use nsh_core::{
    element::{Style, StyleRefinement},
    geometry::Axis,
    prelude::*,
};

use crate::{
    foundation::{
        field::{self, Kind},
        motion, shadow,
        size::pointer,
    },
    theme::{
        Theme,
        seeds::SPACING,
        semantic::{
            BACKGROUND, CONTROL_HEIGHT_MD, CONTROL_HEIGHT_TOUCH_MD, FOREGROUND, MUTED,
            OPACITY_DISABLED, PRIMARY, RING, RING_OPACITY, RING_WIDTH, TOUCH_TARGET,
        },
        theme,
    },
    tokens,
};

tokens! {
    /// The press area across the slider: the thumb's size with a pointer,
    /// a finger's on touch screens.
    SLIDER_THICKNESS: Length = "slider-thickness" => |t| t.pointer_touch(t.length(SLIDER_THUMB_SIZE), t.length(TOUCH_TARGET)),
    /// A vertical slider's least height.
    SLIDER_VERTICAL_MIN_HEIGHT: Length = "slider-vertical-min-height" => |t| t.length(SPACING) * 44.0,
    /// The rail's thickness.
    SLIDER_TRACK_THICKNESS: Length = "slider-track-thickness" => |t| t.length(SPACING) * 1.5,
    /// The rail.
    SLIDER_TRACK: Color = "slider-track" => |t| t.color(MUTED),
    /// The filled stretch.
    SLIDER_RANGE: Color = "slider-range" => |t| t.color(PRIMARY),
    /// A thumb's size.
    SLIDER_THUMB_SIZE: Length = "slider-thumb-size" => |t| t.pointer_touch(t.length(CONTROL_HEIGHT_MD) * 0.5, t.length(CONTROL_HEIGHT_TOUCH_MD) * 0.5),
    /// A thumb's fill: white in both appearances.
    SLIDER_THUMB: Color = "slider-thumb" => |t| t.light_dark(t.color(BACKGROUND), t.color(FOREGROUND)),
    /// A thumb's edge.
    SLIDER_THUMB_BORDER: Color = "slider-thumb-border" => |t| t.color(SLIDER_RANGE),
    /// A thumb's shadow level (2: `sm`; 0 for none).
    SLIDER_THUMB_SHADOW: Number = "slider-thumb-shadow" => |t| t.value(2.0),
    /// The ring around a thumb under the pointer, dragged or focused.
    SLIDER_THUMB_RING: Color = "slider-thumb-ring" => |t| t.color(RING).opacity(t.number(RING_OPACITY)),
    /// The ring's width.
    SLIDER_THUMB_RING_WIDTH: Length = "slider-thumb-ring-width" => |t| t.length(RING_WIDTH) + 1.0,
}

/// The props of [`Slider`].
#[derive(Debug, Clone, PartialEq, Props)]
pub struct SliderProps {
    /// The value (controlled): a number, or `[start, end]` for two thumbs.
    /// The app sets the one `onChange` reports. Left out, the slider keeps
    /// its own, from `defaultValue`.
    pub value: Prop<Option<SliderValue>>,
    /// The first value while `value` is left out.
    #[prop(default)]
    pub default_value: Prop<SliderValue>,
    /// The smallest value.
    #[prop(default)]
    pub min: Prop<f64>,
    /// The largest value.
    #[prop(default = 100.0)]
    pub max: Prop<f64>,
    /// The step values snap to, from `min` (`max` always reachable); 0 for
    /// any value.
    #[prop(default = 1.0)]
    pub step: Prop<f64>,
    /// How values spread along the track: `linear` or `logarithmic`.
    #[prop(default)]
    pub scale: Prop<SliderScale>,
    /// `horizontal` (`min` at the start) or `vertical` (`min` at the
    /// bottom). Read when it is made.
    #[prop(default = Axis::Horizontal)]
    pub axis: Prop<Axis>,
    /// No changes, dimmed. Or'ed with an enclosing `Field`'s or
    /// `FieldSet`'s.
    #[prop(default)]
    pub disabled: Prop<bool>,
    /// What screen readers call the thumb (a range's end thumb).
    pub accessibility_label: Prop<Option<SharedString>>,
    /// What screen readers call a range's start thumb (default
    /// `accessibilityLabel`).
    pub start_accessibility_label: Prop<Option<SharedString>>,
    /// Tailwind classes of the rail, after the library's look.
    pub track_class: Prop<Option<SharedString>>,
    /// The inline style of the rail, merged last.
    #[prop(default)]
    pub track_style: Prop<Style>,
    /// Tailwind classes of the filled stretch.
    pub range_class: Prop<Option<SharedString>>,
    /// The inline style of the filled stretch, merged last.
    #[prop(default)]
    pub range_style: Prop<Style>,
    /// Tailwind classes of each thumb.
    pub thumb_class: Prop<Option<SharedString>>,
    /// The inline style of each thumb, merged last.
    #[prop(default)]
    pub thumb_style: Prop<Style>,
}

/// The events of a [`Slider`].
#[derive(Debug, Clone, PartialEq, Event)]
pub enum SliderEvent {
    /// The user changed the value (a press, a drag, a key, a screen
    /// reader), each time it changes: the value it asks for.
    #[args(value)]
    Change(SliderValue),
    /// The change ended (released, a key's step done): the value it ended
    /// at.
    #[args(value)]
    ChangeEnd(SliderValue),
}

/// A slider: a value, or a range of two (`[start, end]`), picked along a
/// thin rail by dragging its thumbs, pressing the rail or with the arrow
/// keys. The filled stretch runs from `min` to the thumb (between the
/// thumbs of a range). Under the pointer, dragged or focused, a thumb
/// shows a ring. On touch screens the slider is a finger tall.
#[component]
pub fn Slider(props: SliderProps, cx: Scope<SliderEvent>) -> impl IntoElement {
    let t = theme();
    let joined = field::join(cx.__host(), Kind::Inert, move || props.disabled.get());
    let disabled = computed(move || props.disabled.get() || joined.disabled());
    let vertical = props.axis.get_untracked() == Axis::Vertical;
    let thickness = move || t.length(SLIDER_THICKNESS);
    let rail = move || t.length(SLIDER_TRACK_THICKNESS);
    // The thumb never outgrows the press area.
    let thumb = move || t.length(SLIDER_THUMB_SIZE).clamp(px(0.0), thickness());

    // `nsh/base`'s track: the press area, the rail centered in it.
    let press_area = move || {
        let s = StyleRefinement::default().justify_center();
        let s = if vertical {
            s.flex_1().flex_row().w(thickness())
        } else {
            s.flex_col().h(thickness())
        };
        Style::from(s)
    };
    let rail_style = move || {
        let s = StyleRefinement::default()
            .rounded_full()
            .bg(t.color(SLIDER_TRACK));
        let s = if vertical { s.w(rail()) } else { s.h(rail()) };
        Style::from(s).then(&props.track_style.get())
    };
    let range_style = move || {
        let s = StyleRefinement::default()
            .rounded_full()
            .bg(t.color(SLIDER_RANGE));
        Style::from(s).then(&props.range_style.get())
    };
    let thumb_style = move || {
        let s = StyleRefinement::default()
            .size(thumb())
            .rounded_full()
            .bg(t.color(SLIDER_THUMB))
            .border(t.length(crate::theme::semantic::BORDER_WIDTH))
            .border_color(t.color(SLIDER_THUMB_BORDER))
            .shadow(shadow::box_shadows(
                t,
                shadow::level(t.number(SLIDER_THUMB_SHADOW)),
            ))
            .transition_shadow();
        let s = motion::timing(s, t);
        Style::from(s).then(&props.thumb_style.get())
    };
    let thumb_class = move || {
        let ring = (pointer() && !disabled.get()).then(|| ring_class(t));
        join_classes(ring, props.thumb_class.get())
    };

    let mut root = base_slider()
        .value(props.value)
        .default_value(props.default_value)
        .min(props.min)
        .max(props.max)
        .step(props.step)
        .scale(props.scale)
        .axis(props.axis.get_untracked())
        .disabled(disabled)
        .accessibility_label(props.accessibility_label)
        .start_accessibility_label(props.start_accessibility_label)
        .indicator_class(props.track_class)
        .range_class(props.range_class)
        .thumb_class(thumb_class)
        .on_change(move |value: &SliderValue| cx.emit(SliderEvent::Change(*value)))
        .on_change_end(move |value: &SliderValue| cx.emit(SliderEvent::ChangeEnd(*value)))
        .map(|s| {
            if vertical {
                s.flex_col()
                    .items_center()
                    .min_h(move || t.length(SLIDER_VERTICAL_MIN_HEIGHT))
                    .app_class(Some(SharedString::from("h-full")))
            } else {
                s.app_class(Some(SharedString::from("w-full")))
            }
        })
        .cursor(Cursor::Default)
        .disabled_style(move |s| s.opacity(move || t.number(OPACITY_DISABLED)));
    // The parts' styles follow the tokens: bindings, which the builder's
    // setters (styles built once) do not take.
    let parts = root.__element().__props_mut();
    parts.track_style = reactive(press_area);
    parts.indicator_style = reactive(rail_style);
    parts.range_style = reactive(range_style);
    parts.thumb_style = reactive(thumb_style);
    root
}

/// A style binding as a part's style prop.
fn reactive(style: impl Fn() -> Style + 'static) -> Prop<Style> {
    Prop::from(IntoReactive::<Style>::into_reactive(style))
}

/// The ring a thumb shows under the pointer, dragged and with the keyboard
/// focus: a class made from the tokens, as the states are `nsh/base`'s.
fn ring_class(t: Theme) -> String {
    let width = t.length(SLIDER_THUMB_RING_WIDTH).0;
    let color = t.color(SLIDER_THUMB_RING);
    let (light, dark) = (color.light.to_hex(), color.dark.to_hex());
    ["hover", "active", "focus-visible"]
        .map(|state| {
            format!("{state}:ring-[{width}px] {state}:ring-[{light}] dark:{state}:ring-[{dark}]")
        })
        .join(" ")
}

/// The library's classes, then the app's.
fn join_classes(ours: Option<String>, app: Option<SharedString>) -> Option<SharedString> {
    match (ours, app) {
        (None, app) => app,
        (Some(ours), None) => Some(ours.into()),
        (Some(ours), Some(app)) => Some(format!("{ours} {app}").into()),
    }
}
