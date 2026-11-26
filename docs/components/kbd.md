# Kbd

Key caps: a key or a key combination drawn as small caps. Reference: shadcn/ui new-york v4 `kbd.tsx` (`Kbd`,
`KbdGroup`).

## Usage

- Shortcuts in menu items, buttons and tooltips (`Tooltip`); a key mentioned in help text.
- Search terms: kbd, keyboard shortcut, hotkey, key hint.

## When not to use

- A read-only status mark: use `Badge`.
- Monospaced code that is not a key: the app's own `text` (`font-mono`), not `Kbd`.
- Keys of an on-screen keyboard are buttons: use `Button`.

## Built on

`nsh/base` has no counterpart; this library draws a `view` and a `text`. It has no behavior: not focusable, no pointer
events (shadcn's `pointer-events-none`), text not selectable (`select-none`).

Key names are the app's: `Kbd` does not parse `"mod+k"`. For the platform's form (`⌘K` on Apple platforms, `Ctrl+K`
elsewhere), compute it with NSH's `keymap.label("mod+k")` (Rust `nsh_core::element::key_label("mod+k")`) and pass the
result; see "Examples".

## API

### `Kbd`

| TSX | Rust | Type | Default | Description |
|---|---|---|---|---|
| `children` | `.child(..)` | Text or an icon | | The key's name; may be a library icon (`⌘`, arrow keys) |
| `class` / `style` | `class` / `style` | | | The root, which is the cap |

### `KbdGroup`

Lays several `Kbd`s out in a row (`Ctrl` `+` `B`), `kbd-group-gap` apart.

| TSX | Rust | Type | Description |
|---|---|---|---|
| `children` | `.child(..)` | `Kbd`, text | Words between keys (`+`, `then`) are the app's own `text` |
| `class` / `style` | `class` / `style` | | The root |

Neither has events or other parts (the cap is the root and the text is the children; no `textClass`).

## Variants and sizes

- No `variant`.
- No `size` (none of `xs` / `sm` / `md` / `lg`): shadcn has one (20 high). It follows the row it sits in, not control
  sizes. If it is too large in a `size="xs"` button, change `kbd-height` with `class` or a scoped token.

### Look

shadcn: `inline-flex h-5 w-fit min-w-5 items-center justify-center gap-1 rounded-sm bg-muted px-1 font-sans text-xs
font-medium text-muted-foreground select-none`, icons `size-3`.

| | Value |
|---|---|
| Height / min width | `kbd-height` (20); the min width equals it, so a single character is square |
| Padding | `kbd-padding-x` (4) horizontally, centered vertically |
| Corners | `kbd-radius` |
| Fill / text | `kbd` / `kbd-foreground` |
| Text | `kbd-font-size` (12), `font-weight-medium`, `font-sans`, line height from the foundation's font size → line height, one line |
| Icons | 12 (the `xs` icon size the cap gives as context), colored as the text; `gap-1` (4) between icon and text |

### In a `Tooltip`

shadcn switches to `bg-background/20 text-background` (dark `bg-background/10`) with the selector
`[[data-slot=tooltip-content]_&]`, because the tooltip is inverted (`foreground` fill). NSH has no ancestor selectors,
so this library uses context: `Tooltip` provides an "inside a tooltip" context, and a `Kbd` that reads it uses
`kbd-tooltip` / `kbd-tooltip-foreground`. Nothing for the app to do.

## States

No hover / active / focus / disabled: not interactive. When the menu item or button around it is disabled, that row's
`opacity-disabled` already covers it; the cap does not dim again.

## Pointer and touch

Same look. Touch screens rarely have a hardware keyboard, so shortcut hints mean little there. Whether to show them is
the app's call (for example only when `platform.desktop`); the component does not hide itself.

## Accessibility

- No Tab stop, no pointer events.
- Read as plain text (`text`), without the `keyboardkey` role: in NSH `keyboardkey` is an element that acts as a key
  and can be pressed, and this is only a label. A cap usually sits in a button or menu item with its own label, which
  should say everything.

## Tokens

| Token | Type | Derived default | Light / dark |
|---|---|---|---|
| `kbd` | color | `muted` | Follows `muted` |
| `kbd-foreground` | color | `muted-foreground` | Follows `muted-foreground` |
| `kbd-tooltip` | color | `light_dark(tooltip-foreground at 20%, tooltip-foreground at 10%)` | shadcn's `bg-background/20` and `dark:bg-background/10`, based on the tooltip's text color rather than `background`, so the caps follow a recolored tooltip |
| `kbd-tooltip-foreground` | color | `tooltip-foreground` | Same |
| `kbd-radius` | length | `radius-sm` (6) | Same |
| `kbd-height` | length | 20 (`spacing` × 5) | Same |
| `kbd-padding-x` | length | `spacing` (4) | Same |
| `kbd-font-size` | length | `font-size-xs` (12) | Same |
| `kbd-group-gap` | length | `spacing` (4) | Same |

`kbd-height` does not follow `control-height`: it is a small label inside controls, and shadcn fixes it at `h-5`. A
denser `spacing` shrinks it along with everything else.

## Examples

```tsx
import { keymap } from "nsh";
import { Kbd, KbdGroup } from "nsh-longbridge";

<KbdGroup>
  <Kbd>{keymap.label("mod")}</Kbd>
  <Kbd>K</Kbd>
</KbdGroup>

<KbdGroup>
  <Kbd>Ctrl</Kbd>
  <Text class="text-xs text-muted-foreground">+</Text>
  <Kbd>B</Kbd>
</KbdGroup>

<Button tooltip={<Tooltip>Save <Kbd>{keymap.label("mod+s")}</Kbd></Tooltip>}>Save</Button>
```

```rust
use nsh_core::element::key_label;
use nsh_longbridge::{kbd, kbd_group};

kbd_group()
    .child(kbd().child(key_label("mod")))
    .child(kbd().child("K"))
```

## NSH gaps

- **No ancestor selectors** (shadcn's `[[data-slot=tooltip-content]_&]`). Workaround: `Tooltip` provides a context that
  `Kbd` reads. A `Kbd` a TSX app passes as a `Tooltip`'s child is adopted where the tooltip places its children, so it
  reads that context (NSH's `docs/dev/components.md`, "The host node and ids"). Headless tests guard both the TSX and the
  Rust path.

## Decisions

- No `keys="mod+k"`: the component does not parse key names; apps use `keymap.label(..)`.
