# Card

Status: implemented (P1). A container with a border, a fill and a light shadow, laid out as header, content and footer. Reference: shadcn/ui
new-york v4 `card.tsx`.

## Usage

- A group of settings, a block of a form, the summary of one object, a cell of a dashboard.
- Search terms: card, panel, surface, tile, group box.

## When not to use

- A bordered area without the title structure: give a `view` `class="rounded-xl border bg-card"`; no `Card` needed.
- Notices, warnings and errors: the later `Alert` (after P2), not a recolored card.
- A whole card that is pressed or selected: shadcn's card is not interactive and neither is this one (decided in
  review). Put pressable things in a `Button` inside `CardAction` or `CardFooter`.
- A panel that pops up: `Popover` / `Dialog` (P2).

## Built on

`nsh/base` has no counterpart; this library composes `view`s and `text`s. No behavior: not focusable, no presses; the
controls inside work as usual.

Following "composition over switches" in the conventions, the structure is made of child components, one for each of
shadcn's:

```
Card
├─ CardHeader
│  ├─ CardTitle
│  ├─ CardDescription
│  └─ CardAction        (optional: top right, spanning the title and description rows)
├─ CardContent
└─ CardFooter
```

Each part is a component with its own `class` / `style`, so `Card` has no part props such as `headerClass` (the
conventions' `<part>Class` is for parts that are not components of their own).

### Implementation

- **CardHeader's two columns**: shadcn uses `has-data-[slot=card-action]:grid-cols-[1fr_auto]`. NSH has no `:has()`,
  so this library uses context: `CardHeader` provides a "has an action" signal that `CardAction` sets when it mounts
  and clears in its `cleanup`, and the header's column template is bound to it. `1fr auto` is not used unconditionally:
  without an action the second column is 0 wide, but the column gap `gap-2` remains and narrows the title by 8 for
  nothing.
- A `CardAction` outside a `CardHeader` (no context to read) renders as usual and logs a warning in development builds.
- `CardAction`'s grid placement is set span first (`row_span(2)`, then `row_start(1)`, then `col_start(2)`): in NSH a
  span sets the start line too, so a span set after the start would undo it.
- `justify-self-end` is not set: the action's column is `auto`, as wide as the action, so it already ends at the
  header's right padding.
- `CardTitle` and `CardDescription` are a `text` each; their children are its content (a string child's text takes
  their styles).

## API

| Component | TSX | Rust | Takes | Description |
|---|---|---|---|---|
| `Card` | `<Card>` | `card()` | `children`, `class`, `style` | The root |
| `CardHeader` | `<CardHeader>` | `card_header()` | Same | The title area |
| `CardTitle` | `<CardTitle>` | `card_title()` | Same | The title; string children are wrapped in a text |
| `CardDescription` | `<CardDescription>` | `card_description()` | Same | The description; string children are wrapped in a text |
| `CardAction` | `<CardAction>` | `card_action()` | Same | The action at the top right (a button, a menu trigger); only inside `CardHeader` |
| `CardContent` | `<CardContent>` | `card_content()` | Same | The body |
| `CardFooter` | `<CardFooter>` | `card_footer()` | Same | The bottom row (buttons, notes) |

No events, no controlled values. Every part is optional; the order is the children's order.

## Variants and sizes

- No `variant`: shadcn has one. A muted card or one without a shadow uses `class` or a scoped token (`card-shadow: 0`).
- No `size` (none of `xs` / `sm` / `md` / `lg`): the padding is a token; for a denser layout change `card-padding` /
  `card-gap` or the `spacing` seed.

### Look

| Part | shadcn classes | This library |
|---|---|---|
| `Card` | `flex flex-col gap-6 rounded-xl border bg-card py-6 text-card-foreground shadow-sm` | A column; `card-gap` between parts; `card-padding` top and bottom; corners `card-radius`; border `border-width` × `card-border`; fill `card`; text color `card-foreground` (inherited by the text inside); shadow tier `card-shadow` |
| `CardHeader` | `grid auto-rows-min grid-rows-[auto_auto] items-start gap-2 px-6`, `grid-cols-[1fr_auto]` with an action; `[.border-b]:pb-6` | A grid: rows `auto auto`, `items-start`, gap `card-header-gap`; `card-padding` left and right. Columns `1fr auto` when it has a `CardAction` (see "Implementation") |
| `CardTitle` | `leading-none font-semibold` | Size `card-title-font-size`, line height 1 (shadcn's `leading-none`, deliberately not the foundation's font size → line height), `font-weight-semibold` |
| `CardDescription` | `text-sm text-muted-foreground` | Size `card-description-font-size`, line height from the foundation's font size → line height (14 → 20, as `text-sm`), color `card-description-foreground` |
| `CardAction` | `col-start-2 row-span-2 row-start-1 self-start justify-self-end` | Same |
| `CardContent` | `px-6` | `card-padding` left and right |
| `CardFooter` | `flex items-center px-6 [.border-t]:pt-6` | A row, centered vertically; `card-padding` left and right |

**Header and footer with a divider**: shadcn's `[.border-b]:pb-6` adds bottom padding when the app gives the header a
`border-b`. NSH cannot match on whether an element has a class, so the app writes both:
`<CardHeader class="border-b pb-6">`, `<CardFooter class="border-t pt-6">`. The docs and the gallery write it that way;
see "Examples".

No other differences from shadcn. The 24 padding is roomy for desktop density, but the conventions take the look from
shadcn; the library's compact preset theme sets `card-padding` and `card-gap` to 16 (P0's `themes/` presets, not decided
here).

## States

No hover / active / focus / disabled: the card is not interactive. The controls inside have their own states.

## Pointer and touch

Same look. On touch screens the buttons in a card grow to `control-height-touch-*` by themselves; the card does nothing
extra.

## Accessibility

- Neither the card nor its parts are Tab stops; Tab moves between the controls inside as usual.
- `CardTitle` has the `header` role, so screen readers can jump from card to card by heading. shadcn's `CardTitle` is a
  `div` without heading semantics; this library adds it because NSH's screen reader navigation relies on the `header`
  role, and without it a card reads as loose text.
- The other parts have no role (`none`); their text is read as usual.

## Tokens

Generated from the code's declarations (`NSH_LONGBRIDGE_WRITE=1 cargo test --test tokens`); a test keeps it current.
Values are those of `nsh/base`'s default theme.

<!-- tokens:card -->
| Token | Type | Derived default | Value |
|---|---|---|---|
| `card-border` | color | `border` |  |
| `card-description-foreground` | color | `muted-foreground` |  |
| `card-radius` | length | `radius-xl` | 14 |
| `card-shadow` | number | `2` | 2 |
| `card-padding` | length | `spacing × 6` | 24 |
| `card-gap` | length | `spacing × 6` | 24 |
| `card-header-gap` | length | `spacing × 2` | 8 |
| `card-title-font-size` | length | `16` | 16 |
| `card-description-font-size` | length | `14` | 14 |
<!-- /tokens -->

- `card` and `card-foreground` are shadcn's semantic tokens, so the component uses them directly and declares no
  component tokens of the same name (they are absent from `tokens!` and so from the table above). To change only the cards' fill, change the semantic `card`, which only card-like surfaces use anyway.
- `card-title-font-size` is Tailwind's fixed size, not a `font-size-*`: tokens.md gives body text and headings
  Tailwind's values, and `font-size-*` is for controls.

## Examples

```tsx
import { Button, Card, CardAction, CardContent, CardDescription, CardFooter, CardHeader, CardTitle } from "nsh-longbridge";

<Card class="w-[360px]">
  <CardHeader>
    <CardTitle>Login to your account</CardTitle>
    <CardDescription>Enter your email below to login to your account</CardDescription>
    <CardAction>
      <Button variant="link">Sign Up</Button>
    </CardAction>
  </CardHeader>
  <CardContent>
    {/* fields */}
  </CardContent>
  <CardFooter class="border-t pt-6 gap-2">
    <Button class="flex-1">Login</Button>
  </CardFooter>
</Card>
```

```rust
use nsh_longbridge::{button, card, card_action, card_content, card_description, card_footer, card_header, card_title, ButtonVariant};

card().w(px(360.0))
    .child(
        card_header()
            .child(card_title().child("Login to your account"))
            .child(card_description().child("Enter your email below to login to your account"))
            .child(card_action().child(button().variant(ButtonVariant::Link).child("Sign Up"))),
    )
    .child(card_content().child(/* fields */))
    .child(card_footer().app_class(Some("border-t pt-6 gap-2".into())).child(button().flex_1().child("Login")))
```

## NSH gaps

- **No `:has()` and no matching on an element's own classes** (shadcn's `has-data-[slot=card-action]:` and
  `[.border-b]:`). Workaround: the action registers through context; the app writes the divider's padding itself (see
  "Look").
- **Text color inheritance**: the root's `card-foreground` reaches the app's own `text`s inside through NSH's text
  style inheritance. Verified by a headless test; not a gap.

## Decisions

1. No pressable or selectable whole card.
2. `CardTitle` has the `header` role.
3. `card-shadow` = 2 (`sm`).
