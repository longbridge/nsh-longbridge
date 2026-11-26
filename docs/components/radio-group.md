# RadioGroup

Status: API spec, reviewed 2026-11-26, not yet implemented. The look follows shadcn/ui new-york v4 `radio-group.tsx`
(`RadioGroup` and `RadioGroupItem`); the behavior is `nsh/base`'s `RadioGroup` and `Radio`.

Focus, disabled, invalid, the label and the hit area are handled as in [`Checkbox`](checkbox.md) and
[`Switch`](switch.md). A change to one of those rules in one spec is made in all three.

## Usage

A radio group: exactly one of a small set of options, all visible (a plan, a sort order, how often to notify).

## When not to use

| Case | Use instead |
|---|---|
| Many options (about five or more), or little room | `Select` (P2) |
| Several options, or none | `Checkbox`, or a later `ToggleGroup` |
| One setting on or off | `Switch` |
| Switching pages or views | `Tabs` (P2) |
| One of two or more as buttons (a segmented control) | A later `ToggleGroup` / segmented control; this component has no button variant |
| A whole card that is the option | `RadioGroupItem` inside `Field` (P3); this component has no card variant |

## Built on

`RadioGroup`'s root is `nsh/base`'s `RadioGroup`, and each `RadioGroupItem`'s root is `nsh/base`'s `Radio`
(`crates/nsh-base/src/radio.rs`). What `nsh/base` already does is not rewritten:

- Presses: a press on an unchecked item (its circle or its label: `children` are inside the press target), or Space /
  Return while focused, reports `Change(true)`. A checked item reports nothing when pressed (a radio cannot uncheck
  itself).
- The keyboard's roving focus and screen readers: see Accessibility.
- `hitSlop`: widens where an item's presses count, with no change to layout or look.
- Terminal: `nsh/base` shows `[●]` / `[ ]` before the item. This library draws no circle there (no `indicator` part
  while `platform.idiom === "terminal"`); the label shows as usual.

`nsh/base`'s `RadioGroup` holds no value; each `Radio` takes its own `checked`. As in shadcn, this library puts the
value on the group: `RadioGroup` takes `value` and passes it to its items through context; an item is checked when
the group's `value` equals the item's `value`; when an item reports `Change(true)`, the group reports
`onChange({ value: <the item's value> })`.

This library adds the group's value and event, the circle and dot, the label's layout, each state's colors, sizes, the
touch hit area, the press feedback, and the group's `disabled` and `invalid`.

## API

### `RadioGroup` props

| TSX | Rust | Type | Default | Notes |
|---|---|---|---|---|
| `value` | `value` | `string?` / `Prop<Option<SharedString>>` | none | Controlled; none checked when absent or matching no item |
| `disabled` | `disabled` | `boolean` / `Prop<bool>` | `false` | Disables every item (or-ed with each item's `disabled`); with all disabled the group is no stop of the focus |
| `invalid` | `invalid` | `boolean` / `Prop<bool>` | `false` | Every item's look (States); not announced (NSH gaps, 1) |
| `size` | `size` | `Size` | `"md"` | Every item; `xs` / `sm` / `md` / `lg` |
| `accessibilityLabel` | `accessibility_label` | `string?` | — | The group's name; give it always |
| `children` | `children` | elements | — | The items, and whatever the app lays out between them |
| `class` / `style` | `class` / `style` | | | The group's root, merged last; a row is `class="flex-row"` |

### `RadioGroupItem` props

| TSX | Rust | Type | Default | Notes |
|---|---|---|---|---|
| `value` | `value` | `string` / `Prop<SharedString>` | required | The value the item stands for, unique in the group |
| `disabled` | `disabled` | `boolean` / `Prop<bool>` | `false` | |
| `accessibilityLabel` | `accessibility_label` | `string?` | — | Required when `children` have no text |
| `hitSlop` | `hit_slop` | `EdgeInsets?` | automatic (Pointer and touch) | When given, used as is |
| `children` | `children` | string or elements | — | The label, inside the `label` part; a string is wrapped in a text |
| `class` / `style` | `class` / `style` | | | The item's root (the row: circle and label), merged last |
| `indicatorClass` / `indicatorStyle` | `indicator_class` / `indicator_style` | | | The circle |
| `dotClass` / `dotStyle` | `dot_class` / `dot_style` | | | The dot inside the circle, made only while checked |
| `labelClass` / `labelStyle` | `label_class` / `label_style` | | | The view around `children`; not made without `children` |

`invalid` and `size` are set on the group only and the items follow it (decided in review): a required choice and its
size belong to the whole set. A `RadioGroupItem` must be inside a `RadioGroup`; outside one, debug builds warn, the
item draws unchecked and its presses report nothing.

Not provided: `defaultValue` (controlled only); `orientation` (`nsh/base` takes both pairs of arrows, so the direction
is only layout, set with `class`); `required` / `name` (NSH has no form submission; the app validates and sets
`invalid`).

### Events

| Component | TSX | Rust | When |
|---|---|---|---|
| `RadioGroup` | `onChange({ value })` | `RadioGroupEvent::Change(SharedString)` (`#[args(value)]`); builder `.on_change(\|value: &SharedString\| ..)` | An item reports `Change(true)`: an unchecked item was pressed, or an arrow key moved the focus to it |

`RadioGroupItem` has no events. The name and payload follow NSH's value state (`value` with `onChange({ value })`),
not Radix's `onValueChange`.

### Parts

```
RadioGroup root (nsh/base RadioGroup, flex-col, gap radio-group-gap)
└─ RadioGroupItem root (nsh/base Radio, flex-row items-start, group/nlb-radio)
    ├─ indicator (the circle, drawn by this library)
    │   └─ dot (made while checked)
    └─ label (a view of this library's, holding children)
```

## Variants and sizes

No variants (shadcn has none). All four sizes, set on the group:

| size | Circle (pointer) | Circle (touch) | Dot | Label font size |
|---|---|---|---|---|
| `xs` | 12 | 16 | circle / 2 | `font-size-xs` 12 |
| `sm` | 14 | 18 | circle / 2 | `font-size-sm` 13 |
| `md` | 16 | 20 | circle / 2 (8, shadcn's `size-2`) | `font-size-md` 14 |
| `lg` | 18 | 24 | circle / 2 | `font-size-lg` 14 |

The circle's side is half the control height of the size (`control-height-<size>` with a pointer,
`control-height-touch-<size>` on touch): the side of a `Checkbox`'s box and a `Switch`'s thumb of the same size, so
the three line up side by side. `md` with a pointer is 16, shadcn's `size-4`.

Fixed geometry, in the component:

- The circle is round (corners = side / 2, not following the `radius` seed), with a `border-width` border and
  `flex-shrink: 0`; the dot is round and centered.
- An item's root is `flex-row items-start`, with `radio-group-item-gap` between the circle and the label.
- The `label` part is `Checkbox`'s: `min-height` = the circle's side and `justify-center`; font size
  `font-size-<size>`, weight `label-font-weight`, color `label-foreground`, line height from foundation's font size →
  line height; text children inherit them.
- The group's root is `flex-col` with `radio-group-gap` between items (shadcn's `grid gap-3`, the same in one column).

Differences from shadcn: shadcn's item is the circle alone and its label a separate `Label`. Here the label is the
item's `children`, since `nsh/base` puts them in the press target and the accessible name.

## States

Every color is a token in Tokens; shadcn's classes are in parentheses. Only the states shadcn has.

| State | Circle fill | Circle border | Dot | Other |
|---|---|---|---|---|
| Unchecked | `radio-group-item` (light: transparent; dark: `input` 30%, `dark:bg-input/30`) | `radio-group-item-border` (`border-input`) | none | Shadow `radio-group-item-shadow` (1, `shadow-xs`) |
| Checked | unchanged | `radio-group-item-checked-border` (the unchecked border by default: shadcn keeps it) | `radio-group-item-dot` (`fill-primary`) | |
| hover | none | | | shadcn has none |
| active (pressed) | | | | foundation `press(0.9)` on the circle; no scaling with reduced motion |
| focus-visible (pointer) | | `radio-group-item-border-focus` (`focus-visible:border-ring`) | | A ring outside the circle: `ring-width` 3, color `radio-group-item-ring` (`ring-[3px] ring-ring/50`), outside layout |
| focus-visible (touch) | | `radio-group-item-border-focus` | | No ring |
| invalid | | `radio-group-item-border-invalid` (`aria-invalid:border-destructive`), checked too | | With a pointer, the focus ring is `radio-group-item-ring-invalid` (`aria-invalid:ring-destructive/20 dark:…/40`) |
| disabled | | | | The item's whole root at `opacity-disabled` (circle and label together); no press feedback; skipped by the arrows |
| readonly, loading | not supported | | | Neither shadcn nor `nsh/base` has them |

The border's precedence: invalid > focus-visible > checked > rest (shadcn has no checked border, and its `aria-`
variants come after `focus-visible`). The same with a pointer and on touch.

Transitions: the circle's border color and the ring transition (shadcn's `transition-[color,box-shadow]`, over
`duration`); the dot appears and disappears with the value at once.

Cursor: always the default arrow.

Implementation notes:

- An item's colors and dot are bindings that read the `checked` this library computes (the group's `value` equals the
  item's), not the `checked:` variant (`nsh/base`'s `checked:` holds only while checked and not disabled).
- The ring and the press go on the circle: the item's root gets `group/nlb-radio` and the circle
  `group-focus-visible/nlb-radio` and `group-active/nlb-radio`. **Verify this first when implementing**; if it fails,
  draw the ring around the item's root and report it ([checkbox.md, NSH gaps](checkbox.md#nsh-gaps), 1).
- The group's `disabled` goes to each `nsh/base` `Radio`'s `disabled` (or-ed), so `nsh/base`'s roving focus skips them.

## Pointer and touch

| | Pointer | Touch |
|---|---|---|
| Circle side | `control-height-<size>` / 2 (12 / 14 / 16 / 18) | `control-height-touch-<size>` / 2 (16 / 18 / 20 / 24) |
| Hit area | none added | For an item without `children`, `max(0, (touch-target − circle) / 2)` on each side, look unchanged; with a label the label is the target and nothing is added |
| Focus | Ring outside plus border color | Border color only |
| Hover | none | none |
| Item gap | `radio-group-gap` 12 | the same |

## Accessibility

All of it is `nsh/base`'s, except the group's `disabled`, which this library passes to each item:

- Keyboard (WAI-ARIA's radio group, where pressable views are stops of the focus: macOS, the web): the group is one
  stop of Tab, at the checked item, else the first that is not disabled. Down and Right move to the next item, Up and
  Left to the previous, around the ends and past disabled items, and the item the focus moves to reports
  `Change(true)`. Both pairs of arrows work in a row and in a column. Space checks the focused item.
- Screen readers: the group has role `radiogroup`, named by `accessibilityLabel` and read on entering it; each item has
  role `radio` and states both checked and selected (screen readers read one or the other); `disabled` reads as
  dimmed; an item's name is the text of its `children`, else its `accessibilityLabel`. `invalid` is not announced
  (NSH gaps, 1).
- The focus ring is drawn on the circle (States); with a pointer only when the keyboard moved the focus there
  (`focus-visible`).

## Tokens

| Token | Type | Derived default | Value |
|---|---|---|---|
| `radio-group-gap` | length | `spacing` × 3 | 12 (follows the `spacing` seed) |
| `radio-group-item-size-xs` | length | `pointer_touch(control-height-xs × 0.5, control-height-touch-xs × 0.5)` | 12 (touch 16) |
| `radio-group-item-size-sm` | length | `pointer_touch(control-height-sm × 0.5, control-height-touch-sm × 0.5)` | 14 (touch 18) |
| `radio-group-item-size-md` | length | `pointer_touch(control-height-md × 0.5, control-height-touch-md × 0.5)` | 16 (touch 20) |
| `radio-group-item-size-lg` | length | `pointer_touch(control-height-lg × 0.5, control-height-touch-lg × 0.5)` | 18 (touch 24) |
| `radio-group-item-gap` | length | `spacing` × 2 | 8 |
| `radio-group-item` | color | `light_dark(transparent, input × 30%)` | transparent in light |
| `radio-group-item-border` | color | `input` | |
| `radio-group-item-checked-border` | color | `radio-group-item-border` | |
| `radio-group-item-border-focus` | color | `ring` | |
| `radio-group-item-border-invalid` | color | `destructive` | |
| `radio-group-item-ring` | color | `ring` × `ring-opacity` (50%) | |
| `radio-group-item-ring-invalid` | color | `ring-invalid` | `destructive` 20%, dark 40% |
| `radio-group-item-dot` | color | `primary` | |
| `radio-group-item-shadow` | number | 1 (`xs`) | 0 for none |

- An app that changes the touch size alone writes `radio-group-item-size-<size>-touch`.
- `checked` takes the variant's place (`radio-group-item-checked-border`): state suffixes are only `-hover`,
  `-active`, `-focus`, `-invalid` and `-disabled`, and checked is a value, not an interaction state.
- A theme that wants the checked border in the primary color: `radio-group-item-checked-border: var(primary)`.
- The label uses Label's `label-*` tokens. The ring's geometry is foundation's.

## Examples

TSX:

```tsx
import { signal } from "nsh";
import { RadioGroup, RadioGroupItem } from "nsh-longbridge";

const plan = signal<string | undefined>("monthly");
const sort = signal("name");

<RadioGroup
  accessibilityLabel="Billing"
  value={plan()}
  invalid={plan() === undefined}
  onChange={({ value }) => plan.set(value)}
>
  <RadioGroupItem value="monthly">Monthly</RadioGroupItem>
  <RadioGroupItem value="yearly">Yearly</RadioGroupItem>
  <RadioGroupItem value="lifetime" disabled>Lifetime</RadioGroupItem>
</RadioGroup>

{/* A row, small */}
<RadioGroup class="flex-row" size="sm" accessibilityLabel="Sort by" value={sort()} onChange={({ value }) => sort.set(value)}>
  <RadioGroupItem value="name">Name</RadioGroupItem>
  <RadioGroupItem value="date">Date</RadioGroupItem>
</RadioGroup>
```

Rust:

```rust
use nsh_core::prelude::*;
use nsh_longbridge::{radio_group, radio_group_item};

let plan = signal(Some(SharedString::from("monthly")));

radio_group()
    .accessibility_label("Billing")
    .value(move || plan.get())
    .invalid(move || plan.get().is_none())
    .on_change(move |value: &SharedString| plan.set(Some(value.clone())))
    .child(radio_group_item().value("monthly").child(text("Monthly")))
    .child(radio_group_item().value("yearly").child(text("Yearly")))
    .child(radio_group_item().value("lifetime").disabled(true).child(text("Lifetime")))
```

## NSH gaps

1. **`Radio` and `RadioGroup` have no `invalid` or `accessibilityDescribedBy`.** `nsh/base`'s `Checkbox` has both;
   `Radio` and `RadioGroup` have neither (and `Radio` has no `accessibilityHint`). This library's `invalid` is a look
   only: screen readers do not hear it, and an error line cannot be attached. No accessibility state is set on
   `nsh/base`'s element from outside: it would replace the checked / selected state `nsh/base` gives it. Workaround:
   the app shows the error next to the group and announces it when it appears (P3's `Field` will). `nsh/base` needs
   `invalid` and `accessibilityDescribedBy` on `RadioGroup` (WAI-ARIA puts both on the radiogroup).
2. **`RadioGroup` holds no value and has no `disabled`.** This library adds both through its own context (Built on, States).
   Not a defect; noted so nobody expects `nsh/base` to handle them.
3. **The parts do not follow focus-visible or active.** As `Checkbox`'s NSH gap 1; `Radio` has no indicator part at all
   (the circle is one this library puts among the children), so named groups are the only way.
4. **The projected `checked:` does not hold while disabled.** As `Checkbox`; bindings.
