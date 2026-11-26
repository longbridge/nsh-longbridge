# Checkbox

Status: API spec, reviewed 2026-11-26, not yet implemented. The look follows shadcn/ui new-york v4 `checkbox.tsx`; the
behavior is `nsh/base`'s `Checkbox`.

`Checkbox`, [`Switch`](switch.md) and [`RadioGroup`](radio-group.md) are one family of choice controls: they handle
focus, disabled, invalid, the label and the hit area the same way. A change to one of those rules in one spec is made
in all three.

## Usage

A checkbox: accepting terms in a form, an item of a multiple selection, a "select all" row (indeterminate). The value
takes effect when the form is submitted or confirmed.

## When not to use

| Case | Use instead |
|---|---|
| A setting that takes effect at once (on / off) | `Switch` |
| Exactly one of a set | `RadioGroup`; `Select` (P2) for a long set |
| A toggle or formatting button in a toolbar | `Toggle` / `ToggleGroup` (later) |
| A checkable menu item | `DropdownMenu`'s checkbox item (P2); never a `Checkbox` inside a menu |
| A whole card that is the option, a form item with a description and an error line | `Checkbox` inside `Field` (P3); this component has no card variant |

## Built on

The root is `nsh/base`'s `Checkbox` (`crates/nsh-base/src/checkbox.rs`). This library adds the look only; what
`nsh/base` already does is not rewritten:

- Presses: a press on the box or on the label (`children` are inside the press target), or Space / Return while
  focused, reports `Change(next)`: `true` from unchecked or indeterminate, `false` from checked. The app owns the value
  and the checkbox shows the value passed back.
- Focus and screen readers: see Accessibility.
- `hitSlop`: widens where presses count, with no change to layout or look.
- Terminal: `nsh/base` shows `[x]` / `[ ]` / `[-]` in place of the indicator. The box and mark drawn here do not
  appear there; the label does.

This library adds the box, the check / dash mark, the label's layout, each state's colors, sizes, the touch hit area
and the press feedback.

## API

### Props

| TSX | Rust | Type | Default | Notes |
|---|---|---|---|---|
| `checked` | `checked` | `boolean` / `Prop<bool>` | `false` | Controlled value |
| `indeterminate` | `indeterminate` | `boolean` / `Prop<bool>` | `false` | Wins over `checked`; a press asks for `true` |
| `disabled` | `disabled` | `boolean` / `Prop<bool>` | `false` | |
| `invalid` | `invalid` | `boolean` / `Prop<bool>` | `false` | Look in States; announced as invalid |
| `size` | `size` | `Size` | `"md"` | `xs` / `sm` / `md` / `lg` |
| `accessibilityLabel` | `accessibility_label` | `string?` / `Prop<Option<SharedString>>` | — | Required when `children` have no text |
| `accessibilityHint` | `accessibility_hint` | `string?` | — | Passed to `nsh/base` |
| `accessibilityDescribedBy` | `accessibility_described_by` | `ElementTarget?` | — | Error / help line, passed to `nsh/base` |
| `hitSlop` | `hit_slop` | `EdgeInsets?` | automatic (Pointer and touch) | When given, used as is |
| `children` | `children` | string or elements | — | The label, inside the `label` part; a string is wrapped in a text |
| `class` / `style` | `class` / `style` | | | The root (the row: box and label), merged last |
| `indicatorClass` / `indicatorStyle` | `indicator_class` / `indicator_style` | | | The box |
| `markClass` / `markStyle` | `mark_class` / `mark_style` | | | The check / dash icon inside the box |
| `labelClass` / `labelStyle` | `label_class` / `label_style` | | | The view around `children`; not made without `children` |

Controlled only: no `defaultChecked` (the app owns the value). `nsh/base`'s `role`, `indicator` slot, `checkedStyle`
and `indeterminateStyle` are not exposed: checkable menu items are `DropdownMenu`'s; the mark is drawn here, sized and
colored with `markClass` / `markStyle`, and the state colors are the tokens in Tokens.

### Events

| TSX | Rust | When |
|---|---|---|
| `onChange({ checked })` | `CheckboxEvent::Change(bool)` (`#[args(checked)]`); builder `.on_change(\|checked: &bool\| ..)` | `nsh/base` reports `Change(next)`, forwarded as is |

The name and payload are NSH's (conventions, "Controlled"), the same as `nsh/base`'s `Checkbox`.

### Parts

```
Checkbox root (nsh/base Checkbox, flex-row items-start, group/nlb-checkbox)
├─ indicator (nsh/base's indicator view: the box)
│   └─ mark (check or minus icon, made only while checked or indeterminate)
└─ label (a view of this library's, holding children)
```

## Variants and sizes

No variants (shadcn has none). All four sizes. The box's side is the token `checkbox-size-<size>`; the rest follows it:

| size | Box (pointer) | Box (touch) | Check / dash icon | Label font size |
|---|---|---|---|---|
| `xs` | 12 | 16 | box − 2 | `font-size-xs` 12 |
| `sm` | 14 | 18 | box − 2 | `font-size-sm` 13 |
| `md` | 16 | 20 | box − 2 (14, shadcn's `size-3.5`) | `font-size-md` 14 |
| `lg` | 18 | 24 | box − 2 | `font-size-lg` 14 |

The box's side is half the control height of the size (`control-height-<size>` with a pointer,
`control-height-touch-<size>` on touch), so it scales with the `control-height` seed. `md` with a pointer is 16,
shadcn's `size-4`.

Fixed geometry, in the component, not tokens:

- The check and the dash are foundation icons sized explicitly to box − 2 (not `Size`'s default icon sizes
  12 / 14 / 16 / 20, which are a button's), at the icon's default stroke, centered. Icons take the text color: the box's
  text color is `checkbox-checked-foreground`, and the mark follows it.
- Corners `checkbox-radius`, at most a quarter of the box's side, so a theme with large radii does not turn the box
  into a circle that reads as a radio.
- The root is `flex-row items-start`, with `checkbox-gap` between the box and the label.
- The `label` part: `min-height` = the box's side and `justify-center`, so a one-line label is centered on the box and
  a longer one starts at its top. Font size `font-size-<size>`, weight `label-font-weight`, color `label-foreground`
  (Label's tokens); line height from foundation's font size → line height. Text children inherit them.
- The box is `flex-shrink: 0`.

Differences from shadcn: shadcn's checkbox has one size, 16, and its label is a separate `Label`. Here there are four
sizes (conventions), and the label is `children`, since `nsh/base` puts `children` in the press target and the
accessible name: a press on the label is a press on the box, with no `htmlFor`. Box-to-label gap 8 (shadcn's examples
use `gap-3`, 12; `Label` itself uses `gap-2`, 8): the desktop's medium density.

## States

Every color is a token in Tokens; shadcn's classes are in parentheses. Only the states shadcn has (conventions, "States").

| State | Box fill | Box border | Mark | Other |
|---|---|---|---|---|
| Unchecked | `checkbox` (light: transparent; dark: `input` 30%, `dark:bg-input/30`) | `checkbox-border` (`border-input`) | none | Shadow `checkbox-shadow` (1, `shadow-xs`) |
| Checked | `checkbox-checked` (`bg-primary`) | `checkbox-checked-border` (`border-primary`) | `check` in `checkbox-checked-foreground` (`text-primary-foreground`) | |
| Indeterminate | as checked | as checked | `minus`, same color | shadcn v4 draws none; `nsh/base` has the state, so the dash is added |
| hover | none | | | shadcn has none |
| active (pressed) | | | | foundation `press(0.9)` on the box; no scaling with reduced motion |
| focus-visible (pointer) | | `checkbox-border-focus` while unchecked (`focus-visible:border-ring`); checked / indeterminate keep their border | | A ring outside the box: `ring-width` 3, color `checkbox-ring` (`ring-[3px] ring-ring/50`), outside layout |
| focus-visible (touch) | | `checkbox-border-focus` in every state (`checkbox-border-invalid` while invalid) | | No ring |
| invalid | | `checkbox-border-invalid` while unchecked (`aria-invalid:border-destructive`); checked / indeterminate keep their border | | With a pointer, the focus ring is `checkbox-ring-invalid` (`aria-invalid:ring-destructive/20 dark:…/40`) |
| disabled | | | | The whole root at `opacity-disabled` (box and label together, as shadcn's `disabled:opacity-50` with Label's `peer-disabled:opacity-50`); no press feedback |
| readonly, loading | not supported | | | Neither shadcn nor `nsh/base` has them |

The border's precedence with a pointer (the order of shadcn's classes in Tailwind 4): checked / indeterminate > invalid
> focus-visible > rest. On touch there is no ring, so the focus border wins over the checked one and yields only to
invalid.

Transitions: only the ring transitions (shadcn's `transition-shadow`, over `duration`); fill, border and mark change
with the value at once (shadcn's indicator is `transition-none`).

Cursor: always the default arrow, disabled too.

Implementation notes:

- The checked / indeterminate colors are bindings that read `checked` / `indeterminate`, not the `checked:` variant
  `nsh/base` projects onto the indicator: that variant holds only while checked and not disabled, so a disabled checked
  box would draw as unchecked.
- The ring and the press go on the box, but `nsh/base` projects only checked / indeterminate / disabled onto the
  indicator. The root gets `group/nlb-checkbox` and the box `group-focus-visible/nlb-checkbox` and
  `group-active/nlb-checkbox` (nsh-core has `group_focus_visible_named` / `group_active_named`). **Verify this first when
  implementing** (NSH gaps, 1).

## Pointer and touch

| | Pointer | Touch |
|---|---|---|
| Box side | `control-height-<size>` / 2 (12 / 14 / 16 / 18) | `control-height-touch-<size>` / 2 (16 / 18 / 20 / 24) |
| Hit area | none added | Without `children`, `max(0, (touch-target − box) / 2)` on each side, 44 square, look unchanged; with a label the label is the target and nothing is added (slop would reach into the next row) |
| Focus | Ring outside plus border color | Border color only |
| Hover | none | none |

Pointer / touch is foundation's `pointer_touch`. A `hitSlop` the app gives is used as is.

## Accessibility

All of it is `nsh/base`'s:

- Focus: a stop of the keyboard focus where pressable views are (macOS, the web); Space / Return toggles. While
  `disabled`, no presses and no press feedback.
- Screen readers: role `checkbox`; checked / unchecked / mixed (`indeterminate` wins over `checked`); `disabled` reads
  as dimmed; `invalid` is announced each time it is reached; the name is the text of `children`, else
  `accessibilityLabel`. `accessibilityHint` and `accessibilityDescribedBy` (an error or help line, read after the name)
  are passed through.
- The focus ring is drawn on the box (States); with a pointer only when the keyboard moved the focus there
  (`focus-visible`).

## Tokens

| Token | Type | Derived default | Value |
|---|---|---|---|
| `checkbox-size-xs` | length | `pointer_touch(control-height-xs × 0.5, control-height-touch-xs × 0.5)` | 12 (touch 16) |
| `checkbox-size-sm` | length | `pointer_touch(control-height-sm × 0.5, control-height-touch-sm × 0.5)` | 14 (touch 18) |
| `checkbox-size-md` | length | `pointer_touch(control-height-md × 0.5, control-height-touch-md × 0.5)` | 16 (touch 20) |
| `checkbox-size-lg` | length | `pointer_touch(control-height-lg × 0.5, control-height-touch-lg × 0.5)` | 18 (touch 24) |
| `checkbox-radius` | length | `max(radius-sm − 2, 0)` | 4 (shadcn's `rounded-[4px]`) |
| `checkbox-gap` | length | `spacing` × 2 | 8 |
| `checkbox` | color | `light_dark(transparent, input × 30%)` | transparent; dark `dark:bg-input/30` |
| `checkbox-border` | color | `input` | |
| `checkbox-checked` | color | `primary` | |
| `checkbox-checked-border` | color | `checkbox-checked` | follows the fill |
| `checkbox-checked-foreground` | color | `primary-foreground` | |
| `checkbox-border-focus` | color | `ring` | |
| `checkbox-border-invalid` | color | `destructive` | |
| `checkbox-ring` | color | `ring` × `ring-opacity` (50%) | |
| `checkbox-ring-invalid` | color | `ring-invalid` | `destructive` 20%, dark 40% |
| `checkbox-shadow` | number | 1 (`xs`) | 0 for none |

- An app that changes the touch size alone writes `checkbox-size-<size>-touch` (tokens.md's general rule).
- `checked` takes the variant's place (`checkbox-checked-border`): state suffixes are only `-hover`, `-active`, `-focus`,
  `-invalid` and `-disabled`, and checked is a value, not an interaction state.
- The label uses Label's `label-font-weight` and `label-foreground` and the semantic `font-size-<size>`; no tokens of its
  own. The ring's geometry is foundation's.
- Changing `primary` changes the checked fill and border; changing `checkbox-checked` changes the border too
  (`checkbox-checked-border` refers to it).

## Examples

TSX:

```tsx
import { signal, View } from "nsh";
import { Checkbox } from "nsh-longbridge";

const agreed = signal(false);
const items = signal<string[]>(["mentions"]);
const all = ["mentions", "approvals", "summary"];

<View class="gap-3">
  <Checkbox checked={agreed()} invalid={!agreed()} onChange={({ checked }) => agreed.set(checked)}>
    Accept terms and conditions
  </Checkbox>

  {/* Select all: indeterminate while only some are selected */}
  <Checkbox
    checked={items().length === all.length}
    indeterminate={items().length > 0 && items().length < all.length}
    onChange={({ checked }) => items.set(checked ? all : [])}
  >
    All notifications
  </Checkbox>

  {/* A box alone needs a name */}
  <Checkbox size="sm" checked={agreed()} onChange={({ checked }) => agreed.set(checked)} accessibilityLabel="Select row" />

  <Checkbox disabled checked>Locked option</Checkbox>
</View>
```

Rust:

```rust
use nsh_core::prelude::*;
use nsh_longbridge::{Size, checkbox};

let agreed = signal(false);

view()
    .gap_3()
    .child(
        checkbox()
            .checked(move || agreed.get())
            .invalid(move || !agreed.get())
            .on_change(move |checked: &bool| agreed.set(*checked))
            .child(text("Accept terms and conditions")),
    )
    .child(
        checkbox()
            .size(Size::Sm)
            .checked(move || agreed.get())
            .on_change(move |checked: &bool| agreed.set(*checked))
            .accessibility_label("Select row"),
    )
```

## NSH gaps

1. **The indicator does not follow the root's focus-visible, hover or active.** `nsh/base`'s `Checkbox` projects only
   checked / indeterminate / disabled onto the indicator (`project_*` in `checkbox.rs`); `Switch` adds active; neither
   projects focus-visible. Workaround: `group/nlb-checkbox` on the root and named group variants on the box (States).
   Verify first, when implementing, that a `group` declaration given to an `nsh/base` component's element merges into
   its root, and that `indicatorStyle` carries group variants. If either fails, draw the ring around the whole root
   (as gpui-kit does) and report it.
2. **The projected `checked:` does not hold while disabled.** It means checked and not disabled, so a disabled checked
   box drawn with the variant would look unchecked. Workaround: bindings that read the value (States). Not a bug; noted so
   the implementation does not trip on it.
