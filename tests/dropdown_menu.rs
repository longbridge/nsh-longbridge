//! `DropdownMenu` in a headless engine: its trigger opens it, items report
//! and close it, checkbox and radio items report the values they ask for,
//! disabled items and labels are passed over, the popup surface and its
//! shadow reach the popup, a scope around the menu reaches it, row sizes
//! with a pointer and on touch screens (a sheet), the highlight, and
//! submenus.

mod common;

use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
    time::Duration,
};

use common::{light, start};
use nsh_core::{Checked, MountItem, NodeId, prelude::*};
use nsh_longbridge::{
    ButtonVariant, DropdownMenuEvent, IconName, MenuItemVariant, ScopeTokens, Size,
    ThemeAppearance, button, dropdown_menu, dropdown_menu_checkbox_item, dropdown_menu_item,
    dropdown_menu_label, dropdown_menu_radio_group, dropdown_menu_radio_item,
    dropdown_menu_separator, dropdown_menu_shortcut, dropdown_menu_sub, dropdown_menu_sub_trigger,
    foundation::menu::MENU_ITEM_FOCUS, theme::semantic::POPOVER, theme_scope, tooltip,
};
use nsh_testing::Headless;

/// A menu with every kind of part, writing down what it reports.
#[component]
fn Account() -> impl IntoElement {
    let log = signal(Vec::<String>::new());
    let note = move |s: String| log.update(|l| l.push(s));
    let status_bar = signal(true);
    let position = signal(Some(SharedString::from("bottom")));
    view()
        .pt(px(40.0))
        .pl(px(40.0))
        .items_start()
        .child(
            dropdown_menu()
                .trigger(button().variant(ButtonVariant::Outline).child("Open"))
                .on_open_change(move |e: &DropdownMenuEvent| {
                    if let DropdownMenuEvent::OpenChange(open, reason) = e {
                        note(format!("open:{open}:{reason:?}"));
                    }
                })
                .child(dropdown_menu_label().child("My Account"))
                .child(
                    dropdown_menu_item()
                        .on_select(move || note("profile".into()))
                        .child("Profile")
                        .child(dropdown_menu_shortcut().child("⇧⌘P")),
                )
                .child(
                    dropdown_menu_item()
                        .icon(IconName::Settings)
                        .on_select(move || note("settings".into()))
                        .child("Settings"),
                )
                .child(
                    dropdown_menu_item()
                        .disabled(true)
                        .on_select(move || note("billing".into()))
                        .child("Billing"),
                )
                .child(dropdown_menu_separator())
                .child(
                    dropdown_menu_sub()
                        .trigger(dropdown_menu_sub_trigger().child("Invite users"))
                        .child(
                            dropdown_menu_item()
                                .on_select(move || note("email".into()))
                                .child("Email"),
                        ),
                )
                .child(
                    dropdown_menu_checkbox_item()
                        .checked(move || status_bar.get())
                        .on_change(move |checked: &bool| {
                            note(format!("status:{checked}"));
                            status_bar.set(*checked);
                        })
                        .child("Status Bar"),
                )
                .child(
                    dropdown_menu_radio_group()
                        .value(move || position.get())
                        .on_change(move |value: &SharedString| {
                            note(format!("position:{value}"));
                            position.set(Some(value.clone()));
                        })
                        .child(dropdown_menu_radio_item().value("top").child("Top"))
                        .child(dropdown_menu_radio_item().value("bottom").child("Bottom")),
                )
                .child(
                    dropdown_menu_item()
                        .variant(MenuItemVariant::Destructive)
                        .on_select(move || note("logout".into()))
                        .child("Log out"),
                ),
        )
        .child(view().test_id("away").w(px(200.0)).h(px(40.0)))
        .child(text(move || format!("log:{}", log.get().join(","))))
}

fn open(app: &mut Headless) {
    app.press("role:button[Open]").unwrap();
    app.run_for(Duration::from_millis(300)).unwrap();
}

#[test]
fn the_trigger_opens_it_and_an_item_closes_it() {
    let mut app = start::<Account>(true);
    assert!(!app.shows("Profile"));
    open(&mut app);
    assert!(app.screen().matches("role:menuitem[Profile ⇧⌘P]").unwrap());
    assert_eq!(
        app.screen()
            .accessibility_state("role:button[Open]")
            .unwrap()
            .expanded,
        Some(true)
    );
    app.press("role:menuitem[Settings]").unwrap();
    app.wait_for("text:/log:open:true:TriggerPress,settings,open:false:Select/")
        .unwrap();
    app.run_for(Duration::from_millis(400)).unwrap();
    assert!(!app.shows("Profile"));
}

#[test]
fn a_disabled_item_and_a_label_are_not_chosen() {
    let mut app = start::<Account>(true);
    open(&mut app);
    let screen = app.screen();
    assert!(
        screen
            .accessibility_state("role:menuitem[Billing]")
            .unwrap()
            .disabled
    );
    // A label is text, not an item.
    assert!(screen.shows("My Account"));
    assert!(!screen.matches("role:menuitem[My Account]").unwrap());
    let _ = app.press("role:menuitem[Billing]");
    assert!(app.shows("Profile"));
    assert!(!app.shows("billing"));
}

#[test]
fn a_checkbox_item_asks_for_the_other_value() {
    let mut app = start::<Account>(true);
    open(&mut app);
    let state = app
        .screen()
        .accessibility_state("role:menuitemcheckbox[Status Bar]")
        .unwrap();
    assert_eq!(state.checked, Some(Checked::True));
    app.press("role:menuitemcheckbox[Status Bar]").unwrap();
    app.wait_for("text:/status:false/").unwrap();
    open(&mut app);
    let state = app
        .screen()
        .accessibility_state("role:menuitemcheckbox[Status Bar]")
        .unwrap();
    assert_eq!(state.checked, Some(Checked::False));
}

#[test]
fn a_radio_item_reports_its_value_once() {
    let mut app = start::<Account>(true);
    open(&mut app);
    let screen = app.screen();
    assert_eq!(
        screen
            .accessibility_state("role:menuitemradio[Bottom]")
            .unwrap()
            .checked,
        Some(Checked::True)
    );
    assert_eq!(
        screen
            .accessibility_state("role:menuitemradio[Top]")
            .unwrap()
            .checked,
        Some(Checked::False)
    );
    // The checked item reports nothing.
    app.press("role:menuitemradio[Bottom]").unwrap();
    open(&mut app);
    app.press("role:menuitemradio[Top]").unwrap();
    app.wait_for("text:/position:top/").unwrap();
    assert!(!app.shows("position:bottom"));
}

#[test]
fn a_submenu_opens_beside_its_trigger() {
    let mut app = start::<Account>(true);
    open(&mut app);
    assert!(!app.shows("Email"));
    app.press("role:menuitem[Invite users]").unwrap();
    app.run_for(Duration::from_millis(300)).unwrap();
    // Its first item is level with its trigger (`menu-sub-offset`), beside
    // the menu.
    let trigger = common::bounds(&app, "role:menuitem[Invite users]");
    let email = common::bounds(&app, "role:menuitem[Email]");
    assert_eq!(email.origin.y, trigger.origin.y);
    assert!(email.origin.x > trigger.origin.x + trigger.size.width);
    app.press("role:menuitem[Email]").unwrap();
    app.wait_for("text:/email/").unwrap();
}

#[test]
fn the_keys_open_it_on_its_first_item_highlighted() {
    let mut app = start::<Account>(true);
    app.press_key("Tab").unwrap();
    app.press_key("ArrowDown").unwrap();
    app.run_for(Duration::from_millis(300)).unwrap();
    let style = app
        .screen()
        .view_style("role:menuitem[Profile ⇧⌘P]")
        .unwrap()
        .unwrap();
    assert_eq!(style.background_color, light(MENU_ITEM_FOCUS));
    app.press_key("Enter").unwrap();
    app.wait_for("text:/profile/").unwrap();
}

#[test]
fn rows_take_the_menus_size() {
    let mut app = start::<Account>(true);
    open(&mut app);
    assert_eq!(common::size(&app, "role:menuitem[Profile ⇧⌘P]").1, 28.0);
    // The shortcut sits at the row's end.
    let row = common::bounds(&app, "role:menuitem[Profile ⇧⌘P]");
    let shortcut = common::bounds(&app, "text:⇧⌘P");
    let end = (row.origin.x + row.size.width).0;
    assert!((shortcut.origin.x + shortcut.size.width).0 > end - 12.0);
}

/// Watches the views' styles: the latest of each.
fn watch_styles(app: &mut Headless) -> Arc<Mutex<HashMap<NodeId, nsh_core::ViewStyle>>> {
    let styles = Arc::new(Mutex::new(HashMap::new()));
    let seen = styles.clone();
    app.observe(move |_, batch| {
        for item in &batch.items {
            if let MountItem::ViewStyle { id, style } = item {
                seen.lock().unwrap().insert(*id, (**style).clone());
            }
        }
    });
    styles
}

/// The popup: the parent of the rows' column.
fn popup(app: &Headless) -> NodeId {
    let nodes = app.screen().layout_nodes();
    let row = app.screen().id("role:menuitem[Profile ⇧⌘P]").unwrap();
    // The nearest ancestor with the popover fill.
    let mut at = row;
    loop {
        let n = nodes.iter().find(|n| n.id == at).unwrap();
        let style = app
            .screen()
            .view_style(&format!("view:{}", n.parent))
            .unwrap();
        if style.is_some_and(|s| s.background_color == light(POPOVER)) {
            return n.parent;
        }
        at = n.parent;
    }
}

#[test]
fn the_surface_and_its_shadow_reach_the_popup() {
    let mut app = start::<Account>(true);
    let styles = watch_styles(&mut app);
    open(&mut app);
    let popup = popup(&app);
    let style = styles.lock().unwrap().get(&popup).cloned().unwrap();
    let shadows = style
        .effects
        .as_ref()
        .map(|e| e.box_shadows.len())
        .unwrap_or(0);
    assert_eq!(shadows, 2, "tier 3 has two layers: {style:?}");
    assert_eq!(style.border_radius.0, 8.0);
    // Nothing around it clips it.
    let nodes = app.screen().layout_nodes();
    let mut at = Some(popup);
    while let Some(id) = at {
        let Some(n) = nodes.iter().find(|n| n.id == id) else {
            break;
        };
        assert!(!n.clips || id == popup, "{id:?} clips the popup");
        at = (n.parent != nsh_core::ROOT_ID).then_some(n.parent);
    }
    // At least `menu-min-width` wide, its rows inside `menu-padding`.
    let b = common::bounds(&app, &format!("view:{popup}"));
    assert!(b.size.width.0 >= 128.0);
    let row = common::bounds(&app, "role:menuitem[Profile ⇧⌘P]");
    assert_eq!((row.origin.x - b.origin.x).0, 5.0);
}

/// A menu in a dark scope that also changes `popover`.
#[component]
fn Scoped() -> impl IntoElement {
    view().child(
        theme_scope()
            .appearance(ThemeAppearance::Dark)
            .tokens(ScopeTokens::new().set("menu-item-focus", "#2563eb"))
            .child(
                dropdown_menu()
                    .trigger(button().child("Open"))
                    .child(dropdown_menu_item().child("Profile")),
            ),
    )
}

#[test]
fn a_scope_around_the_menu_reaches_its_popup_and_rows() {
    let mut app = start::<Scoped>(true);
    app.press_key("Tab").unwrap();
    app.press_key("ArrowDown").unwrap();
    app.run_for(Duration::from_millis(300)).unwrap();
    let screen = app.screen();
    let popup = popup_of(&app);
    let fill = screen
        .view_style(&format!("view:{popup}"))
        .unwrap()
        .unwrap();
    let base = nsh_base::Theme::default().into_tokens();
    let dark = nsh_longbridge::theme::resolve_color_in(&base, &[], true, POPOVER).dark;
    assert_eq!(fill.background_color, Some(nsh_core::Color::from(dark).0));
    let row = screen
        .view_style("role:menuitem[Profile]")
        .unwrap()
        .unwrap();
    assert_eq!(row.background_color, common::css("#2563eb"));
}

/// The popup of the only row: its row's parent chain's first fill.
fn popup_of(app: &Headless) -> NodeId {
    let nodes = app.screen().layout_nodes();
    let mut at = app.screen().id("role:menuitem[Profile]").unwrap();
    loop {
        let parent = nodes.iter().find(|n| n.id == at).unwrap().parent;
        let style = app.screen().view_style(&format!("view:{parent}")).unwrap();
        if style.is_some_and(|s| s.background_color.is_some()) {
            return parent;
        }
        at = parent;
    }
}

/// A small menu and a large one.
#[component]
fn Sizes() -> impl IntoElement {
    view().child(
        dropdown_menu()
            .size(Size::Xs)
            .default_open(true)
            .trigger(button().child("Small"))
            .child(dropdown_menu_item().child("Tiny")),
    )
}

#[test]
fn touch_screens_get_a_sheet_of_taller_rows() {
    let desktop = start::<Sizes>(true);
    assert_eq!(common::size(&desktop, "role:menuitem[Tiny]").1, 20.0);
    let mut touch = start::<Sizes>(false);
    touch.run_for(Duration::from_millis(600)).unwrap();
    assert_eq!(common::size(&touch, "role:menuitem[Tiny]").1, 44.0);
    // A sheet: as wide as the window.
    let popup = popup_of_item(&touch, "role:menuitem[Tiny]");
    assert_eq!(common::size(&touch, &format!("view:{popup}")).0, 800.0);
}

fn popup_of_item(app: &Headless, selector: &str) -> NodeId {
    let nodes = app.screen().layout_nodes();
    let mut at = app.screen().id(selector).unwrap();
    loop {
        let parent = nodes.iter().find(|n| n.id == at).unwrap().parent;
        let style = app.screen().view_style(&format!("view:{parent}")).unwrap();
        if style.is_some_and(|s| s.background_color.is_some()) {
            return parent;
        }
        at = parent;
    }
}

/// A trigger with a tooltip, opened from the keyboard.
#[component]
fn TippedTrigger() -> impl IntoElement {
    view().pt(px(100.0)).pl(px(100.0)).child(
        view()
            .self_start()
            .tooltip(tooltip().delay(0.0).child("Hint"))
            .child(
                dropdown_menu()
                    .trigger(button().child("Open"))
                    .child(dropdown_menu_item().child("Profile")),
            ),
    )
}

/// The trigger's tooltip stays hidden while its menu is open (the menu
/// takes the focus, and its content counts among the open popups).
#[test]
fn a_triggers_tooltip_stays_out_of_its_open_menu() {
    let mut app = start::<TippedTrigger>(true);
    app.press_key("Tab").unwrap();
    app.run_for(Duration::from_millis(300)).unwrap();
    assert!(app.shows("Hint"), "the focused trigger's hint");
    app.press_key("Enter").unwrap();
    app.run_for(Duration::from_millis(300)).unwrap();
    assert!(app.shows("Profile"), "opened from the keyboard");
    assert!(!app.shows("Hint"), "the tooltip shows over the open menu");
    app.press_key("Escape").unwrap();
    app.run_for(Duration::from_millis(400)).unwrap();
    assert!(!app.shows("Profile"), "the first Escape closes the menu");
}

/// From TSX, a library `Button` as the trigger (it takes the press before
/// the trigger slot's view) opens and closes it; an item closes it.
#[test]
fn from_tsx_a_button_trigger_opens_and_closes_it() {
    let mut app = common::start_tsx("tests/tsx/dropdown_menu.tsx", true);
    let open = |app: &mut Headless| {
        app.press("role:button[Account]").unwrap();
        app.run_for(Duration::from_millis(400)).unwrap();
    };
    open(&mut app);
    assert!(app.shows("Settings"));
    open(&mut app);
    assert!(!app.shows("Settings"), "a second press closes it");
    open(&mut app);
    app.press("role:menuitem[Settings]").unwrap();
    app.run_for(Duration::from_millis(400)).unwrap();
    assert!(!app.shows("Settings"), "an item closes it");
}
