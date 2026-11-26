//! `Size`: the one size scale of every control. Height, text, icon and
//! padding change together with it.

use nsh_core::{
    element::is_desktop,
    geometry::{Points, px},
    prelude::*,
};

use crate::theme::{
    Theme, Token, kind,
    semantic::{
        CONTROL_HEIGHT_LG, CONTROL_HEIGHT_MD, CONTROL_HEIGHT_SM, CONTROL_HEIGHT_TOUCH_LG,
        CONTROL_HEIGHT_TOUCH_MD, CONTROL_HEIGHT_TOUCH_SM, CONTROL_HEIGHT_TOUCH_XS,
        CONTROL_HEIGHT_XS, TOUCH_TARGET,
    },
};

/// A control's size: `xs`, `sm`, `md` (the default) or `lg`. With a pointer
/// 24 / 28 / 32 / 36 points tall, on touch screens 32 / 36 / 40 / 48
/// (`control-height-*`, `control-height-touch-*`).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default, PropEnum)]
pub enum Size {
    /// Extra small: dense tables, toolbars of tools.
    Xs,
    /// Small: toolbars, menus, compact forms.
    Sm,
    /// Medium: most controls.
    #[default]
    Md,
    /// Large: sparse forms, prominent decisions.
    Lg,
}

impl Size {
    /// One of four values, by size.
    pub fn pick<T>(self, xs: T, sm: T, md: T, lg: T) -> T {
        match self {
            Size::Xs => xs,
            Size::Sm => sm,
            Size::Md => md,
            Size::Lg => lg,
        }
    }

    /// Its name: `"xs"`, `"sm"`, `"md"` or `"lg"`.
    pub fn as_str(self) -> &'static str {
        self.pick("xs", "sm", "md", "lg")
    }
}

nsh_core::static_reactive!(Size: Size);

/// Whether a pointer drives the UI (desktop and the desktop web): sizes
/// and hit slop follow it. It does not change while the app runs.
pub fn pointer() -> bool {
    is_desktop()
}

/// The semantic control height of `size`, for this device (pointer or
/// touch).
pub fn control_height_token(size: Size) -> Token<kind::Length> {
    if pointer() {
        size.pick(
            CONTROL_HEIGHT_XS,
            CONTROL_HEIGHT_SM,
            CONTROL_HEIGHT_MD,
            CONTROL_HEIGHT_LG,
        )
    } else {
        size.pick(
            CONTROL_HEIGHT_TOUCH_XS,
            CONTROL_HEIGHT_TOUCH_SM,
            CONTROL_HEIGHT_TOUCH_MD,
            CONTROL_HEIGHT_TOUCH_LG,
        )
    }
}

/// How far presses count around a control `height` tall so a finger gets
/// `touch-target` (44) points, without the control looking bigger: none
/// with a pointer. Read it in a binding.
pub fn hit_slop(t: Theme, height: Points) -> EdgeInsets {
    if pointer() {
        return EdgeInsets::default();
    }
    let target = t.length(TOUCH_TARGET);
    let slop = ((target - height) / 2.0).max(px(0.0));
    EdgeInsets::from(f32::from(slop))
}
