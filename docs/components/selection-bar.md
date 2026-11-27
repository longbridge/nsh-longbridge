# SelectionBar

Status: implemented (P3, under review). A bar that floats at the bottom of the window (or of a container) while
the user has selected items, saying how many and offering what can be done to all of them at once: "3 selected ×
| Status · Assign · Tags · Archive · 🗑 · ⋯".

## Usage

- Acting on several items at once: rows checked in a [DataTable](data-table.md) or a [Table](table.md), files in a
  grid, messages in a list. The bar appears with the first selected item and goes with the last.
- The count tells the user the selection is still there after scrolling; its "×" (or Escape in the bar) clears it.
- Search terms: selection bar, bulk actions, batch actions, bulk edit bar, selection toolbar, contextual action bar,
  floating action bar, multi-select actions.

## When not to use

| Need | Use instead |
|---|---|
| Actions on one item | A row's own buttons, a `DropdownMenu` on the row, a context menu |
| A page's or a panel's permanent actions | Buttons in the page's header, or `nsh/base`'s `Toolbar` with ghost `Button`s |
| A message after the action ("3 archived", with Undo) | [Toast](toast.md) |
| Confirming a destructive action | [AlertDialog](alert-dialog.md), opened from the bar's action |
| Formatting a selection of text | The editor's own toolbar (`nsh/base`'s editors) |

## Built on

`nsh/base`'s `Toolbar` and `ToolbarSeparator` (`crates/nsh-base/src/toolbar.rs`, keys in `roving.rs`), NSH's overlay
(`overlay()`), and this library's [Button](button.md) and [Tooltip](tooltip.md).

| Provided by `nsh/base` (not rewritten here) | |
|---|---|
| Keyboard | Where pressables take the focus (macOS, the web): the toolbar is one Tab stop, at the item the focus last left; Left / Right move between its items around the ends, Home / End to the ends, past disabled ones, without pressing; Space and Return press. Its items are the `Button`s inside it and the triggers of its menus |
| Accessibility | The `toolbar` role and its name; each action a `button` |
| Separators | `ToolbarSeparator`: never a stop of the focus; the terminal's own line in a terminal |
| Overlay | NSH's overlay: drawn above the window's content and the overlays shown before it, outside its parent's layout and clipping, in the Tab order where the app placed the component |

What this library adds: the floating surface and where it goes (the window's bottom, or its parent's), the count
and its "×", the announcement of the count, Escape, the entrance and exit, the room it takes (`onInsetChange`),
the actions' look and size, the steps it takes as it runs out of room, and a dark bar (`appearance`).

### Implementation

- **`SelectionBar`** keeps the count it shows (the app's `count`, the last one above 0 while it leaves, so a bar on
  its way out does not say "0 selected") and its presence (dialog's: mounted while shown and while it leaves, a
  fade from 0 to 1 that drives its opacity, an 8-point slide up and a zoom from 95%). The component's own view is
  an empty, zero-size, absolutely placed view where the app put it; inside it:
  - `placement="window"`: an overlay of the window, `inset-0`, its content at the bottom centre, taking no presses
    itself (`box-none`), padded by `selection-bar-offset` plus the safe area;
  - `placement="container"`: a view stretched across the bottom of the component's parent (`absolute`, `inset-x-0`,
    `bottom-0`), the same way: NSH places an absolute view against its parent, so the parent is the container;
  - in both, the bar: `nsh/base`'s `Toolbar` named by the count, holding the count (a text), the "×" (an icon-only
    ghost `Button`), a separator, and a horizontal scroll view with the app's actions (padded inside by
    `ring-width`, and drawn that much wider, so a focus ring at its edge is not clipped).
- **The announcement**: NSH's `announceForAccessibility` with the count's words, as the bar appears and as they
  change while it is shown; nothing as it goes, and nothing for a bar already shown when the page is made.
- **Escape**: the bar follows the keyboard focus coming into it and leaving it (an `ElementRef`'s `onFocusIn` /
  `onFocusOut`; a menu opened from the bar is inside it). While the focus is in the bar, an empty overlay of its own
  listens to `onCancel`: Escape reaches the top overlay listening, so a menu or popover opened from the bar takes
  it first. As a guard the bar also counts its open layers (as `TableRow` does) and reports nothing while one is
  open. With the focus elsewhere there is no such overlay, and Escape is the app's.
- **Narrow steps**: the bar measures the room its parent gives it, its head (the count and its "×") with the count's
  words, and its actions at full size and with icons alone, each in the form before the step that needs it, and
  keeps them, so a step taken never undoes itself (see [Narrow widths](#narrow-widths)).
- **The fade**: the strip watches its scroll (NSH's `watchScroll`: offset, content and viewport) and masks itself
  with a linear gradient (`maskLinear`) that goes transparent over `selection-bar-fade` at a side where more actions
  are scrolled out of it; no mask while all of them show.
- **`SelectionBarAction`** is a [Button](button.md) of the bar's `size`, `ghost`, with its `label` as its text,
  drawn by Button's own drawing, so the button is the action's root (a menu places and presses its trigger's root);
  an icon-only one (`iconOnly`, or icons alone) shows the label as its tooltip, mounted only then, and its name. `destructive` draws the ghost
  look in the destructive colors (a scope over the button's ghost tokens: one button, not a copy of its drawing).
- **`appearance`** forces the bar's light or dark values for what it draws, as a `ThemeScope` with `appearance`
  around it would, without a view between the bar and its container (which `placement="container"` needs).

## API

### Components

| Component | TSX | Rust | What it is |
|---|---|---|---|
| `SelectionBar` | `<SelectionBar>` | `selection_bar()` | The floating bar: the count, its "×", the actions |
| `SelectionBarAction` | `<SelectionBarAction>` | `selection_bar_action()` | An action: an icon and a label, or the icon alone |
| `SelectionBarSeparator` | `<SelectionBarSeparator>` | `selection_bar_separator()` | A line between runs of actions |

An overflow (⋯) menu is a [DropdownMenu](dropdown-menu.md) whose `trigger` is a `SelectionBarAction` (see Examples);
any `Button` works as an action too (a filled "Merge").

### `SelectionBar`

| TSX | Rust | Type | Default | Description |
|---|---|---|---|---|
| `count` | `count` | `number` (`u32`) | `0` | How many items are selected |
| `open` | `open` | `boolean` | `count > 0` | Shown; left out, while `count` is above 0. Set it for a selection mode that shows the bar with nothing selected yet |
| `label` | `label` | `string` | "{count} selected" | What the count says ("3 tasks selected"); left out, the library's words (`labels`' `count-selected`, its `{count}` replaced) |
| `placement` | `placement` | `"window"` \| `"container"` (`SelectionBarPlacement`) | `"window"` | Where it floats: the bottom centre of the window, or of its parent (the view it is a child of, which it floats over) |
| `size` | `size` | `Size` | `"md"` | The actions' and the "×"'s size (Button's heights), the count's type |
| `appearance` | `appearance` | `"light"` \| `"dark"` (`ThemeAppearance`) | The app's | Draws the bar in these values whatever the app's appearance: `"dark"` for a dark bar on a light page |
| `accessibilityLabel` | `accessibility_label` | `string` | The count's words | What screen readers call the toolbar |
| `children` | `.child(..)` | `SelectionBarAction`s, `SelectionBarSeparator`s, `DropdownMenu`s, `Button`s | — | The actions |
| `class` / `style` | `class` / `style` | | | The bar (its surface), merged last |
| `countClass` / `countStyle` | `count_class` / `count_style` | | | The count's text |
| `clearClass` / `clearStyle` | `clear_class` / `clear_style` | | | The "×" |

### `SelectionBar` events

| TSX | Rust | Description |
|---|---|---|
| `onClear()` | `SelectionBarEvent::Clear` | The user asks to clear the selection: the "×", or Escape while the keyboard focus is in the bar (and no menu of the bar is open). The app empties its selection (and, from the keyboard, may move the focus back to the list: the bar goes, and the focus with it). Escape in the list is the app's to hear (Decisions 9) |
| `onInsetChange({ inset })` | `SelectionBarEvent::InsetChange(f32)` | The room the bar covers at the bottom of its window or container changed: from that edge to the bar's top (its height, `selection-bar-offset` and, in the window, the safe area), `0` once it is gone. Give the scrolling content as much room at its end, so its last rows can scroll above the bar |

### `SelectionBarAction`

| TSX | Rust | Type | Default | Description |
|---|---|---|---|---|
| `label` | `label` | `string` | — | Its words: shown beside the icon, or its tooltip and its name when the icon shows alone |
| `icon` | `icon` | `IconName` | — | An icon before the label (the library's names) |
| `iconEnd` | `icon_end` | `IconName` | — | An icon after the label (`chevron-down` on a menu's trigger) |
| `iconOnly` | `icon_only` | `boolean` | `false` | Always the icon alone (a trash can), the label in its tooltip |
| `variant` | `variant` | `"default"` \| `"destructive"` (`SelectionBarActionVariant`) | `"default"` | `destructive`: the label and icon in the destructive color, a destructive tint under the pointer |
| `size` | `size` | `Size` | The bar's | Its size. Give the bar's `size` to an action in a menu's `trigger`, which is made where the app writes it and does not see the bar (it then also keeps its label when the bar shows icons alone: make it `iconOnly`) |
| `disabled` | `disabled` | `boolean` | `false` | Cannot be pressed (an action that does not apply to every selected item) |
| `loading` | `loading` | `boolean` | `false` | Its action is running (Button's) |
| `accessibilityLabel` | `accessibility_label` | `string` | `label` | What screen readers say, when it should differ from `label` |
| `accessibilityHint` | `accessibility_hint` | `string` | — | What pressing it does, for screen readers |
| `class` / `style` | `class` / `style` | | | The button (the action's root), merged last |
| `labelClass` / `labelStyle`, `iconClass` / `iconStyle` | | | | Button's parts |

Event: `onPress()` (`SelectionBarActionEvent::Press`), never while disabled or loading. Left out, `size` is the
bar's, so a bar's actions always match.

### `SelectionBarSeparator`

No props but `class` / `style`: a vertical line between runs of actions, `nsh/base`'s `ToolbarSeparator`.

### Parts

| Part | What | Customize with |
|---|---|---|
| bar | The surface: fill, edge, corners, shadow, padding | `class` / `style` |
| `count` | The count's text | `countClass` / `countStyle` |
| `clear` | The "×" (a ghost icon-only `Button`) | `clearClass` / `clearStyle` |
| action | A `SelectionBarAction`'s button, its `label` and `icon` parts | The action's `class`, `labelClass`, `iconClass` |

## Variants and sizes

### Look

| | |
|---|---|
| Surface | `selection-bar` (`popover`) with `selection-bar-foreground`, an edge `border-width` × `selection-bar-border`, corners `selection-bar-radius` (`radius-xl`, 14), shadow tier `selection-bar-shadow` (4, `lg`), padding `selection-bar-padding` (6) |
| Radii | Concentric: the bar's corners are the buttons' (`button-radius`, 8) plus the padding (6) |
| Row | The count, the "×", a separator, the actions; gap `selection-bar-gap` (4); as wide as its content, at most the room less `selection-bar-offset` on each side |
| Count | `font-size-<size>`, medium, tabular figures, `px` `spacing × 2`, never wrapped |
| "×" | An icon-only ghost `Button` of the bar's size, its icon `x`, named and tipped "Clear selection" |
| Separator | `border-width` wide, half the actions' height, `selection-bar-separator` (`border`) |
| Actions | Ghost `Button`s of the bar's size (24 / 28 / 32 / 36, touch 32 / 36 / 40 / 48); `destructive` in `selection-bar-action-destructive-foreground` (`destructive`), under the pointer `selection-bar-action-destructive-hover` (`destructive` at 10%, 20% in the dark) |
| Dark bar | `appearance="dark"`: every color of the bar and of what is inside it (actions, menus it opens, their tooltips) from the dark values; in the dark appearance it reads as the popover surface it already is |

The default surface follows the theme (the popover surface, as every floating surface of the library) and is lifted
by the `lg` shadow and an edge, so it stands apart from a white table and from a dark one (Decisions 3).

### Placement and geometry

| | |
|---|---|
| `window` | Bottom centre of the window, `selection-bar-offset` (24, 16 on touch screens) above its bottom edge inside the safe area; above the page and the overlays shown before it |
| `container` | Bottom centre of its parent, `selection-bar-offset` above the parent's bottom edge, over the parent's content (the parent clips it if it clips) |
| Width | Its content's; at most the room less `selection-bar-offset` on each side |
| Not covering the last rows | `onInsetChange({ inset })`: the app gives its scrolling content `inset` of room at its end while the bar shows (`paddingBottom` on a scroll view's content; under a `DataTable`, which has no end padding of its own, a margin below the table: NSH gaps 2) |

### Narrow widths

The bar takes these steps in order, each only while the one before it does not fit the room, and goes back the same
way as the room comes back:

| Step | What changes | Still there |
|---|---|---|
| 1. Full | Nothing: the count's words, every action's icon and label | — |
| 2. Icons alone | Every action with an icon shows the icon alone; its label goes to its tooltip and its name | Actions without an icon keep their words |
| 3. The number alone | The count shows its number ("12"), its words stay its name and the toolbar's ("12 selected") | The "×" |
| 4. A scroll | The actions scroll sideways under the count, which stays; the strip fades out over `selection-bar-fade` (24) at each side where more actions are scrolled out of it | Every action, one swipe away |

A long list of actions belongs in the overflow menu anyway: keep the rarer ones there from the start, and the bar
rarely needs step 4.

### Motion

Shown: from transparent, 8 points lower and at 95% to its place over `selection-bar-duration` (200), Tailwind's
easing; hidden: back the same way, taking no presses on its way out. Nothing moves with reduced motion or
`selection-bar-animate` 0. The count changes in place.

## States

| State | Look | Behavior |
|---|---|---|
| Hidden | Not mounted | — |
| Shown | The bar | Its toolbar is a Tab stop |
| Leaving | Fading | No presses |
| Action: hover, pressed, focus, disabled, loading | Button's ghost states | Button's |
| Action open (its menu is open) | Lit as under the pointer (Button's `expanded`) | The menu has the keys; Escape closes it, not the selection |

## Pointer and touch

| | Pointer | Touch |
|---|---|---|
| Actions and "×" | `button-height-<size>` | The touch heights (40 at `md`); smaller than `touch-target`, a hit slop |
| Offset from the bottom | 24 | 16 |
| Hover | The ghost fill | None |
| Tooltips of icon-only actions | After the pointer rests, or at once from the keyboard | Hardly ever: the icon and the name only, so choose icons that read alone, or keep the label |
| Narrow window | Icons alone, the number alone, then a sideways scroll with a fade ([Narrow widths](#narrow-widths)) | The same |

## Accessibility

- The bar is a `toolbar` named by the count's words ("3 selected"), or `accessibilityLabel`. Its actions and the "×"
  are `button`s; an icon-only action is named by its `label`. The arrow keys move between them without pressing
  (`nsh/base`'s toolbar).
- The count is announced politely as it changes ("3 selected"), from the first selection on; clearing announces
  nothing (the bar goes).
- Escape reports `onClear` while the keyboard focus is in the bar; a menu or popover opened from the bar takes it
  first. On a focused "×" (or icon-only action) whose tooltip shows, the first Escape hides the tooltip, the next is
  the bar's. Escape in the list or a field beside it is the app's: listen on the list (`onKeyDown` on a view around
  it bubbles from its rows) and clear there, as the gallery's example does (Decisions 9).
- The number alone (a narrow bar) is still named by the words: screen readers say "12 selected".
- Tab reaches the bar where the app placed the component: place it right after what it acts on (the table), so Tab
  goes from the rows to the actions.
- The words are the library's (`labels`): `count-selected` ("{count} selected") and `clear-selection` ("Clear
  selection"). A language with plural forms that one pattern cannot hold gives `label` itself.

## Tokens

Generated from the code's declarations (`NSH_LONGBRIDGE_WRITE=1 cargo test --test tokens`); a test keeps it current.

<!-- tokens:selection-bar -->
| Token | Type | Derived default | Value |
|---|---|---|---|
| `selection-bar` | color | `popover` |  |
| `selection-bar-foreground` | color | `popover-foreground` |  |
| `selection-bar-border` | color | `border` |  |
| `selection-bar-separator` | color | `border` |  |
| `selection-bar-radius` | length | `radius-xl` | 14 |
| `selection-bar-padding` | length | `spacing × 1.5` | 6 |
| `selection-bar-gap` | length | `spacing` | 4 |
| `selection-bar-shadow` | number | `4` | 4 |
| `selection-bar-offset` | length | `pointer_touch(spacing × 6, spacing × 4)` | 24 (touch 16) |
| `selection-bar-fade` | length | `spacing × 6` | 24 |
| `selection-bar-action-destructive-foreground` | color | `destructive` |  |
| `selection-bar-action-destructive-hover` | color | `light_dark(destructive × 10%, destructive × 20%)` |  |
| `selection-bar-duration` | number | `duration × 4 / 3` | 200 |
| `selection-bar-animate` | number | `1` | 1 |
<!-- /tokens -->

- The actions and the "×" read Button's ghost tokens ([button.md](button.md#tokens)); a destructive action reads
  them through a scope with the two `selection-bar-action-destructive-*` tokens and `button-destructive-ring`.
- Other tokens it reads: `spacing` (the count's padding), `ring-width` (the strip's room for focus rings), `border-width`, `font-size-*`, `font-weight-medium`,
  `shadow`, `touch-target`.

## Examples

TSX:

```tsx
import { signal } from "nsh";
import {
  DataTable,
  DropdownMenu,
  DropdownMenuItem,
  SelectionBar,
  SelectionBarAction,
  SelectionBarSeparator,
} from "nsh-longbridge";

const picked = signal<string[]>([]);
const inset = signal(0);

<View class="h-[420px]">
  {/* Escape in the table is the page's: it clears the selection there too. */}
  <View
    class="flex-1"
    style={{ marginBottom: inset() }}
    onKeyDown={(e) => {
      if (e.key === "Escape" && picked().length > 0) picked.set([]);
    }}
  >
    <DataTable
      class="flex-1"
      accessibilityLabel="Tasks"
      columns={columns}
      data={tasks()}
      checkboxColumn
      checkedRows={tasks().map((t) => picked().includes(t.id))}
      onCheckedChange={({ row, checked }) => toggle(tasks()[row].id, checked)}
      onCheckedAllChange={({ checked }) => picked.set(checked ? tasks().map((t) => t.id) : [])}
    />
  </View>
  <SelectionBar
    placement="container"
    count={picked().length}
    onClear={() => picked.set([])}
    onInsetChange={(e) => inset.set(e.inset)}
  >
    <SelectionBarAction icon="tag" label="Tags" onPress={tag} />
    <SelectionBarAction icon="user" label="Assign" onPress={assign} />
    <SelectionBarAction icon="archive" label="Archive" onPress={archive} />
    <SelectionBarSeparator />
    <SelectionBarAction icon="trash-2" iconOnly variant="destructive" label="Delete" onPress={confirmDelete} />
    <DropdownMenu placement="top" align="end" trigger={<SelectionBarAction icon="ellipsis" iconOnly label="More" />}>
      <DropdownMenuItem onSelect={duplicate}>Duplicate</DropdownMenuItem>
      <DropdownMenuItem onSelect={exportCsv}>Export as CSV</DropdownMenuItem>
    </DropdownMenu>
  </SelectionBar>
</View>

// A dark bar at the bottom of the window, its own words, one filled action.
<SelectionBar appearance="dark" count={n()} label={`${n()} contacts selected`} onClear={clear}>
  <SelectionBarAction icon="chevron-down" label="Add to group" onPress={openGroups} />
  <Button size="md" onPress={merge}>Merge</Button>
</SelectionBar>
```

With NSH 0.7.0, write a string child inside `<Text>` for now ([Button's G0](button.md#nsh-gaps)); an action's
words are its `label`, a prop, so they need nothing.

Rust:

```rust
use nsh_core::prelude::*;
use nsh_longbridge::{
    IconName, SelectionBarActionVariant, SelectionBarPlacement, selection_bar,
    selection_bar_action, selection_bar_separator,
};

let picked = signal(Vec::<u32>::new());
let inset = signal(0.0_f32);

selection_bar()
    .placement(SelectionBarPlacement::Container)
    .count(move || picked.with(|p| p.len() as u32))
    .on_clear(move || picked.set(Vec::new()))
    .on_inset_change(move |v: &f32| inset.set(*v))
    .child(selection_bar_action().icon(IconName::Archive).label("Archive").on_press(archive))
    .child(selection_bar_separator())
    .child(
        selection_bar_action()
            .icon(IconName::Trash2)
            .icon_only(true)
            .variant(SelectionBarActionVariant::Destructive)
            .label("Delete")
            .on_press(delete),
    )
```

## NSH gaps

1. **No ancestor for an absolute view.** NSH places an absolute view against its parent, not the nearest
   positioned ancestor, so `placement="container"` floats over the component's parent: a `ThemeScope` or any view
   between them becomes the container. `appearance` covers the dark bar without one.
2. **`DataTable` has no padding at the end of its rows.** `nsh/base`'s table scrolls its rows in its own view and
   takes no content inset, so a bar over its bottom cannot let the last rows scroll above it; the app shortens the
   table by `inset` (a margin below it) while the bar shows. With an end inset on `nsh/base`'s table the bar could
   float over the rows themselves.
3. **No focus to give back.** When `onClear` from the keyboard removes the bar, the focus inside it goes with it;
   NSH tells a component neither which view had the focus before Tab brought it in nor lets it return it there, so
   the app moves it (a `DataTable` ref's `focus()`).
4. **Narrow steps measured, not declared.** NSH reports sizes after layout, so the bar learns that its actions do
   not fit one layout late (one frame per step on a narrow window). What a step measured in the form before it is
   kept: actions added, or a count grown by a digit, while a step is taken are measured again once the bar is wide
   enough to leave it.
5. **String children** (Button's G0) — not met here: an action's words are a prop.
6. **Back has no focus to go by.** Android's back reaches the top overlay listening to `onCancel`, so it clears the
   selection only while the keyboard focus is in the bar, which on a touch screen it hardly ever is. A contextual
   action bar is expected to leave selection mode on back: the app listens to back itself (NSH's back listeners)
   while items are selected. A component-level "while shown, back is mine" listener would let the bar do it.
7. **A hint on its way out takes Escape.** While a tooltip hidden by Escape plays its exit, NSH still counts it as
   shown and gives it the next Escape, so two Escapes in quick succession (within its exit) on a focused "×" hide
   the tooltip only.
8. **No mask on Windows and GPUI.** NSH draws `maskLinear` on the other platforms only; there the scrolled strip
   ends at a hard edge (step 4 still scrolls).

## Decisions

1. **The name: `SelectionBar`.** It says what the bar is for and when it shows: a selection. `ActionBar` is
   Android's name for a screen's top app bar (and a page header in several libraries); `Toolbar` is `nsh/base`'s
   bar of controls with no look, which this one is built on; "bulk actions" names a web habit, not the thing.
2. **Floating, two placements.** The window's bottom centre (an overlay, above the page) is the common form and the
   default; a container's bottom centre (absolute in its parent) keeps the bar with a table that is one panel of a
   window. The header-row form (the actions replacing a table's header while rows are checked) is the table's own
   layout, written with `Table` parts, not this component.
3. **The look: the popover surface, a dark bar on request.** The library's floating surfaces (menus, toasts,
   popovers) are the theme's popover surface; the bar is one more, lifted by the `lg` shadow and an edge so it reads
   above a table in both appearances. An always-inverted bar (dark on light, light on dark) glares in the dark
   appearance; a dark bar on a light page is a product's choice, made with `appearance="dark"`, which keeps the
   dark appearance's own treatment (not an inverted light) for the bar and everything it opens.
4. **An action's words are a `label` prop, not children.** The bar shows them beside the icon, or gives them to the
   tooltip and to screen readers when the icon shows alone (`iconOnly`, a narrow bar): one string in three
   places, which an element child could not be. Plain `Button`s still work as actions for anything else.
5. **Narrow widths: icons alone, the number alone, then a scroll with a fade.** Collapsing actions into the
   overflow menu would need each action as data (a menu item and a button), which children are not; dropping the
   labels of actions with icons keeps every action one press away. The count's words go next, before any action
   is hidden: the number alone still says how many, and "selected" is what the bar itself says. The scroll is the
   last resort, and its fade shows that more actions are there. The app keeps rarer actions in its overflow menu.
6. **`onClear`, not `onOpenChange`.** The bar holds no state of its own: it shows the app's selection. Its "×" and
   Escape ask the app to clear that selection, which hides the bar.
7. **A count with a pattern** (`{count} selected`) rather than per-count words: English needs one form ("1
   selected", "3 selected"); a language with plural forms gives its own `label`.
8. **The announcement is an explicit `announceForAccessibility`**, not a live region: a live region that appears
   with its words is not announced on every platform, so the first count would be lost, and the visible count would
   be read twice where it is. The bar says the count as it appears and as it changes, and nothing as it goes (the
   app's action says what happened, a toast for instance).
9. **Escape clears only from the bar.** Escape in a search field means "clear the search", in a list it may mean
   something of the list's; losing a selection the user built row by row to a key meant for something else costs
   too much. So the bar hears Escape only while the keyboard focus is in it, and the app, which knows its list,
   clears from there itself (an `onKeyDown` on a view around the list: key events bubble from the focused row). No
   prop turns the window-wide Escape back on: an app that wants it listens at its root with the same one line.
10. **An action may take a `size`.** Slot content does not see the context a component provides
    ([conventions](../conventions.md)), so an action in a `DropdownMenu`'s `trigger` cannot read the bar's size; an
    optional `size` (left out: the bar's) lets the app match it, without a size on every action.
