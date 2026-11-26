//! Seed tokens: the few knobs every size follows. Change `radius` and every
//! corner follows; change `control-height` and every control follows.

use crate::tokens;

tokens! {
    /// The base corner radius (shadcn's `--radius`, 0.625rem). `radius-sm`
    /// to `radius-xl` derive from it. Its default is `nsh/base`'s theme's.
    RADIUS: Length = "radius",
    /// The spacing unit (Tailwind 4's `--spacing`): paddings and gaps are
    /// multiples of it.
    SPACING: Length = "spacing" => |t| t.px(4.0),
    /// The base control text size: `font-size-xs` to `font-size-lg` derive
    /// from it.
    FONT_SIZE: Length = "font-size" => |t| t.px(14.0),
    /// The medium control height with a pointer: `control-height-xs` to
    /// `control-height-lg` derive from it.
    CONTROL_HEIGHT: Length = "control-height" => |t| t.px(32.0),
    /// The medium control height on touch screens:
    /// `control-height-touch-xs` to `control-height-touch-lg` derive from it.
    CONTROL_HEIGHT_TOUCH: Length = "control-height-touch" => |t| t.px(40.0),
    /// The base transition duration in milliseconds: `duration-fast` and
    /// `duration-slow` derive from it.
    DURATION: Number = "duration" => |t| t.value(150.0),
}
