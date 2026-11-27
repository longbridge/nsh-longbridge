//! `DataTable` in a headless engine: row heights at a size on a pointer
//! and on a touch screen, a scope's tokens and a forced appearance reaching
//! a selected row, the app's part classes after the library's, the resize
//! line under the pointer, and the checkbox column (a box's press reports
//! the app's data row, the boxes follow `checkedRows` through a sort, the
//! "select all" box, column indexes as the app counts them), the empty
//! state centered under the titles.

mod common;

use std::sync::Mutex;

use common::{bounds, fill, light, start};
use nsh_core::{Checked, NodeId, prelude::*};
use nsh_longbridge::{
    DataTableClasses, DataTableColumn, DataTableEvent, DataTablePress, DataTableRows,
    DataTableSelection, DataTableSort, DataTableSortDirection, ScopeTokens, ThemeAppearance,
    components::{data_table::DATA_TABLE_RESIZE_HANDLE_HOVER, table::TABLE_ROW_SELECTED},
    data_table, theme_scope,
};
use nsh_testing::Headless;

fn column(key: &str, title: &str) -> DataTableColumn {
    let mut c = DataTableColumn::default();
    c.key = key.to_owned().into();
    c.title = Some(title.to_owned().into());
    c.width = Some(px(120.0));
    c.sortable = true;
    c
}

const NAMES: [&str; 4] = ["Ada", "Ben", "Cy", "Dee"];
const AMOUNTS: [f64; 4] = [250.0, 150.0, 350.0, 450.0];

/// The rows in `order` (indexes into the names).
fn rows(order: &[usize]) -> DataTableRows {
    DataTableRows::new(order.len())
        .texts("name", order.iter().map(|&i| NAMES[i]))
        .numbers("amount", order.iter().map(|&i| AMOUNTS[i]))
}

/// What the tables reported, in order.
static SEEN: Mutex<Vec<String>> = Mutex::new(Vec::new());

fn note(s: String) {
    SEEN.lock().unwrap().push(s);
}

fn seen() -> Vec<String> {
    SEEN.lock().unwrap().clone()
}

fn forget() {
    SEEN.lock().unwrap().clear();
}

/// Payments with a checkbox column: the app keeps checked names and sorts
/// by name on a press of its title; a plain table beside it with row 0
/// selected and the app's cell padding; the same in a scope forced dark
/// with another selected fill.
#[component]
fn Payments() -> impl IntoElement {
    let order = signal(vec![0usize, 1, 2, 3]);
    let checked = signal(Vec::<usize>::new());
    let sort = signal(None::<DataTableSort>);
    let mut selected = DataTableSelection::default();
    selected.row = Some(0);
    let mut padded = DataTableClasses::default();
    padded.cell = Some("px-4".into());
    let plain = move |label: &'static str| {
        data_table()
            .app_class(Some("h-[200px]".into()))
            .accessibility_label(label)
            .columns(vec![column("name", "Name"), column("amount", "Amount")])
            .data(rows(&[0, 1, 2, 3]))
            .selection(Some(selected))
            .classes(padded.clone())
    };
    view()
        .w(px(600.0))
        .child(
            data_table()
                .app_class(Some("h-[200px]".into()))
                .accessibility_label("Payments")
                .checkbox_column(true)
                .columns(vec![column("name", "Name"), column("amount", "Amount")])
                .data(move || rows(&order.get()))
                .checked_rows(move || {
                    let checked = checked.get();
                    order
                        .get()
                        .iter()
                        .map(|i| checked.contains(i))
                        .collect::<Vec<_>>()
                })
                .sort(sort)
                .on_checked_change(move |e: &DataTableEvent| {
                    let DataTableEvent::CheckedChange(row, on) = e else {
                        return;
                    };
                    note(format!("checked:{row}:{on}"));
                    let name = order.with(|o| o[*row as usize]);
                    checked.update(|c| {
                        c.retain(|&i| i != name);
                        if *on {
                            c.push(name);
                        }
                    });
                })
                .on_checked_all_change(move |on: &bool| {
                    note(format!("all:{on}"));
                    checked.set(if *on { order.get() } else { Vec::new() });
                })
                .on_row_press(move |e: &DataTablePress| {
                    note(format!("row:{}:{:?}", e.row, e.column));
                })
                .on_sort_change(move |next: &Option<DataTableSort>| {
                    let mut next = next.clone();
                    if let Some(s) = next.as_mut() {
                        s.direction = DataTableSortDirection::Descending;
                    }
                    let mut o = vec![0usize, 1, 2, 3];
                    if next.is_some() {
                        o.sort_by(|a, b| NAMES[*b].cmp(NAMES[*a]));
                    }
                    order.set(o);
                    sort.set(next);
                }),
        )
        .child(plain("Plain"))
        .child(
            theme_scope()
                .appearance(ThemeAppearance::Dark)
                .tokens(ScopeTokens::new().set("table-row-selected", "#1e3a8a"))
                .child(plain("Scoped")),
        )
}

/// The cell of `grid` showing `text`.
fn row_of(_app: &Headless, grid: &str, text: &str) -> String {
    format!("role:grid[{grid}] >> role:cell[{text}]")
}

#[test]
fn rows_take_the_sizes_height() {
    let desktop = start::<Payments>(true);
    let cell = bounds(&desktop, &row_of(&desktop, "Plain", "Ben"));
    let above = bounds(&desktop, &row_of(&desktop, "Plain", "Ada"));
    assert_eq!(cell.origin.y.0 - above.origin.y.0, 32.0);
    let touch = start::<Payments>(false);
    let cell = bounds(&touch, &row_of(&touch, "Plain", "Ben"));
    let above = bounds(&touch, &row_of(&touch, "Plain", "Ada"));
    assert_eq!(cell.origin.y.0 - above.origin.y.0, 40.0);
}

/// The view `selector` is in, `levels` up.
fn ancestor(app: &Headless, selector: &str, levels: usize) -> NodeId {
    let screen = app.screen();
    let nodes = screen.layout_nodes();
    let mut id = screen.id(selector).unwrap();
    for _ in 0..levels {
        id = nodes.iter().find(|n| n.id == id).unwrap().parent;
    }
    id
}

/// The first fill from `selector` up.
fn fill_up(app: &Headless, selector: &str) -> Option<u32> {
    let screen = app.screen();
    let nodes = screen.layout_nodes();
    let mut id = screen.id(selector).unwrap();
    loop {
        let style = screen.view_style(&format!("view:{id}")).unwrap();
        if let Some(f) = style.and_then(|s| s.background_color) {
            return Some(f);
        }
        id = nodes.iter().find(|n| n.id == id)?.parent;
    }
}

/// ★ NSH gaps 1: the selected row's fill comes from the tokens, a scope's
/// and its forced appearance included; the app's `classes.cell` wins over
/// the library's padding.
#[test]
fn a_scope_reaches_the_selected_row_and_the_apps_classes_win() {
    let app = start::<Payments>(true);
    assert_eq!(
        fill_up(&app, &row_of(&app, "Plain", "Ada")),
        light(TABLE_ROW_SELECTED)
    );
    assert_eq!(
        fill_up(&app, &row_of(&app, "Scoped", "Ada")),
        common::css("#1e3a8a")
    );
    assert_eq!(fill_up(&app, &row_of(&app, "Plain", "Ben")), None);
    // `px-4`: 16 from the cell's start.
    let text = bounds(&app, "role:grid[Plain] >> role:cell[Ben] >> text:Ben");
    let cell = bounds(&app, &row_of(&app, "Plain", "Ben"));
    assert_eq!(text.origin.x.0 - cell.origin.x.0, 16.0);
    // The library's: `table-cell-padding-x`, 8.
    let text = bounds(&app, "role:grid[Payments] >> role:cell[Ben] >> text:Ben");
    let cell = bounds(&app, &row_of(&app, "Payments", "Ben"));
    assert_eq!(text.origin.x.0 - cell.origin.x.0, 8.0);
}

/// ★ NSH gaps 3: the resize band under the pointer lights its line.
#[test]
fn the_resize_line_lights_under_the_pointer() {
    let mut app = start::<Payments>(true);
    let line = "role:grid[Plain] >> id:resize:name";
    assert_eq!(fill(&app, line), None);
    let band = format!("view:{}", ancestor(&app, line, 1));
    app.hover(&band).unwrap();
    assert_eq!(fill(&app, line), light(DATA_TABLE_RESIZE_HANDLE_HOVER));
}

fn checked(app: &Headless, name: &str) -> Option<Checked> {
    app.screen()
        .accessibility(Some(&format!(
            "role:grid[Payments] >> role:checkbox[{name}]"
        )))
        .unwrap()
        .unwrap()
        .checked
}

/// ★ NSH gaps 6: a box's press reports the data row; the boxes stay on the
/// app's rows through a sort; the title's box checks every row; row
/// presses count the columns as the app does.
#[test]
fn the_checkbox_column_follows_the_apps_rows() {
    forget();
    let mut app = start::<Payments>(true);
    app.press("role:grid[Payments] >> role:checkbox[Select row 2]")
        .unwrap();
    app.run_for(std::time::Duration::from_millis(50)).unwrap();
    assert_eq!(seen(), ["checked:1:true"]);
    assert_eq!(checked(&app, "Select row 2"), Some(Checked::True));
    assert_eq!(checked(&app, "Select all"), Some(Checked::Mixed));
    // Ben's row is lit.
    assert_eq!(
        fill_up(&app, &row_of(&app, "Payments", "Ben")),
        light(TABLE_ROW_SELECTED)
    );
    // Sorted by name descending: Dee, Cy, Ben, Ada; Ben is row 3.
    // The title's sort button.
    app.press("role:grid[Payments] >> role:columnheader[Name] >> role:button")
        .unwrap();
    app.run_for(std::time::Duration::from_millis(50)).unwrap();
    assert_eq!(checked(&app, "Select row 3"), Some(Checked::True));
    assert_eq!(checked(&app, "Select row 2"), Some(Checked::False));
    // The title's box.
    app.press("role:grid[Payments] >> role:checkbox[Select all]")
        .unwrap();
    app.run_for(std::time::Duration::from_millis(50)).unwrap();
    assert_eq!(checked(&app, "Select all"), Some(Checked::True));
    for row in 1..=4 {
        assert_eq!(
            checked(&app, &format!("Select row {row}")),
            Some(Checked::True)
        );
    }
    // A press on Ada's amount: column 1, as the app counts.
    forget();
    app.press("role:grid[Payments] >> text:250").unwrap();
    assert!(
        seen().iter().any(|s| s == "row:3:Some(1)"),
        "the row press counts the app's columns: {:?}",
        seen()
    );
}

/// A table with no rows, taller than the empty part's minimum.
#[component]
fn NoRows() -> impl IntoElement {
    view().w(px(600.0)).child(
        data_table()
            .app_class(Some("h-[300px]".into()))
            .accessibility_label("None")
            .columns(vec![column("name", "Name"), column("amount", "Amount")])
            .data(rows(&[]))
            .empty(text("No results.")),
    )
}

/// The empty slot is centered in the room under the titles, as a
/// `loadingView` is, not pinned under them at its minimum height.
#[test]
fn the_empty_state_is_centered_under_the_titles() {
    let app = start::<NoRows>(true);
    let table = bounds(&app, "role:grid[None]");
    let title = bounds(&app, "role:grid[None] >> role:columnheader[Name]");
    let empty = bounds(&app, "text:No results.");
    let below = title.origin.y.0 + title.size.height.0;
    let bottom = table.origin.y.0 + table.size.height.0;
    let center = empty.origin.y.0 + empty.size.height.0 / 2.0;
    assert!(
        (center - (below + bottom) / 2.0).abs() <= 1.0,
        "the empty text's center {center} is not between {below} and {bottom}"
    );
}
