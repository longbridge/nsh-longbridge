# DropdownMenu

Status: spec (P2), not implemented. A menu of commands and settings that a trigger (usually a `Button`) opens: items,
checkbox and radio items, labels, separators, shortcuts, submenus and destructive items. Reference: shadcn/ui
new-york v4 `dropdown-menu.tsx` (Radix DropdownMenu).

This spec also owns the **menu look** (`foundation::menu`, the `menu-*` tokens under [Tokens](#tokens)): the popup
list's surface and its rows, which [Select](select.md)'s list shares now and the later ContextMenu, Combobox and
Command will share, as `foundation::field` is shared by every field. A change to the menu look is made here and
applies to all of them.

## Usage

- The actions of an object behind a "More" (`…`) button, an account menu, a "New" button with kinds to create, view
  options (checkbox items), a sort order (radio items), a short list of commands with their shortcuts.
- Search terms: dropdown menu, menu button, overflow menu, more menu, actions menu, kebab menu, popup menu, pull-down
  menu.

## When not to use

| Need | Use instead |
|---|---|
| Choosing a value a form submits, shown in the trigger | [Select](select.md) (its trigger shows the value; the menu's does not) |
| Searching many commands | Command (later) or Combobox (P3) |
| A menu opened by the secondary button or a held finger | ContextMenu (later; `nsh/base`'s `ContextMenu` with this look) |
| Content that is not a list of commands (a form, a preview) | Popover (P2) |
| The system's own menu (native look, drawn by the OS) | `nsh/base`'s `MenuButton` (iOS and Android draw the system menu; desktops the platform's context menu) |
| The app's menu bar | `nsh/desktop`'s menus |
| Switching views of one page | [Tabs](tabs.md) |

## Built on

`nsh/base`'s `Menu`, `MenuItem`, `MenuSeparator` and `Submenu` (`crates/nsh-base/src/menu.rs`). This library adds the
look, the item kinds (checkbox, radio, label, shortcut, destructive, inset) and the radio group's value; all behavior
is `nsh/base`'s:

| Provided by `nsh/base` (not rewritten here) | |
|---|---|
| Presentation | Desktops and an iPad in a regular width: a popup at the trigger (`placement`, `align`), on the side where it fits whole, never taller than the room on its side (its items scroll, without a scroll bar). Phones, other tablets, an iPad in a compact width: a sheet at the bottom over a backdrop, below the status bar at most |
| Opening | A press on the trigger opens it with no item focused; Return, Space or Down on the focused trigger opens it on its first enabled item, Up on its last (WAI-ARIA's menu button) |
| Keys | Up / Down (around the ends), Home / End, Page Up / Page Down a view at a time, the item reached scrolled into view; letters and digits by typeahead on the items' text (or `accessibilityLabel`); Return / Space choose; Escape closes |
| Pointer | Moving over an item gives it the keyboard focus (`focus:` is the highlight), so the keys go on from it |
| Choosing | An item's press reports `onSelect` and closes the menu (`"select"`) unless `closeOnSelect` is false |
| Submenus | Desktop popups cascade beside their trigger item, its first item level with it, on hover, press, Right / Return / Space; Left or Escape go back with the focus on the trigger. In a sheet a submenu replaces its menu's items, with a back row first; Escape and back go up a level |
| Dismissal | A press outside (desktops: it goes on to what is under the pointer; touch screens: a transparent overlay takes it), Escape, Android's back; the focus goes back to the trigger as it closes (not after a press outside) |
| Control | `open` / `defaultOpen` / `onOpenChange({ open, reason })`, `onOpen`, `onClose` (`popup.rs`'s `OpenState`, shared by every popup) |
| Entrance and exit | The popup's starting and ending looks (`popupStartingClass` / `popupEndingClass`), skipped under reduced motion; the sheet's own motion (`SheetMotion`) |
| Toolbars and menu bars | A trigger in a `Toolbar` is one of its items; the pointer moving to another menu's trigger in the bar moves the open menu there |
| Accessibility | The trigger's `expanded` and `hasPopup`; the popup's `menu` role; items `menuitem`, `menuitemcheckbox`, `menuitemradio` with `checked` |
| Terminal | Rows of text; a checkable item's mark (`✓`, `●`) at the row's end, a submenu's ` ›`, separators as `─`. This library's look does not apply there |

### Differs from shadcn's structure

shadcn has `DropdownMenu > DropdownMenuTrigger + DropdownMenuContent > items`. In NSH the menu's trigger is a **slot
of `Menu`** (`trigger`), which the menu binds (its press, keys, `expanded`, the anchor its popup is placed against),
and its children are the items; a component cannot route a child component into its parent's slot. So:

- No `DropdownMenuTrigger`: the trigger is the `trigger` prop, any element (`trigger={<Button>Open</Button>}`), as
  shadcn's `<DropdownMenuTrigger asChild>`.
- No `DropdownMenuContent`: its props are `DropdownMenu`'s (`placement`, `align`, `contentClass`, `contentStyle`), and
  `DropdownMenu`'s children are the items.
- No `DropdownMenuPortal`: NSH's overlays are above everything already.
- `DropdownMenuSub`'s trigger is its `trigger` prop as well (`trigger={<DropdownMenuSubTrigger>…}`), and its children
  are its items (no `DropdownMenuSubContent`).

Every other part keeps shadcn's name and place: `DropdownMenuItem`, `DropdownMenuCheckboxItem`,
`DropdownMenuRadioGroup`, `DropdownMenuRadioItem`, `DropdownMenuLabel`, `DropdownMenuSeparator`,
`DropdownMenuShortcut`, `DropdownMenuGroup`, `DropdownMenuSub`, `DropdownMenuSubTrigger`.

### Implementation

- **Structure**: `DropdownMenu` is a `#[component]` whose root is `nsh_base::menu()` with `open`, `defaultOpen`,
  `placement`, `align`, `trigger` and the children passed through; `offset` is `popover-offset`, `margin` `nsh/base`'s
  default. It provides a `MenuLook` context (the `size`, the presentation) that every part reads. Each item kind is
  a component whose root is `nsh_base::menu_item()` (`menu_separator()`, `submenu()`), styled on its root element with
  bindings, the app's `class` / `style` merged last (as Checkbox's root).
- **The popup surface** is `foundation::popup::surface(t)`, the one every popup shares (fill, edge, corners, shadow
  and the text-style reset, on the `popover-*` tokens; defined in [Popover](popover.md), "The popup surface"). The
  menu adds its padding (`menu-padding`) and minimum width (`menu-min-width`). Both go on `nsh/base`'s `popupClass` /
  `popupStyle`, because the items' scroll view is inside the popup and the surface must stay put around it (a view of
  this library's inside would scroll with the items and clip the shadow). The surface is built on each opening, so
  its tokens are read as the menu opens (see [NSH gaps](#nsh-gaps) 1); the app's `contentStyle` is merged after it.
- **Popup or sheet**: `nsh/base` takes one `popupClass` / `popupStyle` for both. The library builds them for the
  presentation, computed as `menu.rs`'s private `as_sheet()` (`!is_desktop()`, except an iPad, not Catalyst, whose
  `horizontal_size_class()` is regular), from public functions (NSH gaps 2).
- **Text styles**: the popup is placed among the menu's own children, so it inherits the text styles around the
  `DropdownMenu` (as Tooltip's gap 7). The surface resets them: regular weight, normal wrapping, no underline, no case
  change, no letter spacing; its text color is `popover-foreground` and its size the items'.
- **Scope**: the popup and the items are made under the `DropdownMenu`, so they read the `ThemeScope` around it: a
  menu whose trigger is in a dark scope is dark. Guarded by a headless test, as Tooltip's.
- **Terminal**: in a terminal (`nsh_base::terminal::is_terminal()`) the library passes no indicators, draws no look
  and sets no popup classes: `nsh/base`'s terminal rows apply.

## API

Coming from shadcn: `<DropdownMenuTrigger asChild>{x}</DropdownMenuTrigger>` is `trigger={x}`; `<DropdownMenuContent
side align className>` is `DropdownMenu`'s `placement`, `align` and `contentClass`, its children `DropdownMenu`'s;
`<DropdownMenuSubTrigger>` goes in `DropdownMenuSub`'s `trigger`, and `<DropdownMenuSubContent>`'s children are
`DropdownMenuSub`'s; `onCheckedChange` / `onValueChange` are `onChange`. Every other part is written as in shadcn.

### Components

| Component | TSX | Rust | What it is |
|---|---|---|---|
| `DropdownMenu` | `<DropdownMenu>` | `dropdown_menu()` | The menu: its trigger, its popup, its items |
| `DropdownMenuItem` | `<DropdownMenuItem>` | `dropdown_menu_item()` | A command |
| `DropdownMenuCheckboxItem` | `<DropdownMenuCheckboxItem>` | `dropdown_menu_checkbox_item()` | A setting on or off, with a check |
| `DropdownMenuRadioGroup` | `<DropdownMenuRadioGroup>` | `dropdown_menu_radio_group()` | Holds the value of the radio items inside it |
| `DropdownMenuRadioItem` | `<DropdownMenuRadioItem>` | `dropdown_menu_radio_item()` | One choice of its group, with a dot |
| `DropdownMenuLabel` | `<DropdownMenuLabel>` | `dropdown_menu_label()` | A heading over a group of items; not an item |
| `DropdownMenuSeparator` | `<DropdownMenuSeparator>` | `dropdown_menu_separator()` | A line between groups |
| `DropdownMenuShortcut` | `<DropdownMenuShortcut>` | `dropdown_menu_shortcut()` | The shortcut at the end of an item: text or `Kbd`s |
| `DropdownMenuGroup` | `<DropdownMenuGroup>` | `dropdown_menu_group()` | A plain view around items (shadcn's, for structure) |
| `DropdownMenuSub` | `<DropdownMenuSub>` | `dropdown_menu_sub()` | A submenu: its trigger and its items |
| `DropdownMenuSubTrigger` | `<DropdownMenuSubTrigger>` | `dropdown_menu_sub_trigger()` | The item that opens a submenu, with a chevron |

Every component takes `class` and `style` on its root, merged last.

### `DropdownMenu`

| TSX | Rust | Type | Default | Description |
|---|---|---|---|---|
| `trigger` | `trigger` | element (slot) | — | What opens it: usually a `Button`. The menu sets its `expanded` / `hasPopup` states, its press and keys, and places the popup against it |
| `children` | `.child(..)` | items | — | The items (any of the parts above, and the app's own views); made on each opening |
| `open` | `open` | `boolean` / `Prop<Option<bool>>` | Uncontrolled | Controlled: the app owns it and changes it in `onOpenChange` |
| `defaultOpen` | `default_open` | `boolean` | `false` | Open at first while uncontrolled |
| `placement` | `placement` | `"top"` \| `"bottom"` \| `"left"` \| `"right"` (`Placement`) | `"bottom"` | The side of the trigger (shadcn's `side`); the other side when only that one has room |
| `align` | `align` | `"start"` \| `"center"` \| `"end"` (`Align`) | `"start"` | Alignment along that side (`nsh/base`'s default; Radix's is `center`, but shadcn's examples and the desktops' pull-down menus start at the trigger's edge) |
| `size` | `size` | `Size` | `"md"` | The items' height, type, icon and padding; see [Variants and sizes](#variants-and-sizes) |
| `contentClass` / `contentStyle` | `content_class` / `content_style` | | | The popup (or sheet) surface. `contentStyle` is merged after the library's look and wins (a width, `{ width: 224 }`); `contentClass` is applied before the library's inline look, so it can add (`w-56`) but not override what the look sets (fill, edge, corners, padding, shadow): change those with tokens or `contentStyle` (NSH gaps 1) |
| `class` / `style` | `class` / `style` | | | The root (`nsh/base`'s wrapper around the trigger): layout only |

The distance to the trigger is the `popover-offset` token (shadcn's `sideOffset`), not a prop, as Tooltip's.

### `DropdownMenu` events

| TSX | Rust (`DropdownMenuEvent`) | Payload | When |
|---|---|---|---|
| `onOpenChange` | `OpenChange(open, reason)` | `{ open, reason }` (`nsh/base`'s `OpenChangeReason`: `"triggerPress"`, `"pressOutside"`, `"backdropPress"`, `"cancel"`, `"select"`, `"hover"`) | The user asks to open or close it |
| `onOpen` | `Open` | — | It is shown (a sheet once it came in) |
| `onClose` | `Close` | — | It is gone (after its exit), whoever closed it |

Passed through from `nsh/base`'s `Menu`. `onCancel` and `onPosition` are not exposed in P2 (shadcn has neither); they
are added under these names when a need comes.

### `DropdownMenuItem`

| TSX | Rust | Type | Default | Description |
|---|---|---|---|---|
| `children` | `.child(..)` | text, elements | — | Its label (in a `label` part), then a `DropdownMenuShortcut` if any |
| `variant` | `variant` | `"default"` \| `"destructive"` (`MenuItemVariant`) | `"default"` | `destructive`: red text and highlight, for deleting or leaving |
| `icon` | `icon` | `IconName` | — | An icon before the label, in `menu-item-icon-foreground` (`destructive` in the destructive variant) |
| `inset` | `inset` | `boolean` | `false` | The label starts where an icon's or a check's label would, lining up with such items |
| `disabled` | `disabled` | `boolean` | `false` | Not chosen, skipped by the keys, dimmed |
| `closeOnSelect` | `close_on_select` | `boolean` | `true` | Choosing it closes the menu (Radix's `onSelect` with `preventDefault()` keeps it open) |
| `accessibilityLabel` | `accessibility_label` | `string` | — | What screen readers say and typeahead matches, when the text is not enough |
| `accessibilityDescription` | `accessibility_description` | `string` | — | Read after the name |
| `labelClass` / `labelStyle`, `iconClass` / `iconStyle` | `label_class` …, `icon_class` … | | | The `label` and `icon` parts |

Event: `onSelect` (`DropdownMenuItemEvent::Select`, `.on_select(..)`), `nsh/base`'s, when it is chosen (a press,
Return or Space).

An `Icon` given as a child takes the item's text color (shadcn turns every uncolored icon muted with `[&_svg]`;
NSH has no descendant selectors). Use `icon` for the leading icon; an `Icon` child with its own class otherwise.

### `DropdownMenuCheckboxItem`

As `DropdownMenuItem` without `variant` and `icon`, plus:

| TSX | Rust | Type | Default | Description |
|---|---|---|---|---|
| `checked` | `checked` | `boolean` | `false` | Controlled: the check shows the app's value |
| `indicatorClass` / `indicatorStyle` | `indicator_class` / `indicator_style` | | | The box at the start that holds the check |

Event: `onChange({ checked })` (`DropdownMenuCheckboxItemEvent::Change(bool)`, `#[args(checked)]`), the value it asks
for (the opposite of `checked`), as the library's `Checkbox`. `nsh/base` reports `onSelect`; the library turns it into
the requested value. Choosing it closes the menu unless `closeOnSelect={false}` (Radix's default closes too).

### `DropdownMenuRadioGroup` and `DropdownMenuRadioItem`

| Component | TSX | Rust | Type | Description |
|---|---|---|---|---|
| Group | `value` | `value` | `string` | The value of the checked item (controlled) |
| Group | `children` | `.child(..)` | radio items, labels, separators | |
| Item | `value` | `value` | `string` | What choosing it sets the group's value to |
| Item | `disabled`, `closeOnSelect`, `accessibilityLabel`, `indicatorClass` / `indicatorStyle` | | | As the checkbox item's |

Event: the group's `onChange({ value })` (`DropdownMenuRadioGroupEvent::Change(SharedString)`), when an item other than
the checked one is chosen. The group provides its value and change through context; each item is `nsh/base`'s
`MenuItem` with `role="menuitemradio"` and `checked = value == group.value`. A radio item outside a group is drawn
unchecked and logs a warning in development builds.

### `DropdownMenuLabel`, `DropdownMenuSeparator`, `DropdownMenuShortcut`, `DropdownMenuGroup`

| Component | Props | Description |
|---|---|---|
| `DropdownMenuLabel` | `children`, `inset` | A text, not an item: not focusable, passed over by the keys and typeahead |
| `DropdownMenuSeparator` | — | `nsh/base`'s `MenuSeparator`, styled: shown with its level's items |
| `DropdownMenuShortcut` | `children` | A text at the end of its item (`ml-auto`); its children may be `Kbd`s instead (see Examples) |
| `DropdownMenuGroup` | `children` | A view in a column, no look, no role |

### `DropdownMenuSub` and `DropdownMenuSubTrigger`

| Component | TSX | Rust | Type | Default | Description |
|---|---|---|---|---|---|
| Sub | `trigger` | `trigger` | element (slot) | — | A `DropdownMenuSubTrigger` |
| Sub | `children` | `.child(..)` | items | — | Its items, made as it opens |
| Sub | `open` | `open` | `boolean` | Uncontrolled | Controlled, as `nsh/base`'s `Submenu` |
| Sub | `disabled` | `disabled` | `boolean` | `false` | It does not open; its trigger is dimmed |
| Sub | `backLabel` | `back_label` | `string` | "Back" (`labels`) | In a sheet: the text of the row that goes back up |
| SubTrigger | `children`, `icon`, `inset` | | | | As an item's; a `chevron-right` is drawn at its end |

Event: the sub's `onOpenChange({ open, reason })` (`DropdownMenuSubEvent::OpenChange`), `nsh/base`'s.

In a sheet the library fills `nsh/base`'s `back` slot with a row of the item look: `chevron-left` and `backLabel`.
"Back" is a new word in `labels` (`Label::Back`, key `"back"`), translated as "Loading" is.

### Parts

| Part | On | Customize with |
|---|---|---|
| content | The popup or sheet (`nsh/base`'s popup part) | `contentClass` / `contentStyle` on `DropdownMenu` |
| item root | Each item kind, the label, the separator | `class` / `style` |
| `label` | The text wrapping an item's children | `labelClass` / `labelStyle` |
| `icon` | An item's or a sub trigger's leading icon | `iconClass` / `iconStyle` |
| `indicator` | The check or dot box of a checkbox or radio item | `indicatorClass` / `indicatorStyle` |

The indicator is a view of this library's in `nsh/base`'s `indicator` slot, not `nsh/base`'s indicator view (whose
Rust style setter takes a fixed style; Checkbox's gap 3).

## Variants and sizes

### The menu look

shadcn's classes:

- Content: `min-w-[8rem] overflow-y-auto rounded-md border bg-popover p-1 text-popover-foreground shadow-md`,
  `sideOffset` 4. Sub content: the same with `shadow-lg`.
- Item: `flex items-center gap-2 rounded-sm px-2 py-1.5 text-sm focus:bg-accent focus:text-accent-foreground
  data-[disabled]:opacity-50 data-[inset]:pl-8`, icons `size-4 text-muted-foreground`; destructive
  `text-destructive focus:bg-destructive/10 dark:focus:bg-destructive/20`, its icons `text-destructive`.
- Checkbox and radio items: `pl-8`, the indicator `absolute left-2 size-3.5`, a `CheckIcon size-4` or a
  `CircleIcon size-2 fill-current`.
- Label: `px-2 py-1.5 text-sm font-medium`, `pl-8` inset. Separator: `-mx-1 my-1 h-px bg-border`. Shortcut: `ml-auto
  text-xs tracking-widest text-muted-foreground`. Sub trigger: an item, `data-[state=open]:bg-accent`, a
  `ChevronRightIcon ml-auto size-4`.

| Part | This library |
|---|---|
| Surface | `foundation::popup::surface(t)`: fill `popover`, text `popover-foreground`, edge `border-width` × `popover-border`, corners `popover-radius` (8), shadow tier `popover-shadow` (3, `md`; submenus too, NSH gaps 3); plus padding `menu-padding` (4), at least `menu-min-width` (128) wide, as wide as its widest item |
| Item | A row, items centered, `menu-item-gap` (8) apart; height `menu-item-height-<size>`, its content centered; padding `menu-item-padding-x` (8) at the sides; corners `menu-item-radius` (4); text `menu-item-font-size-<size>`, regular weight, one line (`truncate`); full width of the surface's inner box |
| Icon | `icon-size-<size>` (16 at `md`), `menu-item-icon-foreground` |
| Indicator column | Checkbox and radio items, and `inset`, start their label at `menu-item-padding-x` + `icon-size-<size>` + `menu-item-gap` (32 at `md`, shadcn's `pl-8`): the indicator is a box `icon-size-<size>` square before the label, the check `icon-size-<size>`, the dot `icon-size-<size>` / 2 (8) round, in the item's text color |
| Label | As an item's geometry, `font-weight-medium`, `popover-foreground`; not highlighted |
| Separator | `border-width` high, `menu-separator`, across the surface's padding (`-mx` `menu-padding`), `menu-padding` above and below |
| Shortcut | `margin-left: auto`, `menu-shortcut-font-size` (12), letter spacing 0.1 em (`tracking-widest`), `menu-shortcut-foreground` |
| Sub trigger | An item, with `chevron-right` at its end (`icon-size-<size>`, `menu-item-icon-foreground`) |

Differences from shadcn:

- **Item heights follow `size`, one step below shadcn's** (as Button's): `md` is 28 (shadcn's `py-1.5` around a 20
  line, 32), the desktop's medium density; gpui-kit's menu rows are 26, its Design Guides' "compact list and menu rows:
  4 px vertical, 8–12 px horizontal". For shadcn's 32 use `size="lg"`.
- **Concentric item corners**: `menu-item-radius` is `popover-radius − menu-padding` (4), not shadcn's `rounded-sm` (6):
  the highlight inside the surface's padding follows its corner (conventions, gpui-kit's concentric radii).
- **Indicator in the row, not absolutely placed**: the label starts at the same 32 at `md` and lines up with other
  sizes too (the column grows with `icon-size-<size>`).
- **The trigger shows that its menu is open**: see [States](#states) (Decisions 3).

### Sizes

| `size` | Item height (pointer / touch) | Text (pointer / touch) | Icon | Label start, checkbox / radio / inset |
|---|---|---|---|---|
| `xs` | 20 / 44 | 12 / 14 | 12 | 8 + 12 + 8 = 28 |
| `sm` | 24 / 44 | 13 / 14 | 14 | 30 |
| `md` | 28 / 44 | 14 / 16 | 16 | 32 |
| `lg` | 32 / 48 | 14 / 16 | 16 | 32 |

Pointer heights are `control-height-<size> − 4`; touch heights are `touch-target` (44), `lg` `control-height-touch-lg`
(48): rows of a sheet touch each other, so a hit slop cannot make them reachable, the rows themselves are. Touch text
is 16 at `md` / `lg` (iOS's and Android's menus use 16–17), 14 at `xs` / `sm`.

### Variants

`DropdownMenuItem`'s `variant`: `default` and `destructive` (shadcn's). The other kinds have none.

### Motion

| | shadcn (tw-animate-css) | This library |
|---|---|---|
| Entrance | `fade-in-0 zoom-in-95`, `slide-in-from-*-2` (8) away from the side it went on, 150 ms, origin toward the trigger | Popover's: from opacity 0, scale 0.95 and 8 toward the trigger to its look over `duration` (150 ms), the scale origin on the trigger's side, through `nsh/base`'s `popupStartingClass` / `Style`; submenus the same (they take the menu's looks) |
| Exit | `fade-out-0 zoom-out-95` | To opacity 0 and scale 0.95 (no slide) through `popupEndingClass` / `Style`, then unmounted |
| Side | The side Radix reports | The side `onPosition` reports, the requested `placement` until the first report. If that report comes after the first frame (★ NSH gaps 4), the slide is dropped and only fade and zoom stay |
| Reduced motion | Not handled | No transition (`nsh/base` skips the state looks) |
| Sheet | — | `nsh/base`'s sheet motion; the backdrop fades with it |
| Switch | — | `popover-animate` 0: no state looks, shown and gone in one frame (every popup at once) |

The motion is [Popover](popover.md)'s, shared by every popup; the menu defines none of its own.

## States

| State | Look | shadcn |
|---|---|---|
| Item highlighted (`focus:`: the keys or the pointer are on it) | Fill `menu-item-focus` (`accent`), text and icon `menu-item-focus-foreground` | `focus:bg-accent focus:text-accent-foreground` |
| Item pressed on touch (`active:`) | As highlighted: touch moves no focus | — |
| Destructive item | Text and icon `menu-item-destructive-foreground`; highlighted fill `menu-item-destructive-focus`, text unchanged | `text-destructive focus:bg-destructive/10 dark:…/20` |
| Disabled item | `opacity-disabled`, no highlight; cursor stays the arrow | `data-[disabled]:opacity-50` |
| Checked (checkbox, radio) | The check or dot shown; the row as at rest | — |
| Sub trigger, submenu open (`expanded:`) | As highlighted | `data-[state=open]:bg-accent` |
| Trigger, menu open (`expanded:`) | The library's `Button` keeps its hover fill while its popup is open ([Button](button.md)'s States; Decisions 3) | none |

No focus ring on items: the highlight is the focus (shadcn's `outline-hidden`). The cursor is the arrow everywhere
(`cursor-default`). The highlight changes at once (shadcn's items have no transition; a menu swept by the pointer
should not trail).

## Pointer and touch

| | Desktops and desktop browsers (an iPad in a regular width too) | Phones, other tablets, an iPad in a compact width |
|---|---|---|
| Form | The popup at the trigger: the surface above | A sheet at the bottom, full width: fill `popover`, top corners `menu-sheet-radius` (10), a top edge in `popover-border`, padding `menu-sheet-padding` (8) and the bottom safe area, no shadow; the backdrop `overlay` (shadcn's Drawer: `rounded-t-lg border-t bg-background`, `bg-black/50`) |
| Item height | `control-height-<size> − 4` (20 / 24 / 28 / 32) | 44 / 44 / 44 / 48, full width; an iPad's popup uses these too |
| Highlight | `focus:` from the pointer and the keys | `active:` while pressed |
| Submenus | Cascade beside their item | Replace the items, a back row first (`chevron-left` and `backLabel`) |
| Shortcuts | Shown | Shown if the app gives them; whether to show them on touch is the app's call (as Kbd says) |

macOS draws the same popup as the web (shadcn's look, not `NSMenu`'s): the conventions take the look from shadcn.
An app that wants the system's menu uses `MenuButton`.

## Accessibility

All of it is `nsh/base`'s:

- The trigger is `expanded` while open and `hasPopup`; the popup is a `menu`; items are `menuitem`, checkbox items
  `menuitemcheckbox` and radio items `menuitemradio` with `checked`; disabled items are dimmed and skipped.
- The keys and focus return are under [Built on](#built-on). Typeahead matches each item's `accessibilityLabel`, else
  its text.
- The shortcut is part of the item's text for screen readers. Icons and indicators are hidden from them.
- A label is plain text; a separator is not focusable.

## Tokens

The menu look's tokens are declared once, in `foundation::menu` (`tokens!`), and read by DropdownMenu and Select (later
ContextMenu, Combobox and Command); a theme that changes `menu-item-focus` changes them all. The surface's tokens are
Popover's (`popover-border`, `popover-radius`, `popover-shadow`, `popover-offset`, `popover-animate`), read directly: a
theme that changes `popover-radius` changes popovers, menus and Select's list together. Values are those of
`nsh/base`'s default theme. When implemented, this table is generated (`<!-- tokens:menu -->`).

<!-- tokens:menu -->
| Token | Type | Derived default | Value |
|---|---|---|---|
| `menu-padding` | length | `spacing` | 4 |
| `menu-min-width` | length | `128` | 128 |
| `menu-sub-offset` | length | `−(menu-padding + border-width)` | −5 |
| `menu-item-height-xs` | length | `pointer_touch(control-height-xs − 4, touch-target)` | 20 (touch 44) |
| `menu-item-height-sm` | length | `pointer_touch(control-height-sm − 4, touch-target)` | 24 (touch 44) |
| `menu-item-height-md` | length | `pointer_touch(control-height-md − 4, touch-target)` | 28 (touch 44) |
| `menu-item-height-lg` | length | `pointer_touch(control-height-lg − 4, control-height-touch-lg)` | 32 (touch 48) |
| `menu-item-font-size-xs` | length | `pointer_touch(font-size-xs, 14)` | 12 (touch 14) |
| `menu-item-font-size-sm` | length | `pointer_touch(font-size-sm, 14)` | 13 (touch 14) |
| `menu-item-font-size-md` | length | `pointer_touch(font-size-md, 16)` | 14 (touch 16) |
| `menu-item-font-size-lg` | length | `pointer_touch(font-size-lg, 16)` | 14 (touch 16) |
| `menu-item-padding-x` | length | `spacing × 2` | 8 |
| `menu-item-gap` | length | `spacing × 2` | 8 |
| `menu-item-radius` | length | `popover-radius − menu-padding` | 4 |
| `menu-item-focus` | color | `accent` |  |
| `menu-item-focus-foreground` | color | `accent-foreground` |  |
| `menu-item-icon-foreground` | color | `muted-foreground` |  |
| `menu-item-destructive-foreground` | color | `destructive` |  |
| `menu-item-destructive-focus` | color | `light_dark(destructive × 10%, destructive × 20%)` |  |
| `menu-separator` | color | `border` |  |
| `menu-shortcut-foreground` | color | `muted-foreground` |  |
| `menu-shortcut-font-size` | length | `12` | 12 |
| `menu-sheet-radius` | length | `radius-lg` | 10 |
| `menu-sheet-padding` | length | `spacing × 2` | 8 |
<!-- /tokens -->

- The fill and text are the semantic `popover` / `popover-foreground`, which the menu uses directly and declares no
  token for (as Card's `card`).
- `menu-item-radius` is drawn between 0 and half the item's height, so `radius: 0` gives square highlights rather than
  a negative radius.
- `menu-shortcut-font-size` is Tailwind's fixed 12, as `tooltip-font-size`: explanatory text, not a control.
- `menu-sub-offset` is negative because Radix places a submenu against its trigger item, which sits inside the
  surface's padding and edge, while `nsh/base` measures the gap from the menu's edge.
- An app that changes a touch value alone writes `<name>-touch` (tokens.md).

Other tokens it reads: `popover`, `popover-foreground`, `popover-border`, `popover-radius`, `popover-shadow`,
`popover-offset`, `popover-animate`, `overlay`, `icon-size-*`, `font-weight-medium`,
`border-width`, `opacity-disabled`, `touch-target`, `shadow`, `duration`.

## Examples

```tsx
import { keymap, signal } from "nsh";
import {
  Button, DropdownMenu, DropdownMenuCheckboxItem, DropdownMenuGroup, DropdownMenuItem, DropdownMenuLabel,
  DropdownMenuRadioGroup, DropdownMenuRadioItem, DropdownMenuSeparator, DropdownMenuShortcut, DropdownMenuSub,
  DropdownMenuSubTrigger, Kbd,
} from "nsh-longbridge";

const showStatusBar = signal(true);
const position = signal("bottom");

<DropdownMenu
  trigger={<Button variant="outline">Open</Button>}
  contentClass="w-56"
>
  <DropdownMenuLabel>My Account</DropdownMenuLabel>
  <DropdownMenuGroup>
    <DropdownMenuItem onSelect={openProfile}>
      Profile
      <DropdownMenuShortcut>{keymap.label("shift+mod+p")}</DropdownMenuShortcut>
    </DropdownMenuItem>
    <DropdownMenuItem icon="settings" onSelect={openSettings}>
      Settings
      <DropdownMenuShortcut><Kbd>{keymap.label("mod+,")}</Kbd></DropdownMenuShortcut>
    </DropdownMenuItem>
  </DropdownMenuGroup>
  <DropdownMenuSeparator />
  <DropdownMenuSub trigger={<DropdownMenuSubTrigger>Invite users</DropdownMenuSubTrigger>}>
    <DropdownMenuItem onSelect={inviteByEmail}>Email</DropdownMenuItem>
    <DropdownMenuItem onSelect={inviteByMessage}>Message</DropdownMenuItem>
  </DropdownMenuSub>
  <DropdownMenuSeparator />
  <DropdownMenuCheckboxItem checked={showStatusBar()} onChange={({ checked }) => showStatusBar.set(checked)}>
    Status Bar
  </DropdownMenuCheckboxItem>
  <DropdownMenuSeparator />
  <DropdownMenuLabel>Panel Position</DropdownMenuLabel>
  <DropdownMenuRadioGroup value={position()} onChange={({ value }) => position.set(value)}>
    <DropdownMenuRadioItem value="top">Top</DropdownMenuRadioItem>
    <DropdownMenuRadioItem value="bottom">Bottom</DropdownMenuRadioItem>
  </DropdownMenuRadioGroup>
  <DropdownMenuSeparator />
  <DropdownMenuItem variant="destructive" icon="log-out" onSelect={logOut}>Log out</DropdownMenuItem>
</DropdownMenu>

// An icon button's overflow menu, aligned to its end, kept open on a toggle.
<DropdownMenu
  align="end"
  trigger={<Button variant="ghost" iconOnly icon="ellipsis" accessibilityLabel="More actions" />}
>
  <DropdownMenuItem onSelect={rename}>Rename</DropdownMenuItem>
  <DropdownMenuCheckboxItem closeOnSelect={false} checked={pinned()} onChange={({ checked }) => pinned.set(checked)}>
    Pinned
  </DropdownMenuCheckboxItem>
  <DropdownMenuItem variant="destructive" disabled={!canDelete()} onSelect={remove}>Delete</DropdownMenuItem>
</DropdownMenu>
```

With NSH 0.7.0, write a string child inside `<Text>` for now (Button's G0). The icons named here (`settings`,
`log-out`, `ellipsis`) are lucide names the library adds to `IconName` with this component.

```rust
use nsh_core::{element::key_label, prelude::*};
use nsh_longbridge::{
    button, dropdown_menu, dropdown_menu_checkbox_item, dropdown_menu_item, dropdown_menu_separator,
    dropdown_menu_shortcut, dropdown_menu_sub, dropdown_menu_sub_trigger, ButtonVariant, MenuItemVariant,
};

let status_bar = signal(true);

dropdown_menu()
    .trigger(button().variant(ButtonVariant::Outline).child("Open"))
    .content_class("w-56")
    .child(
        dropdown_menu_item()
            .on_select(move || open_profile())
            .child("Profile")
            .child(dropdown_menu_shortcut().child(key_label("shift+mod+p"))),
    )
    .child(dropdown_menu_separator())
    .child(
        dropdown_menu_sub()
            .trigger(dropdown_menu_sub_trigger().child("Invite users"))
            .child(dropdown_menu_item().child("Email"))
            .child(dropdown_menu_item().child("Message")),
    )
    .child(
        dropdown_menu_checkbox_item()
            .checked(move || status_bar.get())
            .on_change(move |checked: &bool| status_bar.set(*checked))
            .child("Status Bar"),
    )
    .child(dropdown_menu_item().variant(MenuItemVariant::Destructive).child("Log out"))
```

## NSH gaps

1. **The popup surface is `nsh/base`'s part, styled through `popupClass` / `popupStyle`** (accepted limits, shared with
   Popover). The items' scroll view is inside the popup, so the surface cannot be a view of this library's
   (conventions' rule for parts the app restyles). `popupStyle`'s Rust setter builds a style once: the surface is built
   on each opening, so its tokens are read as the menu opens, and a theme switched while it is open applies at the next
   opening. The library's look is an inline style, which wins over the app's `contentClass`: an app changes the
   surface with tokens or `contentStyle` (merged after the look). ★ The first headless test checks that the shadow
   survives this path; if it does not, the surface has no shadow, only its edge, and this is reported. A popup that
   accepted a reactive `Style`, or a surface slot around the scroll view, would lift these limits.
2. **Whether a menu is a sheet is private** (`menu.rs`'s `as_sheet()`), and `popupClass` serves both forms.
   Workaround: the library computes the same condition from public functions (`is_desktop`, `os`, `idiom`,
   `is_catalyst`, `horizontal_size_class`), guarded by a test per platform idiom. Read as the surface is built, so an
   iPad window resized across its size class while the menu is open keeps the other form's look until the next
   opening. `nsh/base` exporting it, or taking separate `sheetClass` / `sheetStyle`, would remove the copy (asked of
   NSH).
3. **Submenus take their menu's popup class and style**, so a submenu cannot have shadcn's `shadow-lg` while the menu
   has `shadow-md`: both use `popover-shadow` (3). A per-level popup look in `nsh/base` would allow the `lg` tier.
4. ★ **The slide follows the side only if `onPosition` comes before the first frame.** `Menu` reports its placement,
   but the starting look is applied as the popup is made. One of the first headless tests (shared with Popover); if it
   comes later, the slide is dropped and only fade and zoom stay, as Tooltip (its gap 3).
5. ★ **The popup reads the `ThemeScope` around the `DropdownMenu`**: it is made under the menu, so the context reaches
   it. A headless test checks it first (a menu in a dark scope is dark); if it does not, the menu passes its scope to
   the surface and items through its own context.
6. **No descendant selectors** (shadcn's `[&_svg]:size-4 text-muted-foreground`): an `Icon` child gets its size from the
   item's context (`provide_icon_size`) but the item's text color. Workaround: the `icon` prop.
7. **String children** (Button's G0): until NSH releases its fix, the gallery wraps them in `<Text>`.

## Decisions

1. **`trigger` is a prop and `DropdownMenu` takes the content's props**, not `DropdownMenuTrigger` /
   `DropdownMenuContent` children: NSH's `Menu` binds its trigger as a slot (see "Differs from shadcn's structure").
   The same for every overlay of the library (Dialog, Popover).
2. **Item heights one step below shadcn's** (28 at `md`), as every control here; `size="lg"` is shadcn's 32. Item
   corners concentric (4).
3. **A trigger stays visibly open**: gpui-kit's Design Guides ask that "a Button that owns a dropdown remain visibly
   pressed or open until the popup closes", and shadcn has nothing for it. `Menu` sets the trigger's `expanded` state,
   so the library's `Button` draws its hover fill under `expanded:`, a public change to Button's look recorded in
   [Button](button.md)'s States and made with Popover's implementation; Select's trigger keeps its focus edge while
   open.
4. **Checkable items report `onChange({ checked })`**, as the library's `Checkbox`, rather than `nsh/base`'s bare
   `onSelect`; radio groups report `onChange({ value })`.
5. **The menu look is shared** (`foundation::menu`, `menu-*`: the rows, separators, shortcuts, the sheet), owned by
   this spec: Select's list uses it now; Combobox, ContextMenu and Command will. The surface is Popover's
   (`foundation::popup::surface`, `popover-*`).
6. **Close reasons are `nsh/base`'s** (`"select"`, `"cancel"`, …); none is added.
