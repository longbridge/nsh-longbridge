//! `DataTable`: a data table on `nsh/base`'s `DataTable` (virtualized rows
//! and columns, sorting, selection, resizing, moving, fixed columns,
//! loading more, the keyboard, the grid role for screen readers), drawn
//! with `Table`'s look. The library adds the parts' look (classes generated
//! from the tokens), `size`, the bordered box, and a checkbox column. API:
//! `docs/components/data-table.md`.

use nsh_base::{
    DataTable as BaseDataTable, DataTableApi as BaseApi, DataTableEvent as BaseEvent,
    data_table as base_data_table,
};
use nsh_core::{
    element::{Renderer, component_ref},
    prelude::*,
};

pub use nsh_base::{
    DataTableAlign, DataTableCell, DataTableCellClass, DataTableClasses, DataTableColumn,
    DataTableColumnGroup, DataTableColumnMove, DataTableColumnPress, DataTableContextMenu,
    DataTableDump, DataTableHeaderCell, DataTablePress, DataTableRange, DataTableRows,
    DataTableSelection, DataTableSort, DataTableSortDirection, DataTableStyles, DataTableTarget,
    data_table::ColumnFixed,
};

use crate::{
    components::{
        checkbox::{self, checkbox},
        table::{
            TABLE_BORDER, TABLE_CAPTION_FOREGROUND, TABLE_CELL_PADDING_X, TABLE_HEAD_FOREGROUND,
            TABLE_ROW_HOVER, TABLE_ROW_SELECTED, head_height_token, row_height_token,
        },
    },
    foundation::{
        class::Classes,
        icon::provide_text_color,
        labels::{self, Label},
        size::{Size, pointer},
        typography::font_size_token,
    },
    theme::{
        Theme,
        seeds::SPACING,
        semantic::{
            BORDER, BORDER_WIDTH, FOREGROUND, MUTED, MUTED_FOREGROUND, RADIUS_MD, RADIUS_SM, RING,
            TEXT_SM, TEXT_XS,
        },
        theme,
    },
    tokens,
};

tokens! {
    /// The edge of the table's box.
    DATA_TABLE_BORDER: Color = "data-table-border" => |t| t.color(TABLE_BORDER),
    /// The corner radius of the table's box.
    DATA_TABLE_RADIUS: Length = "data-table-radius" => |t| t.length(RADIUS_MD),
    /// The fill of odd rows with `stripe`.
    DATA_TABLE_ROW_ODD: Color = "data-table-row-odd" => |t| t.color(MUTED).opacity(0.5),
    /// A sort button's chevrons (the direction in use takes
    /// `table-head-foreground`).
    DATA_TABLE_SORT_FOREGROUND: Color = "data-table-sort-foreground" => |t| t.color(MUTED_FOREGROUND),
    /// A column's resize line under the pointer.
    DATA_TABLE_RESIZE_HANDLE_HOVER: Color = "data-table-resize-handle-hover" => |t| t.color(BORDER),
    /// The line where a moved column will land.
    DATA_TABLE_MOVE_INDICATOR: Color = "data-table-move-indicator" => |t| t.color(RING),
    /// The line at the fixed columns' edge.
    DATA_TABLE_FIXED_EDGE: Color = "data-table-fixed-edge" => |t| t.color(TABLE_BORDER),
    /// The edge of a selected cell.
    DATA_TABLE_SELECTION_OUTLINE: Color = "data-table-selection-outline" => |t| t.color(RING),
    /// The checkbox column's width at `md`: a box and a cell's padding on
    /// each side.
    DATA_TABLE_CHECKBOX_WIDTH: Length = "data-table-checkbox-width" => |t| t.length(checkbox::CHECKBOX_SIZE_MD) + t.length(TABLE_CELL_PADDING_X) * 2.0,
}

/// The key of the checkbox column.
pub const CHECKBOX_COLUMN: &str = "nlb-select";

/// The props of [`DataTable`]: `nsh/base`'s, passed through, and the
/// library's (`size`, the checkbox column).
#[derive(Clone, PartialEq, Props)]
pub struct DataTableProps {
    /// The columns, left to right. Fixed ones lead.
    #[prop(default)]
    pub columns: Prop<Vec<DataTableColumn>>,
    /// The rows: objects whose fields the columns' `key`s name, or arrays
    /// the columns index.
    #[prop(default)]
    pub data: Prop<DataTableRows>,
    /// How many rows there are, when the app holds only some in `data`.
    pub row_count: Prop<Option<u32>>,
    /// The row of the data's first row (default 0).
    #[prop(default)]
    pub data_start: Prop<u32>,
    /// Classes of the rows given, by their place in `data`, after the
    /// library's.
    #[prop(default)]
    pub row_classes: Prop<Vec<SharedString>>,
    /// Classes of single cells (a data row and a column).
    #[prop(default)]
    pub cell_classes: Prop<Vec<DataTableCellClass>>,
    /// The table's size (`xs`, `sm`, `md`, `lg`): row and title heights and
    /// type, as `Table`'s.
    #[prop(default)]
    pub size: Prop<Size>,
    /// A body row's height, over the size's.
    pub row_height: Prop<Option<Points>>,
    /// A title row's height, over the size's.
    pub header_height: Prop<Option<Points>>,
    /// Labels above the columns, the lowest level first.
    #[prop(default)]
    pub groups: Prop<Vec<Vec<DataTableColumnGroup>>>,
    /// What the data is sorted by, shown on the title; the app sorts.
    pub sort: Prop<Option<DataTableSort>>,
    /// Whether rows select (default true).
    #[prop(default = true)]
    pub row_selectable: Prop<bool>,
    /// Whether columns select (default true).
    #[prop(default = true)]
    pub column_selectable: Prop<bool>,
    /// Whether cells select (default false).
    #[prop(default)]
    pub cell_selectable: Prop<bool>,
    /// Whether a press with Shift or the secondary modifier selects
    /// (default true).
    #[prop(default = true)]
    pub select_on_modifier_press: Prop<bool>,
    /// What is selected, when the app sets it.
    pub selection: Prop<Option<DataTableSelection>>,
    /// The arrow keys go around the ends.
    #[prop(default = true)]
    pub loop_selection: Prop<bool>,
    /// A column of row numbers at the left.
    pub row_header: Prop<Option<bool>>,
    /// What screen readers call the table.
    pub accessibility_label: Prop<Option<SharedString>>,
    /// Odd rows get a faint fill.
    #[prop(default)]
    pub stripe: Prop<bool>,
    /// Whether a drag of a column's edge resizes it (default true).
    #[prop(default = true)]
    pub resizable_columns: Prop<bool>,
    /// Whether a drag of a title moves its column (default true).
    #[prop(default = true)]
    pub movable_columns: Prop<bool>,
    /// The table is loading: the `loadingView` shows.
    #[prop(default)]
    pub loading: Prop<bool>,
    /// There are more rows to load.
    #[prop(default)]
    pub has_more: Prop<bool>,
    /// How many rows from the last make `onLoadMore` fire (default 20).
    #[prop(default = 20)]
    pub load_more_threshold: Prop<u32>,
    /// The font family of cells.
    pub font_family: Prop<Option<SharedString>>,
    /// The font size of cells, over the size's.
    pub font_size: Prop<Option<Points>>,
    /// The scroll bars (default true).
    #[prop(default = true)]
    pub scrollbar: Prop<bool>,
    /// The vertical scroll bar, else as `scrollbar`.
    pub vertical_scrollbar: Prop<Option<bool>>,
    /// The horizontal scroll bar, else as `scrollbar`.
    pub horizontal_scrollbar: Prop<Option<bool>>,
    /// Whether scrolling bounces at the ends (default true).
    #[prop(default = true)]
    pub bounces: Prop<bool>,
    /// The cells of the app's own (only those in view): with it, every
    /// cell is the app's.
    pub render_cell: Prop<Option<Renderer<DataTableCell>>>,
    /// A title drawn by the app, across its cell.
    pub render_header_cell: Prop<Option<Renderer<DataTableHeaderCell>>>,
    /// What shows when there are no rows.
    pub empty: Prop<Option<Slot>>,
    /// What shows over the rows while `loading`.
    pub loading_view: Prop<Option<Slot>>,
    /// Classes of the table's parts, after the library's.
    #[prop(default)]
    pub classes: Prop<DataTableClasses>,
    /// Inline styles of the table's parts, over the classes.
    #[prop(default)]
    pub styles: Prop<DataTableStyles>,
    /// A first column, fixed at the start, with a `Checkbox` per row and a
    /// "select all" one in its title.
    #[prop(default)]
    pub checkbox_column: Prop<bool>,
    /// The checked rows, by their place in `data` (row `dataStart + i` is
    /// `checkedRows[i]`); missing entries are unchecked.
    #[prop(default)]
    pub checked_rows: Prop<Vec<bool>>,
}

impl std::fmt::Debug for DataTableProps {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("DataTableProps").finish_non_exhaustive()
    }
}

/// The events of a [`DataTable`].
#[derive(Debug, Clone, PartialEq, Event)]
pub enum DataTableEvent {
    /// The user changed the selection.
    #[args(selection)]
    SelectionChange(Option<DataTableSelection>),
    /// A row or a cell was pressed.
    #[args(event)]
    RowPress(DataTablePress),
    /// A title was pressed.
    #[args(event)]
    ColumnPress(DataTableColumnPress),
    /// A row, cell or title was pressed twice.
    #[args(target)]
    DoublePress(DataTableTarget),
    /// The focused row was activated (Return).
    #[args(row)]
    RowActivate(u32),
    /// A context menu was asked for.
    #[args(event)]
    ContextMenu(DataTableContextMenu),
    /// The user pressed a sortable title: what to sort by next.
    #[args(sort)]
    SortChange(Option<DataTableSort>),
    /// The user resized a column.
    #[args(key, width)]
    ColumnResize(SharedString, Points),
    /// The user moved a column.
    #[args(event)]
    ColumnMove(DataTableColumnMove),
    /// The rows in view changed.
    #[args(range)]
    VisibleRowsChange(DataTableRange),
    /// The scrolling columns in view changed.
    #[args(range)]
    VisibleColumnsChange(DataTableRange),
    /// The rows in view came near the last while `hasMore`.
    LoadMore,
    /// A row's box was pressed: the data row and the value it asks for.
    #[args(row, checked)]
    CheckedChange(u32, bool),
    /// The title's box was pressed: `true` to check every row of `data`,
    /// `false` to uncheck them.
    #[args(checked)]
    CheckedAllChange(bool),
}

/// The methods of a [`DataTable`]'s ref: `nsh/base`'s, the checkbox
/// column left out of the column indexes.
pub struct DataTableApi {
    inner: ComponentRef<BaseDataTable>,
    shift: Memo<bool>,
}

impl DataTableApi {
    fn base(&self) -> Option<std::rc::Rc<BaseApi>> {
        self.inner.api::<BaseApi>()
    }
}

#[methods]
impl DataTableApi {
    /// Scrolls row `row` into view.
    pub fn scroll_to_row(&self, row: u32, animated: Option<bool>) {
        if let Some(api) = self.base() {
            api.scroll_to_row(row, animated);
        }
    }

    /// Scrolls the column `key` into view.
    pub fn scroll_to_column(&self, key: String, animated: Option<bool>) {
        if let Some(api) = self.base() {
            api.scroll_to_column(key, animated);
        }
    }

    /// What is selected, `null` for nothing.
    pub fn selection(&self) -> Option<DataTableSelection> {
        let shift = self.shift.get_untracked();
        self.base()
            .and_then(|api| api.selection())
            .map(|s| from_base_selection(s, shift))
    }

    /// Selects (`null`: nothing) and brings it into view.
    pub fn select(&self, selection: Option<DataTableSelection>) {
        let shift = self.shift.get_untracked();
        if let Some(api) = self.base() {
            api.select(selection.map(|s| to_base_selection(s, shift)));
        }
    }

    /// The rows in view.
    pub fn visible_rows(&self) -> DataTableRange {
        self.base()
            .map(|api| api.visible_rows())
            .unwrap_or_default()
    }

    /// The scrolling columns in view.
    pub fn visible_columns(&self) -> DataTableRange {
        let shift = self.shift.get_untracked();
        self.base()
            .map(|api| from_base_range(api.visible_columns(), shift))
            .unwrap_or_default()
    }

    /// The widths now, by the columns' order in `columns`.
    pub fn column_widths(&self) -> Vec<Points> {
        let skip = usize::from(self.shift.get_untracked());
        self.base()
            .map(|api| api.column_widths().into_iter().skip(skip).collect())
            .unwrap_or_default()
    }

    /// Sets the width of the column `key` (within its limits).
    pub fn set_column_width(&self, key: String, width: Points) {
        if let Some(api) = self.base() {
            api.set_column_width(key, width);
        }
    }

    /// Fits the column `key` to its widest text and title; the width.
    pub fn fit_column(&self, key: String) -> Result<Points, JsError> {
        match self.base() {
            Some(api) => api.fit_column(key),
            None => Ok(px(0.0)),
        }
    }

    /// The texts of rows `start` to `end` and of the titles, as shown,
    /// without the checkbox column.
    pub fn dump(&self, start: Option<u32>, end: Option<u32>) -> DataTableDump {
        let skip = usize::from(self.shift.get_untracked());
        let Some(api) = self.base() else {
            return DataTableDump::default();
        };
        let mut dump = api.dump(start, end);
        if skip > 0 {
            dump.headers = dump.headers.into_iter().skip(skip).collect();
            for row in &mut dump.rows {
                *row = std::mem::take(row).into_iter().skip(skip).collect();
            }
        }
        dump
    }

    /// Puts the selection's text on the clipboard.
    pub fn copy(&self) {
        if let Some(api) = self.base() {
            api.copy();
        }
    }

    /// Gives the table the keyboard.
    pub fn focus(&self) {
        if let Some(api) = self.base() {
            api.focus();
        }
    }

    /// What the last context menu was asked for at.
    pub fn menu_target(&self) -> Option<DataTableTarget> {
        let shift = self.shift.get_untracked();
        self.base()
            .and_then(|api| api.menu_target())
            .map(|t| from_base_target(t, shift))
    }

    /// Drops the context menu's mark.
    pub fn clear_menu_target(&self) {
        if let Some(api) = self.base() {
            api.clear_menu_target();
        }
    }

    /// Takes the keyboard from the table.
    pub fn blur(&self) {
        if let Some(api) = self.base() {
            api.blur();
        }
    }
}

/// A column index of `nsh/base`'s table as the app counts it (the checkbox
/// column left out); `None` for the checkbox column.
fn from_base(column: u32, shift: bool) -> Option<u32> {
    if shift {
        column.checked_sub(1)
    } else {
        Some(column)
    }
}

/// The app's column index as `nsh/base`'s table counts it.
fn to_base(column: u32, shift: bool) -> u32 {
    column + u32::from(shift)
}

fn from_base_selection(mut s: DataTableSelection, shift: bool) -> DataTableSelection {
    s.column = s.column.and_then(|c| from_base(c, shift));
    s
}

fn to_base_selection(mut s: DataTableSelection, shift: bool) -> DataTableSelection {
    s.column = s.column.map(|c| to_base(c, shift));
    s
}

fn from_base_target(mut t: DataTableTarget, shift: bool) -> DataTableTarget {
    t.column = t.column.and_then(|c| from_base(c, shift));
    t
}

fn from_base_range(mut r: DataTableRange, shift: bool) -> DataTableRange {
    if shift {
        r.start = r.start.saturating_sub(1);
        r.end = r.end.saturating_sub(1);
    }
    r
}

/// A data table: any number of rows and columns, only those in view made;
/// sorting, selection, resizing, moving and fixed columns; in a bordered,
/// rounded box with `Table`'s rows, titles and cells. `checkboxColumn` adds
/// a box per row and a "select all" box, the checked rows lit.
#[component(methods = DataTableApi)]
pub fn DataTable(props: DataTableProps, cx: Scope<DataTableEvent>) -> impl IntoElement {
    provide_text_color();
    let t = theme();
    let size = computed(move || props.size.get());
    let shift = computed(move || props.checkbox_column.get());
    let inner = component_ref::<BaseDataTable>();
    cx.methods(DataTableApi { inner, shift });

    // The checkbox column first, the app's columns after.
    let columns = computed(move || {
        let mut columns = props.columns.get();
        if shift.get() {
            let mut select = DataTableColumn::default();
            select.key = CHECKBOX_COLUMN.into();
            select.title = Some(SharedString::default());
            select.width = Some(checkbox_width(t, size.get()));
            select.min_width = select.width;
            select.max_width = select.width;
            select.align = DataTableAlign::Center;
            select.fixed = ColumnFixed::Left;
            select.resizable = Some(false);
            select.movable = Some(false);
            select.selectable = Some(false);
            columns.insert(0, select);
        }
        columns
    });
    // The rows' own looks: checked rows lit, the last row without its edge,
    // then the app's.
    let row_classes = computed(move || {
        let app = props.row_classes.get();
        let checked = props.checked_rows.get();
        let len = props.data.with(DataTableRows::len);
        let start = props.data_start.get() as usize;
        let total = props
            .row_count
            .get()
            .map_or(start + len, |count| count as usize);
        let lit = Classes::new()
            .color("", "bg", t.color(TABLE_ROW_SELECTED))
            .color("hover", "bg", t.color(TABLE_ROW_SELECTED))
            .build();
        (0..len.max(app.len()))
            .map(|i| {
                let mut c = Classes::new();
                if shift.get() && checked.get(i).copied().unwrap_or(false) {
                    c = c.push(&lit);
                }
                if start + i + 1 == total {
                    c = c.push("border-b-0");
                }
                c.then(app.get(i).cloned())
            })
            .collect::<Vec<_>>()
    });
    let cell_classes = computed(move || {
        let shift = shift.get();
        props
            .cell_classes
            .get()
            .into_iter()
            .map(|mut c| {
                c.column = to_base(c.column, shift);
                c
            })
            .collect::<Vec<_>>()
    });
    let classes = computed(move || part_classes(t, props.classes.get()));
    let selection = move || {
        let shift = shift.get();
        props.selection.get().map(|s| to_base_selection(s, shift))
    };
    let custom_cells = computed(move || shift.get() || props.render_cell.with(Option::is_some));
    let custom_heads =
        computed(move || shift.get() || props.render_header_cell.with(Option::is_some));

    let mut root = base_data_table()
        .component_ref(&inner)
        .columns(move || columns.get())
        .data(props.data)
        .row_count(props.row_count)
        .data_start(props.data_start)
        .row_classes(move || row_classes.get())
        .cell_classes(move || cell_classes.get())
        .row_height(move || {
            props
                .row_height
                .get()
                .unwrap_or_else(|| t.length(row_height_token(size.get())))
        })
        .header_height(move || {
            Some(
                props
                    .header_height
                    .get()
                    .unwrap_or_else(|| t.length(head_height_token(size.get()))),
            )
        })
        .groups(props.groups)
        .sort(props.sort)
        .row_selectable(props.row_selectable)
        .column_selectable(props.column_selectable)
        .cell_selectable(props.cell_selectable)
        .select_on_modifier_press(props.select_on_modifier_press)
        .selection(selection)
        .loop_selection(props.loop_selection)
        .row_header(props.row_header)
        .accessibility_label(props.accessibility_label)
        .stripe(props.stripe)
        .resizable_columns(props.resizable_columns)
        .movable_columns(props.movable_columns)
        .loading(props.loading)
        .has_more(props.has_more)
        .load_more_threshold(props.load_more_threshold)
        .font_family(props.font_family)
        .font_size(move || {
            props
                .font_size
                .get()
                .unwrap_or_else(|| t.length(font_size_token(size.get())))
        })
        .scrollbar(props.scrollbar)
        .vertical_scrollbar(props.vertical_scrollbar)
        .horizontal_scrollbar(props.horizontal_scrollbar)
        .bounces(props.bounces)
        .empty(props.empty.place(|content| content))
        .loading_view(props.loading_view.place(|content| content))
        .classes(move || classes.get())
        .styles(props.styles)
        .app_class(move || Some(root_class(t)))
        .on_selection_change(move |s: &Option<DataTableSelection>| {
            let shift = shift.get_untracked();
            cx.emit(DataTableEvent::SelectionChange(
                s.map(|s| from_base_selection(s, shift)),
            ));
        })
        .on_row_press(move |e: &DataTablePress| {
            let mut e = *e;
            e.column = e.column.and_then(|c| from_base(c, shift.get_untracked()));
            cx.emit(DataTableEvent::RowPress(e));
        })
        .on_column_press(move |e: &DataTableColumnPress| {
            let mut e = *e;
            if let Some(column) = from_base(e.column, shift.get_untracked()) {
                e.column = column;
                cx.emit(DataTableEvent::ColumnPress(e));
            }
        })
        .on_double_press(move |target: &DataTableTarget| {
            cx.emit(DataTableEvent::DoublePress(from_base_target(
                *target,
                shift.get_untracked(),
            )));
        })
        .on_row_activate(move |row: &u32| cx.emit(DataTableEvent::RowActivate(*row)))
        .on_context_menu(move |e: &DataTableContextMenu| {
            let mut e = *e;
            e.column = e.column.and_then(|c| from_base(c, shift.get_untracked()));
            cx.emit(DataTableEvent::ContextMenu(e));
        })
        .on_sort_change(move |sort: &Option<DataTableSort>| {
            cx.emit(DataTableEvent::SortChange(sort.clone()))
        })
        .on_column_resize(move |e: &BaseEvent| {
            if let BaseEvent::ColumnResize(key, width) = e {
                cx.emit(DataTableEvent::ColumnResize(key.clone(), *width));
            }
        })
        .on_column_move(move |e: &DataTableColumnMove| {
            let mut e = e.clone();
            if shift.get_untracked() {
                e.from = e.from.saturating_sub(1);
                e.to = e.to.saturating_sub(1);
                e.order.retain(|key| key != CHECKBOX_COLUMN);
            }
            cx.emit(DataTableEvent::ColumnMove(e));
        })
        .on_visible_rows_change(move |range: &DataTableRange| {
            cx.emit(DataTableEvent::VisibleRowsChange(*range))
        })
        .on_visible_columns_change(move |range: &DataTableRange| {
            cx.emit(DataTableEvent::VisibleColumnsChange(from_base_range(
                *range,
                shift.get_untracked(),
            )))
        })
        .on_load_more(move || cx.emit(DataTableEvent::LoadMore));

    let parts = root.__element().__props_mut();
    let render_cell = cell_renderer(props.clone(), t, size, shift, cx);
    let render_head = head_renderer(props.clone(), size, shift, cx);
    parts.render_cell = Prop::from(
        IntoReactive::<Option<Renderer<DataTableCell>>>::into_reactive(move || {
            custom_cells.get().then(|| render_cell.clone())
        }),
    );
    parts.render_header_cell = Prop::from(
        IntoReactive::<Option<Renderer<DataTableHeaderCell>>>::into_reactive(move || {
            custom_heads.get().then(|| render_head.clone())
        }),
    );
    root
}

/// The checkbox column's width at `size`: the box and a cell's padding on
/// each side.
fn checkbox_width(t: Theme, size: Size) -> Points {
    t.length(DATA_TABLE_CHECKBOX_WIDTH) - t.length(checkbox::CHECKBOX_SIZE_MD)
        + t.length(checkbox::size_token(size))
}

/// The box around the table: its edge and corners, clipping the rows; the
/// table's text color.
fn root_class(t: Theme) -> SharedString {
    Classes::new()
        .push("overflow-hidden")
        .length("", "border", t.length(BORDER_WIDTH))
        .color("", "border", t.color(DATA_TABLE_BORDER))
        .length("", "rounded", t.length(DATA_TABLE_RADIUS))
        .color("", "text", t.color(FOREGROUND))
        .build()
}

/// Every part's look from the tokens, the app's classes after each.
fn part_classes(t: Theme, app: DataTableClasses) -> DataTableClasses {
    let x = t.length(TABLE_CELL_PADDING_X);
    let edge = t.length(BORDER_WIDTH);
    let border = t.color(TABLE_BORDER);
    let hover = t.color(TABLE_ROW_HOVER);
    let selected = t.color(TABLE_ROW_SELECTED);
    let head = || {
        Classes::new()
            .length("", "px", x)
            .color("", "text", t.color(TABLE_HEAD_FOREGROUND))
            .push("font-medium")
    };
    let mut c = DataTableClasses::default();
    let merge = |look: Classes, app: &Option<SharedString>| Some(look.then(app.clone()));
    c.root = app.root.clone();
    c.header = merge(
        Classes::new()
            .length("", "border-b", edge)
            .color("", "border", border),
        &app.header,
    );
    c.group_cell = merge(
        head()
            .push("justify-center")
            .length("", "border-b", edge)
            .color("", "border", border),
        &app.group_cell,
    );
    c.header_cell = merge(head(), &app.header_cell);
    c.header_content = merge(
        Classes::new().length("", "gap", t.length(SPACING)),
        &app.header_content,
    );
    c.header_cell_dragging = merge(
        Classes::new().color("", "bg", hover).push("opacity-80"),
        &app.header_cell_dragging,
    );
    c.header_cell_sorted = app.header_cell_sorted.clone();
    c.header_cell_selected = merge(
        Classes::new().color("", "bg", selected),
        &app.header_cell_selected,
    );
    let sort = Classes::new()
        .color("", "text", t.color(DATA_TABLE_SORT_FOREGROUND))
        .length("", "rounded", t.length(RADIUS_SM));
    let sort = if pointer() {
        sort.color("hover", "bg", hover)
    } else {
        sort
    };
    c.sort_indicator = merge(sort, &app.sort_indicator);
    let direction = || Classes::new().color("selected", "text", t.color(TABLE_HEAD_FOREGROUND));
    c.sort_ascending = merge(direction(), &app.sort_ascending);
    c.sort_descending = merge(direction(), &app.sort_descending);
    c.resize_handle = merge(
        // The band around the line is a `group`: its hover and press.
        Classes::new()
            .color("group-hover", "bg", t.color(DATA_TABLE_RESIZE_HANDLE_HOVER))
            .color(
                "group-active",
                "bg",
                t.color(DATA_TABLE_RESIZE_HANDLE_HOVER),
            ),
        &app.resize_handle,
    );
    c.move_indicator = merge(
        Classes::new().color("", "bg", t.color(DATA_TABLE_MOVE_INDICATOR)),
        &app.move_indicator,
    );
    c.row = merge(
        Classes::new()
            .length("", "border-b", edge)
            .color("", "border", border),
        &app.row,
    );
    c.row_odd = merge(
        Classes::new().color("", "bg", t.color(DATA_TABLE_ROW_ODD)),
        &app.row_odd,
    );
    c.row_hover = merge(
        if pointer() {
            Classes::new().color("", "bg", hover)
        } else {
            Classes::new()
        },
        &app.row_hover,
    );
    c.row_selected = merge(Classes::new().color("", "bg", selected), &app.row_selected);
    c.row_active = merge(Classes::new().color("", "bg", hover), &app.row_active);
    c.cell = merge(Classes::new().length("", "px", x), &app.cell);
    c.cell_selected = merge(
        Classes::new()
            .color("", "bg", selected)
            .length("", "border", edge)
            .color("", "border", t.color(DATA_TABLE_SELECTION_OUTLINE)),
        &app.cell_selected,
    );
    c.cell_active = merge(Classes::new().color("", "bg", hover), &app.cell_active);
    c.cell_column_selected = merge(
        Classes::new().color("", "bg", hover),
        &app.cell_column_selected,
    );
    let xs = t.length(TEXT_XS);
    c.row_header = merge(
        Classes::new()
            .push("justify-center")
            .color("", "text", t.color(MUTED_FOREGROUND))
            .length("", "text", xs)
            .length("", "border-r", edge)
            .color("", "border", border),
        &app.row_header,
    );
    c.row_header_corner = merge(
        Classes::new()
            .length("", "border-r", edge)
            .length("", "border-b", edge)
            .color("", "border", border),
        &app.row_header_corner,
    );
    c.fixed_edge = merge(
        Classes::new().color("", "bg", t.color(DATA_TABLE_FIXED_EDGE)),
        &app.fixed_edge,
    );
    c.empty = merge(
        Classes::new()
            .push("items-center justify-center")
            .length("", "h", t.length(SPACING) * 24.0)
            .color("", "text", t.color(TABLE_CAPTION_FOREGROUND))
            .length("", "text", t.length(TEXT_SM)),
        &app.empty,
    );
    c
}

/// The cells, while the checkbox column is on or the app draws its own: a
/// box in the checkbox column, the app's cell (its column counted as the
/// app counts it), else the cell's text as the table would show it.
fn cell_renderer(
    props: DataTableProps,
    t: Theme,
    size: Memo<Size>,
    shift: Memo<bool>,
    cx: Scope<DataTableEvent>,
) -> Renderer<DataTableCell> {
    let _ = t;
    Renderer::new(move |cell: &DataTableCell| -> AnyElement {
        let row = cell.row;
        if shift.get_untracked() && cell.key == CHECKBOX_COLUMN {
            let checked = move || {
                let start = props.data_start.get();
                row.checked_sub(start)
                    .and_then(|i| props.checked_rows.with(|c| c.get(i as usize).copied()))
                    .unwrap_or(false)
            };
            return view()
                .flex_1()
                .flex_row()
                .items_center()
                .justify_center()
                .child(
                    checkbox()
                        .size(move || size.get())
                        .checked(checked)
                        .accessibility_label(move || {
                            Some(format!("{} {}", labels::text(Label::SelectRow), row + 1).into())
                        })
                        .on_change(move |next: &bool| {
                            cx.emit(DataTableEvent::CheckedChange(row, *next))
                        }),
                )
                .into_element();
        }
        let mut cell = cell.clone();
        if let Some(column) = from_base(cell.column, shift.get_untracked()) {
            cell.column = column;
        }
        if let Some(render) = props.render_cell.get() {
            return render
                .render(format!("cell:{row}:{}", cell.column), cell)
                .flex_1()
                .min_w(px(0.0))
                .into_element();
        }
        let align = props.columns.with(|columns| {
            columns
                .iter()
                .find(|c| c.key == cell.key)
                .map(|c| c.align)
                .unwrap_or_default()
        });
        let label = text(cell.text.clone())
            .number_of_lines(1)
            .flex_1()
            .min_w(px(0.0));
        match align {
            DataTableAlign::Start => label.text_left(),
            DataTableAlign::Center => label.text_center(),
            DataTableAlign::End => label.text_right(),
        }
        .into_element()
    })
}

/// The titles, while the checkbox column is on or the app draws its own:
/// the "select all" box in the checkbox column, the app's title, else the
/// title's text (the sort button stays `nsh/base`'s).
fn head_renderer(
    props: DataTableProps,
    size: Memo<Size>,
    shift: Memo<bool>,
    cx: Scope<DataTableEvent>,
) -> Renderer<DataTableHeaderCell> {
    Renderer::new(move |head: &DataTableHeaderCell| -> AnyElement {
        if shift.get_untracked() && head.key == CHECKBOX_COLUMN {
            let count = move || {
                let len = props.data.with(DataTableRows::len);
                let checked = props
                    .checked_rows
                    .with(|c| c.iter().take(len).filter(|&&c| c).count());
                (checked, len)
            };
            return view()
                .flex_1()
                .flex_row()
                .items_center()
                .justify_center()
                .child(
                    checkbox()
                        .size(move || size.get())
                        .checked(move || {
                            let (checked, len) = count();
                            len > 0 && checked == len
                        })
                        .indeterminate(move || {
                            let (checked, len) = count();
                            checked > 0 && checked < len
                        })
                        .accessibility_label(move || Some(labels::text(Label::SelectAll)))
                        .on_change(move |next: &bool| {
                            cx.emit(DataTableEvent::CheckedAllChange(*next))
                        }),
                )
                .into_element();
        }
        let mut head = head.clone();
        if let Some(column) = from_base(head.column, shift.get_untracked()) {
            head.column = column;
        }
        if let Some(render) = props.render_header_cell.get() {
            return render
                .render(format!("head:{}", head.column), head)
                .flex_1()
                .min_w(px(0.0))
                .into_element();
        }
        let align = props.columns.with(|columns| {
            columns
                .iter()
                .find(|c| c.key == head.key)
                .map(|c| c.align)
                .unwrap_or_default()
        });
        let label = text(head.title.clone())
            .number_of_lines(1)
            .flex_1()
            .min_w(px(0.0));
        match align {
            DataTableAlign::Start => label.text_left(),
            DataTableAlign::Center => label.text_center(),
            DataTableAlign::End => label.text_right(),
        }
        .into_element()
    })
}
