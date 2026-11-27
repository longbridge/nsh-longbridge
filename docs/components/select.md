# Select

Status: implemented (P2); `searchable` and `cleanable` added. One value chosen from a list: a trigger that shows the
chosen item (or a placeholder) and opens the list, optionally with a search field at its top and a clear button on the
trigger. The trigger is drawn by foundation's **field look** (the `input-*` tokens, as
[Input](input.md)); the list by the **menu look** (the `menu-*` tokens, owned by [DropdownMenu](dropdown-menu.md)).

## Usage

- A choice from a fixed set that a form submits: a country, a time zone, a currency, a theme, a sort order, a page
  size. More than about five options, or too little room for a `RadioGroup`.
- A long fixed set the user narrows by typing while the trigger stays a button showing the value (`searchable`): a
  country, a time zone, a person or a tag in a filter bar.
- An optional choice or a filter that can go back to "any" (`cleanable`): a clear button on the trigger.
- Search terms: select, dropdown, drop-down list, picker, pop-up button, combo box (not editable), option list,
  searchable select, filter dropdown, select with search, clearable select.

## When not to use

| Need | Use instead |
|---|---|
| Two to five options, all worth seeing at once | [RadioGroup](radio-group.md) |
| Typing into the field itself, several values, the app filtering the options (results from a server) | [Combobox](combobox.md) |
| Commands, not a value (the trigger does not show a choice) | [DropdownMenu](dropdown-menu.md) |
| A setting on or off | [Switch](switch.md) |
| Switching views of a page | [Tabs](tabs.md) |
| A date, a number with steps | DatePicker (P3), `nsh/base`'s `NumberInput` |

## Built on

`nsh/base`'s `Select` (`crates/nsh-base/src/select.rs`) and its list (`listbox.rs`, which `Combobox` shares). This
library adds the trigger it draws, the look of the list, its rows and headers, and `size` / `invalid`; all behavior is
`nsh/base`'s:

| Provided by `nsh/base` (not rewritten here) | |
|---|---|
| Value | `value` (controlled, `null` for none) / `defaultValue`, `onChange({ value })` only when it changes (choosing the chosen item again only closes the list) |
| Presentation | Desktops: a popup at the trigger, as wide as the trigger, never taller than the room on its side, its rows scrolling. On macOS it opens with the chosen row **over the trigger** (the system's pop-up button); elsewhere below it (Windows', Linux's and browsers' drop-down lists). Touch screens: a sheet at the bottom over a backdrop |
| Trigger keys | Down, Up, Return, Space open it (Down / Return / Space on the first enabled item when nothing is chosen, Up on the last); letters and digits choose by typeahead **without opening it**, as a native select |
| List keys | Up / Down (around the ends), Home / End, Page Up / Down by the rows in view, typeahead; Return or a press chooses and closes (`"select"`); Escape, back and a press outside close (`"cancel"`, `"pressOutside"`); Tab moves on and closes (`"focus"`) |
| Pointer | Moving over a row highlights it; desktops choose on press down |
| Highlight | The chosen row on opening, revealed; the row's `focus:` state while highlighted, `selected:` while chosen, `disabled:` |
| Focus | The trigger keeps the keyboard focus while the list is open (the highlighted row is its active descendant); closing gives it back to the trigger, except after a press outside or Tab |
| Sections | Adjacent items with the same `group` show under a header with that title |
| Search (`searchable`) | A text field above the rows filters them by title (a case-insensitive substring, in order; a section without matches goes); desktops focus it on opening, a touch screen's sheet waits for a tap (its keyboard comes when asked); while it edits, Up / Down / Page Up / Page Down move the highlight and Return chooses (Home / End stay the field's, Space types a space), the highlighted row being the field's active descendant; a new query highlights its first enabled match; each opening starts empty; the list's typeahead is off (the trigger's stays) |
| Empty | The `empty` slot while no row shows (a search that matches nothing, no items) |
| Clear button (`cleanable`) | A button after the trigger while a value is set (not part of the trigger, so a press on it never opens the list); a press reports `onChange({ value: null })` and leaves the list closed; not a stop of Tab |
| Rows | A virtualized list: thousands of items make only the rows in view |
| Control | `open` / `defaultOpen` / `onOpenChange({ open, reason })`, `onOpen`, `onClose` |
| Accessibility | The trigger is a `combobox` (`expanded`, `hasPopup`, its value the chosen title, else the placeholder); the list a `listbox`; rows `option` with `selected` and `disabled`; headers `header` |
| Terminal | A one-row trigger with `▾`, rows of titles with `✓`; this library's look does not apply there |

### Why the options are data

Some libraries compose the list from children (`Select > SelectTrigger > SelectValue`, `SelectContent > SelectGroup >
SelectLabel + SelectItem`, `SelectSeparator`). `nsh/base`'s `Select` takes its options as **data** (`items`: value,
title, disabled, group), because its list is virtualized and owns search, typeahead and highlight over the titles; a
component cannot read its children's props. So:

- `SelectItem` is the item's data (`nsh/base`'s type: `{ value, title, disabled?, group? }`), not a component; its
  row is drawn from it, or by `renderItem`.
- `SelectGroup` + `SelectLabel` are the items' `group`: the header shows the group's title.
- `SelectTrigger` + `SelectValue` are drawn by `Select` itself: the props `size`, `invalid`, `placeholder`,
  `triggerClass` / `triggerStyle`.
- `SelectContent`'s props are `Select`'s (`placement`, `align`, `cover`, `contentClass`).
- No `SelectSeparator` (NSH gaps 3) and no `SelectScrollUpButton` / `SelectScrollDownButton`: the list scrolls with
  the wheel, the trackpad, a finger and the keys, as the desktops' lists do.

### Implementation

- **Structure**: a `#[component] fn Select` whose root is `nsh_base::select()`, `items`, `open`, `defaultOpen`,
  `placeholder`, `disabled`, `accessibilityLabel`, `placement`, `align` and `cover` passed through; `offset` is
  `popover-offset`, `margin` `popover-margin` (4) from the safe area's edges, as Popover's.
- **The value**: the library always passes `nsh/base` a controlled `value`, its own signal when the app gives none
  (from `defaultValue`), so the trigger's text and the rows' checks read it. `onChange` is `nsh/base`'s, forwarded;
  the own signal follows it first.
- **The trigger** is a view of this library's filling `nsh/base`'s `trigger` slot: `nsh/base` binds its role, value,
  states, focus, keys and press on it, and places the list against it. The library draws it with `field_look` (edge,
  fill, shadow, focus ring, invalid, disabled), `input-height-<size>`, `input-padding-x-<size>`, the value or the
  placeholder and the chevron.
- **The rows**: each row is `nsh/base`'s view (it carries the `option` role, the presses and the `focus:` /
  `selected:` / `disabled:` states). Its geometry and text are a bound `itemStyle` (the item look, the app's
  `itemStyle` merged after it); its highlight is an `itemClass` the library generates in a binding from the resolved
  `menu-item-focus` / `menu-item-focus-foreground` (`focus:` and `active:`, each appearance's value under `dark:`),
  the app's `itemClass` appended (NSH gaps 1). Its content is the library's `renderItem`: the title (or the app's
  `renderItem`, which it calls) and the check, shown while the row's value is the current value (a binding on the
  value, so moving the highlight or choosing draws nothing again, as `nsh/base` intends). The row lays its one child
  across its width (a column), and the content lays out the title and the check; a disabled item's content is at
  `opacity-disabled` (a disabled row is never highlighted, so the row itself needs no state).
- **The headers**: `nsh/base`'s (the group's title), drawn through a bound `groupStyle` (the app's `labelStyle` merged
  after it) and `groupClass` (the app's `labelClass`).
- **The popup and the sheet**: `foundation::popup::surface(t)` (Popover's, shared by every popup) on `nsh/base`'s
  `popupClass` / `popupStyle`, set as a bound prop as DropdownMenu's (its NSH gaps 1, the same accepted limits), so it
  follows the tokens while open; `listStyle` (bound too) gives the list the menu's padding (`menu-padding`;
  `menu-sheet-padding` in the sheet, whose panel has none but the bottom safe area). The sheet is `!is_desktop()`
  here (`popup::dropdown`), unlike a menu's, which stays a popup on an iPad in a regular width.
- **Text styles and scope**: as DropdownMenu's: the popup resets inherited text styles and reads the `ThemeScope`
  around the `Select`.
- **The search field** (`searchable`): `nsh/base`'s input above the rows, its look a bound `searchStyle` (the app's
  `searchStyle` merged after it): a band across the surface, as tall as the trigger (a row's height in the sheet), the
  text inset to line up with the rows' titles, the menu's row text, no fill or edge of its own and a `menu-separator`
  line under it. No focus ring: it is the only place the keys go while the list is open. Its placeholder and the name
  screen readers hear are `searchPlaceholder`, else the library's "Search" (`labels`, translatable). On desktops the
  list is capped at `select-list-max-height` while searchable, so the popup does not span the window and then shrink
  under the pointer as the query narrows it; and it opens on a side of the trigger unless the app sets `cover`
  (Decisions 6).
- **Empty**: `nsh/base`'s `empty` slot, holding the app's `empty` or the library's "No items found." (`labels`, as
  Combobox's), centered, in `select-empty-foreground` at 14, which the app's `empty` inherits.
- **The clear button** (`cleanable`): `nsh/base`'s button, a sibling after the trigger in the root. The library places
  it over the trigger's end (absolute, `end` and `top` inset 4, logical so it follows a right-to-left layout), a square
  of the trigger's height less 8 with concentric corners, the ghost button's hover fill as a class made from the tokens
  (only `nsh/base` knows the button's states), and fills its content slot with the `x` icon in
  `select-clear-foreground` (`iconClass` / `iconStyle` reach it, as the chevron). Combobox's cross, drawn the same way.
  While it shows, the chevron stays in place but transparent, so the value's room does not change and the cross takes
  the chevron's place. It is dimmed with the trigger while disabled. Its name is the library's "Clear".
- **After a clear** the keyboard focus goes to the trigger on desktops and while a screen reader runs (the button it
  was on is gone); a touch screen's focus stays where it was.
- **Delete and Backspace** on the focused trigger, while the list is closed and a value is set, clear it as the button
  does (`cleanable` only): the keyboard's way to the button, which is no stop of Tab.

## API

Coming from a part-based API: `<SelectTrigger className size>` + `<SelectValue placeholder>` are `Select`'s
`triggerClass`, `size` and `placeholder`; `<SelectContent position align>` is `cover` and `align`; each `<SelectItem
value>Title</SelectItem>` is an entry `{ value, title }` of `items`, and a `<SelectGroup>` with its `<SelectLabel>` is
those entries' `group`; `onValueChange` is `onChange`.

### Props

| TSX | Rust | Type | Default | Description |
|---|---|---|---|---|
| `items` | `items` | `SelectItem[]` / `Vec<SelectItem>` | `[]` | The options, in order: `{ value, title, disabled?, group? }` (`nsh/base`'s type) |
| `value` | `value` | `string \| null` / `SelectValue` | Uncontrolled | The chosen item's `value`, `null` for none; the app sets it in `onChange` |
| `defaultValue` | `default_value` | `string` | — | The first value while uncontrolled |
| `placeholder` | `placeholder` | `string` | — | What the trigger shows while nothing is chosen (a `SelectValue placeholder`) |
| `size` | `size` | `Size` | `"md"` | The trigger's height, type, padding and icon, and the rows'; see Variants and sizes |
| `disabled` | `disabled` | `boolean` | `false` | It does not open, takes no focus, is dimmed |
| `invalid` | `invalid` | `boolean` | `false` | The app's checks reject the value: the field look's red edge and ring; screen readers say invalid |
| `open` / `defaultOpen` | `open` / `default_open` | `boolean` | Uncontrolled / `false` | The list's open state |
| `placement` | `placement` | `Placement` | `"bottom"` | Desktops, below-the-trigger form: the side |
| `align` | `align` | `Align` | `"start"` | Desktops, that form: the alignment along the side |
| `cover` | `cover` | `boolean` | The platform's | Desktops: `true` opens with the chosen row over the trigger, `false` on a side; unset: over it on macOS, on a side elsewhere |
| `searchable` | `searchable` | `boolean` | `false` | A search field at the top of the list filters the options by title |
| `searchPlaceholder` | `search_placeholder` | `string` | "Search" (`labels`) | The search field's placeholder, which screen readers also hear as its name ("Search countries") |
| `empty` | `empty` | element (slot) | "No items found." (`labels`) | What the list shows while no option does (a search that matches nothing) |
| `cleanable` | `cleanable` | `boolean` | `false` | A clear button on the trigger while a value is set; a press reports `onChange({ value: null })` |
| `renderItem` | `render_item` | `(entry: SelectEntry) => element` / `Renderer<SelectEntry>` | — | A row's content in place of its title (an icon and a title, a flag); the library still draws the row and its check. `entry` is what the item is (`value`, `title`, `disabled`, `index`, `group`), never its state |
| `accessibilityLabel` | `accessibility_label` | `string` | — | What screen readers call it (what the choice is about); set it even beside a `Label` (Input's G1) |
| `class` / `style` | `class` / `style` | | | The root, the box the trigger fills: width and layout (`w-[180px]`, `w-full`) |
| `triggerClass` / `triggerStyle` | `trigger_class` / `trigger_style` | | | The trigger, merged over the field look |
| `valueClass` / `valueStyle` | `value_class` / `value_style` | | | The trigger's text (the title or the placeholder) |
| `iconClass` / `iconStyle` | `icon_class` / `icon_style` | | | The trigger's chevron and the clear button's cross |
| `searchClass` / `searchStyle` | `search_class` / `search_style` | | | The search field: `searchStyle` (text styles too: `color`, `fontSize`, `placeholderColor`) is merged after the library's look and wins |
| `contentClass` / `contentStyle` | `content_class` / `content_style` | | | The popup or sheet surface, as DropdownMenu's: `contentStyle` wins over the library's look, `contentClass` only adds to it |
| `itemClass` / `itemStyle` | `item_class` / `item_style` | | | Each row: `itemStyle` is merged after the item look and wins; `itemClass` follows the library's highlight classes (NSH gaps 1) |
| `labelClass` / `labelStyle` | `label_class` / `label_style` | | | Each group header |

Not exposed, though `nsh/base` has them: `searchAccessibilityLabel` (the placeholder is the name, Decisions 8),
`searchTestID`, `clearButton` / `clearButtonClass` / `clearButtonStyle` / `clearButtonAccessibilityLabel` (the
library draws and names the button), `listClass`, `renderGroup` (the header is the group's title).

### Events

| TSX | Rust (`SelectEvent`) | Payload | When |
|---|---|---|---|
| `onChange` | `Change(Option<SharedString>)` | `{ value: string \| null }` | The user chose another item (a press, Return, typeahead on the closed trigger), or cleared it (`null`: the clear button, Delete or Backspace) |
| `onOpenChange` | `OpenChange(open, reason)` | `{ open, reason }` | The user asks to open or close the list |
| `onOpen` / `onClose` | `Open` / `Close` | — | The list is all there / all gone |

`nsh/base`'s events, forwarded; `onChange` is NSH's name (conventions), not `onValueChange`.

### Ref

TSX: `ref={country}` gives a `SelectRef`; Rust: a `ComponentRef<Select>`, whose `api::<SelectApi>()` has the method.
A [Label](label.md)'s `htmlFor` takes it: a press on the label focuses the trigger, as a browser's label does.

| Method | Notes |
|---|---|
| `focus()` | Moves the keyboard focus to the trigger, the list staying closed; does nothing while `disabled` |

### Inside a `Field`

A [Field](field.md) around it hands it its `invalid` and `disabled` (or'ed with its own) and its error or description,
which describes the trigger for screen readers; its `FieldLabel` focuses the trigger. Outside a `Field` nothing changes.

### Parts

| Part | What | Customize with |
|---|---|---|
| root | `nsh/base`'s wrapper: the box the trigger fills | `class` / `style` |
| trigger | The field: value, chevron | `triggerClass` / `triggerStyle` |
| value | The chosen title or the placeholder | `valueClass` / `valueStyle` |
| icon | The `chevron-down`, and the clear button's `x` | `iconClass` / `iconStyle` |
| clear | The clear button over the trigger's end (`cleanable`) | its icon: `iconClass` / `iconStyle` |
| content | The popup or sheet | `contentClass` / `contentStyle` |
| search | The search field at the top of the list (`searchable`) | `searchClass` / `searchStyle` |
| empty | What the list shows while no option does | `empty` |
| item | Each row | `itemClass` / `itemStyle` |
| label | Each group header | `labelClass` / `labelStyle` |

## Variants and sizes

No variants.

### The trigger

In classes: `flex w-fit items-center justify-between gap-2 rounded-md border border-input bg-transparent px-3 py-2
text-sm whitespace-nowrap shadow-xs data-[placeholder]:text-muted-foreground dark:bg-input/30 dark:hover:bg-input/50`,
the value `line-clamp-1`, a `ChevronDownIcon size-4 opacity-50` (`text-muted-foreground`), and the field's focus,
invalid and disabled classes.

| `size` | Height (pointer / touch) | Text (pointer / touch) | Padding | Chevron |
|---|---|---|---|---|
| `xs` | 24 / 32 | 12 / 14 | 8 | 12 |
| `sm` | 28 / 36 | 13 / 14 | 10 | 14 |
| `md` | 32 / 40 | 14 / 16 | 12 | 16 |
| `lg` | 36 / 48 | 14 / 16 | 12 | 16 |

- Height, text and padding are Input's (`input-height-<size>`, `input-font-size-<size>`, `input-padding-x-<size>`), so a
  Select and an Input of a `size` line up in a row: 32 at `md`, 36 at `lg`, as every control here.
- A row: the value (one line, cut with an ellipsis, `flex: 1`, left-aligned), then the chevron (`icon-size-<size>`,
  `select-icon-foreground`), `spacing × 2` (8) apart.
- The value's color is `input-foreground`; the placeholder's `input-placeholder-foreground`.
- Width: as wide as its content, `w-fit` (a column that stretches its children does not widen it, and a row's
  `items-center` still centers it); the app sets one with `class` (`w-[180px]`, `w-full`), which goes over `w-fit` and
  the trigger fills.

### The list

The menu look (DropdownMenu's "The menu look"), with a select's own parts:

| Part | Layout | Classes |
|---|---|---|
| Surface | The popup surface (`foundation::popup::surface`: `popover`, `popover-border`, `popover-radius`, `popover-shadow` 3, `md`); padding `menu-padding` (4) inside the scrolling list; as wide as the trigger, at least `menu-min-width` (128) | `rounded-md border bg-popover shadow-md min-w-[8rem]`, viewport `p-1`, at least the trigger's width |
| Row | Height `menu-item-height-<size>`; padding `menu-item-padding-x` (8) at the start; the title (one line, ellipsis) `flex: 1`; then a box `icon-size-<size>` wide holding the `check` while chosen, `menu-item-gap` (8) after the title and `menu-item-padding-x` before the end (32 at `md`, `pr-8`); corners `menu-item-radius`; text `menu-item-font-size-<size>` | `py-1.5 pr-8 pl-2 text-sm rounded-sm`, the check `absolute right-2 size-3.5`, `CheckIcon size-4` |
| Check | `icon-size-<size>`, the row's text color | — |
| Group header | Height `menu-item-height-<size>`, padding `menu-item-padding-x` at the sides, `menu-label-font-size` (12), `menu-label-foreground` (the menu family's) | `px-2 py-1.5 text-xs text-muted-foreground` |
| Distance | `popover-offset` (4) from the trigger when on a side; none over it | `popper`: `translate-y-1` |

The row heights and concentric row corners are DropdownMenu's; there are no separators or scroll buttons.

### The search field and the empty state

| Part | Layout | Classes |
|---|---|---|
| Search field | A band across the surface above the rows (which scroll under it): height `input-height-<size>` (32 at `md`; on a touch screen `menu-item-height-<size>`, 44); text inset `menu-padding` + `menu-item-padding-x` (12) at both sides, so it starts where the rows' titles do (`menu-sheet-padding` + `menu-item-padding-x` in the sheet); text `menu-item-font-size-<size>` in `popover-foreground`, placeholder `input-placeholder-foreground`; no fill, no edge, no ring; a `border-width` line in `menu-separator` under it | `flex h-9 items-center gap-2 border-b px-3`, the input `h-10 w-full bg-transparent py-3 text-sm outline-hidden placeholder:text-muted-foreground` |
| List | While searchable, at most `select-list-max-height` (348) tall on desktops; the sheet keeps its own height | `max-h-[300px]` |
| Empty | Centered words, 14 (`text-sm`), `select-empty-foreground`, `spacing × 3` (12) from the search field's line (the empty list's own padding counted) and from the surface's edge; the app's `empty` inherits the color and size | `py-6 text-center text-sm` |

There is no search icon at the field's start: `nsh/base` gives its search field no slot beside it (NSH gaps 6).

### The clear button

| `size` | Button (pointer / touch) | Cross |
|---|---|---|
| `xs` | 16 / 24 | 12 |
| `sm` | 20 / 28 | 14 |
| `md` | 24 / 32 | 16 |
| `lg` | 28 / 40 | 16 |

A square of the trigger's height less 8 at its end, inset 4 from the edge at the end, above and below; corners
`input-radius` less 4 (concentric); the `x` at `icon-size-<size>` in `select-clear-foreground` (`muted-foreground`,
Combobox's cross: a button of its own, stronger than the chevron's mark); under the pointer the ghost button's fill
(`button-ghost-hover`), none while pressed. The chevron, transparent while the cross shows, keeps its place, so the
value is cut at the same width with or without the cross.

### Motion

Popover's (every popup's): fade and zoom from 95% over `duration`, through `nsh/base`'s state looks, none with reduced
motion, `popover-animate` 0 to turn it off. `Select` has no `onPosition`, so no slide and a centered origin (Tooltip's gap 3);
over the trigger (macOS) no slide is wanted anyway. The sheet has `nsh/base`'s motion.

## States

The trigger takes the field look's states (Input's table): rest, focus-visible (`input-border-focus` edge and the
`input-ring` ring), invalid, invalid + focus-visible, disabled (`opacity-disabled`, the arrow cursor). In addition:

| State | Look | Classes |
|---|---|---|
| Hover (pointer) | Fill `select-trigger-hover` (dark only: `input` at 50%; light unchanged) | `dark:hover:bg-input/50` |
| Placeholder shown | Value text in `input-placeholder-foreground` | `data-[placeholder]:text-muted-foreground` |
| Open (`expanded:`) | Edge `input-border-focus`, no ring (Decisions 2) | none |
| Row highlighted (`focus:`) | Fill `menu-item-focus`, text `menu-item-focus-foreground` | `focus:bg-accent focus:text-accent-foreground` |
| Row chosen (`selected:`) | The check shown; otherwise as at rest | `ItemIndicator` |
| Row disabled | `opacity-disabled`, never highlighted | `data-[disabled]:opacity-50` |
| Row pressed on touch (`active:`) | As highlighted | — |
| Clear button under the pointer | Fill `button-ghost-hover` (the cross unchanged) | `hover:bg-accent` |
| Clear button pressed | No fill | — |
| Clear button, select disabled | Dimmed with the trigger (`opacity-disabled`), no hover, no press | — |
| Search matches nothing | The empty words in place of the rows | — |

The trigger's colors, edge and ring transition over `duration` (the field look's); the rows' highlight changes at once.
The cursor is the arrow on the trigger and the rows (a select is not text).

## Pointer and touch

| | Desktops and desktop browsers | Phones and tablets |
|---|---|---|
| Trigger height | `input-height-<size>`: 24 / 28 / 32 / 36 | 32 / 36 / 40 / 48, and below `touch-target` a vertical hit slop of (44 − height) / 2, as Input |
| Focus | The edge and the ring | The edge only |
| List | A popup: over the trigger on macOS, below it elsewhere | A sheet at the bottom, the menu's sheet look (`menu-sheet-radius`, `menu-sheet-padding`, the `overlay` backdrop) |
| Row height | `menu-item-height-<size>`: 20 / 24 / 28 / 32 | 44 / 44 / 44 / 48, full width |
| Check | At the row's end | At the row's end (iOS's lists mark the choice there too) |
| Search field | Focused on opening: typing goes to it at once | At the top of the sheet, focused by a tap (the keyboard comes when asked, as `nsh/base` does: a keyboard over half the sheet on every opening would hide the rows) |
| Clear button | 16 / 20 / 24 / 28, no Tab stop; Delete or Backspace on the focused trigger | 24 / 28 / 32 / 40, presses in its own bounds (NSH gaps 7) |

## Accessibility

`nsh/base`'s (see Built on): the trigger is a `combobox` whose value is the chosen title (or the placeholder), the list a
`listbox`, rows `option`s with `selected` / `disabled`, headers `header`s; the highlighted row is the trigger's active
descendant, and a key that moves the highlight moves the screen reader's focus to it. The library adds `invalid` to the
trigger's accessibility state. The check and the chevron are hidden from screen readers. `accessibilityLabel` names the
select (Input's G1: no labelledby between elements).

- **The search field** is a text field named by `searchPlaceholder` (else "Search"); it keeps the focus while the keys
  move the highlight, the highlighted row being its active descendant (where a screen reader does not follow active
  descendants, `nsh/base` announces the row's title instead, so the field keeps the focus). It controls the listbox.
- **The empty words** are plain text in the list's place, read as the user moves to them; nothing is announced as the
  query changes (NSH has no polite live region on a popup's text yet; Combobox's empty state is the same).
- **The clear button** is a `button` named "Clear" (`labels`, translatable), reached by a screen reader's own
  navigation; it is no stop of Tab (a select is one stop, as a native one). After a clear the focus goes to the
  trigger, which says the placeholder. Delete or Backspace on the focused trigger clears it from the keyboard.

## Tokens

The trigger reads the `input-*` tokens ([Input](input.md#tokens)) and the rows and surface the `menu-*` tokens
([DropdownMenu](dropdown-menu.md#tokens)); a theme that changes `input-border` or `menu-item-focus` changes Select with
them. Its own tokens are what a select adds. The table is generated (`<!-- tokens:select -->`).

<!-- tokens:select -->
| Token | Type | Derived default | Value |
|---|---|---|---|
| `select-trigger-hover` | color | `light_dark(input-background, input × 50%)` |  |
| `select-icon-foreground` | color | `muted-foreground × 50%` |  |
| `select-clear-foreground` | color | `muted-foreground` |  |
| `select-empty-foreground` | color | `muted-foreground` |  |
| `select-list-max-height` | length | `spacing × 87` | 348 |
<!-- /tokens -->

- `select-icon-foreground` is `text-muted-foreground opacity-50` as one color.
- The group headers read the menu family's `menu-label-*` ([DropdownMenu](dropdown-menu.md#tokens)); the list is drawn
  by `foundation::menu`'s list look, which `Combobox` shares.
- A select's own `select-radius` or `select-height-*` are not declared: the trigger is a field, and fields share one set
  of tokens; a different select uses `triggerClass` or a scope.
- `select-clear-foreground`, `select-empty-foreground` and `select-list-max-height` have Combobox's values
  (`combobox-icon-foreground`, `combobox-empty-foreground`, `combobox-list-max-height`), so a searchable select and a
  combobox open the same list. The search field takes the menu's row text, `menu-separator` and
  `input-placeholder-foreground`, and adds no tokens.

## Examples

```tsx
import { signal, Text, View } from "nsh";
import { Icon, Select, type SelectItem } from "nsh-longbridge";

const fruits: SelectItem[] = [
  { value: "apple", title: "Apple", group: "Fruits" },
  { value: "banana", title: "Banana", group: "Fruits" },
  { value: "blueberry", title: "Blueberry", group: "Fruits" },
  { value: "grapes", title: "Grapes", group: "Fruits", disabled: true },
  { value: "carrot", title: "Carrot", group: "Vegetables" },
];

const fruit = signal<string | null>(null);

<Select
  class="w-[180px]"
  items={fruits}
  value={fruit()}
  onChange={({ value }) => fruit.set(value)}
  placeholder="Select a fruit"
  accessibilityLabel="Fruit"
/>

// Uncontrolled, small, invalid until chosen.
<Select size="sm" items={timeZones} defaultValue="utc" accessibilityLabel="Time zone" />
<Select items={plans} value={plan()} invalid={plan() === null} onChange={({ value }) => plan.set(value)} placeholder="Choose a plan" accessibilityLabel="Plan" />

// Rows with an icon: the library still draws the row and the check.
<Select
  items={currencies}
  value={currency()}
  onChange={({ value }) => currency.set(value)}
  renderItem={(entry) => (
    <View class="flex-row items-center gap-2">
      <Icon name={flagOf(entry.value)} />
      <Text class="truncate">{entry.title}</Text>
    </View>
  )}
  accessibilityLabel="Currency"
/>
```

Many options in groups, which is why options are data: the list makes only the rows in view, and its typeahead and
highlight run over the titles, so a select of every time zone stays cheap to open. As children, all of them would be
made on each opening and the list could not read their titles.

```tsx
// Hundreds of items, grouped by region: adjacent items with the same `group` show under one header.
// `timeZoneIds`: the app's list of IANA ids ("Europe/Berlin", ...).
const zone = signal<string | null>(null);
const zones: SelectItem[] = timeZoneIds.map((id) => ({
  value: id,
  title: id.split("/").slice(1).join(" / ").replaceAll("_", " "),
  group: id.split("/")[0],
}));

<Select class="w-[280px]" items={zones} value={zone()} onChange={({ value }) => zone.set(value)} placeholder="Select a time zone" accessibilityLabel="Time zone" />

// The same list with a search field at its top: the trigger stays a button showing the zone.
<Select
  class="w-[280px]"
  items={zones}
  value={zone()}
  onChange={({ value }) => zone.set(value)}
  searchable
  searchPlaceholder="Search time zones"
  empty={<Text>No time zone matches.</Text>}
  placeholder="Select a time zone"
  accessibilityLabel="Time zone"
/>
```

A filter that can go back to "any": a clear button while a value is set.

```tsx
const owner = signal<string | null>(null);

<Select
  size="sm"
  items={people}
  value={owner()}
  onChange={({ value }) => owner.set(value)}
  searchable
  searchPlaceholder="Search people"
  cleanable
  placeholder="Owner"
  accessibilityLabel="Owner"
/>
```

With NSH 0.7.0, a string child (`empty`'s `<Text>`) is written as shown ([Button's G0](button.md#nsh-gaps)).

```rust
use nsh_core::prelude::*;
use nsh_longbridge::{select, SelectItem, Size};

let fruit = signal(None::<SharedString>);

select()
    .app_class(Some("w-[180px]".into()))
    .items(vec![
        SelectItem::new("apple", "Apple").group("Fruits"),
        SelectItem::new("banana", "Banana").group("Fruits"),
        SelectItem::new("carrot", "Carrot").group("Vegetables"),
    ])
    .value(move || fruit.get())
    .on_change(move |value: &Option<SharedString>| fruit.set(value.clone()))
    .placeholder("Select a fruit")
    .size(Size::Sm)
    .accessibility_label("Fruit")
```

```rust
// Searchable and cleanable.
select()
    .items(countries)
    .value(move || country.get())
    .on_change(move |value: &Option<SharedString>| country.set(value.clone()))
    .searchable(true)
    .search_placeholder("Search countries")
    .cleanable(true)
    .placeholder("Any country")
    .accessibility_label("Country")
```

`SelectItem` and `SelectEntry` are `nsh/base`'s types, re-exported by this library (with `SelectValue`, the `value`
prop's type). The library's generated TypeScript declarations declare `SelectItem` and `SelectEntry` themselves (the
same interfaces as `nsh/base`'s), so a TSX app imports them from `nsh-longbridge`.

## NSH gaps

1. **The rows and the popup are `nsh/base`'s parts** (accepted limits, as DropdownMenu's gap 1). The row (its role,
   presses and `focus:` / `selected:` states) and the popup surface are `nsh/base`'s views, styled through `itemStyle` /
   `popupStyle`, whose Rust setters build a style once. Workaround: the library writes those props itself, bound
   (DropdownMenu's gap 1), so they follow the tokens and merge the app's `itemStyle` / `contentStyle`. The library's
   look is an inline style, which wins over the app's `contentClass`: an app changes the surface with tokens or
   `contentStyle` (merged after it). ★ Checked: the shadow (two layers of tier 3) reaches the popup and nothing around
   it clips it. An inline style has no states, and nsh-core's typed styles have no `group_focus` / `group_selected` /
   `group_disabled` (only `group_active`, `group_hover`, `group_focus_within`, `group_focus_visible`), so neither
   `itemStyle` nor the library's row content can take the row's highlight. Workaround: the highlight is an
   `itemClass` the library generates in a binding from the resolved tokens (`focus:bg-[…] dark:focus:bg-[…]
   focus:text-[…]`, `active:` the same; a scope's tokens and forced appearance reach it, checked), the app's
   `itemClass` appended (its classes follow the library's, so an app's `focus:` fill wins); the check is bound to the
   value the library holds. Named `group_focus_named` / `group_selected_named` / `group_disabled_named` in nsh-core, or
   states in an inline style, would let the library draw the highlight without generated classes.
2. **The trigger shows the title only**, not the chosen item's children (an icon too).
   `nsh/base` gives the trigger nothing but the value; the library shows the chosen item's title. A `renderValue`
   would follow `renderItem` later, if asked for.
3. **No separators in the list**: `nsh/base`'s rows are items and headers; groups are split by their headers only.
   Separators between groups are rarely needed; a separator row kind in `listbox.rs` would add it.
4. **String children** do not apply (titles are data); `renderItem`'s elements follow Button's G0 in TSX.
5. **The list is exactly as wide as the trigger** on desktops: `nsh/base` gives the popup the trigger's width, which
   `menu-min-width` only raises, so rows wider than the trigger (`renderItem`'s, a long title) are cut or truncated,
   where a list should be at least the trigger's width and grow with its rows. Workaround: a trigger wide
   enough for the rows (`class`). A width mode in `nsh/base` (at least the trigger's, else its content's) would allow
   it. A searchable list in a narrow filter-bar trigger feels it most: its search field is as narrow as the trigger.
6. **No search icon in the search field**: `nsh/base`'s search field is a bare input placed directly in the popup,
   with only classes, a style and a placeholder; the popup takes no other content of the app's above the rows. A
   magnifier at the field's start (a search field's usual mark) needs a slot beside the field (`searchPrefix`) or a
   header part above the rows in `nsh/base`'s `Select`. Until then the placeholder ("Search") says what the field is.
7. **No hit slop on the clear button**: it is `nsh/base`'s view, whose Rust props take classes and styles, and a hit
   slop is no style (Combobox's gap 9). On touch screens it takes presses in its own bounds (32 at `md`).
8. **Escape cannot clear the query first**: the query is `nsh/base`'s own (Select takes no `query`), and Escape goes to
   the popup's dismissal. Escape closes the list, and the next opening starts with an empty field (Decisions 7).
9. **The rows cannot show the matched text**: the query is the search field's own inside `nsh/base` (Select takes no
   `query`, reports no `onQueryChange`, and `renderItem` gets the item alone), so the library's rows never learn what
   was typed. Combobox draws the part of a title its query matches in `menu-item-match-font-weight` through the shared
   `foundation::menu` row (Combobox's Decisions 8); Select's rows would take the same row as soon as `nsh/base` gives
   them the query, by either: a controlled `query` with `onQueryChange` on `Select` (as Combobox's), or the current
   query passed to `renderItem` with the item. Until then Select's rows show their titles plain.

## Decisions

1. **Options are data (`items`), not `SelectItem` children**, because `nsh/base`'s list is virtualized and owns search,
   typeahead and highlight over titles (see the time-zone example). The name `SelectItem` is the data type (`nsh/base`'s), not a component.
2. **The open trigger keeps the focus edge** (`input-border-focus` while `expanded`): gpui-kit's Design Guides ask for
   persistent feedback while an attached popup is open. The library-wide decision (DropdownMenu's
   3: a `Button` trigger keeps its hover fill).
3. **macOS opens the list over the trigger, other desktops below it** (`nsh/base`'s default, `cover`): the
   platforms' own habit wins.
4. **Searchable Select beside Combobox**: both filter one list by title. A searchable select keeps a trigger that is a
   button showing the value (it opens a list, the search is inside the list: a filter bar's "Owner", a form's
   country); a combobox is a field typed into (the query is what it shows), takes several values and lets the app
   filter. In P2 Select had neither search nor a clear button; they are `nsh/base`'s, given the library's look here.
5. **As wide as its content through `w-fit`, not `align-self: flex-start`**: an `align-self` of its own would override
   the cross-axis alignment of a row it sits in (a filter bar), putting it at the top instead of the middle (as
   `Badge`'s decision 3).
6. **A searchable list opens on a side of the trigger, also on macOS** (unless `cover` is set): over the trigger, the
   search field would sit wherever the chosen row puts it and the rows under the pointer would move as the query
   narrows them; a field typed into belongs at a fixed place under the trigger, as a combobox's list. It is also
   capped (`select-list-max-height`), for the same reason. An unsearchable select is unchanged.
7. **Escape closes the list, even with a query typed**: the list is a popup, and every popup here closes on the first
   Escape (a menu with a search field, a command palette and a combobox's list do the same); a search field that is a
   page's own control clears on Escape, but this one only exists while the list is open, and each opening starts empty,
   so nothing typed outlives the close. `nsh/base` owns both the query and Escape (NSH gaps 8).
8. **The placeholder names the search field**: it is the only words the field shows, and what a screen reader would
   fall back to; an app that writes "Search countries" names the field and hints it at once. No separate
   `searchAccessibilityLabel`.
9. **The clear button is no Tab stop; Delete and Backspace clear the focused trigger**: a select stays one stop
   (`nsh/base`'s and native selects' habit, gpui-component's `tab_stop(false)`), and the keyboard still reaches the
   action. The keys act only while the list is closed and only with `cleanable` (a select that cannot be emptied
   ignores them), and never while a text field edits.
10. **The cross takes the chevron's place**, as Combobox's: one control at the trigger's end at a time, the chevron kept
    transparent so the value is cut at the same width. A select whose value can be cleared still opens on a press of the
    trigger, the cross only on its own square.
11. **After a clear the focus goes to the trigger** on desktops and with a screen reader: the button it was on is gone,
    and the trigger says the placeholder, which confirms the clear. A touch screen's focus stays, as a tap on a
    button's does.
12. **No highlighting of the matched text in the rows yet**: Combobox's highlight is a shared `foundation::menu` row
    that needs the query, which `nsh/base`'s Select keeps to itself (NSH gaps 9). The library reads no query any other
    way (from the field's keys or text), so Select's rows wait for `nsh/base` and then take Combobox's row, its token
    and its look unchanged.
13. **The empty words show in every empty list**, searchable or not: a select given no items used to open an empty
    surface, which reads as broken; it now says "No items found." (or the app's `empty`). No API changes.

## Later

- **The matched text highlighted in the rows** while searching, as Combobox's, once `nsh/base` gives the rows the
  query (NSH gaps 9, Decisions 12).
- **A search icon** at the field's start, once `nsh/base` gives the field a slot (NSH gaps 6).
