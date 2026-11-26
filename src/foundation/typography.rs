//! Control text: its size by `Size`, and the line height that goes with a
//! font size.

use nsh_core::{
    geometry::{Points, px},
    prelude::*,
};

use super::size::Size;
use crate::theme::{
    Theme, Token, kind,
    semantic::{
        FONT_SIZE_LG, FONT_SIZE_MD, FONT_SIZE_SM, FONT_SIZE_XS, FONT_WEIGHT_MEDIUM,
        FONT_WEIGHT_SEMIBOLD,
    },
};

/// The control text size token of `size` (`font-size-xs` to `-lg`).
pub fn font_size_token(size: Size) -> Token<kind::Length> {
    size.pick(FONT_SIZE_XS, FONT_SIZE_SM, FONT_SIZE_MD, FONT_SIZE_LG)
}

/// The line height that goes with `font_size`, as Tailwind pairs them
/// (12 → 16, 13 → 18, 14 → 20, 16 → 24, 18 → 28, 20 → 28, 24 → 32): every
/// component sets its line height with this, none writes its own.
pub fn line_height(font_size: Points) -> Points {
    let f = f32::from(font_size);
    let lh = if f <= 18.0 {
        (2.0 * f - 8.0).max(f)
    } else if f <= 24.0 {
        (f * 4.0 / 3.0 / 4.0).round() * 4.0
    } else {
        (f * 1.2).round()
    };
    px(lh)
}

/// The medium weight (`font-weight-medium`, 500): controls' labels. Read
/// when the view is made (a font weight takes no binding).
pub fn medium(t: Theme) -> FontWeight {
    weight(t.number(FONT_WEIGHT_MEDIUM))
}

/// The semibold weight (`font-weight-semibold`, 600): titles.
pub fn semibold(t: Theme) -> FontWeight {
    weight(t.number(FONT_WEIGHT_SEMIBOLD))
}

/// A weight token's value as a font weight (`label-font-weight`'s 500).
/// Read when the view is made.
pub fn weight(n: f32) -> FontWeight {
    FontWeight(n.round().clamp(1.0, 1000.0) as u16)
}
