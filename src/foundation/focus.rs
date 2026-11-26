//! The keyboard focus ring: drawn outside the control with a pointer, the
//! edge alone on touch screens (as the platforms' own controls).

use nsh_core::{element::StyleRefinement, prelude::*};

use super::size::pointer;
use crate::theme::{Theme, Token, kind, semantic::RING_WIDTH};

/// The focus-visible style: the edge turns `border` (when the control has
/// an edge), and with a pointer a `ring-width` ring of `ring` is drawn
/// outside it. `ring` is the full ring color (`<c>-ring`, which already
/// carries `ring-opacity`). Use it in the control's `.focus_visible(..)`.
pub fn focus_ring(
    s: StyleRefinement,
    t: Theme,
    ring: impl Fn() -> AdaptiveColor + 'static,
    border: Option<Token<kind::Color>>,
) -> StyleRefinement {
    let s = match border {
        Some(border) => s.border_color(move || t.color(border)),
        None => s,
    };
    if pointer() {
        s.ring(move || t.length(RING_WIDTH)).ring_color(ring)
    } else {
        s
    }
}
