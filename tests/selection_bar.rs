//! `SelectionBar` in a headless engine: shown while items are selected and
//! gone after its exit, the count's words and their announcement, the "×"
//! and Escape in the bar asking to clear, Escape elsewhere left to the app
//! and to an open menu or dialog, the arrows moving between the actions,
//! where it floats (the window, a container), the room it reports, the
//! steps of a narrow bar (icons alone, the count's number alone), touch
//! sizes, a destructive action's colors, a dark bar and a scope's tokens.

mod common;

use std::time::Duration;

use common::{bounds, dark, light, start, start_dark, stroke};
use nsh_core::{Command, NodeId, prelude::*};
use nsh_longbridge::{
    IconName, ScopeTokens, SelectionBarActionVariant, SelectionBarPlacement, Size, ThemeAppearance,
    button,
    components::{
        button::BUTTON_GHOST_FOREGROUND,
        selection_bar::{SELECTION_BAR, SELECTION_BAR_ACTION_DESTRUCTIVE_FOREGROUND},
    },
    dialog, dialog_content, dialog_title, dropdown_menu, dropdown_menu_item, selection_bar,
    selection_bar_action, selection_bar_separator, theme_scope,
};
use nsh_testing::Headless;

/// A list whose selection a button grows, with the bar in the window: a
/// log of what the bar and its actions report.
#[component]
fn Window() -> impl IntoElement {
    let count = signal(0_u32);
    let log = signal(Vec::<String>::new());
    let push = move |s: String| log.update(|l| l.push(s));
    view()
        .flex_col()
        .items_start()
        .gap(px(8.0))
        .p(px(20.0))
        .child(
            button()
                .on_press(move || count.update(|c| *c += 1))
                .child("Select one more"),
        )
        .child(
            selection_bar()
                .count(move || count.get())
                .on_clear(move || {
                    push("clear".into());
                    count.set(0);
                })
                .on_inset_change(move |v: &f32| push(format!("inset:{v}")))
                .child(
                    selection_bar_action()
                        .icon(IconName::Archive)
                        .label("Archive")
                        .on_press(move || push("archive".into())),
                )
                .child(
                    selection_bar_action()
                        .icon(IconName::Tag)
                        .label("Tags")
                        .on_press(move || push("tags".into())),
                )
                .child(selection_bar_separator())
                .child(
                    selection_bar_action()
                        .icon(IconName::Trash2)
                        .icon_only(true)
                        .variant(SelectionBarActionVariant::Destructive)
                        .label("Delete")
                        .on_press(move || push("delete".into())),
                )
                .child(
                    dropdown_menu()
                        .trigger(
                            selection_bar_action()
                                .icon(IconName::Ellipsis)
                                .icon_only(true)
                                .label("More"),
                        )
                        .child(dropdown_menu_item().child("Duplicate")),
                ),
        )
        .child(text(move || {
            format!("count:{} log:{}", count.get(), log.get().join(","))
        }))
}

fn select(app: &mut Headless, n: u32) {
    for _ in 0..n {
        app.press("role:button[Select one more]").unwrap();
    }
    app.run_for(Duration::from_millis(400)).unwrap();
}

fn log(app: &Headless) -> String {
    app.screen()
        .texts()
        .into_iter()
        .find(|t| t.starts_with("count:"))
        .unwrap()
        .to_string()
}

/// The nearest ancestor of `from` with the bar's fill, `fill`.
fn bar_of(app: &Headless, from: &str, fill_color: Option<u32>) -> NodeId {
    let nodes = app.screen().layout_nodes();
    let mut at = app.screen().id(from).unwrap();
    loop {
        let n = nodes
            .iter()
            .find(|n| n.id == at)
            .unwrap_or_else(|| panic!("no bar around {from}"));
        let style = app
            .screen()
            .view_style(&format!("view:{}", n.parent))
            .unwrap();
        if style.is_some_and(|s| s.background_color == fill_color) {
            return n.parent;
        }
        at = n.parent;
    }
}

#[test]
fn it_shows_while_items_are_selected_and_goes_after_its_exit() {
    let mut app = start::<Window>(true);
    assert!(!app.screen().matches("role:toolbar").unwrap());
    select(&mut app, 1);
    assert!(app.screen().matches("role:toolbar[1 selected]").unwrap());
    assert!(app.shows("1 selected"));
    select(&mut app, 2);
    assert!(app.shows("3 selected"));
    assert!(app.screen().matches("role:toolbar[3 selected]").unwrap());
    app.press("role:button[Clear selection]").unwrap();
    app.run_for(Duration::from_millis(50)).unwrap();
    assert!(log(&app).starts_with("count:0 log:"), "{}", log(&app));
    assert!(log(&app).contains("clear"));
    // On its way out it still says the last count, never "0 selected".
    assert!(!app.shows("0 selected"));
    app.run_for(Duration::from_millis(500)).unwrap();
    assert!(!app.screen().matches("role:toolbar").unwrap());
}

#[test]
fn the_count_is_announced_as_it_changes() {
    let mut app = start::<Window>(true);
    app.clear_commands();
    select(&mut app, 1);
    select(&mut app, 1);
    let said: Vec<String> = app
        .screen()
        .commands()
        .iter()
        .filter_map(|(_, c)| match c {
            Command::Announce(m) => Some(m.clone()),
            _ => None,
        })
        .collect();
    assert_eq!(said, ["1 selected", "2 selected"]);
}

#[test]
fn the_arrows_move_between_the_actions_without_pressing() {
    let mut app = start::<Window>(true);
    select(&mut app, 2);
    // Tab: from the button to the bar's first item, the "×".
    app.press_key("Tab").unwrap();
    app.press_key("Tab").unwrap();
    app.press_key("ArrowRight").unwrap();
    app.run_for(Duration::from_millis(50)).unwrap();
    assert!(!log(&app).contains("archive"), "an arrow presses nothing");
    app.press_key("Space").unwrap();
    app.run_for(Duration::from_millis(50)).unwrap();
    assert!(log(&app).contains("archive"), "{}", log(&app));
}

#[test]
fn escape_in_the_bar_asks_to_clear() {
    let mut app = start::<Window>(true);
    select(&mut app, 2);
    app.press_key("Tab").unwrap();
    app.press_key("Tab").unwrap();
    // The first Escape hides the focused "×"'s tooltip, the next, once
    // the tooltip is gone (a hint on its way out still takes Escape), is
    // the bar's.
    app.press_key("Escape").unwrap();
    app.run_for(Duration::from_millis(300)).unwrap();
    assert!(log(&app).starts_with("count:2 "), "{}", log(&app));
    app.press_key("Escape").unwrap();
    app.run_for(Duration::from_millis(50)).unwrap();
    assert!(
        log(&app).starts_with("count:0 "),
        "from the bar: {}",
        log(&app)
    );
}

#[test]
fn escape_outside_the_bar_is_the_apps() {
    let mut app = start::<Window>(true);
    // The focus stays on the button that selects.
    app.press_key("Tab").unwrap();
    app.press_key("Space").unwrap();
    app.run_for(Duration::from_millis(400)).unwrap();
    assert!(log(&app).starts_with("count:1 "), "{}", log(&app));
    app.press_key("Escape").unwrap();
    app.run_for(Duration::from_millis(50)).unwrap();
    assert!(
        log(&app).starts_with("count:1 "),
        "from the list: {}",
        log(&app)
    );
    // Into the bar and out again.
    app.press_key("Tab").unwrap();
    app.run_for(Duration::from_millis(50)).unwrap();
    app.press_key("Shift+Tab").unwrap();
    app.run_for(Duration::from_millis(300)).unwrap();
    // Twice: a tooltip left over would take the first.
    for _ in 0..2 {
        app.press_key("Escape").unwrap();
        app.run_for(Duration::from_millis(300)).unwrap();
    }
    assert!(
        log(&app).starts_with("count:1 "),
        "after leaving the bar: {}",
        log(&app)
    );
    assert!(!log(&app).contains("clear"));
}

/// A bar with a dialog beside it.
#[component]
fn Guards() -> impl IntoElement {
    let count = signal(3_u32);
    let open = signal(false);
    view()
        .flex_col()
        .items_start()
        .gap(px(8.0))
        .p(px(20.0))
        .child(
            dialog()
                .open(move || Some(open.get()))
                .on_open_change(move |e: &nsh_longbridge::DialogEvent| {
                    if let nsh_longbridge::DialogEvent::OpenChange(o, _) = e {
                        open.set(*o);
                    }
                })
                .trigger(button().child("Open dialog"))
                .child(dialog_content().child(dialog_title().child("A dialog"))),
        )
        .child(
            selection_bar()
                .count(move || count.get())
                .on_clear(move || count.set(0))
                .child(selection_bar_action().label("Archive")),
        )
        .child(text(move || {
            format!("count:{} dialog:{}", count.get(), open.get())
        }))
}

#[test]
fn escape_is_an_open_dialogs_first() {
    let mut app = start::<Guards>(true);
    app.run_for(Duration::from_millis(400)).unwrap();
    app.press("role:button[Open dialog]").unwrap();
    app.run_for(Duration::from_millis(400)).unwrap();
    assert!(app.shows("count:3 dialog:true"));
    app.press_key("Escape").unwrap();
    app.run_for(Duration::from_millis(400)).unwrap();
    let shown: Vec<String> = app
        .screen()
        .texts()
        .iter()
        .filter(|t| t.starts_with("count:"))
        .map(|t| t.to_string())
        .collect();
    assert!(
        app.shows("count:3 dialog:false"),
        "Escape closes the dialog only: {shown:?}"
    );
}

#[test]
fn escape_in_an_open_menu_of_the_bar_closes_the_menu_only() {
    let mut app = start::<Window>(true);
    select(&mut app, 2);
    app.press("role:button[More]").unwrap();
    app.run_for(Duration::from_millis(400)).unwrap();
    assert!(app.shows("Duplicate"));
    app.press_key("Escape").unwrap();
    app.run_for(Duration::from_millis(400)).unwrap();
    assert!(!app.shows("Duplicate"), "Escape closes the menu");
    assert!(log(&app).starts_with("count:2 "), "{}", log(&app));
    assert!(!log(&app).contains("clear"));
}

#[test]
fn it_floats_at_the_bottom_centre_of_the_window_and_reports_its_room() {
    let mut app = start::<Window>(true);
    select(&mut app, 1);
    let bar = bar_of(&app, "role:toolbar", light(SELECTION_BAR));
    let b = bounds(&app, &format!("view:{bar}"));
    // 24 above the window's bottom edge, centered.
    assert!(
        (b.origin.y.0 + b.size.height.0 - 576.0).abs() < 0.5,
        "{b:?}"
    );
    let centre = b.origin.x.0 + b.size.width.0 / 2.0;
    assert!((centre - 400.0).abs() < 0.5, "{b:?}");
    // md actions: 32 high, padding 6 and an edge of 1 around them.
    assert!((b.size.height.0 - 46.0).abs() < 0.5, "{b:?}");
    let expected = format!("inset:{}", b.size.height.0 + 24.0);
    assert!(log(&app).contains(&expected), "{} vs {expected}", log(&app));
    app.press("role:button[Clear selection]").unwrap();
    app.run_for(Duration::from_millis(500)).unwrap();
    assert!(log(&app).ends_with("inset:0"), "{}", log(&app));
}

#[test]
fn an_icon_only_action_is_named_and_square() {
    let mut app = start::<Window>(true);
    select(&mut app, 1);
    let screen = app.screen();
    assert!(screen.matches("role:button[Delete]").unwrap());
    assert!(screen.matches("role:button[More]").unwrap());
    let (w, h) = common::size(&app, "role:button[Delete]");
    assert_eq!((w, h), (32.0, 32.0));
    let (w, _) = common::size(&app, "role:button[Archive]");
    assert!(w > 40.0, "the label shows beside the icon: {w}");
}

/// Bars in containers: a wide one and a narrow one.
#[component]
fn Containers() -> impl IntoElement {
    let bar = |placement_width: f32, id: &'static str| {
        view()
            .w(px(placement_width))
            .h(px(200.0))
            .test_id(id)
            .child(
                selection_bar()
                    .placement(SelectionBarPlacement::Container)
                    .count(4)
                    .label(SharedString::from(format!("{id} picked")))
                    .child(
                        selection_bar_action()
                            .icon(IconName::Archive)
                            .label(SharedString::from(format!("{id} archive"))),
                    )
                    .child(
                        selection_bar_action()
                            .icon(IconName::Copy)
                            .label(SharedString::from(format!("{id} copy"))),
                    )
                    .child(
                        selection_bar_action()
                            .icon(IconName::Download)
                            .label(SharedString::from(format!("{id} download"))),
                    )
                    .child(selection_bar_action().label(SharedString::from(format!("{id} words")))),
            )
    };
    view()
        .flex_col()
        .gap(px(10.0))
        .p(px(10.0))
        .child(bar(780.0, "wide"))
        .child(bar(260.0, "narrow"))
}

#[test]
fn in_a_container_it_floats_at_the_containers_bottom_centre() {
    let mut app = start::<Containers>(true);
    app.run_for(Duration::from_millis(400)).unwrap();
    let container = bounds(&app, "id:wide");
    let bar = bar_of(&app, "role:toolbar[wide picked]", light(SELECTION_BAR));
    let b = bounds(&app, &format!("view:{bar}"));
    let bottom = container.origin.y.0 + container.size.height.0;
    assert!(
        (b.origin.y.0 + b.size.height.0 - (bottom - 24.0)).abs() < 0.5,
        "{b:?}"
    );
    let centre = container.origin.x.0 + container.size.width.0 / 2.0;
    assert!(
        (b.origin.x.0 + b.size.width.0 / 2.0 - centre).abs() <= 1.0,
        "{b:?}"
    );
}

#[test]
fn short_of_room_actions_with_an_icon_show_it_alone() {
    let mut app = start::<Containers>(true);
    app.run_for(Duration::from_millis(400)).unwrap();
    let (w, h) = common::size(&app, "role:button[wide archive]");
    assert!(w > h, "wide: the label shows ({w} × {h})");
    let (w, h) = common::size(&app, "role:button[narrow archive]");
    assert_eq!(w, h, "narrow: the icon alone");
    let (w, h) = common::size(&app, "role:button[narrow words]");
    assert!(w > h, "an action without an icon keeps its words");
    // The bar stays inside its container, less the offset on each side.
    let container = bounds(&app, "id:narrow");
    let bar = bar_of(&app, "role:toolbar[narrow picked]", light(SELECTION_BAR));
    let b = bounds(&app, &format!("view:{bar}"));
    assert!(
        b.size.width.0 <= container.size.width.0 - 48.0 + 0.5,
        "{b:?}"
    );
}

/// A bar of `sm` actions on a phone.
#[component]
fn Phone() -> impl IntoElement {
    view().child(
        selection_bar().count(2).size(Size::Sm).child(
            selection_bar_action()
                .icon(IconName::Archive)
                .label("Archive"),
        ),
    )
}

#[test]
fn touch_screens_get_taller_actions_and_a_smaller_offset() {
    let mut app = start::<Phone>(false);
    app.run_for(Duration::from_millis(400)).unwrap();
    let (_, h) = common::size(&app, "role:button[Archive]");
    assert_eq!(h, 36.0, "control-height-touch-sm");
    let bar = bar_of(&app, "role:toolbar", light(SELECTION_BAR));
    let b = bounds(&app, &format!("view:{bar}"));
    let screen_h = 600.0;
    let bottom = b.origin.y.0 + b.size.height.0;
    assert!(bottom <= screen_h - 16.0 + 0.5, "{b:?}");
}

/// A destructive and a plain action, in the dark appearance.
#[component]
fn Colors() -> impl IntoElement {
    view().child(
        selection_bar()
            .count(1)
            .child(
                selection_bar_action()
                    .test_id("plain")
                    .icon(IconName::Archive)
                    .label("Archive"),
            )
            .child(
                selection_bar_action()
                    .test_id("danger")
                    .icon(IconName::Trash2)
                    .variant(SelectionBarActionVariant::Destructive)
                    .label("Delete"),
            ),
    )
}

#[test]
fn a_destructive_action_takes_the_destructive_color() {
    let (mut app, strokes) = start_dark::<Colors>();
    app.run_for(Duration::from_millis(400)).unwrap();
    let icon = |app: &Headless, name: &str| {
        let nodes = app.screen().layout_nodes();
        let button = app.screen().id(&format!("role:button[{name}]")).unwrap();
        // The button's first drawn child: its icon.
        let svg = nodes
            .iter()
            .find(|n| n.parent == button && strokes.lock().unwrap().contains_key(&n.id))
            .unwrap()
            .id;
        stroke(app, &strokes, &format!("view:{svg}"))
    };
    assert_eq!(
        icon(&app, "Delete"),
        dark(SELECTION_BAR_ACTION_DESTRUCTIVE_FOREGROUND)
    );
    assert_eq!(icon(&app, "Archive"), dark(BUTTON_GHOST_FOREGROUND));
}

/// A dark bar on a light page, and one in a scope with its own fill.
#[component]
fn Looks() -> impl IntoElement {
    view()
        .child(
            selection_bar()
                .count(1)
                .label("dark bar")
                .appearance(ThemeAppearance::Dark)
                .child(selection_bar_action().label("A")),
        )
        .child(
            theme_scope()
                .tokens(ScopeTokens::new().set("selection-bar", "#2563eb"))
                .child(
                    selection_bar()
                        .count(1)
                        .label("scoped bar")
                        .child(selection_bar_action().label("B")),
                ),
        )
}

#[test]
fn appearance_and_a_scope_reach_the_bar() {
    let mut app = start::<Looks>(true);
    app.run_for(Duration::from_millis(400)).unwrap();
    let dark_fill = Some(dark(SELECTION_BAR).0);
    let _ = bar_of(&app, "role:toolbar[dark bar]", dark_fill);
    let _ = bar_of(&app, "role:toolbar[scoped bar]", common::css("#2563eb"));
}

/// A bar of `sm` actions, one of them a menu's trigger.
#[component]
fn Small() -> impl IntoElement {
    view().child(
        selection_bar()
            .count(2)
            .size(Size::Sm)
            .child(selection_bar_action().label("Plain"))
            .child(
                dropdown_menu()
                    .trigger(selection_bar_action().size(Size::Sm).label("Menu"))
                    .child(dropdown_menu_item().child("Duplicate")),
            ),
    )
}

#[test]
fn actions_take_the_bars_size_and_a_trigger_is_given_it() {
    let mut app = start::<Small>(true);
    app.run_for(Duration::from_millis(400)).unwrap();
    assert_eq!(
        common::size(&app, "role:button[Plain]").1,
        28.0,
        "button-height-sm"
    );
    assert_eq!(common::size(&app, "role:button[Menu]").1, 28.0);
}

/// Bars of four icon actions in panels 300 and 420 wide.
#[component]
fn Widths() -> impl IntoElement {
    let bar = |w: f32, id: &'static str| {
        let action = move |icon: IconName, n: &str| {
            selection_bar_action()
                .icon(icon)
                .label(SharedString::from(format!("{id} {n}")))
        };
        view().w(px(w)).h(px(120.0)).test_id(id).child(
            selection_bar()
                .placement(SelectionBarPlacement::Container)
                .count(12)
                .label(SharedString::from(format!("{id} picked")))
                .child(action(IconName::Archive, "archive"))
                .child(action(IconName::Copy, "copy"))
                .child(action(IconName::Download, "download"))
                .child(action(IconName::Tag, "tags")),
        )
    };
    view()
        .flex_col()
        .gap(px(4.0))
        .child(bar(300.0, "w300"))
        .child(bar(420.0, "w420"))
}

#[test]
fn shorter_still_the_count_shows_its_number_before_the_actions_scroll() {
    let mut app = start::<Widths>(true);
    app.run_for(Duration::from_millis(400)).unwrap();
    // 420: the icons alone are enough; the count keeps its words.
    assert!(app.shows("w420 picked"));
    assert_eq!(
        common::size(&app, "role:button[w420 archive]"),
        (32.0, 32.0)
    );
    // 300: the number alone, named by the words, and every action in view.
    assert!(!app.shows("w300 picked"));
    assert!(app.screen().matches("text:12").unwrap());
    assert!(app.screen().matches("label:w300 picked").unwrap());
    assert!(app.screen().matches("role:toolbar[w300 picked]").unwrap());
    let panel = bounds(&app, "id:w300");
    let last = bounds(&app, "role:button[w300 tags]");
    assert!(
        last.origin.x.0 + last.size.width.0 <= panel.origin.x.0 + panel.size.width.0 - 24.0,
        "{last:?} in {panel:?}"
    );
}
