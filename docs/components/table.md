# Table

Status: implemented (P3).

A table of rows the app writes as children: the library's look on `nsh/base`'s `Table` parts.
This spec owns the **table look** (rows, header cells, cells, separators, hover and selected rows, the footer, the
caption) and its `table-*` tokens, which [DataTable](data-table.md) reads too. It also settles the line between the
two (see "When not to use").

## Usage

- A few to a few hundred rows of mixed content the app lays out itself: invoices, members and their roles, a
  settings matrix, a comparison, a summary with totals, a row of boxes for a selection the app keeps.
- Search terms: table, simple table, static table, rows and columns, list with columns, invoice table.

## When not to use

| Need | Use instead |
|---|---|
| Thousands of rows or columns, rows from a server page by page | [DataTable](data-table.md): only the rows in view are made |
| Sorting by a column's title, resizing, moving or fixing columns, a keyboard cursor over rows or cells, copying cells | [DataTable](data-table.md) |
| A list of items of different shapes (a feed, messages, notifications) | A list (`nsh/base`'s `VirtualList`), not a table |
| Items that hold items | `nsh/base`'s `Tree` |
| Laying out a form or a page in columns | Flex or grid classes; a table is for data compared across the same fields |

### Table or DataTable

| | `Table` | `DataTable` |
|---|---|---|
| Content | Elements the app writes: any view in any cell | Data (`columns`, `data`); a cell is its text, or what `renderCell` draws |
| Rows made | All of them | Only those in view, reused while scrolling |
| Good for | Up to a few hundred rows | Any number, a million included |
| Scrolling | The page's (the app's `Scroll` around it) | Its own, both ways |
| Header while scrolling | `sticky` on `TableHeader` | Always stays |
| Column widths | Each cell's classes (`w-24 flex-none`, `flex-1`) | `width`, `grow`, `minWidth`; resize, move, fit, fixed at a side |
| Sorting | The app's own button in a `TableHead` | A sort button in sortable titles; the app sorts its data |
| Selection | The app's: a `Checkbox` per row, `TableRow selected` | The table's selection (a row, a column, a cell) and a checkbox column |
| Keyboard | Tab through the controls in the cells | One Tab stop; arrows, Home, End, Page Up / Down move the selection |

The rule: static rows the app writes → `Table`; anything the user does to the rows or columns as a whole, or more rows
than are worth making at once → `DataTable`. Both share one look: a theme that changes `table-border` or
`table-row-hover` changes both.

## Built on

`nsh/base`'s `Table` parts (`crates/nsh-base/src/table.rs`): `Table`, `TableHeader`, `TableBody`, `TableFooter`,
`TableRow`, `TableHead`, `TableCell`, `TableCaption`. They give the roles screen readers know and nothing else:

| Provided by `nsh/base` (not rewritten here) | |
|---|---|
| Roles | `table` on `Table` (named by `accessibilityLabel`), `rowgroup` on the header, body and footer, `row`, `columnheader`, `cell` |
| Layout | `TableRow` is a row (`flex-row`); the rest are columns |
| Terminal | The same views; this library's look is drawn there with the terminal's colors as any view |

This library adds the look (tokens), `size`, the header's `sticky` form, the row's `selected` form and hover, the
cells' padding and one-line text, and the caption's place.

### Implementation

- Each part is a `#[component]` whose root is the `nsh/base` part. Its look is **classes generated from the resolved
  tokens** (`foundation::class`), before the app's `class`: an inline style would win over every class, and the app's
  column classes (`w-24 flex-none`, `items-end`, `pr-0`, `border-b-0`) must win over the library's look.
- `Table` provides its `size` in context (`TableSize`); every part reads it. A `TableHead` / `TableCell` outside a
  `Table` takes `md`.
- `TableHeader` provides "in the header" in context; `TableRow` reads it to draw no hover there (Decisions 3).
- Body and footer rows take `last:border-b-0`: NSH's `last:` variant drops the last row's bottom edge and follows rows
  added and removed, so the body does not count its rows. Header rows keep theirs (the line under the titles).
- A `TableCell` / `TableHead` provides a context a library `Checkbox` inside it registers with (as `CardAction` does in
  a `CardHeader`), and drops its end padding while it holds one (NSH has no `:has()`; NSH gaps 2). This is a small
  addition to `Checkbox` (it reports to an enclosing cell).
- `TableRow` provides a context the library's overlays (`DropdownMenu`, `Popover`, `Select`) report their open state
  to through `foundation::trigger` (NSH gaps 3); a small addition to `foundation::trigger` and `Select`.
- `TableRow` is drawn by the library as a row view with the `row` role (what `nsh/base`'s `TableRow` is), because
  `nsh/base`'s part takes no accessibility state and a `selected` row must be announced selected.
- Every child of `TableHead`, `TableCell` and `TableCaption` sits in a text part (conventions, "Styled parts"), so a
  string child takes the cell's text styles. In cells it is one line cut with an ellipsis through
  `whitespace-nowrap overflow-hidden text-ellipsis`, not `truncate`: `truncate` also clamps the text to one line,
  which a later `whitespace-normal` does not lift (NSH gaps 5).
- A cell that holds a library control (a `Checkbox`, which registers with it, or an overlay such as a `DropdownMenu`,
  which reports to it through `foundation::trigger`) places its children as they are, not inside the text part: a
  view inside a text sits on the text's baseline, which sets a box or a button about 2 points above the row's middle.
  A `Skeleton` or a `Progress` registers too, keeping the padding: inside a text it would have no width, so a
  `w-full` bar would not show. The cell keeps that form once it has held one.

## API

### Components

| Component | TSX | Rust | Description |
|---|---|---|---|
| `Table` | `<Table>` | `table()` | The root: a column of its parts; `size` for all of them |
| `TableHeader` | `<TableHeader>` | `table_header()` | The header group: rows of `TableHead`s |
| `TableBody` | `<TableBody>` | `table_body()` | The body group: rows of `TableCell`s |
| `TableFooter` | `<TableFooter>` | `table_footer()` | The footer group: totals, a summary row |
| `TableRow` | `<TableRow>` | `table_row()` | A row: cells side by side |
| `TableHead` | `<TableHead>` | `table_head()` | A column's title cell |
| `TableCell` | `<TableCell>` | `table_cell()` | A cell |
| `TableCaption` | `<TableCaption>` | `table_caption()` | Text about the table, under it |

`nsh/base` exports the same names; an app using both renames one (`import { Table as BaseTable } from "nsh/base"`).

### Props

| Component | TSX | Rust | Type | Default | Description |
|---|---|---|---|---|---|
| `Table` | `size` | `size` | `Size` | `"md"` | Row heights and type of every part; see Variants and sizes |
| `Table` | `accessibilityLabel` | `accessibility_label` | `string` | — | What screen readers call the table. A `TableCaption` is shown, not the name: set both when there is a caption (`nsh/base`'s rule) |
| `TableHeader` | `sticky` | `sticky` | `boolean` | `false` | Stays at the top of the nearest scroll view while the body scrolls under it, filled with `table-sticky-header` so the rows do not show through |
| `TableRow` | `selected` | `selected` | `boolean` | `false` | The row is selected: the selected fill; screen readers hear it selected |
| `TableHead`, `TableCell`, `TableCaption` | `labelClass` / `labelStyle` | `label_class` / `label_style` | | — | The `label` part: the text around each child |
| every part | `class` / `style` | `class` / `style` | | — | The part's root, merged last |
| every part | `children` | `.child(..)` | string, elements | — | Its content |

No events: a plain table has no behavior of its own. A pressable row is `DataTable`'s (`onRowPress`), or a `Button` in
a cell.

### Parts

| Part | What | Customize with |
|---|---|---|
| `Table` | A column, `w-full`, text `foreground` at `font-size-<size>`, tabular figures | `class` / `style` |
| `TableHeader` | A column; with `sticky`, `position: sticky; top: 0` and its fill | `class` / `style` |
| `TableBody`, `TableFooter` | A column | `class` / `style` |
| `TableRow` | A row: min height, bottom edge, hover and selected fills | `class` / `style` |
| `TableHead`, `TableCell` | A cell: padding, vertically centered, one line | `class` / `style` |
| `TableCaption` | Muted text under the table | `class` / `style` |
| `label` | In `TableHead`, `TableCell` and `TableCaption`: a text around each child, one line cut with an ellipsis in cells (`whitespace-nowrap overflow-hidden text-ellipsis`), wrapping in the caption; none in a cell holding a library control (Implementation) | `labelClass` / `labelStyle` |

Column widths are the cells' (NSH gaps 1): by default every `TableHead` and `TableCell` takes an equal share of the
row (`flex: 1 1 0`, `min-width: 0`); a column of its own width gives each of its cells the same classes:
`class="w-24 flex-none"` (`flex-none` is needed: a share's basis wins over a width). Alignment is the cell's too:
`class="items-end"` puts a number column's text at the end.

#### Column widths, worked

An HTML table sizes its columns to their content. Here each column is a choice the app writes once, as a
class string, and gives to every cell of that column, the header's included:

```tsx
// One class string per column: a fixed width, a share, an end-aligned share.
const col = {
  invoice: "w-[100px] flex-none", // fixed: 100 points, never grows or shrinks
  status: "flex-1",                // a share (the default; written for clarity)
  method: "flex-[2]",              // twice the share of `status`
  amount: "w-[96px] flex-none items-end", // fixed and its text at the end
};

<Table accessibilityLabel="Invoices">
  <TableHeader>
    <TableRow>
      <TableHead class={col.invoice}>Invoice</TableHead>
      <TableHead class={col.status}>Status</TableHead>
      <TableHead class={col.method}>Method</TableHead>
      <TableHead class={col.amount}>Amount</TableHead>
    </TableRow>
  </TableHeader>
  <TableBody>
    {invoices.map((i) => (
      <TableRow>
        <TableCell class={col.invoice}>{i.id}</TableCell>
        <TableCell class={col.status}>{i.status}</TableCell>
        <TableCell class={col.method}>{i.method}</TableCell>
        <TableCell class={col.amount}>{i.amount}</TableCell>
      </TableRow>
    ))}
  </TableBody>
</Table>
```

- `flex-none` with every fixed width: a share's basis (`flex: 1 1 0`) wins over a width.
- A column as wide as its content does not exist (NSH gaps 1): pick a width that holds the widest value, or let the
  text cut with an ellipsis in a share.
- A footer cell spanning columns takes the sum: `flex-[3]` over three shares, or the fixed widths added up
  (`w-[196px] flex-none`).
- A Rust app keeps the same strings in constants and passes them with `.app_class(..)`.

#### Wide tables

No scroll container is built in (Decisions 1). A table wider than its place gets a min width and sits in the app's
horizontal `Scroll`; its fixed columns keep their widths, its shares divide the min width:

```tsx
<Scroll horizontal class="w-full">
  <Table class="min-w-[720px]" accessibilityLabel="Orders">
    …
  </Table>
</Scroll>
```

A table that scrolls both ways (a long and wide one) nests the horizontal `Scroll` inside the page's vertical one,
with `<TableHeader sticky>` keeping its titles; past a few hundred rows, or with columns that must stay at a side,
it is a [DataTable](data-table.md).

## Variants and sizes

No variants.

### Look

| Part | In classes | This library |
|---|---|---|
| `Table` | `relative w-full overflow-x-auto` container; `w-full caption-bottom text-sm` | A column, `w-full`; text `foreground`, `font-size-<size>` (14 at `md`), tabular figures (`tabular-nums`, so a column of numbers lines up); no scroll container (Decisions 1) |
| `TableHeader` | `[&_tr]:border-b` | Its rows keep their bottom edge (the line under the titles); no fill unless `sticky` |
| `TableBody` | `[&_tr:last-child]:border-0` | The last row draws no bottom edge |
| `TableFooter` | `border-t bg-muted/50 font-medium [&>tr]:last:border-b-0` | Top edge `border-width` × `table-border`, fill `table-footer`, `font-weight-medium`; its last row no bottom edge |
| `TableRow` | `border-b transition-colors hover:bg-muted/50 has-aria-expanded:bg-muted/50 data-[state=selected]:bg-muted` | Bottom edge `border-width` × `table-border`; min height `table-row-height-<size>` (header rows `table-head-height-<size>`); hover fill `table-row-hover` (pointer only, not in the header); `selected` fill `table-row-selected`; colors transition over `duration` |
| `TableHead` | `h-10 px-2 text-left align-middle font-medium whitespace-nowrap text-foreground [&:has([role=checkbox])]:pr-0` | Padding `table-cell-padding-x` at the sides, `table-cell-padding-y` above and below; centered vertically, at the start; text `table-head-foreground`, `font-weight-medium`, one line cut with an ellipsis; no end padding while it holds a `Checkbox` |
| `TableCell` | `p-2 align-middle whitespace-nowrap [&:has([role=checkbox])]:pr-0` | As `TableHead`, without the weight and color (the table's) |
| `TableCaption` | `mt-4 text-sm text-muted-foreground` | `table-caption-gap` (16) above, 14 / 20, `table-caption-foreground`, centered (HTML's caption is centered) |

A web table nudges a checkbox down (`[&>[role=checkbox]]:translate-y-[2px]`) to fix HTML cells' baseline; a cell here
centers its content, so it is not needed.

### Sizes

| `size` | Body row (pointer / touch) | Header row (pointer / touch) | Type |
|---|---|---|---|
| `xs` | 24 / 32 | 28 / 36 | 12 |
| `sm` | 28 / 36 | 32 / 40 | 13 |
| `md` | 32 / 40 | 36 / 44 | 14 |
| `lg` | 36 / 48 | 40 / 52 | 14 |

- A body row's height is the control height of its size, so a `Button`, `Input`, `Select` or `Checkbox` of the same
  `size` fits a row; a header row is `spacing` taller. `lg` is the classes above (`p-2` around a 20-point line, 36;
  an `h-10` header, 40); `md`, one step down, is the library's desktop density, as every control here (gpui-kit's
  tables use 26 / 30 / 32 / 40).
- Heights are minimums: a cell with an avatar and two lines makes its row taller, keeping `table-cell-padding-y` above
  and below.
- Type is `font-size-<size>`, its line height foundation's `line_height`, its figures tabular, so a column of numbers
  at the end (`items-end`) lines up digit under digit; an icon in a cell without a size takes
  `icon-size-<size>` (the `Size` context, as in a `Button`).

### Sticky header

`<TableHeader sticky>` inside the app's scroll view (`Scroll`, or the page's) is `position: sticky; top: 0` (NSH
pins it natively on every scroll); it is filled with `table-sticky-header` (`background`) and draws above the rows. In
a `Card` or a surface of another color, set `table-sticky-header` in a scope or the header's `class`. For a long
table, prefer [DataTable](data-table.md), which keeps its header by construction.

## States

| State | Look | In classes |
|---|---|---|
| Row at rest | Bottom edge `table-border`, no fill | `border-b` |
| Row hover (pointer, body and footer rows) | Fill `table-row-hover` (`muted` at 50%) | `hover:bg-muted/50` |
| Row `selected` | Fill `table-row-selected` (`muted`); with hover still `table-row-selected` | `data-[state=selected]:bg-muted` |
| Row holding an open menu or popover | As hover, while the menu is open (Decisions 4) | `has-aria-expanded:bg-muted/50` |
| Last row of the body or footer | No bottom edge | `[&_tr:last-child]:border-0` |
| Sticky header | Fill `table-sticky-header` | — |

No focus, press or disabled look: the table is not interactive; the controls in its cells have theirs. Fills
transition over `duration` (`transition-colors`), none with reduced motion. The cursor is the arrow.

## Pointer and touch

| | Desktops and desktop browsers | Phones and tablets |
|---|---|---|
| Row heights | `table-row-height-<size>`: 24 / 28 / 32 / 36 | 32 / 36 / 40 / 48, as the controls in them |
| Hover | The hover fill | None |
| Controls in cells | Their own pointer sizes | Their own touch sizes and hit slop (a `Checkbox` alone gets 44 points, which may reach the next row at `xs` / `sm`: use `md` rows on touch screens) |
| Wide tables | A min width on the `Table` (`class="min-w-[640px]"`) in a horizontal `Scroll` | The same; phones usually show fewer columns (`hidden md:flex` on a column's cells) |

## Accessibility

- Roles from `nsh/base`: `table` named by `accessibilityLabel`, `rowgroup`s, `row`s, `columnheader`s, `cell`s.
- A `selected` row is announced selected (`accessibilityState.selected`).
- The caption is read as text after the table; it does not name it (set `accessibilityLabel` too).
- A `Checkbox` alone in a cell needs its `accessibilityLabel` ("Select INV-002"): it has no label beside it.

## Tokens

Generated from the code's declarations (`NSH_LONGBRIDGE_WRITE=1 cargo test --test tokens`); a test keeps it current.
Values are those of `nsh/base`'s default theme. `Table` owns these; [DataTable](data-table.md) reads them for its rows,
titles and cells, and declares only what it alone has.

<!-- tokens:table -->
| Token | Type | Derived default | Value |
|---|---|---|---|
| `table-row-height-xs` | length | `pointer_touch(control-height-xs, control-height-touch-xs)` | 24 (touch 32) |
| `table-row-height-sm` | length | `pointer_touch(control-height-sm, control-height-touch-sm)` | 28 (touch 36) |
| `table-row-height-md` | length | `pointer_touch(control-height-md, control-height-touch-md)` | 32 (touch 40) |
| `table-row-height-lg` | length | `pointer_touch(control-height-lg, control-height-touch-lg)` | 36 (touch 48) |
| `table-head-height-xs` | length | `table-row-height-xs + spacing` | 28 (touch 36) |
| `table-head-height-sm` | length | `table-row-height-sm + spacing` | 32 (touch 40) |
| `table-head-height-md` | length | `table-row-height-md + spacing` | 36 (touch 44) |
| `table-head-height-lg` | length | `table-row-height-lg + spacing` | 40 (touch 52) |
| `table-cell-padding-x` | length | `spacing × 2` | 8 |
| `table-cell-padding-y` | length | `spacing` | 4 |
| `table-border` | color | `border` |  |
| `table-head-foreground` | color | `foreground` |  |
| `table-row-hover` | color | `muted × 50%` |  |
| `table-row-selected` | color | `muted` |  |
| `table-footer` | color | `muted × 50%` |  |
| `table-sticky-header` | color | `background` |  |
| `table-caption-foreground` | color | `muted-foreground` |  |
| `table-caption-gap` | length | `spacing × 4` | 16 |
<!-- /tokens -->

- The table's text is `foreground` (the semantic token, as `Card` uses `card-foreground`); a `table-foreground` would
  mean the same.
- `table-footer` is the footer's fill (the bare name: the part is the footer); `table-sticky-header` the header's in
  its `sticky` form (the form where a variant goes, tokens.md).
- An app that changes the touch heights alone writes `table-row-height-<size>-touch`.
- Other tokens it reads: `border-width`, `font-size-*`, `icon-size-*`, `font-weight-medium`, `duration`, `spacing`.

## Examples

TSX:

```tsx
import { Scroll, signal, Text, View } from "nsh";
import { Badge, Checkbox, Table, TableBody, TableCaption, TableCell, TableFooter, TableHead, TableHeader, TableRow } from "nsh-longbridge";

const invoices = [
  { id: "INV001", status: "Paid", method: "Credit Card", amount: "$250.00" },
  { id: "INV002", status: "Pending", method: "PayPal", amount: "$150.00" },
  { id: "INV003", status: "Unpaid", method: "Bank Transfer", amount: "$350.00" },
];

<Table accessibilityLabel="Recent invoices">
  <TableHeader>
    <TableRow>
      <TableHead class="w-[100px] flex-none">Invoice</TableHead>
      <TableHead>Status</TableHead>
      <TableHead>Method</TableHead>
      <TableHead class="items-end">Amount</TableHead>
    </TableRow>
  </TableHeader>
  <TableBody>
    {invoices.map((invoice) => (
      <TableRow>
        <TableCell class="w-[100px] flex-none font-medium">{invoice.id}</TableCell>
        <TableCell>{invoice.status}</TableCell>
        <TableCell>{invoice.method}</TableCell>
        <TableCell class="items-end">{invoice.amount}</TableCell>
      </TableRow>
    ))}
  </TableBody>
  <TableFooter>
    <TableRow>
      <TableCell class="flex-[3]">Total</TableCell>
      <TableCell class="items-end">$750.00</TableCell>
    </TableRow>
  </TableFooter>
  <TableCaption>A list of your recent invoices.</TableCaption>
</Table>

// Rows the user picks: a box per row, "select all" in the header (the Checkbox page's table).
const picked = signal<string[]>([]);
const all = () => picked().length === invoices.length;

<Table size="md" accessibilityLabel="Invoices">
  <TableHeader>
    <TableRow>
      <TableHead class="w-8 flex-none">
        <Checkbox
          accessibilityLabel="Select all"
          checked={all()}
          indeterminate={picked().length > 0 && !all()}
          onChange={({ checked }) => picked.set(checked ? invoices.map((i) => i.id) : [])}
        />
      </TableHead>
      <TableHead>Invoice</TableHead>
      <TableHead class="items-end">Amount</TableHead>
    </TableRow>
  </TableHeader>
  <TableBody>
    {invoices.map((invoice) => (
      <TableRow selected={picked().includes(invoice.id)}>
        <TableCell class="w-8 flex-none">
          <Checkbox
            accessibilityLabel={`Select ${invoice.id}`}
            checked={picked().includes(invoice.id)}
            onChange={({ checked }) =>
              picked.set(checked ? [...picked(), invoice.id] : picked().filter((id) => id !== invoice.id))
            }
          />
        </TableCell>
        <TableCell>
          <View class="flex-row items-center gap-2">
            <Text>{invoice.id}</Text>
            {invoice.status === "Pending" && <Badge variant="secondary">Pending</Badge>}
          </View>
        </TableCell>
        <TableCell class="items-end">{invoice.amount}</TableCell>
      </TableRow>
    ))}
  </TableBody>
</Table>

// A long table in a scroll view: the titles stay.
<Scroll class="h-80">
  <Table accessibilityLabel="Members">
    <TableHeader sticky>…</TableHeader>
    <TableBody>…</TableBody>
  </Table>
</Scroll>
```

With NSH 0.7.0, write a string child inside `<Text>` for now (see [button.md, NSH gaps](button.md#nsh-gaps), G0).

Rust:

```rust
use nsh_core::prelude::*;
use nsh_longbridge::{Size, table, table_body, table_caption, table_cell, table_head, table_header, table_row};

table()
    .size(Size::Sm)
    .accessibility_label("Recent invoices")
    .child(table_header().child(
        table_row()
            .child(table_head().app_class(Some("w-[100px] flex-none".into())).child("Invoice"))
            .child(table_head().child("Status"))
            .child(table_head().app_class(Some("items-end".into())).child("Amount")),
    ))
    .child(table_body().child(
        table_row()
            .selected(true)
            .child(table_cell().app_class(Some("w-[100px] flex-none".into())).child("INV001"))
            .child(table_cell().child("Paid"))
            .child(table_cell().app_class(Some("items-end".into())).child("$250.00")),
    ))
    .child(table_caption().child("A list of your recent invoices."))
```

### The gallery

The Checkbox page's "A box alone" table, the Switch page's "A track alone" table and the Skeleton page's loading table
are rows of views with `h-10` / `h-12`, `border-b` and `px-4` of their own. They become `Table` parts: `TableHeader`
with `TableHead`s for the titles, `TableRow selected` for the Checkbox page's picked rows (its `bg-muted/50` becomes
the selected fill), a `TableCell` around each `Checkbox`, `Switch` or `Skeleton`, and `size="lg"` where they want
their taller rows.

## NSH gaps

1. **No table layout.** NSH lays out flex and grid boxes; a row does not know the widths of the cells above it, so
   columns cannot size to their widest cell as HTML tables do (there the columns follow their content and the
   container scrolls). Workaround: every cell takes an equal share, and a column's cells share the app's width
   classes (`w-24 flex-none`). A grid with subgrid rows, or a `columns` template on `Table` that its rows read, would
   allow content-sized columns. A `columns` template on `Table` is not added in P3 (Decisions 2).
2. **No `:has()` selector.** A cell cannot drop its end padding with `:has([role=checkbox])`. Workaround: a library
   `Checkbox` registers with its cell (as `CardAction` with `CardHeader`). An app's own checkbox-like control does not
   register: its cell takes `class="pr-0"`. The last row's edge needs no workaround: NSH's `last:` variant drops it.
3. ★ **No `has-aria-expanded:`.** A row that holds an open menu trigger is lit. Workaround: `TableRow` provides a
   context the library's overlays (`DropdownMenu`, `Popover`, `Select` through `foundation::trigger`) report their
   open state to, and the row takes the hover fill while one is open. An app's own popup is not covered.
   Headless test: a `DropdownMenu` in a cell lights its row while open, from TSX and Rust. **Verified**: it passes.
   (The fallback was a row that is not lit.)
4. **String children** follow Button's G0 in TSX.
5. ★ **A later class lifting a one-line label.** NSH's `truncate` also clamps the text to one line, which a later
   `whitespace-normal` does not lift. **Verified** with the label's one line written as `whitespace-nowrap
   overflow-hidden text-ellipsis`: `labelClass="whitespace-normal"` after it wraps (headless test). The fallback (a
   bound style the app's `labelStyle` overrides) is not needed.
6. ★ **The last row's edge and the checkbox cell's padding** (2): headless tests that the body's last row draws no
   edge after rows are added and removed (`last:`), and that a cell holding a library `Checkbox` drops its end padding
   (context). **Verified**: both pass. (The fallback was the app's `class="border-b-0"` and `class="pr-0"`.)

## Decisions

Settled in review.

1. **No scroll container.** A web table usually sits in an `overflow-x-auto` wrapper. A scroll view in NSH is a native one that
   takes the wheel and nests with the page's; a table that fits needs none. A wide table goes in the app's
   horizontal `Scroll` with a min width on `Table` ("Wide tables").
2. **Column widths by classes, no `columns` prop** (NSH gaps 1): per-cell width classes ("Column widths, worked"),
   and `DataTable` for real column management. A `columns` template is revisited when an app needs it.
3. **Header rows have no hover** (a web table's often do, as a side effect of `tr:hover`): a title row lit under the pointer
   reads as pressable.
4. **A row holding an open popup stays lit** (`has-aria-expanded:` on the web), through the overlays' open state
   (NSH gaps 3).
5. **`sticky` is a prop on `TableHeader`**, not only a class: it also sets the fill the header needs over scrolling
   rows, from a token, which a class cannot follow in a `ThemeScope`.
6. **One line per cell by default, cut with an ellipsis** (a web table's `whitespace-nowrap` scrolls instead):
   columns are shares of the width here. A cell that wraps sets `labelClass="whitespace-normal"` (NSH gaps 5).
