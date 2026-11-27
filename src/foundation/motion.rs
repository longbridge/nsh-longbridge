//! Motion: the easing every state change uses and its duration, none with
//! reduced motion.

use nsh_core::prelude::*;

use crate::theme::{Theme, seeds::DURATION};

/// The duration of a state change in milliseconds: `duration` (150), 0
/// with reduced motion. Read it in a binding.
pub fn duration(t: Theme) -> f32 {
    if reduce_motion() {
        0.0
    } else {
        t.number(DURATION)
    }
}

/// Transitions every property `el` changes between states over
/// [`duration`], with Tailwind's default easing (`cubic-bezier(0.4, 0,
/// 0.2, 1)`, `transition-all`).
pub fn transition_all<E: Styled>(el: E, t: Theme) -> E {
    timing(el.transition_all(), t)
}

/// The duration ([`duration`]) and easing of the transitions `el` already
/// names (`el.transition_shadow()`, `el.transition_transform()`): for a
/// control that animates only some of what changes.
pub fn timing<E: Styled>(el: E, t: Theme) -> E {
    el.duration(move || duration(t))
        .ease_bezier(0.4, 0.0, 0.2, 1.0)
}

/// Turns `el` around once a second, at a constant speed, forever
/// (Tailwind's `animate-spin`), **even with reduced motion**: a spinner's
/// turning says that something is still going on, and a still one looks
/// stuck (as the platforms' own spinners keep turning). `animate-spin`
/// itself stops with reduced motion, so a spinner uses this instead.
pub fn spin<E: Element>(el: E) -> E {
    let turn = motion();
    let _ = turn.play_ignoring_reduce_motion(
        MotionSpec::new(
            KeyframeTiming::new(1000.0)
                .easing(Easing::Linear)
                .infinite(),
        )
        .values(MotionChannel::Rotate, [0.0, 360.0]),
    );
    el.bind_motion(&turn)
}
