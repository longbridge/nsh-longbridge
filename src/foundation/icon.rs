//! Icons: a few of lucide's (ISC license, `icons/LICENSE-lucide.txt`),
//! drawn with `currentColor`, so they take the color of the text of the
//! library's part they are in (`foreground` outside every such part). Their
//! size follows `Size` (12 / 14 / 16 / 16), from the control they are in
//! when not given.

use nsh_core::{element::ImageSrc, prelude::*};

use super::size::Size;
use crate::theme::{
    Theme, Token, kind,
    semantic::{FOREGROUND, ICON_SIZE_LG, ICON_SIZE_MD, ICON_SIZE_SM, ICON_SIZE_XS},
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
    /// `x`: a close button's mark.
    #[prop(rename = "x")]
    X,
    /// `circle-check`: success.
    #[prop(rename = "circle-check")]
    CircleCheck,
    /// `info`: information.
    #[prop(rename = "info")]
    Info,
    /// `triangle-alert`: a warning.
    #[prop(rename = "triangle-alert")]
    TriangleAlert,
    /// `octagon-x`: an error.
    #[prop(rename = "octagon-x")]
    OctagonX,
    /// `chevron-right`: a submenu's mark, a next page.
    #[prop(rename = "chevron-right")]
    ChevronRight,
    /// `chevron-left`: a sheet's back row.
    #[prop(rename = "chevron-left")]
    ChevronLeft,
    /// `ellipsis`: an overflow menu's trigger.
    #[prop(rename = "ellipsis")]
    Ellipsis,
    /// `settings`.
    #[prop(rename = "settings")]
    Settings,
    /// `log-out`.
    #[prop(rename = "log-out")]
    LogOut,
    /// `layout-dashboard`.
    #[prop(rename = "layout-dashboard")]
    LayoutDashboard,
    /// `activity`.
    #[prop(rename = "activity")]
    Activity,
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
            IconName::X => include_str!("icons/x.svg"),
            IconName::CircleCheck => include_str!("icons/circle-check.svg"),
            IconName::Info => include_str!("icons/info.svg"),
            IconName::TriangleAlert => include_str!("icons/triangle-alert.svg"),
            IconName::OctagonX => include_str!("icons/octagon-x.svg"),
            IconName::ChevronRight => include_str!("icons/chevron-right.svg"),
            IconName::ChevronLeft => include_str!("icons/chevron-left.svg"),
            IconName::Ellipsis => include_str!("icons/ellipsis.svg"),
            IconName::Settings => include_str!("icons/settings.svg"),
            IconName::LogOut => include_str!("icons/log-out.svg"),
            IconName::LayoutDashboard => include_str!("icons/layout-dashboard.svg"),
            IconName::Activity => include_str!("icons/activity.svg"),
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
    // The control's size wins over a part's side around it.
    provide_context(IconSideContext(None));
}

/// The size an icon without its own takes here: the control's, else `md`.
pub fn icon_size_here() -> Memo<Size> {
    match use_context::<IconSizeContext>() {
        Some(IconSizeContext(size)) => size,
        None => computed(|| Size::Md),
    }
}

/// Says the text here takes a color this library set (a button's, a
/// card's, a tooltip's): an icon or a spinner inside follows the text
/// around it. One outside every such part takes `foreground` instead, since NSH's own text
/// color, where nothing sets one, is black in both appearances.
#[derive(Debug, Clone, Copy)]
pub struct TextColorContext;

/// Says the text inside the current component takes a color this library
/// set (call it in the body of a component that colors its text).
pub fn provide_text_color() {
    provide_context(TextColorContext);
}

/// Whether a part of this library around here colors the text.
pub fn text_color_here() -> bool {
    use_context::<TextColorContext>().is_some()
}

/// A length icons take inside a part that sizes them apart from the
/// control sizes (an alert dialog's media: 32): provided by the part;
/// `None` inside a control within it.
#[derive(Debug, Clone, Copy)]
pub struct IconSideContext(pub Option<Memo<Points>>);

/// Gives the icons inside the current component `side` points when they
/// have no size of their own; the nearest of it and [`provide_icon_size`]
/// wins.
pub fn provide_icon_side(side: impl Fn() -> Points + 'static) {
    provide_context(IconSideContext(Some(computed(side))));
}

/// The props of [`Icon`].
#[derive(Debug, Clone, PartialEq, Props)]
pub struct IconProps {
    /// Which icon.
    pub name: Prop<IconName>,
    /// Its size: 12 / 14 / 16 / 16 points for `xs` / `sm` / `md` / `lg`
    /// (`icon-size-*`).
    /// Left out, the size of the control or part it is in (`md`
    /// elsewhere).
    pub size: Prop<Option<Size>>,
    /// Spin it (a spinner); it keeps spinning with reduced motion, as it
    /// says something is going on.
    #[prop(default)]
    pub spin: Prop<bool>,
}

/// An icon of the library, in the color of the text of the library's part
/// it is in (a button's, a badge's), `foreground` outside every such part;
/// `class` `text-…` goes over both.
#[component]
pub fn Icon(props: IconProps, _cx: Scope<NoEvent>) -> impl IntoElement {
    let t = theme();
    let here = icon_size_here();
    let part = use_context::<IconSideContext>().and_then(|IconSideContext(side)| side);
    let side = move || match (props.size.get(), part) {
        (Some(size), _) => t.length(icon_size_token(size)),
        (None, Some(side)) => side.get(),
        (None, None) => t.length(icon_size_token(here.get())),
    };
    let icon =
        icon_view(t, move || props.name.get(), side).style_when(props.spin, |s| s.animate_spin());
    // Inside a part that colors its text, it takes that color; elsewhere
    // the text color NSH passes is black whatever the appearance.
    if text_color_here() {
        icon
    } else {
        icon.text_color(move || t.color(FOREGROUND))
    }
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
