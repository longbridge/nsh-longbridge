# Badge

Status: implemented (P1). A small mark: a status, a category, a count.

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

A link badge (the badge rendered as an `<a>` through an `asChild`, which brings `[a&]:hover` colors and a focus ring) is
not done: NSH has no `asChild`-style mechanism that puts one element's look on another, and something pressable should
not be a badge. So link badges' hover and focus looks are absent, and so are the `ghost` and `link` variants, which
exist only for links (see "Variants and sizes").

### Implementation

- Icon edge: the badge sets the foundation's icon size context to `xs`, so the library's `Icon` without a size is 12
  (`[&>svg]:size-3`). No `badge-icon-size` token.
- Children: as `Button` does (its NSH gap G1), every child is wrapped in a `label` text, since a component cannot tell a
  string child from an element. A string's text takes the label's styles; an element sits in its label unchanged.

## API

| TSX | Rust | Type | Default | Description |
|---|---|---|---|---|
| `variant` | `variant` | `"default"` \| `"secondary"` \| `"destructive"` \| `"outline"` (`BadgeVariant`) | `"default"` | Colors |
| `children` | `.child(..)` | Text, icons | | Content: strings, and the library's `Icon` before or after the text |
| `class` / `style` | `class` / `style` | | | The root |
| `labelClass` / `labelStyle` | `label_class` / `label_style` | | | The text made from string children |

No events.

Each child is wrapped in a `text` (one line, no wrapping, truncated when too long): the `label` part. A string child's
text takes `labelClass` / `labelStyle`; an element child sits in its label, its own classes winning.

With NSH 0.7.0, write a string child inside `<Text>` for now (`Button`'s G0).

## Variants and sizes

| `variant` | Fill | Text | Border | Classes |
|---|---|---|---|---|
| `default` | `badge-default` | `badge-default-foreground` | Transparent | `bg-primary text-primary-foreground` |
| `secondary` | `badge-secondary` | `badge-secondary-foreground` | Transparent | `bg-secondary text-secondary-foreground` |
| `destructive` | `badge-destructive` | `badge-destructive-foreground` | Transparent | `bg-destructive text-white dark:bg-destructive/60` |
| `outline` | Transparent | `badge-outline-foreground` | `badge-outline-border` | `border-border text-foreground` |

- No `ghost` or `link`: they differ from the others only in `[a&]:hover` (hovered as a link). Without interaction what is
  left is "transparent fill, foreground text" and "transparent fill, `primary` text", which `outline` with
  `class="border-transparent"` or a text color `class` gives. Not worth a variant each.
- No `success` / `warning` / `info`. The semantic `success` tokens and their kin are there for scoped
  tokens or `class="bg-success text-success-foreground"`. Decided in review: not added; status colors are revisited with
  Alert in P2.
- **No `size`** (none of `xs` / `sm` / `md` / `lg`): one size. A badge marks content and does not line up with
  control heights.

### Look

In classes: `inline-flex w-fit min-w-[22px] shrink-0 items-center justify-center gap-1 overflow-hidden rounded-full border
border-transparent px-2 py-0.5 text-xs font-medium whitespace-nowrap`, icons `size-3`.

| | Value |
|---|---|
| Layout | A row, centered both ways, as wide as its content (`w-fit`: a column that stretches its children does not widen it, and a row's `items-center` still centers it), `shrink-0`, at most as wide as its parent (`max-w-full`); a width `class` goes over `w-fit` |
| Border | `border-width` (1), colored per variant (transparent except `outline`: it keeps its room, so all variants are the same height) |
| Padding | `badge-padding-x` (8) horizontally, `badge-padding-y` (2) vertically |
| Corners | `badge-radius` |
| Text | `badge-font-size` (12), `font-weight-medium`, line height from the foundation's font size → line height (12 → 16, Tailwind's `text-xs`), one line |
| Icons | 12 (the `xs` icon size the badge gives as context), colored as the text; `badge-gap` (4) between icon and text |
| Height | 16 line height + 2 × 2 padding + 2 × 1 border = 22 |
| Least width | Its height (22), as a class (`min-w-[22px]`, from the tokens) the app's `class` goes over: one character (a count) makes a circle, not a pill narrower than it is tall |

Long text is truncated with an ellipsis (one line, as Rust's `number_of_lines(1)`) at the parent's width, rather than
clipped (`overflow-hidden whitespace-nowrap`) with the badge overflowing a narrower parent. An ellipsis is the desktop
norm, and screen readers still read the full text.

## States

No hover / active / focus-visible / disabled: not interactive.

An invalid look (`aria-invalid:border-destructive aria-invalid:ring-destructive/20`) is for link badges in forms; not
done here.

## Pointer and touch

Same look; no hit slop (nothing to press).

## Accessibility

- No Tab stop.
- Read as its text (`text`). A badge with an icon and no text needs the app's `accessibilityLabel`.

## Tokens

Generated from the code's declarations (`NSH_LONGBRIDGE_WRITE=1 cargo test --test tokens`); a test keeps it current.
Values are those of `nsh/base`'s default theme.

<!-- tokens:badge -->
| Token | Type | Derived default | Value |
|---|---|---|---|
| `badge-default` | color | `primary` |  |
| `badge-default-foreground` | color | `primary-foreground` |  |
| `badge-secondary` | color | `secondary` |  |
| `badge-secondary-foreground` | color | `secondary-foreground` |  |
| `badge-destructive` | color | `light_dark(destructive, destructive × 60%)` |  |
| `badge-destructive-foreground` | color | `destructive-foreground` |  |
| `badge-outline-border` | color | `border` |  |
| `badge-outline-foreground` | color | `foreground` |  |
| `badge-radius` | length | `9999` | 9999 |
| `badge-padding-x` | length | `spacing × 2` | 8 |
| `badge-padding-y` | length | `spacing × 0.5` | 2 |
| `badge-gap` | length | `spacing` | 4 |
| `badge-font-size` | length | `font-size-xs` | 12 |
<!-- /tokens -->

`badge-destructive` is `dark:bg-destructive/60`. `badge-destructive-foreground` follows the library's semantic
`destructive-foreground` (white by default) rather than a fixed `text-white`.

`badge-radius` does not refer to `radius-*`: a badge is a pill (`rounded-full`) and ignores the `radius` seed.
An app that wants squarer badges following the seed sets `"badge-radius": "var(--radius-sm)"` in its theme. With
`radius: 0` everything else turns square while badges stay pills.

## Examples

```tsx
import { Badge, Icon } from "nsh-longbridge";

<View class="flex-row gap-2">
  <Badge>Badge</Badge>
  <Badge variant="secondary">Secondary</Badge>
  <Badge variant="destructive">Destructive</Badge>
  <Badge variant="outline">Outline</Badge>
  <Badge variant="secondary">
    <Icon name="plus" />
    New
  </Badge>
  <Badge class="px-1 tabular-nums">8</Badge>
</View>
```

```rust
use nsh_longbridge::{badge, icon, BadgeVariant, IconName};

view().flex_row().gap_2()
    .child(badge().child("Badge"))
    .child(badge().variant(BadgeVariant::Secondary).child("Secondary"))
    .child(badge().variant(BadgeVariant::Destructive).child("Destructive"))
    .child(badge().variant(BadgeVariant::Outline).child("Outline"))
    .child(
        badge().variant(BadgeVariant::Secondary)
            .child(icon().name(IconName::Plus))
            .child("New"),
    )
```

The count badge (`px-1 tabular-nums`) is a `class`, not a variant; the least width already makes a single digit round.

## NSH gaps

- **No `asChild` and no selector by element type** (CSS's `[a&]:hover`). Workaround: the badge is not interactive;
  there are no link badges.

## Decisions

1. No `href` (link badges).
2. No `success` / `warning` / `info` variants; revisited with Alert in P2.
3. As wide as its content through `w-fit` (`width: fit-content`), not `align-self: flex-start`: an `align-self` of its
   own would override the cross-axis alignment of a row it sits in, so a badge beside a two-line name would sit at the
   top instead of the middle.
