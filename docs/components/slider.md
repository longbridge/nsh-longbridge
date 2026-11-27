# Slider

Status: implemented (P3); the histogram, marks and value bubble added after P3.

A value, or a range of two, picked along a track: a volume, a brightness, an opacity, a price range. The behavior is
`nsh/base`'s `Slider`; this library draws it, and can draw around it what a filter or a setting often needs: how the
data spreads over the range (a histogram above the track), named stops (marks below it) and the value in a bubble
above the thumb.

## Usage

- A value within known bounds where the position matters more than the exact number, applied as it moves: volume,
  zoom, opacity, a radius in a theme editor. Two thumbs: a range (a price from 20 to 80, a time window).
- A filter over a range of results: the histogram shows how many fall at each value, so the user sees where the
  results are before dragging (a price, a market cap, a duration); paired Min / Max fields below take an exact value
  (Examples).
- Named stops along a scale (0 / 25 / 50 / 75 / 100, Low / High): `marks`.
- Search terms: slider, range slider, range input, track bar, volume slider, scrubber (not for media), dual slider,
  histogram slider, price filter, distribution, ticks, value label, value tooltip.

## When not to use

| Need | Use instead |
|---|---|
| An exact number, typed or stepped by one | `nsh/base`'s `NumberInput` (wrapped by this library later); the gallery's `Stepper` is that |
| Progress the user does not change | Progress (P3) |
| A few named steps | [ToggleGroup](toggle-group.md), [RadioGroup](radio-group.md) |
| A media timeline with buffering and chapters | The app's own control on `nsh/base`'s `Slider` |
| A chart of the data to read on its own | A chart: the histogram is a guide to the slider, without axes or values |

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

This library adds the look (tokens), the press area's size, and three parts of its own around `nsh/base`'s slider:
the histogram, the marks and the value bubble. No `size`: see Decisions 1.

### Parts and `nsh/base`'s

This library's part names are the ones apps know from web sliders (`track` is the rail); `nsh/base`'s differ:

| This library's part | What | `nsh/base`'s part |
|---|---|---|
| root | The slider: `nsh/base`'s slider itself; with a histogram, marks or a bubble, a column around it and the parts | the root view (with parts: a view of the library's) |
| (the slider row, not exposed) | With parts: `nsh/base`'s slider and the bubbles over it | the root view |
| (the press area, not exposed) | The area taking presses and drags | `track` |
| `track` | The rail | `indicator` |
| `range` | The filled stretch | `range` |
| `thumb` | One per value | `thumb` |
| `histogram` | The bars above the track | (the library's) |
| `marks` | The ticks and labels below the track | (the library's) |
| `bubble` | The value above a thumb | (the library's) |

The value is `nsh/base`'s `SliderValue` (`number | [number, number]`), not a list: one number for one thumb, a pair
for two. `nsh/base` draws no more than two thumbs.

### Parts of one slider, not components beside it

The histogram, the marks and the bubble are props of `Slider`, not components placed next to it
(`<SliderHistogram>` above a `<Slider>`): each must sit exactly where a thumb is, and follow a drag in the same
frame. Only the slider knows both: its length (the rail is the slider's whole width) and the value a drag has
reached before the app's `onChange` has come back as `value`. A component beside it would follow the app's value a
JS turn late, and would line up only while the app keeps the two equally wide (Decisions 5).

### Implementation

- `#[component] fn Slider` whose root is `nsh_base::slider()` itself, as before the parts came, while it has none.
  Given a histogram, marks or a bubble when it is made (a horizontal slider), its root is a column instead: the
  histogram (while given), the slider row (`nsh_base::slider()`, with the bubbles over it), the marks (while given).
  The value, bounds, `step`, `scale`, `axis`, `disabled` and labels are passed through, its events forwarded.
- The parts' looks are style bindings on `nsh/base`'s part style props (set through the builder's props, as the
  builder's setters take a style built once): the geometry, fills, the edge and the thumb's shadow follow the tokens,
  a scope and the appearance, and the app's `<part>Style` is merged last. The thumb's ring under `hover:`, `active:`
  and `focus-visible:` (states only `nsh/base` knows) is a class generated from the resolved tokens, placed before the
  app's `thumbClass`; it is left out while disabled and on touch screens.
- The value the library's parts show is the slider's: the value of a drag from its first change (heard from
  `nsh/base`'s `onChange`, in Rust, in the same frame), until the app has answered the drag's last change
  (`after_answer`, as `nsh/base` does for its thumbs), then the app's `value` (or the slider's own when uncontrolled).
  So the histogram's bars, the marks and the bubbles move with the thumbs, not a turn after them.
- Where a value sits along the track (from 0 at `min` to 1 at `max`, linear or logarithmic) and its default text are
  `nsh/base`'s rules, repeated in the library (NSH gaps 6).
- It joins an enclosing [Field](field.md) (`field::join`, as an inert control): `disabled` from the field (and a
  `FieldSet`); there is nothing for a label's press to do (NSH gaps 3). Beside a `FieldContent` in a horizontal field,
  a plain track (no histogram, marks or bubbles) is centered on the content's first line, as a choice control is
  (Field's Decisions 5).
- The root: as wide as its parent (`w-full`; vertical: as tall, at least `slider-vertical-min-height`),
  `opacity-disabled` while disabled (the histogram and marks dim with the thumbs), the arrow cursor.

## API

### Props

| TSX | Rust | Type | Default | Description |
|---|---|---|---|---|
| `value` | `value` | `number \| [number, number]` / `Prop<Option<SliderValue>>` | Uncontrolled | The value (controlled); a pair makes two thumbs. The app sets it in `onChange` |
| `defaultValue` | `default_value` | `SliderValue` | `0` | The first value while uncontrolled |
| `min` / `max` | `min` / `max` | `number` | `0` / `100` | The bounds |
| `step` | `step` | `number` | `1` | The step values snap to; `0`: any |
| `scale` | `scale` | `"linear"` \| `"logarithmic"` (`SliderScale`) | `"linear"` | How values spread along the track |
| `axis` | `axis` | `"horizontal"` \| `"vertical"` (`Axis`) | `"horizontal"` | The direction (an orientation); read when made |
| `disabled` | `disabled` | `boolean` | `false` | No changes, dimmed |
| `histogram` | `histogram` | `number[]` / `Vec<f64>` | `[]` (none) | How many results fall in each of equal stretches of the track, from `min` to `max`: bars above the track, those between the thumbs (from `min` to a single thumb) in the range's color. Horizontal sliders only. Counts that come later: give zeros of the right length at first (Decisions 6) |
| `marks` | `marks` | `SliderMark[]` (`number \| { value, label? }`) / `Vec<SliderMark>` | `[]` (none) | Ticks below the track at these values, each with an optional label under it. Horizontal sliders only |
| `bubble` | `bubble` | `"none"` \| `"auto"` \| `"always"` (`SliderBubble`) | `"none"` | When the value shows in a bubble above its thumb: never; while the user is on a thumb (under the pointer, dragged, with the keyboard focus); or always. Horizontal sliders only |
| `formatValue` | `format_value` | `(value: number) => string` / `Option<SliderFormat>` | The value with the step's decimals | The bubble's text for a value (`v => "$" + v`, `v => v + "%"`) |
| `accessibilityLabel` | `accessibility_label` | `string` | — | What screen readers call the thumb (a range's end thumb). Set it even beside a label (Input's G1) |
| `startAccessibilityLabel` | `start_accessibility_label` | `string` | `accessibilityLabel` | A range's start thumb ("Minimum price") |
| `class` / `style` | `class` / `style` | | | The root, merged last |
| `trackClass` / `trackStyle` | `track_class` / `track_style` | | | The rail (`nsh/base`'s indicator) |
| `rangeClass` / `rangeStyle` | `range_class` / `range_style` | | | The filled stretch |
| `thumbClass` / `thumbStyle` | `thumb_class` / `thumb_style` | | | Each thumb |
| `histogramClass` / `histogramStyle` | `histogram_class` / `histogram_style` | | | The histogram's box (its height: `h-16`) |
| `marksClass` / `marksStyle` | `marks_class` / `marks_style` | | | The row of ticks and labels |
| `bubbleClass` / `bubbleStyle` | `bubble_class` / `bubble_style` | | | Each bubble |

A part's `<part>Class` follows the library's generated class and overrides it (`trackClass="bg-secondary"`); its
`<part>Style` is merged last. `nsh/base`'s `children` (content after the track) is not exposed: a value label is the
app's, beside the slider, where it lines up with the form's other labels; `marks` and `bubble` cover the labels that
belong to the track itself.

`SliderMark` is a number (a tick without a label) or `{ value: number; label?: string }`; Rust:
`SliderMark::new(25.0)`, `SliderMark::new(0.0).label("Low")`, or `25.0.into()`. A mark outside `min`..`max` is not
drawn.

`SliderFormat` is the app's function: in TSX any `(value: number) => string`; in Rust
`SliderFormat::new(|v| format!("{v}%").into())`. A TSX function answers in the next JS turn (NSH's functions from
Rust): the library asks it ahead for the values shown and their neighbors a step away, keeps its answers while it is
the same function, and shows a value's last answer until a new one comes (NSH gaps 7).

### Events

| TSX | Rust (`SliderEvent`) | Payload | When |
|---|---|---|---|
| `onChange` | `Change(SliderValue)` | `{ value }` | The user changed the value (a press, a drag, a key, a screen reader), each time it changes |
| `onChangeEnd` | `ChangeEnd(SliderValue)` | `{ value }` | The change ended (released, a key's step done): the time to save it |

`nsh/base`'s events, forwarded, under NSH's names (conventions).

### Parts

| Part | What | Customize with |
|---|---|---|
| root | `nsh/base`'s slider; with parts, the column around the histogram, the slider and the marks | `class` / `style` |
| `track` | The rail | `trackClass` / `trackStyle` |
| `range` | From `min` to the thumb, or between two thumbs | `rangeClass` / `rangeStyle` |
| `thumb` | Each thumb | `thumbClass` / `thumbStyle` |
| `histogram` | The bars' box, above the track | `histogramClass` / `histogramStyle`; the bars by the `slider-histogram-*` tokens |
| `marks` | The ticks and labels' row, below the track | `marksClass` / `marksStyle`; ticks and labels by the `slider-mark-*` tokens |
| `bubble` | Each value bubble | `bubbleClass` / `bubbleStyle` |

## Variants and sizes

No variants, no `size` (Decisions 1).

| Part | |
|---|---|
| Press area | Across the slider: `slider-thickness` (pointer 16, the thumb's size; touch 44), the rail centered in it; the whole length |
| Track (rail) | `slider-track-thickness` (6) thick, the whole length, fully rounded, fill `slider-track` (`muted`) |
| Range | Fully rounded, fill `slider-range` (`primary`) |
| Thumb | A circle `slider-thumb-size` (16; touch 20) across, fill `slider-thumb` (white: `background` in light, `foreground` in dark), edge `border-width` × `slider-thumb-border` (`primary`), shadow tier `slider-thumb-shadow` (2, `sm`), centered on its value |
| Vertical | The same turned: the press area `slider-thickness` wide, at least `slider-vertical-min-height` (176) tall |

Decisions in the look:

- **The slider is as tall as its press area (16), not as its rail (6).** Thumbs overflowing a 6-tall slider would
  overlap the rows around it, and presses on a thumb's edge would miss the slider; here the thumbs fit inside its box.
- **The range is rounded itself** rather than clipped by the rail (`overflow-hidden` on the rail would clip
  `nsh/base`'s thumbs, which sit inside it); its end under a thumb is covered by the thumb.
- **The thumb is white in both appearances**, as `light_dark(background, foreground)`: a component has no color
  literals, and in the default themes both resolve to white or near-white. A white thumb reads as the thing to grab
  on both a light and a dark rail.

### Histogram

```
        ▂▅█▇▅▃▂▁                       bars: slider-histogram-bar (muted)
   ▁▂▃▅ ████████ ▆▄▃▂▁▁                in range: slider-histogram-bar-active (the range's color)
   ════●━━━━━━━━●══════════            the thumbs' centers split the bars
```

- A box `slider-histogram-height` (40) tall, as wide as the rail, ending at the top of the thumbs: its bottom
  overlaps the press area by the space above the thumb (none with a pointer, 12 on touch), so the bars sit as close
  to the track on both.
- `histogram` holds one count per bucket; the buckets split the track into equal stretches from `min` to `max`
  (with a logarithmic scale, equal ratios of values). Bucket `i` of `n` covers the track from `i / n` to
  `(i + 1) / n`, the bar inset by half of `slider-histogram-gap` (2) on each side, so the gaps fall on the buckets'
  edges and the edges on the values the thumbs are at.
- Heights scale to the largest count: the largest bar is the box's height; a count above 0 is at least
  `slider-histogram-min-bar` (2) tall, so a single result shows; 0 draws no bar. Negative and non-finite counts
  are 0. The tops are rounded by `slider-histogram-radius` (`radius-xs`).
- Between the thumbs (from `min` to a single thumb) the bars take `slider-histogram-bar-active` (`slider-range`),
  the rest `slider-histogram-bar` (`muted-foreground` at 30%). The split is exactly at the thumbs' centers, where the
  range's ends are: a bar a thumb stands on is split, not colored whole by where most of it lies. The bars are drawn
  twice, the active copy clipped to the range, so the split moves with the thumbs at any position, in the same frame.
- No motion: the bars change color as the thumbs move (pointer-driven), and a new `histogram` replaces the bars at
  once (a filter's counts change while the user drags another filter; animated heights would lag the data).

### Marks

```
   ●━━━━━━━━━━━━━●══════════════
   |      |      |      |      |       ticks: slider-mark (in range: slider-mark-active)
   0     25     50     75    100       labels: slider-mark-label, text-xs
```

- A row below the track, starting at the bottom of the thumbs (as the histogram, it overlaps the press area on
  touch). Each mark is a tick `border-width` wide and `slider-mark-size` (6) long at its value, colored
  `slider-mark-active` (`slider-range`) between the thumbs and `slider-mark` (`muted-foreground` at 50%) elsewhere.
- A label is centered under its tick, `slider-mark-gap` (4) below it, in `slider-mark-label` (`muted-foreground`)
  at `slider-mark-font-size` (12), tabular figures. A label that would cross the slider's ends is held inside them:
  the first and last labels line up with the ends rather than overflow (a label is never cut).
- Labels too close for all of them are thinned: the labels at the two ends (the first and last ticks with a label)
  always show; going from the start, a label between them shows only if it starts at least `slider-mark-gap` after
  the last one shown and ends at least that far before the end's. The others are not drawn (their ticks stay). A
  label shows once every label is measured, so none is first drawn crowded.
- Marks do not change the values a thumb takes: `step` does. Marks on the step's values make the thumbs stop on them
  (`step={25}` with marks at 0, 25, 50, 75, 100).

### Bubble

- A pill above its thumb, centered on the thumb's value: `slider-bubble` fill (`tooltip`, inverted), text
  `slider-bubble-foreground` at `slider-bubble-font-size` (12, tabular figures), padding `slider-bubble-padding-x`
  (8) × `slider-bubble-padding-y` (4), 24 tall, corners `slider-bubble-radius` (`radius-sm`), `slider-bubble-offset` (8; touch
  24, above the finger) above the thumb. It never takes the pointer and does not change the layout: it floats over
  what is above the slider (the app leaves room for `"always"`).
- It follows its thumb in the same frame (pointer-driven, no transition on its place) and fades and zooms in from
  95% over `duration` when it shows (none with reduced motion or with `slider-bubble-animate` 0).
- **A range's two bubbles never overlap**: once they would touch, one bubble between the thumbs shows both values
  (`20 – 80`), in the library's words `value-range` (`"{start} – {end}"`, translated with `setTranslator`; a `DatePicker`'s range
  of dates takes the same words), or one
  value when both read the same.

## States

| State | Look |
|---|---|
| Rest | As above |
| Thumb hover (pointer) | A ring of `slider-thumb-ring-width` (4) in `slider-thumb-ring` (`ring` at 50%) outside the thumb, no layout; `bubble="auto"`: its bubble |
| Thumb dragged (pointer, touch) | The same ring (pointer), kept while the pointer leaves the thumb during the drag; `bubble="auto"`: the dragged thumb's bubble |
| Thumb focus-visible | The same ring; `bubble="auto"`: the focused thumb's bubble (a range: once a key moves it, NSH gaps 8) |
| Touch: dragged, focused | No ring (no hover; focus changes nothing on touch, conventions) |
| Disabled | The root at `opacity-disabled` (the histogram and marks too), no ring, no bubble with `"auto"`, the arrow cursor |
| Invalid | Not offered: a slider's value is always in range; in a [Field](field.md) the error shows in `FieldError` |

`bubble="auto"` in detail:

| Input | The bubble shows |
|---|---|
| Pointer over a thumb | That thumb's, until the pointer leaves it |
| Pointer or finger dragging | The dragged thumb's, from the press (a single thumb; a range's once the drag has moved it: NSH gaps 4) until the release |
| Keyboard focus | The focused thumb's while it has the focus; not after a pointer's press focused it. A range's: none on the focus coming, then the thumb a key moves (NSH gaps 8) |

The ring transitions over `duration` (the thumb's shadows only), none with reduced motion. The thumb and the range
follow the value with no transition (pointer-driven motion, and the keys move them at once). The cursor is the arrow
(gpui-kit's Design Guides).

## Pointer and touch

| | Pointer | Touch |
|---|---|---|
| Thumb | 16 | 20 (`control-height-touch-md` / 2, as a checkbox's box) |
| Press area | 16 across the slider | 44 (`touch-target`) across: it takes layout (a hit slop cannot, as `nsh/base`'s parts take none) |
| Drag | From the thumb or anywhere on the press area | The same; a scroll view around it does not take the drag (`nsh/base`) |
| Ring | Hover, drag, keyboard focus | None |
| Bubble (`"auto"`) | Hover, drag, keyboard focus | While a finger drags, 24 above the thumb so the finger does not cover it |
| Histogram, marks | Do not take presses: a press on them goes through to the press area where they overlap it, and does nothing elsewhere | The same |

iOS's slider is 31 tall and takes touches in 44, Android's is 48: the 44 press area follows them; the look stays a
thin rail.

## Accessibility

`nsh/base`'s: each thumb is `adjustable`, named by `accessibilityLabel` (a range's start by
`startAccessibilityLabel`), with its value and the increment and decrement actions (VoiceOver's swipe up / down,
TalkBack's volume keys); disabled reads as dimmed. While no screen reader runs, a drag sends the thumbs' value to the
accessibility tree once, at its end (`nsh/base`'s, to spare a props update per frame). Each thumb is a keyboard
stop. In a [Field](field.md) the slider
joins it for `disabled`; screen readers do not read the field's description with the thumb (Field's NSH gaps 5).

The library's parts are hidden from screen readers (`accessible(false)`, `accessibility_elements_hidden`):

- **The histogram is decorative.** What it says in numbers (how many results the range holds) is the result count
  the app shows beside the filter ("128 stays"), which a screen reader reads where the user expects it; a summary of
  the bars' shape would be long and not actionable.
- **The marks** repeat values a thumb's value already says.
- **The bubble** repeats the thumb's value, which screen readers read from the thumb. `formatValue` does not reach
  that value (NSH gaps 5): screen readers say the number (`250`, not `$250`).

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
| `slider-histogram-height` | length | `spacing × 10` | 40 |
| `slider-histogram-gap` | length | `spacing × 0.5` | 2 |
| `slider-histogram-min-bar` | length | `spacing × 0.5` | 2 |
| `slider-histogram-radius` | length | `radius-xs` | 2 |
| `slider-histogram-bar` | color | `muted-foreground × 30%` |  |
| `slider-histogram-bar-active` | color | `slider-range` |  |
| `slider-mark` | color | `muted-foreground × 50%` |  |
| `slider-mark-active` | color | `slider-range` |  |
| `slider-mark-size` | length | `spacing × 1.5` | 6 |
| `slider-mark-gap` | length | `spacing` | 4 |
| `slider-mark-label` | color | `muted-foreground` |  |
| `slider-mark-font-size` | length | `12` | 12 |
| `slider-bubble` | color | `tooltip` |  |
| `slider-bubble-foreground` | color | `tooltip-foreground` |  |
| `slider-bubble-font-size` | length | `tooltip-font-size` | 12 |
| `slider-bubble-radius` | length | `radius-sm` | 6 |
| `slider-bubble-padding-x` | length | `spacing × 2` | 8 |
| `slider-bubble-padding-y` | length | `spacing` | 4 |
| `slider-bubble-offset` | length | `pointer_touch(spacing × 2, spacing × 6)` | 8 (touch 24) |
| `slider-bubble-animate` | number | `1` | 1 |
<!-- /tokens -->

- The rail is `slider-track` (the part is `track`); the slider has no bare-name token, having no one surface that is
  "the control".
- `slider-thumb-border` follows `slider-range`: a theme that recolors the range recolors the thumb's edge, so the
  two stay one color. `slider-histogram-bar-active` and `slider-mark-active` follow it too: what is in range is one
  color.
- `slider-thumb-ring-width` is 4, one more than the fields' 3: the ring around a small circle must read as wide as a
  field's.
- The thumb is drawn between 0 and `slider-thickness` across (a theme's larger thumb never outgrows the press area);
  the clamp is the component's.
- The bubble's fill and text follow the [Tooltip](tooltip.md)'s (`tooltip`, `tooltip-foreground`): a theme that
  restyles hints restyles the value bubble, which is one.
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

// A price filter: how the results spread, the range's bars in its color.
<Slider
  min={0}
  max={1000}
  step={10}
  histogram={counts}            // e.g. 40 buckets of 25 each
  value={price()}
  onChange={({ value }) => price.set(value as [number, number])}
  startAccessibilityLabel="Minimum price"
  accessibilityLabel="Maximum price"
/>

// Marks with labels; the step makes the thumb stop on them.
<Slider step={25} marks={[{ value: 0, label: "0" }, 25, { value: 50, label: "50" }, 75, { value: 100, label: "100" }]} accessibilityLabel="Volume" />

// The value in a bubble while the user is on the thumb.
<Slider bubble="auto" formatValue={(v) => `${v}%`} defaultValue={80} accessibilityLabel="Opacity" />

// Uncontrolled, vertical, applied when released.
<Slider axis="vertical" defaultValue={30} onChangeEnd={({ value }) => save(value as number)} accessibilityLabel="Brightness" />

<Slider disabled value={40} accessibilityLabel="Locked" />
```

Paired Min / Max fields (the gallery's recipe): two [Input](input.md)s below a range slider, synced both ways. The
slider sets both fields' text as it moves; a field's text sets its end of the slider while it is a number within the
bounds (on the step's grid or not), and on Return or when the field is left it is held to the bounds and to the other
end (the minimum never past the maximum) and written back as the value.

```tsx
const range = signal<[number, number]>([200, 750]);
const minText = signal("200");
const maxText = signal("750");
const setRange = (v: [number, number]) => { range.set(v); minText.set(String(v[0])); maxText.set(String(v[1])); };
// Typing: follow while it is a number in reach; Return or leaving: hold it and write it back.
const typed = (end: 0 | 1, text: string, commit: boolean) => {
  const n = Number(text.replace(/[^\d.]/g, ""));
  const [lo, hi] = range();
  if (text.trim() === "" || !Number.isFinite(n)) { if (commit) setRange([lo, hi]); return; }
  const next: [number, number] = end === 0 ? [Math.min(Math.max(n, 0), hi), hi] : [lo, Math.max(Math.min(n, 1000), lo)];
  if (commit) setRange(next); else range.set(next);
};

<Slider min={0} max={1000} step={10} histogram={counts} value={range()} onChange={({ value }) => setRange(value as [number, number])} … />
<View class="flex-row gap-3">
  <Input value={minText()} onChange={({ value }) => { minText.set(value); typed(0, value, false); }}
         onSubmit={({ value }) => typed(0, value, true)} onBlur={() => typed(0, minText(), true)} keyboardType="numeric" accessibilityLabel="Minimum price" />
  <Input value={maxText()} … accessibilityLabel="Maximum price" />
</View>
```

Rust:

```rust
use nsh_core::prelude::*;
use nsh_longbridge::{SliderBubble, SliderFormat, SliderMark, SliderValue, slider};

let volume = signal(50.0);

slider()
    .value(move || Some(SliderValue::Single(volume.get())))
    .on_change(move |value: &SliderValue| volume.set(value.end()))
    .max(100.0)
    .step(25.0)
    .marks(vec![SliderMark::new(0.0).label("0"), 25.0.into(), SliderMark::new(50.0).label("50")])
    .bubble(SliderBubble::Auto)
    .format_value(Some(SliderFormat::new(|v| format!("{v}%").into())))
    .accessibility_label("Volume")
```

`SliderValue` and `SliderScale` are `nsh/base`'s types, re-exported (the generated TypeScript declares `SliderValue`
too). `nsh/base` exports a `Slider` as well; an app using both renames one.

### The gallery's `Slider`

The gallery's local `Slider` (`gallery/ui/controls.tsx`, the seed sliders of the Customize page) is `nsh/base`'s
`Slider` with part classes of its own: it becomes this component with the same props (`value`, `min`, `max`, `step`,
`onChange`, `accessibilityLabel`, `class`), and its part classes go. Two visible changes: the thumb is white in dark,
where the gallery's is `bg-background`; the slider is 16 tall, where the gallery's track is 20.

The gallery's `Stepper` (the token editor's − value + row) is a number stepped by buttons: `nsh/base`'s `NumberInput`,
which this library wraps later; it stays local until then.

## NSH gaps

1. **Part styles built once** (`nsh/base`'s `*_style` setters in Rust take a style built once): the look is set as
   bindings on the part style props directly, and the thumb's state ring is a generated class. A setter taking a
   binding would make this the public way.
2. **No hit slop on `nsh/base`'s parts**: the press area takes layout on touch screens (44 across) rather than a slop
   around a thinner slider. A `hitSlop` on `nsh/base`'s track would let the touch slider stay 20 tall.
3. **No focus method on a thumb and no described-by**: a `FieldLabel`'s press cannot focus the slider, and a field's
   description is not read with it (Field's NSH gaps 5). A `focus()` on `nsh/base`'s `Slider` ref and an
   `accessibilityDescribedBy` passed to the thumbs would close both.
4. **No thumb state reported**: `nsh/base`'s `Slider` keeps which thumb is under the pointer, pressed, dragged or
   focused to itself (its `hover:`, `active:`, `focus-visible:` styles). The library reads the pointer's place over
   the slider (to tell a hovered thumb), its presses and the keyboard focus coming into and leaving the slider, and
   learns a range's dragged thumb from the value that changes. So it cannot tell which thumb a finger pressed before
   the drag moves it. A thumb state (`onThumbStateChange({ thumb, hovered, pressed, focusVisible })`, or children
   placed on each thumb) would close it.
5. **No value text for screen readers**: a thumb's accessibility value is `nsh/base`'s text of the number (the
   step's decimals); `formatValue` cannot reach it. A `formatValue` (or `getValueText`) on `nsh/base`'s `Slider`,
   used for both its value text and this library's bubble, would close it.
6. **The value's place and text are `nsh/base`'s private rules** (`numeric::Range`: the fraction along a linear or
   logarithmic track, the default text): the library repeats them to place the histogram's split, the marks and the
   bubbles, and keeps them equal by test. A public `Range` (or the thumbs' fractions) would remove the copy.
7. **A JS function answers in the next JS turn**: the bubble's `formatValue` from TSX is asked ahead (the values
   shown and those a step away) and its answers kept, so keys and slow drags show the formatted text at once; a
   fast drag can show a value's text a turn after the thumb has reached it. A synchronous call from Rust to a pure JS
   function (or a format pattern `nsh/base` applies, as `Intl.NumberFormat`'s options) would remove the lag.
8. **Which thumb has the keyboard focus is `nsh/base`'s own**: it keeps the focused thumb to itself (it steers the
   arrow keys to it), and the focus is heard only coming into and leaving the slider as a whole. So on a range the
   library learns the focused thumb from the value a key changes: Tab onto a range shows no bubble until a key moves
   a thumb, then that thumb's, and a key that changes nothing (at a bound) shows the last thumb moved. The thumb
   state of gap 4 with a `focused` thumb, or a focus listener per thumb, would show the bubble as the focus comes.

## Decisions

Settled in the P3 review:

1. **No `size`.** A slider's size is its length (the app's width); its thickness follows the `control-height` seed
   through `slider-thumb-size`, as a checkbox's box. A dense toolbar slider uses the tokens in a scope.
2. **The press area is the thumb's size with a pointer, 44 on touch** (taking layout), rather than a 6-tall slider
   with overflowing thumbs.
3. **A white thumb in both appearances**, through `light_dark(background, foreground)`.
4. **Part names are the web's** (`track` is the rail), not `nsh/base`'s (`track` is its press area, `indicator` the
   rail); the press area is the library's geometry and not exposed.

Settled with the histogram, marks and bubble:

5. **Props of `Slider`, not components beside it** (Built on). The histogram, marks and bubble line up with the
   thumbs because the slider places them on its own length, and follow a drag in the same frame because the slider
   hears its drag before the app does. Composition (`<SliderHistogram>` next to the slider) would make the app keep
   two widths equal and would lag a turn behind every drag.
6. **Without its parts the root is `nsh/base`'s slider, as it always was**; with a histogram, marks or a bubble it
   is a column around it. A slider without them keeps the tree, the `class` and `style` target and the layout it had
   before the parts came, so an app's classes and its tests see no change. The choice is made once, when the slider
   is made (a root does not change kind once mounted, and making the slider again would drop a drag and the focus):
   a horizontal slider given a non-empty `histogram`, a non-empty `marks` or a `bubble` other than `"none"` then
   takes the column, and its parts may come and go after. An app whose counts arrive later gives zeros of the right
   length meanwhile: zero buckets draw nothing, and the box keeps its height so nothing moves when they come. The
   column's look (width, disabled dimming, the field's share of a row) is the bare slider's.
7. **Buckets are equal stretches of the track**, not of the values the app picks edges for: the bars line up with
   the thumbs and the marks, and a filter's counts are commonly bucketed evenly. With a logarithmic scale a bucket is
   an equal ratio of values (as the track's own spacing). Uneven buckets are the app's to rebucket.
8. **The bars are split at the thumbs**, not colored whole by their centers: the bars and the range agree to the
   point, as the thumb moves through a bucket.
9. **The histogram is decorative for screen readers**; the result count is the app's text (Accessibility).
10. **Horizontal only.** A vertical slider (an equalizer, a level) has no room above or below its track for bars,
    marks or bubbles, and its labels would read sideways; on a vertical slider `histogram`, `marks` and `bubble` are
    ignored. An app that needs them places its own beside the slider.
11. **Marks are a picture of the step, not a second step.** Values snap to `step`; a mark does not attract the
    thumb. A slider whose only values are its marks uses a step equal to their spacing (`nsh/base` has no list of
    allowed values).
12. **`bubble="none"` by default**: the desktops' own sliders show no value bubble, and a form shows the value in
    its label (the gallery's volume row). `"auto"` follows the web's value tooltips: on the thumb under the pointer,
    while dragged, with the keyboard focus (not after a pointer's press, which leaves the focus there).
13. **The bubble is the tooltip's look**: it is a hint about the thumb, so it reads as the library's other hints.
14. **A crowded range shows one bubble** with both values, rather than two overlapping or pushed apart; its words
    are the library's `value-range` label, so a translation can reorder or replace the dash.
15. **`formatValue` is a function, both in TSX and Rust**, rather than a pattern: currencies, compact numbers
    (`$1.2M`) and units are the app's (it already formats them for its labels); NSH gaps 7 for its turn.
16. **Crowded mark labels are thinned, not moved or shrunk**: the end labels always show (they name the bounds),
    and a label between them that would touch its neighbor is left out, its tick kept. Moving labels off their ticks
    would misplace the values, and smaller text would not read; the app still picks fewer labels for a tidy row.
