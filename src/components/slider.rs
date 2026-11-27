//! `Slider`: a value, or a range of two, picked along a track, on
//! `nsh/base`'s `Slider` (the value and its bounds, steps and scale,
//! pointer, touch and keyboard changes, the thumbs' adjustable role, the
//! terminal's slider). This file adds the look (the `slider-*` tokens), the
//! press area's size, and the parts drawn around `nsh/base`'s slider: the
//! histogram above the track, the marks below it and the value bubbles over
//! the thumbs. API: `docs/components/slider.md`.

use std::{collections::HashMap, rc::Rc};

use nsh_base::slider as base_slider;
pub use nsh_base::{SliderScale, SliderValue};
use nsh_core::{
    TouchEvent, TouchPhase,
    element::{
        Arg, Callback, JsHost, JsIn, JsKind, JsOwned, JsValue, PropError, Style, StyleRefinement,
        TsTypes, layout_rtl,
    },
    geometry::Axis,
    prelude::*,
};

use crate::{
    components::tooltip::{TOOLTIP, TOOLTIP_FONT_SIZE, TOOLTIP_FOREGROUND},
    foundation::{
        class::Classes,
        field::{self, Kind},
        labels::{self, Label},
        motion, shadow,
        size::pointer,
        typography::{line_height, reset_inherited},
    },
    theme::{
        Theme,
        seeds::SPACING,
        semantic::{
            BACKGROUND, BORDER_WIDTH, CONTROL_HEIGHT_MD, CONTROL_HEIGHT_TOUCH_MD, FOREGROUND,
            MUTED, MUTED_FOREGROUND, OPACITY_DISABLED, PRIMARY, RADIUS_SM, RADIUS_XS, RING,
            RING_OPACITY, RING_WIDTH, TOUCH_TARGET,
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
    /// The histogram's height: its largest bar.
    SLIDER_HISTOGRAM_HEIGHT: Length = "slider-histogram-height" => |t| t.length(SPACING) * 10.0,
    /// The space between two bars, half on each side of a bucket's edge.
    SLIDER_HISTOGRAM_GAP: Length = "slider-histogram-gap" => |t| t.length(SPACING) * 0.5,
    /// The least height of a bar whose count is above 0.
    SLIDER_HISTOGRAM_MIN_BAR: Length = "slider-histogram-min-bar" => |t| t.length(SPACING) * 0.5,
    /// The radius of a bar's top corners.
    SLIDER_HISTOGRAM_RADIUS: Length = "slider-histogram-radius" => |t| t.length(RADIUS_XS),
    /// A bar outside the range.
    SLIDER_HISTOGRAM_BAR: Color = "slider-histogram-bar" => |t| t.color(MUTED_FOREGROUND).opacity(0.3),
    /// A bar between the thumbs (from `min` to a single thumb).
    SLIDER_HISTOGRAM_BAR_ACTIVE: Color = "slider-histogram-bar-active" => |t| t.color(SLIDER_RANGE),
    /// A mark's tick outside the range.
    SLIDER_MARK: Color = "slider-mark" => |t| t.color(MUTED_FOREGROUND).opacity(0.5),
    /// A mark's tick between the thumbs.
    SLIDER_MARK_ACTIVE: Color = "slider-mark-active" => |t| t.color(SLIDER_RANGE),
    /// A tick's length.
    SLIDER_MARK_SIZE: Length = "slider-mark-size" => |t| t.length(SPACING) * 1.5,
    /// The space between a tick and its label.
    SLIDER_MARK_GAP: Length = "slider-mark-gap" => |t| t.length(SPACING),
    /// A mark's label.
    SLIDER_MARK_LABEL: Color = "slider-mark-label" => |t| t.color(MUTED_FOREGROUND),
    /// A mark's label's text size (Tailwind's `text-xs`).
    SLIDER_MARK_FONT_SIZE: Length = "slider-mark-font-size" => |t| t.px(12.0),
    /// The value bubble's fill: the tooltip's.
    SLIDER_BUBBLE: Color = "slider-bubble" => |t| t.color(TOOLTIP),
    /// The value bubble's text.
    SLIDER_BUBBLE_FOREGROUND: Color = "slider-bubble-foreground" => |t| t.color(TOOLTIP_FOREGROUND),
    /// The value bubble's text size.
    SLIDER_BUBBLE_FONT_SIZE: Length = "slider-bubble-font-size" => |t| t.length(TOOLTIP_FONT_SIZE),
    /// The value bubble's corner radius.
    SLIDER_BUBBLE_RADIUS: Length = "slider-bubble-radius" => |t| t.length(RADIUS_SM),
    /// The value bubble's horizontal padding.
    SLIDER_BUBBLE_PADDING_X: Length = "slider-bubble-padding-x" => |t| t.length(SPACING) * 2.0,
    /// The value bubble's vertical padding.
    SLIDER_BUBBLE_PADDING_Y: Length = "slider-bubble-padding-y" => |t| t.length(SPACING),
    /// The space between a thumb and its bubble: above the finger on touch
    /// screens.
    SLIDER_BUBBLE_OFFSET: Length = "slider-bubble-offset" => |t| t.pointer_touch(t.length(SPACING) * 2.0, t.length(SPACING) * 6.0),
    /// Whether the bubble fades and zooms in and out (1) or shows and hides
    /// in one frame (0).
    SLIDER_BUBBLE_ANIMATE: Number = "slider-bubble-animate" => |t| t.value(1.0),
}

/// A mark below a slider's track: a tick at `value`, with an optional label
/// under it. TSX takes a number (a tick alone) or `{ value, label }`.
#[derive(Debug, Clone, PartialEq)]
pub struct SliderMark {
    /// Where the tick is.
    pub value: f64,
    /// The text under the tick.
    pub label: Option<SharedString>,
}

impl SliderMark {
    /// A tick at `value`, without a label.
    pub fn new(value: f64) -> Self {
        Self { value, label: None }
    }

    /// The mark with `label` under its tick.
    pub fn label(mut self, label: impl Into<SharedString>) -> Self {
        self.label = Some(label.into());
        self
    }
}

impl From<f64> for SliderMark {
    fn from(value: f64) -> Self {
        Self::new(value)
    }
}

impl JsValue for SliderMark {
    fn ts_type(types: &mut TsTypes) -> String {
        types.declare("SliderMark", |_| {
            "/** A mark below a slider's track: a number (a tick alone) or a tick with a label under it. */\n\
             export type SliderMark = number | { value: number; label?: string | null };\n"
                .to_owned()
        })
    }

    fn from_js(value: JsIn<'_>) -> Result<Self, PropError> {
        if let Some(n) = value.as_number() {
            return Ok(Self::new(n));
        }
        if value.kind() != JsKind::Object {
            return Err(value.expected("a number or `{ value, label }`"));
        }
        let at = value.field("value")?;
        let Some(n) = at.get().as_number() else {
            return Err(value.expected("a mark's `value`, a number"));
        };
        let label = value.field("label")?;
        let label = label.get().as_string().map(SharedString::from);
        Ok(Self { value: n, label })
    }

    fn to_js<'a>(&self, host: &'a dyn JsHost) -> Result<JsOwned<'a>, PropError> {
        let o = JsOwned::new(host, host.new_object()?);
        host.set_field(o.get().raw(), "value", self.value.to_js(host)?.into_raw())?;
        host.set_field(
            o.get().raw(),
            "label",
            self.label.clone().to_js(host)?.into_raw(),
        )?;
        Ok(o)
    }
}

/// The app's text for a value in a slider's bubble: a Rust function, or a
/// TSX `(value: number) => string`.
#[derive(Clone)]
pub struct SliderFormat(Format);

#[derive(Clone)]
enum Format {
    Rust(Rc<dyn Fn(f64) -> SharedString>),
    Js(Callback<f64, SharedString>),
}

impl SliderFormat {
    /// The text `f` gives a value (`|v| format!("{v}%").into()`).
    pub fn new(f: impl Fn(f64) -> SharedString + 'static) -> Self {
        Self(Format::Rust(Rc::new(f)))
    }
}

impl std::fmt::Debug for SliderFormat {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self.0 {
            Format::Rust(_) => "SliderFormat(Rust)",
            Format::Js(_) => "SliderFormat(Js)",
        })
    }
}

/// A Rust function is the same while it is the same closure; a JS function
/// set again (a new arrow function) is a new one, asked again.
impl PartialEq for SliderFormat {
    fn eq(&self, other: &Self) -> bool {
        match (&self.0, &other.0) {
            (Format::Rust(a), Format::Rust(b)) => Rc::ptr_eq(a, b),
            _ => false,
        }
    }
}

impl JsValue for SliderFormat {
    fn ts_type(_types: &mut TsTypes) -> String {
        "((value: number) => string)".into()
    }

    fn from_js(value: JsIn<'_>) -> Result<Self, PropError> {
        if value.kind() != JsKind::Function {
            return Err(value.expected("a function of the value, giving its text"));
        }
        <Callback<f64, SharedString> as Arg>::from_js(value).map(|f| Self(Format::Js(f)))
    }

    fn to_js<'a>(&self, host: &'a dyn JsHost) -> Result<JsOwned<'a>, PropError> {
        Ok(JsOwned::new(host, host.undefined()))
    }
}

/// When a slider shows its value in a bubble above a thumb.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PropEnum)]
pub enum SliderBubble {
    /// Never (the value is the app's text beside the slider).
    #[default]
    None,
    /// While the user is on a thumb: under the pointer, dragged, or with
    /// the keyboard focus.
    Auto,
    /// Always, above every thumb.
    Always,
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
    /// How many results fall in each of equal stretches of the track, from
    /// `min` to `max`: bars above the track, those between the thumbs in
    /// the range's color. Horizontal sliders only.
    #[prop(default)]
    pub histogram: Prop<Vec<f64>>,
    /// Ticks below the track at these values, each with an optional label
    /// under it. Horizontal sliders only.
    #[prop(default)]
    pub marks: Prop<Vec<SliderMark>>,
    /// When the value shows in a bubble above its thumb: `none` (default),
    /// `auto` (under the pointer, dragged, with the keyboard focus) or
    /// `always`. Horizontal sliders only.
    #[prop(default)]
    pub bubble: Prop<SliderBubble>,
    /// The bubble's text for a value (default: the number with the step's
    /// decimals).
    pub format_value: Prop<Option<SliderFormat>>,
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
    /// Tailwind classes of the histogram's box (its height: `h-16`).
    pub histogram_class: Prop<Option<SharedString>>,
    /// The inline style of the histogram's box, merged last.
    #[prop(default)]
    pub histogram_style: Prop<Style>,
    /// Tailwind classes of the row of marks.
    pub marks_class: Prop<Option<SharedString>>,
    /// The inline style of the row of marks, merged last.
    #[prop(default)]
    pub marks_style: Prop<Style>,
    /// Tailwind classes of each value bubble.
    pub bubble_class: Prop<Option<SharedString>>,
    /// The inline style of each value bubble, merged last.
    #[prop(default)]
    pub bubble_style: Prop<Style>,
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

/// One of a slider's thumbs: a range's start, or the end (a single value's
/// only thumb).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Thumb {
    Start,
    End,
}

/// The track's values: where a value sits along it and its default text.
/// `nsh/base`'s rules (its `numeric::Range`, private to it), repeated: a
/// non-finite `min` is 0, a `max` below `min` is `min`, and a logarithmic
/// scale needs `min > 0`.
#[derive(Debug, Clone, Copy, PartialEq)]
struct Track {
    min: f64,
    max: f64,
    step: f64,
    log: bool,
}

impl Track {
    fn new(min: f64, max: f64, step: f64, scale: SliderScale) -> Self {
        let min = if min.is_finite() { min } else { 0.0 };
        let max = if max.is_finite() && max >= min {
            max
        } else {
            min
        };
        let step = if step.is_finite() && step > 0.0 {
            step
        } else {
            0.0
        };
        let log = scale == SliderScale::Logarithmic && min > 0.0 && max > min;
        Self {
            min,
            max,
            step,
            log,
        }
    }

    /// `v` held to the track; `NaN` is `min`.
    fn clamp(&self, v: f64) -> f64 {
        if v.is_nan() {
            self.min
        } else {
            v.clamp(self.min, self.max)
        }
    }

    /// The value as the slider shows it: held to the track, a range's ends
    /// in order.
    fn held(&self, value: SliderValue) -> SliderValue {
        match value {
            SliderValue::Single(v) => SliderValue::Single(self.clamp(v)),
            SliderValue::Range(a, b) => {
                let (a, b) = (self.clamp(a), self.clamp(b));
                SliderValue::Range(a.min(b), a.max(b))
            }
        }
    }

    /// Where `v` sits along the track, from 0 at `min` to 1 at `max`.
    fn fraction(&self, v: f64) -> f64 {
        let v = self.clamp(v);
        if self.max <= self.min {
            return 0.0;
        }
        let f = if self.log {
            (v / self.min).ln() / (self.max / self.min).ln()
        } else {
            (v - self.min) / (self.max - self.min)
        };
        f.clamp(0.0, 1.0)
    }

    /// Whether `v` lies on the track.
    fn contains(&self, v: f64) -> bool {
        v.is_finite() && v >= self.min && v <= self.max
    }

    /// `v` moved by `steps` steps (a hundredth of the track without a
    /// step), held to the track: a key's neighbor, asked of the app's
    /// format ahead.
    fn step_by(&self, v: f64, steps: f64) -> f64 {
        let step = if self.step > 0.0 {
            self.step
        } else {
            (self.max - self.min) * 0.01
        };
        self.round(self.clamp(v + steps * step))
    }

    /// `v` rounded to the decimals of the step and `min`.
    fn round(&self, v: f64) -> f64 {
        let scale = 10f64.powi(self.decimals());
        (v * scale).round() / scale
    }

    fn decimals(&self) -> i32 {
        decimals(self.step).max(decimals(self.min))
    }

    /// `v` as text, with the decimals of the step and `min` and no trailing
    /// zeros (`2.5`, `3`): what screen readers say for a thumb.
    fn format(&self, v: f64) -> String {
        let d = self.decimals() as usize;
        let text = format!("{v:.d$}");
        let text = if text.contains('.') {
            text.trim_end_matches('0').trim_end_matches('.').to_owned()
        } else {
            text
        };
        if text == "-0" { "0".to_owned() } else { text }
    }
}

/// How many decimals `x` has (at most 10).
fn decimals(x: f64) -> i32 {
    if !x.is_finite() {
        return 0;
    }
    let mut v = x.abs();
    let mut d = 0;
    while d < 10 && (v - v.round()).abs() > 1e-9 * v.max(1.0) {
        v *= 10.0;
        d += 1;
    }
    d
}

/// A value's key in the format's answers.
fn key(v: f64) -> u64 {
    // -0 and 0 are one value.
    (v + 0.0).to_bits()
}

/// What the library's parts share: the value they show, where it sits, the
/// slider's length and what the user is doing with the thumbs.
#[derive(Clone, Copy)]
struct Ctl {
    track: Memo<Track>,
    /// The value shown, held to the track: a drag's, else the app's, else
    /// the slider's own.
    shown: Memo<SliderValue>,
    /// A horizontal slider in a right-to-left layout: `min` at the right.
    mirrored: Memo<bool>,
    /// The slider's length (the rail's: the slider's whole width).
    width: Signal<f32>,
}

impl Ctl {
    /// Where `thumb` sits, from 0 at `min` to 1 at `max` (tracked).
    fn fraction(self, thumb: Thumb) -> f64 {
        let value = self.shown.get();
        let v = match thumb {
            Thumb::Start => value.start(),
            Thumb::End => value.end(),
        };
        self.track.get().fraction(v)
    }

    /// The stretch the range covers, from 0 to 1 (tracked): between a
    /// range's thumbs, from `min` to a single thumb.
    fn span(self) -> (f64, f64) {
        let end = self.fraction(Thumb::End);
        match self.shown.get() {
            SliderValue::Range(..) => (self.fraction(Thumb::Start), end),
            SliderValue::Single(_) => (0.0, end),
        }
    }

    /// `f` along the track as a fraction of the width from the left: the
    /// same, or mirrored.
    fn left(self, f: f64) -> f32 {
        let f = f as f32;
        if self.mirrored.get() { 1.0 - f } else { f }
    }

    /// The thumbs the value has.
    fn thumbs(self) -> &'static [Thumb] {
        if self.shown.get().is_range() {
            &[Thumb::Start, Thumb::End]
        } else {
            &[Thumb::End]
        }
    }
}

/// A slider: a value, or a range of two (`[start, end]`), picked along a
/// thin rail by dragging its thumbs, pressing the rail or with the arrow
/// keys. The filled stretch runs from `min` to the thumb (between the
/// thumbs of a range). Under the pointer, dragged or focused, a thumb
/// shows a ring. On touch screens the slider is a finger tall. A
/// horizontal slider can show how results spread over its range
/// (`histogram`: bars above the track, those in range in its color), ticks
/// with labels below it (`marks`), and its value in a bubble above a thumb
/// (`bubble`, `formatValue`).
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

    // The value the library's parts show: a drag's from its first change,
    // heard here in the same frame, until the app has answered its last
    // one (as `nsh/base` shows its thumbs); then the app's, else the
    // slider's own.
    let own = signal(props.default_value.get_untracked());
    let pending = signal(None::<SliderValue>);
    // Bumped by each change: a release lets go of the drag's value only if
    // no change came since.
    let changes = stored(0u64);
    let track = computed(move || {
        Track::new(
            props.min.get(),
            props.max.get(),
            props.step.get(),
            props.scale.get(),
        )
    });
    let shown = computed(move || {
        let raw = pending
            .get()
            .or_else(|| props.value.get())
            .unwrap_or_else(|| own.get());
        track.get().held(raw)
    });
    let ctl = Ctl {
        track,
        shown,
        mirrored: computed(move || !vertical && layout_rtl()),
        width: signal(0.0),
    };

    // What the user is doing with the thumbs, for `bubble="auto"` (NSH
    // gaps 4: `nsh/base` keeps its thumbs' states to itself).
    let hovered = signal(None::<Thumb>);
    let dragged = signal(None::<Thumb>);
    let pressed = signal(false);
    let keyboard = signal(false);
    let focused = signal(false);
    // The thumb the last change moved: with the keyboard, the focused one
    // (NSH gaps 8: `nsh/base` keeps which thumb has the focus to itself).
    let moved_last = signal(None::<Thumb>);
    let owner = Owner::current();
    // The parts beside `nsh/base`'s slider ask for the column around it;
    // without them the root is `nsh/base`'s slider itself, as it always
    // was. Decided when the slider is made (its root cannot change kind).
    let wrapped = !vertical
        && untracked(|| {
            !props.histogram.with(Vec::is_empty)
                || !props.marks.with(Vec::is_empty)
                || props.bubble.get() != SliderBubble::None
        });

    let on_change = move |value: &SliderValue| {
        let before = shown.get_untracked();
        let next = track.get_untracked().held(*value);
        changes.with(|n| *n += 1);
        if props.value.get_untracked().is_none() {
            own.set(*value);
        }
        pending.set(Some(*value));
        // The thumb that moved is the one dragged (a key's change ends at
        // once).
        let moved = match (before, next) {
            (SliderValue::Range(a, _), SliderValue::Range(c, _)) if a != c => Thumb::Start,
            _ => Thumb::End,
        };
        if dragged.get_untracked() != Some(moved) {
            dragged.set(Some(moved));
        }
        if moved_last.get_untracked() != Some(moved) {
            moved_last.set(Some(moved));
        }
        cx.emit(SliderEvent::Change(*value));
    };
    let on_change_end = move |value: &SliderValue| {
        if dragged.get_untracked().is_some() && !pressed.get_untracked() {
            dragged.set(None);
        }
        let at = changes.get();
        owner.run(|| {
            after_answer(move || {
                if changes.get() == at {
                    pending.set(None);
                }
            });
        });
        cx.emit(SliderEvent::ChangeEnd(*value));
    };

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
            .border(t.length(BORDER_WIDTH))
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

    let mut slider = base_slider()
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
        .on_change(on_change)
        .on_change_end(on_change_end)
        .map(|s| {
            if wrapped {
                // In the row, which the column sizes.
                s.app_class(Some(SharedString::from("w-full")))
            } else if vertical {
                s.flex_col()
                    .items_center()
                    .min_h(move || t.length(SLIDER_VERTICAL_MIN_HEIGHT))
                    .app_class(Some(SharedString::from("h-full")))
            } else {
                // Beside a label in a horizontal field, an equal share of the row.
                s.app_class(move || {
                    Some(SharedString::from(if joined.in_row() {
                        "w-full min-w-0 flex-1"
                    } else {
                        "w-full"
                    }))
                })
            }
        })
        .cursor(Cursor::Default);
    // The parts' styles follow the tokens: bindings, which the builder's
    // setters (styles built once) do not take.
    let parts = slider.__element().__props_mut();
    parts.track_style = reactive(press_area);
    parts.indicator_style = reactive(rail_style);
    parts.range_style = reactive(range_style);
    parts.thumb_style = reactive(thumb_style);
    if !wrapped {
        return slider
            .disabled_style(move |s| s.opacity(move || t.number(OPACITY_DISABLED)))
            .into_element();
    }

    // `nsh/base`'s slider, with the bubbles over it.
    let row = view()
        .relative()
        .w_full()
        .on_layout(move |frame: &Bounds<Points>| {
            let w = frame.size.width.0;
            if ctl.width.get_untracked() != w {
                ctl.width.set(w);
            }
        })
        .child(slider);
    // Which thumb the pointer is over: the thumb's circle, the end's first
    // (drawn over the start's).
    let hit = move |at: Point<Points>| {
        let size = f64::from(thumb().0);
        let w = f64::from(ctl.width.get_untracked());
        let middle = f64::from(thickness().0) / 2.0;
        let (x, y) = (f64::from(at.x.0), f64::from(at.y.0));
        if (y - middle).abs() > size / 2.0 {
            return None;
        }
        untracked(|| {
            ctl.thumbs().iter().rev().copied().find(|&which| {
                let cx = f64::from(ctl.left(ctl.fraction(which))) * w;
                (x - cx).abs() <= size / 2.0
            })
        })
    };
    let row = row
        .on_mouse_move(move |e: &MouseEvent| {
            let over = hit(e.position);
            if hovered.get_untracked() != over {
                hovered.set(over);
            }
        })
        .on_mouse_leave(move |_: &MouseEvent| hovered.set(None))
        // A press (desktops report the mouse's as a touch): a single thumb
        // is the one it moves; a range's shows once it moves one.
        .on_touch(move |e: &TouchEvent| match e.phase {
            TouchPhase::Start => {
                pressed.set(true);
                keyboard.set(false);
                moved_last.set(None);
                if !untracked(|| ctl.shown.get().is_range()) {
                    dragged.set(Some(Thumb::End));
                } else if let Some(over) = hovered.get_untracked() {
                    dragged.set(Some(over));
                }
            }
            TouchPhase::End | TouchPhase::Cancel => {
                pressed.set(false);
                dragged.set(None);
            }
            TouchPhase::Move => {}
        })
        .on_key_down(move |_: &KeyEvent| {
            if !keyboard.get_untracked() {
                keyboard.set(true);
            }
        })
        .child(bubbles(
            t,
            props.clone(),
            ctl,
            disabled,
            Pointing {
                hovered,
                dragged,
                keyboard,
                focused,
                moved_last,
            },
        ));

    let root_ref = element_ref();
    root_ref.on_focus_in(move || {
        focused.set(true);
        // Focused by the keyboard, not by a pointer's press.
        if !pressed.get_untracked() && hovered.get_untracked().is_none() {
            keyboard.set(true);
        }
    });
    root_ref.on_focus_out(move || {
        focused.set(false);
        keyboard.set(false);
        moved_last.set(None);
    });

    let overlap = move || (thickness() - thumb()) * 0.5;
    view()
        .element_ref(&root_ref)
        .flex_col()
        // Beside a label in a horizontal field, an equal share of the row.
        .app_class(move || {
            Some(SharedString::from(if joined.in_row() {
                "w-full min-w-0 flex-1"
            } else {
                "w-full"
            }))
        })
        .cursor(Cursor::Default)
        .opacity(move || {
            if disabled.get() {
                t.number(OPACITY_DISABLED)
            } else {
                1.0
            }
        })
        .child({
            let p = props.clone();
            when(
                move || !props.histogram.with(Vec::is_empty),
                move || histogram(t, p.clone(), ctl, overlap),
            )
        })
        .child(row)
        .child(when(
            move || {
                props
                    .marks
                    .with(|m| m.iter().any(|m| track.get().contains(m.value)))
            },
            {
                let p = props.clone();
                move || marks(t, p.clone(), ctl, overlap)
            },
        ))
        .into_element()
}

/// The bars above the track, the active copy clipped to the range.
fn histogram(
    t: Theme,
    props: SliderProps,
    ctl: Ctl,
    overlap: impl Fn() -> Points + Copy + 'static,
) -> impl IntoElement {
    let counts = props.histogram;
    let largest = computed(move || {
        counts.with(|c| {
            c.iter()
                .copied()
                .filter(|v| v.is_finite())
                .fold(0.0f64, f64::max)
        })
    });
    let buckets = computed(move || (0..counts.with(Vec::len)).collect::<Vec<usize>>());
    let width = ctl.width;
    // One copy of the bars, `active` in the range's color: bucket `i` of
    // `n` from `i / n` of the track to `(i + 1) / n`, inset by half the gap.
    let bars = move |active: bool| {
        view()
            .absolute()
            .top(px(0.0))
            .bottom(px(0.0))
            .w(move || px(width.get()))
            .child(each(
                move || buckets.get(),
                |i: &usize| *i,
                move |i: ReadSignal<usize>, _| {
                    let n = move || counts.with(Vec::len).max(1) as f32;
                    let share = move || {
                        let count = counts.with(|c| c.get(i.get()).copied().unwrap_or(0.0));
                        let largest = largest.get();
                        if count.is_finite() && count > 0.0 && largest > 0.0 {
                            Some((count / largest) as f32)
                        } else {
                            None
                        }
                    };
                    let half_gap = move || t.length(SLIDER_HISTOGRAM_GAP) * 0.5;
                    view()
                        .absolute()
                        .top(px(0.0))
                        .bottom(px(0.0))
                        .w(move || relative(1.0 / n()))
                        .style_with(move |s| {
                            let at = i.get() as f32 / n();
                            if ctl.mirrored.get() {
                                s.right(relative(at))
                            } else {
                                s.left(relative(at))
                            }
                        })
                        .flex_col()
                        .justify_end()
                        .px(half_gap)
                        .child(
                            view()
                                .w_full()
                                .h(move || relative(share().unwrap_or(0.0)))
                                .min_h(move || {
                                    if share().is_some() {
                                        t.length(SLIDER_HISTOGRAM_MIN_BAR)
                                    } else {
                                        px(0.0)
                                    }
                                })
                                .rounded_t(move || t.length(SLIDER_HISTOGRAM_RADIUS))
                                .bg(move || {
                                    t.color(if active {
                                        SLIDER_HISTOGRAM_BAR_ACTIVE
                                    } else {
                                        SLIDER_HISTOGRAM_BAR
                                    })
                                }),
                        )
                },
            ))
    };
    // The range's stretch of the box, which shows the active copy placed
    // as the muted one is.
    let window = view()
        .absolute()
        .top(px(0.0))
        .bottom(px(0.0))
        .overflow_hidden()
        .style_with(move |s| {
            let (a, b) = ctl.span();
            let (from, to) = (ctl.left(a), ctl.left(b));
            let (left, right) = (from.min(to), from.max(to));
            s.left(relative(left)).w(relative(right - left))
        })
        .child(bars(true).style_with(move |s| {
            let (a, b) = ctl.span();
            let left = ctl.left(a).min(ctl.left(b));
            s.left(px(-left * width.get()))
        }));
    view()
        .relative()
        .w_full()
        .h(move || t.length(SLIDER_HISTOGRAM_HEIGHT))
        // Down to the top of the thumbs, over the press area's margin.
        .mb(move || -overlap())
        .pointer_events(PointerEvents::None)
        .accessible(false)
        .accessibility_elements_hidden(true)
        .app_class(props.histogram_class)
        .inline_style(move || props.histogram_style.get())
        .child(bars(false).left(px(0.0)))
        .child(window)
}

/// The ticks and labels below the track.
fn marks(
    t: Theme,
    props: SliderProps,
    ctl: Ctl,
    overlap: impl Fn() -> Points + Copy + 'static,
) -> impl IntoElement {
    let shown = computed(move || {
        let track = ctl.track.get();
        props.marks.with(|m| {
            m.iter()
                .filter(|m| track.contains(m.value))
                .cloned()
                .collect::<Vec<_>>()
        })
    });
    let labelled = computed(move || shown.with(|m| m.iter().any(|m| m.label.is_some())));
    let font_size = move || t.length(SLIDER_MARK_FONT_SIZE);
    let tick = move || t.length(SLIDER_MARK_SIZE);
    let width = ctl.width;
    // Each label's width, by its mark's value, as laid out.
    let widths = signal(HashMap::<u64, f32>::new());
    // Where the labels drawn start: centered on their ticks, held inside
    // the ends; the two at the ends always, one between them unless it
    // would come within `slider-mark-gap` of the one drawn before it or of
    // the last. None until every label is measured.
    let placed = computed(move || {
        let w = width.get();
        let track = ctl.track.get();
        let gap = t.length(SLIDER_MARK_GAP).0;
        let mut labels = Vec::new();
        let measured = shown.with(|m| {
            m.iter()
                .filter(|m| m.label.as_ref().is_some_and(|l| !l.is_empty()))
                .all(|m| {
                    let k = key(m.value);
                    let Some(own) = widths.with(|ws| ws.get(&k).copied()) else {
                        return false;
                    };
                    let tick = ctl.left(track.fraction(m.value)) * w;
                    let x = tick - own / 2.0;
                    labels.push((tick, x.min(w - own).max(0.0), own, k));
                    true
                })
        });
        let mut out = HashMap::new();
        if !measured {
            return out;
        }
        // In the ticks' order: a label held in at an end may start before
        // its neighbour's.
        labels.sort_by(|a, b| a.0.total_cmp(&b.0));
        let last = labels.last().map_or(0.0, |l| l.1);
        let mut edge = f32::NEG_INFINITY;
        for (i, &(_, x, own, k)) in labels.iter().enumerate() {
            let end = i == 0 || i + 1 == labels.len();
            if end || (x >= edge + gap && x + own + gap <= last) {
                out.insert(k, x);
                edge = x + own;
            }
        }
        out
    });
    view()
        .relative()
        .w_full()
        .mt(move || -overlap())
        .h(move || {
            if labelled.get() {
                tick() + t.length(SLIDER_MARK_GAP) + line_height(font_size())
            } else {
                tick()
            }
        })
        .pointer_events(PointerEvents::None)
        .accessible(false)
        .accessibility_elements_hidden(true)
        .app_class(props.marks_class)
        .inline_style(move || props.marks_style.get())
        .child(each(
            move || shown.get(),
            |m: &SliderMark| key(m.value),
            move |mark: ReadSignal<SliderMark>, _| {
                let at = computed(move || {
                    let f = ctl.track.get().fraction(mark.with(|m| m.value));
                    (f, ctl.left(f))
                });
                let active = move || {
                    let (a, b) = ctl.span();
                    let f = at.get().0;
                    f >= a - 1e-9 && f <= b + 1e-9
                };
                let at_x = move || placed.with(|p| p.get(&mark.with(|m| key(m.value))).copied());
                // A full-width row the label is placed in by its start, so
                // it keeps its own width (never wraps) and is held inside
                // the slider's ends.
                let label = move || {
                    view()
                        .absolute()
                        .left(px(0.0))
                        .right(px(0.0))
                        .top(move || tick() + t.length(SLIDER_MARK_GAP))
                        .flex_row()
                        .child(
                            reset_inherited(text(move || {
                                mark.with(|m| m.label.clone().unwrap_or_default())
                            }))
                            .flex_shrink_0()
                            .whitespace_nowrap()
                            .tabular_nums()
                            .text_size(font_size)
                            .line_height(move || line_height(font_size()))
                            .text_color(move || t.color(SLIDER_MARK_LABEL))
                            .on_layout(move |frame: &Bounds<Points>| {
                                let w = frame.size.width.0;
                                let k = mark.with_untracked(|m| key(m.value));
                                if widths.with_untracked(|ws| ws.get(&k) != Some(&w)) {
                                    widths.update(|ws| {
                                        ws.insert(k, w);
                                    });
                                }
                            })
                            // Hidden until measured (no frame off its place)
                            // and while it would crowd its neighbours.
                            .opacity(move || if at_x().is_some() { 1.0f32 } else { 0.0 })
                            .ml(move || px(at_x().unwrap_or(0.0))),
                        )
                };
                let tick_view = view()
                    .absolute()
                    .top(px(0.0))
                    .h(tick)
                    .w(move || t.length(BORDER_WIDTH))
                    .style_with(move |s| s.left(relative(at.get().1)))
                    // Centered on its value, in layout (a translation would
                    // leave the box where a test or a hit sees it).
                    .ml(move || t.length(BORDER_WIDTH) * -0.5)
                    .bg(move || {
                        t.color(if active() {
                            SLIDER_MARK_ACTIVE
                        } else {
                            SLIDER_MARK
                        })
                    });
                view()
                    .absolute()
                    .left(px(0.0))
                    .right(px(0.0))
                    .top(px(0.0))
                    .bottom(px(0.0))
                    .child(tick_view)
                    .child(when(
                        move || mark.with(|m| m.label.as_ref().is_some_and(|l| !l.is_empty())),
                        label,
                    ))
            },
        ))
}

/// What the user is doing with the thumbs, which `bubble="auto"` follows.
#[derive(Clone, Copy)]
struct Pointing {
    /// The thumb under the pointer.
    hovered: Signal<Option<Thumb>>,
    /// The thumb a press or a drag holds.
    dragged: Signal<Option<Thumb>>,
    /// The focus came, or a key went, after no pointer's press.
    keyboard: Signal<bool>,
    /// The focus is on a thumb.
    focused: Signal<bool>,
    /// The thumb the last change moved.
    moved_last: Signal<Option<Thumb>>,
}

/// The value bubbles over the thumbs: one per thumb, and a range's two in
/// one once they would touch.
fn bubbles(
    t: Theme,
    props: SliderProps,
    ctl: Ctl,
    disabled: Memo<bool>,
    pointing: Pointing,
) -> impl IntoElement {
    let Pointing {
        hovered,
        dragged,
        keyboard,
        focused,
        moved_last,
    } = pointing;
    // The app's format: a Rust function now; a JS function's answers, asked
    // ahead and kept while it is the same function (NSH gaps 7).
    let answers = signal(HashMap::<u64, SharedString>::new());
    let owner = Owner::current();
    effect(move || {
        let Some(SliderFormat(Format::Js(f))) = props.format_value.get() else {
            untracked(|| answers.update(HashMap::clear));
            return;
        };
        untracked(|| answers.update(HashMap::clear));
        let asked = stored(std::collections::HashSet::<u64>::new());
        effect(move || {
            if props.bubble.get() == SliderBubble::None {
                return;
            }
            let track = ctl.track.get();
            let value = ctl.shown.get();
            let mut wanted = Vec::new();
            for v in [value.start(), value.end()] {
                for v in [v, track.step_by(v, -1.0), track.step_by(v, 1.0)] {
                    if asked.with(|a| a.insert(key(v))) {
                        wanted.push(v);
                    }
                }
            }
            if wanted.is_empty() {
                return;
            }
            let calls: Vec<_> = wanted.iter().map(|v| f.call_async(*v)).collect();
            owner.run(|| {
                spawn(async move {
                    let mut got = Vec::new();
                    for (v, call) in wanted.into_iter().zip(calls) {
                        if let Ok(text) = call.await {
                            got.push((key(v), text));
                        }
                    }
                    if !got.is_empty() {
                        answers.update(|a| a.extend(got));
                    }
                });
            });
        });
    });
    let text_of = move |v: f64| -> Option<SharedString> {
        match props.format_value.get() {
            None => Some(ctl.track.get().format(v).into()),
            Some(SliderFormat(Format::Rust(f))) => Some(f(v)),
            Some(SliderFormat(Format::Js(_))) => answers.with(|a| a.get(&key(v)).cloned()),
        }
    };
    let value_of = move |which: Thumb| {
        let value = ctl.shown.get();
        match which {
            Thumb::Start => value.start(),
            Thumb::End => value.end(),
        }
    };
    // Whether `which`'s bubble is wanted, before a range's two are joined.
    // With the keyboard, a range's focused thumb is known once a key moves
    // it (NSH gaps 8): until then neither shows.
    let wanted = move |which: Thumb| match props.bubble.get() {
        SliderBubble::None => false,
        SliderBubble::Always => true,
        SliderBubble::Auto => {
            let keyed = keyboard.get()
                && focused.get()
                && (!ctl.shown.get().is_range() || moved_last.get() == Some(which));
            !disabled.get()
                && (hovered.get() == Some(which) || dragged.get() == Some(which) || keyed)
        }
    };
    let widths = (signal(0.0f32), signal(0.0f32));
    let width_of = move |which: Thumb| match which {
        Thumb::Start => widths.0,
        Thumb::End => widths.1,
    };
    // A range's bubbles would touch: one between the thumbs shows both.
    let joined = computed(move || {
        if !ctl.shown.get().is_range() {
            return false;
        }
        let w = ctl.width.get();
        let a = ctl.left(ctl.fraction(Thumb::Start)) * w;
        let b = ctl.left(ctl.fraction(Thumb::End)) * w;
        let room = (widths.0.get() + widths.1.get()) / 2.0 + t.length(SPACING).0;
        (b - a).abs() < room
    });
    let animate = move || t.number(SLIDER_BUBBLE_ANIMATE) != 0.0;
    let font_size = move || t.length(SLIDER_BUBBLE_FONT_SIZE);
    // One bubble at `x` (a fraction of the width from the left), showing
    // `label` while `visible`.
    let bubble = move |x: Rc<dyn Fn() -> f32>,
                       label: Rc<dyn Fn() -> SharedString>,
                       visible: Rc<dyn Fn() -> bool>,
                       measured: Signal<f32>| {
        let last = stored(SharedString::default());
        let shown_label = move || {
            let now = label();
            if now.is_empty() {
                last.get()
            } else {
                last.replace(now.clone());
                now
            }
        };
        let place = x.clone();
        let on = visible.clone();
        let look = visible.clone();
        let el = view()
            .flex_row()
            .flex_shrink_0()
            .px(move || t.length(SLIDER_BUBBLE_PADDING_X))
            .py(move || t.length(SLIDER_BUBBLE_PADDING_Y))
            .rounded(move || t.length(SLIDER_BUBBLE_RADIUS))
            .bg(move || t.color(SLIDER_BUBBLE))
            .on_layout(move |frame: &Bounds<Points>| {
                let w = frame.size.width.0;
                if measured.get_untracked() != w {
                    measured.set(w);
                }
            })
            .ml(move || px(place() * ctl.width.get() - measured.get() / 2.0))
            .opacity(move || if on() { 1.0f32 } else { 0.0 })
            .scale(move || if look() || !animate() { 1.0f32 } else { 0.95 })
            .origin_bottom()
            .transition_opacity()
            .transition_transform()
            .app_class(props.bubble_class)
            .inline_style(move || props.bubble_style.get())
            .child(
                reset_inherited(text(shown_label))
                    .whitespace_nowrap()
                    .tabular_nums()
                    .text_size(font_size)
                    .line_height(move || line_height(font_size()))
                    .text_color(move || t.color(SLIDER_BUBBLE_FOREGROUND)),
            );
        let el = el.duration(move || if animate() { motion::duration(t) } else { 0.0 });
        view()
            .absolute()
            .left(px(0.0))
            .right(px(0.0))
            .bottom(move || {
                let thickness = t.length(SLIDER_THICKNESS);
                let thumb = t.length(SLIDER_THUMB_SIZE).clamp(px(0.0), thickness);
                (thickness + thumb) * 0.5 + t.length(SLIDER_BUBBLE_OFFSET)
            })
            .flex_row()
            .items_end()
            .child(el)
    };
    let single = move |which: Thumb| {
        bubble(
            Rc::new(move || ctl.left(ctl.fraction(which))),
            Rc::new(move || text_of(value_of(which)).unwrap_or_default()),
            Rc::new(move || {
                let present = which == Thumb::End || ctl.shown.get().is_range();
                present && wanted(which) && !joined.get()
            }),
            width_of(which),
        )
    };
    let both = move || {
        bubble(
            Rc::new(move || {
                let (a, b) = (
                    ctl.left(ctl.fraction(Thumb::Start)),
                    ctl.left(ctl.fraction(Thumb::End)),
                );
                (a + b) / 2.0
            }),
            Rc::new(move || {
                let (a, b) = (
                    text_of(value_of(Thumb::Start)),
                    text_of(value_of(Thumb::End)),
                );
                match (a, b) {
                    (Some(a), Some(b)) if a == b => a,
                    (Some(a), Some(b)) => labels::text(Label::ValueRange)
                        .replace("{start}", &a)
                        .replace("{end}", &b)
                        .into(),
                    _ => SharedString::default(),
                }
            }),
            Rc::new(move || joined.get() && (wanted(Thumb::Start) || wanted(Thumb::End))),
            signal(0.0f32),
        )
    };
    view()
        .absolute()
        .left(px(0.0))
        .right(px(0.0))
        .top(px(0.0))
        .bottom(px(0.0))
        .pointer_events(PointerEvents::None)
        .accessible(false)
        .accessibility_elements_hidden(true)
        .child(when(
            move || props.bubble.get() != SliderBubble::None,
            move || {
                view()
                    .absolute()
                    .left(px(0.0))
                    .right(px(0.0))
                    .top(px(0.0))
                    .bottom(px(0.0))
                    .child(single(Thumb::End))
                    // A range's start, and its two values in one.
                    .child(when(
                        move || ctl.shown.get().is_range(),
                        move || {
                            view()
                                .absolute()
                                .left(px(0.0))
                                .right(px(0.0))
                                .top(px(0.0))
                                .bottom(px(0.0))
                                .child(single(Thumb::Start))
                                .child(both())
                        },
                    ))
            },
        ))
}

/// A style binding as a part's style prop.
fn reactive(style: impl Fn() -> Style + 'static) -> Prop<Style> {
    Prop::from(IntoReactive::<Style>::into_reactive(style))
}

/// The ring a thumb shows under the pointer, dragged and with the keyboard
/// focus: a class made from the tokens, as the states are `nsh/base`'s.
fn ring_class(t: Theme) -> String {
    let width = t.length(SLIDER_THUMB_RING_WIDTH);
    let color = t.color(SLIDER_THUMB_RING);
    ["hover", "active", "focus-visible"]
        .into_iter()
        .fold(Classes::new(), |c, state| {
            c.length(state, "ring", width).color(state, "ring", color)
        })
        .build()
        .to_string()
}

/// The library's classes, then the app's.
fn join_classes(ours: Option<String>, app: Option<SharedString>) -> Option<SharedString> {
    match (ours, app) {
        (None, app) => app,
        (Some(ours), None) => Some(ours.into()),
        (Some(ours), Some(app)) => Some(format!("{ours} {app}").into()),
    }
}
