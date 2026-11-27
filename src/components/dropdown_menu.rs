//! `DropdownMenu` and its parts: a dropdown menu on `nsh/base`'s
//! `Menu`, `MenuItem`, `MenuSeparator` and `Submenu` (the popup at the
//! trigger or a sheet on touch screens, the keys, typeahead, submenus,
//! dismissal, the menu roles and their states for screen readers). The
//! library adds the look (`foundation::popup`'s surface, `foundation::menu`'s
//! rows), the item kinds (checkbox, radio, label, shortcut, destructive,
//! inset) and the radio group's value. API:
//! `docs/components/dropdown-menu.md`.

use nsh_base::{
    MenuEvent as BaseMenuEvent, OpenChangeReason, SubmenuEvent as BaseSubmenuEvent,
    menu as base_menu, menu_item as base_item, menu_separator as base_separator,
    submenu as base_submenu, terminal::is_terminal,
};
use nsh_core::{
    element::{ImageSrc, Slot, Style, StyleRefinement, TextPartStyle, safe_area},
    prelude::*,
};

use crate::{
    foundation::{
        icon::{IconName, icon_size_token, icon_view, provide_icon_size, provide_text_color},
        labels::{self, Label},
        menu::{
            MENU_ITEM_FOCUS_FOREGROUND, MENU_MIN_WIDTH, MENU_PADDING, MENU_SHEET_PADDING,
            MENU_SHEET_RADIUS, MENU_SUB_OFFSET, ROW_GROUP, highlight, item_row, menu_as_sheet,
            menu_size, provide_menu_look, row_icon_color, separator, shortcut,
        },
        popup::{self, POPOVER_BORDER, POPOVER_MARGIN, POPOVER_OFFSET, set_popup_style},
        size::Size,
        trigger,
        typography::medium,
    },
    theme::{
        Theme,
        semantic::{BORDER_WIDTH, OPACITY_DISABLED, OVERLAY, POPOVER_FOREGROUND},
        theme,
    },
};

/// A radio item's dot: a filled circle in the text's color.
const DOT: &str = r#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 8 8"><circle cx="4" cy="4" r="4" fill="currentColor"/></svg>"#;

/// The props of [`DropdownMenu`].
#[derive(Debug, Clone, PartialEq, Props)]
pub struct DropdownMenuProps {
    /// What opens it: usually a `Button`. The menu sets its expanded and
    /// has-popup states, its press and keys, and places the popup against
    /// it.
    pub trigger: Prop<Option<Slot>>,
    /// Open (controlled): the app owns the value and changes it in
    /// `onOpenChange`. Left out, the menu keeps its own state.
    pub open: Prop<Option<bool>>,
    /// Open at first, while `open` is left out.
    #[prop(default)]
    pub default_open: Prop<bool>,
    /// The side of the trigger it goes on (`"bottom"` by default); the
    /// other side when only that one has room.
    #[prop(default)]
    pub placement: Prop<Placement>,
    /// How it lines up with the trigger along that side (`"start"` by
    /// default).
    #[prop(default = Align::Start)]
    pub align: Prop<Align>,
    /// The items' size (`xs`, `sm`, `md`, `lg`): their height, text, icon
    /// and padding.
    #[prop(default)]
    pub size: Prop<Size>,
    /// Tailwind classes of the popup (or sheet): they add to the library's
    /// look (`w-56`) but cannot override what it sets inline (fill, edge,
    /// corners, padding, shadow).
    pub content_class: Prop<Option<SharedString>>,
    /// The inline style of the popup (or sheet), over the library's look
    /// (`{ width: 224 }`).
    #[prop(default)]
    pub content_style: Prop<Style>,
    /// The items (any of this library's menu parts, and the app's own
    /// views), made on each opening.
    #[prop(lazy)]
    pub children: Prop<Children>,
}

/// The events of a [`DropdownMenu`].
#[derive(Debug, Clone, PartialEq, Event)]
pub enum DropdownMenuEvent {
    /// The user asks to open or close it (the trigger, a press outside,
    /// Escape or back, an item chosen), with why: set `open` to follow it.
    #[args(open, reason)]
    OpenChange(bool, OpenChangeReason),
    /// It is shown (a sheet once it came in).
    Open,
    /// It is gone, after its exit (whoever closed it).
    Close,
}

/// A dropdown menu: a list of commands and settings its
/// `trigger` opens, at the trigger on desktops (a sheet at the bottom on
/// phones). Its items are [`DropdownMenuItem`]s, checkbox and radio items,
/// labels, separators and submenus. The keys move between the items and
/// type ahead; choosing one, a press outside or Escape closes it.
#[component]
pub fn DropdownMenu(props: DropdownMenuProps, cx: Scope<DropdownMenuEvent>) -> impl IntoElement {
    let t = theme();
    provide_menu_look(move || props.size.get());
    // The side the popup went on, for its entrance's slide.
    let side = signal(props.placement.get_untracked());
    let terminal = is_terminal();
    // The open state, the app's or its own: kept here (`nsh/base`'s menu
    // is controlled) so a library `Button` as the trigger, which takes the
    // press itself, can open it too.
    let own_open = signal(props.default_open.get_untracked());
    let is_open = move || props.open.get().unwrap_or_else(|| own_open.get());
    let one = trigger::OnePress::new();
    let change = move |open: bool, reason: OpenChangeReason| {
        if reason == OpenChangeReason::TriggerPress && !one.first() {
            return;
        }
        if props.open.get_untracked().is_none() {
            own_open.set(open);
        }
        cx.emit(DropdownMenuEvent::OpenChange(open, reason));
    };
    trigger::provide(
        move || change(!untracked(is_open), OpenChangeReason::TriggerPress),
        is_open,
    );
    let mut root = base_menu()
        .open(move || Some(is_open()))
        .placement(props.placement)
        .align(props.align)
        .offset(move || t.length(POPOVER_OFFSET))
        .margin(move || t.length(POPOVER_MARGIN))
        .trigger(props.trigger.place(|slot| slot))
        .on_open_change(move |e: &BaseMenuEvent| {
            if let BaseMenuEvent::OpenChange(open, reason) = e {
                change(*open, *reason);
            }
        })
        .on_open(move || cx.emit(DropdownMenuEvent::Open))
        .on_close(move || cx.emit(DropdownMenuEvent::Close))
        .on_position(move |e: &BaseMenuEvent| {
            if let BaseMenuEvent::Position(placement, ..) = e {
                side.set(*placement);
            }
        });
    if !terminal {
        root = root
            .popup_class(props.content_class)
            .popup_starting_class(move || popup::starting_class(side.get(), popup::animate(t)))
            .popup_ending_class(move || popup::ending_class(popup::animate(t)))
            .backdrop_style(move |s| s.bg(move || t.color(OVERLAY)));
        let content_style = props.content_style;
        set_popup_style(&mut root, move || {
            let look = if menu_as_sheet() {
                sheet_look(t)
            } else {
                popup_look(t, side.get())
            };
            Style::from(look).then(&content_style.get())
        });
    }
    // Made on each opening: counted among the open popups while it is
    // there, so the trigger's tooltip stays hidden over the menu.
    root.child(dynamic(move || {
        untracked(|| {
            popup::hold_open();
            dropdown_menu_items().child(props.children)
        })
    }))
}

/// The props of [`DropdownMenuItems`].
#[derive(Debug, Clone, PartialEq, Props)]
struct DropdownMenuItemsProps {
    /// The menu's items.
    children: Prop<Children>,
}

/// A menu's items: a component of their own so that they are not the
/// menu's trigger (`trigger::clear`), as they would be under `nsh/base`'s
/// menu alone (the trigger is under it too).
#[component]
fn DropdownMenuItems(props: DropdownMenuItemsProps, _cx: Scope<NoEvent>) -> impl IntoElement {
    provide_text_color();
    trigger::clear();
    props.children
}

/// The menu's popup: the popup surface, `menu-padding` inside, at least
/// `menu-min-width` wide, zooming from the trigger's side.
fn popup_look(t: Theme, side: Placement) -> StyleRefinement {
    let s = popup::surface(StyleRefinement::default(), t)
        .p(move || t.length(MENU_PADDING))
        .min_w(move || t.length(MENU_MIN_WIDTH));
    popup::origin(s, side)
}

/// The menu's sheet: the popup surface's fill and text, its top corners
/// `menu-sheet-radius`, a top edge, `menu-sheet-padding` inside and the
/// bottom safe area below, no shadow (a drawer's look).
pub(crate) fn sheet_look(t: Theme) -> StyleRefinement {
    popup::surface(StyleRefinement::default(), t)
        .rounded(px(0.0))
        .rounded_t(move || t.length(MENU_SHEET_RADIUS))
        .border(px(0.0))
        .border_t(move || t.length(BORDER_WIDTH))
        .border_color(move || t.color(POPOVER_BORDER))
        .style_with(|s| s.shadow(Vec::<BoxShadow>::new()))
        .p(move || t.length(MENU_SHEET_PADDING))
        .pb(move || t.length(MENU_SHEET_PADDING) + safe_area().bottom)
}

/// A menu item's look.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, PropEnum)]
pub enum MenuItemVariant {
    /// A command.
    #[default]
    Default,
    /// Deletes or leaves something: red text and highlight.
    Destructive,
}

nsh_core::static_reactive!(MenuItemVariant: MenuItemVariant);

/// The row look on an item kind's root (`nsh/base`'s `MenuItem`): its
/// geometry, its text color at rest (`destructive` or the surface's), the
/// highlight while focused or pressed (and `expanded`, a submenu
/// trigger's), the disabled opacity, the group its icons follow.
fn row<B: ComponentBuilder + Styled>(
    el: B,
    t: Theme,
    size: Memo<Size>,
    destructive: impl Fn() -> bool + Copy + 'static,
) -> B {
    if is_terminal() {
        return el;
    }
    item_row(el, t, move || size.get())
        .text_color(move || {
            if destructive() {
                t.color(crate::foundation::menu::MENU_ITEM_DESTRUCTIVE_FOREGROUND)
            } else {
                t.color(POPOVER_FOREGROUND)
            }
        })
        .group_named(ROW_GROUP)
        .focus(move |s| highlight(s, t, destructive))
        .active(move |s| highlight(s, t, destructive))
        .expanded(move |s| highlight(s, t, destructive))
        .disabled_style(move |s| s.opacity(move || t.number(OPACITY_DISABLED)))
}

/// A row's leading part: its icon (`menu-item-icon-foreground`, the
/// highlight's text color while `lit`), else a blank of the icon's side
/// for an `inset` row, else nothing.
#[allow(clippy::too_many_arguments)]
fn leading(
    t: Theme,
    size: Memo<Size>,
    icon: Prop<Option<IconName>>,
    inset: Prop<bool>,
    class: Prop<Option<SharedString>>,
    style: Prop<Style>,
    destructive: impl Fn() -> bool + Copy + 'static,
    lit: impl Fn() -> bool + Copy + 'static,
) -> AnyElement {
    let side = move || t.length(icon_size_token(size.get()));
    dynamic(move || match icon.get() {
        Some(name) => Some(
            row_icon_color(icon_view(t, move || name, side), t, destructive)
                .style_when(lit, move |s| {
                    s.text_color(move || t.color(MENU_ITEM_FOCUS_FOREGROUND))
                })
                .app_class(class)
                .inline_style(move || style.get())
                .into_element(),
        ),
        None => inset.get().then(|| blank(t, size).into_element()),
    })
}

/// A blank the side of a row's icon: the indicator column of an `inset`
/// row.
fn blank(t: Theme, size: Memo<Size>) -> View {
    let side = move || t.length(icon_size_token(size.get()));
    view().w(side).h(side).flex_shrink_0()
}

/// A row's children, each in a `label` text; the first takes the room
/// left (cut with an ellipsis), so a `DropdownMenuShortcut` after it sits
/// at the row's end.
fn labels_of<B: ParentElement>(
    el: B,
    children: Prop<Children>,
    class: Prop<Option<SharedString>>,
    style: Prop<TextPartStyle>,
) -> B {
    el.children_each(children, move |child, at| {
        text("")
            .when(at.index() == 0, |t| t.flex_1().min_w(px(0.0)).truncate())
            .app_class(class)
            .inline_style(move || Style::from(style.get()))
            .child(child)
    })
}

/// The props of [`DropdownMenuItem`].
#[derive(Debug, Clone, PartialEq, Props)]
pub struct DropdownMenuItemProps {
    /// Its look: `default`, or `destructive` (red text and highlight) for
    /// deleting or leaving.
    #[prop(default)]
    pub variant: Prop<MenuItemVariant>,
    /// An icon before the label.
    pub icon: Prop<Option<IconName>>,
    /// The label starts where an icon's or a check's label would.
    #[prop(default)]
    pub inset: Prop<bool>,
    /// Not chosen, skipped by the keys, at half opacity.
    #[prop(default)]
    pub disabled: Prop<bool>,
    /// Choosing it closes the menu (the default).
    #[prop(default = true)]
    pub close_on_select: Prop<bool>,
    /// What screen readers say and typeahead matches, when the text is not
    /// enough.
    pub accessibility_label: Prop<Option<SharedString>>,
    /// More about it, read after its name.
    pub accessibility_description: Prop<Option<SharedString>>,
    /// Tailwind classes of the label: the part each child sits in.
    pub label_class: Prop<Option<SharedString>>,
    /// The inline style of the label part.
    #[prop(default)]
    pub label_style: Prop<TextPartStyle>,
    /// Tailwind classes of the icon.
    pub icon_class: Prop<Option<SharedString>>,
    /// The inline style of the icon.
    #[prop(default)]
    pub icon_style: Prop<Style>,
    /// Its label, then a `DropdownMenuShortcut` if any.
    pub children: Prop<Children>,
}

/// The events of a [`DropdownMenuItem`].
#[derive(Debug, Clone, PartialEq, Event)]
pub enum DropdownMenuItemEvent {
    /// It was chosen (a press, Return or Space).
    Select,
}

/// A command of a [`DropdownMenu`]: choosing it reports `onSelect` and
/// closes the menu.
#[component]
pub fn DropdownMenuItem(
    props: DropdownMenuItemProps,
    cx: Scope<DropdownMenuItemEvent>,
) -> impl IntoElement {
    let t = theme();
    let size = menu_size();
    provide_icon_size(move || size.get());
    let destructive = move || props.variant.get() == MenuItemVariant::Destructive;
    let el = base_item()
        .disabled(props.disabled)
        .close_on_select(props.close_on_select)
        .accessibility_label(props.accessibility_label)
        .accessibility_description(props.accessibility_description)
        .on_select(move || cx.emit(DropdownMenuItemEvent::Select));
    let el = row(el, t, size, destructive).child(leading(
        t,
        size,
        props.icon,
        props.inset,
        props.icon_class,
        props.icon_style,
        destructive,
        || false,
    ));
    labels_of(el, props.children, props.label_class, props.label_style)
}

/// The box at a checkable row's start holding its mark (`icon-size-<size>`
/// square), with the app's `indicatorClass` / `indicatorStyle`.
fn indicator_box(
    t: Theme,
    size: Memo<Size>,
    class: Prop<Option<SharedString>>,
    style: Prop<Style>,
    mark: impl Fn() -> Option<AnyElement> + 'static,
) -> View {
    let side = move || t.length(icon_size_token(size.get()));
    view()
        .w(side)
        .h(side)
        .flex_shrink_0()
        .items_center()
        .justify_center()
        .accessibility_elements_hidden(true)
        .app_class(class)
        .inline_style(move || style.get())
        .child(dynamic(mark))
}

/// The props of [`DropdownMenuCheckboxItem`].
#[derive(Debug, Clone, PartialEq, Props)]
pub struct DropdownMenuCheckboxItemProps {
    /// Checked (controlled): the check shows the app's value; choosing the
    /// item asks for the other one with `onChange`.
    #[prop(default)]
    pub checked: Prop<bool>,
    /// Not chosen, skipped by the keys, at half opacity.
    #[prop(default)]
    pub disabled: Prop<bool>,
    /// Choosing it closes the menu (the default).
    #[prop(default = true)]
    pub close_on_select: Prop<bool>,
    /// What screen readers say and typeahead matches, when the text is not
    /// enough.
    pub accessibility_label: Prop<Option<SharedString>>,
    /// More about it, read after its name.
    pub accessibility_description: Prop<Option<SharedString>>,
    /// Tailwind classes of the indicator: the box at its start holding the
    /// check.
    pub indicator_class: Prop<Option<SharedString>>,
    /// The inline style of the indicator.
    #[prop(default)]
    pub indicator_style: Prop<Style>,
    /// Tailwind classes of the label.
    pub label_class: Prop<Option<SharedString>>,
    /// The inline style of the label part.
    #[prop(default)]
    pub label_style: Prop<TextPartStyle>,
    /// Its label, then a `DropdownMenuShortcut` if any.
    pub children: Prop<Children>,
}

/// The events of a [`DropdownMenuCheckboxItem`].
#[derive(Debug, Clone, PartialEq, Event)]
pub enum DropdownMenuCheckboxItemEvent {
    /// It was chosen: the value it asks for (the opposite of `checked`).
    #[args(checked)]
    Change(bool),
}

/// A setting on or off in a [`DropdownMenu`], a check before its label
/// while on.
#[component]
pub fn DropdownMenuCheckboxItem(
    props: DropdownMenuCheckboxItemProps,
    cx: Scope<DropdownMenuCheckboxItemEvent>,
) -> impl IntoElement {
    let t = theme();
    let size = menu_size();
    provide_icon_size(move || size.get());
    let checked = props.checked;
    let side = move || t.length(icon_size_token(size.get()));
    let el = base_item()
        .checked(move || Some(checked.get()))
        .disabled(props.disabled)
        .close_on_select(props.close_on_select)
        .accessibility_label(props.accessibility_label)
        .accessibility_description(props.accessibility_description)
        .on_select(move || {
            cx.emit(DropdownMenuCheckboxItemEvent::Change(
                !checked.get_untracked(),
            ))
        });
    let el = if is_terminal() {
        el
    } else {
        el.indicator(indicator_box(
            t,
            size,
            props.indicator_class,
            props.indicator_style,
            move || {
                checked
                    .get()
                    .then(|| icon_view(t, || IconName::Check, side).into_element())
            },
        ))
    };
    labels_of(
        row(el, t, size, || false),
        props.children,
        props.label_class,
        props.label_style,
    )
}

/// What a radio group gives its items.
#[derive(Clone, Copy)]
struct RadioContext {
    value: Prop<Option<SharedString>>,
    cx: Scope<DropdownMenuRadioGroupEvent>,
}

/// The props of [`DropdownMenuRadioGroup`].
#[derive(Debug, Clone, PartialEq, Props)]
pub struct DropdownMenuRadioGroupProps {
    /// The value of the checked item (controlled): the app sets it in
    /// `onChange`.
    pub value: Prop<Option<SharedString>>,
    /// Its radio items, and labels or separators between them.
    pub children: Prop<Children>,
}

/// The events of a [`DropdownMenuRadioGroup`].
#[derive(Debug, Clone, PartialEq, Event)]
pub enum DropdownMenuRadioGroupEvent {
    /// An item other than the checked one was chosen: its value.
    #[args(value)]
    Change(SharedString),
}

/// The value of the [`DropdownMenuRadioItem`]s inside it: one of a set
/// (a sort order, a position).
#[component]
pub fn DropdownMenuRadioGroup(
    props: DropdownMenuRadioGroupProps,
    cx: Scope<DropdownMenuRadioGroupEvent>,
) -> impl IntoElement {
    provide_context(RadioContext {
        value: props.value,
        cx,
    });
    view().flex_col().child(props.children)
}

/// The props of [`DropdownMenuRadioItem`].
#[derive(Debug, Clone, PartialEq, Props)]
pub struct DropdownMenuRadioItemProps {
    /// What choosing it sets its group's value to.
    #[prop(default)]
    pub value: Prop<SharedString>,
    /// Not chosen, skipped by the keys, at half opacity.
    #[prop(default)]
    pub disabled: Prop<bool>,
    /// Choosing it closes the menu (the default).
    #[prop(default = true)]
    pub close_on_select: Prop<bool>,
    /// What screen readers say and typeahead matches, when the text is not
    /// enough.
    pub accessibility_label: Prop<Option<SharedString>>,
    /// More about it, read after its name.
    pub accessibility_description: Prop<Option<SharedString>>,
    /// Tailwind classes of the indicator: the box at its start holding the
    /// dot.
    pub indicator_class: Prop<Option<SharedString>>,
    /// The inline style of the indicator.
    #[prop(default)]
    pub indicator_style: Prop<Style>,
    /// Tailwind classes of the label.
    pub label_class: Prop<Option<SharedString>>,
    /// The inline style of the label part.
    #[prop(default)]
    pub label_style: Prop<TextPartStyle>,
    /// Its label.
    pub children: Prop<Children>,
}

/// One choice of a [`DropdownMenuRadioGroup`], a dot before its label
/// while it is the group's value. Outside a group it is never checked.
#[component]
pub fn DropdownMenuRadioItem(
    props: DropdownMenuRadioItemProps,
    _cx: Scope<NoEvent>,
) -> impl IntoElement {
    let t = theme();
    let size = menu_size();
    provide_icon_size(move || size.get());
    let group = use_context::<RadioContext>();
    if group.is_none() {
        #[cfg(debug_assertions)]
        log::warn!(
            "a `DropdownMenuRadioItem` is outside a `DropdownMenuRadioGroup`: it is never checked"
        );
    }
    let checked =
        move || group.is_some_and(|g| g.value.with(|v| v.as_ref() == Some(&props.value.get())));
    let el = base_item()
        .role(Some(AccessibilityRole::MenuItemRadio))
        .checked(move || Some(checked()))
        .disabled(props.disabled)
        .close_on_select(props.close_on_select)
        .accessibility_label(props.accessibility_label)
        .accessibility_description(props.accessibility_description)
        .on_select(move || {
            if let Some(g) = group
                && !checked()
            {
                g.cx.emit(DropdownMenuRadioGroupEvent::Change(
                    props.value.get_untracked(),
                ));
            }
        });
    let el = if is_terminal() {
        el
    } else {
        let dot = move || t.length(icon_size_token(size.get())) / 2.0;
        el.indicator(indicator_box(
            t,
            size,
            props.indicator_class,
            props.indicator_style,
            move || {
                checked().then(|| {
                    svg(|| ImageSrc::svg_markup(DOT))
                        .w(dot)
                        .h(dot)
                        .into_element()
                })
            },
        ))
    };
    labels_of(
        row(el, t, size, || false),
        props.children,
        props.label_class,
        props.label_style,
    )
}

/// The props of [`DropdownMenuLabel`].
#[derive(Debug, Clone, PartialEq, Props)]
pub struct DropdownMenuLabelProps {
    /// It starts where an icon's or a check's label would.
    #[prop(default)]
    pub inset: Prop<bool>,
    /// Its text.
    pub children: Prop<Children>,
}

/// A heading over a group of items: plain text, not an item (the keys and
/// typeahead pass it over).
#[component]
pub fn DropdownMenuLabel(props: DropdownMenuLabelProps, _cx: Scope<NoEvent>) -> impl IntoElement {
    let t = theme();
    let size = menu_size();
    let el = if is_terminal() {
        view().flex_row()
    } else {
        item_row(view(), t, move || size.get())
            .text_color(move || t.color(POPOVER_FOREGROUND))
            .font_weight(medium(t))
    };
    el.child(dynamic(move || props.inset.get().then(|| blank(t, size))))
        .children_each(props.children, |child, _| {
            text("").flex_1().min_w(px(0.0)).truncate().child(child)
        })
}

/// The props of [`DropdownMenuSeparator`].
#[derive(Debug, Clone, PartialEq, Props)]
pub struct DropdownMenuSeparatorProps {}

/// A line between groups of items.
#[component]
pub fn DropdownMenuSeparator(
    _props: DropdownMenuSeparatorProps,
    _cx: Scope<NoEvent>,
) -> impl IntoElement {
    let t = theme();
    let el = base_separator();
    if is_terminal() { el } else { separator(el, t) }
}

/// The props of [`DropdownMenuShortcut`].
#[derive(Debug, Clone, PartialEq, Props)]
pub struct DropdownMenuShortcutProps {
    /// The shortcut: its text (`keymap.label("mod+s")`), or `Kbd`s.
    pub children: Prop<Children>,
}

/// The shortcut at the end of an item, muted and spaced out; screen
/// readers read it with the item.
#[component]
pub fn DropdownMenuShortcut(
    props: DropdownMenuShortcutProps,
    _cx: Scope<NoEvent>,
) -> impl IntoElement {
    let t = theme();
    // A text, so the row's label reads on into it (a view inside a text
    // would be an attachment there).
    shortcut(text(""), t).child(props.children)
}

/// The props of [`DropdownMenuGroup`].
#[derive(Debug, Clone, PartialEq, Props)]
pub struct DropdownMenuGroupProps {
    /// Its items.
    pub children: Prop<Children>,
}

/// A plain column around items, for structure: no look, no role.
#[component]
pub fn DropdownMenuGroup(props: DropdownMenuGroupProps, _cx: Scope<NoEvent>) -> impl IntoElement {
    view().flex_col().child(props.children)
}

/// The props of [`DropdownMenuSub`].
#[derive(Debug, Clone, PartialEq, Props)]
pub struct DropdownMenuSubProps {
    /// The item that opens it: a `DropdownMenuSubTrigger`.
    pub trigger: Prop<Option<Slot>>,
    /// Open (controlled), as the app has it; left out, it keeps its own
    /// state.
    pub open: Prop<Option<bool>>,
    /// It does not open; its trigger is dimmed.
    #[prop(default)]
    pub disabled: Prop<bool>,
    /// In a sheet: the text of the row that goes back up ("Back" by
    /// default, translated with the library's words).
    pub back_label: Prop<Option<SharedString>>,
    /// Its items, made as it opens.
    #[prop(lazy)]
    pub children: Prop<Children>,
}

/// The events of a [`DropdownMenuSub`].
#[derive(Debug, Clone, PartialEq, Event)]
pub enum DropdownMenuSubEvent {
    /// The user asks to open or close it, with why.
    #[args(open, reason)]
    OpenChange(bool, OpenChangeReason),
}

/// A menu inside a [`DropdownMenu`]: beside its trigger item in a popup,
/// in place of the items in a sheet (a back row first).
#[component]
pub fn DropdownMenuSub(
    props: DropdownMenuSubProps,
    cx: Scope<DropdownMenuSubEvent>,
) -> impl IntoElement {
    let t = theme();
    let size = menu_size();
    let back_label = props.back_label;
    let side = move || t.length(icon_size_token(size.get()));
    let back = item_row(view(), t, move || size.get())
        .text_color(move || t.color(POPOVER_FOREGROUND))
        .active(move |s| highlight(s, t, || false))
        .child(row_icon_color(
            icon_view(t, || IconName::ChevronLeft, side),
            t,
            || false,
        ))
        .child(
            text(move || {
                back_label
                    .get()
                    .unwrap_or_else(|| labels::text(Label::Back))
            })
            .flex_1()
            .truncate(),
        );
    // Its trigger item is lit while the focus is on it or inside the
    // submenu, and while the submenu is open: `nsh/base` gives the focus
    // and the expanded state to the slot around the item, which the item's
    // own `focus:` and `expanded:` looks do not see.
    let own_open = signal(false);
    let within = signal(false);
    let root = element_ref();
    root.on_focus_in(move || within.set(true));
    root.on_focus_out(move || within.set(false));
    let open = props.open;
    provide_context(SubTriggerLit(computed(move || {
        within.get() || open.get().unwrap_or_else(|| own_open.get())
    })));
    base_submenu()
        .element_ref(&root)
        .open(props.open)
        .disabled(props.disabled)
        .offset(move || t.length(MENU_SUB_OFFSET))
        .trigger(props.trigger.place(|slot| slot))
        .back(back)
        .on_open_change(move |e: &BaseSubmenuEvent| {
            let BaseSubmenuEvent::OpenChange(open, reason) = e;
            if props.open.get_untracked().is_none() {
                own_open.set(*open);
            }
            cx.emit(DropdownMenuSubEvent::OpenChange(*open, *reason));
        })
        .child(props.children)
}

/// What a [`DropdownMenuSub`] gives its trigger item: whether it is lit
/// (the focus on it or in the submenu, or the submenu open).
#[derive(Clone, Copy)]
struct SubTriggerLit(Memo<bool>);

/// The props of [`DropdownMenuSubTrigger`].
#[derive(Debug, Clone, PartialEq, Props)]
pub struct DropdownMenuSubTriggerProps {
    /// An icon before the label.
    pub icon: Prop<Option<IconName>>,
    /// The label starts where an icon's or a check's label would.
    #[prop(default)]
    pub inset: Prop<bool>,
    /// What screen readers say and typeahead matches, when the text is not
    /// enough.
    pub accessibility_label: Prop<Option<SharedString>>,
    /// Tailwind classes of the label.
    pub label_class: Prop<Option<SharedString>>,
    /// The inline style of the label part.
    #[prop(default)]
    pub label_style: Prop<TextPartStyle>,
    /// Tailwind classes of the icon.
    pub icon_class: Prop<Option<SharedString>>,
    /// The inline style of the icon.
    #[prop(default)]
    pub icon_style: Prop<Style>,
    /// Its label.
    pub children: Prop<Children>,
}

/// The item that opens a [`DropdownMenuSub`] (its `trigger`), a chevron at
/// its end; highlighted while its submenu is open.
#[component]
pub fn DropdownMenuSubTrigger(
    props: DropdownMenuSubTriggerProps,
    _cx: Scope<NoEvent>,
) -> impl IntoElement {
    let t = theme();
    let size = menu_size();
    provide_icon_size(move || size.get());
    let side = move || t.length(icon_size_token(size.get()));
    // Lit by its submenu (the slot `nsh/base` focuses is around it).
    let lit = use_context::<SubTriggerLit>().map(|l| l.0);
    let lit = move || lit.is_some_and(|l| l.get());
    let el = base_item()
        .close_on_select(false)
        .accessibility_label(props.accessibility_label);
    let el = row(el, t, size, || false)
        .style_when(lit, move |s| highlight(s, t, || false))
        .child(leading(
            t,
            size,
            props.icon,
            props.inset,
            props.icon_class,
            props.icon_style,
            || false,
            lit,
        ));
    let el = labels_of(el, props.children, props.label_class, props.label_style);
    if is_terminal() {
        return el;
    }
    el.child(
        row_icon_color(icon_view(t, || IconName::ChevronRight, side), t, || false)
            .style_when(lit, move |s| {
                s.text_color(move || t.color(MENU_ITEM_FOCUS_FOREGROUND))
            })
            .ml(auto()),
    )
}
