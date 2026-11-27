//! `Select` in a headless engine: the trigger opens the list and a row
//! chooses, controlled and uncontrolled, what screen readers get, the
//! check on the chosen row, the highlight from the tokens (and a scope's),
//! group headers and disabled items, the popup surface, the trigger's
//! states, pointer and touch sizes (a sheet), and the app's `renderItem`.

mod common;

use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
    time::Duration,
};

use common::{bounds, css, light, size, start};
use nsh_core::{MountItem, NodeId, prelude::*};
use nsh_longbridge::{
    ScopeTokens, SelectEntry, SelectEvent, SelectItem, Size, ThemeAppearance,
    foundation::{field::INPUT_BORDER_FOCUS, menu::MENU_ITEM_FOCUS},
    select,
    theme::semantic::POPOVER,
    theme_scope, tooltip,
};
use nsh_testing::Headless;

fn fruits() -> Vec<SelectItem> {
    vec![
        SelectItem::new("apple", "Apple").group("Fruits"),
        SelectItem::new("banana", "Banana").group("Fruits"),
        SelectItem::new("grapes", "Grapes")
            .group("Fruits")
            .disabled(true),
        SelectItem::new("carrot", "Carrot").group("Vegetables"),
    ]
}

/// A controlled select (on a side of its trigger), writing down what it
/// reports, and an uncontrolled one.
#[component]
fn Fruit() -> impl IntoElement {
    let fruit = signal(None::<SharedString>);
    let log = signal(Vec::<String>::new());
    let note = move |s: String| log.update(|l| l.push(s));
    view()
        .pt(px(40.0))
        .pl(px(40.0))
        .child(
            select()
                .items(fruits())
                .value(move || fruit.get())
                .cover(Some(false))
                .placeholder("Select a fruit")
                .accessibility_label("Fruit")
                .app_class(Some("w-[180px]".into()))
                .on_change(move |value: &Option<SharedString>| {
                    note(format!("change:{}", value.clone().unwrap_or_default()));
                    fruit.set(value.clone());
                })
                .on_open_change(move |e: &SelectEvent| {
                    if let SelectEvent::OpenChange(open, reason) = e {
                        note(format!("open:{open}:{reason:?}"));
                    }
                }),
        )
        .child(
            select()
                .items(fruits())
                .default_value(Some(SharedString::from("carrot")))
                .accessibility_label("Free"),
        )
        .child(view().test_id("away").w(px(200.0)).h(px(40.0)))
        .child(text(move || format!("log:{}", log.get().join(","))))
}

fn open(app: &mut Headless, name: &str) {
    app.press(&format!("role:combobox[{name}]")).unwrap();
    app.run_for(Duration::from_millis(300)).unwrap();
}

#[test]
fn a_row_chooses_and_closes_the_list() {
    let mut app = start::<Fruit>(true);
    assert!(app.shows("Select a fruit"));
    open(&mut app, "Fruit");
    assert!(app.screen().matches("role:option[Banana]").unwrap());
    app.press("role:option[Banana]").unwrap();
    app.wait_for("text:/log:open:true:TriggerPress,change:banana,open:false:Select/")
        .unwrap();
    app.run_for(Duration::from_millis(400)).unwrap();
    assert!(!app.screen().matches("role:option[Banana]").unwrap());
    // The trigger shows the title.
    assert!(app.shows("Banana"));
    assert!(!app.shows("Select a fruit"));
}

#[test]
fn an_uncontrolled_select_keeps_its_own_value() {
    let mut app = start::<Fruit>(true);
    assert!(app.shows("Carrot"));
    open(&mut app, "Free");
    app.press("role:option[Apple]").unwrap();
    app.run_for(Duration::from_millis(400)).unwrap();
    assert!(app.shows("Apple"));
    assert!(!app.shows("Carrot"));
}

#[test]
fn screen_readers_get_the_roles_and_states() {
    let mut app = start::<Fruit>(true);
    let state = app
        .screen()
        .accessibility_state("role:combobox[Fruit]")
        .unwrap();
    assert_eq!(state.expanded, Some(false));
    assert!(state.has_popup);
    open(&mut app, "Free");
    let screen = app.screen();
    assert_eq!(
        screen
            .accessibility_state("role:combobox[Free]")
            .unwrap()
            .expanded,
        Some(true)
    );
    assert!(screen.matches("role:listbox").unwrap());
    assert!(
        screen
            .accessibility_state("role:option[Carrot]")
            .unwrap()
            .selected
    );
    assert!(
        screen
            .accessibility_state("role:option[Grapes]")
            .unwrap()
            .disabled
    );
    assert!(screen.matches("role:header[Vegetables]").unwrap());
}

/// The check's view on the row `name`: the row's last svg.
fn check_opacity(app: &Headless, name: &str) -> f32 {
    let screen = app.screen();
    let nodes = screen.layout_nodes();
    let row = screen.id(&format!("role:option[{name}]")).unwrap();
    // The row's deepest last descendant is the check.
    let mut at = row;
    let mut opacity = 1.0;
    loop {
        let Some(last) = nodes.iter().rfind(|n| n.parent == at) else {
            return opacity;
        };
        if let Some(style) = screen.view_style(&format!("view:{}", last.id)).unwrap() {
            opacity = style.opacity;
        }
        at = last.id;
    }
}

#[test]
fn the_chosen_row_has_the_check_at_its_end() {
    let mut app = start::<Fruit>(true);
    open(&mut app, "Free");
    assert_eq!(check_opacity(&app, "Carrot"), 1.0);
    assert_eq!(check_opacity(&app, "Apple"), 0.0);
}

#[test]
fn the_rows_and_the_trigger_take_their_sizes() {
    let mut app = start::<Fruit>(true);
    // `input-height-md`, and the app's width.
    assert_eq!(size(&app, "role:combobox[Fruit]"), (180.0, 32.0));
    open(&mut app, "Fruit");
    // `menu-item-height-md`.
    assert_eq!(size(&app, "role:option[Apple]").1, 28.0);
    assert_eq!(size(&app, "role:header[Fruits]").1, 28.0);
}

#[test]
fn the_keys_open_it_on_the_first_row_highlighted() {
    let mut app = start::<Fruit>(true);
    app.press_key("Tab").unwrap();
    app.press_key("ArrowDown").unwrap();
    app.run_for(Duration::from_millis(300)).unwrap();
    let style = app
        .screen()
        .view_style("role:option[Apple]")
        .unwrap()
        .unwrap();
    assert_eq!(style.background_color, light(MENU_ITEM_FOCUS));
    let other = app
        .screen()
        .view_style("role:option[Banana]")
        .unwrap()
        .unwrap();
    assert_eq!(other.background_color, None);
    app.press_key("ArrowDown").unwrap();
    app.press_key("Enter").unwrap();
    app.wait_for("text:/change:banana/").unwrap();
}

#[test]
fn a_disabled_row_is_not_chosen() {
    let mut app = start::<Fruit>(true);
    open(&mut app, "Fruit");
    let _ = app.press("role:option[Grapes]");
    app.run_for(Duration::from_millis(100)).unwrap();
    assert!(!app.shows("change:grapes"));
}

#[test]
fn the_open_trigger_keeps_the_focus_edge() {
    let mut app = start::<Fruit>(true);
    let styles = watch_styles(&mut app);
    open(&mut app, "Fruit");
    app.hover("id:away").unwrap();
    app.run_for(Duration::from_millis(300)).unwrap();
    let trigger = app.screen().id("role:combobox[Fruit]").unwrap();
    let style = styles.lock().unwrap().get(&trigger).cloned().unwrap();
    assert_eq!(Some(style.borders.colors[0].0), light(INPUT_BORDER_FOCUS));
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

/// The popup (or sheet) of row `selector`: the nearest ancestor with a
/// fill.
fn popup_of(app: &Headless, selector: &str) -> NodeId {
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

#[test]
fn the_surface_and_its_shadow_reach_the_popup() {
    let mut app = start::<Fruit>(true);
    let styles = watch_styles(&mut app);
    open(&mut app, "Fruit");
    let popup = popup_of(&app, "role:option[Apple]");
    let style = styles.lock().unwrap().get(&popup).cloned().unwrap();
    assert_eq!(Some(style.background_color.0), light(POPOVER));
    let shadows = style
        .effects
        .as_ref()
        .map(|e| e.box_shadows.len())
        .unwrap_or(0);
    assert_eq!(shadows, 2, "tier 3 has two layers: {style:?}");
    // As wide as the trigger, below it, the rows inside `menu-padding`
    // (and the edge).
    let b = bounds(&app, &format!("view:{popup}"));
    let trigger = bounds(&app, "role:combobox[Fruit]");
    assert_eq!(b.size.width, trigger.size.width);
    assert!(b.origin.y > trigger.origin.y + trigger.size.height);
    let row = bounds(&app, "role:option[Apple]");
    assert_eq!((row.origin.x - b.origin.x).0, 5.0);
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
}

/// A select with its value chosen, room above it for the rows before.
#[component]
fn Chosen() -> impl IntoElement {
    view().pt(px(300.0)).pl(px(40.0)).child(
        select()
            .items(fruits())
            .default_value(Some(SharedString::from("carrot")))
            .accessibility_label("Chosen"),
    )
}

#[test]
fn on_a_mac_the_chosen_row_opens_over_the_trigger() {
    let mut app = start::<Chosen>(true);
    open(&mut app, "Chosen");
    let trigger = bounds(&app, "role:combobox[Chosen]");
    let row = bounds(&app, "role:option[Carrot]");
    let middle = |b: nsh_core::geometry::Bounds<Points>| (b.origin.y + b.size.height / 2.0).0;
    assert!(
        (middle(row) - middle(trigger)).abs() <= 1.0,
        "{row:?} {trigger:?}"
    );
}

/// A select in a dark scope that also changes `menu-item-focus`.
#[component]
fn Scoped() -> impl IntoElement {
    view().child(
        theme_scope()
            .appearance(ThemeAppearance::Dark)
            .tokens(ScopeTokens::new().set("menu-item-focus", "#2563eb"))
            .child(
                select()
                    .items(fruits())
                    .cover(Some(false))
                    .accessibility_label("Fruit"),
            ),
    )
}

#[test]
fn a_scope_around_the_select_reaches_its_popup_and_rows() {
    let mut app = start::<Scoped>(true);
    app.press_key("Tab").unwrap();
    app.press_key("ArrowDown").unwrap();
    app.run_for(Duration::from_millis(300)).unwrap();
    let screen = app.screen();
    let row = screen.view_style("role:option[Apple]").unwrap().unwrap();
    assert_eq!(row.background_color, css("#2563eb"));
    let popup = popup_of(&app, "role:option[Apple]");
    let fill = screen
        .view_style(&format!("view:{popup}"))
        .unwrap()
        .unwrap();
    let base = nsh_base::Theme::default().into_tokens();
    let dark = nsh_longbridge::theme::resolve_color_in(&base, &[], true, POPOVER).dark;
    assert_eq!(fill.background_color, Some(nsh_core::Color::from(dark).0));
}

/// A small select open from the start.
#[component]
fn Small() -> impl IntoElement {
    view().child(
        select()
            .size(Size::Xs)
            .items(fruits())
            .default_open(true)
            .accessibility_label("Small"),
    )
}

#[test]
fn touch_screens_get_a_sheet_of_taller_rows() {
    let desktop = start::<Small>(true);
    assert_eq!(size(&desktop, "role:combobox[Small]").1, 24.0);
    assert_eq!(size(&desktop, "role:option[Apple]").1, 20.0);
    let mut touch = start::<Small>(false);
    touch.run_for(Duration::from_millis(600)).unwrap();
    assert_eq!(size(&touch, "role:combobox[Small]").1, 32.0);
    assert_eq!(size(&touch, "role:option[Apple]").1, 44.0);
    let sheet = popup_of(&touch, "role:option[Apple]");
    assert_eq!(size(&touch, &format!("view:{sheet}")).0, 800.0);
}

/// An invalid select, and one with the app's rows.
#[component]
fn Custom() -> impl IntoElement {
    view()
        .child(
            select()
                .items(fruits())
                .invalid(true)
                .accessibility_label("Plan"),
        )
        .child(
            select()
                .items(fruits())
                .default_value(Some(SharedString::from("apple")))
                .cover(Some(false))
                .render_item(|entry: &SelectEntry| text(format!("★ {}", entry.title)))
                .accessibility_label("Starred"),
        )
}

#[test]
fn an_invalid_select_says_so() {
    let app = start::<Custom>(true);
    assert!(
        app.screen()
            .accessibility_state("role:combobox[Plan]")
            .unwrap()
            .invalid
    );
}

#[test]
fn the_apps_rows_keep_the_check() {
    let mut app = start::<Custom>(true);
    open(&mut app, "Starred");
    assert!(app.shows("★ Apple"));
    assert_eq!(check_opacity(&app, "Apple"), 1.0);
    assert_eq!(check_opacity(&app, "Banana"), 0.0);
}

/// TSX apps' declarations name the item types (`nsh/base`'s, declared by
/// this library too) and `Select`'s props.
#[test]
fn tsx_gets_the_item_types() {
    let registry = nsh_longbridge::registry();
    assert_eq!(registry.undocumented(), Vec::<String>::new());
    let ts = nsh_core::element::declarations(&registry, &[]);
    for wanted in [
        "export interface SelectItem {",
        "export interface SelectEntry {",
        "items?: (SelectItem)[];",
        "value?: string | null;",
        "renderItem?: (value: SelectEntry) => JSX.Element;",
    ] {
        assert!(ts.contains(wanted), "{wanted} in:\n{ts}");
    }
}

/// A select with a tooltip, opened from the keyboard.
#[component]
fn Tipped() -> impl IntoElement {
    view().pt(px(100.0)).pl(px(100.0)).child(
        view()
            .self_start()
            .tooltip(tooltip().delay(0.0).child("Hint"))
            .child(
                select()
                    .items(fruits())
                    .cover(Some(false))
                    .accessibility_label("Fruit"),
            ),
    )
}

/// The trigger's tooltip stays hidden while its list is open.
#[test]
fn a_triggers_tooltip_stays_out_of_its_open_list() {
    let mut app = start::<Tipped>(true);
    app.press_key("Tab").unwrap();
    app.run_for(Duration::from_millis(300)).unwrap();
    assert!(app.shows("Hint"), "the focused trigger's hint");
    app.press_key("ArrowDown").unwrap();
    app.run_for(Duration::from_millis(300)).unwrap();
    assert!(app.screen().matches("role:option[Apple]").unwrap());
    assert!(!app.shows("Hint"), "the tooltip shows over the open list");
    app.press_key("Escape").unwrap();
    app.run_for(Duration::from_millis(400)).unwrap();
    assert!(!app.screen().matches("role:option[Apple]").unwrap());
}

/// A long list from a trigger at the bottom of the window.
#[component]
fn Years() -> impl IntoElement {
    let years: Vec<SelectItem> = (0..120)
        .map(|i| {
            let year = (2026 - i).to_string();
            SelectItem::new(year.clone(), year)
        })
        .collect();
    view().pt(px(540.0)).pl(px(40.0)).child(
        select()
            .items(years)
            .cover(Some(false))
            .accessibility_label("Year"),
    )
}

/// The list keeps `popover-margin` (4) from the window's edges, as a
/// popover does.
#[test]
fn the_list_keeps_the_popover_margin_from_the_edges() {
    let mut app = start::<Years>(true);
    open(&mut app, "Year");
    let popup = popup_of(&app, "role:option[2026]");
    let b = bounds(&app, &format!("view:{popup}"));
    assert_eq!(b.origin.y.0, 4.0, "{b:?}");
}

/// A select in a column that stretches its children, one in a centered row
/// beside taller content, and one given a width by its class.
#[component]
fn Layouts() -> impl IntoElement {
    view()
        .w(px(400.0))
        .child(
            select()
                .test_id("in-column")
                .items(fruits())
                .default_value(Some(SharedString::from("apple")))
                .accessibility_label("Column"),
        )
        .child(
            view()
                .test_id("row")
                .flex_row()
                .items_center()
                .child(view().w(px(120.0)).h(px(60.0)))
                .child(
                    select()
                        .test_id("in-row")
                        .items(fruits())
                        .default_value(Some(SharedString::from("apple")))
                        .accessibility_label("Row"),
                ),
        )
        .child(
            select()
                .test_id("classed")
                .items(fruits())
                .app_class(Some("w-[180px]".into()))
                .accessibility_label("Classed"),
        )
}

#[test]
fn a_stretching_column_does_not_widen_it() {
    let app = start::<Layouts>(true);
    let (w, _) = size(&app, "id:in-column");
    assert!(w < 200.0, "as wide as its value, not the column: {w}");
}

#[test]
fn a_row_centers_it_beside_taller_content() {
    let app = start::<Layouts>(true);
    let row = bounds(&app, "id:row");
    let select = bounds(&app, "id:in-row");
    let above = select.origin.y.0 - row.origin.y.0;
    let below = row.size.height.0 - above - select.size.height.0;
    assert_eq!(row.size.height.0, 60.0);
    assert_eq!(above, below, "centered in the row, not at its top");
}

#[test]
fn a_width_class_goes_over_its_own_width() {
    let app = start::<Layouts>(true);
    assert_eq!(size(&app, "id:classed").0, 180.0);
}
