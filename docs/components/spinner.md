# Spinner

A spinning loading indicator: something is in progress and how long it takes is unknown. Reference: shadcn/ui
new-york v4 `spinner.tsx`.

## Usage

- A small area, a row or a panel is loading or submitting.
- Search terms: spinner, loading indicator, activity indicator, loader.

## When not to use

- A button is submitting: use `Button`'s `loading` (it uses this spinner and also blocks presses).
- First load of a block whose shape is known: use `Skeleton`.
- The progress is known (a percentage): use a progress bar (`nsh/base`'s `Progress`; this library's `Progress` comes
  later), not a spinner.

## Built on

`nsh/base` has no counterpart; this library draws a foundation icon, lucide's `loader-circle` (shadcn's `Loader2Icon`),
turned by Tailwind's `animate-spin` (supported by NSH). It has no behavior.

## API

| TSX | Rust | Type | Default | Description |
|---|---|---|---|---|
| `size` | `size` | `Size`: `"xs"` \| `"sm"` \| `"md"` \| `"lg"` | From context, else `"md"` | The icon's edge; see below |
| `accessibilityLabel` | `accessibility_label` | `string` | `"Loading"` (translated through `labels::set_translator`) | Screen reader label |
| `class` / `style` | `class` / `style` | | | The root, which is the icon; `class="size-8"` goes beyond the four sizes |

No events, no children, no parts.

## Variants and sizes

- No `variant`. The color follows the text (see "Look").
- `size`:

| `size` | Edge | shadcn |
|---|---|---|
| `xs` | 12 | `size-3`, the icon in an xs button |
| `sm` | 14 | `size-3.5` |
| `md` | 16 | `size-4`, shadcn's default |
| `lg` | 16 | `size-4`, same as `md`; for larger use `class="size-5"` or change the `icon-size-lg` token |

The edge is the foundation's icon edge for that `Size`; the spinner declares no token of its own. Without `size` it takes
the default from context as icons do, so in a `size="sm"` button it is 14.

shadcn has only `size-4` and leaves the rest to `className`. This library gives the four `size`s of the conventions, so a
spinner in a control of the same `size` matches its icons. Larger ones (32 in the middle of a page) use `class="size-8"`
as in shadcn. Not enlarged on touch screens: it is not a control.

### Look

shadcn: `size-4 animate-spin`, color `currentColor` (the surrounding text).

| | Value |
|---|---|
| Shape | The `loader-circle` icon, with the icon set's stroke width (lucide's 2) |
| Color | Follows the surrounding text (the foundation's icon rule, shadcn's `currentColor`): the button's text color in a button, muted in a `text-muted-foreground` row. Another color: `class="text-…"` |
| Animation | `animate-spin`: one turn per second, linear, endless |

No color token: following the text keeps it right in every button variant; to change it everywhere, change the text
color where it sits.

## States

- **Reduced motion**: keeps spinning (decided). The spin is the "still in progress" information; a still icon looks
  frozen. macOS and iOS system spinners keep spinning under reduced motion, and WCAG 2.3.3 exempts animation that is
  essential to the information. This is an exception to conventions.md's "transitions go to zero under reduced motion":
  it is not a transition.
- No hover / active / focus / disabled.

## Pointer and touch

No difference.

## Accessibility

- No Tab stop, no pointer events.
- Role `progressbar` (indeterminate), label `accessibilityLabel` (default "Loading"), state `busy`. shadcn uses
  `role="status"`; NSH has no `status` role, and an indeterminate `progressbar` is the role macOS and iOS give their own
  spinners (`NSProgressIndicator`, `UIActivityIndicatorView`).
- Inside `Button`'s `loading`, the button removes the spinner's own accessibility node and reports busy on itself, so it
  is not read twice (P0 Button's job; noted here).

## Tokens

No component tokens: the edge is the foundation's icon edge (`icon-size-xs/sm/md/lg`, 12 / 14 / 16 / 16), the color
follows the text, and the animation is Tailwind's fixed values (NSH tokens cannot hold a curve). To change size or color
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
    .child(text("Syncing…").class("text-sm text-muted-foreground"))

spinner().class("size-8 text-primary").accessibility_label("Loading orders")
```

## NSH gaps

- **No `status` accessibility role.** Workaround: an indeterminate `progressbar` with `busy` (see "Accessibility").

## Decisions

- Keeps spinning under reduced motion (review outcome).
