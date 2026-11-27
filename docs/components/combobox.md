# Combobox

Status: implemented (P3).

A text field that filters a list as the user types and chooses one value, or several shown as chips. The field is
drawn by foundation's **field look** (the `input-*` tokens, as [Input](input.md)); the list by the **menu look** (the
`menu-*` tokens, owned by [DropdownMenu](dropdown-menu.md)) on the popup surface (owned by [Popover](popover.md)),
exactly as [Select](select.md)'s list.

## Usage

- A choice from a long list the user would rather type into than scroll: a country, a framework, a time zone, an
  assignee, a stock symbol of the app's own list; with `multiple`, several of them (labels, recipients, tags from a
  known set); with `creatable`, a tag input that also makes the typed text a new tag; a recent group while nothing is
  typed (the app's items, Examples).
- Search terms: combobox, autocomplete, typeahead, searchable select, filter select, multi-select, tag input, chips
  input, creatable, create option, match highlight, recent searches.

## When not to use

| Need | Use instead |
|---|---|
| A choice from a short fixed list, no typing | [Select](select.md) |
| A button that opens a list with a search field inside it (a "popup" combobox) | Not in P3: a later `searchable` on [Select](select.md) (its "Later") |
| Free text with suggestions the user need not take | An [Input](input.md); a suggestion list is not offered yet (Decisions 5) |
| Commands, searched | Command (later) |
| Two to five options, all worth seeing | [RadioGroup](radio-group.md), [ToggleGroup](toggle-group.md) |
| A date | DatePicker (P3) |

## Built on

`nsh/base`'s `Combobox` (`crates/nsh-base/src/combobox.rs`) and its list (`listbox.rs`, shared with `Select`). This
library adds the field it draws around the input, the chevron and clear buttons, the chips, the look of the list (the
matched part of each title emphasized), the create row (`creatable`) and `size` / `invalid`; all other behavior is
`nsh/base`'s:

| Provided by `nsh/base` (not rewritten here) | |
|---|---|
| Value | `value: string[]` (controlled) / `defaultValue`; without `multiple` a choice replaces it and closes the list, with `multiple` it toggles the item and the list stays open; `onChange({ value })` on each change, `onConfirm({ value })` once as the list closes; Cmd+Return (Ctrl+Return off Apple's) confirms and closes |
| Query | Typing filters the items by title (a case-insensitive substring, in order; a section without matches goes); each opening starts from `defaultQuery`, else empty; `query` / `onQueryChange({ query })` when the app filters itself |
| The input | One value: shows the chosen title until the user types, selected when the input takes the focus (typing replaces it). Several: the query only |
| Opening | Typing, Down or Up (Down on the first enabled item when none is chosen, Up on the last), a press on the input (not in a browser: NSH gaps 12) or on the chevron button opens it; desktops: Tab into the input does not; touch screens: the tap that focuses the input opens it once |
| List keys | Up / Down (around the ends), Home / End, Page Up / Down, Return chooses, Escape, back and a press outside close (`"cancel"`, `"pressOutside"`), Tab or Shift+Tab close (`"focus"`) and move on; a new query highlights its first enabled match |
| Focus | The input keeps the keyboard focus while the list is open (the highlighted row is its active descendant); the clear and chevron buttons are no Tab stops |
| Presentation | Desktops: a popup below the field (`placement`, `align`), as wide as it, never taller than the room on its side. Touch screens: a sheet at the bottom over a backdrop |
| Rows | Virtualized: thousands of items make only the rows in view; sections from the items' `group`; `focus:` (highlighted), `selected:`, `disabled:` on each row |
| Clear button | With `cleanable`, while a value is set: a press clears it (`onChange({ value: [] })`) and gives the focus back to the input |
| Control | `open` / `defaultOpen` / `onOpenChange({ open, reason })`, `onOpen`, `onClose`, `onCancel` |
| Accessibility | The input a `combobox` (`expanded`, `hasPopup`, its value the chosen titles); the list a `listbox`, rows `option` with `selected` and `disabled`, headers `header`; the buttons hidden from screen readers |
| Terminal | A one-row field with ` ×` and ` ▾`, rows of titles with `✓`; this library's look does not apply there |

### Options are data

A combobox composed from parts on the web builds its list from them: an input with its trigger and clear buttons, a
content with an empty state, groups with labels, items, separators, and chips with their input for several values.
`nsh/base`'s `Combobox` takes its options as **data** (`items`), for the reasons Select gives (virtualized rows; the
filter, highlight and typeahead run over the titles; a component cannot read its children's props). So, as Select:

| A composed combobox's | This library |
|---|---|
| Items and a function drawing each | `items: SelectItem[]` (`{ value, title, disabled?, group? }`); a row's content by `renderItem` |
| Groups with a label | The items' `group`: a header with the group's title |
| The input with its trigger and clear buttons | `Combobox`'s `placeholder`, `showTrigger` (default `true`), `cleanable` (`nsh/base`'s name) |
| The content's side and alignment | `placement`, `align`, `contentClass` / `contentStyle` |
| The empty state | `empty` (a slot; default: "No items found.") |
| An item that makes the typed text a new value | `creatable` and `onCreate({ query })`: the library's row (Decisions 9) |
| Chips and their input | `multiple`: the library draws a chip per value before the input |
| Separators | None (Select's NSH gaps 3) |
| Value, query and open events | `onChange({ value })` / `onQueryChange({ query })` / `onOpenChange({ open, reason })` |
| A value that is an item, or a list | Always a list of values (`nsh/base`'s): one value is `[value]` |

### Implementation

- **Structure**: a `#[component] fn Combobox` whose root is `nsh_base::combobox()`, `items`, `multiple`, `value`,
  `defaultValue`, `query`, `defaultQuery`, `cleanable`, `open`, `defaultOpen`, `placeholder`, `disabled`,
  `accessibilityLabel`, `placement`, `align` and `renderItem` passed through; `offset` is `popover-offset`, `margin`
  `popover-margin`, as Select's.
- **The value**: the library always passes `nsh/base` a controlled `value`, its own signal when the app gives none (as
  Select), so the rows' checks and the chips read it, and a chip's remove button can change it (reporting `onChange`
  itself, its own signal following first).
- **The field**: `nsh/base`'s root view (a row: the prefix, the input, the clear button, the chevron button) is drawn
  with the field look on its root element: edge, fill, shadow and invalid edge from `foundation::field`'s
  `field_rest`, and the focus edge and ring (`field_focus`, the edge only on touch screens) under `focus-within:`,
  while the input inside has the focus. `field_look` is these two with `focus-visible:`; the split is an addition to
  foundation. `nsh/base`'s root has no disabled state, so the disabled opacity is a binding. Height (`min-h`)
  `input-height-<size>`.
- **The input**: `nsh/base`'s part. Its look is a style binding set on the part's `inputStyle` prop (the builder's
  setter takes a style built once), the app's `inputStyle` merged after it, the app's `inputClass` passed through: no
  edge, no fill, no ring, `flex: 1`, `min-width: 0` (64 with `multiple`), padding `input-padding-x-<size>` at the start
  (none beside chips, none at the end when a button follows), text `input-font-size-<size>` in `input-foreground`,
  placeholder `input-placeholder-foreground`. The list's parts are set the same way (`foundation::menu`).
- **The buttons**: the chevron goes in `nsh/base`'s `triggerButton` slot and the cross in its `clearButton` slot,
  each a view of this library's (not a library `Button`, which would take the press from `nsh/base`'s wrapper:
  Popover's NSH gaps 7) holding the icon, with the ghost button's hover fill. One button shows at a time: the chevron
  hides while the clear button shows (a slot is filled once, so the chevron is placed and hidden, not left out). The
  clear button's frame is a style binding on `nsh/base`'s wrapper and its hover a class made from the tokens (only
  `nsh/base` knows the wrapper's states). `clearButtonAccessibilityLabel` is the library's "Clear" (`labels`,
  translatable).
- **The chips** (`multiple`): a row of this library's in `nsh/base`'s `prefix` slot, one chip per value (its title
  from `items`), each with a remove button; it wraps within the room left of the input (NSH gaps 2).
- **The list**: Select's list drawing in `foundation::menu` (`list_look!`), which Select and Combobox call: the popup
  surface, `menu-padding`, the rows' look and highlight, the library's `renderItem` with the check (here shown for
  each value in the current value) and the group headings (`menu-label-*`), on `nsh/base`'s `popup*`, `list*`,
  `item*` and `group*` parts. The list's height, its padding inside, is capped at `combobox-list-max-height`; the
  popup shrinks with it.
- **The query**: the library holds it (the app's `query` when it sets one, else its own, starting empty, set to
  `defaultQuery` at each opening and once the list is gone) and always passes it to `nsh/base` as a controlled
  `query`, so the rows' emphasis and the create row read the very query the rows are filtered by, in the same turn
  (`nsh/base` tells a typed query only after filtering with it: NSH gaps 10). Its own query going back to
  `defaultQuery` is reported with `onQueryChange`, so an app watching it (a recent group, Examples) stays right.
- **The matched part**: without `renderItem`, a row's title is drawn by `foundation::menu`'s list row as runs of
  text, the first place the query matches it (`foundation::text_match`, the filter's own matching: ignoring case as
  `nsh/base` folds it) in `menu-item-match-font-weight`, the rest as before. The runs share the row's color, so the
  highlight, the disabled dimming and a theme's colors apply to them as to the title. An app's `renderItem` draws
  its rows as it likes (it can call the same splitting in Rust).
- **The create row** (`creatable`): while the query has text other than spaces and no item's title is the same text
  (ignoring case and spaces at the ends), the library adds one item after the app's: a value of its own no app value
  can be (it starts with a NUL character, and changes after each use) and the title "Create “<query>”" (`labels`'
  `create`, translatable), which holds the query, so `nsh/base`'s filter keeps it and screen readers read it. It is a
  row as any other for the keys, the pointer and the highlight (alone, it is highlighted as a query's first match
  is); the library draws it (a plus, then the words with the query in the match weight), even beside an app's
  `renderItem`. Choosing it reports `onCreate({ query })` (the query without the spaces at its ends) and no
  `onChange`; the value never holds the row's own value (`onChange` and `onConfirm` leave it out). Without
  `multiple` the list closes as after a choice; with `multiple` it stays open and the query is emptied (reported by
  `onQueryChange({ query: "" })`, which an app that sets `query` follows), ready for the next one. The app adds the
  item (and, to choose it, its value) in `onCreate`.
- **Empty**: `nsh/base`'s `empty` slot, holding the app's `empty` or the library's words, centered, in
  `combobox-empty-foreground` at 14, which the app's `empty` inherits (as Select's).
- **Text styles and scope**: the popup resets inherited text styles and reads the `ThemeScope` around the
  `Combobox`, as Select's.
- **Field**: inside a [Field](field.md) the combobox joins it (`invalid`, `disabled`, and a `FieldSet`'s `disabled`).
  A label's press reaches nothing: `nsh/base` gives no handle on its input (NSH gaps 8).

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
| `cleanable` | `cleanable` | `boolean` | `false` | A clear button while a value is set |
| `creatable` | `creatable` | `boolean` | `false` | A last row, "Create “<query>”", while no item's title is the query: choosing it reports `onCreate` |
| `showTrigger` | `show_trigger` | `boolean` | `true` | The chevron button at the end, hidden while the clear button shows. Read when made |
| `open` / `defaultOpen` | `open` / `default_open` | `boolean` | Uncontrolled / `false` | The list's open state |
| `placement` | `placement` | `Placement` | `"bottom"` | Desktops: the side of the field |
| `align` | `align` | `Align` | `"start"` | Desktops: the alignment along it |
| `renderItem` | `render_item` | `(entry: SelectEntry) => element` / `Renderer<SelectEntry>` | — | A row's content in place of its title (then no emphasis of the matched part); the library still draws the row and its check, and the create row |
| `empty` | `empty` | element (slot) | "No items found." | What the list shows while no item does |
| `accessibilityLabel` | `accessibility_label` | `string` | — | What screen readers call it; set it even beside a label (Input's G1) |
| `class` / `style` | `class` / `style` | | | The field (the root): width and layout (`w-[240px]`) |
| `inputClass` / `inputStyle` | `input_class` / `input_style` | | | The text input; `inputStyle` merged after the library's |
| `iconClass` / `iconStyle` | `icon_class` / `icon_style` | | | The chevron and clear buttons' icons |
| `chipClass` / `chipStyle` | `chip_class` / `chip_style` | | | Each chip (`multiple`) |
| `contentClass` / `contentStyle` | `content_class` / `content_style` | | | The popup or sheet, as Select's |
| `itemClass` / `itemStyle` | `item_class` / `item_style` | | | Each row, as Select's |
| `labelClass` / `labelStyle` | `label_class` / `label_style` | | | Each group header, as Select's |

Not exposed, though `nsh/base` has them: `footer` (no look for one yet; added with a look when asked for),
`prefix` / `suffix` (the field's own icons; [InputGroup](input-group.md) gives an `Input` its icons and buttons, not a
combobox, which draws its own field), `renderGroup` (the header is the group's title).

### Events

| TSX | Rust (`ComboboxEvent`) | Payload | When |
|---|---|---|---|
| `onChange` | `Change(Vec<SharedString>)` | `{ value }` | A choice, a toggle, the clear button, a chip's remove button |
| `onConfirm` | `Confirm(Vec<SharedString>)` | `{ value }` | The list closed: the value then, once per opening |
| `onQueryChange` | `QueryChange(SharedString)` | `{ query }` | The user typed; the combobox's own query went back to `defaultQuery` (the list gone, or opening); a created value emptied it (`multiple`) |
| `onCreate` | `Create(SharedString)` | `{ query }` | The create row was chosen (`creatable`): the query, without spaces at its ends |
| `onOpenChange` | `OpenChange(open, reason)` | `{ open, reason }` | The user asks to open or close the list |
| `onOpen` / `onClose` / `onCancel` | `Open` / `Close` / `Cancel` | — | Shown / gone / dismissed without a choice |

`nsh/base`'s events, forwarded; `onCreate` is the library's, and so are the query's resets (`nsh/base` makes them
silently).

### Ref

None yet: `nsh/base`'s `Combobox` gives no handle on its input (no ref, no focus method), so no `ComboboxRef` with
`focus()` is offered, a `Label`'s `htmlFor` cannot name a combobox, and a `FieldLabel`'s press reaches nothing (NSH
gaps 8).

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

| `size` | Field height (pointer / touch) | Text | Start padding | Button (square) | Icon |
|---|---|---|---|---|---|
| `xs` | 24 / 32 | 12 / 14 | 8 | 16 / 24 | 12 |
| `sm` | 28 / 36 | 13 / 14 | 10 | 20 / 28 | 14 |
| `md` | 32 / 40 | 14 / 16 | 12 | 24 / 32 | 16 |
| `lg` | 36 / 48 | 14 / 16 | 12 | 28 / 40 | 16 |

- Height, text and padding are Input's tokens, so an Input, a Select and a Combobox of one `size` line up.
- **The buttons** are squares of the field's height − 8, inset 4 from the field's end and centered (`md`: 24). Their
  corners are concentric with the field's: `input-radius − 4` (4 at `md`), clamped at 0. Hover fill
  `button-ghost-hover` (the ghost button's), the icon `combobox-icon-foreground` (`muted-foreground`); pressed: no
  fill. The input's end padding is 0 when a button follows (the button's inset is the gap).
- **Width**: as wide as its parent (`w-full min-w-0`, Input's), the app sets one with `class`: NSH's input has no
  intrinsic width to size it by.

### Chips (`multiple`)

| | |
|---|---|
| Field | At least `input-height-<size>` tall, growing as chips wrap; start padding `spacing × 1.5` (6) while there are chips; chips and input `spacing × 1.5` (6) apart; vertical padding 4 within the field (with its edge), the buttons' inset, so the chips sit centered beside the buttons |
| Chip | `combobox-chip-height` (22) tall, corners `radius-sm` (6), fill `combobox-chip` (`muted`), text 12 (`text-xs`) `font-weight-medium` `combobox-chip-foreground` (`foreground`), padding `spacing × 1.5` (6) at the start, the title cut with an ellipsis |
| Remove | A square the chip's height, the `x` icon at 12, at 50% opacity, 100% under the pointer, no fill |
| Input | At least 64 wide, a chip's height, taking the room left |

### The list

Select's list, the same parts and values (Select's "The list"): the popup surface (`popover`, `popover-border`,
`popover-radius`, shadow tier 3), padding `menu-padding`, rows `menu-item-height-<size>` with the title and the check
at the end (each chosen value checked), group headings in `menu-label-font-size` / `menu-label-foreground`, as wide
as the field (at least `menu-min-width`), `popover-offset` (4) below it. Its height is at most
`combobox-list-max-height` (348, its padding inside: about eight and a half rows at `md`, so a cut row shows there is
more) and the room on its side; the popup shrinks with it.

The empty state: padding 8 above and below, centered, 14, `combobox-empty-foreground` (`muted-foreground`); the app's
`empty` inherits the color and size (Decisions 11).

**The matched part.** While the query has text, the first place it matches each title (as the filter matches:
ignoring case) is drawn in `menu-item-match-font-weight` (`font-weight-semibold`, 600), the rest of the title in the
row's regular weight; same color, size and line, the ellipsis still at the title's end. Only the first match: the
reason the row is there, without speckling a title with a one-letter query ("a" in "Alabama"). With an empty query,
and for an app's `renderItem`, titles are drawn as before.

**The create row** (`creatable`), the last row: a row of the list's size, a `plus` icon at the row's icon size where
the title starts, then "Create “<query>”" with the query in the match weight, cut with an ellipsis; no check. The
icon takes the row's color (the highlight's on the highlighted row).

Its edge, offset (4) and motion (`duration`) are the popup surface's, as every list's: one popup look (Popover's "The
popup surface"). Rows are one density step down, as Select's.

### Motion

Select's: fade and zoom from 95% over `duration`, none with reduced motion, `popover-animate` 0 to turn it off; no
slide (`nsh/base`'s `Combobox` has no `onPosition`, as `Select`). The sheet has `nsh/base`'s motion.

## States

The field takes the field look's states (Input's table), its focus on the input inside (`focus-within`):

| State | Look |
|---|---|
| Rest | Edge `input-border`, fill `input-background`, shadow 1 |
| Input focused (pointer) | Edge `input-border-focus`, ring `input-ring` |
| Input focused (touch) | The edge only |
| Invalid | Edge `input-border-invalid`; focused, ring `input-ring-invalid` |
| Disabled | `opacity-disabled` over the field, no presses, the arrow cursor |
| Open | The input keeps the focus: the focused look |
| Placeholder | Input's placeholder color |
| Button hover (pointer) | `button-ghost-hover` fill |
| Row highlighted, chosen, disabled, pressed on touch | Select's rows |
| A title the query matches | The first match in `menu-item-match-font-weight`; the row's colors in every state |
| Create row | A row as the others (highlight, press); the plus and the query's weight |
| Chip remove hover | Icon at 100% |

The field's colors, edge and ring transition over `duration` (the field look's); the rows' highlight changes at once.
The cursor is the I-beam over the input (native), the arrow on the buttons, chips and rows.

## Pointer and touch

| | Desktops and desktop browsers | Phones and tablets |
|---|---|---|
| Field height | `input-height-<size>`: 24 / 28 / 32 / 36 | 32 / 36 / 40 / 48 (no hit slop: NSH gaps 9) |
| Buttons | 16 / 20 / 24 / 28 | 24 / 28 / 32 / 40; the chevron a vertical hit slop to 44 (none sideways: the input is beside it), the clear button none (NSH gaps 9) |
| Focus | The edge and the ring | The edge only |
| Opening | Typing, the arrow keys, a press on the input (not in a browser: NSH gaps 12) or the chevron | The tap that focuses the input, typing |
| List | A popup below the field, as wide as it | A sheet at the bottom (`nsh/base`'s), the menu's sheet look, rows 44 / 44 / 44 / 48 |
| Chip remove | 22 square | 22 square, a hit slop to 44 vertically |

On a phone the sheet and the software keyboard share the bottom of the screen with the input typed into: whether
the input stays visible above both is `nsh/base`'s placement, checked when phones are reviewed (the README's
platforms); the library adds nothing there.

## Accessibility

`nsh/base`'s (see Built on): the input is a `combobox` (`expanded`, `hasPopup`), its value the chosen titles; the list
a `listbox`, rows `option`s with `selected` / `disabled`, headers `header`s; the highlighted row is the input's active
descendant. `invalid` shows as the red edge only: `nsh/base`'s input takes no `invalid` state (NSH gaps 3). The
library hides its icons, names the clear button ("Clear"; `nsh/base` hides the button from screen readers, the input
saying whether a value is set) and each chip's remove button ("Remove" and the title: a button screen readers reach,
not a Tab stop). `accessibilityLabel` names the combobox (Input's G1). The matched part's emphasis is the look only:
a row is read as its title. The create row is an `option` read as its words, "Create “web”" (its item's title), and
announced as the active descendant like any row.

## Tokens

The field reads the `input-*` tokens ([Input](input.md#tokens)), the rows and surface the `menu-*` and `popover-*`
tokens ([DropdownMenu](dropdown-menu.md#tokens), [Popover](popover.md#tokens)), the headers Select's
`menu-label-*`, the buttons' hover `button-ghost-hover`. Its own tokens are below. Generated from the code's
declarations (`NSH_LONGBRIDGE_WRITE=1 cargo test --test tokens`); a test keeps it current.

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

- `combobox-icon-foreground` is `muted-foreground`, stronger than Select's chevron (`select-icon-foreground`, muted at
  50%): the combobox's chevron and cross are buttons of their own, the select's chevron a mark on a trigger.
- The group headings read the menu family's `menu-label-foreground` (`muted-foreground`) and `menu-label-font-size`
  (12), declared in `foundation::menu` (DropdownMenu's spec owns them): one owner for list looks, so a theme changing
  `menu-label-foreground` changes Select's and Combobox's headings.
- The matched part's weight is the menu family's `menu-item-match-font-weight` (`font-weight-semibold`), declared in
  `foundation::menu` beside the rows' other tokens, for the same reason: a searchable list of Select's would read it
  too. A theme sets it to 400 (the rows' regular weight) to draw no emphasis.
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

// Tags: the typed text becomes a new tag when none is called that.
const tags = signal<SelectItem[]>(knownTags);
const chosen = signal<string[]>([]);
<Combobox
  multiple
  creatable
  items={tags()}
  value={chosen()}
  onChange={({ value }) => chosen.set(value)}
  onCreate={({ query: title }) => {
    const value = `tag-${title.toLowerCase()}`;
    tags.set([...tags(), { value, title }]);
    chosen.set([...chosen(), value]);
  }}
  placeholder="Add tags"
  accessibilityLabel="Tags"
/>

// Recent choices while nothing is typed: the items are data, so the app gives a "Recent" group then, and the whole
// list once a query filters it. The query is the combobox's own; the app only watches it.
const typed = signal("");
const recent = signal<string[]>(["sin", "hnd"]);
<Combobox
  items={typed().trim() === "" ? withRecent(airports, recent()) : airports}
  onQueryChange={({ query: q }) => typed.set(q)}
  value={airport()}
  onChange={({ value }) => {
    airport.set(value);
    recent.set([...value, ...recent().filter((v) => !value.includes(v))].slice(0, 3));
  }}
  placeholder="Search airports"
  accessibilityLabel="Airport"
/>
// withRecent: the recent items under group "Recent", then the others under "All airports" (one value, one row).
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

// Tags that can be made: the app adds the item and chooses it.
let tags = signal(known_tags());
let chosen = signal(Vec::<SharedString>::new());
combobox()
    .multiple(true)
    .creatable(true)
    .items(move || tags.get())
    .value(move || Some(chosen.get()))
    .on_change(move |value: &Vec<SharedString>| chosen.set(value.clone()))
    .on_create(move |query: &SharedString| {
        let value = SharedString::from(format!("tag-{}", query.to_lowercase()));
        tags.update(|t| t.push(SelectItem::new(value.clone(), query.clone())));
        chosen.update(|c| c.push(value));
    })
    .accessibility_label("Tags")
```

`nsh/base` exports a `Combobox` too; an app using both renames one.

## NSH gaps

1. **The input, the clear button's wrapper, the rows and the popup are `nsh/base`'s parts**, whose Rust setters take
   styles built once: their look is set as bindings on the part style props, and what depends on states only
   `nsh/base` knows (the clear button's hover, the rows' highlight) as classes made from the tokens.
2. **Chips wrap inside the prefix, not around the input.** `nsh/base` places the app's `prefix` as one element before
   the input, so the chips wrap among themselves within the room the input leaves, where a web combobox's input flows
   after the last chip on the same line. With few chips both look the same; with many, the input keeps a column at
   the end. A combobox root that laid its prefix's children and the input out in one wrapping row would flow the
   input after the chips.
3. **`invalid` on the input.** `nsh/base`'s `Combobox` takes no `invalid` and its input's accessibility state is
   `nsh/base`'s own (expanded, has a popup, disabled), so the library cannot add `invalid` to it: screen readers do
   not hear the combobox as invalid (the red edge shows). An `invalid` prop on `nsh/base`'s `Combobox` (as on its
   `Select`, which the library's trigger carries) would close it; the field's `FieldError` is still announced when it
   appears ([Field](field.md)).
4. **No Backspace to remove the last chip**, no arrow keys between chips (web comboboxes often have both):
   `nsh/base`'s combobox has neither. Chips are removed by their buttons or by choosing the item again in the list.
5. **No separators in the list** (Select's NSH gaps 3).
6. **No described-by** on `nsh/base`'s combobox (Field's NSH gaps 5).
7. **The list's maximum height** goes on `nsh/base`'s list part (the scroll view of the rows). The headless test
   passes: the popup shrinks to the capped list (350 with its edges) rather than keeping the room's height, and to its
   rows for a short list.
8. **No handle on the input.** `nsh/base`'s `Combobox` takes no ref for its input and has no focus method, so the
   library offers no `ComboboxRef`, a `Label`'s `htmlFor` cannot name a combobox, and a `FieldLabel`'s press reaches
   nothing (the combobox joins a field for its state only). An `inputRef`, or a `focus()` on `nsh/base`'s ref, would
   close it.
9. **No hit slop on `nsh/base`'s input part and clear button's wrapper**: their Rust props take styles, and a hit slop
   is no style. On touch screens the field and the clear button take presses in their own bounds (the field is 40
   tall at `md`); the chevron, the library's own view, has its slop. A `hitSlop` on those parts would close it.
10. **The query is not readable.** `nsh/base`'s `Combobox` tells a typed query (`onQueryChange`) only after its
    list filtered with it, and goes back to `defaultQuery` (opening, the list gone) without telling. Rows drawn from
    the query (the matched part, the create row) would lag a keystroke behind and miss the resets, and the create
    row could miss the highlight a query's only match gets. So the library holds the query and passes it controlled
    (Implementation). A read-only query on `nsh/base`'s side (or `onQueryChange` before filtering, and on resets)
    would let it leave the query to `nsh/base`.
11. **No row of the list's own.** `nsh/base`'s list shows `items` only (its `footer` is below the rows, out of the
    keys' reach), so the create row is an item the library adds, with a value of its own that the library keeps out
    of the value it reports. An action row in `nsh/base`'s list (a row after the items that the keys reach and Return
    runs, reported by an event of its own) would replace it.
12. **In a browser a press on the input does not open the list.** NSH's web host keeps a press that lands in a text
    field for the field itself (the caret, the selection) and reports no press for it, so `nsh/base`'s input never
    hears the press that opens and closes the list: on a desktop browser the press only focuses the input. Typing,
    Down or Up and the chevron button open it as elsewhere, and a touch screen's browser opens it on the tap's focus.
    A press reported by the web host to a text field that listens for one (the field still placing the caret) would
    close it.

## Decisions

Settled in the P3 review:

1. **Options are data, the value a list** (`nsh/base`'s): as Select's Decisions 1; one value is `[value]`.
2. **One popup look**: the popup surface and offset shared with Select, not a ring and a larger offset of the
   combobox's own.
3. **The buttons are concentric with the field** (`input-radius − 4`): their corners follow the field's.
4. **A "popup" combobox** (a button-like trigger showing the value, the search field inside the popup) is not in P3:
   noted in [select.md](select.md)'s "Later" as a future `searchable` on `Select`.
5. **No free-text mode.** Some web comboboxes keep typed text that matches no item; `nsh/base`'s chooses only
   items. An autocomplete field (suggestions under an Input) waits for a real need and `nsh/base` support. Making
   the typed text a new item is another thing, and offered: `creatable` (Decisions 9), where the app adds the item.
6. **Library words**: "No items found." (`empty`'s default), "Clear" and "Remove", added to
   `labels` (`Label::NoItems`, `Label::Clear`, `Label::Remove`), translatable as the others.
7. **One list look, one owner**: Select's list drawing moved to `foundation::menu` (`list_look!`), shared by Select
   and Combobox, and the group headings' tokens moved into the menu family as `menu-label-foreground` and
   `menu-label-font-size` (renamed from `select-label-*`).

Added with the match highlight, the create row and recent items:

8. **The matched part is emphasized by weight, its first match only.** A list of options says why each row is there
   by its matched letters, in a heavier weight (`menu-item-match-font-weight`, semibold), not a color: a color of
   its own would fight the highlighted row's and the disabled dimming, while weight keeps the row's color in every
   state, light and dark. The first match only: it is the reason the row shows, and every match of a short query
   speckles the titles. The reverse (the typed part plain, the rest bold) suits a list of completions of what is
   typed (search suggestions), not a list of options, and is not offered. It matches as the filter does (the same
   case folding, including folds that change a text's length, and nothing the filter would not match), so a row
   never shows a match the filter did not make. The splitting (`foundation::text_match`) is the library's, for any
   list with a query (Select's search later). The library's rows only: `renderItem` is the app's drawing.
9. **`creatable` and `onCreate`**: a flag and an event, not a part. Options are data (Decisions 1) and the row is the
   library's own, as the empty state's words are: a part would only carry its words, which `labels` translates. The
   row shows while no item's title is the query (ignoring case and the spaces at its ends: "react" does not create
   a second "React"), last, so the matches come first and Return on a query with matches takes the first match; alone
   it is highlighted, so Return creates. The app makes the item (its value, perhaps from a server), so `onCreate`
   carries the query, not a value, and the library changes no value itself. With `multiple` the query is emptied after
   a creation (the tag input's rhythm: type, Return, type the next); a choice of an existing item keeps it, as
   `nsh/base` does.
10. **Recent items are the app's items.** Nothing in the API stopped a recent group; what was missing was knowing when
    the query is empty, since the combobox's own query went back to `defaultQuery` silently. Its resets are now
    reported with `onQueryChange`, so an app gives a "Recent" group while the query it watches is empty, without
    holding the query itself. An app that sets `query` resets it itself (in `onOpenChange`), as before. A value is
    one row: the recent ones are left out of the group after them.
11. **The app's `empty` inherits the empty state's look**: its words take `combobox-empty-foreground` and 14 as the
    library's do (and as Select's `empty` does), so `empty={<Text>No time zone matches.</Text>}` reads as the same
    quiet note, not as a row's text. A `<Text>` with its own color or size keeps them. Before, the app's words were the
    popup's foreground at the inherited size.
