# ToggleGroup

Status: spec (P3), not implemented.

A row of toggles that holds one value or several: an alignment (left, center, right), a view switcher, a set of
formatting options, a segmented control. The look is shadcn/ui new-york v4's `toggle-group.tsx` (Radix ToggleGroup),
whose items take [Toggle](toggle.md)'s look; the behavior is `nsh/base`'s `ToggleGroup` and `Toggle`.

## Usage

- One of a few options shown as buttons, applied at once: text alignment, list or grid view, light / dark / system,
  a density, a chart's period. `multiple`: several independent options in one row (bold, italic, underline).
- Search terms: toggle group, segmented control, segmented buttons, button group, radio buttons as buttons, view
  switcher, alignment buttons.

## When not to use

| Need | Use instead |
|---|---|
| Switching between views of a page, each with its content | [Tabs](tabs.md) |
| One of a set in a form, with labels and descriptions | [RadioGroup](radio-group.md) |
| Many options | [Select](select.md) |
| Buttons that act, not options | Buttons in `nsh/base`'s `Toolbar` |
| One option on or off | [Toggle](toggle.md) |
| The system's segmented control on phones | `nsh/mobile`'s `SegmentedControl` |

## Built on

`nsh/base`'s `ToggleGroup` and `Toggle` (`crates/nsh-base/src/toggle.rs`, keys in `roving.rs`). `nsh/base`'s group
holds no value: each toggle is told whether it is pressed and reports its press. This library adds what shadcn's
ToggleGroup has around them: the value (one or several, controlled or not), the variant and size handed to the items,
the gap or the joined look, and the items' look (Toggle's).

| Provided by `nsh/base` (not rewritten here) | |
|---|---|
| Keyboard | Where pressables take the focus (macOS, the web): the group is one Tab stop, at the first pressed toggle (else the first enabled); Left / Right and Up / Down move the focus around the ends, Home / End to the ends, past disabled toggles, **without pressing** (WAI-ARIA's toolbar pattern, as Radix's roving focus); Space and Return press the focused toggle |
| Presses | Each toggle's press, from the pointer, touch, the keyboard and screen readers |
| Disabled | The group's `disabled` disables every toggle |
| Accessibility | The group a `toolbar` (a `group` inside a `Toolbar`, whose stop it then shares); each toggle a `button` with its pressed state |
| Toolbars | Inside `nsh/base`'s `Toolbar`, its toggles join the toolbar's items and arrows |

### Implementation

- **`ToggleGroup`** is a `#[component]` whose root is `nsh_base::toggle_group()`, `disabled` passed through,
  `accessibilityLabel` set on its root element. It holds the value (the app's `value`, else its own signal from
  `defaultValue`) and provides a `ToggleGroupContext`: the value, the setter that reports `onChange`, `multiple`,
  `variant`, `size` and `spacing`. Its layout is set on the root with bindings: a row, centered, as wide as its
  content (`w-fit`), the gap.
- **`ToggleGroupItem`** is the library's [Toggle](toggle.md) drawing (the same root, `nsh_base::toggle()`, and the
  same look functions), with `pressed` = the value holds its `value`, and a press setting the value: without
  `multiple`, `[value]`, or `[]` when it was pressed (Radix's single group lets the pressed item go); with `multiple`,
  its `value` added or removed. `onChange` is reported only when the value changes. Its `variant` and `size` are the
  group's. An item outside a `ToggleGroup` renders unpressed and logs a warning in development builds (as
  `TabsTrigger`).
- **The joined look** (`spacing` 0): the items' corners and edges depend on their place, from NSH's `first:` / `last:`
  variants on the item's root (an item wrapped in a view of the app's loses its place: see NSH gaps 1).

## API

### Components

| Component | TSX | Rust | What it is |
|---|---|---|---|
| `ToggleGroup` | `<ToggleGroup>` | `toggle_group()` | The row and its value |
| `ToggleGroupItem` | `<ToggleGroupItem>` | `toggle_group_item()` | One option |

`nsh/base` exports a `ToggleGroup` too; an app using both renames one.

### `ToggleGroup`

| TSX | Rust | Type | Default | Description |
|---|---|---|---|---|
| `multiple` | `multiple` | `boolean` | `false` | Several values (shadcn's `type="multiple"`); without it one value at most (`type="single"`). Read when made |
| `value` | `value` | `string[]` / `Prop<Option<Vec<SharedString>>>` | Uncontrolled | The pressed items' values (controlled): one at most without `multiple`. The app sets it in `onChange` |
| `defaultValue` | `default_value` | `string[]` | `[]` | The first value while uncontrolled |
| `variant` | `variant` | `"default"` \| `"outline"` (`ToggleVariant`) | `"default"` | The items' look ([Toggle](toggle.md)'s variants) |
| `size` | `size` | `Size` | `"md"` | The items' height, type and icon |
| `spacing` | `spacing` | `number` | `0` | The gap between items in `spacing` units (shadcn's: `2` is 8); `0` joins them into one shape |
| `disabled` | `disabled` | `boolean` | `false` | Every item disabled |
| `accessibilityLabel` | `accessibility_label` | `string` | — | What screen readers call the set ("Text alignment") |
| `children` | `.child(..)` | `ToggleGroupItem`s | — | The items, direct children for the joined look |
| `class` / `style` | `class` / `style` | | | The root, merged last |

Event: `onChange({ value })` (`ToggleGroupEvent::Change(Vec<SharedString>)`, `#[args(value)]`), when the user presses
an item and the value changes. NSH's name (conventions), shadcn's `onValueChange`. A group that must always have one
value (a segmented control) ignores `[]` (see Examples).

### `ToggleGroupItem`

| TSX | Rust | Type | Default | Description |
|---|---|---|---|---|
| `value` | `value` | `string` | — | Its value, unique in its group |
| `disabled` | `disabled` | `boolean` | `false` | This item disabled |
| `accessibilityLabel` | `accessibility_label` | `string` | — | Its name when its content is an icon alone: required then |
| `children` | `.child(..)` | string, elements | — | An icon (the app's), a word, or both, each in a `label` part |
| `class` / `style`, `labelClass` / `labelStyle` | | | | As Toggle's |

No events of its own: the group reports `onChange`. No `variant` or `size` of its own: shadcn lets an item's apply
only where the group sets none, which here it always does.

### Parts

| Part | What | Customize with |
|---|---|---|
| root | `nsh/base`'s toggle group: the row | `ToggleGroup`'s `class` / `style` |
| item | `nsh/base`'s toggle | `ToggleGroupItem`'s `class` / `style` |
| `label` | A text wrapping each of an item's children | `labelClass` / `labelStyle` |

## Variants and sizes

shadcn:

- Group: `flex w-fit items-center gap-[--spacing(var(--gap))] rounded-md`, and `shadow-xs` on an outline group of
  joined items.
- Item: `toggleVariants` for the group's variant and size, then `w-auto min-w-0 shrink-0 px-3 focus:z-10
  focus-visible:z-10`; joined (`spacing` 0): `rounded-none shadow-none first:rounded-l-md last:rounded-r-md`, outline:
  `border-l-0 first:border-l`.

| | This library |
|---|---|
| Group | A row, items centered, as wide as its items (`w-fit`); gap `spacing × spacing-prop` |
| Item | Toggle's look for the group's `variant` and `size` (heights, type, icons, colors, focus ring, press sink), with horizontal padding `toggle-group-item-padding-x` (12, shadcn's `px-3`) at every size and no minimum width; it does not shrink |
| Joined (`spacing` 0) | Items touch. Corners: none, but the first item's start corners and the last item's end corners (`toggle-radius`, 8): the group reads as one shape (gpui-kit's Design Guides: a segmented control's end segments follow the container's corners, its middle segments are square). Outline: no edge at an item's start but the first's, so each boundary has one line; the group carries the `xs` shadow (`toggle-outline-shadow`), the items none |
| Apart (`spacing` > 0) | Each item is a whole Toggle: its own corners, edge and shadow |
| Focus | A focused item is raised above its neighbors (shadcn's `z-10`), so its ring is not covered |

Corners and edges follow the layout direction (start and end), where shadcn's are left and right: in a right-to-left
layout the first item is on the right.

Sizes: Toggle's (24 / 28 / 32 / 36, touch 32 / 36 / 40 / 48), one step below shadcn's as Button's.

## States

The items take Toggle's states (rest, hover, pressed down, on, focus-visible, disabled), with the group's variant. On
a joined outline group, a focused item turns the edges it draws `toggle-border-focus` (not the start line, which is
its neighbor's), and its ring, raised above the neighbors, goes around all of it (shadcn's look). The group itself has
no state but `disabled`, which disables its items (each at `opacity-disabled`).

## Pointer and touch

| | Pointer | Touch |
|---|---|---|
| Item height | `toggle-height-<size>` | The touch heights |
| Hit area | The item | Below `touch-target` (44): a vertical hit slop only, as Toggle's in a joined group (horizontal slop would take the neighbor's presses); with `spacing` the slop reaches sideways at most half the gap |
| Hover | Yes | No |
| Focus | The ring | The edge only |

## Accessibility

`nsh/base`'s: the group is a `toolbar` named by `accessibilityLabel` (a `group` inside a `Toolbar`); items are
`button`s with their pressed state; the arrow keys move between them without pressing; Space and Return press.
Radix's single group gives its items the `radio` role instead; `nsh/base` keeps toggle buttons, which screen readers
announce as pressed or not. An app whose set is a form's choice uses `RadioGroup`, which has the radio roles. Inside a
[Field](field.md) the group joins it: `disabled`, and its description read with the group.

## Tokens

The items read Toggle's tokens ([toggle.md](toggle.md#tokens)); the group adds one. Generated from the code's
declarations (`NSH_LONGBRIDGE_WRITE=1 cargo test --test tokens`); a test keeps it current.

<!-- tokens:toggle-group -->
| Token | Type | Derived default | Value |
|---|---|---|---|
| `toggle-group-item-padding-x` | length | `spacing × 3` | 12 |
<!-- /tokens -->

- The joined group's shadow is `toggle-outline-shadow`, its corners `toggle-radius`.
- Other tokens it reads: `spacing` (the gap), `border-width`, `touch-target`, `shadow`.

## Examples

TSX:

```tsx
import { signal } from "nsh";
import { Icon } from "nsh-icon";
import { AlignCenter, AlignLeft, AlignRight, Bold, Italic, Monitor, Moon, Sun, Underline } from "nsh-icon-lucide";
import { ToggleGroup, ToggleGroupItem } from "nsh-longbridge";

// One value, always one: a press on the pressed item asks for [], which the app ignores.
const appearance = signal("system");

<ToggleGroup
  variant="outline"
  size="sm"
  value={[appearance()]}
  onChange={({ value }) => value[0] && appearance.set(value[0])}
  accessibilityLabel="Appearance"
>
  <ToggleGroupItem value="light" accessibilityLabel="Light"><Icon icon={Sun} size={14} /></ToggleGroupItem>
  <ToggleGroupItem value="dark" accessibilityLabel="Dark"><Icon icon={Moon} size={14} /></ToggleGroupItem>
  <ToggleGroupItem value="system" accessibilityLabel="System"><Icon icon={Monitor} size={14} /></ToggleGroupItem>
</ToggleGroup>

// Several values, uncontrolled, items apart.
<ToggleGroup multiple spacing={2} defaultValue={["bold"]} accessibilityLabel="Text style">
  <ToggleGroupItem value="bold" accessibilityLabel="Bold"><Icon icon={Bold} size={16} /></ToggleGroupItem>
  <ToggleGroupItem value="italic" accessibilityLabel="Italic"><Icon icon={Italic} size={16} /></ToggleGroupItem>
  <ToggleGroupItem value="underline" accessibilityLabel="Underline"><Icon icon={Underline} size={16} /></ToggleGroupItem>
</ToggleGroup>

// Words
<ToggleGroup variant="outline" value={[density()]} onChange={({ value }) => value[0] && density.set(value[0])} accessibilityLabel="Density">
  <ToggleGroupItem value="compact">Compact</ToggleGroupItem>
  <ToggleGroupItem value="default">Default</ToggleGroupItem>
  <ToggleGroupItem value="comfortable">Comfortable</ToggleGroupItem>
</ToggleGroup>
```

With NSH 0.7.0, write a string child inside `<Text>` for now ([Button's G0](button.md#nsh-gaps)).

Rust:

```rust
use nsh_core::prelude::*;
use nsh_longbridge::{Size, ToggleVariant, toggle_group, toggle_group_item};

let align = signal(SharedString::from("left"));

toggle_group()
    .variant(ToggleVariant::Outline)
    .size(Size::Sm)
    .value(move || Some(vec![align.get()]))
    .on_change(move |value: &Vec<SharedString>| {
        if let Some(v) = value.first() {
            align.set(v.clone());
        }
    })
    .accessibility_label("Text alignment")
    .child(toggle_group_item().value("left").accessibility_label("Align left").child(align_left_icon()))
    .child(toggle_group_item().value("center").accessibility_label("Align center").child(align_center_icon()))
    .child(toggle_group_item().value("right").accessibility_label("Align right").child(align_right_icon()))
```

### The gallery's `Segmented`

The gallery's local `Segmented` (`gallery/ui/controls.tsx`: Density and Appearance in the shell and the Customize
page) is `nsh/base`'s `ToggleGroup` drawn as a tab list (a muted track, the chosen option raised). It maps to
`ToggleGroup` with one value: `options` become `ToggleGroupItem`s (`label` as children; `icon` as an icon child with
`name` as `accessibilityLabel`), `value` / `onChange` as above, `class` passes through, and its height
(`control-height-sm`) is `size="sm"`. Its look becomes shadcn's toggle group (`variant="outline"`, joined), not the tab
list's: the tab-list look belongs to [Tabs](tabs.md), whose `tablist` role would be wrong for a setting. See Decisions 3.

## NSH gaps

1. **The joined look reads the items' places** (`first:` / `last:`). An item the app wraps in a view of its own is
   the first and last child of that view; NSH's `Each` and `If` hand their place on, other wrappers do not. Keep the
   items direct children (or in `Each`). If NSH gave a component its children's count, the group could tell each item
   its place through context instead.
2. **Font weight and cursor are read once** (Button's G2).
3. **String children** (Button's G0).
4. ★ **A shadow on a view without a fill.** The joined outline group carries the `xs` shadow while its items carry
   the fills; a platform that draws a view's shadow from its fill (a layer with no background) may draw none. To check
   on macOS when building; the fallback is the shadow on each item (a line of shadow at each boundary, faint at `xs`).

## Decisions

To settle in review:

1. **The value is a list in both forms** (`value: string[]`, `multiple`), as `nsh/base`'s `Combobox` holds its
   value, rather than shadcn's `type` with a string or a list: one TypeScript type that the Rust props can generate,
   and one name with `Combobox`. A single group's app writes `[x]` and reads `value[0]`.
2. **Items stay toggle buttons for screen readers** (`nsh/base`'s), not radios as in Radix's single group.
3. **The gallery's Segmented takes the toggle group's look** (outline, joined): no "segmented" variant is added,
   which neither shadcn's toggle group nor `nsh/base` has. The alternative, keeping the muted track, is the gallery's
   own `class` on the group and its items.
4. **Logical corners** (start / end), so the joined group mirrors in a right-to-left layout.
