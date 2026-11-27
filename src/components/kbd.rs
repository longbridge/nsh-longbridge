//! `Kbd` and `KbdGroup`: shadcn/ui's key caps, a key or a key combination
//! drawn as small caps. No behavior. API: `docs/components/kbd.md`.

use nsh_core::prelude::*;

use crate::{
    components::tooltip::{TOOLTIP_FOREGROUND, in_tooltip},
    foundation::{
        icon::{provide_icon_size, provide_text_color},
        size::Size,
        typography::{line_height, medium},
    },
    theme::{
        seeds::SPACING,
        semantic::{FONT_SIZE_XS, MUTED, MUTED_FOREGROUND, RADIUS_SM},
        theme,
    },
    tokens,
};

tokens! {
    /// The cap's fill.
    KBD: Color = "kbd" => |t| t.color(MUTED),
    /// The cap's text and icons.
    KBD_FOREGROUND: Color = "kbd-foreground" => |t| t.color(MUTED_FOREGROUND),
    /// The cap's fill inside a tooltip: its text color at 20 %, 10 % in the
    /// dark (shadcn's `bg-background/20`, `dark:bg-background/10` there).
    KBD_TOOLTIP: Color = "kbd-tooltip" => |t| t.light_dark(t.color(TOOLTIP_FOREGROUND).opacity(0.2), t.color(TOOLTIP_FOREGROUND).opacity(0.1)),
    /// The cap's text inside a tooltip.
    KBD_TOOLTIP_FOREGROUND: Color = "kbd-tooltip-foreground" => |t| t.color(TOOLTIP_FOREGROUND),
    /// The cap's corner radius.
    KBD_RADIUS: Length = "kbd-radius" => |t| t.length(RADIUS_SM),
    /// The cap's height, and its least width (a single character is
    /// square).
    KBD_HEIGHT: Length = "kbd-height" => |t| t.length(SPACING) * 5.0,
    /// The cap's horizontal padding.
    KBD_PADDING_X: Length = "kbd-padding-x" => |t| t.length(SPACING),
    /// The cap's text size.
    KBD_FONT_SIZE: Length = "kbd-font-size" => |t| t.length(FONT_SIZE_XS),
    /// The gap between the caps of a `KbdGroup`.
    KBD_GROUP_GAP: Length = "kbd-group-gap" => |t| t.length(SPACING),
}

/// The props of [`Kbd`].
#[derive(Debug, Clone, PartialEq, Props)]
pub struct KbdProps {
    /// The key's name (`⌘`, `K`, `Ctrl`), or one of the library's icons.
    pub children: Prop<Children>,
}

/// A key cap, shadcn/ui's: a key's name as a small muted cap, for
/// shortcuts in buttons, menus and tooltips. It does not parse key names:
/// give it the platform's form (`keymap.label("mod+k")`). Inside a
/// `Tooltip` it takes the tooltip's colors.
#[component]
pub fn Kbd(props: KbdProps, _cx: Scope<NoEvent>) -> impl IntoElement {
    provide_text_color();
    let t = theme();
    let tooltip = in_tooltip();
    let font_size = move || t.length(KBD_FONT_SIZE);
    provide_icon_size(|| Size::Xs);
    view()
        .flex_row()
        .items_center()
        .justify_center()
        .flex_shrink_0()
        .gap(move || t.length(SPACING))
        .h(move || t.length(KBD_HEIGHT))
        .min_w(move || t.length(KBD_HEIGHT))
        .px(move || t.length(KBD_PADDING_X))
        .rounded(move || t.length(KBD_RADIUS))
        .bg(move || {
            if tooltip {
                t.color(KBD_TOOLTIP)
            } else {
                t.color(KBD)
            }
        })
        .text_color(move || {
            if tooltip {
                t.color(KBD_TOOLTIP_FOREGROUND)
            } else {
                t.color(KBD_FOREGROUND)
            }
        })
        .text_size(font_size)
        .line_height(move || line_height(font_size()))
        .font_weight(medium(t))
        .whitespace_nowrap()
        .select_none()
        .pointer_events(PointerEvents::None)
        .children_each(props.children, |child, _| text("").child(child))
}

/// The props of [`KbdGroup`].
#[derive(Debug, Clone, PartialEq, Props)]
pub struct KbdGroupProps {
    /// The caps, and the app's own words between them (`+`, `then`).
    pub children: Prop<Children>,
}

/// Key caps in a row, `kbd-group-gap` apart: a key combination
/// (`Ctrl` `+` `B`).
#[component]
pub fn KbdGroup(props: KbdGroupProps, _cx: Scope<NoEvent>) -> impl IntoElement {
    let t = theme();
    view()
        .flex_row()
        .items_center()
        .gap(move || t.length(KBD_GROUP_GAP))
        .child(props.children)
}
