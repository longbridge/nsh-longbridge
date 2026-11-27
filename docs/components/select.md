# Select

Status: implemented (P2). One value chosen from a list: a trigger that shows the chosen item (or a
placeholder) and opens the list. The trigger is drawn by foundation's **field look** (the `input-*` tokens, as
[Input](input.md)); the list by the **menu look** (the `menu-*` tokens, owned by [DropdownMenu](dropdown-menu.md)).
Reference: shadcn/ui new-york v4 `select.tsx` (Radix Select).

## Usage

- A choice from a fixed set that a form submits: a country, a time zone, a currency, a theme, a sort order, a page
  size. More than about five options, or too little room for a `RadioGroup`.
- Search terms: select, dropdown, drop-down list, picker, pop-up button, combo box (not editable), option list.

## When not to use

| Need | Use instead |
|---|---|
| Two to five options, all worth seeing at once | [RadioGroup](radio-group.md) |
| Searching or typing a value, many options, several values | Combobox (P3) |
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
| Presentation | Desktops: a popup at the trigger, as wide as the trigger, never taller than the room on its side, its rows scrolling. On macOS it opens with the chosen row **over the trigger** (the system's pop-up button; shadcn's default `position="item-aligned"`); elsewhere below it (Windows', Linux's and browsers' drop-down lists; shadcn's `position="popper"`). Touch screens: a sheet at the bottom over a backdrop |
| Trigger keys | Down, Up, Return, Space open it (Down / Return / Space on the first enabled item when nothing is chosen, Up on the last); letters and digits choose by typeahead **without opening it**, as a native select |
| List keys | Up / Down (around the ends), Home / End, Page Up / Down by the rows in view, typeahead; Return or a press chooses and closes (`"select"`); Escape, back and a press outside close (`"cancel"`, `"pressOutside"`); Tab moves on and closes (`"focus"`) |
| Pointer | Moving over a row highlights it; desktops choose on press down |
| Highlight | The chosen row on opening, revealed; the row's `focus:` state while highlighted, `selected:` while chosen, `disabled:` |
| Focus | The trigger keeps the keyboard focus while the list is open (the highlighted row is its active descendant); closing gives it back to the trigger, except after a press outside or Tab |
| Sections | Adjacent items with the same `group` show under a header with that title |
| Rows | A virtualized list: thousands of items make only the rows in view |
| Control | `open` / `defaultOpen` / `onOpenChange({ open, reason })`, `onOpen`, `onClose` |
| Accessibility | The trigger is a `combobox` (`expanded`, `hasPopup`, its value the chosen title, else the placeholder); the list a `listbox`; rows `option` with `selected` and `disabled`; headers `header` |
| Terminal | A one-row trigger with `▾`, rows of titles with `✓`; this library's look does not apply there |

### Differs from shadcn's structure

shadcn composes the list from children (`Select > SelectTrigger > SelectValue`, `SelectContent > SelectGroup >
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

## API

Coming from shadcn: `<SelectTrigger className size>` + `<SelectValue placeholder>` are `Select`'s `triggerClass`,
`size` and `placeholder`; `<SelectContent position align>` is `cover` and `align`; each `<SelectItem value>Title</SelectItem>`
is an entry `{ value, title }` of `items`, and a `<SelectGroup>` with its `<SelectLabel>` is those entries' `group`;
`onValueChange` is `onChange`.

### Props

| TSX | Rust | Type | Default | Description |
|---|---|---|---|---|
| `items` | `items` | `SelectItem[]` / `Vec<SelectItem>` | `[]` | The options, in order: `{ value, title, disabled?, group? }` (`nsh/base`'s type) |
| `value` | `value` | `string \| null` / `SelectValue` | Uncontrolled | The chosen item's `value`, `null` for none; the app sets it in `onChange` |
| `defaultValue` | `default_value` | `string` | — | The first value while uncontrolled |
| `placeholder` | `placeholder` | `string` | — | What the trigger shows while nothing is chosen (shadcn's `SelectValue placeholder`) |
| `size` | `size` | `Size` | `"md"` | The trigger's height, type, padding and icon, and the rows'; see Variants and sizes |
| `disabled` | `disabled` | `boolean` | `false` | It does not open, takes no focus, is dimmed |
| `invalid` | `invalid` | `boolean` | `false` | The app's checks reject the value: the field look's red edge and ring; screen readers say invalid |
| `open` / `defaultOpen` | `open` / `default_open` | `boolean` | Uncontrolled / `false` | The list's open state |
| `placement` | `placement` | `Placement` | `"bottom"` | Desktops, below-the-trigger form: the side (shadcn's `side`) |
| `align` | `align` | `Align` | `"start"` | Desktops, that form: the alignment along the side |
| `cover` | `cover` | `boolean` | The platform's | Desktops: `true` opens with the chosen row over the trigger (shadcn's `item-aligned`), `false` on a side (`popper`); unset: over it on macOS, on a side elsewhere |
| `renderItem` | `render_item` | `(entry: SelectEntry) => element` / `Renderer<SelectEntry>` | — | A row's content in place of its title (an icon and a title, a flag); the library still draws the row and its check. `entry` is what the item is (`value`, `title`, `disabled`, `index`, `group`), never its state |
| `accessibilityLabel` | `accessibility_label` | `string` | — | What screen readers call it (what the choice is about); set it even beside a `Label` (Input's G1) |
| `class` / `style` | `class` / `style` | | | The root, the box the trigger fills: width and layout (`w-[180px]`, `w-full`) |
| `triggerClass` / `triggerStyle` | `trigger_class` / `trigger_style` | | | The trigger, merged over the field look |
| `valueClass` / `valueStyle` | `value_class` / `value_style` | | | The trigger's text (the title or the placeholder) |
| `iconClass` / `iconStyle` | `icon_class` / `icon_style` | | | The trigger's chevron |
| `contentClass` / `contentStyle` | `content_class` / `content_style` | | | The popup or sheet surface, as DropdownMenu's: `contentStyle` wins over the library's look, `contentClass` only adds to it |
| `itemClass` / `itemStyle` | `item_class` / `item_style` | | | Each row: `itemStyle` is merged after the item look and wins; `itemClass` follows the library's highlight classes (NSH gaps 1) |
| `labelClass` / `labelStyle` | `label_class` / `label_style` | | | Each group header |

Not exposed in P2, though `nsh/base` has them: `searchable` and its search field (Combobox, P3, is the searchable
choice), `cleanable` and its clear button (shadcn has none; added later with a look), `empty`, `listClass`,
`renderGroup` (the header is the group's title).

### Events

| TSX | Rust (`SelectEvent`) | Payload | When |
|---|---|---|---|
| `onChange` | `Change(Option<SharedString>)` | `{ value: string \| null }` | The user chose another item (a press, Return, typeahead on the closed trigger) |
| `onOpenChange` | `OpenChange(open, reason)` | `{ open, reason }` | The user asks to open or close the list |
| `onOpen` / `onClose` | `Open` / `Close` | — | The list is all there / all gone |

`nsh/base`'s events, forwarded; `onChange` is NSH's name (conventions), shadcn's `onValueChange`.

### Parts

| Part | What | Customize with |
|---|---|---|
| root | `nsh/base`'s wrapper: the box the trigger fills | `class` / `style` |
| trigger | The field: value, chevron | `triggerClass` / `triggerStyle` |
| value | The chosen title or the placeholder | `valueClass` / `valueStyle` |
| icon | The `chevron-down` | `iconClass` / `iconStyle` |
| content | The popup or sheet | `contentClass` / `contentStyle` |
| item | Each row | `itemClass` / `itemStyle` |
| label | Each group header | `labelClass` / `labelStyle` |

## Variants and sizes

No variants (shadcn has none).

### The trigger

shadcn: `flex w-fit items-center justify-between gap-2 rounded-md border border-input bg-transparent px-3 py-2 text-sm
whitespace-nowrap shadow-xs data-[placeholder]:text-muted-foreground data-[size=default]:h-9 data-[size=sm]:h-8
dark:bg-input/30 dark:hover:bg-input/50`, the value `line-clamp-1`, a `ChevronDownIcon size-4 opacity-50`
(`text-muted-foreground`), and the field's focus, invalid and disabled classes.

| `size` | Height (pointer / touch) | Text (pointer / touch) | Padding | Chevron |
|---|---|---|---|---|
| `xs` | 24 / 32 | 12 / 14 | 8 | 12 |
| `sm` | 28 / 36 | 13 / 14 | 10 | 14 |
| `md` | 32 / 40 | 14 / 16 | 12 | 16 |
| `lg` | 36 / 48 | 14 / 16 | 12 | 16 |

- Height, text and padding are Input's (`input-height-<size>`, `input-font-size-<size>`, `input-padding-x-<size>`), so a
  Select and an Input of a `size` line up in a row. shadcn's `default` (36) is `lg`, its `sm` (32) is `md`: one density
  step down, as every control here.
- A row: the value (one line, cut with an ellipsis, `flex: 1`, left-aligned), then the chevron (`icon-size-<size>`,
  `select-icon-foreground`), `spacing × 2` (8) apart.
- The value's color is `input-foreground`; the placeholder's `input-placeholder-foreground`.
- Width: as wide as its content, shadcn's `w-fit` (a column that stretches its children does not widen it, and a row's
  `items-center` still centers it); the app sets one with `class` (`w-[180px]`, `w-full`), which goes over `w-fit` and
  the trigger fills.

### The list

The menu look (DropdownMenu's "The menu look"), with shadcn's select differences:

| Part | This library | shadcn |
|---|---|---|
| Surface | The popup surface (`foundation::popup::surface`: `popover`, `popover-border`, `popover-radius`, `popover-shadow` 3, `md`); padding `menu-padding` (4) inside the scrolling list; as wide as the trigger, at least `menu-min-width` (128) | `rounded-md border bg-popover shadow-md min-w-[8rem]`, viewport `p-1`, `min-w-(--radix-select-trigger-width)` |
| Row | Height `menu-item-height-<size>`; padding `menu-item-padding-x` (8) at the start; the title (one line, ellipsis) `flex: 1`; then a box `icon-size-<size>` wide holding the `check` while chosen, `menu-item-gap` (8) after the title and `menu-item-padding-x` before the end (32 at `md`, shadcn's `pr-8`); corners `menu-item-radius`; text `menu-item-font-size-<size>` | `py-1.5 pr-8 pl-2 text-sm rounded-sm`, the check `absolute right-2 size-3.5`, `CheckIcon size-4` |
| Check | `icon-size-<size>`, the row's text color | — |
| Group header | Height `menu-item-height-<size>`, padding `menu-item-padding-x` at the sides, `select-label-font-size` (12), `select-label-foreground` | `px-2 py-1.5 text-xs text-muted-foreground` |
| Distance | `popover-offset` (4) from the trigger when on a side; none over it | `popper`: `translate-y-1` |

The differences from shadcn are DropdownMenu's (row heights one step down, concentric row corners), and no separators
or scroll buttons.

### Motion

Popover's (every popup's): fade and zoom from 95% over `duration`, through `nsh/base`'s state looks, none with reduced
motion, `popover-animate` 0 to turn it off. `Select` has no `onPosition`, so no slide and a centered origin (Tooltip's gap 3);
over the trigger (macOS) no slide is wanted anyway. The sheet has `nsh/base`'s motion.

## States

The trigger takes the field look's states (Input's table): rest, focus-visible (`input-border-focus` edge and the
`input-ring` ring), invalid, invalid + focus-visible, disabled (`opacity-disabled`, the arrow cursor). In addition:

| State | Look | shadcn |
|---|---|---|
| Hover (pointer) | Fill `select-trigger-hover` (dark only: `input` at 50%; light unchanged) | `dark:hover:bg-input/50` |
| Placeholder shown | Value text in `input-placeholder-foreground` | `data-[placeholder]:text-muted-foreground` |
| Open (`expanded:`) | Edge `input-border-focus`, no ring (Decisions 2) | none |
| Row highlighted (`focus:`) | Fill `menu-item-focus`, text `menu-item-focus-foreground` | `focus:bg-accent focus:text-accent-foreground` |
| Row chosen (`selected:`) | The check shown; otherwise as at rest | `ItemIndicator` |
| Row disabled | `opacity-disabled`, never highlighted | `data-[disabled]:opacity-50` |
| Row pressed on touch (`active:`) | As highlighted | — |

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

## Accessibility

`nsh/base`'s (see Built on): the trigger is a `combobox` whose value is the chosen title (or the placeholder), the list a
`listbox`, rows `option`s with `selected` / `disabled`, headers `header`s; the highlighted row is the trigger's active
descendant, and a key that moves the highlight moves the screen reader's focus to it. The library adds `invalid` to the
trigger's accessibility state. The check and the chevron are hidden from screen readers. `accessibilityLabel` names the
select (Input's G1: no labelledby between elements).

## Tokens

The trigger reads the `input-*` tokens ([Input](input.md#tokens)) and the rows and surface the `menu-*` tokens
([DropdownMenu](dropdown-menu.md#tokens)); a theme that changes `input-border` or `menu-item-focus` changes Select with
them. Its own tokens are what shadcn's select adds. The table is generated (`<!-- tokens:select -->`).

<!-- tokens:select -->
| Token | Type | Derived default | Value |
|---|---|---|---|
| `select-trigger-hover` | color | `light_dark(input-background, input × 50%)` |  |
| `select-icon-foreground` | color | `muted-foreground × 50%` |  |
| `select-label-foreground` | color | `muted-foreground` |  |
| `select-label-font-size` | length | `12` | 12 |
<!-- /tokens -->

- `select-icon-foreground` is shadcn's `text-muted-foreground opacity-50` as one color.
- `select-label-font-size` is Tailwind's fixed 12 (`text-xs`), as `menu-shortcut-font-size`.
- A select's own `select-radius` or `select-height-*` are not declared: the trigger is a field, and fields share one set
  of tokens; a different select uses `triggerClass` or a scope.

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
```

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
2. **The trigger shows the title only**: shadcn's `SelectValue` repeats the chosen item's children (an icon too).
   `nsh/base` gives the trigger nothing but the value; the library shows the chosen item's title. A `renderValue`
   would follow `renderItem` later, if asked for.
3. **No separators in the list**: `nsh/base`'s rows are items and headers; groups are split by their headers only.
   shadcn's examples rarely use `SelectSeparator` between groups; a separator row kind in `listbox.rs` would add it.
4. **String children** do not apply (titles are data); `renderItem`'s elements follow Button's G0 in TSX.
5. **The list is exactly as wide as the trigger** on desktops: `nsh/base` gives the popup the trigger's width, which
   `menu-min-width` only raises, so rows wider than the trigger (`renderItem`'s, a long title) are cut or truncated,
   where shadcn's popper list is at least the trigger's width and grows with its rows. Workaround: a trigger wide
   enough for the rows (`class`). A width mode in `nsh/base` (at least the trigger's, else its content's) would allow
   it.

## Decisions

1. **Options are data (`items`), not `SelectItem` children**, because `nsh/base`'s list is virtualized and owns search,
   typeahead and highlight over titles (see the time-zone example). The name `SelectItem` is the data type (`nsh/base`'s), not a component.
2. **The open trigger keeps the focus edge** (`input-border-focus` while `expanded`): gpui-kit's Design Guides ask for
   persistent feedback while an attached popup is open; shadcn draws none. The library-wide decision (DropdownMenu's
   3: a `Button` trigger keeps its hover fill).
3. **macOS opens the list over the trigger, other desktops below it** (`nsh/base`'s default, `cover`): shadcn's
   default is item-aligned everywhere; the platforms' own habit wins.
4. **No search, no clear button in P2**: Combobox (P3) is the searchable choice; `cleanable` waits for a look.
5. **As wide as its content through `w-fit`, not `align-self: flex-start`**: an `align-self` of its own would override
   the cross-axis alignment of a row it sits in (a filter bar), putting it at the top instead of the middle (as
   `Badge`'s decision 3).

## Later

- **A search field in the list** (`searchable`, `nsh/base`'s): shadcn's "popup" combobox, a trigger showing the value
  with the search field inside the layer. Not in P3; [Combobox](combobox.md) is the searchable choice until then.
