# Slider

Status: spec (P3), not implemented.

A value, or a range of two, picked along a track: a volume, a brightness, an opacity, a price range. The look is
shadcn/ui new-york v4's `slider.tsx` (Radix Slider); the behavior is `nsh/base`'s `Slider`.

## Usage

- A value within known bounds where the position matters more than the exact number, applied as it moves: volume,
  zoom, opacity, a radius in a theme editor. Two thumbs: a range (a price from 20 to 80, a time window).
- Search terms: slider, range slider, range input, track bar, volume slider, scrubber (not for media), dual slider.

## When not to use

| Need | Use instead |
|---|---|
| An exact number, typed or stepped by one | `nsh/base`'s `NumberInput` (wrapped by this library later); the gallery's `Stepper` is that |
| Progress the user does not change | Progress (P3) |
| A few named steps | [ToggleGroup](toggle-group.md), [RadioGroup](radio-group.md) |
| A media timeline with buffering and chapters | The app's own control on `nsh/base`'s `Slider` |

## Built on

`nsh/base`'s `Slider` (`crates/nsh-base/src/slider.rs`). It has no look; this library adds it. What `nsh/base`
already does, not rewritten:

| Provided by `nsh/base` | |
|---|---|
| Value | `value` (controlled: a number, or `[start, end]` for two thumbs) / `defaultValue`; shown held to `min` and `max`, a range's ends in order; `min`, `max`, `step` (from `min`, `max` always reachable; `0` for any value), `scale` (`linear` or `logarithmic`) |
| Pointer and touch | A press on the track moves the thumb there and drags it (on a range, the start's before the middle of the two thumbs, else the end's); a thumb dragged by its offset (no jump under the pointer); a range's thumbs never cross; the thumb follows the pointer in the same frame, before the app's `value` comes back; a scroll view around it does not take the drag |
| Keyboard | On a focused thumb (each thumb is a stop where pressables are): Right / Up raise, Left / Down lower (a horizontal slider in a right-to-left layout mirrored), Page Up / Down by a tenth of the range (at least a step), Home / End to `min` / `max`; arrows and pages repeat while held |
| Events | `onChange({ value })` as often as it changes during a drag; `onChangeEnd({ value })` once when the change ends |
| Axis | `axis`: horizontal (`min` at the start) or vertical (`min` at the bottom); read when made |
| Accessibility | Each thumb `adjustable`, with its value (`min`, `max`, `now`), its label (`accessibilityLabel`, `startAccessibilityLabel` for a range's start thumb), increment and decrement actions; disabled |
| Terminal | `─` / `━` / `●` on the Cell grid; this library's look does not apply there |

This library adds the look (tokens) and the press area's size. No `size`: see Decisions 1.

### Differs from shadcn's structure

The same parts, under the names this library exposes (shadcn's), drawn on `nsh/base`'s:

| This library's part | shadcn | `nsh/base`'s part |
|---|---|---|
| root | `Slider` root | the root view |
| (the press area, not exposed) | — (the root) | `track`: the area taking presses and drags |
| `track` | `SliderTrack`: the rail | `indicator` |
| `range` | `SliderRange` | `range` |
| `thumb` | `SliderThumb`, one per value | `thumb` |

The value is `nsh/base`'s `SliderValue` (`number | [number, number]`), not shadcn's `number[]`: one number for one
thumb, a pair for two. `nsh/base` draws no more than two thumbs.

### Implementation

- `#[component] fn Slider` whose root is `nsh_base::slider()`, the value, bounds, `step`, `scale`, `axis`,
  `disabled` and labels passed through, its events forwarded.
- The parts' looks go on `nsh/base`'s part props as bound styles (the library writes the props struct's
  `track_style`, `indicator_style`, `range_style` and `thumb_style` itself, as Select writes its rows': Select's NSH
  gaps 1): geometry, fills, the edge and the thumb's shadow, following the tokens, the scope and the appearance, the
  app's `<part>Style` merged after the library's.
- The thumb's ring under the pointer, while dragged and with the keyboard focus is a `thumbClass` the library
  generates in a binding from the resolved `slider-thumb-ring` and `slider-thumb-ring-width` (`hover:`, `active:`,
  `focus-visible:`, each appearance's color under `dark:`), the app's `thumbClass` appended: an inline style has no
  states (Select's NSH gaps 1).
- The root: as wide as its parent (`w-full`; vertical: as tall, at least `slider-vertical-min-height`),
  `opacity-disabled` while disabled, the arrow cursor.

## API

### Props

| TSX | Rust | Type | Default | Description |
|---|---|---|---|---|
| `value` | `value` | `number \| [number, number]` / `Prop<Option<SliderValue>>` | Uncontrolled | The value (controlled); a pair makes two thumbs. The app sets it in `onChange` |
| `defaultValue` | `default_value` | `SliderValue` | `0` | The first value while uncontrolled |
| `min` / `max` | `min` / `max` | `number` | `0` / `100` | The bounds |
| `step` | `step` | `number` | `1` | The step values snap to; `0`: any |
| `scale` | `scale` | `"linear"` \| `"logarithmic"` (`SliderScale`) | `"linear"` | How values spread along the track |
| `axis` | `axis` | `"horizontal"` \| `"vertical"` (`Axis`) | `"horizontal"` | shadcn's `orientation`; read when made |
| `disabled` | `disabled` | `boolean` | `false` | No changes, dimmed |
| `accessibilityLabel` | `accessibility_label` | `string` | — | What screen readers call the thumb (a range's end thumb). Set it even beside a label (Input's G1) |
| `startAccessibilityLabel` | `start_accessibility_label` | `string` | `accessibilityLabel` | A range's start thumb ("Minimum price") |
| `class` / `style` | `class` / `style` | | | The root, merged last |
| `trackClass` / `trackStyle` | `track_class` / `track_style` | | | The rail (`nsh/base`'s indicator) |
| `rangeClass` / `rangeStyle` | `range_class` / `range_style` | | | The filled stretch |
| `thumbClass` / `thumbStyle` | `thumb_class` / `thumb_style` | | | Each thumb |

The part looks are inline styles (Implementation): a part's `<part>Style` wins over them, its `<part>Class` adds only
what the look does not set (as Select's `contentClass`), so a different rail color goes in `trackStyle` or the tokens.
`nsh/base`'s `children` (content after the track) is not exposed: shadcn's slider has none, and a value label is the
app's, beside it.

### Events

| TSX | Rust (`SliderEvent`) | Payload | When |
|---|---|---|---|
| `onChange` | `Change(SliderValue)` | `{ value }` | The user changed the value (a press, a drag, a key, a screen reader), each time it changes |
| `onChangeEnd` | `ChangeEnd(SliderValue)` | `{ value }` | The change ended (released, a key's step done): shadcn's `onValueCommit` |

`nsh/base`'s events, forwarded; NSH's names (conventions), shadcn's `onValueChange` / `onValueCommit`.

### Parts

| Part | What | Customize with |
|---|---|---|
| root | `nsh/base`'s root view | `class` / `style` |
| `track` | The rail | `trackClass` / `trackStyle` |
| `range` | From `min` to the thumb, or between two thumbs | `rangeClass` / `rangeStyle` |
| `thumb` | Each thumb | `thumbClass` / `thumbStyle` |

## Variants and sizes

No variants, no `size` (shadcn has one size). shadcn:

- Root: `relative flex w-full touch-none items-center select-none data-[disabled]:opacity-50`; vertical `h-full
  min-h-44 w-auto flex-col`.
- Track: `relative grow overflow-hidden rounded-full bg-muted h-1.5 w-full` (vertical `h-full w-1.5`).
- Range: `absolute bg-primary h-full` (vertical `w-full`).
- Thumb: `block size-4 shrink-0 rounded-full border border-primary bg-white shadow-sm ring-ring/50
  transition-[color,box-shadow] hover:ring-4 focus-visible:ring-4 focus-visible:outline-hidden`.

| Part | This library |
|---|---|
| Press area | Across the slider: `slider-thickness` (pointer 16, the thumb's size; touch 44), the rail centered in it; the whole length |
| Track (rail) | `slider-track-thickness` (6) thick, the whole length, fully rounded, fill `slider-track` (`muted`) |
| Range | Fully rounded, fill `slider-range` (`primary`) |
| Thumb | A circle `slider-thumb-size` (16; touch 20) across, fill `slider-thumb` (white: `background` in light, `foreground` in dark), edge `border-width` × `slider-thumb-border` (`primary`), shadow tier `slider-thumb-shadow` (2, `sm`), centered on its value |
| Vertical | The same turned: the press area `slider-thickness` wide, at least `slider-vertical-min-height` (176) tall |

Differences from shadcn:

- **The slider is as tall as its press area (16), not as its rail (6).** Radix's thumbs overflow a 6-tall root; here
  the thumbs fit inside the slider's box, so the rows around it do not overlap them and presses on a thumb's edge
  reach the slider.
- **The range is rounded itself** rather than clipped by the rail (`overflow-hidden` on the rail would clip
  `nsh/base`'s thumbs, which sit inside it); its end under a thumb is covered by the thumb.
- **The thumb's white** is `light_dark(background, foreground)`: shadcn writes `bg-white` in both appearances, and a
  component has no color literals. In shadcn's themes both resolve to white or near-white.

## States

| State | Look | shadcn |
|---|---|---|
| Rest | As above | — |
| Thumb hover (pointer) | A ring of `slider-thumb-ring-width` (4) in `slider-thumb-ring` (`ring` at 50%) outside the thumb, no layout | `hover:ring-4` |
| Thumb dragged (pointer) | The same ring, kept while the pointer leaves the thumb during the drag | — (Radix keeps hover) |
| Thumb focus-visible | The same ring | `focus-visible:ring-4` |
| Touch: dragged, focused | No ring (no hover; focus changes nothing on touch, conventions) | — |
| Disabled | The root at `opacity-disabled`, no ring, the arrow cursor | `data-[disabled]:opacity-50` |
| Invalid | Not offered (shadcn has none); in a [Field](field.md) the error shows in `FieldError` | — |

The ring transitions over `duration` (shadcn's `transition-[color,box-shadow]`), none with reduced motion. The thumb
and the range follow the value with no transition (pointer-driven motion, and Radix's keys move them at once). The
cursor is the arrow (gpui-kit's Design Guides; Radix sets none).

## Pointer and touch

| | Pointer | Touch |
|---|---|---|
| Thumb | 16 | 20 (`control-height-touch-md` / 2, as a checkbox's box) |
| Press area | 16 across the slider | 44 (`touch-target`) across: it takes layout (a hit slop cannot, as `nsh/base`'s parts take none) |
| Drag | From the thumb or anywhere on the press area | The same; a scroll view around it does not take the drag (`nsh/base`) |
| Ring | Hover, drag, keyboard focus | None |

iOS's slider is 31 tall and takes touches in 44, Android's is 48: the 44 press area follows them; the look stays
shadcn's thin rail.

## Accessibility

`nsh/base`'s: each thumb is `adjustable`, named by `accessibilityLabel` (a range's start by
`startAccessibilityLabel`), with its value and the increment and decrement actions (VoiceOver's swipe up / down,
TalkBack's volume keys); disabled reads as dimmed. While no screen reader runs, a drag sends the thumbs' value to the
accessibility tree once, at its end (`nsh/base`'s, to spare a props update per frame). Each thumb is a keyboard
stop. In a [Field](field.md) the slider
joins it for `disabled`; screen readers do not read the field's description with the thumb (Field's NSH gaps 5).

## Tokens

Generated from the code's declarations (`NSH_LONGBRIDGE_WRITE=1 cargo test --test tokens`); a test keeps it current.
Values are those of `nsh/base`'s default theme.

<!-- tokens:slider -->
| Token | Type | Derived default | Value |
|---|---|---|---|
| `slider-thickness` | length | `pointer_touch(slider-thumb-size, touch-target)` | 16 (touch 44) |
| `slider-vertical-min-height` | length | `spacing × 44` | 176 |
| `slider-track-thickness` | length | `spacing × 1.5` | 6 |
| `slider-track` | color | `muted` |  |
| `slider-range` | color | `primary` |  |
| `slider-thumb-size` | length | `pointer_touch(control-height-md × 0.5, control-height-touch-md × 0.5)` | 16 (touch 20) |
| `slider-thumb` | color | `light_dark(background, foreground)` |  |
| `slider-thumb-border` | color | `slider-range` |  |
| `slider-thumb-shadow` | number | `2` | 2 |
| `slider-thumb-ring` | color | `ring × ring-opacity` |  |
| `slider-thumb-ring-width` | length | `ring-width + 1` | 4 |
<!-- /tokens -->

- The rail is `slider-track` (the part is `track`); the slider has no bare-name token, having no one surface that is
  "the control".
- `slider-thumb-border` follows `slider-range`: a theme that recolors the range recolors the thumb's edge, as shadcn's
  both are `primary`.
- `slider-thumb-ring-width` is shadcn's `ring-4`, one more than the fields' `ring-3`: the ring around a small circle
  must read as wide as a field's.
- The thumb is drawn between 0 and `slider-thickness` across (a theme's larger thumb never outgrows the press area);
  the clamp is the component's.
- Other tokens it reads: `border-width`, `opacity-disabled`, `shadow`, `duration`.

## Examples

TSX:

```tsx
import { signal, Text, View } from "nsh";
import { Slider } from "nsh-longbridge";

const volume = signal(50);
const price = signal<[number, number]>([20, 80]);

<Slider value={volume()} onChange={({ value }) => volume.set(value as number)} max={100} step={1} accessibilityLabel="Volume" />

// A range: two thumbs.
<View class="gap-2">
  <View class="flex-row justify-between">
    <Text class="text-sm font-medium">Price</Text>
    <Text class="text-sm text-muted-foreground tabular-nums">{`$${price()[0]} – $${price()[1]}`}</Text>
  </View>
  <Slider
    value={price()}
    onChange={({ value }) => price.set(value as [number, number])}
    step={5}
    startAccessibilityLabel="Minimum price"
    accessibilityLabel="Maximum price"
  />
</View>

// Uncontrolled, vertical, applied when released.
<Slider axis="vertical" defaultValue={30} onChangeEnd={({ value }) => save(value as number)} accessibilityLabel="Brightness" />

<Slider disabled value={40} accessibilityLabel="Locked" />
```

Rust:

```rust
use nsh_core::prelude::*;
use nsh_longbridge::{SliderValue, slider};

let volume = signal(50.0);

slider()
    .value(move || Some(SliderValue::Single(volume.get())))
    .on_change(move |value: &SliderValue| volume.set(value.end()))
    .max(100.0)
    .accessibility_label("Volume")
```

`SliderValue` and `SliderScale` are `nsh/base`'s types, re-exported (the generated TypeScript declares `SliderValue`
too). `nsh/base` exports a `Slider` as well; an app using both renames one.

### The gallery's `Slider`

The gallery's local `Slider` (`gallery/ui/controls.tsx`, the seed sliders of the Customize page) is `nsh/base`'s
`Slider` with shadcn's classes: it becomes this component with the same props (`value`, `min`, `max`, `step`,
`onChange`, `accessibilityLabel`, `class`), and its part classes go. Two visible changes: the thumb is white in dark
(shadcn's), where the gallery's is `bg-background`; the slider is 16 tall, where the gallery's track is 20.

The gallery's `Stepper` (the token editor's − value + row) is a number stepped by buttons: `nsh/base`'s `NumberInput`,
which this library wraps later; it stays local until then.

## NSH gaps

1. **Part styles built once, inline styles without states** (Select's NSH gaps 1): the library writes the part props
   bound, and the thumb's ring is a generated class. The same accepted limits: an app's `<part>Class` cannot override
   the look (its `<part>Style` can).
2. **No hit slop on `nsh/base`'s parts**: the press area takes layout on touch screens (44 across) rather than a slop
   around a thinner slider. A `hitSlop` on `nsh/base`'s track would let the touch slider stay 20 tall.
3. **No focus method on a thumb and no described-by**: a `FieldLabel`'s press cannot focus the slider, and a field's
   description is not read with it (Field's NSH gaps 5). A `focus()` on `nsh/base`'s `Slider` ref and an
   `accessibilityDescribedBy` passed to the thumbs would close both.

## Decisions

To settle in review:

1. **No `size`.** shadcn's slider has one size, and a slider's size is its length (the app's width); its thickness
   follows the `control-height` seed through `slider-thumb-size`, as a checkbox's box. A dense toolbar slider uses the
   tokens in a scope. If `size` is wanted for consistency with the other controls, the thumb would be
   `control-height-<size>` / 2 (12 / 14 / 16 / 18) and the rail scale with it.
2. **The press area is the thumb's size with a pointer, 44 on touch** (taking layout), rather than shadcn's 6-tall
   root with overflowing thumbs.
3. **shadcn's white thumb in both appearances**, through `light_dark(background, foreground)`.
4. **Part names are shadcn's** (`track` is the rail), not `nsh/base`'s (`track` is its press area, `indicator` the
   rail); the press area is the library's geometry and not exposed.
