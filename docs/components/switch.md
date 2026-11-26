# Switch

Status: API spec, reviewed 2026-11-26, not yet implemented. The look follows shadcn/ui new-york v4 `switch.tsx`; the
behavior is `nsh/base`'s `Switch`.

Focus, disabled, invalid, the label and the hit area are handled as in [`Checkbox`](checkbox.md) and
[`RadioGroup`](radio-group.md). A change to one of those rules in one spec is made in all three.

## Usage

A switch: a setting that takes effect at once (notifications, dark mode, auto-renew).

## When not to use

| Case | Use instead |
|---|---|
| An option that takes effect on submit, accepting terms, a required item | `Checkbox` |
| One of a set | `RadioGroup` / `Select` (P2) |
| A toolbar button that stays pressed | `Toggle` (later) |
| The system switch in a phone's grouped list | A later list row component; this `Switch` draws shadcn's look on touch too and never becomes the system control |

## Built on

The root is `nsh/base`'s `Switch` (`crates/nsh-base/src/switch.rs`). This library adds the look only; what `nsh/base`
already does is not rewritten:

- Presses: a press on the track or on the label (`children` are inside the press target), or Space / Return while
  focused, reports `Change(!checked)`. The app owns the value and the switch shows the value passed back.
- Focus and screen readers: see Accessibility.
- Parts: a track and a thumb inside it. Both follow checked / disabled, and active while the switch is held, wherever
  the press went down, the label included (`project_group_active`).
- `hitSlop`: widens where presses count, with no change to layout or look.
- Terminal: `nsh/base` shows a checkbox's mark in place of the track and thumb. The track drawn here does not appear
  there; the label does.

This library adds the track's and thumb's geometry and colors, the thumb's motion, the label's layout, sizes, the touch
hit area and the press feedback.

## API

### Props

| TSX | Rust | Type | Default | Notes |
|---|---|---|---|---|
| `checked` | `checked` | `boolean` / `Prop<bool>` | `false` | Controlled value |
| `disabled` | `disabled` | `boolean` / `Prop<bool>` | `false` | |
| `size` | `size` | `Size` | `"md"` | `xs` / `sm` / `md` / `lg` |
| `accessibilityLabel` | `accessibility_label` | `string?` | — | Required when `children` have no text |
| `accessibilityHint` | `accessibility_hint` | `string?` | — | Passed to `nsh/base` |
| `hitSlop` | `hit_slop` | `EdgeInsets?` | automatic (Pointer and touch) | When given, used as is |
| `children` | `children` | string or elements | — | The label, inside the `label` part; a string is wrapped in a text |
| `class` / `style` | `class` / `style` | | | The root (the row: track and label), merged last |
| `trackClass` / `trackStyle` | `track_class` / `track_style` | | | The track |
| `thumbClass` / `thumbStyle` | `thumb_class` / `thumb_style` | | | The thumb |
| `labelClass` / `labelStyle` | `label_class` / `label_style` | | | The view around `children`; not made without `children` |

Controlled only: no `defaultChecked`. No `invalid` (decided in review): a switch takes effect at once, so there is
nothing to validate on submit; shadcn's switch has no invalid look, and `nsh/base`'s `Switch` has neither `invalid`
nor `accessibilityDescribedBy`. An option that needs validating is a `Checkbox`.

### Events

| TSX | Rust | When |
|---|---|---|
| `onChange({ checked })` | `SwitchEvent::Change(bool)` (`#[args(checked)]`); builder `.on_change(\|checked: &bool\| ..)` | `nsh/base` reports `Change(next)`, forwarded as is |

The name and payload are NSH's, the same as `nsh/base`'s `Switch` and this library's `Checkbox`.

### Parts

```
Switch root (nsh/base Switch, flex-row items-start, group/nlb-switch)
├─ track
│   └─ thumb
└─ label (a view of this library's, holding children)
```

## Variants and sizes

No variants. All four sizes. The geometry follows from the thumb's diameter `t` (the token
`switch-thumb-size-<size>`) and the border `b` (`border-width`, 1), in the component:

- The track is 2t wide and t + 2b high, with a transparent border `b` wide that takes a color on focus.
- The track's corners are `min(switch-radius, track height / 2)`; the thumb's are `max(0, track corners − b)`,
  concentric with the track. A pill by default.
- The thumb is t across: at the start while off, moved t − 2b toward the end while on.

| size | Thumb t (pointer) | Track (pointer) | Thumb t (touch) | Track (touch) | Label font size |
|---|---|---|---|---|---|
| `xs` | 12 | 24 × 14 (shadcn's `size="sm"`) | 16 | 32 × 18 | `font-size-xs` 12 |
| `sm` | 14 | 28 × 16 | 18 | 36 × 20 | `font-size-sm` 13 |
| `md` | 16 | 32 × 18 (shadcn's `default`) | 20 | 40 × 22 | `font-size-md` 14 |
| `lg` | 18 | 36 × 20 | 24 | 48 × 26 | `font-size-lg` 14 |

t is half the control height of the size (`control-height-<size>` with a pointer, `control-height-touch-<size>` on
touch), the side of a `Checkbox` of the same size, so the two line up. shadcn's default track is `1.15rem` (18.4) high;
here it is a whole 18. The thumb's travel, 14, is shadcn's `translate-x-[calc(100%-2px)]`.

Right to left, the whole track is mirrored (`rtl:-scale-x-100`): the thumb rests at the start while off and at the end
while on.

The label part is `Checkbox`'s: the root is `flex-row items-start`, with `switch-gap` between the track and the label;
`label` has `min-height` = the track's height and `justify-center`; font size `font-size-<size>`, weight
`label-font-weight`, color `label-foreground`, line height from foundation's font size → line height. The track is
`flex-shrink: 0`. shadcn's examples put 8 (`space-x-2`) between a switch and its `Label`; so does this library.

The corners do not follow the `radius` seed (shadcn's `rounded-full`), but an app can change `switch-radius` (decided
in review).

## States

Every color is a token in Tokens; shadcn's classes are in parentheses. Only the states shadcn or the platform has.

| State | Track | Thumb | Other |
|---|---|---|---|
| Off | `switch` (light `input`; dark `input` 80%, `data-[state=unchecked]:bg-input dark:…bg-input/80`) | `switch-thumb` (light `background`; dark `foreground`) | Track shadow `switch-shadow` (1, `shadow-xs`) |
| On | `switch-checked` (`bg-primary`) | `switch-checked-thumb` (light `background`; dark `primary-foreground`) | |
| hover | none | | shadcn has none |
| active (held) | | Stretches to 1.25 times its width, from its left end while off and its right end while on (the `active:` `nsh/base` projects; as macOS and iOS switches do) | No stretch with reduced motion |
| focus-visible (pointer) | The border goes from transparent to `switch-border-focus` (`focus-visible:border-ring`) | | A ring outside the track: `ring-width` 3, color `switch-ring` (`ring-[3px] ring-ring/50`), outside layout |
| focus-visible (touch) | Border `switch-border-focus` | | No ring |
| invalid | not supported | | |
| disabled | | | The whole root at `opacity-disabled`; no press feedback |
| readonly, loading | not supported | | Neither shadcn nor `nsh/base` has them |

Transitions: the thumb's travel and stretch (`transform`) and the track's fill, border and ring transition over
`duration` (shadcn's `transition-all` / `transition-transform`, 150 ms) with foundation's easing; none with reduced
motion. A value changed elsewhere (not by a press) animates the same way.

Cursor: always the default arrow.

Implementation notes:

- The colors are bindings that read `checked`, not the `checked:` variant: `nsh/base`'s `checked:` holds only while on
  and not disabled, so a disabled switch that is on would draw as off.
- The stretch while held uses the `active:` `nsh/base` already projects onto the thumb, not foundation's
  `press(scale)`, which shrinks evenly.
- The ring goes on the track, but `nsh/base` does not project focus-visible onto the parts: the root gets
  `group/nlb-switch` and the track `group-focus-visible/nlb-switch` (not `nsh/base`'s internal `nsh-switch` group).
  **Verify this first when implementing**; if it fails, draw the ring around the whole root and report it
  ([checkbox.md, NSH gaps](checkbox.md#nsh-gaps), 1).

## Pointer and touch

| | Pointer | Touch |
|---|---|---|
| Size | Variants and sizes, pointer columns | Variants and sizes, touch columns |
| Hit area | none added | Without `children`, `max(0, (touch-target − track height) / 2)` above and below and `max(0, (touch-target − track width) / 2)` on each side, look unchanged; with a label the label is the target and nothing is added |
| Focus | Ring outside plus border color | Border color only |
| Hover | none | none |

## Accessibility

All of it is `nsh/base`'s:

- Focus: a stop of the keyboard focus; Space / Return toggles. While `disabled`, no presses and no press feedback.
- Screen readers: role `switch`, on / off; `disabled` reads as dimmed; the name is the text of `children`, else
  `accessibilityLabel`; `accessibilityHint` is passed through. No invalid state and no described-by (NSH gaps, 3).
- The focus ring is drawn on the track (States); with a pointer only when the keyboard moved the focus there
  (`focus-visible`).

## Tokens

| Token | Type | Derived default | Value |
|---|---|---|---|
| `switch-thumb-size-xs` | length | `pointer_touch(control-height-xs × 0.5, control-height-touch-xs × 0.5)` | 12 (touch 16) |
| `switch-thumb-size-sm` | length | `pointer_touch(control-height-sm × 0.5, control-height-touch-sm × 0.5)` | 14 (touch 18) |
| `switch-thumb-size-md` | length | `pointer_touch(control-height-md × 0.5, control-height-touch-md × 0.5)` | 16 (touch 20) |
| `switch-thumb-size-lg` | length | `pointer_touch(control-height-lg × 0.5, control-height-touch-lg × 0.5)` | 18 (touch 24) |
| `switch-radius` | length | 9999 (a pill; the corners are `min(it, track height / 2)`) | does not follow `radius` |
| `switch-gap` | length | `spacing` × 2 | 8 |
| `switch` | color | `light_dark(input, input × 80%)` | |
| `switch-checked` | color | `primary` | |
| `switch-thumb` | color | `light_dark(background, foreground)` | in dark, the thumb is light while off |
| `switch-checked-thumb` | color | `light_dark(background, primary-foreground)` | |
| `switch-border-focus` | color | `ring` | |
| `switch-ring` | color | `ring` × `ring-opacity` (50%) | |
| `switch-shadow` | number | 1 (`xs`) | 0 for none |

- An app that changes the touch size alone writes `switch-thumb-size-<size>-touch`.
- `checked` takes the variant's place (`switch-checked-thumb`): state suffixes are only `-hover`, `-active`, `-focus`,
  `-invalid` and `-disabled`, and checked is a value, not an interaction state.
- No invalid, so no `-invalid` tokens. The label uses Label's `label-*` tokens. The ring's geometry is foundation's.

## Examples

TSX:

```tsx
import { signal, View } from "nsh";
import { Switch } from "nsh-longbridge";

const airplane = signal(false);

<View class="gap-3">
  <Switch checked={airplane()} onChange={({ checked }) => airplane.set(checked)}>
    Airplane mode
  </Switch>

  {/* The label is elsewhere: a track alone needs a name */}
  <Switch size="xs" checked={airplane()} onChange={({ checked }) => airplane.set(checked)} accessibilityLabel="Airplane mode" />

  <Switch disabled checked>Managed by your organization</Switch>
</View>
```

Rust:

```rust
use nsh_core::prelude::*;
use nsh_longbridge::{Size, switch};

let airplane = signal(false);

view()
    .gap_3()
    .child(
        switch()
            .checked(move || airplane.get())
            .on_change(move |checked: &bool| airplane.set(*checked))
            .child(text("Airplane mode")),
    )
    .child(
        switch()
            .size(Size::Xs)
            .checked(move || airplane.get())
            .on_change(move |checked: &bool| airplane.set(*checked))
            .accessibility_label("Airplane mode"),
    )
```

## NSH gaps

1. **The parts do not follow focus-visible.** As [checkbox.md, NSH gaps](checkbox.md#nsh-gaps), 1; `Switch` projects
   active already and lacks only focus-visible.
2. **The projected `checked:` does not hold while disabled.** As `Checkbox`; the colors are bindings.
3. **No `invalid` / `accessibilityDescribedBy`.** `nsh/base`'s `Switch` has neither (`Checkbox` has both). This
   library has no invalid switch, so there is nothing to work around; one later would need them in `nsh/base` for
   screen readers.
