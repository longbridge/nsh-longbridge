# Pagination

Status: spec (P3), not implemented.

The pages of a set to move among: Previous, the pages around the current one with ellipses for the runs left out,
Next. The look is shadcn/ui new-york v4 `pagination.tsx` (items drawn as `Button`'s `ghost` and, for the current page,
`outline`); the behavior and the pages shown are `nsh/base`'s `Pagination`.

## Usage

- Search results, a table's pages of rows (under a [Table](table.md) holding one page), an archive, a gallery of
  pages.
- Search terms: pagination, pager, page numbers, previous next, paging.

## When not to use

| Need | Use instead |
|---|---|
| Rows that scroll through all their data | [DataTable](data-table.md) virtualizes them: no pages needed |
| Steps of a flow (a wizard) | A stepper of the app's own, or `Tabs` |
| Slides of a carousel | `nsh/base`'s `CarouselPagination` |
| Loading more at the end of a list | The list's `onEndReached` / DataTable's `onLoadMore` |

## Built on

`nsh/base`'s `Pagination` (`crates/nsh-base/src/pagination.rs`). It owns the behavior and has no look:

| Provided by `nsh/base` (not rewritten here) | |
|---|---|
| Pages shown | `visiblePages` (at least 5, the default): every page while they fit, else the first and the last, the current one with its neighbours, an ellipsis for each run left out (gpui-base's rule); `compact` shows Previous, the children and Next only |
| Value | `page` (from 1, controlled) and `totalPages`; `onChange({ page })` for a press of a page, Previous or Next, never for the current page |
| Ends | Previous on the first page and Next on the last take no press (`disabled:` applies); `disabled` makes every item inert |
| Keyboard | Each page, Previous and Next is a Tab stop; Return and Space press it; the focus follows the page it brought (onto its new place, or to the first or last page when Previous or Next turns disabled) |
| Items | Each a view the app styles (`itemClass`: `selected:` on the current page, `disabled:`, `focus-visible:`; `itemStyle`) or draws (`renderItem` with `{ kind, page, start, end, current, disabled }`); made once, a page change changing what they show |
| Accessibility | A `navigation` landmark named by `accessibilityLabel`; items `button`s, the current page `selected` and `current="page"` (ARIA's `aria-current`); Previous and Next named by their labels |
| Terminal | A row of words; this library's look does not apply there |

This library adds the look of the items (Button's tokens), `size`, the content of each item (the number, the
chevrons and their words, the ellipsis icon), and its own words for Previous and Next.

### Differs from shadcn's structure

shadcn composes the row by hand: `Pagination > PaginationContent > PaginationItem > PaginationLink isActive` /
`PaginationPrevious` / `PaginationNext` / `PaginationEllipsis`, and the app decides which pages to show. `nsh/base`
computes the pages and ellipses from `page`, `totalPages` and `visiblePages`, and owns the items' keyboard and focus,
so here:

| shadcn | This library |
|---|---|
| `<Pagination><PaginationContent>…</PaginationContent></Pagination>` | `<Pagination page totalPages onChange />` |
| `<PaginationLink href isActive>2</PaginationLink>` | A page item; the current one is drawn as `isActive` |
| `<PaginationPrevious href>` / `<PaginationNext href>` | Drawn by `Pagination`; `onChange` with the page before / after |
| `<PaginationEllipsis />` | Drawn where `nsh/base` leaves pages out |
| Links (`<a href>`) | Buttons reporting `onChange`: an NSH app navigates by state (a router updates the page from `onChange`) |
| `size` on each link | `size` on `Pagination` |

### Implementation

- `#[component] fn Pagination` whose root is `nsh_base::pagination()`, every prop and `onChange` passed through.
- **Items' look**: an `itemClass` generated from the resolved Button tokens in a binding (as `Select`'s highlight,
  select.md NSH gaps 1): the ghost look at rest, `hover:`, `active:` (with the press's `scale-[0.97]`, none with reduced
  motion), `selected:` the outline look (fill, edge, `xs` shadow, its own `hover:`), `focus-visible:` the ring and
  edge, `disabled:` `opacity-disabled` without hover; each appearance's color under `dark:`. The app's `itemClass`
  follows it (its classes win). Geometry (height, min width, radius, centered content, the text's size and weight)
  is a bound `itemStyle`, the app's `itemStyle` merged after.
- **Items' content**: the library's `renderItem`: a page's number; Previous a row of `chevron-left` and "Previous",
  Next of "Next" and `chevron-right` (the words hidden in a window narrower than 640, `hidden sm:flex`, as shadcn's
  `hidden sm:block`); an ellipsis the `ellipsis` icon. With the app's `renderItem`, the library calls it for the
  content and still draws the item (as `Select`'s `renderItem`).
- `Pagination` provides its `size` in context: icons in the items take `icon-size-<size>`.

## API

### Props

| TSX | Rust | Type | Default | Description |
|---|---|---|---|---|
| `page` | `page` | `number` | — | The current page, from 1 (controlled: the app sets it in `onChange`) |
| `totalPages` | `total_pages` | `number` | — | How many pages there are, at least 1 |
| `visiblePages` | `visible_pages` | `number` | `5` | The most pages shown at once, the first and the last included (at least 5; odd numbers center the current page) |
| `compact` | `compact` | `boolean` | `false` | Previous, the children and Next only: a pager for a narrow place, the page in words as children |
| `size` | `size` | `Size` | `"md"` | The items' height, type and icons: Button's sizes |
| `disabled` | `disabled` | `boolean` | `false` | No item takes a press or the focus; dimmed |
| `accessibilityLabel` | `accessibility_label` | `string` | "Pagination" (`labels`) | What screen readers call the navigation (shadcn's `aria-label="pagination"`); name what it pages ("Search results pages") |
| `previousAccessibilityLabel` | `previous_accessibility_label` | `string` | "Go to previous page" (`labels`) | Previous's name for screen readers (shadcn's) |
| `nextAccessibilityLabel` | `next_accessibility_label` | `string` | "Go to next page" (`labels`) | Next's name |
| `renderItem` | `render_item` | `(item: PaginationItemState) => element` / `Renderer<PaginationItemState>` | — | An item's content in place of the library's (a page in the app's words, a menu of the pages an ellipsis leaves out); the library still draws the item |
| `itemClass` / `itemStyle` | `item_class` / `item_style` | | — | Each item, after the library's look |
| `class` / `style` | `class` / `style` | | — | The row: a centered row as wide as its parent by default (`class="justify-end"` puts it at the end) |
| `children` | `.child(..)` | string, elements | — | After the pages, before Next: "Page 3 of 12" while `compact`, a field to go to a page |

### Events

| TSX | Rust (`PaginationEvent`) | Payload | When |
|---|---|---|---|
| `onChange` | `Change(u32)` | `{ page }` | The user picked a page: a page, Previous or Next (`nsh/base`'s, passed on) |

### Parts

| Part | What | Customize with |
|---|---|---|
| root | The navigation row | `class` / `style` |
| item | Previous, each page, each ellipsis, Next | `itemClass` / `itemStyle`; `renderItem` for the content |

`PaginationItemState`, `PaginationItemKind` and `pagination_items` are `nsh/base`'s, re-exported (the TypeScript
declarations declare them too, as `Select`'s `SelectItem`).

## Variants and sizes

No variants: the items' looks are fixed by their kind, as shadcn's.

### Look

| Item | shadcn | This library |
|---|---|---|
| Page | `buttonVariants({ variant: "ghost", size: "icon" })`: `size-9 rounded-md`, `hover:bg-accent hover:text-accent-foreground dark:hover:bg-accent/50` | A square of `button-height-<size>`, corners `button-radius`, text `button-ghost-foreground` at `font-size-<size>`, `font-weight-medium`, centered; hover fill `button-ghost-hover`, text `button-ghost-hover-foreground` |
| Current page | `variant: "outline"`, `aria-current="page"`: `border bg-background shadow-xs hover:bg-accent … dark:border-input dark:bg-input/30 dark:hover:bg-input/50` | The same square: fill `button-outline`, edge `border-width` × `button-outline-border`, shadow `button-outline-shadow` (1, `xs`), text `button-outline-foreground`; hover fill `button-outline-hover` |
| Previous / Next | `variant: "ghost", size: "default"`, `gap-1 px-2.5`, `ChevronLeftIcon` + "Previous" (`hidden sm:block`), "Next" + `ChevronRightIcon` | Height `button-height-<size>`, padding `pagination-step-padding-x` (10) at the sides, `pagination-step-gap` (4) between the icon and the word; the ghost look; the word hidden in a window narrower than 640 (then a square) |
| Ellipsis | `flex size-9 items-center justify-center`, `MoreHorizontalIcon size-4`, `aria-hidden` | A square of `button-height-<size>`, the `ellipsis` icon at `icon-size-<size>`, no hover or press, hidden from screen readers |
| Row | `nav`: `mx-auto flex w-full justify-center`; `ul`: `flex flex-row items-center gap-1` | A row, `w-full`, centered, `pagination-gap` (4) between items |

### Sizes

Button's: items 24 / 28 / 32 / 36 tall with a pointer (32 / 36 / 40 / 48 on touch screens), type 12 / 13 / 14 / 14,
icons 12 / 14 / 16 / 16. shadcn's `size-9` (36) is `lg` here; `md` (32) is one density step down, as `Button`.

## States

| State | Look | Behavior |
|---|---|---|
| Page at rest | Ghost: no fill | Pressable |
| hover (pointer) | Ghost's hover fill; the current page the outline's | Cursor: the arrow (buttons, not links: the desktops' habit) |
| active (pressed) | The hover fill and a sink to 0.97 (none with reduced motion) | — |
| focus-visible | A `ring-width` ring of `button-ring` outside the item; the current page's edge `button-border-focus` | Keyboard focus only; on touch screens the edge only |
| current (`selected:`) | The outline look | Pressing it reports nothing (`nsh/base`) |
| disabled (Previous at the first page, Next at the last, everything with `disabled`) | `opacity-disabled`, no hover, no sink | No press, no focus |

Colors and the sink transition over `duration` (Button's), none with reduced motion.

## Pointer and touch

| | Desktops and desktop browsers | Phones and tablets |
|---|---|---|
| Item height | 24 / 28 / 32 / 36 | 32 / 36 / 40 / 48 |
| Hit area | The item | Below `touch-target` (44): a vertical hit slop of (44 − height) / 2 and a horizontal one of half `pagination-gap`, so neighbours keep their own presses (as Input's fields side by side) |
| Focus | Ring and edge | Edge only |
| Words of Previous / Next | Shown from a 640-wide window | Hidden on a phone's width; `compact` with "3 / 12" as children is the usual pager there |

## Accessibility

`nsh/base`'s: a `navigation` landmark named by `accessibilityLabel` ("Pagination" by default); items are `button`s;
the current page is `selected` and announced as the current page; Previous and Next are named "Go to previous page" /
"Go to next page" (shadcn's labels; their visible words are not their names), and dimmed at the ends. The ellipsis is
hidden (shadcn's `aria-hidden`; its "More pages" is inside the hidden span). Icons are hidden. The library's words
are translated with `setTranslator` / `labels::set_translator`.

## Tokens

The items read Button's tokens ([Button](button.md#tokens)): `button-height-<size>`, `button-radius`,
`button-ghost-*`, `button-outline-*`, `button-ring`, `button-border-focus`; a theme that changes `button-radius`
changes the pagination with every button. Its own are the row's spacing. Generated (`<!-- tokens:pagination -->`).

<!-- tokens:pagination -->
| Token | Type | Derived default | Value |
|---|---|---|---|
| `pagination-gap` | length | `spacing` | 4 |
| `pagination-step-padding-x` | length | `spacing × 2.5` | 10 |
| `pagination-step-gap` | length | `spacing` | 4 |
<!-- /tokens -->

- Other tokens it reads: `font-size-*`, `icon-size-*`, `font-weight-medium`, `border-width`, `ring-width`,
  `opacity-disabled`, `touch-target`, `duration`, `shadow`.

## Examples

TSX:

```tsx
import { signal, Text } from "nsh";
import { Pagination } from "nsh-longbridge";

const page = signal(2);

<Pagination page={page()} totalPages={10} onChange={({ page: next }) => page.set(next)} accessibilityLabel="Results pages" />

// Under a table showing one page of rows, at its end, small.
<Pagination class="justify-end" size="sm" page={page()} totalPages={pages()} onChange={({ page: next }) => page.set(next)} />

// A phone's pager.
<Pagination compact page={page()} totalPages={12} onChange={({ page: next }) => page.set(next)}>
  <Text class="text-sm text-muted-foreground">{`${page()} / 12`}</Text>
</Pagination>
```

With NSH 0.7.0, write a string child inside `<Text>` for now (see [button.md, NSH gaps](button.md#nsh-gaps), G0).

Rust:

```rust
use nsh_core::prelude::*;
use nsh_longbridge::{Size, pagination};

let page = signal(2u32);

pagination()
    .size(Size::Sm)
    .page(move || page.get())
    .total_pages(10)
    .on_change(move |next: &u32| page.set(*next))
    .accessibility_label("Results pages")
```

## NSH gaps

1. **The items are `nsh/base`'s views** (accepted, as `Select`'s rows): their states (`hover:`, `selected:`,
   `focus-visible:`, `disabled:`) are the item's, which an inline style cannot take and the library's content cannot
   see (nsh-core's typed styles have no `group_selected` / `group_disabled`). Workaround: the states as an `itemClass`
   generated from the resolved tokens in a binding (Implementation), the geometry as a bound `itemStyle`. Named
   group variants for selected and disabled, or states in an inline style, would let the library draw them without
   generated classes.
2. **No library `Button` inside an item**: an item is `nsh/base`'s pressable, and a pressable inside it would take
   the press (popover.md, NSH gaps 7). So the items are drawn as Button looks, not as Buttons; a change to Button's
   look reaches them through the shared tokens, not its code.
3. **The words of Previous / Next by window width** (`sm:`): NSH's breakpoints are the window's, as Tailwind's
   viewport ones; a pagination in a narrow panel of a wide window keeps its words. `compact` is the way there.

## Decisions

Proposed; for review.

1. **Data-driven, not composed by hand** (Differs from shadcn's structure): `nsh/base` computes the pages shown and
   owns the items' focus, so `Pagination` takes `page` and `totalPages`.
2. **Buttons, not links**: an NSH app navigates by state; `onChange` reports the page. The cursor is the arrow.
3. **Button's tokens for the items**, no `pagination-item-*` copies: the items are buttons in shadcn
   (`buttonVariants`) and should follow a theme's buttons.
4. **The library's words** "Previous", "Next", "Go to previous page", "Go to next page" and "Pagination" join `labels`
   (contract change: new `Label` keys).
