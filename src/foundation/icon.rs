//! Icons: a few of lucide's (ISC license, `icons/LICENSE-lucide.txt`),
//! drawn with `currentColor`, so they take the color of the text around
//! them. Their size follows `Size` (12 / 14 / 16 / 20), from the control
//! they are in when not given.

use nsh_core::{element::ImageSrc, prelude::*};

use super::size::Size;
use crate::theme::{
    Theme, Token, kind,
    semantic::{ICON_SIZE_LG, ICON_SIZE_MD, ICON_SIZE_SM, ICON_SIZE_XS},
    theme,
};

/// The icons the library draws (lucide's names).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default, PropEnum)]
pub enum IconName {
    /// `loader-circle`: an arc, spun for a spinner.
    #[prop(rename = "loader-circle")]
    LoaderCircle,
    /// `plus`.
    #[default]
    #[prop(rename = "plus")]
    Plus,
    /// `arrow-right`.
    #[prop(rename = "arrow-right")]
    ArrowRight,
    /// `trash-2`.
    #[prop(rename = "trash-2")]
    Trash2,
    /// `chevron-down`.
    #[prop(rename = "chevron-down")]
    ChevronDown,
    /// `check`: a checked checkbox's mark.
    #[prop(rename = "check")]
    Check,
    /// `minus`: an indeterminate checkbox's mark.
    #[prop(rename = "minus")]
    Minus,
}

nsh_core::static_reactive!(IconName: IconName);

impl IconName {
    /// The SVG.
    pub fn markup(self) -> &'static str {
        match self {
            IconName::LoaderCircle => include_str!("icons/loader-circle.svg"),
            IconName::Plus => include_str!("icons/plus.svg"),
            IconName::ArrowRight => include_str!("icons/arrow-right.svg"),
            IconName::Trash2 => include_str!("icons/trash-2.svg"),
            IconName::ChevronDown => include_str!("icons/chevron-down.svg"),
            IconName::Check => include_str!("icons/check.svg"),
            IconName::Minus => include_str!("icons/minus.svg"),
        }
    }
}

/// The icon size token of `size` (`icon-size-xs` to `-lg`).
pub fn icon_size_token(size: Size) -> Token<kind::Length> {
    size.pick(ICON_SIZE_XS, ICON_SIZE_SM, ICON_SIZE_MD, ICON_SIZE_LG)
}

/// The size icons take inside a control that does not give them one
/// (a button's, an input's): provided by the control.
#[derive(Debug, Clone, Copy)]
pub struct IconSizeContext(pub Memo<Size>);

/// Gives the icons inside the current component `size` when they have none
/// of their own (call it in the control's body, before its content).
pub fn provide_icon_size(size: impl Fn() -> Size + 'static) {
    provide_context(IconSizeContext(computed(size)));
}

/// The size an icon without its own takes here: the control's, else `md`.
pub fn icon_size_here() -> Memo<Size> {
    match use_context::<IconSizeContext>() {
        Some(IconSizeContext(size)) => size,
        None => computed(|| Size::Md),
    }
}

/// The props of [`Icon`].
#[derive(Debug, Clone, PartialEq, Props)]
pub struct IconProps {
    /// Which icon.
    pub name: Prop<IconName>,
    /// Its size: 12 / 14 / 16 / 16 points for `xs` / `sm` / `md` / `lg`
    /// (`icon-size-*`).
    /// Left out, the size of the control it is in (`md` elsewhere).
    pub size: Prop<Option<Size>>,
    /// Spin it (a spinner); it keeps spinning with reduced motion, as it
    /// says something is going on.
    #[prop(default)]
    pub spin: Prop<bool>,
}

/// An icon of the library, in the color of the text around it (`class`
/// `text-…` changes it).
#[component]
pub fn Icon(props: IconProps, _cx: Scope<NoEvent>) -> impl IntoElement {
    let t = theme();
    let here = icon_size_here();
    let side = move || {
        t.length(icon_size_token(
            props.size.get().unwrap_or_else(|| here.get()),
        ))
    };
    icon_view(t, move || props.name.get(), side).style_when(props.spin, |s| s.animate_spin())
}

/// The svg of icon `name`, `side` points square (for components drawing
/// their own icon parts).
pub(crate) fn icon_view(
    _t: Theme,
    name: impl Fn() -> IconName + 'static,
    side: impl Fn() -> Points + Copy + 'static,
) -> Svg {
    svg(move || ImageSrc::svg_markup(name().markup()))
        .w(side)
        .h(side)
        .flex_shrink_0()
        .accessibility_elements_hidden(true)
}
