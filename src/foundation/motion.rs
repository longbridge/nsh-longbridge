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
    el.transition_all()
        .duration(move || duration(t))
        .ease_bezier(0.4, 0.0, 0.2, 1.0)
}
