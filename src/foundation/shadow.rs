//! Shadows: Tailwind's levels with fixed geometry, in the `shadow` color.
//! A component picks its level with a number token (`<c>-shadow`): 0 none,
//! 1 `xs`, 2 `sm`, 3 `md`, 4 `lg`.

use nsh_core::{
    Color,
    element::{Style, StyleRefinement, color},
    prelude::*,
};

use crate::theme::{Theme, semantic::SHADOW};

/// One layer: offset y, blur, spread and the share of the `shadow` color's
/// alpha it takes.
struct Layer(f32, f32, f32, f32);

/// The layers of `level` (Tailwind 4's `shadow-xs` to `shadow-lg`).
fn layers(level: u8) -> &'static [Layer] {
    match level {
        1 => &[Layer(1.0, 2.0, 0.0, 0.05)],
        2 => &[Layer(1.0, 3.0, 0.0, 0.1), Layer(1.0, 2.0, -1.0, 0.1)],
        3 => &[Layer(4.0, 6.0, -1.0, 0.1), Layer(2.0, 4.0, -2.0, 0.1)],
        4 => &[Layer(10.0, 15.0, -3.0, 0.1), Layer(4.0, 6.0, -4.0, 0.1)],
        _ => &[],
    }
}

/// The level a number token holds: 0 to 4, rounded and clamped.
pub fn level(n: f32) -> u8 {
    n.round().clamp(0.0, 4.0) as u8
}

/// The shadow of `level` in this theme's `shadow` color, as an inline
/// style (read it in a binding: `.inline_style(move || shadow(t, ..))`; it
/// follows the appearance, the scope and the tokens).
pub fn shadow(t: Theme, level: u8) -> Style {
    Style::from(StyleRefinement::default().shadow(box_shadows(t, level)))
}

/// The layers of the shadow of `level` in this theme's `shadow` color,
/// for a style that sets them itself (a part's style built in a
/// `style_with` binding). Read it in a binding.
pub fn box_shadows(t: Theme, level: u8) -> Vec<BoxShadow> {
    let base = color(t.color(SHADOW));
    layers(level)
        .iter()
        .map(|l| BoxShadow {
            offset_x: px(0.0),
            offset_y: px(l.0),
            blur: px(l.1),
            spread: px(l.2),
            color: Color::from(base.opacity(l.3)),
            inset: false,
        })
        .collect()
}
