# Switch

Status: implemented (P1). The look is this library's; the
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
| The system switch in a phone's grouped list | A later list row component; this `Switch` draws the library's look on touch too and never becomes the system control |

## Built on

The root is `nsh/base`'s `Switch` (`crates/nsh-base/src/switch.rs`). This library adds the look only; what `nsh/base`
already does is not rewritten:

- Presses: a press on the track or on the label (`children` are inside the press target), or Space / Return while
  focused, reports `Change(!checked)`. The app owns the value and the switch shows the value passed back.
- Focus and screen readers: see Accessibility.
- Parts: a track and a thumb inside it, following checked / disabled / active. This library hides them and draws its
  own track and thumb (see Parts and NSH gaps, 4).
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
nothing to validate on submit; it has no invalid look, and `nsh/base`'s `Switch` has neither `invalid`
nor `accessibilityDescribedBy`. An option that needs validating is a `Checkbox`.

### Events

| TSX | Rust | When |
|---|---|---|
| `onChange({ checked })` | `SwitchEvent::Change(bool)` (`#[args(checked)]`); builder `.on_change(\|checked: &bool\| ..)` | `nsh/base` reports `Change(next)`, forwarded as is |

The name and payload are NSH's, the same as `nsh/base`'s `Switch` and this library's `Checkbox`.

### Ref

TSX: `ref={wifi}` gives a `SwitchRef`; Rust: a `ComponentRef<Switch>` (`api::<SwitchApi>()`), with `focus()` (nothing
while `disabled`). A [Label](label.md)'s `htmlFor` takes it: a press on the label toggles the switch.

### Inside a `Field`

A [Field](field.md) around it hands it its `disabled` (or'ed with its own); its `FieldLabel` toggles it. Its
description is not read with it: `nsh/base`'s `Switch` takes no `accessibilityDescribedBy` ([Field's NSH
gaps](field.md#nsh-gaps)). Outside a `Field` nothing changes.

### Parts

```
Switch root (nsh/base Switch, flex-row items-start, group/nlb-switch)
├─ nsh/base's track and thumb (hidden: display none)
├─ track (a view of this library's)
│   └─ thumb
└─ label (a view of this library's, holding children)
```

The track and thumb are views of this library's, placed first among `nsh/base`'s children (in its press target), and
`nsh/base`'s own are hidden: `nsh/base`'s `trackStyle` / `thumbStyle` setters take a fixed style in Rust, so they could
not carry the app's `trackStyle` / `thumbStyle` after the library's own (NSH gaps, 4). In a terminal the library's
track is not made and `nsh/base`'s terminal switch shows.

## Variants and sizes

No variants. All four sizes. The geometry follows from the thumb's diameter `t` (the token
`switch-thumb-size-<size>`) and the border `b` (`border-width`, 1), in the component:

- The track is 2t wide and t + 2b high, with a transparent border `b` wide that takes a color on focus.
- The track's corners are `min(switch-radius, track height / 2)`; the thumb's are `max(0, track corners − b)`,
  concentric with the track. A pill by default.
- The thumb is t across: at the start while off, moved t − 2b toward the end while on.

| size | Thumb t (pointer) | Track (pointer) | Thumb t (touch) | Track (touch) | Label font size |
|---|---|---|---|---|---|
| `xs` | 12 | 24 × 14 | 16 | 32 × 18 | `font-size-xs` 12 |
| `sm` | 14 | 28 × 16 | 18 | 36 × 20 | `font-size-sm` 13 |
| `md` | 16 | 32 × 18 | 20 | 40 × 22 | `font-size-md` 14 |
| `lg` | 18 | 36 × 20 | 24 | 48 × 26 | `font-size-lg` 14 |

t is half the control height of the size (`control-height-<size>` with a pointer, `control-height-touch-<size>` on
touch), the side of a `Checkbox` of the same size, so the two line up. The medium track is a whole 18 high, not
`1.15rem` (18.4). The thumb's travel, 14, is `translate-x-[calc(100%-2px)]`.

Right to left, the whole track is mirrored (`rtl:-scale-x-100`): the thumb rests at the start while off and at the end
while on.

The label part is `Checkbox`'s: the root is `flex-row items-start`, with `switch-gap` between the track and the label;
`label` has `min-height` = the track's height and `justify-center`; font size `font-size-<size>`, weight
`label-font-weight`, color `label-foreground`, line height from foundation's font size → line height. The track is
`flex-shrink: 0` and takes `Checkbox`'s top margin, so it stays centered on the label's first line. 8 (`space-x-2`) separates a switch and its label.

The corners do not follow the `radius` seed (`rounded-full`), but an app can change `switch-radius` (decided
in review).

## States

Every color is a token in Tokens; the classes are in parentheses. Only the states a switch or the platform needs.

| State | Track | Thumb | Other |
|---|---|---|---|
| Off | `switch` (light `input`; dark `input` 80%, `data-[state=unchecked]:bg-input dark:…bg-input/80`) | `switch-thumb` (light `background`; dark `foreground`) | Track shadow `switch-shadow` (1, `shadow-xs`) |
| On | `switch-checked` (`bg-primary`) | `switch-checked-thumb` (light `background`; dark `primary-foreground`) | |
| hover | none | | The track already says where it is |
| active (held) | | Stretches to 1.25 times its width, from its left end while off and its right end while on (`group-active/nlb-switch`; as macOS and iOS switches do) | No stretch with reduced motion |
| focus-visible (pointer) | The border goes from transparent to `switch-border-focus` (`focus-visible:border-ring`) | | A ring outside the track: `ring-width` 3, color `switch-ring` (`ring-[3px] ring-ring/50`), outside layout |
| focus-visible (touch) | Border `switch-border-focus` | | No ring |
| invalid | not supported | | |
| disabled | | | The whole root at `opacity-disabled`; no press feedback |
| readonly, loading | not supported | | `nsh/base` has neither |

Transitions: the thumb's travel and stretch (`transform`) and the track's fill, border and ring transition over
`duration` (`transition-all` / `transition-transform`, 150 ms) with foundation's easing; none with reduced
motion. A value changed elsewhere (not by a press) animates the same way.

Cursor: always the default arrow.

Implementation notes:

- The colors are bindings that read `checked`, not the `checked:` variant: `nsh/base`'s `checked:` holds only while on
  and not disabled, so a disabled switch that is on would draw as off.
- The stretch while held is `group-active/nlb-switch` on the thumb (`scale-x` 1.25 from the end it rests at), not
  foundation's `press(scale)`, which shrinks evenly. As the track is the library's, `nsh/base`'s projected `active:`
  does not reach it; the named group holds while the switch is held, wherever the press went down, as that does.
- The ring goes on the track: the root gets `group/nlb-switch` and the track `group-focus-visible/nlb-switch` (not
  `nsh/base`'s internal `nsh-switch` group). A test keeps the track following the keyboard focus.

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

Generated from the code's declarations (`NSH_LONGBRIDGE_WRITE=1 cargo test --test tokens`); a test keeps it current.
Values are those of `nsh/base`'s default theme.

<!-- tokens:switch -->
| Token | Type | Derived default | Value |
|---|---|---|---|
| `switch-thumb-size-xs` | length | `pointer_touch(control-height-xs × 0.5, control-height-touch-xs × 0.5)` | 12 (touch 16) |
| `switch-thumb-size-sm` | length | `pointer_touch(control-height-sm × 0.5, control-height-touch-sm × 0.5)` | 14 (touch 18) |
| `switch-thumb-size-md` | length | `pointer_touch(control-height-md × 0.5, control-height-touch-md × 0.5)` | 16 (touch 20) |
| `switch-thumb-size-lg` | length | `pointer_touch(control-height-lg × 0.5, control-height-touch-lg × 0.5)` | 18 (touch 24) |
| `switch-radius` | length | `9999` | 9999 |
| `switch-gap` | length | `spacing × 2` | 8 |
| `switch` | color | `light_dark(input, input × 80%)` |  |
| `switch-checked` | color | `primary` |  |
| `switch-thumb` | color | `light_dark(background, foreground)` |  |
| `switch-checked-thumb` | color | `light_dark(background, primary-foreground)` |  |
| `switch-border-focus` | color | `ring` |  |
| `switch-ring` | color | `ring × ring-opacity` |  |
| `switch-shadow` | number | `1` | 1 |
<!-- /tokens -->

- `switch-radius` 9999 is a pill: the track's corners are `min(switch-radius, track height / 2)`. It does not follow
  `radius` (`rounded-full`).

- `switch-thumb` is light while off in dark (`foreground`); `switch-shadow` 0 draws none.
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

With NSH 0.7.0, write a string child inside `<Text>` for now (see [button.md, NSH gaps](button.md#nsh-gaps), G0).

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
   active already and lacks only focus-visible. Worked around with `group/nlb-switch`, verified.
2. **The projected `checked:` does not hold while disabled.** As `Checkbox`; the colors are bindings.
3. **No `invalid` / `accessibilityDescribedBy`.** `nsh/base`'s `Switch` has neither (`Checkbox` has both). This
   library has no invalid switch, so there is nothing to work around; one later would need them in `nsh/base` for
   screen readers.
4. **`nsh/base`'s part styles take a fixed style in Rust.** As [checkbox.md, NSH gaps](checkbox.md#nsh-gaps), 3.
   `Switch` has no slot for its track, so this library hides `nsh/base`'s track and thumb and draws its own as the
   first child.
5. **A view in the label keeps its one-line width.** As [checkbox.md, NSH gaps](checkbox.md#nsh-gaps), 4.
