//! The menu look: what every menu-like list shares (`DropdownMenu`,
//! `Select`'s list, later `ContextMenu`, `Combobox` and `Command`): its
//! rows (height, padding, corners, text, the highlight), their icons and
//! indicator column, separators, shortcuts, group labels, and the sheet a
//! touch screen shows the list in, from the `menu-*` tokens. The popup
//! around the rows is the popup surface (`popup`, the `popover-*`
//! tokens). One set of tokens, so a theme that changes `menu-item-focus`
//! changes them all. `DropdownMenu`'s spec owns them.

use nsh_core::{
    element::{StyleRefinement, horizontal_size_class, idiom, is_catalyst, is_desktop, os},
    platform::{Idiom, Os, SizeClass},
    prelude::*,
};

use super::{size::Size, typography::line_height};
use crate::{
    foundation::popup::POPOVER_RADIUS,
    theme::{
        Theme, Token, kind,
        seeds::SPACING,
        semantic::{
            ACCENT, ACCENT_FOREGROUND, BORDER, BORDER_WIDTH, CONTROL_HEIGHT_LG, CONTROL_HEIGHT_MD,
            CONTROL_HEIGHT_SM, CONTROL_HEIGHT_TOUCH_LG, CONTROL_HEIGHT_XS, DESTRUCTIVE,
            FONT_SIZE_LG, FONT_SIZE_MD, FONT_SIZE_SM, FONT_SIZE_XS, MUTED_FOREGROUND, RADIUS_LG,
            TOUCH_TARGET,
        },
    },
    tokens,
};

tokens! {
    /// The space between the surface's edge and its rows (`p-1`).
    MENU_PADDING: Length = "menu-padding" => |t| t.length(SPACING),
    /// The narrowest a menu's surface gets (`min-w-[8rem]`).
    MENU_MIN_WIDTH: Length = "menu-min-width" => |t| t.px(128.0),
    /// The gap between a menu and its submenu's popup: negative, so the
    /// submenu's first row is level with its trigger row, across the
    /// surface's padding and edge.
    MENU_SUB_OFFSET: Length = "menu-sub-offset" => |t| -(t.length(MENU_PADDING) + t.length(BORDER_WIDTH)),
    /// An extra small row's height.
    MENU_ITEM_HEIGHT_XS: Length = "menu-item-height-xs" => |t| t.pointer_touch(t.length(CONTROL_HEIGHT_XS) - 4.0, t.length(TOUCH_TARGET)),
    /// A small row's height.
    MENU_ITEM_HEIGHT_SM: Length = "menu-item-height-sm" => |t| t.pointer_touch(t.length(CONTROL_HEIGHT_SM) - 4.0, t.length(TOUCH_TARGET)),
    /// A medium row's height.
    MENU_ITEM_HEIGHT_MD: Length = "menu-item-height-md" => |t| t.pointer_touch(t.length(CONTROL_HEIGHT_MD) - 4.0, t.length(TOUCH_TARGET)),
    /// A large row's height (32).
    MENU_ITEM_HEIGHT_LG: Length = "menu-item-height-lg" => |t| t.pointer_touch(t.length(CONTROL_HEIGHT_LG) - 4.0, t.length(CONTROL_HEIGHT_TOUCH_LG)),
    /// An extra small row's text size: 14 on touch screens.
    MENU_ITEM_FONT_SIZE_XS: Length = "menu-item-font-size-xs" => |t| t.pointer_touch(t.length(FONT_SIZE_XS), t.px(14.0)),
    /// A small row's text size: 14 on touch screens.
    MENU_ITEM_FONT_SIZE_SM: Length = "menu-item-font-size-sm" => |t| t.pointer_touch(t.length(FONT_SIZE_SM), t.px(14.0)),
    /// A medium row's text size: 16 on touch screens, as the platforms'
    /// menus.
    MENU_ITEM_FONT_SIZE_MD: Length = "menu-item-font-size-md" => |t| t.pointer_touch(t.length(FONT_SIZE_MD), t.px(16.0)),
    /// A large row's text size: 16 on touch screens.
    MENU_ITEM_FONT_SIZE_LG: Length = "menu-item-font-size-lg" => |t| t.pointer_touch(t.length(FONT_SIZE_LG), t.px(16.0)),
    /// A row's horizontal padding (`px-2`).
    MENU_ITEM_PADDING_X: Length = "menu-item-padding-x" => |t| t.length(SPACING) * 2.0,
    /// The space between a row's icon or indicator, its label and its end.
    MENU_ITEM_GAP: Length = "menu-item-gap" => |t| t.length(SPACING) * 2.0,
    /// A row's corner radius: concentric with the surface's. Drawn between
    /// 0 and half the row's height.
    MENU_ITEM_RADIUS: Length = "menu-item-radius" => |t| t.length(POPOVER_RADIUS) - t.length(MENU_PADDING),
    /// The highlighted row's fill (the keys or the pointer on it).
    MENU_ITEM_FOCUS: Color = "menu-item-focus" => |t| t.color(ACCENT),
    /// The highlighted row's text and icons.
    MENU_ITEM_FOCUS_FOREGROUND: Color = "menu-item-focus-foreground" => |t| t.color(ACCENT_FOREGROUND),
    /// A row's leading icon and a submenu trigger's chevron.
    MENU_ITEM_ICON_FOREGROUND: Color = "menu-item-icon-foreground" => |t| t.color(MUTED_FOREGROUND),
    /// A destructive row's text and icons.
    MENU_ITEM_DESTRUCTIVE_FOREGROUND: Color = "menu-item-destructive-foreground" => |t| t.color(DESTRUCTIVE),
    /// A destructive row's fill while highlighted: `destructive` at 10 %,
    /// 20 % in the dark.
    MENU_ITEM_DESTRUCTIVE_FOCUS: Color = "menu-item-destructive-focus" => |t| t.light_dark(t.color(DESTRUCTIVE).opacity(0.1), t.color(DESTRUCTIVE).opacity(0.2)),
    /// The line between groups of rows.
    MENU_SEPARATOR: Color = "menu-separator" => |t| t.color(BORDER),
    /// A shortcut's text at a row's end.
    MENU_SHORTCUT_FOREGROUND: Color = "menu-shortcut-foreground" => |t| t.color(MUTED_FOREGROUND),
    /// A shortcut's text size: Tailwind's `text-xs`, explanatory text.
    MENU_SHORTCUT_FONT_SIZE: Length = "menu-shortcut-font-size" => |t| t.px(12.0),
    /// The top corners of the sheet a touch screen shows a menu in.
    MENU_SHEET_RADIUS: Length = "menu-sheet-radius" => |t| t.length(RADIUS_LG),
    /// The space between the sheet's edge and its rows.
    MENU_SHEET_PADDING: Length = "menu-sheet-padding" => |t| t.length(SPACING) * 2.0,
}

/// The group a menu row's parts (its icon, its chevron) take the row's
/// highlight from.
pub const ROW_GROUP: &str = "nlb-menu-item";

/// What a menu gives its rows: their size.
#[derive(Debug, Clone, Copy)]
pub struct MenuLook {
    /// The rows' size.
    pub size: Memo<Size>,
}

/// Gives the rows made inside the current component `size` (call it in the
/// menu's body, before its items).
pub fn provide_menu_look(size: impl Fn() -> Size + 'static) {
    provide_context(MenuLook {
        size: computed(size),
    });
}

/// The rows' size here: the menu's, else `md`.
pub fn menu_size() -> Memo<Size> {
    match use_context::<MenuLook>() {
        Some(look) => look.size,
        None => computed(|| Size::Md),
    }
}

/// Whether `nsh/base`'s menus show as a sheet here: on a touch screen,
/// except an iPad (not a Mac app made from one) in a regular width, whose
/// menus are popups. The same condition as `nsh/base`'s own, which it
/// does not export. Read it in a binding: an iPad window resized across
/// its size class changes it.
pub fn menu_as_sheet() -> bool {
    if is_desktop() {
        return false;
    }
    let ipad = os() == Os::Ios && idiom() == Idiom::Pad && !is_catalyst();
    !(ipad && horizontal_size_class() == SizeClass::Regular)
}

/// Whether a list (`Select`'s) shows as a sheet here: on every touch
/// screen.
pub fn list_as_sheet() -> bool {
    !is_desktop()
}

/// The row height token of `size`.
pub fn item_height_token(size: Size) -> Token<kind::Length> {
    size.pick(
        MENU_ITEM_HEIGHT_XS,
        MENU_ITEM_HEIGHT_SM,
        MENU_ITEM_HEIGHT_MD,
        MENU_ITEM_HEIGHT_LG,
    )
}

/// The row text size token of `size`.
pub fn item_font_size_token(size: Size) -> Token<kind::Length> {
    size.pick(
        MENU_ITEM_FONT_SIZE_XS,
        MENU_ITEM_FONT_SIZE_SM,
        MENU_ITEM_FONT_SIZE_MD,
        MENU_ITEM_FONT_SIZE_LG,
    )
}

/// A row's geometry and text on `el`: a row, its content centered
/// vertically and `menu-item-gap` apart, `menu-item-height-<size>` tall,
/// `menu-item-padding-x` at the sides, `menu-item-radius` corners,
/// `menu-item-font-size-<size>` text in one line, regular weight, the arrow
/// cursor. The caller adds its colors and states.
pub fn item_row<E: Styled>(el: E, t: Theme, size: impl Fn() -> Size + Copy + 'static) -> E {
    let height = move || t.length(item_height_token(size()));
    let font_size = move || t.length(item_font_size_token(size()));
    el.flex_row()
        .items_center()
        .flex_shrink_0()
        .gap(move || t.length(MENU_ITEM_GAP))
        .h(height)
        .px(move || t.length(MENU_ITEM_PADDING_X))
        .rounded(move || item_radius(t, height()))
        .text_size(font_size)
        .line_height(move || line_height(font_size()))
        .font_weight(FontWeight::NORMAL)
        .whitespace_nowrap()
        .cursor(Cursor::Default)
}

/// `menu-item-radius`, between 0 and half a row `height` tall.
pub fn item_radius(t: Theme, height: Points) -> Points {
    t.length(MENU_ITEM_RADIUS)
        .clamp(px(0.0), (height / 2.0).max(px(0.0)))
}

/// The highlight of a row (`focus:`, `active:` on touch screens, a
/// submenu trigger's `expanded:`): the `menu-item-focus` fill and
/// `menu-item-focus-foreground` text, or a destructive row's
/// `menu-item-destructive-focus` fill with its text unchanged.
pub fn highlight(
    s: StyleRefinement,
    t: Theme,
    destructive: impl Fn() -> bool + Copy + 'static,
) -> StyleRefinement {
    s.bg(move || {
        if destructive() {
            t.color(MENU_ITEM_DESTRUCTIVE_FOCUS)
        } else {
            t.color(MENU_ITEM_FOCUS)
        }
    })
    .text_color(move || {
        if destructive() {
            t.color(MENU_ITEM_DESTRUCTIVE_FOREGROUND)
        } else {
            t.color(MENU_ITEM_FOCUS_FOREGROUND)
        }
    })
}

/// The color of a row's leading icon or chevron on `el`
/// (`menu-item-icon-foreground`, a destructive row's text color), turning
/// the highlight's text color while the row ([`ROW_GROUP`]) is focused or
/// pressed.
pub fn row_icon_color<E: Styled>(
    el: E,
    t: Theme,
    destructive: impl Fn() -> bool + Copy + 'static,
) -> E {
    let lit = move || {
        if destructive() {
            t.color(MENU_ITEM_DESTRUCTIVE_FOREGROUND)
        } else {
            t.color(MENU_ITEM_FOCUS_FOREGROUND)
        }
    };
    el.text_color(move || {
        if destructive() {
            t.color(MENU_ITEM_DESTRUCTIVE_FOREGROUND)
        } else {
            t.color(MENU_ITEM_ICON_FOREGROUND)
        }
    })
    .group_focus_within_named(ROW_GROUP, move |s| s.text_color(lit))
    .group_active_named(ROW_GROUP, move |s| s.text_color(lit))
}

/// The line between groups of rows on `el`: `border-width` tall in
/// `menu-separator`, across the surface's padding (`-mx` `menu-padding`),
/// `menu-padding` above and below.
pub fn separator<E: Styled>(el: E, t: Theme) -> E {
    el.h(move || t.length(BORDER_WIDTH))
        .flex_shrink_0()
        .mx(move || -t.length(MENU_PADDING))
        .my(move || t.length(MENU_PADDING))
        .bg(move || t.color(MENU_SEPARATOR))
}

/// A shortcut's text on `el`: at the row's end (`margin-left: auto`),
/// `menu-shortcut-font-size` with its line height, letters 0.1 em apart
/// (`tracking-widest`), in `menu-shortcut-foreground`.
pub fn shortcut<E: Styled>(el: E, t: Theme) -> E {
    let font_size = move || t.length(MENU_SHORTCUT_FONT_SIZE);
    el.ml(auto())
        .flex_shrink_0()
        .text_size(font_size)
        .line_height(move || line_height(font_size()))
        .letter_spacing(move || font_size() * 0.1)
        .text_color(move || t.color(MENU_SHORTCUT_FOREGROUND))
}
