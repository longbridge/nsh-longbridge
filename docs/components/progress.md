# Progress

Status: spec (P3), not implemented.

A horizontal bar showing how far a task is: shadcn/ui new-york v4 `progress.tsx`'s look on `nsh/base`'s `Progress`.

## Usage

- A task with a known extent: an upload, a download, an import, the steps of a setup, a quota used.
- With an unknown extent (connecting, preparing), the indeterminate form.
- Search terms: progress, progress bar, loading bar, upload progress, meter, quota bar.

## When not to use

| Need | Use instead |
|---|---|
| A short wait of unknown length in a control or a small place | [Spinner](spinner.md); a button's own wait is `Button`'s `loading` |
| Content still loading, in its shape | [Skeleton](skeleton.md) |
| A value the user sets | Slider (P3) |
| A level in a range that is not progress (disk use, a score) | Still `Progress` for a simple bar (screen readers say a percentage); a gauge of the app's own for more |
| Steps with their names | A stepper of the app's own; `Progress` shows only how far |

## Built on

`nsh/base`'s `Progress` (`crates/nsh-base/src/progress.rs`): it owns the value and what screen readers get, and has no
look.

| Provided by `nsh/base` (not rewritten here) | |
|---|---|
| Value | `value` from 0 to 100, clamped (NaN is 0); the indicator's width is that share of the track's while determinate |
| Indeterminate | `indeterminate`: no value for screen readers; the indicator's width is left to the app |
| Accessibility | One `progressbar` element, range 0 to 100, its value (none while indeterminate), named by `accessibilityLabel` or its children's text |
| Parts | A track and the indicator inside it, then the children (a label, the value as text); `trackClass` / `trackStyle`, `indicatorClass` / `indicatorStyle` |
| Terminal | A one-row bar of `━` and `─` in the indicator's color, a moving segment while indeterminate; this library's look does not apply there |

This library adds the look (the track and indicator from tokens), the indicator's motion as the value changes (a
scale, not a width, so it can transition), and the indeterminate form's moving segment.

### Implementation

- `#[component] fn Progress` whose root is `nsh_base::progress()`, `value`, `indeterminate`, `accessibilityLabel` and
  `children` passed through.
- **The look is classes generated from the resolved tokens, before the app's `trackClass` / `indicatorClass`**
  (conventions, "Drawing": a part of an `nsh/base` component), so `trackClass="h-1"` wins over the library's height;
  the app's `trackStyle` / `indicatorStyle` win over both.
- **The track**: `w-full`, height `progress-height`, corners `progress-radius`, fill `progress`, clipped
  (`overflow: hidden`).
- **The indicator** is the full width of the track (`w-full`, which wins over `nsh/base`'s width by value: its rule,
  a `w-*` in `indicatorClass` is the app's) and is scaled along x to `value / 100` from its start
  (`origin-left`, `origin-right` in a right-to-left layout), fill `progress-indicator`. A width changes at once in NSH;
  a scale transitions. The track's clip and corners round its start, as shadcn's `translateX(-…%)` indicator inside
  `overflow-hidden rounded-full` does; its end is straight, as shadcn's.
- The scale is the one inline style the library writes (`indicatorStyle`, bound to `value`; the app's
  `indicatorStyle` merged after), and transitions over `duration` (shadcn's `transition-all`), none with reduced
  motion.
- **Indeterminate**: the indicator is a segment of `progress-indeterminate-width` (40%) of the track that moves from
  beyond the start to beyond the end, again and again, each pass taking `progress-indeterminate-duration` (1500 ms),
  with the standard easing. With reduced motion it stays still across the whole track at 50% opacity (a full bar
  would read as done; an empty one as not started). See NSH gaps 1 for how the loop is driven.
- `children` go under the track (`nsh/base` places them after it): the root is a column with `progress-gap` (8)
  between the track and them when there are any.

## API

| TSX | Rust | Type | Default | Description |
|---|---|---|---|---|
| `value` | `value` | `number` | `0` | How far, from 0 to 100 (clamped) |
| `indeterminate` | `indeterminate` | `boolean` | `false` | The extent is unknown: the moving segment, no value for screen readers |
| `accessibilityLabel` | `accessibility_label` | `string` | — | What is in progress ("Uploading report.pdf"), when the children do not say it |
| `trackClass` / `trackStyle` | `track_class` / `track_style` | | — | The track: height (`h-1`), radius, fill; after its look |
| `indicatorClass` / `indicatorStyle` | `indicator_class` / `indicator_style` | | — | The indicator: its fill (`bg-green-600`); after its look |
| `class` / `style` | `class` / `style` | | — | The root: width (`w-[60%]`), margins; merged last |
| `children` | `.child(..)` | string, elements | — | Under the track: a label, the value as text; screen readers read them as the name |

No events: the app owns the value. No `size` and no variants (shadcn has neither): one 8-point bar, its height a token
or `trackClass`. Coming from shadcn: `<Progress value={33} className="w-[60%]" />` is the same; shadcn's `className`
styles the track, which is also its root (`h-1` there is `trackClass="h-1"` here, `w-[60%]` is `class`).

### Parts

| Part | What | Customize with |
|---|---|---|
| root | A column: the track, then the children | `class` / `style` |
| track | The bar's background, clipping the indicator | `trackClass` / `trackStyle` |
| indicator | The filled share, or the moving segment | `indicatorClass` / `indicatorStyle` |

## Variants and sizes

### Look

| Part | shadcn classes | This library |
|---|---|---|
| track | `relative h-2 w-full overflow-hidden rounded-full bg-primary/20` | Height `progress-height` (8), `w-full`, corners `progress-radius` (half the height), fill `progress` (`primary` at 20%), clipped |
| indicator | `h-full w-full flex-1 bg-primary transition-all`, `translateX(-(100 − value)%)` | Full size, fill `progress-indicator` (`primary`), scaled along x to `value` % from the start, transitioning over `duration` |

The same in dark (shadcn has no dark treatment): `primary` is light there, its 20% a dark track.

### Indeterminate

shadcn draws nothing while indeterminate (its indicator sits at `translateX(-100%)`: an empty track). Radix and
`nsh/base` have the state, and `nsh/base`'s terminal form moves a segment along the bar; this library does the same:

| | Look |
|---|---|
| Segment | `progress-indeterminate-width` (40%) of the track, fill `progress-indicator` |
| Motion | From fully before the track's start to fully past its end, `progress-indeterminate-duration` (1500 ms) a pass, ease in-out, repeating |
| Reduced motion | No motion: the whole track at `progress-indicator`, 50% opacity |

## States

| State | Look |
|---|---|
| 0 | The track only |
| 1–99 | The indicator's share of the track |
| 100 | The full track in `progress-indicator` |
| Value changing | The indicator's scale transitions over `duration` (none with reduced motion) |
| Indeterminate | The moving segment (above) |

No hover, focus, press or disabled: a progress bar is not interactive.

## Pointer and touch

The same on both: not a press target. Its height does not change on touch screens (shadcn's `h-2` everywhere).

## Accessibility

`nsh/base`'s: one `progressbar` element, the range 0 to 100 and the value (screen readers say "33%"), none while
indeterminate (they say it is busy); named by `accessibilityLabel`, else the children's text. The library adds
nothing; the moving segment is drawn only.

## Tokens

Generated from the code's declarations (`NSH_LONGBRIDGE_WRITE=1 cargo test --test tokens`); a test keeps it current.
Values are those of `nsh/base`'s default theme.

<!-- tokens:progress -->
| Token | Type | Derived default | Value |
|---|---|---|---|
| `progress` | color | `primary × 20%` |  |
| `progress-indicator` | color | `primary` |  |
| `progress-height` | length | `spacing × 2` | 8 |
| `progress-radius` | length | `progress-height × 0.5` | 4 |
| `progress-gap` | length | `spacing × 2` | 8 |
| `progress-indeterminate-width` | number | `0.4` | 0.4 |
| `progress-indeterminate-duration` | number | `duration × 10` | 1500 |
<!-- /tokens -->

- `progress` is the track (the control itself has the bare name, tokens.md); it is `primary` at 20%, not
  `progress-indicator` at 20%, as shadcn's.
- `progress-radius` is drawn at most half the track's height, so a `trackClass="h-1"` keeps round ends.
- `progress-indeterminate-duration` is a pass in milliseconds; `progress-indeterminate-width` a share of the track.
- Other tokens it reads: `duration`, `spacing`.

## Examples

TSX:

```tsx
import { signal, Text } from "nsh";
import { Progress } from "nsh-longbridge";

const uploaded = signal(13);

<Progress value={uploaded()} class="w-[60%]" accessibilityLabel="Uploading report.pdf" />

// With its value under it: read as the name.
<Progress value={66}>
  <Text class="text-sm text-muted-foreground">66% · 2 of 3 files</Text>
</Progress>

// Unknown extent.
<Progress indeterminate accessibilityLabel="Connecting" />

// Thinner, another color.
<Progress value={90} trackClass="h-1" indicatorClass="bg-destructive" accessibilityLabel="Storage used" />
```

Rust:

```rust
use nsh_core::prelude::*;
use nsh_longbridge::progress;

let uploaded = signal(13.0_f32);

progress()
    .value(move || uploaded.get())
    .accessibility_label("Uploading report.pdf")

progress().indeterminate(true).accessibility_label("Connecting")
```

## NSH gaps

1. ★ **A repeating motion.** NSH runs Tailwind's `spin`, `ping`, `pulse` and `bounce` keyframes; whether a library
   can name keyframes of its own (a slide across) is the first headless test of `Progress`. If it can, the segment
   moves with them (`animation-[nlb-progress-slide_1500ms_ease-in-out_infinite]`, reduced motion skipping it).
   Fallback: the library drives it with a timer (`set_interval`, as `nsh/base`'s terminal bar) every pass sets the segment back
   to the start without a transition and then to the end with one. Either way the motion runs only while
   indeterminate and shown.
2. ★ **A width does not transition** (NSH animates transforms, not layout): the indicator scales instead (see
   Implementation). Headless test: `value` 30 → 60 transitions the indicator's scale from 0.3 to 0.6 from its start,
   and the track's clip rounds its start. Fallback: the indicator's width by value (`nsh/base`'s own), changing at
   once. If a later NSH animates widths, nothing changes in the API.
3. **The track and the indicator are `nsh/base`'s parts** (conventions, "Drawing"): generated classes, and the
   indicator's scale as a bound `indicatorStyle` with the app's merged after.

## Decisions

Settled in review.

1. **The indeterminate form moves** (`nsh/base` has the state, and its terminal bar moves a segment; shadcn draws an
   empty track, which reads as "not started").
2. **No `size`**: shadcn has one height; `progress-height` and `trackClass` cover the rest.
3. **`children` under the track**: `nsh/base` puts them after it; a label beside the bar is the app's row around a
   `Progress` without children.
