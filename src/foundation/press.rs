//! Press feedback: a control sinks while held.

use nsh_core::{element::StyleRefinement, prelude::*};

/// How far a button sinks while pressed.
pub const BUTTON_SCALE: f32 = 0.97;
/// How far a small control (a checkbox, a radio) sinks while pressed.
pub const SMALL_SCALE: f32 = 0.9;

/// The pressed style: sinks to `scale` while held, unless `idle` (disabled,
/// loading) or the user asked for reduced motion. Use it in the control's
/// `.active(..)`.
pub fn press(
    s: StyleRefinement,
    scale: f32,
    idle: impl Fn() -> bool + Copy + 'static,
) -> StyleRefinement {
    s.scale(move || {
        if idle() || reduce_motion() {
            1.0_f32
        } else {
            scale
        }
    })
}
