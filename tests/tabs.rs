//! `Tabs` in a headless engine: presses and the arrow keys selecting tabs
//! and reporting values, controlled and uncontrolled, the selected panel
//! alone made, disabled tabs, what screen readers get, pointer and touch
//! sizes, the variants' looks, and a scope's tokens reaching them.

mod common;

use common::{bounds, css, fill, light, size, start};
use nsh_core::{geometry::Axis, prelude::*};
use nsh_longbridge::{
    ScopeTokens, Size, TabsVariant,
    components::tabs::{TABS_LIST, TABS_TRIGGER_SELECTED},
    tabs, tabs_content, tabs_list, tabs_trigger, theme_scope,
};

/// Controlled tabs (the last one disabled) with a change counter, and
/// uncontrolled ones.
#[component]
fn Settings() -> impl IntoElement {
    let tab = signal(SharedString::from("account"));
    let changes = signal(0u32);
    view()
        .items_start()
        .child(
            tabs()
                .value(move || Some(tab.get()))
                .on_change(move |value: &SharedString| {
                    changes.update(|n| *n += 1);
                    tab.set(value.clone());
                })
                .child(
                    tabs_list()
                        .accessibility_label("Settings")
                        .child(tabs_trigger().value("account").child("Account"))
                        .child(tabs_trigger().value("password").child("Password"))
                        .child(
                            tabs_trigger()
                                .value("billing")
                                .disabled(true)
                                .child("Billing"),
                        ),
                )
                .child(tabs_content().value("account").child("Account panel"))
                .child(tabs_content().value("password").child("Password panel"))
                .child(tabs_content().value("billing").child("Billing panel")),
        )
        .child(
            tabs()
                .default_value(Some(SharedString::from("one")))
                .child(
                    tabs_list()
                        .accessibility_label("Free")
                        .child(tabs_trigger().value("one").child("One"))
                        .child(tabs_trigger().value("two").child("Two")),
                )
                .child(tabs_content().value("one").child("First panel"))
                .child(tabs_content().value("two").child("Second panel")),
        )
        .child(text(move || {
            format!("tab:{} changes:{}", tab.get(), changes.get())
        }))
}

#[test]
fn a_press_selects_a_tab_and_reports_it_once() {
    let mut app = start::<Settings>(true);
    assert!(app.shows("Account panel"));
    assert!(!app.shows("Password panel"));
    app.press("role:tab[Password]").unwrap();
    app.wait_for("text:tab:password changes:1").unwrap();
    app.wait_for("text:Password panel").unwrap();
    assert!(!app.shows("Account panel"));
    // The selected tab reports nothing.
    app.press("role:tab[Password]").unwrap();
    app.press("role:tab[Account]").unwrap();
    app.wait_for("text:tab:account changes:2").unwrap();
}

#[test]
fn uncontrolled_tabs_keep_their_own_value() {
    let mut app = start::<Settings>(true);
    assert!(app.shows("First panel"));
    app.press("role:tab[Two]").unwrap();
    app.wait_for("text:Second panel").unwrap();
    assert!(!app.shows("First panel"));
    assert!(
        app.screen()
            .accessibility_state("role:tab[Two]")
            .unwrap()
            .selected
    );
}

#[test]
fn the_arrows_select_skipping_disabled_tabs() {
    let mut app = start::<Settings>(true);
    app.press_key("Tab").unwrap();
    app.press_key("ArrowRight").unwrap();
    app.wait_for("text:tab:password changes:1").unwrap();
    // Past the disabled tab, around the end.
    app.press_key("ArrowRight").unwrap();
    app.wait_for("text:tab:account changes:2").unwrap();
    app.press_key("End").unwrap();
    app.wait_for("text:tab:password changes:3").unwrap();
}

#[test]
fn a_disabled_tab_does_not_select_and_is_dimmed() {
    let mut app = start::<Settings>(true);
    assert!(!app.screen().enabled("role:tab[Billing]").unwrap());
    let _ = app.press("role:tab[Billing]");
    app.press("role:tab[Password]").unwrap();
    app.wait_for("text:tab:password changes:1").unwrap();
    let style = app
        .screen()
        .view_style("role:tab[Billing]")
        .unwrap()
        .unwrap();
    assert_eq!(style.opacity, 0.5);
}

#[test]
fn screen_readers_get_the_roles_and_states() {
    let app = start::<Settings>(true);
    let screen = app.screen();
    assert!(screen.matches("role:tablist[Settings]").unwrap());
    assert_eq!(
        screen
            .accessibility_orientation("role:tablist[Settings]")
            .unwrap(),
        Some(Axis::Horizontal)
    );
    assert!(
        screen
            .accessibility_state("role:tab[Account]")
            .unwrap()
            .selected
    );
    assert!(
        !screen
            .accessibility_state("role:tab[Password]")
            .unwrap()
            .selected
    );
    assert!(
        screen
            .accessibility_state("role:tab[Billing]")
            .unwrap()
            .disabled
    );
    // One panel each for the two `Tabs`, the others hidden.
    assert_eq!(screen.count("role:tabpanel").unwrap(), 2);
}

/// A medium list and an extra small one, each with one tab.
#[component]
fn Sizes() -> impl IntoElement {
    let list = |size: Size, name: &'static str| {
        tabs().default_value(Some(SharedString::from(name))).child(
            tabs_list()
                .size(size)
                .accessibility_label(name)
                .child(tabs_trigger().value(name).child(name)),
        )
    };
    view()
        .items_start()
        .child(list(Size::Md, "Medium"))
        .child(list(Size::Xs, "Small"))
}

#[test]
fn touch_screens_get_taller_lists() {
    let desktop = start::<Sizes>(true);
    assert_eq!(size(&desktop, "role:tablist[Medium]").1, 32.0);
    assert_eq!(size(&desktop, "role:tab[Medium]").1, 25.0);
    assert_eq!(size(&desktop, "role:tablist[Small]").1, 24.0);
    assert_eq!(size(&desktop, "role:tab[Small]").1, 17.0);
    let touch = start::<Sizes>(false);
    assert_eq!(size(&touch, "role:tablist[Medium]").1, 40.0);
    assert_eq!(size(&touch, "role:tab[Medium]").1, 33.0);
    assert_eq!(size(&touch, "role:tablist[Small]").1, 32.0);
}

/// The two variants, and a vertical list.
#[component]
fn Variants() -> impl IntoElement {
    let list = |variant: TabsVariant, label: &'static str, a: &'static str, b: &'static str| {
        tabs().default_value(Some(SharedString::from(a))).child(
            tabs_list()
                .variant(variant)
                .accessibility_label(label)
                .child(tabs_trigger().value(a).child(a))
                .child(tabs_trigger().value(b).child(b)),
        )
    };
    view()
        .items_start()
        .child(list(TabsVariant::Default, "Segmented", "Alpha", "Beta"))
        .child(list(TabsVariant::Line, "Line", "Gamma", "Delta"))
        .child(
            tabs()
                .orientation(Axis::Vertical)
                .default_value(Some(SharedString::from("general")))
                .child(
                    tabs_list()
                        .accessibility_label("Preferences")
                        .child(tabs_trigger().value("general").child("General"))
                        .child(tabs_trigger().value("appearance").child("Appearance")),
                )
                .child(tabs_content().value("general").child("General panel")),
        )
}

#[test]
fn the_segmented_list_raises_the_selected_tab() {
    let app = start::<Variants>(true);
    assert_eq!(fill(&app, "role:tablist[Segmented]"), light(TABS_LIST));
    assert_eq!(fill(&app, "role:tab[Alpha]"), light(TABS_TRIGGER_SELECTED));
    assert_eq!(fill(&app, "role:tab[Beta]"), None);
}

/// Labels of different lengths in a list as wide as its triggers.
#[component]
fn Ranges() -> impl IntoElement {
    view().items_start().child(
        tabs()
            .default_value(Some(SharedString::from("week")))
            .child(
                tabs_list()
                    .accessibility_label("Range")
                    .child(tabs_trigger().value("day").child("Day"))
                    .child(tabs_trigger().value("week").child("Week"))
                    .child(tabs_trigger().value("month").child("Month"))
                    .child(tabs_trigger().value("year").child("Year")),
            ),
    )
}

/// Every label, a long one too, keeps `tabs-trigger-padding-x` (and the
/// edge) on both sides: the triggers grow from their content.
#[test]
fn every_label_keeps_the_trigger_padding() {
    let app = start::<Ranges>(true);
    for name in ["Day", "Week", "Month", "Year"] {
        let tab = bounds(&app, &format!("role:tab[{name}]"));
        let label = bounds(&app, &format!("text:{name}"));
        let start = (label.origin.x - tab.origin.x).0;
        let end = (tab.origin.x + tab.size.width - label.origin.x - label.size.width).0;
        assert_eq!((start, end), (9.0, 9.0), "{name}");
    }
}

#[test]
fn the_line_list_has_no_fills() {
    let app = start::<Variants>(true);
    assert_eq!(fill(&app, "role:tablist[Line]"), None);
    assert_eq!(fill(&app, "role:tab[Gamma]"), None);
}

#[test]
fn a_vertical_list_is_a_column_of_full_width_tabs() {
    let app = start::<Variants>(true);
    assert_eq!(
        app.screen()
            .accessibility_orientation("role:tablist[Preferences]")
            .unwrap(),
        Some(Axis::Vertical)
    );
    let general = common::bounds(&app, "role:tab[General]");
    let appearance = common::bounds(&app, "role:tab[Appearance]");
    assert_eq!(general.origin.x, appearance.origin.x);
    assert_eq!(general.size.width, appearance.size.width);
    assert!(appearance.origin.y > general.origin.y);
    assert_eq!(general.size.height.0, 25.0);
    // The panel is beside the list.
    let panel = common::bounds(&app, "text:General panel");
    assert!(panel.origin.x > general.origin.x + general.size.width);
}

/// The same tabs outside and inside a scope that changes `tabs-list` and
/// `background`.
#[component]
fn Scoped() -> impl IntoElement {
    let list = |name: &'static str| {
        tabs().default_value(Some(SharedString::from(name))).child(
            tabs_list()
                .accessibility_label(name)
                .child(tabs_trigger().value(name).child(name)),
        )
    };
    view().child(list("Outside")).child(
        theme_scope()
            .tokens(
                ScopeTokens::new()
                    .set("tabs-list", "#2563eb")
                    .set("background", "#16a34a"),
            )
            .child(list("Inside")),
    )
}

#[test]
fn a_scope_reaches_the_list_and_the_selected_tab() {
    let app = start::<Scoped>(true);
    assert_eq!(fill(&app, "role:tablist[Inside]"), css("#2563eb"));
    assert_ne!(fill(&app, "role:tablist[Outside]"), css("#2563eb"));
    // `tabs-trigger-selected` derives from `background`.
    assert_eq!(fill(&app, "role:tab[Inside]"), css("#16a34a"));
    assert_ne!(fill(&app, "role:tab[Outside]"), css("#16a34a"));
}

/// A list in a column that stretches its children, a list in a centered
/// toolbar row beside taller content, and a vertical list beside a tall
/// panel.
#[component]
fn Layouts() -> impl IntoElement {
    view()
        .w(px(500.0))
        .child(
            tabs().default_value(Some(SharedString::from("day"))).child(
                tabs_list()
                    .test_id("in-column")
                    .child(tabs_trigger().value("day").child("Day"))
                    .child(tabs_trigger().value("week").child("Week")),
            ),
        )
        .child(
            tabs().default_value(Some(SharedString::from("day"))).child(
                view()
                    .test_id("row")
                    .flex_row()
                    .items_center()
                    .child(
                        tabs_list()
                            .test_id("in-row")
                            .child(tabs_trigger().value("day").child("Day"))
                            .child(tabs_trigger().value("week").child("Week")),
                    )
                    .child(view().w(px(120.0)).h(px(60.0))),
            ),
        )
        .child(
            tabs()
                .orientation(Axis::Vertical)
                .default_value(Some(SharedString::from("general")))
                .child(
                    tabs_list()
                        .test_id("vertical")
                        .child(tabs_trigger().value("general").child("General"))
                        .child(tabs_trigger().value("security").child("Security")),
                )
                .child(
                    tabs_content()
                        .value("general")
                        .child(view().w(px(200.0)).h(px(240.0))),
                ),
        )
}

#[test]
fn a_stretching_column_does_not_widen_the_list() {
    let app = start::<Layouts>(true);
    let (w, _) = size(&app, "id:in-column");
    assert!(w < 200.0, "as wide as its triggers, not the column: {w}");
}

#[test]
fn a_row_centers_the_list_beside_taller_content() {
    let app = start::<Layouts>(true);
    let row = bounds(&app, "id:row");
    let list = bounds(&app, "id:in-row");
    let above = list.origin.y.0 - row.origin.y.0;
    let below = row.size.height.0 - above - list.size.height.0;
    assert_eq!(row.size.height.0, 60.0);
    assert_eq!(above, below, "centered in the row, not at its top");
}

#[test]
fn a_vertical_list_is_as_tall_as_its_triggers_not_the_panel() {
    let app = start::<Layouts>(true);
    let (w, h) = size(&app, "id:vertical");
    assert!(h < 120.0, "as tall as its two triggers: {h}");
    assert!(w < 200.0, "as wide as its triggers: {w}");
}
