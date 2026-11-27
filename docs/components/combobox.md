# Combobox

Status: spec (P3), not implemented.

A text field that filters a list as the user types and chooses one value, or several shown as chips. The field is
drawn by foundation's **field look** (the `input-*` tokens, as [Input](input.md)); the list by the **menu look** (the
`menu-*` tokens, owned by [DropdownMenu](dropdown-menu.md)) on the popup surface (owned by [Popover](popover.md)),
exactly as [Select](select.md)'s list. Reference: shadcn/ui new-york v4 `combobox.tsx` (Base UI Combobox).

## Usage

- A choice from a long list the user would rather type into than scroll: a country, a framework, a time zone, an
  assignee, a stock symbol of the app's own list; with `multiple`, several of them (labels, recipients, tags from a
  known set).
- Search terms: combobox, autocomplete, typeahead, searchable select, filter select, multi-select, tag input, chips
  input.

## When not to use

| Need | Use instead |
|---|---|
| A choice from a short fixed list, no typing | [Select](select.md) |
| A button that opens a list with a search field inside it (shadcn's "popup" combobox) | `Select` with a search field: not exposed yet (Decisions 4) |
| Free text with suggestions the user need not take | An [Input](input.md); a suggestion list is not offered yet (Decisions 5) |
| Commands, searched | Command (later) |
| Two to five options, all worth seeing | [RadioGroup](radio-group.md), [ToggleGroup](toggle-group.md) |
| A date | DatePicker (P3) |

## Built on

`nsh/base`'s `Combobox` (`crates/nsh-base/src/combobox.rs`) and its list (`listbox.rs`, shared with `Select`). This
library adds the field it draws around the input, the chevron and clear buttons, the chips, the look of the list and
`size` / `invalid`; all behavior is `nsh/base`'s:

| Provided by `nsh/base` (not rewritten here) | |
|---|---|
| Value | `value: string[]` (controlled) / `defaultValue`; without `multiple` a choice replaces it and closes the list, with `multiple` it toggles the item and the list stays open; `onChange({ value })` on each change, `onConfirm({ value })` once as the list closes; Cmd+Return (Ctrl+Return off Apple's) confirms and closes |
| Query | Typing filters the items by title (a case-insensitive substring, in order; a section without matches goes); each opening starts from `defaultQuery`, else empty; `query` / `onQueryChange({ query })` when the app filters itself |
| The input | One value: shows the chosen title until the user types, selected when the input takes the focus (typing replaces it). Several: the query only |
| Opening | Typing, Down or Up (Down on the first enabled item when none is chosen, Up on the last), a press on the input or on the chevron button opens it; desktops: Tab into the input does not; touch screens: the tap that focuses the input opens it once |
| List keys | Up / Down (around the ends), Home / End, Page Up / Down, Return chooses, Escape, back and a press outside close (`"cancel"`, `"pressOutside"`), Tab or Shift+Tab close (`"focus"`) and move on; a new query highlights its first enabled match |
| Focus | The input keeps the keyboard focus while the list is open (the highlighted row is its active descendant); the clear and chevron buttons are no Tab stops |
| Presentation | Desktops: a popup below the field (`placement`, `align`), as wide as it, never taller than the room on its side. Touch screens: a sheet at the bottom over a backdrop |
| Rows | Virtualized: thousands of items make only the rows in view; sections from the items' `group`; `focus:` (highlighted), `selected:`, `disabled:` on each row |
| Clear button | With `cleanable`, while a value is set: a press clears it (`onChange({ value: [] })`) and gives the focus back to the input |
| Control | `open` / `defaultOpen` / `onOpenChange({ open, reason })`, `onOpen`, `onClose`, `onCancel` |
| Accessibility | The input a `combobox` (`expanded`, `hasPopup`, its value the chosen titles); the list a `listbox`, rows `option` with `selected` and `disabled`, headers `header`; the buttons hidden from screen readers |
| Terminal | A one-row field with ` ×` and ` ▾`, rows of titles with `✓`; this library's look does not apply there |

### Differs from shadcn's structure

shadcn composes the combobox from parts: `Combobox > ComboboxInput` (an `InputGroup` with the trigger and clear
buttons) and `ComboboxContent > ComboboxEmpty + ComboboxList > ComboboxGroup > ComboboxLabel + ComboboxItem`,
`ComboboxSeparator`, and `ComboboxChips > ComboboxChip + ComboboxChipsInput` for several values. `nsh/base`'s
`Combobox` takes its options as **data** (`items`), for the reasons Select gives (virtualized rows; the filter,
highlight and typeahead run over the titles; a component cannot read its children's props). So, as Select:

| shadcn | This library |
|---|---|
| `<Combobox items>` + `ComboboxList` render function + `ComboboxItem` | `items: SelectItem[]` (`{ value, title, disabled?, group? }`); a row's content by `renderItem` |
| `ComboboxGroup` + `ComboboxLabel` | The items' `group`: a header with the group's title |
| `ComboboxInput placeholder showTrigger showClear` | `Combobox`'s `placeholder`, `showTrigger` (default `true`), `cleanable` (`nsh/base`'s name) |
| `ComboboxContent side align` | `placement`, `align`, `contentClass` / `contentStyle` |
| `ComboboxEmpty` | `empty` (a slot; default: "No items found.") |
| `ComboboxChips` / `ComboboxChip` / `ComboboxChipsInput` | `multiple`: the library draws a chip per value before the input |
| `ComboboxSeparator` | None (Select's NSH gaps 3) |
| `onValueChange` / `onInputValueChange` / `onOpenChange` | `onChange({ value })` / `onQueryChange({ query })` / `onOpenChange({ open, reason })` |
| Value: an item, or a list with `multiple` | Always a list of values (`nsh/base`'s): one value is `[value]` |

### Implementation

- **Structure**: a `#[component] fn Combobox` whose root is `nsh_base::combobox()`, `items`, `multiple`, `value`,
  `defaultValue`, `query`, `defaultQuery`, `cleanable`, `open`, `defaultOpen`, `placeholder`, `disabled`,
  `accessibilityLabel`, `placement`, `align` and `renderItem` passed through; `offset` is `popover-offset`, `margin`
  `popover-margin`, as Select's.
- **The value**: the library always passes `nsh/base` a controlled `value`, its own signal when the app gives none (as
  Select), so the rows' checks and the chips read it, and a chip's remove button can change it (reporting `onChange`
  itself, its own signal following first).
- **The field**: `nsh/base`'s root view (a row: the prefix, the input, the clear button, the chevron button) is drawn
  with the field look on its root element: edge, fill, shadow, invalid, disabled, as Input's, and the focus ring on
  `focus-within:` (the input inside has the focus, shadcn's `InputGroup`), the edge only on touch screens. This needs
  `foundation::field_look` to take where its focus comes from (`focus_visible` or `focus_within`): an addition to
  foundation. Height (`min-h`) `input-height-<size>`.
- **The input**: `nsh/base`'s, styled by a bound `inputStyle` (as Select's rows: the props struct written by the
  library): no edge, no fill, no ring, `flex: 1`, `min-width: 0`, padding `input-padding-x-<size>` at the start (none
  at the end when a button follows), text `input-font-size-<size>` in `input-foreground`, placeholder
  `input-placeholder-foreground`; the app's `inputStyle` merged after.
- **The buttons** (shadcn's `InputGroupButton variant="ghost" size="icon-xs"`): the chevron goes in `nsh/base`'s
  `triggerButton` slot and the cross in its `clearButton` slot, each a view of this library's (not a library `Button`,
  which would take the press from `nsh/base`'s wrapper: Popover's NSH gaps 7) holding the icon, with the ghost
  button's hover fill. While the clear button shows, the chevron is not passed (shadcn hides the trigger next to the
  clear button). `clearButtonAccessibilityLabel` is the library's "Clear" (`labels`, translatable).
- **The chips** (`multiple`): a row of this library's in `nsh/base`'s `prefix` slot, one chip per value (its title
  from `items`), each with a remove button; it wraps within the room left of the input (NSH gaps 2).
- **The list**: Select's, shared: `foundation::popup::surface` on `popupStyle` (bound), `menu-padding` on `listStyle`,
  rows through the same bound `itemStyle`, the generated highlight `itemClass` and the library's `renderItem` with
  the check (shown for each value in the current value), headers through `groupStyle`. The implementation moves
  Select's list helpers into `foundation::menu` (a `list` module) so both call the same code. The list's height is
  capped at `combobox-list-max-height`.
- **Empty**: `nsh/base`'s `empty` slot, holding the app's `empty` or the library's words, centered.
- **Text styles and scope**: the popup resets inherited text styles and reads the `ThemeScope` around the
  `Combobox`, as Select's.
- **Field**: inside a [Field](field.md) the combobox joins it (`invalid`, `disabled`, a `FieldLabel`'s press focusing
  the input).

## API

### Props

| TSX | Rust | Type | Default | Description |
|---|---|---|---|---|
| `items` | `items` | `SelectItem[]` / `Vec<SelectItem>` | `[]` | The options, in order (`nsh/base`'s type, Select's) |
| `multiple` | `multiple` | `boolean` | `false` | Several values, shown as chips; a choice toggles and the list stays open |
| `value` | `value` | `string[]` / `Prop<Option<Vec<SharedString>>>` | Uncontrolled | The chosen values; the app sets them in `onChange` |
| `defaultValue` | `default_value` | `string[]` | `[]` | The first value while uncontrolled |
| `query` | `query` | `string` / `Prop<Option<SharedString>>` | Uncontrolled | What is typed (controlled), for an app that filters `items` itself |
| `defaultQuery` | `default_query` | `string` | — | The query each opening starts with |
| `placeholder` | `placeholder` | `string` | — | Shown while the input is empty |
| `size` | `size` | `Size` | `"md"` | The field's height, type, padding and icons, and the rows'; Input's and Select's sizes |
| `disabled` | `disabled` | `boolean` | `false` | It does not open, takes no focus, is dimmed |
| `invalid` | `invalid` | `boolean` | `false` | The app rejects the value: the field look's red edge and ring; screen readers say invalid |
| `cleanable` | `cleanable` | `boolean` | `false` | A clear button while a value is set (shadcn's `showClear`) |
| `showTrigger` | `show_trigger` | `boolean` | `true` | The chevron button at the end (shadcn's) |
| `open` / `defaultOpen` | `open` / `default_open` | `boolean` | Uncontrolled / `false` | The list's open state |
| `placement` | `placement` | `Placement` | `"bottom"` | Desktops: the side of the field (shadcn's `side`) |
| `align` | `align` | `Align` | `"start"` | Desktops: the alignment along it |
| `renderItem` | `render_item` | `(entry: SelectEntry) => element` / `Renderer<SelectEntry>` | — | A row's content in place of its title; the library still draws the row and its check |
| `empty` | `empty` | element (slot) | "No items found." | What the list shows while no item does |
| `accessibilityLabel` | `accessibility_label` | `string` | — | What screen readers call it; set it even beside a label (Input's G1) |
| `class` / `style` | `class` / `style` | | | The field (the root): width and layout (`w-[240px]`) |
| `inputClass` / `inputStyle` | `input_class` / `input_style` | | | The text input; `inputStyle` merged after the library's |
| `iconClass` / `iconStyle` | `icon_class` / `icon_style` | | | The chevron and clear buttons' icons |
| `chipClass` / `chipStyle` | `chip_class` / `chip_style` | | | Each chip (`multiple`) |
| `contentClass` / `contentStyle` | `content_class` / `content_style` | | | The popup or sheet, as Select's |
| `itemClass` / `itemStyle` | `item_class` / `item_style` | | | Each row, as Select's |
| `labelClass` / `labelStyle` | `label_class` / `label_style` | | | Each group header, as Select's |

Not exposed, though `nsh/base` has them: `footer` (shadcn has no footer; added with a look when asked for),
`prefix` / `suffix` (the field's own icons: InputGroup, later), `renderGroup` (the header is the group's title).

### Events

| TSX | Rust (`ComboboxEvent`) | Payload | When |
|---|---|---|---|
| `onChange` | `Change(Vec<SharedString>)` | `{ value }` | A choice, a toggle, the clear button, a chip's remove button |
| `onConfirm` | `Confirm(Vec<SharedString>)` | `{ value }` | The list closed: the value then, once per opening |
| `onQueryChange` | `QueryChange(SharedString)` | `{ query }` | The user typed |
| `onOpenChange` | `OpenChange(open, reason)` | `{ open, reason }` | The user asks to open or close the list |
| `onOpen` / `onClose` / `onCancel` | `Open` / `Close` / `Cancel` | — | Shown / gone / dismissed without a choice |

`nsh/base`'s events, forwarded.

### Ref

`ComboboxRef`, with `focus()` (the input), so a `FieldLabel` or a `Label`'s `htmlFor` focuses it ([Field](field.md),
Decisions 3).

### Parts

| Part | What | Customize with |
|---|---|---|
| root | The field: `nsh/base`'s row | `class` / `style` |
| input | The text input | `inputClass` / `inputStyle` |
| icon | The chevron, the cross | `iconClass` / `iconStyle` |
| chip | Each chosen value (`multiple`) | `chipClass` / `chipStyle` |
| content | The popup or sheet | `contentClass` / `contentStyle` |
| item | Each row | `itemClass` / `itemStyle` |
| label | Each group header | `labelClass` / `labelStyle` |

## Variants and sizes

No variants.

### The field

shadcn: `InputGroup` (`flex w-full items-center rounded-md border border-input shadow-xs dark:bg-input/30 h-9`, the
focus ring on `has-[…:focus-visible]`, the invalid edge on `has-[…][aria-invalid=true]`), its input
`flex-1 border-0 bg-transparent shadow-none`, `pr-2` before an end addon; the addon `pr-3 has-[>button]:mr-[-0.45rem]`;
the buttons `size-6 rounded-[calc(var(--radius)-5px)]`, ghost, the chevron `size-4 text-muted-foreground`.

| `size` | Field height (pointer / touch) | Text | Start padding | Button (square) | Icon |
|---|---|---|---|---|---|
| `xs` | 24 / 32 | 12 / 14 | 8 | 16 / 24 | 12 |
| `sm` | 28 / 36 | 13 / 14 | 10 | 20 / 28 | 14 |
| `md` | 32 / 40 | 14 / 16 | 12 | 24 / 32 | 16 |
| `lg` | 36 / 48 | 14 / 16 | 12 | 28 / 40 | 16 |

- Height, text and padding are Input's tokens, so an Input, a Select and a Combobox of one `size` line up.
- **The buttons** are squares of the field's height − 8, inset 4 from the field's end and centered (`md`: 24,
  shadcn's `icon-xs`, 4 from the edge as shadcn's `mr-[-0.45rem]` gives within 1). Their corners are concentric with
  the field's: `input-radius − 4` (4 at `md`), clamped at 0 (shadcn's `radius − 5`, 5, is concentric with nothing).
  Hover fill `button-ghost-hover` (the ghost button's), the icon `combobox-icon-foreground` (`muted-foreground`);
  pressed: no fill (shadcn's `data-pressed:bg-transparent`). The input's end padding is 0 when a button follows
  (shadcn's `pr-2` with the addon's padding comes to about the same).
- **Width**: as wide as its parent (`w-full min-w-0`, Input's), the app sets one with `class`. shadcn's
  `ComboboxInput` is `w-auto` around an HTML input's intrinsic width, which NSH's input has not.

### Chips (`multiple`)

shadcn: `ComboboxChips` `min-h-9 flex-wrap gap-1.5 px-2.5 py-1.5`, with chips `px-1.5`; `ComboboxChip` `h-5.5 gap-1
rounded-sm bg-muted px-1.5 text-xs font-medium text-foreground`, its remove button ghost `icon-xs`, `-ml-1 opacity-50
hover:opacity-100`, an `XIcon`; `ComboboxChipsInput` `min-w-16 flex-1`.

| | This library |
|---|---|
| Field | At least `input-height-<size>` tall, growing as chips wrap; start padding `spacing × 1.5` (6) while there are chips; chips and input `spacing × 1.5` (6) apart; vertical padding (field height − chip height) / 2 |
| Chip | `combobox-chip-height` (22) tall, corners `radius-sm` (6), fill `combobox-chip` (`muted`), text 12 (`text-xs`) `font-weight-medium` `combobox-chip-foreground` (`foreground`), padding `spacing × 1.5` (6) at the start, the title cut with an ellipsis |
| Remove | A square the chip's height, the `x` icon at 12, at 50% opacity, 100% under the pointer (shadcn's `opacity-50 hover:opacity-100`), no fill |
| Input | At least 64 wide (`min-w-16`), taking the room left |

### The list

Select's list, the same parts and values (Select's "The list"): the popup surface (`popover`, `popover-border`,
`popover-radius`, shadow tier 3), padding `menu-padding`, rows `menu-item-height-<size>` with the title and the check
at the end (each chosen value checked), group headers in `select-label-font-size` / `select-label-foreground`, as wide
as the field (at least `menu-min-width`), `popover-offset` (4) below it. Its height is at most
`combobox-list-max-height` (348, shadcn's `--spacing(96) − --spacing(9)`) and the room on its side.

The empty state: `py-2 text-center text-sm text-muted-foreground` → padding 8 above and below, centered, 14,
`combobox-empty-foreground` (`muted-foreground`).

Differences from shadcn's list: its edge is the popup surface's `border` (shadcn's combobox uses `ring-1
ring-foreground/10`, its select and menus a border), its offset 4 (shadcn's 6), its motion `duration` (shadcn's
`duration-100`): one popup look for every list (Popover's "The popup surface"). Rows are one density step down, as
Select's.

### Motion

Select's: fade and zoom from 95% over `duration`, none with reduced motion, `popover-animate` 0 to turn it off; no
slide (`nsh/base`'s `Combobox` has no `onPosition`, as `Select`). The sheet has `nsh/base`'s motion.

## States

The field takes the field look's states (Input's table), its focus on the input inside (`focus-within`):

| State | Look | shadcn |
|---|---|---|
| Rest | Edge `input-border`, fill `input-background`, shadow 1 | `border-input shadow-xs dark:bg-input/30` |
| Input focused (pointer) | Edge `input-border-focus`, ring `input-ring` | `has-[…:focus-visible]:border-ring ring-[3px] ring-ring/50` |
| Input focused (touch) | The edge only | — |
| Invalid | Edge `input-border-invalid`; focused, ring `input-ring-invalid` | `has-[…][aria-invalid=true]:border-destructive …` |
| Disabled | `opacity-disabled` over the field, no presses, the arrow cursor | `group-data-[disabled=true]` addons `opacity-50` |
| Open | The input keeps the focus: the focused look | — |
| Placeholder | Input's placeholder color | — |
| Button hover (pointer) | `button-ghost-hover` fill | `hover:bg-accent` (ghost) |
| Row highlighted, chosen, disabled, pressed on touch | Select's rows | `data-highlighted:bg-accent …` |
| Chip remove hover | Icon at 100% | `hover:opacity-100` |

The field's colors, edge and ring transition over `duration` (the field look's); the rows' highlight changes at once.
The cursor is the I-beam over the input (native), the arrow on the buttons, chips and rows.

## Pointer and touch

| | Desktops and desktop browsers | Phones and tablets |
|---|---|---|
| Field height | `input-height-<size>`: 24 / 28 / 32 / 36 | 32 / 36 / 40 / 48, a vertical hit slop below 44, as Input |
| Buttons | 16 / 20 / 24 / 28 | 24 / 28 / 32 / 40, a vertical hit slop to 44 (none sideways: the input is beside them) |
| Focus | The edge and the ring | The edge only |
| Opening | Typing, the arrow keys, a press on the input or the chevron | The tap that focuses the input, typing |
| List | A popup below the field, as wide as it | A sheet at the bottom (`nsh/base`'s), the menu's sheet look, rows 44 / 44 / 44 / 48 |
| Chip remove | 22 square | 22 square, a hit slop to 44 vertically |

★ On a phone the sheet and the software keyboard share the bottom of the screen with the input typed into: whether
the input stays visible above both is `nsh/base`'s placement, to check when phones are reviewed.

## Accessibility

`nsh/base`'s (see Built on): the input is a `combobox` (`expanded`, `hasPopup`), its value the chosen titles; the list
a `listbox`, rows `option`s with `selected` / `disabled`, headers `header`s; the highlighted row is the input's active
descendant. `invalid` shows as the red edge only: `nsh/base`'s input takes no `invalid` state (NSH gaps 3). The
library hides its icons, names the clear button ("Clear"; `nsh/base` hides the button from screen readers, the input
saying whether a value is set) and each chip's remove button ("Remove" and the title: a button screen readers reach,
not a Tab stop). `accessibilityLabel` names the combobox (Input's G1).

## Tokens

The field reads the `input-*` tokens ([Input](input.md#tokens)), the rows and surface the `menu-*` and `popover-*`
tokens ([DropdownMenu](dropdown-menu.md#tokens), [Popover](popover.md#tokens)), the headers Select's
`select-label-*`, the buttons' hover `button-ghost-hover`. Its own tokens are what shadcn's combobox adds. Generated
from the code's declarations (`NSH_LONGBRIDGE_WRITE=1 cargo test --test tokens`); a test keeps it current.

<!-- tokens:combobox -->
| Token | Type | Derived default | Value |
|---|---|---|---|
| `combobox-icon-foreground` | color | `muted-foreground` |  |
| `combobox-list-max-height` | length | `spacing × 87` | 348 |
| `combobox-empty-foreground` | color | `muted-foreground` |  |
| `combobox-chip` | color | `muted` |  |
| `combobox-chip-foreground` | color | `foreground` |  |
| `combobox-chip-height` | length | `spacing × 5.5` | 22 |
<!-- /tokens -->

- `combobox-icon-foreground` is shadcn's `text-muted-foreground` (Select's chevron is `select-icon-foreground`, muted
  at 50%: shadcn's two differ).
- The group headers read Select's tokens rather than declaring their own: Select owns the list header look of the
  lists `nsh/base`'s listbox draws (Select and Combobox); a theme changing `select-label-foreground` changes both.
- Other tokens it reads: `radius-sm`, `text-xs`, `text-sm`, `font-weight-medium`, `opacity-disabled`, `icon-size-*`,
  `touch-target`, `overlay`, `shadow`, `duration`.

## Examples

```tsx
import { signal, Text } from "nsh";
import { Combobox, type SelectItem } from "nsh-longbridge";

const frameworks: SelectItem[] = [
  { value: "next", title: "Next.js" },
  { value: "sveltekit", title: "SvelteKit" },
  { value: "nuxt", title: "Nuxt.js" },
  { value: "remix", title: "Remix" },
  { value: "astro", title: "Astro" },
];

const framework = signal<string[]>([]);

<Combobox
  class="w-[240px]"
  items={frameworks}
  value={framework()}
  onChange={({ value }) => framework.set(value)}
  placeholder="Select a framework"
  cleanable
  accessibilityLabel="Framework"
/>

// Several values as chips.
const picked = signal<string[]>(["next"]);
<Combobox
  class="w-[320px]"
  multiple
  items={frameworks}
  value={picked()}
  onChange={({ value }) => picked.set(value)}
  placeholder="Add frameworks"
  accessibilityLabel="Frameworks"
/>

// Grouped, small, invalid until chosen, with an empty state of the app's.
<Combobox
  size="sm"
  items={timeZones}
  value={zone()}
  invalid={zone().length === 0}
  onChange={({ value }) => zone.set(value)}
  empty={<Text>No time zone matches.</Text>}
  accessibilityLabel="Time zone"
/>

// The app filters a server's results itself: it keeps the query and gives the items that match.
<Combobox
  items={results()}
  query={query()}
  onQueryChange={({ query: q }) => { query.set(q); search(q); }}
  value={user()}
  onChange={({ value }) => user.set(value)}
  placeholder="Search people"
  accessibilityLabel="Assignee"
/>
```

With NSH 0.7.0, a string child (`empty`'s `<Text>`) is written as shown ([Button's G0](button.md#nsh-gaps)).

```rust
use nsh_core::prelude::*;
use nsh_longbridge::{SelectItem, Size, combobox};

let framework = signal(Vec::<SharedString>::new());

combobox()
    .app_class(Some("w-[240px]".into()))
    .items(vec![
        SelectItem::new("next", "Next.js"),
        SelectItem::new("sveltekit", "SvelteKit"),
        SelectItem::new("astro", "Astro"),
    ])
    .value(move || Some(framework.get()))
    .on_change(move |value: &Vec<SharedString>| framework.set(value.clone()))
    .placeholder("Select a framework")
    .size(Size::Sm)
    .cleanable(true)
    .accessibility_label("Framework")
```

`nsh/base` exports a `Combobox` too; an app using both renames one.

## NSH gaps

1. **The rows and the popup are `nsh/base`'s parts** (Select's NSH gaps 1, the same accepted limits and the same
   workaround: bound part props, a generated highlight class).
2. **Chips wrap inside the prefix, not around the input.** `nsh/base` places the app's `prefix` as one element before
   the input, so the chips wrap among themselves within the room the input leaves, where shadcn's input flows after
   the last chip on the same line. With few chips both look the same; with many, the input keeps a column at the end.
   A combobox root that laid its prefix's children and the input out in one wrapping row would match shadcn.
3. **`invalid` on the input.** `nsh/base`'s `Combobox` takes no `invalid` and its input's accessibility state is
   `nsh/base`'s own (expanded, has a popup, disabled), so the library cannot add `invalid` to it: screen readers do
   not hear the combobox as invalid (the red edge shows). An `invalid` prop on `nsh/base`'s `Combobox` (as on its
   `Select`, which the library's trigger carries) would close it; the field's `FieldError` is still announced when it
   appears ([Field](field.md)).
4. **No Backspace to remove the last chip**, no arrow keys between chips (Base UI has both): `nsh/base`'s combobox
   has neither. Chips are removed by their buttons or by choosing the item again in the list.
5. **No separators in the list** (Select's NSH gaps 3).
6. **No described-by** on `nsh/base`'s combobox (Field's NSH gaps 5).
7. ★ **The list's maximum height** goes on `nsh/base`'s `listStyle` (the scroll view of the rows); to check that the
   popup then shrinks to the list rather than keeping the room's height.

## Decisions

To settle in review:

1. **Options are data, the value a list** (`nsh/base`'s): as Select's Decisions 1; one value is `[value]`.
2. **One popup look**: the popup surface and offset shared with Select, not shadcn's combobox ring and 6 offset.
3. **The buttons are concentric with the field** (`input-radius − 4`), not shadcn's `radius − 5`.
4. **shadcn's "popup" combobox** (a button-like trigger showing the value, the search field inside the popup) is
   `nsh/base`'s `Select` with `searchable`, which Select's spec did not expose in P2. Proposed: expose `searchable` on
   `Select` with the search field drawn as shadcn's (`m-1 mb-0 h-8`, `input` at 30% fill and edge, no shadow), in a
   change to [select.md](select.md); not part of this component.
5. **No free-text mode.** Base UI's combobox can keep typed text that matches no item; `nsh/base`'s chooses only
   items. An autocomplete field (suggestions under an Input) waits for a real need and `nsh/base` support.
6. **Library words**: "No items found." (`empty`'s default, shadcn's examples' words), "Clear" and "Remove", added to
   `labels` (`Label::NoItems`, `Label::Clear`, `Label::Remove`), translatable as the others.
