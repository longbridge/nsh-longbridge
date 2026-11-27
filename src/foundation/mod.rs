//! What every component shares: sizes, text, focus, press feedback,
//! motion, shadows, icons, the field look, the choice controls' label and
//! hit area, the popup surface, a trigger slot's press, and the library's
//! own words.
//! Each rule reads tokens; components use these instead of writing their
//! own.

pub mod choice;
pub mod field;
pub mod field_ref;
pub mod focus;
pub mod icon;
pub mod labels;
pub mod menu;
pub mod motion;
pub mod popup;
pub mod press;
pub mod shadow;
pub mod size;
pub mod trigger;
pub mod typography;

pub use field::field_look;
pub use field_ref::FieldRef;
pub use focus::focus_ring;
pub use icon::{
    Icon, IconElement, IconName, IconProps, icon, provide_icon_side, provide_icon_size,
};
pub use press::press;
pub use size::Size;
pub use typography::line_height;
