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
/// 0.2, 1)`, shadcn's `transition-all`).
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
