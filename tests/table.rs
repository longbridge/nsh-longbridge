//! `Table` in a headless engine: the roles screen readers get, row heights
//! on a pointer and on a touch screen, the selected and lit rows, the
//! body's last row without its edge, a cell holding a `Checkbox` without
//! its end padding, a label that wraps, and a scope's tokens reaching the
//! look.

mod common;

use std::time::Duration;

use common::{bounds, fill, light, start};
use nsh_core::prelude::*;
use nsh_longbridge::{
    ButtonVariant, ScopeTokens, Size, button, checkbox,
    components::table::{TABLE_ROW_HOVER, TABLE_ROW_SELECTED, TABLE_STICKY_HEADER},
    dropdown_menu, dropdown_menu_item, table, table_body, table_caption, table_cell, table_footer,
    table_head, table_header, table_row, theme_scope,
};
use nsh_testing::Headless;

/// An invoice table: a header, three rows (the second selected, a box in
/// the first, a menu in the third), a footer and a caption; a button adds
/// and removes a row.
#[component]
fn Invoices() -> impl IntoElement {
    let extra = signal(false);
    let picked = signal(false);
    view()
        .w(px(400.0))
        .child(
            table()
                .accessibility_label("Invoices")
                .child(
                    table_header().child(
                        table_row()
                            .test_id("head")
                            .child(table_head().child("Invoice"))
                            .child(table_head().child("Amount")),
                    ),
                )
                .child(
                    table_body()
                        .child(
                            table_row()
                                .test_id("r1")
                                .child(
                                    table_cell()
                                        .test_id("box")
                                        .app_class(Some("flex-none".into()))
                                        .child(
                                            checkbox()
                                                .accessibility_label("Select INV001")
                                                .checked(picked)
                                                .on_change(move |c: &bool| picked.set(*c)),
                                        ),
                                )
                                .child(table_cell().test_id("c1").child("INV001"))
                                .child(
                                    table_cell()
                                        .test_id("square-cell")
                                        .app_class(Some("flex-none".into()))
                                        .child(view().w(px(16.0)).h(px(16.0))),
                                ),
                        )
                        .child(
                            table_row()
                                .test_id("r2")
                                .selected(true)
                                .child(table_cell().child("INV002"))
                                .child(table_cell().child("$150.00")),
                        )
                        .child(
                            table_row()
                                .test_id("r3")
                                .child(table_cell().child("INV003"))
                                .child(
                                    table_cell().child(
                                        dropdown_menu()
                                            .trigger(
                                                button()
                                                    .variant(ButtonVariant::Ghost)
                                                    .child("Actions"),
                                            )
                                            .child(dropdown_menu_item().child("Copy")),
                                    ),
                                ),
                        )
                        .child(when(extra, || {
                            table_row()
                                .test_id("r4")
                                .child(table_cell().child("INV004"))
                                .child(table_cell().child("$90.00"))
                        })),
                )
                .child(
                    table_footer().child(
                        table_row()
                            .test_id("foot")
                            .child(table_cell().child("Total"))
                            .child(table_cell().child("$750.00")),
                    ),
                )
                .child(table_caption().child("Recent invoices.")),
        )
        .child(
            view()
                .test_id("add")
                .w(px(40.0))
                .h(px(40.0))
                .on_press(move || extra.update(|e| *e = !*e)),
        )
        .child(view().test_id("away").w(px(200.0)).h(px(40.0)))
}

/// The row's bottom edge: what its height leaves its cells (they stretch
/// across the rest).
fn bottom_edge(app: &Headless, selector: &str) -> f32 {
    let row = height(app, selector);
    let cell = app
        .screen()
        .layout(&format!("{selector} >> role:cell >> nth=0"))
        .unwrap()
        .or_else(|| {
            app.screen()
                .layout(&format!("{selector} >> role:columnheader >> nth=0"))
                .unwrap()
        })
        .unwrap_or_else(|| panic!("no cell in {selector}"))
        .bounds
        .size
        .height
        .0;
    row - cell
}

fn height(app: &Headless, selector: &str) -> f32 {
    bounds(app, selector).size.height.0
}

#[test]
fn screen_readers_get_the_table_roles_and_the_selected_row() {
    let app = start::<Invoices>(true);
    let screen = app.screen();
    assert!(screen.matches("role:table[Invoices]").unwrap());
    assert_eq!(screen.count("role:rowgroup").unwrap(), 3);
    assert_eq!(screen.count("role:row").unwrap(), 5);
    assert_eq!(screen.count("role:columnheader").unwrap(), 2);
    assert!(screen.accessibility_state("id:r2").unwrap().selected);
    assert!(!screen.accessibility_state("id:r1").unwrap().selected);
}

#[test]
fn rows_are_the_control_height_and_the_header_a_step_taller() {
    let desktop = start::<Invoices>(true);
    assert_eq!(height(&desktop, "id:r2"), 32.0);
    assert_eq!(height(&desktop, "id:head"), 36.0);
    let touch = start::<Invoices>(false);
    assert_eq!(height(&touch, "id:r2"), 40.0);
    assert_eq!(height(&touch, "id:head"), 44.0);
}

/// ★ NSH gaps 2 / 6: the body's last row draws no edge, after rows are
/// added and removed; the header's row keeps its edge; the footer's last
/// row has none.
#[test]
fn the_last_row_of_the_body_draws_no_edge() {
    let mut app = start::<Invoices>(true);
    assert_eq!(bottom_edge(&app, "id:head"), 1.0);
    assert_eq!(bottom_edge(&app, "id:r1"), 1.0);
    assert_eq!(bottom_edge(&app, "id:r3"), 0.0);
    assert_eq!(bottom_edge(&app, "id:foot"), 0.0);
    app.press("id:add").unwrap();
    app.run_for(Duration::from_millis(50)).unwrap();
    assert_eq!(bottom_edge(&app, "id:r3"), 1.0);
    assert_eq!(bottom_edge(&app, "id:r4"), 0.0);
    app.press("id:add").unwrap();
    app.run_for(Duration::from_millis(50)).unwrap();
    assert_eq!(bottom_edge(&app, "id:r3"), 0.0);
}

/// ★ NSH gaps 2 / 6: a cell holding a library `Checkbox` drops its end
/// padding; the others keep it.
#[test]
fn a_cell_holding_a_checkbox_drops_its_end_padding() {
    let app = start::<Invoices>(true);
    let cell = bounds(&app, "id:box");
    let check = bounds(&app, "role:checkbox[Select INV001]");
    assert_eq!(
        check.origin.x.0 - cell.origin.x.0,
        8.0,
        "start padding kept"
    );
    // Two cells as wide as their content (`flex-none`), the same content
    // but for the `Checkbox`: the box's has no end padding.
    let plain = bounds(&app, "id:square-cell");
    assert_eq!(plain.size.width.0 - cell.size.width.0, 8.0);
}

/// The selected row's fill, the hover fill under the pointer (not on the
/// header's row), and the edge color.
#[test]
fn selected_and_hovered_rows_take_their_fills() {
    let mut app = start::<Invoices>(true);
    assert_eq!(fill(&app, "id:r2"), light(TABLE_ROW_SELECTED));
    assert_eq!(fill(&app, "id:r1"), None);
    app.hover("id:r1").unwrap();
    app.run_for(Duration::from_millis(300)).unwrap();
    assert_eq!(fill(&app, "id:r1"), light(TABLE_ROW_HOVER));
    app.hover("id:head").unwrap();
    app.run_for(Duration::from_millis(300)).unwrap();
    assert_eq!(fill(&app, "id:head"), None);
}

/// ★ NSH gaps 3: a row holding an open `DropdownMenu` is lit as under the
/// pointer until the menu closes.
#[test]
fn a_row_holding_an_open_menu_stays_lit() {
    let mut app = start::<Invoices>(true);
    app.hover("id:away").unwrap();
    assert_eq!(fill(&app, "id:r3"), None);
    app.press("role:button[Actions]").unwrap();
    app.run_for(Duration::from_millis(400)).unwrap();
    assert!(app.shows("Copy"));
    app.hover("id:away").unwrap();
    app.run_for(Duration::from_millis(300)).unwrap();
    assert_eq!(fill(&app, "id:r3"), light(TABLE_ROW_HOVER));
    app.press("role:menuitem[Copy]").unwrap();
    app.run_for(Duration::from_millis(400)).unwrap();
    app.hover("id:away").unwrap();
    app.run_for(Duration::from_millis(300)).unwrap();
    assert_eq!(fill(&app, "id:r3"), None);
}

/// A narrow table: a long label is one line in one cell and wraps in
/// another with `labelClass="whitespace-normal"`.
#[component]
fn Narrow() -> impl IntoElement {
    let long = "A long description that does not fit on one line of a narrow cell";
    view().w(px(240.0)).child(
        table().child(
            table_body()
                .child(table_row().test_id("cut").child(table_cell().child(long)))
                .child(
                    table_row().test_id("wraps").child(
                        table_cell()
                            .label_class(Some("whitespace-normal".into()))
                            .child(long),
                    ),
                ),
        ),
    )
}

/// ★ NSH gaps 5: `whitespace-normal` after the label's `truncate` gives
/// back wrapping.
#[test]
fn a_later_class_lifts_the_one_line() {
    let app = start::<Narrow>(true);
    assert_eq!(height(&app, "id:cut"), 32.0);
    assert!(
        height(&app, "id:wraps") > 32.0,
        "the label wraps: {}",
        height(&app, "id:wraps")
    );
}

/// A small table, and a scope changing the selected fill and a sticky
/// header's fill.
#[component]
fn Scoped() -> impl IntoElement {
    view()
        .child(
            table().size(Size::Sm).child(
                table_body().child(table_row().test_id("small").child(table_cell().child("A"))),
            ),
        )
        .child(
            theme_scope()
                .tokens(
                    ScopeTokens::new()
                        .set("table-row-selected", "#2563eb")
                        .set("table-sticky-header", "#16a34a"),
                )
                .child(
                    table()
                        .child(
                            table_header()
                                .sticky(true)
                                .test_id("sticky")
                                .child(table_row().child(table_head().child("Title"))),
                        )
                        .child(
                            table_body().child(
                                table_row()
                                    .test_id("scoped")
                                    .selected(true)
                                    .child(table_cell().child("B")),
                            ),
                        ),
                ),
        )
        .child(
            table().child(
                table_header()
                    .sticky(true)
                    .test_id("plain-sticky")
                    .child(table_row().child(table_head().child("Plain"))),
            ),
        )
        .child(table_caption().child("Caption"))
}

#[test]
fn a_scope_reaches_the_selected_and_sticky_fills() {
    let app = start::<Scoped>(true);
    assert_eq!(height(&app, "id:small"), 28.0);
    assert_eq!(fill(&app, "id:scoped"), common::css("#2563eb"));
    assert_eq!(fill(&app, "id:sticky"), common::css("#16a34a"));
    assert_eq!(fill(&app, "id:plain-sticky"), light(TABLE_STICKY_HEADER));
}

/// ★ NSH gaps 3, from TSX: a `DropdownMenu` in a cell lights its row.
#[test]
fn from_tsx_a_row_holding_an_open_menu_stays_lit() {
    let mut app = common::start_tsx("tests/tsx/table.tsx", true);
    app.hover("id:away").unwrap();
    assert_eq!(fill(&app, "id:row"), None);
    app.press("role:button[Actions]").unwrap();
    app.run_for(Duration::from_millis(400)).unwrap();
    assert!(app.shows("Copy"));
    app.hover("id:away").unwrap();
    app.run_for(Duration::from_millis(300)).unwrap();
    assert_eq!(fill(&app, "id:row"), light(TABLE_ROW_HOVER));
    // A column's classes win over the cell's share; `items-end` over its
    // alignment.
    assert_eq!(bounds(&app, "id:fixed").size.width.0, 100.0);
    let amount = bounds(&app, "id:amount");
    let text = bounds(&app, "id:amount >> text:$250.00");
    let end = |b: nsh_core::geometry::Bounds<Points>| b.origin.x.0 + b.size.width.0;
    assert_eq!(end(amount) - end(text), 8.0);
}
