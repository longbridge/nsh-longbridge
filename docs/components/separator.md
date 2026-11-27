# Separator

Status: implemented (P1). A thin line that divides content, horizontal or vertical.

## Usage

- Between sections of a card, panel or form (horizontal); between the items of a toolbar or a row of text links
  (vertical).
- Search terms: separator, divider, hr, rule.

## When not to use

- In a menu, use the separator item of P2's `DropdownMenu` (`nsh/base`'s `MenuSeparator`); in a toolbar, `nsh/base`'s
  `ToolbarSeparator`. Those mean something to screen readers and keyboard movement.
- To give a block an edge, use that block's `class` (`border-b`) rather than a `Separator` under it.
- A labelled divider ("line · or · line"): this library has none; put a `text` between two
  `Separator`s.

## Built on

`nsh/base` has no separator component (only `MenuSeparator` and `ToolbarSeparator`); this library draws a `view`. It has
no behavior: not focusable, no pointer events (`pointer-events-none`).

## API

| TSX | Rust | Type | Default | Description |
|---|---|---|---|---|
| `orientation` | `orientation` | `"horizontal"` \| `"vertical"` (`nsh_core::geometry::Axis`) | `"horizontal"` | Direction |
| `class` / `style` | `class` / `style` | | | The root, which is the line itself |

No events, no children, no parts (`*Class` / `*Style`).

## Variants and sizes

- No `variant`.
- No `size` (none of `xs` / `sm` / `md` / `lg`): the thickness is a token, the length comes from layout.

### Look

| | Horizontal | Vertical |
|---|---|---|
| Classes | `shrink-0 bg-border h-px w-full` | `shrink-0 bg-border h-full w-px` |
| Layout | `shrink-0`, height `separator-thickness`, `self-stretch` (spans the parent's width) | `shrink-0`, width `separator-thickness`, `self-stretch` (spans the parent's height) |
| Color | `separator` | `separator` |

Why `self-stretch` and not `w-full` / `h-full`: in a row, a vertical `h-full` needs a
parent with a definite height (a common pitfall: the line ends up 0 high and the app adds `h-4`); NSH lays out with flex,
so `self-stretch` fills the row's height. For a shorter vertical line, give `class="h-4"` (`class` is merged
last and wins over `self-stretch`). An explicit height stops the stretch (Taffy's `align-self: stretch` leaves a definite
size alone, as CSS does): verified by a headless test.

## States

No hover / active / focus / disabled: not interactive.

## Pointer and touch

No difference. Not enlarged on touch screens and no hit slop (nothing to press).

## Accessibility

- No Tab stop, no pointer events.
- Always decorative: out of the accessibility tree (`accessible(false)` / TSX `accessible={false}`). NSH has no
  `separator` role, so a non-decorative separator is not offered; see "NSH gaps".

## Tokens

Generated from the code's declarations (`NSH_LONGBRIDGE_WRITE=1 cargo test --test tokens`); a test keeps it current.
Values are those of `nsh/base`'s default theme.

<!-- tokens:separator -->
| Token | Type | Derived default | Value |
|---|---|---|---|
| `separator` | color | `border` |  |
| `separator-thickness` | length | `border-width` | 1 |
<!-- /tokens -->

`separator` follows `border` (the dark `border` is white at 10%).
## Examples

```tsx
import { Separator } from "nsh-longbridge";

<View class="gap-4">
  <Text>Primitives</Text>
  <Separator />
  <View class="flex-row h-5 items-center gap-4">
    <Text>Blog</Text>
    <Separator orientation="vertical" />
    <Text>Docs</Text>
  </View>
</View>
```

```rust
use nsh_core::geometry::Axis;
use nsh_longbridge::separator;

view().gap_4()
    .child(text("Primitives"))
    .child(separator())
    .child(
        view().flex_row().h_5().items_center().gap_4()
            .child(text("Blog"))
            .child(separator().orientation(Axis::Vertical))
            .child(text("Docs")),
    )
```

## NSH gaps

- **No `separator` accessibility role** (absent from `nsh_core::props`' roles), so a non-decorative separator (read as
  "separator") cannot be done.
 Workaround: the separator is always decorative and has no `decorative` prop. Where a
  screen reader must know that a new section starts, mark the section with a heading (`accessibilityRole="header"`),
  which is the better practice anyway.

## Decisions

None.
