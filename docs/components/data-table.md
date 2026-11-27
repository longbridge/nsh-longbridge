# DataTable

Status: spec (P3), not implemented.

A table over any number of rows and columns: `nsh/base`'s `DataTable` (virtualized rows and columns, sorting,
selection, resizing, moving, fixed columns, loading more, a keyboard cursor) drawn with the table look of
[Table](table.md) and shadcn/ui's data-table guide (a `Table` in a `rounded-md border` box, a checkbox column, a
sortable title). The row, title and cell tokens are `Table`'s (`table-*`); this spec adds what only a data table has.

## Usage

- Rows the user works with: sorts, selects, resizes or moves columns, copies; many rows (a log, orders, a
  million quotes served page by page); columns that stay at a side while the others scroll.
- Search terms: data table, data grid, grid, spreadsheet-like table, sortable table, virtualized table, table with
  row selection, table with checkboxes.

## When not to use

| Need | Use instead |
|---|---|
| A few rows of mixed content the app lays out (a badge and a button in a cell, rows of different heights) | [Table](table.md) (see "Table or DataTable" there) |
| A list of items of different shapes | `nsh/base`'s `VirtualList` |
| Items that hold items | `nsh/base`'s `Tree` |
| Filtering and the columns menu of shadcn's guide | An `Input` and a `DropdownMenu` above the table, which change the app's `data` and `columns`; not part of `DataTable` |

## Built on

`nsh/base`'s `DataTable` (`crates/nsh-base/src/data_table/`). All behavior is its own and not rewritten here:

| Provided by `nsh/base` | |
|---|---|
| Data | `columns`, `data` (objects by the columns' `key`s, or arrays), `rowCount` + `dataStart` for rows the app serves page by page (`onVisibleRowsChange`, `onLoadMore` with `hasMore` / `loading`) |
| Virtualization | Only the rows and columns in view (and a few past each edge) are made, reused while scrolling; the cost of a scroll is the cells that change |
| Scrolling | Its own, both ways, with the platform's scroll bars, inertia and wheel; the header stays above the rows and the fixed columns at their side (`fixed: true` / `"left"` / `"right"`) |
| Columns | `width`, `minWidth`, `maxWidth`, `grow`, `autoFit`, `fitPadding`, `align`, resize by a drag of the title's edge (a double press fits), move by a drag of the title, group labels above them (`groups`) |
| Sorting | A sort button in a `sortable` column's title; `onSortChange` reports the next sort (unsorted, ascending, descending), the app sorts its data and sets `sort` |
| Selection | One row, one column, one cell or a range of cells (`rowSelectable`, `columnSelectable`, `cellSelectable`, `cellRangeSelectable`, `selection`, `onSelectionChange`); a row header of numbers; `selectOnModifierPress` off for the app's own selection of several |
| Keyboard | One Tab stop; while it has the keyboard the arrows, Home, End, Page Up / Down and Tab move the selection, Return reports `onRowActivate`, Escape gives the keyboard back, the platform's copy shortcut copies |
| Presses | `onRowPress`, `onColumnPress`, `onDoublePress`, `onContextMenu` (a secondary or long press, for the app's `Menu`); a press on a pressable view `renderCell` drew goes to that view first |
| Cells | Text by default; `renderCell` / `renderHeaderCell` draw the app's own (only the cells in view); `rowClasses`, `cellClasses`, a column's `cellClass`, `headerClass`, `positiveClass`, `negativeClass` |
| Ref | `scrollToRow`, `scrollToColumn`, `selection`, `select`, `visibleRows`, `visibleColumns`, `columnWidths`, `setColumnWidth`, `fitColumn`, `dump`, `copy`, `focus`, `blur`, `menuTarget`, `clearMenuTarget` |
| Accessibility | A `grid` named by `accessibilityLabel`, `columnheader`s with their sort, `cell`s with their selected state |
| Look | None: it styles its parts with the app's `classes` and `styles` (`root`, `header`, `headerCell`, `row`, `rowHover`, `rowSelected`, `cell`, `sortIndicator`, …) |
| Terminal | A row per Cell, its own glyphs for sorting; this library's look does not apply there |

This library adds: the look of every part from tokens (as generated classes, below), `size` (row and title heights,
type), the box around it, the empty state's place, and a **checkbox column**: a box per row and a "select all" box in
its title, with the checked rows lit (shadcn's row selection).

### Implementation

- `#[component] fn DataTable` whose body is `nsh_base::data_table()` with every prop and event passed through, and the
  ref's methods forwarded (`DataTableApi`).
- **The look is classes generated from the resolved tokens**, per part, in a binding (so a `ThemeScope` and its
  forced appearance reach it, and a token change shows at once), joined before the app's `classes` of the same part.
  So an app's class for a part wins over the library's (`classes={{ cell: "px-4" }}`), and its `styles` win over
  both (`nsh/base`: styles over classes). The same mechanism as `Select`'s generated highlight (select.md, NSH gaps 1).
  The library writes no `styles`.
- **`size`** gives `rowHeight` (`table-row-height-<size>`), `headerHeight` (`table-head-height-<size>`) and
  `fontSize` (`font-size-<size>`) unless the app sets them.
- **The last row draws no bottom edge**: the library adds `border-b-0` to the `rowClasses` entry of row
  `rowCount − 1` when the app's data holds it (shadcn's `[&_tr:last-child]:border-0`), the app's entries kept after.
- **The checkbox column** (`checkboxColumn`): the library puts a column of its own first (`key` `"nlb-select"`,
  `fixed: "left"`, width `data-table-checkbox-width`, not `sortable`, `resizable`, `movable` or `selectable`) and
  draws it through `renderCell` / `renderHeaderCell` (calling the app's for every other column, or, without one,
  drawing the cell's text as the table would, in Rust: `nsh/base`'s renderer is the whole table's, NSH gaps 6): a
  library `Checkbox`
  of the table's `size`, named "Select row" / "Select all" (`labels`). Checked rows take `table-row-selected` through
  the generated `rowClasses`, after the app's. Column indexes in events stay the app's: the library's column is
  removed from what it reports (`column` − 1, `order` without it).
- The table is a `Table`-look surface: the root takes the edge and the radius; its parts the rows' and cells' looks.

## API

Coming from shadcn's data-table guide: TanStack's `columns` and `data` are `columns` and `data`; `flexRender` of a
cell is `renderCell`; `getSortedRowModel` is the app's own sort in `onSortChange`; row selection is
`checkboxColumn` + `checkedRows`; pagination is a [Pagination](pagination.md) under the table with the app's page of
`data`, or no pages at all (a virtualized table scrolls through every row).

### Props

`nsh/base`'s props, passed through unchanged: `columns`, `data`, `rowCount`, `dataStart`, `rowClasses`,
`cellClasses`, `rowHeight`, `headerHeight`, `groups`, `sort`, `rowSelectable`, `columnSelectable`, `cellSelectable`,
`cellRangeSelectable`, `selectOnModifierPress`, `selection`, `loopSelection`, `rowHeader`, `accessibilityLabel`,
`stripe`, `resizableColumns`, `movableColumns`, `loading`, `hasMore`, `loadMoreThreshold`, `fontFamily`, `fontSize`,
`scrollbar`, `verticalScrollbar`, `horizontalScrollbar`, `bounces`, `renderCell`, `renderHeaderCell`, `empty`,
`loadingView`, `classes`, `styles`. Their types and defaults are `nsh/base`'s, except:

| TSX | Rust | Type | Default here | Description |
|---|---|---|---|---|
| `rowHeight` | `row_height` | `number` | `table-row-height-<size>` (32 at `md`) | `nsh/base`'s default 32 |
| `headerHeight` | `header_height` | `number` | `table-head-height-<size>` (36 at `md`) | `nsh/base`'s default the row height |
| `fontSize` | `font_size` | `number` | `font-size-<size>` (14 at `md`) | `nsh/base`'s default 14 |
| `classes` / `styles` | `classes` / `styles` | `DataTableClasses` / `DataTableStyles` | — | The app's, per part, after the library's look |

Added by this library:

| TSX | Rust | Type | Default | Description |
|---|---|---|---|---|
| `size` | `size` | `Size` | `"md"` | Row and title heights and type (Table's sizes) |
| `checkboxColumn` | `checkbox_column` | `boolean` | `false` | A first column, fixed at the start, with a `Checkbox` per row and a "select all" one in its title |
| `checkedRows` | `checked_rows` | `boolean[]` / `Vec<bool>` | `[]` | The checked rows, by their place in `data` as `rowClasses` (row `dataStart + i` is `checkedRows[i]`); missing entries are unchecked. The app keeps its own ids and maps them each time (`data.map((r) => picked.has(r.id))`), so a sort or a filter keeps what is checked |
| `class` / `style` | `class` / `style` | | — | The root (its box), merged last; shorthand for `classes.root` / `styles.root` |

The checkbox column's boxes are controlled: the app owns `checkedRows`.

### Events

`nsh/base`'s, passed through: `onSelectionChange({ selection })`, `onRowPress({ event })`, `onColumnPress({ event })`,
`onDoublePress({ target })`, `onRowActivate({ row })`, `onContextMenu({ event })`, `onSortChange({ sort })`,
`onColumnResize({ key, width })`, `onColumnMove({ event })`, `onVisibleRowsChange({ range })`,
`onVisibleColumnsChange({ range })`, `onLoadMore`. Added:

| TSX | Rust (`DataTableEvent`) | Payload | When |
|---|---|---|---|
| `onCheckedChange` | `CheckedChange(u32, bool)` | `{ row, checked }` | A row's box was pressed: `row` is the data row (`onRowPress`'s number) |
| `onCheckedAllChange` | `CheckedAllChange(bool)` | `{ checked }` | The title's box was pressed: `true` to check every row of `data`, `false` to uncheck them |

The title's box is checked while every row of `data` is, and indeterminate while some are (shadcn's
`getIsAllPageRowsSelected` / `getIsSomePageRowsSelected`: with rows served page by page, the rows the app holds).

### Parts

The parts are `nsh/base`'s (`DataTablePart`), styled through `classes` / `styles`; the library gives each its look:

| Part | The library's look | shadcn |
|---|---|---|
| `root` | Edge `border-width` × `data-table-border`, corners `data-table-radius`, clipped; no fill | The guide's `overflow-hidden rounded-md border` |
| `header` | Bottom edge `border-width` × `table-border` | `[&_tr]:border-b` |
| `headerCell` | Padding `table-cell-padding-x` at the start; text `table-head-foreground`, `font-weight-medium` | `TableHead`: `px-2 font-medium text-foreground` |
| `headerContent` | Gap `spacing` between the title and the sort button | The guide's `ArrowUpDown` button after the title |
| `headerCellSorted` | As `headerCell` | — |
| `headerCellSelected` | Fill `table-row-selected` | — |
| `headerCellDragging` | Fill `table-row-hover`, at 80% opacity | — |
| `groupCell` | As `headerCell`, centered, bottom edge `table-border` | — |
| `sortIndicator` | Text `data-table-sort-foreground`, corners `radius-sm`, hover fill `table-row-hover` | The guide's ghost button |
| `sortAscending` / `sortDescending` | `selected:` text `table-head-foreground` (the direction in use); the other at `data-table-sort-foreground` | — |
| `resizeHandle` | Transparent; `data-table-resize-handle-hover` under the pointer | — (shadcn's guide has no resizing) |
| `moveIndicator` | Fill `data-table-move-indicator` | — |
| `row` | Bottom edge `border-width` × `table-border`; colors transition over `duration` | `TableRow`: `border-b transition-colors` |
| `rowOdd` (`stripe`) | Fill `data-table-row-odd` | — |
| `rowHover` | Fill `table-row-hover` | `hover:bg-muted/50` |
| `rowSelected` | Fill `table-row-selected` | `data-[state=selected]:bg-muted` |
| `rowActive` (a context menu's row) | Fill `table-row-hover` | `has-aria-expanded:bg-muted/50` |
| `cell` | Padding `table-cell-padding-x` at the sides; text the table's (`foreground`) | `TableCell`: `p-2` |
| `cellSelected` | Fill `table-row-selected`; inset edge `border-width` × `data-table-selection-outline` | — |
| `cellActive` | Fill `table-row-hover` | — |
| `cellColumnSelected` | Fill `table-row-hover` | — |
| `selectionFill` | Fill `data-table-selection-fill` | — |
| `selectionOutline` | Edge `border-width` × `data-table-selection-outline` | — |
| `rowHeader` | Text `muted-foreground` at `font-size-xs` (12), centered; end edge `table-border` | — |
| `rowHeaderHighlighted` | Fill `table-row-selected`, text `table-head-foreground` | — |
| `rowHeaderCorner` | End and bottom edges `table-border` | — |
| `fixedEdge` | Fill `data-table-fixed-edge` (a line of `border-width`) | — |
| `empty` | Centered, `spacing × 24` (96) tall, text `table-caption-foreground` at `text-sm` | The guide's `h-24 text-center` "No results." cell |

The empty slot's content is the app's ("No results."), as `nsh/base`'s; `loadingView` too (a `Spinner`, rows of
`Skeleton`).

## Variants and sizes

No variants. `size` as [Table](table.md#sizes): body rows 24 / 28 / 32 / 36 (touch 32 / 36 / 40 / 48), titles
`spacing` taller, type 12 / 13 / 14 / 14. `nsh/base` takes one row height, so every body row has the same height
(as virtualization needs); a cell that needs more room needs a larger `size` or `rowHeight`.

The checkbox column is `data-table-checkbox-width` wide (`checkbox-size-<size>` + `table-cell-padding-x` × 2, 32 at
`md`), its boxes centered (shadcn's `[&:has([role=checkbox])]:pr-0` keeps the box near the next column; here the
column's width does).

## States

| State | Look |
|---|---|
| Row under the pointer | `rowHover`: `table-row-hover` |
| Row selected (the table's selection, or checked in the checkbox column) | `table-row-selected` |
| Row checked and under the pointer | `table-row-selected` |
| Cell selected, range selected | `cellSelected`, `selectionFill`, `selectionOutline` |
| Column selected | Its title `headerCellSelected`, its cells `cellColumnSelected` |
| Sorted column | Its sort button's direction in `table-head-foreground` |
| Column being moved | `headerCellDragging`, the `moveIndicator` line where it will land |
| Loading | `nsh/base`'s: the app's `loadingView` over the rows |
| No rows | The app's `empty` slot in the `empty` part |
| Keyboard focus | No ring (the table is one stop, as the desktops' tables): the selected row or cell shows where the keyboard is |

Rows' fills change at once while scrolling (reused rows must not fade); hover and selection changes transition over
`duration`, none with reduced motion. The cursor is the arrow; `nsh/base` gives the resize cursor on a resize band.

## Pointer and touch

| | Desktops and desktop browsers | Phones and tablets |
|---|---|---|
| Row heights | 24 / 28 / 32 / 36 | 32 / 36 / 40 / 48 |
| Hover | `rowHover` | None |
| Scroll bars | The platform's | The platform's indicators |
| Resizing, moving columns | Drags of the title | `nsh/base`'s; usually off on phones (`resizableColumns={false}`) |
| Context menu | A secondary press | A long press |
| Checkbox column | The boxes at their pointer size | Touch size; the row is the target (a press on the row selects it, the box toggles only under the finger) |

## Accessibility

`nsh/base`'s: a `grid` named by `accessibilityLabel`, its titles `columnheader`s with their sort, its cells with their
selected state; the keyboard as in Built on. The checkbox column's boxes are `checkbox`es named "Select row" (with the
row's number for screen readers: "Select row 3") and "Select all" (the library's words, `labels`); they are not Tab
stops (the table is one stop; NSH gaps 2).

## Tokens

The rows, titles and cells read `Table`'s tokens ([Table](table.md#tokens)): `table-border`, `table-row-hover`,
`table-row-selected`, `table-head-foreground`, `table-cell-padding-x`, `table-row-height-<size>`,
`table-head-height-<size>`, `table-caption-foreground`. Its own are what only a data table has. Generated
(`<!-- tokens:data-table -->`).

<!-- tokens:data-table -->
| Token | Type | Derived default | Value |
|---|---|---|---|
| `data-table-border` | color | `table-border` |  |
| `data-table-radius` | length | `radius-md` | 8 |
| `data-table-row-odd` | color | `muted × 30%` |  |
| `data-table-sort-foreground` | color | `muted-foreground` |  |
| `data-table-resize-handle-hover` | color | `border` |  |
| `data-table-move-indicator` | color | `ring` |  |
| `data-table-fixed-edge` | color | `table-border` |  |
| `data-table-selection-fill` | color | `ring × 10%` |  |
| `data-table-selection-outline` | color | `ring` |  |
| `data-table-checkbox-width` | length | `checkbox-size-md + table-cell-padding-x × 2` | 32 (touch 36) |
<!-- /tokens -->

- `data-table-row-odd` is `nsh/base`'s "faint tint" for `stripe`; shadcn has no stripes.
- `data-table-checkbox-width` is read at the table's `size` (the `md` default above; `checkbox-size-<size>` at
  others).
- Other tokens it reads: `border-width`, `radius-sm`, `font-size-*`, `font-weight-medium`, `spacing`, `duration`, and
  Checkbox's tokens for the boxes.

## Examples

TSX:

```tsx
import { signal, Text } from "nsh";
import { Badge, DataTable, type DataTableColumn, type DataTableSort } from "nsh-longbridge";

type Payment = { id: string; status: string; email: string; amount: number };

const columns: DataTableColumn[] = [
  { key: "status", title: "Status", width: 120 },
  { key: "email", title: "Email", sortable: true, grow: 1 },
  { key: "amount", title: "Amount", align: "end", sortable: true, width: 120 },
];

const payments = signal<Payment[]>(loadPayments());
const sort = signal<DataTableSort | null>(null);
const picked = signal(new Set<string>());

<DataTable
  class="h-[480px]"
  accessibilityLabel="Payments"
  columns={columns}
  data={payments()}
  sort={sort()}
  onSortChange={({ sort: next }) => {
    sort.set(next);
    payments.set(sortPayments(payments(), next));
  }}
  checkboxColumn
  checkedRows={payments().map((p) => picked().has(p.id))}
  onCheckedChange={({ row, checked }) => {
    const id = payments()[row].id;
    const next = new Set(picked());
    checked ? next.add(id) : next.delete(id);
    picked.set(next);
  }}
  onCheckedAllChange={({ checked }) => picked.set(new Set(checked ? payments().map((p) => p.id) : []))}
  renderCell={(cell) =>
    cell.key === "status" ? <Badge variant="secondary">{cell.text}</Badge> : <Text class="truncate">{cell.text}</Text>
  }
  empty={<Text>No results.</Text>}
/>
<Text class="text-sm text-muted-foreground">{`${picked().size} of ${payments().length} row(s) selected.`}</Text>
```

With `renderCell` every cell is the app's (`nsh/base`'s rule), so it returns the text of the columns it does not draw
itself. With NSH 0.7.0, write a string child inside
`<Text>` for now (see [button.md, NSH gaps](button.md#nsh-gaps), G0).

A million rows served page by page, as `nsh/base`'s: `rowCount={1_000_000}`, `dataStart={start()}`, `data={page()}`,
and `onVisibleRowsChange={({ range }) => fetchAround(range)}`; `checkedRows` then covers the rows of `data`.

Rust:

```rust
use nsh_core::prelude::*;
use nsh_longbridge::{DataTableAlign, DataTableColumn, DataTableRows, Size, data_table};

// `DataTableColumn` is `nsh/base`'s (`#[non_exhaustive]`): made from its default, its fields set.
fn column(key: &str, title: &str, sortable: bool) -> DataTableColumn {
    let mut c = DataTableColumn::default();
    c.key = key.into();
    c.title = Some(title.into());
    c.sortable = sortable;
    c
}

let checked = signal(vec![false; 3]);
let mut amount = column("amount", "Amount", true);
amount.align = DataTableAlign::End;

data_table()
    .size(Size::Sm)
    .accessibility_label("Payments")
    .columns(vec![column("email", "Email", true), amount])
    .data(
        DataTableRows::new(3)
            .texts("email", ["ken99@example.com", "abe45@example.com", "silas22@example.com"])
            .numbers("amount", [316.0, 242.0, 874.0]),
    )
    .checkbox_column(true)
    .checked_rows(move || checked.get())
    .on_checked_change(move |row: &u32, value: &bool| checked.update(|c| c[*row as usize] = *value))
```

`DataTableColumn`, `DataTableRows`, `DataTableSort` and the other value types are `nsh/base`'s, re-exported by this
library, whose TypeScript declarations declare them too (as `Select`'s `SelectItem`).

## NSH gaps

1. **The look must be classes, not inline styles** (as `Select`'s rows): `nsh/base`'s parts take the app's classes
   and styles, styles over classes. Were the library's look `styles`, an app's `classes` could not change it.
   Workaround: classes generated from the resolved tokens in a binding (arbitrary values, `dark:` for the other
   appearance), before the app's. A part style that follows tokens (or token-named classes that see the library's
   derived tokens) would let the library write its look as tokens.
2. **Controls inside cells and the keyboard.** The table is one Tab stop and binds Tab while it has the keyboard; a
   `Checkbox` drawn by `renderCell` is pressable, so on macOS and the web it would also be a Tab stop. The library's
   boxes are made not focusable; checking from the keyboard then needs Space on the selected row, which
   `nsh/base`'s `DataTable` does not bind and the library cannot bind without knowing that the table has the
   keyboard (no focus event on the table). To decide in review (Decisions 3); a `focused` state or `onFocusChange`
   on `nsh/base`'s table would allow it.
3. **The resize band's hover.** The handle's line is a part inside a wider band; whether the band's hover reaches
   the line's `hover:` is to check at implementation. If it does not, the line stays transparent and the resize
   cursor alone shows the handle.
4. **One row height** for the whole table (`rowHeight`): virtualization needs it. Rows that need more room take a
   larger `size`; rows of their own heights are a `Table`'s.
5. **String children** follow Button's G0 in TSX (`empty`, `loadingView`, what `renderCell` returns).
6. **`renderCell` is the whole table's.** Once it is set, every cell is drawn by it, not as the table's native text.
   The checkbox column needs one, so with `checkboxColumn` and no `renderCell` of the app's the library draws the
   other cells' text itself (Rust, a text per cell in view, aligned as the column; `positiveClass` / `negativeClass`
   still apply on the cell). That costs a view per cell in view more than the table's own text. A renderer per
   column (`DataTableColumn.render`) in `nsh/base` would keep the other columns native.

## Decisions

Proposed; for review.

1. **`DataTable` is a component of its own**, not a mode of `Table`: its API is data (`columns`, `data`), `Table`'s is
   children, and the two meet only in their tokens.
2. **The checkbox column is built in** (`checkboxColumn`, `checkedRows`, `onCheckedChange`, `onCheckedAllChange`),
   from shadcn's guide (its `select` column with `Checkbox`es and "x of y row(s) selected"), because writing it with
   `renderCell` needs the library's look for checked rows, which an app cannot write as a class. It is separate from
   `nsh/base`'s selection (one row, column, cell or range: the keyboard's cursor), which the app may turn off
   (`rowSelectable={false}`) when only boxes select. `checkedRows` is by place in `data`, as `rowClasses`, so the
   app keeps checked rows by its own ids across sorting and paging (gpui-kit's Design Guides: "preserve selection by
   domain identity").
3. **Space on the selected row toggles its box**: wanted (the desktops' and ARIA grids' habit), blocked by NSH gaps 2.
   Until then the boxes take presses only. To decide: ask `nsh/base` for the table's focus state, or leave it.
4. **A bordered box by default** (`data-table-border`, `data-table-radius`), as every example in shadcn's guide wraps
   its table; a plain `Table` has none. `class="border-0 rounded-none"` removes it.
5. **No filtering, column visibility or pagination inside**: shadcn's guide builds them around the table with an
   `Input`, a `DropdownMenu` of `DropdownMenuCheckboxItem`s and buttons; here too, changing `data` and `columns`.
