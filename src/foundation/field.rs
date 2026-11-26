//! The field look: what every control that looks like a text field shares
//! (`Input`, `Textarea`, a select's trigger): its edge, background, shadow,
//! focus ring and its invalid and disabled states, from the `input-*`
//! tokens. One set of tokens, so a theme that changes `input-border`
//! changes them all.

use nsh_core::prelude::*;

use super::{focus, motion, shadow};
use crate::{
    theme::{
        TRANSPARENT, Theme,
        seeds::SPACING,
        semantic::{
            BORDER_WIDTH, CONTROL_HEIGHT_LG, CONTROL_HEIGHT_MD, CONTROL_HEIGHT_SM,
            CONTROL_HEIGHT_TOUCH_LG, CONTROL_HEIGHT_TOUCH_MD, CONTROL_HEIGHT_TOUCH_SM,
            CONTROL_HEIGHT_TOUCH_XS, CONTROL_HEIGHT_XS, DESTRUCTIVE, FONT_SIZE_LG, FONT_SIZE_MD,
            FONT_SIZE_SM, FONT_SIZE_XS, FOREGROUND, INPUT, MUTED_FOREGROUND, OPACITY_DISABLED,
            RADIUS_MD, RING, RING_INVALID, RING_OPACITY, RING_WIDTH,
        },
    },
    tokens,
};

tokens! {
    /// The field's corner radius.
    INPUT_RADIUS: Length = "input-radius" => |t| t.length(RADIUS_MD),
    /// An extra small field's height.
    INPUT_HEIGHT_XS: Length = "input-height-xs" => |t| t.pointer_touch(t.length(CONTROL_HEIGHT_XS), t.length(CONTROL_HEIGHT_TOUCH_XS)),
    /// A small field's height.
    INPUT_HEIGHT_SM: Length = "input-height-sm" => |t| t.pointer_touch(t.length(CONTROL_HEIGHT_SM), t.length(CONTROL_HEIGHT_TOUCH_SM)),
    /// A medium field's height.
    INPUT_HEIGHT_MD: Length = "input-height-md" => |t| t.pointer_touch(t.length(CONTROL_HEIGHT_MD), t.length(CONTROL_HEIGHT_TOUCH_MD)),
    /// A large field's height.
    INPUT_HEIGHT_LG: Length = "input-height-lg" => |t| t.pointer_touch(t.length(CONTROL_HEIGHT_LG), t.length(CONTROL_HEIGHT_TOUCH_LG)),
    /// An extra small field's horizontal padding.
    INPUT_PADDING_X_XS: Length = "input-padding-x-xs" => |t| t.length(SPACING) * 2.0,
    /// A small field's horizontal padding.
    INPUT_PADDING_X_SM: Length = "input-padding-x-sm" => |t| t.length(SPACING) * 2.5,
    /// A medium field's horizontal padding.
    INPUT_PADDING_X_MD: Length = "input-padding-x-md" => |t| t.length(SPACING) * 3.0,
    /// A large field's horizontal padding.
    INPUT_PADDING_X_LG: Length = "input-padding-x-lg" => |t| t.length(SPACING) * 3.0,
    /// An extra small field's text size.
    INPUT_FONT_SIZE_XS: Length = "input-font-size-xs" => |t| t.length(FONT_SIZE_XS),
    /// A small field's text size.
    INPUT_FONT_SIZE_SM: Length = "input-font-size-sm" => |t| t.length(FONT_SIZE_SM),
    /// A medium field's text size.
    INPUT_FONT_SIZE_MD: Length = "input-font-size-md" => |t| t.length(FONT_SIZE_MD),
    /// A large field's text size.
    INPUT_FONT_SIZE_LG: Length = "input-font-size-lg" => |t| t.length(FONT_SIZE_LG),
    /// The width of the field's edge.
    INPUT_BORDER_WIDTH: Length = "input-border-width" => |t| t.length(BORDER_WIDTH),
    /// The field's edge at rest.
    INPUT_BORDER: Color = "input-border" => |t| t.color(INPUT),
    /// The edge while the field has the keyboard focus.
    INPUT_BORDER_FOCUS: Color = "input-border-focus" => |t| t.color(RING),
    /// The edge while the app's checks reject the value; wins over the focus edge.
    INPUT_BORDER_INVALID: Color = "input-border-invalid" => |t| t.color(DESTRUCTIVE),
    /// The field's background: transparent, `input` at 30 % in the dark.
    INPUT_BACKGROUND: Color = "input-background" => |t| t.light_dark(TRANSPARENT, t.color(INPUT).opacity(0.3)),
    /// The field's text.
    INPUT_FOREGROUND: Color = "input-foreground" => |t| t.color(FOREGROUND),
    /// The placeholder's text.
    INPUT_PLACEHOLDER_FOREGROUND: Color = "input-placeholder-foreground" => |t| t.color(MUTED_FOREGROUND),
    /// The focus ring around the field.
    INPUT_RING: Color = "input-ring" => |t| t.color(RING).opacity(t.number(RING_OPACITY)),
    /// The focus ring of a field whose value the app rejects.
    INPUT_RING_INVALID: Color = "input-ring-invalid" => |t| t.color(RING_INVALID),
    /// The focus ring's width.
    INPUT_RING_WIDTH: Length = "input-ring-width" => |t| t.length(RING_WIDTH),
    /// The field's shadow level (1: `xs`).
    INPUT_SHADOW: Number = "input-shadow" => |t| t.value(1.0),
}

/// Gives `el` the field look: the edge, background, text color, shadow,
/// focus ring (with a pointer; the edge alone on touch screens), the
/// invalid edge and ring and the disabled opacity, with their transitions.
/// The caller sets the size (`input-height-*`, `input-padding-x-*`,
/// `input-font-size-*`) and the placeholder color.
pub fn field_look<E: Styled + Interactive>(
    el: E,
    t: Theme,
    invalid: impl Fn() -> bool + Copy + 'static,
) -> E {
    let el = el
        .rounded(move || t.length(INPUT_RADIUS))
        .border(move || t.length(INPUT_BORDER_WIDTH))
        .border_color(move || {
            if invalid() {
                t.color(INPUT_BORDER_INVALID)
            } else {
                t.color(INPUT_BORDER)
            }
        })
        .bg(move || t.color(INPUT_BACKGROUND))
        .text_color(move || t.color(INPUT_FOREGROUND))
        .inline_style(move || shadow::shadow(t, shadow::level(t.number(INPUT_SHADOW))))
        .focus_visible(move |s| {
            focus::focus_ring(
                s,
                t,
                move || {
                    if invalid() {
                        t.color(INPUT_RING_INVALID)
                    } else {
                        t.color(INPUT_RING)
                    }
                },
                None,
            )
            .border_color(move || {
                if invalid() {
                    t.color(INPUT_BORDER_INVALID)
                } else {
                    t.color(INPUT_BORDER_FOCUS)
                }
            })
        })
        .disabled_style(move |s| s.opacity(move || t.number(OPACITY_DISABLED)));
    motion::transition_all(el, t)
}
