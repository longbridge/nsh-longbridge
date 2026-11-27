# Spinner

Status: implemented (P1). A spinning loading indicator: something is in progress and how long it takes is unknown.

## Usage

- A small area, a row or a panel is loading or submitting.
- Search terms: spinner, loading indicator, activity indicator, loader.

## When not to use

- A button is submitting: use `Button`'s `loading` (it uses this spinner and also blocks presses).
- First load of a block whose shape is known: use `Skeleton`.
- The progress is known (a percentage): use a progress bar (`nsh/base`'s `Progress`; this library's `Progress` comes
  later), not a spinner.

## Built on

`nsh/base` has no counterpart; this library draws a foundation icon, lucide's `loader-circle`,
turned by `foundation`'s `motion::spin`: one turn a second, linear, endless, as Tailwind's `animate-spin`, but played as
an NSH motion that ignores reduced motion (see "States" and "NSH gaps"). It has no behavior.

## API

| TSX | Rust | Type | Default | Description |
|---|---|---|---|---|
| `size` | `size` | `Size`: `"xs"` \| `"sm"` \| `"md"` \| `"lg"` | From context, else `"md"` | The icon's edge; see below |
| `accessibilityLabel` | `accessibility_label` | `string` | `"Loading"` (translated with `setTranslator` in TSX, `labels::set_translator` in Rust) | Screen reader label |
| `class` / `style` | `class` / `style` | | | The root, which is the icon; `class="size-8"` goes beyond the four sizes |

No events, no children, no parts.

## Variants and sizes

- No `variant`. The color follows the text (see "Look").
- `size`:

| `size` | Edge | Class |
|---|---|---|
| `xs` | 12 | `size-3`, the icon in an xs button |
| `sm` | 14 | `size-3.5` |
| `md` | 16 | `size-4`, the default |
| `lg` | 16 | `size-4`, same as `md`; for larger use `class="size-5"` or change the `icon-size-lg` token |

The edge is the foundation's icon edge for that `Size`; the spinner declares no token of its own. Without `size` it takes
the default from context as icons do, so in a `size="sm"` button it is 14.

The spinner has the four `size`s of the conventions, so a spinner in a control of the same `size` matches its icons.
Larger ones (32 in the middle of a page) use `class="size-8"`. Not enlarged on touch screens: it is not a control.

### Look

In classes: `size-4 animate-spin`, color `currentColor` (the surrounding text).

| | Value |
|---|---|
| Shape | The `loader-circle` icon, with the icon set's stroke width (lucide's 2) |
| Color | Inside a library part that colors its text (a `Button`, a `Card`, a menu row, a `Badge`…), that text color (`currentColor`): the button's text color in a button. Elsewhere `foreground`, so it shows in both appearances. Another color: `class="text-…"`, which goes over both |
| Animation | One turn per second, linear, endless (`animate-spin`'s timing; `foundation`'s `motion::spin`) |

No color token: following the text keeps it right in every button variant; to change it everywhere, change the text
color where it sits.

Outside the library's parts NSH hands an svg black as its text color whatever the appearance, and a component cannot
tell that default from a color an app set (see "NSH gaps"). So the parts that color their text say so through the
foundation's text color context, and a spinner outside them takes `foreground`, as the library's `Icon` does. A spinner inside an app's own colored
`View` (`class="text-muted-foreground"`) therefore also takes `foreground`: give the spinner the class itself.

## States

- **Reduced motion**: keeps spinning (decided). The spin is the "still in progress" information; a still icon looks
  frozen. macOS and iOS system spinners keep spinning under reduced motion, and WCAG 2.3.3 exempts animation that is
  essential to the information. This is an exception to conventions.md's "transitions go to zero under reduced motion":
  it is not a transition. NSH's core starts no keyframe animation (`animate-spin` included) under reduced motion, so the
  spinner turns through an NSH motion played with `play_ignoring_reduce_motion` instead.
- No hover / active / focus / disabled.

## Pointer and touch

No difference.

## Accessibility

- No Tab stop, no pointer events.
- Role `progressbar` (indeterminate), label `accessibilityLabel` (default "Loading"), state `busy`. The web would use
  `role="status"`; NSH has no `status` role, and an indeterminate `progressbar` is the role macOS and iOS give their own
  spinners (`NSProgressIndicator`, `UIActivityIndicatorView`).
- Inside `Button`'s `loading`, the button removes the spinner's own accessibility node and reports busy on itself, so it
  is not read twice (P0 Button's job; noted here).

## Tokens

No component tokens: the edge is the foundation's icon edge (`icon-size-xs/sm/md/lg`, 12 / 14 / 16 / 16), the color
follows the text (`foreground` outside the library's parts), and the animation is `animate-spin`'s fixed timing (NSH tokens cannot hold a curve). To change size or color
everywhere, change the `size` or the text color where it sits.

## Examples

```tsx
import { Spinner, Button } from "nsh-longbridge";

<View class="flex-row items-center gap-2">
  <Spinner size="sm" />
  <Text class="text-sm text-muted-foreground">Syncing…</Text>
</View>

<Spinner class="size-8 text-primary" accessibilityLabel="Loading orders" />
```

```rust
use nsh_longbridge::{spinner, Size};

view().flex_row().items_center().gap_2()
    .child(spinner().size(Size::Sm))
    .child(text("Syncing…").app_class(Some("text-sm text-muted-foreground".into())))

spinner().app_class(Some("size-8 text-primary".into())).accessibility_label("Loading orders")
```

## NSH gaps

- **No `status` accessibility role.** Workaround: an indeterminate `progressbar` with `busy` (see "Accessibility").
- **`animate-spin` stops under reduced motion.** NSH's core drops every keyframe animation then, with no way for a view
  to opt out. Workaround: `foundation`'s `motion::spin` (a `motion()` bound to the icon, played ignoring reduced motion).
  A per-animation opt-out (or `animate-spin` exempted, as the platforms' spinners are) would let this be a class again.
- **The default text color is black, and an unset color cannot be told from a set one.** Outside any `text-…` class an
  svg's `currentColor` is black, also in the dark appearance, so a bare spinner (or `Icon`) was black on black.
  Workaround: the library's parts that color their text provide the foundation's text color context, and the spinner
  and `Icon` take `foreground` where none does. A theme-aware default text color (or a way to ask whether one is inherited) would let it follow the
  text everywhere.

## Decisions

- Keeps spinning under reduced motion (review outcome).
- Outside the library's parts it is `foreground`, not the inherited (black) text color, so it shows in the dark
  appearance; a class color goes over it.
