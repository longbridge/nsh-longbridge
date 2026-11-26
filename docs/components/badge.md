# Badge

A small mark: a status, a category, a count. Reference: shadcn/ui new-york v4 `badge.tsx`.

## Usage

- A status next to a list row or a card title ("Published", "Draft"), a category, a version, a count.
- Search terms: badge, status badge, pill, label, count.

## When not to use

- Tags that can be pressed, removed or selected (filters, entered keywords): a badge is not interactive. Those are a
  later Tag / Toggle; for now use a `Button` with `size="xs"`.
- Shortcut keys: use `Kbd`.
- A lone dot (online, unread): a round `view`, not an empty badge.
- Market colors such as gains and losses: business components, not in this library. Do not add badge variants for them
  (use scoped tokens or `class`).

## Built on

`nsh/base` has no counterpart; this library draws a `view` and a `text`. It has no behavior: not focusable, not
pressable.

shadcn's `asChild` (rendering the badge as an `<a>`, which brings the `[a&]:hover` colors and the focus ring) is not
done: NSH has no `asChild`-style mechanism that puts one element's look on another, and something pressable should not be
a badge. So the hover and focus looks shadcn gives link badges are absent, and so are the `ghost` and `link` variants,
which exist only for links (see "Variants and sizes").

### Implementation

- Icon edge: the badge sets the foundation's icon size context to `xs`, so the library's `Icon` without a size is 12
  (shadcn's `[&>svg]:size-3`). No `badge-icon-size` token.
- String children: a TSX string child reaches Rust as a text node, Rust's `.child("…")` as a `&str`. Both become the
  `label` part.

## API

| TSX | Rust | Type | Default | Description |
|---|---|---|---|---|
| `variant` | `variant` | `"default"` \| `"secondary"` \| `"destructive"` \| `"outline"` (`BadgeVariant`) | `"default"` | Colors |
| `children` | `.child(..)` | Text, icons | | Content: strings, and the library's `Icon` before or after the text |
| `class` / `style` | `class` / `style` | | | The root |
| `labelClass` / `labelStyle` | `label_class` / `label_style` | | | The text made from string children |

No events.

String children are wrapped in a `text` (one line, no wrapping, truncated when too long): the `label` part. Element
children are placed as they are, and `labelClass` does not apply to them.

## Variants and sizes

| `variant` | Fill | Text | Border | shadcn |
|---|---|---|---|---|
| `default` | `badge-default` | `badge-default-foreground` | Transparent | `bg-primary text-primary-foreground` |
| `secondary` | `badge-secondary` | `badge-secondary-foreground` | Transparent | `bg-secondary text-secondary-foreground` |
| `destructive` | `badge-destructive` | `badge-destructive-foreground` | Transparent | `bg-destructive text-white dark:bg-destructive/60` |
| `outline` | Transparent | `badge-outline-foreground` | `badge-outline-border` | `border-border text-foreground` |

- No `ghost` or `link`: they differ from the others only in `[a&]:hover` (hovered as a link). Without interaction what is
  left is "transparent fill, foreground text" and "transparent fill, `primary` text", which `outline` with
  `class="border-transparent"` or a text color `class` gives. Not worth a variant each.
- No `success` / `warning` / `info`: shadcn has none. The semantic `success` tokens and their kin are there for scoped
  tokens or `class="bg-success text-success-foreground"`. Decided in review: not added; status colors are revisited with
  Alert in P2.
- **No `size`** (none of `xs` / `sm` / `md` / `lg`): shadcn has one. A badge marks content and does not line up with
  control heights.

### Look

shadcn: `inline-flex w-fit shrink-0 items-center justify-center gap-1 overflow-hidden rounded-full border
border-transparent px-2 py-0.5 text-xs font-medium whitespace-nowrap`, icons `size-3`.

| | Value |
|---|---|
| Layout | A row, centered both ways, as wide as its content (`self-start`, so a stretching parent does not widen it), `shrink-0` |
| Border | `border-width` (1), colored per variant (transparent except `outline`: it keeps its room, so all variants are the same height) |
| Padding | `badge-padding-x` (8) horizontally, `badge-padding-y` (2) vertically |
| Corners | `badge-radius` |
| Text | `badge-font-size` (12), `font-weight-medium`, line height from the foundation's font size → line height (12 → 16, Tailwind's `text-xs`), one line |
| Icons | 12 (the `xs` icon size the badge gives as context), colored as the text; `badge-gap` (4) between icon and text |
| Height | 16 line height + 2 × 2 padding + 2 × 1 border = 22, as shadcn |

Long text is truncated with an ellipsis (`numberOfLines={1}`); shadcn clips it (`overflow-hidden whitespace-nowrap`).
An ellipsis is the desktop norm, and screen readers still read the full text.

## States

No hover / active / focus-visible / disabled: not interactive.

shadcn's `aria-invalid:border-destructive aria-invalid:ring-destructive/20` is for link badges in forms; not done here.

## Pointer and touch

Same look; no hit slop (nothing to press).

## Accessibility

- No Tab stop.
- Read as its text (`text`). A badge with an icon and no text needs the app's `accessibilityLabel`.

## Tokens

| Token | Type | Derived default | Light / dark |
|---|---|---|---|
| `badge-default` | color | `primary` | Follows `primary` |
| `badge-default-foreground` | color | `primary-foreground` | Same |
| `badge-secondary` | color | `secondary` | Same |
| `badge-secondary-foreground` | color | `secondary-foreground` | Same |
| `badge-destructive` | color | `light_dark(destructive, destructive at 60%)` | shadcn's `dark:bg-destructive/60` |
| `badge-destructive-foreground` | color | `destructive-foreground` | shadcn hard-codes `text-white`; this library uses its added semantic token, white by default |
| `badge-outline-border` | color | `border` | Same |
| `badge-outline-foreground` | color | `foreground` | Same |
| `badge-radius` | length | 9999 (a pill, shadcn's `rounded-full`) | Same |
| `badge-padding-x` | length | `spacing` × 2 (8) | Same |
| `badge-padding-y` | length | `spacing` × 0.5 (2) | Same |
| `badge-gap` | length | `spacing` (4) | Same |
| `badge-font-size` | length | `font-size-xs` (12) | Same |

`badge-radius` does not refer to `radius-*`: shadcn v4's badge is a pill (`rounded-full`) and ignores the `radius` seed.
An app that wants squarer badges following the seed sets `"badge-radius": "var(--radius-sm)"` in its theme. With
`radius: 0` everything else turns square while badges stay pills, as in shadcn.

## Examples

```tsx
import { Badge, Icon } from "nsh-longbridge";

<View class="flex-row gap-2">
  <Badge>Badge</Badge>
  <Badge variant="secondary">Secondary</Badge>
  <Badge variant="destructive">Destructive</Badge>
  <Badge variant="outline">Outline</Badge>
  <Badge variant="secondary">
    <Icon name="badge-check" />
    Verified
  </Badge>
  <Badge class="min-w-5 px-1 tabular-nums">8</Badge>
</View>
```

```rust
use nsh_longbridge::{badge, icon, BadgeVariant};

view().flex_row().gap_2()
    .child(badge().child("Badge"))
    .child(badge().variant(BadgeVariant::Secondary).child("Secondary"))
    .child(badge().variant(BadgeVariant::Destructive).child("Destructive"))
    .child(badge().variant(BadgeVariant::Outline).child("Outline"))
    .child(
        badge().variant(BadgeVariant::Secondary)
            .child(icon("badge-check"))
            .child("Verified"),
    )
```

The count badge (`min-w-5 px-1 tabular-nums`) is how shadcn's docs write it: a `class`, not a variant.

## NSH gaps

- **No `asChild` and no selector by element type** (shadcn's `[a&]:hover`). Workaround: the badge is not interactive;
  there are no link badges.

## Decisions

1. No `href` (link badges).
2. No `success` / `warning` / `info` variants; revisited with Alert in P2.
