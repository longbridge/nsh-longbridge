//! The popup surface: the look every popup of the library shares
//! (`Popover`, and the menus' popups: `DropdownMenu`, `Select`'s list):
//! its fill, edge, corners, shadow, text and motion, from the `popover-*`
//! tokens. One set of tokens, so a theme that changes `popover-radius`
//! changes popovers and menus together. `Popover` owns them (its spec
//! lists them); each popup adds its own padding and width.

use nsh_core::{
    element::{Style, StyleRefinement, global},
    prelude::*,
};

use super::{motion, shadow, typography::line_height};
use crate::{
    theme::{
        Theme,
        seeds::SPACING,
        semantic::{BORDER, BORDER_WIDTH, POPOVER, POPOVER_FOREGROUND, RADIUS_MD, TEXT_SM},
    },
    tokens,
};

tokens! {
    /// A popup's edge.
    POPOVER_BORDER: Color = "popover-border" => |t| t.color(BORDER),
    /// A popup's corner radius.
    POPOVER_RADIUS: Length = "popover-radius" => |t| t.length(RADIUS_MD),
    /// A popup's shadow level (3: `md`).
    POPOVER_SHADOW: Number = "popover-shadow" => |t| t.value(3.0),
    /// The gap between a popup and its trigger (the side offset).
    POPOVER_OFFSET: Length = "popover-offset" => |t| t.length(SPACING),
    /// The distance a popup keeps from the edges of the safe area.
    POPOVER_MARGIN: Length = "popover-margin" => |t| t.length(SPACING),
    /// Whether popups fade, zoom and slide in and out (1) or show and hide
    /// in one frame (0).
    POPOVER_ANIMATE: Number = "popover-animate" => |t| t.value(1.0),
}

/// The popup surface on `el` (a popup's panel, or a popup part's style as
/// `nsh/base`'s `popupStyle` takes it): the `popover` fill and its text color,
/// `border-width` × `popover-border`, `popover-radius`, shadow tier
/// `popover-shadow`, the transitions of its entrance and exit, and its
/// text: 14 / 20, regular, wrapping, no underline, case change or letter
/// spacing, whatever the trigger's text is (an overlay keeps its parent's
/// inherited text styles). Each popup adds its padding and width.
pub fn surface<E: Styled>(el: E, t: Theme) -> E {
    let el = el
        .bg(move || t.color(POPOVER))
        .text_color(move || t.color(POPOVER_FOREGROUND))
        .border(move || t.length(BORDER_WIDTH))
        .border_color(move || t.color(POPOVER_BORDER))
        .rounded(move || t.length(POPOVER_RADIUS))
        .style_with(move |s| {
            s.shadow(shadow::box_shadows(
                t,
                shadow::level(t.number(POPOVER_SHADOW)),
            ))
        })
        .text_size(move || t.length(TEXT_SM))
        .line_height(move || line_height(t.length(TEXT_SM)))
        .whitespace_normal()
        .font_weight(FontWeight::NORMAL)
        .no_underline()
        .normal_case()
        .letter_spacing(px(0.0));
    motion::transition_all(el, t)
}

/// Whether popups move as they open and close (`popover-animate`). Read it
/// in a binding.
pub fn animate(t: Theme) -> bool {
    t.number(POPOVER_ANIMATE) != 0.0
}

/// The look a popup placed on `side` of its trigger starts from as it
/// shows: faded, at 95 %, 8 points toward its trigger (`None` when
/// `animate` is false). Give it as the popup's starting class.
pub fn starting_class(side: Placement, animate: bool) -> Option<SharedString> {
    animate.then(|| {
        let slide = match side {
            Placement::Bottom => "-translate-y-2",
            Placement::Top => "translate-y-2",
            Placement::Right => "-translate-x-2",
            Placement::Left => "translate-x-2",
        };
        SharedString::from(format!("opacity-0 scale-95 {slide}"))
    })
}

/// The look a popup ends at as it goes: faded, at 95 % (no slide); `None`
/// when `animate` is false. Give it as the popup's ending class.
pub fn ending_class(animate: bool) -> Option<SharedString> {
    animate.then(|| SharedString::from("opacity-0 scale-95"))
}

/// The scale origin of a popup on `side` of its trigger: the trigger's
/// side, so it zooms out of the trigger (`origin-top` below it).
pub fn origin(s: StyleRefinement, side: Placement) -> StyleRefinement {
    match side {
        Placement::Bottom => s.origin_top(),
        Placement::Top => s.origin_bottom(),
        Placement::Right => s.origin_left(),
        Placement::Left => s.origin_right(),
    }
}

/// The transitions a popup's entrance and exit run with (its starting and
/// ending looks to its look): every property over `duration`, Tailwind's
/// easing; none with reduced motion. Give it to the popup part (the view
/// the core places), whose looks change.
pub fn transitions(s: StyleRefinement, t: Theme) -> StyleRefinement {
    motion::transition_all(s, t)
}

/// How many of the library's popups are open on this surface.
#[derive(Debug, Clone, Copy, Default)]
struct OpenPopups(u32);

/// What the content of an open popup of the library is inside: a
/// `Tooltip` there still shows.
#[derive(Debug, Clone, Copy)]
pub(crate) struct InPopup;

/// Counts the current popup's content among the surface's open popups
/// while it is mounted, and tells what is inside it so (call it where the
/// content is made, on each opening). While one is open, a library
/// `Tooltip` outside every open popup stays hidden: the trigger's own would
/// otherwise show over the popup it opened (NSH shows a focused element's
/// hint whatever popup it opened).
pub(crate) fn hold_open() {
    provide_context(InPopup);
    global::<OpenPopups>().update(|n| n.0 += 1);
    cleanup(|| {
        if let Some(open) = try_global::<OpenPopups>() {
            open.update(|n| n.0 = n.0.saturating_sub(1));
        }
    });
}

/// Whether the current component is inside the content of a library
/// popup (a `Calendar` there drops its fill).
pub(crate) fn in_popup() -> bool {
    use_context::<InPopup>().is_some()
}

/// Whether a library popup is open on this surface and the current
/// component is not inside one (tracked): a `Tooltip` here stays hidden.
pub(crate) fn covered() -> bool {
    use_context::<InPopup>().is_none()
        && try_global::<OpenPopups>().is_some_and(|open| open.get().0 > 0)
}

/// Gives `el`'s popup (an `nsh/base` popup component's `popupStyle`) the
/// style `style` makes, bound: `nsh/base`'s Rust setter takes a style
/// built once, which could neither follow the tokens nor merge the app's
/// `contentStyle`, a prop. The prop is set as the setter sets it.
pub(crate) fn set_popup_style<B>(el: &mut B, style: impl Fn() -> Style + 'static)
where
    B: ComponentBuilder,
    <B::Component as Component>::Props: PopupStyleProp,
{
    el.__element()
        .__props_mut()
        .set_popup_style(Prop::from(IntoReactive::<Style>::into_reactive(style)));
}

/// The props of an `nsh/base` popup component with a `popupStyle`.
pub(crate) trait PopupStyleProp {
    /// Sets `popupStyle`.
    fn set_popup_style(&mut self, style: Prop<Style>);
}

impl PopupStyleProp for nsh_base::MenuProps {
    fn set_popup_style(&mut self, style: Prop<Style>) {
        self.popup_style = style;
    }
}

impl PopupStyleProp for nsh_base::DatePickerProps {
    fn set_popup_style(&mut self, style: Prop<Style>) {
        self.popup_style = style;
    }
}

impl PopupStyleProp for nsh_base::SelectProps {
    fn set_popup_style(&mut self, style: Prop<Style>) {
        self.popup_style = style;
    }
}
