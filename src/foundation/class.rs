//! Classes generated from resolved tokens.
//!
//! A typed style (`.bg(..)`, `.px(..)`) wins over every class, the app's
//! included. A part whose look the app changes with classes (a cell's
//! padding, a row's edge, `w-24 flex-none` on a column) gets that look as
//! classes instead, built here from the tokens and placed before the app's;
//! so does a part of an `nsh/base` component whose Rust style is built once
//! (conventions, "Drawing"). Built in a binding, the classes follow the
//! tokens, a `ThemeScope` and a forced appearance.

use nsh_core::{geometry::Points, prelude::*};

/// A class list, built from fixed utilities and token values.
#[derive(Debug, Default, Clone)]
pub struct Classes(String);

impl Classes {
    /// An empty list.
    pub fn new() -> Self {
        Self::default()
    }

    /// Adds fixed utilities as they are (`"flex-1 min-w-0"`).
    pub fn push(mut self, classes: &str) -> Self {
        if !classes.is_empty() {
            if !self.0.is_empty() {
                self.0.push(' ');
            }
            self.0.push_str(classes);
        }
        self
    }

    /// Adds `classes` when `on`.
    pub fn push_if(self, on: bool, classes: &str) -> Self {
        if on { self.push(classes) } else { self }
    }

    /// `utility` in `color` under `variant` (`""` for none, `"hover"`,
    /// `"selected"`): `hover:bg-[#light] dark:hover:bg-[#dark]`, the dark
    /// one only where it differs.
    pub fn color(self, variant: &str, utility: &str, color: AdaptiveColor) -> Self {
        let (light, dark) = (color.light.to_hex(), color.dark.to_hex());
        let prefix = if variant.is_empty() {
            String::new()
        } else {
            format!("{variant}:")
        };
        let s = self.push(&format!("{prefix}{utility}-[{light}]"));
        if dark == light {
            s
        } else {
            s.push(&format!("dark:{prefix}{utility}-[{dark}]"))
        }
    }

    /// `utility` of `length` points under `variant`: `h-[32px]`.
    pub fn length(self, variant: &str, utility: &str, length: Points) -> Self {
        let prefix = if variant.is_empty() {
            String::new()
        } else {
            format!("{variant}:")
        };
        self.push(&format!("{prefix}{utility}-[{}px]", number(length.0)))
    }

    /// `utility` of a unitless `value` under `variant`: `opacity-[0.5]`.
    pub fn number(self, variant: &str, utility: &str, value: f32) -> Self {
        let prefix = if variant.is_empty() {
            String::new()
        } else {
            format!("{variant}:")
        };
        self.push(&format!("{prefix}{utility}-[{}]", number(value)))
    }

    /// The list, then the app's classes, which win.
    pub fn then(self, app: Option<SharedString>) -> SharedString {
        match app {
            Some(app) => self.push(&app).build(),
            None => self.build(),
        }
    }

    /// The list.
    pub fn build(self) -> SharedString {
        self.0.into()
    }
}

/// A number as a class writes it: no trailing zeros (`32`, `0.5`).
fn number(n: f32) -> String {
    let rounded = (n * 1000.0).round() / 1000.0;
    format!("{rounded}")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn lengths_and_numbers_are_written_plainly() {
        let c = Classes::new()
            .length("", "h", Points(32.0))
            .length("hover", "px", Points(8.5))
            .number("", "opacity", 0.5)
            .build();
        assert_eq!(&*c, "h-[32px] hover:px-[8.5px] opacity-[0.5]");
    }

    #[test]
    fn the_app_classes_come_last() {
        let c = Classes::new().push("flex-1").then(Some("flex-none".into()));
        assert_eq!(&*c, "flex-1 flex-none");
    }
}
