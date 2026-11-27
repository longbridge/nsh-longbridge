//! `Progress`: a progress bar on `nsh/base`'s `Progress` (the value held
//! to 0–100, the progress bar role and its value for screen readers). The
//! library adds the look: the track and the indicator from tokens, the
//! indicator's motion as the value changes, and the moving segment while
//! indeterminate. API: `docs/components/progress.md`.

use std::time::Duration;

use nsh_base::progress as base_progress;
use nsh_core::{
    element::{Style, StyleRefinement},
    prelude::*,
};

use crate::{
    components::table,
    foundation::{class::Classes, motion},
    theme::{Theme, seeds::DURATION, seeds::SPACING, semantic::PRIMARY, theme},
    tokens,
};

tokens! {
    /// The track's fill.
    PROGRESS: Color = "progress" => |t| t.color(PRIMARY).opacity(0.2),
    /// The indicator's fill.
    PROGRESS_INDICATOR: Color = "progress-indicator" => |t| t.color(PRIMARY),
    /// The track's height.
    PROGRESS_HEIGHT: Length = "progress-height" => |t| t.length(SPACING) * 2.0,
    /// The track's corner radius.
    PROGRESS_RADIUS: Length = "progress-radius" => |t| t.length(PROGRESS_HEIGHT) * 0.5,
    /// The space between the track and the children under it.
    PROGRESS_GAP: Length = "progress-gap" => |t| t.length(SPACING) * 2.0,
    /// The moving segment's share of the track while indeterminate.
    PROGRESS_INDETERMINATE_WIDTH: Number = "progress-indeterminate-width" => |t| t.value(0.4),
    /// How long the moving segment takes to cross the track, in
    /// milliseconds.
    PROGRESS_INDETERMINATE_DURATION: Number = "progress-indeterminate-duration" => |t| t.number(DURATION) * 10.0,
}

/// The pause between two passes of the moving segment, in which it goes
/// back to the start without a transition.
const RESET: Duration = Duration::from_millis(20);

/// The props of [`Progress`].
#[derive(Debug, Clone, PartialEq, Props)]
pub struct ProgressProps {
    /// How far, from 0 to 100 (held to the range).
    #[prop(default)]
    pub value: Prop<f32>,
    /// The extent is unknown: a segment moves across the track; screen
    /// readers hear no value.
    #[prop(default)]
    pub indeterminate: Prop<bool>,
    /// What is in progress ("Uploading report.pdf"), when the children do
    /// not say it.
    pub accessibility_label: Prop<Option<SharedString>>,
    /// Tailwind classes of the track (its height: `h-1`), after its look.
    pub track_class: Prop<Option<SharedString>>,
    /// The inline style of the track.
    #[prop(default)]
    pub track_style: Prop<Style>,
    /// Tailwind classes of the indicator (its fill: `bg-green-600`), after
    /// its look.
    pub indicator_class: Prop<Option<SharedString>>,
    /// The inline style of the indicator, merged after the library's.
    #[prop(default)]
    pub indicator_style: Prop<Style>,
    /// Under the track: a label, the value as text; screen readers read
    /// them as the name.
    pub children: Prop<Children>,
}

/// A progress bar: an 8-point track in the primary color at 20 %, filled
/// to `value` with the primary color, the fill moving smoothly as the value
/// changes; while `indeterminate` a segment crosses it again and again
/// (still and dimmed across the track with reduced motion).
#[component]
pub fn Progress(props: ProgressProps, _cx: Scope<NoEvent>) -> impl IntoElement {
    let t = theme();
    table::register_cell_block();
    let indeterminate = props.indeterminate;
    let still = reduce_motion();
    // The moving segment: `running` while it crosses the track with a
    // transition, not while it goes back to the start.
    let running = signal(false);
    effect(move || {
        if !indeterminate.get() || still {
            return;
        }
        let pass = Duration::from_millis(t.number(PROGRESS_INDETERMINATE_DURATION).max(0.0) as u64);
        untracked(|| running.set(false));
        set_timeout(RESET, move || running.set(true));
        let task = set_interval(pass + RESET, move || {
            running.set(false);
            set_timeout(RESET, move || running.set(true));
        });
        cleanup(move || task.cancel());
    });
    let mut root = base_progress()
        .value(props.value)
        .indeterminate(indeterminate)
        .accessibility_label(props.accessibility_label)
        .app_class(move || {
            Some(
                Classes::new()
                    .push("flex-col w-full")
                    .length("", "gap", t.length(PROGRESS_GAP))
                    .build(),
            )
        })
        .track_class(move || {
            Some(
                Classes::new()
                    .push("w-full overflow-hidden")
                    .length("", "h", t.length(PROGRESS_HEIGHT))
                    .length("", "rounded", t.length(PROGRESS_RADIUS))
                    .color("", "bg", t.color(PROGRESS))
                    .then(props.track_class.get()),
            )
        })
        .indicator_class(move || {
            Some(indicator_class(t, indeterminate.get(), still).then(props.indicator_class.get()))
        })
        .child(props.children);
    let value = props.value;
    let app_style = props.indicator_style;
    let parts = root.__element().__props_mut();
    parts.track_style = props.track_style;
    parts.indicator_style = Prop::from(IntoReactive::<Style>::into_reactive(move || {
        let look = indicator_motion(t, value.get(), indeterminate.get(), still, running.get());
        Style::from(look).then(&app_style.get())
    }));
    root
}

/// The indicator's classes: the track's height, its fill, scaled from its
/// start; while indeterminate the segment's width (the whole track,
/// dimmed, with reduced motion).
fn indicator_class(t: Theme, indeterminate: bool, still: bool) -> Classes {
    let c = Classes::new()
        .push("h-full origin-left rtl:origin-right")
        .color("", "bg", t.color(PROGRESS_INDICATOR));
    match (indeterminate, still) {
        (true, false) => c.push(&format!(
            "w-[{}%]",
            (t.number(PROGRESS_INDETERMINATE_WIDTH) * 100.0).round()
        )),
        (true, true) => c.push("w-full opacity-50"),
        (false, _) => c.push("w-full"),
    }
}

/// The indicator's transform and its transition: scaled to `value` from
/// its start; while indeterminate the segment from fully before the track
/// to fully past it (`running`), or back at the start without a transition.
fn indicator_motion(
    t: Theme,
    value: f32,
    indeterminate: bool,
    still: bool,
    running: bool,
) -> StyleRefinement {
    let s = StyleRefinement::default().transition_transform();
    if indeterminate {
        if still {
            return s;
        }
        let share = t.number(PROGRESS_INDETERMINATE_WIDTH).max(0.01);
        let (to, ms) = if running {
            (1.0 / share, t.number(PROGRESS_INDETERMINATE_DURATION))
        } else {
            (-1.0, 0.0)
        };
        return s
            .translate_x(relative(to))
            .duration(ms)
            .ease_bezier(0.4, 0.0, 0.2, 1.0);
    }
    let v = if value.is_nan() {
        0.0
    } else {
        value.clamp(0.0, 100.0)
    };
    s.scale_x(v / 100.0)
        .duration(motion::duration(t))
        .ease_bezier(0.4, 0.0, 0.2, 1.0)
}
