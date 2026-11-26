# Button

Status: implemented (P0). The look follows shadcn/ui new-york v4 `button.tsx`; the behavior is `nsh/base`'s `Button`.
This is the library's reference component: the others are written the same way.

## Usage

A button runs one action when pressed: submit, save, confirm, cancel, delete, open a menu or a dialog.

Search terms: button, primary button, secondary button, outline button, ghost button, destructive button, link button,
icon button, loading button.

## When not to use

| Need | Use instead |
|---|---|
| Open a URL or an external page | `nsh/base`'s `Link` (the system's link handling; screen readers say "link"). Not a `variant="link"` button |
| Turn a state on or off (bold, mute) | `Toggle` (later) or `Switch` |
| Pick one of a set | `ToggleGroup` (later) or `RadioGroup` |
| A row of icon buttons in a toolbar | Still Button (`variant="ghost" iconOnly`), inside `nsh/base`'s `Toolbar` for arrow-key navigation |

## Built on

The root is `nsh/base`'s `Button` (`nsh_base::button()`); the library does not reimplement its behavior. It gives:

- Presses from the pointer, touch and the keyboard (Enter or Space while focused), as `onPress`. Inside a `Toolbar` it
  is one of the toolbar's items (arrow-key navigation, disabled with the toolbar).
- `disabled` (no presses, announced as dimmed, `disabled:` styles apply) and `busy` (announced as busy).
- The `button` role, `accessibilityLabel` and `accessibilityHint`.
- `hitSlop`: a larger press area without changing layout or look.
- A Tab stop; `focus-visible:` holds only for keyboard focus.

The library adds the look (tokens), `variant`, `size`, icons, the icon-only form, `loading`, `invalid` and the press
sink.

## API

### Props

| TSX | Rust (field / setter) | Type | Default | Description |
|---|---|---|---|---|
| `variant` | `variant` | `"default"` \| `"secondary"` \| `"outline"` \| `"ghost"` \| `"destructive"` \| `"link"` (`ButtonVariant`) | `"default"` | The look; see Variants |
| `size` | `size` | `"xs"` \| `"sm"` \| `"md"` \| `"lg"` (`Size`) | `"md"` | Height, type, icon and padding together; see Sizes |
| `iconOnly` | `icon_only` | `boolean` | `false` | A square showing only its icon (`icon`, or the spinner while loading); children are not shown. Its side is the height of its `size`. Needs an `accessibilityLabel` |
| `icon` | `icon` | `IconName` | — | An icon before the content (the library's names: `"plus"`, `"arrow-right"`, …) |
| `iconEnd` | `icon_end` | `IconName` | — | An icon after the content (a menu's `chevron-down`, a next step's `arrow-right`) |
| `loading` | `loading` | `boolean` | `false` | The action is running: a spinner replaces `icon` (or comes first without one), no presses, no hover or press feedback, **not dimmed**, announced as busy |
| `disabled` | `disabled` | `boolean` | `false` | No presses, no hover or press feedback, at `opacity-disabled`; the cursor stays an arrow |
| `invalid` | `invalid` | `boolean` | `false` | The app's checks fail (a form's submit with errors): the outline variant's edge turns `destructive`; the focus ring uses `button-ring-invalid` |
| `accessibilityLabel` | `accessibility_label` | `string` | — | What screen readers say when the content is not enough (always for icon-only). Icon-only and loading without one: "Loading" (`labels`) |
| `accessibilityHint` | `accessibility_hint` | `string` | — | What pressing it does, for screen readers |
| `labelClass` / `labelStyle` | `label_class` / `label_style` | `string` / `StyleProp<TextStyle>` | — | The `label` part; see Parts |
| `iconClass` / `iconStyle` | `icon_class` / `icon_style` | `string` / `StyleProp<ViewStyle>` | — | The `icon` part: `icon`, `iconEnd` and the spinner |
| `class` / `style` | `class` / `style` | | — | The root, merged last, over the component's own |
| `children` | `.child(..)` | string, elements | — | The content: usually its label (a string), or any element (the library's `Icon`, a badge) |

### Events

| TSX | Rust (`ButtonEvent` variant / setter) | Data | Description |
|---|---|---|---|
| `onPress` | `Press` / `.on_press(..)` | — | Pressed (never while `disabled` or `loading`) |

No `onPressDown` / `onPressUp`: the library draws the press feedback itself. They can be added later under
`nsh/base`'s names.

### Parts

| Part | What | Customize with |
|---|---|---|
| root | `nsh/base`'s `Button`: a row, centered, `gap` by `size` | `class` / `style` |
| `label` | A text wrapping each child: a string child's text inherits its type, weight and color | `labelClass` / `labelStyle` |
| `icon` | `icon`, `iconEnd` and the spinner (lucide, `currentColor`: the button's text color) | `iconClass` / `iconStyle` |

Each child gets its own `label` (see NSH gaps, G1): a string child's text takes the label's text styles
(`labelClass="text-base underline"` works). An element child sits in a label too; its own classes win.

## Variants and sizes

### Variants

| `variant` | At rest | Hover / pressed | Dark | shadcn classes |
|---|---|---|---|---|
| `default` | `button-default` (`primary`) fill, `button-default-foreground` text | fill at 90% | same | `bg-primary text-primary-foreground hover:bg-primary/90` |
| `secondary` | `secondary` fill | fill at 80% | same | `bg-secondary text-secondary-foreground hover:bg-secondary/80` |
| `outline` | `background` fill, 1pt `border` edge, `xs` shadow | `accent` fill, `accent-foreground` text | fill `input` at 30%, edge `input`, hover fill `input` at 50% | `border bg-background shadow-xs hover:bg-accent … dark:border-input dark:bg-input/30 dark:hover:bg-input/50` |
| `ghost` | no fill | `accent` fill, `accent-foreground` text | hover fill `accent` at 50% | `hover:bg-accent hover:text-accent-foreground dark:hover:bg-accent/50` |
| `destructive` | `destructive` fill, white text | fill at 90% | fill at 60% (hover 90%), ring at 40% | `bg-destructive text-white hover:bg-destructive/90 … dark:bg-destructive/60` |
| `link` | no fill, `primary` text | underlined | same | `text-primary underline-offset-4 hover:underline` |

A press shows the hover fill (touch screens have no hover, so this is where the change shows), plus the sink (see
States).

### Sizes

| `size` | Height (pointer / touch) | Horizontal padding (with an icon) | Gap | Type | Icon | shadcn |
|---|---|---|---|---|---|---|
| `xs` | 24 / 32 | 8 (6) | 4 | 12 | 12 | `xs`: `h-6 px-2 has-[>svg]:px-1.5 text-xs gap-1 size-3` |
| `sm` | 28 / 36 | 12 (10) | 6 | 13 | 14 | `sm`: `h-8 px-3 has-[>svg]:px-2.5 gap-1.5` |
| `md` | 32 / 40 | 16 (12) | 8 | 14 | 16 | `default`: `h-9 px-4 has-[>svg]:px-3 gap-2` |
| `lg` | 36 / 48 | 24 (16) | 8 | 14 | 16 | `lg`: `h-10 px-6 has-[>svg]:px-4` |

- Heights are `control-height-*` (pointer) and `control-height-touch-*` (touch), one step below shadcn's (36 by
  default, `md` here is 32): a medium desktop density. For shadcn's heights, set `control-height: 36`.
- Paddings and gaps are multiples of `spacing`. With an icon (`icon`, `iconEnd` or loading) both sides shrink, as
  shadcn's `has-[>svg]` does.
- Type is `font-size-*`, its line height from foundation's `line_height`; weight `font-weight-medium` (500).
- Icons are `icon-size-*` (12 / 14 / 16 / 16), paired with the type. The button puts its `size` in context, so an
  `Icon` among its children without a size follows it. shadcn uses 16 at every size but `xs`; `sm` here uses 14 next
  to 13pt type.
- The radius is `button-radius` (`radius-md`, 8) at every size.

## States

| State | Look | Behavior |
|---|---|---|
| hover (pointer) | See Variants | Cursor: a hand for `link`, an arrow for the others (desktop convention) |
| active (pressed) | The hover fill, and a sink to 0.97 (foundation's `press`; no scaling with reduced motion) | — |
| focus-visible | A `ring-width` (3) ring of `button-ring` (`ring` at 50%) outside the button, taking no layout; the outline variant's edge turns `button-border-focus`. `destructive` uses `button-destructive-ring`. On touch screens only the edge changes | Keyboard focus only |
| invalid | The outline variant's edge is `button-border-invalid`; the focus ring `button-ring-invalid` | Not announced (a button holds no form value) |
| disabled | `opacity-disabled` (0.5), no hover, no sink | No presses, no focus |
| loading | The spinner replaces the leading icon, **not dimmed**, no hover, no sink | No `onPress`, from the keyboard either; announced as busy; still focusable |

Colors, shadow and the sink transition over `duration` (150ms, `cubic-bezier(0.4, 0, 0.2, 1)`), as shadcn's
`transition-all`; with reduced motion they take no time. The spinner keeps turning with reduced motion: it says that
something is happening.

## Pointer and touch

| | Pointer (desktop, desktop web) | Touch |
|---|---|---|
| Height | `control-height-*`: 24 / 28 / 32 / 36 | `control-height-touch-*`: 32 / 36 / 40 / 48 |
| Press area | The visible bounds | Below `touch-target` (44), a `hitSlop` of (44 − height) / 2 on each side; the look is unchanged |
| Focus | The ring | The edge color only |
| Hover | Yes | No (the fill changes on press) |

A theme sets a touch-only value with `<name>-touch` (for example `button-height-md-touch`).

## Accessibility

- Role `button` (from `nsh/base`); the name is the content's text, or `accessibilityLabel`.
- `loading` sets the busy state; an icon-only button that is loading and has no `accessibilityLabel` is named
  "Loading" (`labels`, translatable with `set_translator`).
- `disabled` is announced as dimmed and leaves the Tab order; `loading` keeps the button focusable.
- Icons are hidden from screen readers.
- `invalid` is not announced: the field that failed carries the error.

## Tokens

Generated from the code's declarations (`NSH_LONGBRIDGE_WRITE=1 cargo test --test tokens`); a test keeps it current.
Values are those of `nsh/base`'s default theme.

<!-- tokens:button -->
| Token | Type | Derived default | Value |
|---|---|---|---|
| `button-radius` | length | `radius-md` | 8 |
| `button-height-xs` | length | `pointer_touch(control-height-xs, control-height-touch-xs)` | 24 (touch 32) |
| `button-height-sm` | length | `pointer_touch(control-height-sm, control-height-touch-sm)` | 28 (touch 36) |
| `button-height-md` | length | `pointer_touch(control-height-md, control-height-touch-md)` | 32 (touch 40) |
| `button-height-lg` | length | `pointer_touch(control-height-lg, control-height-touch-lg)` | 36 (touch 48) |
| `button-padding-x-xs` | length | `spacing × 2` | 8 |
| `button-padding-x-sm` | length | `spacing × 3` | 12 |
| `button-padding-x-md` | length | `spacing × 4` | 16 |
| `button-padding-x-lg` | length | `spacing × 6` | 24 |
| `button-default` | color | `primary` |  |
| `button-default-foreground` | color | `primary-foreground` |  |
| `button-default-hover` | color | `button-default × 90%` |  |
| `button-secondary` | color | `secondary` |  |
| `button-secondary-foreground` | color | `secondary-foreground` |  |
| `button-secondary-hover` | color | `button-secondary × 80%` |  |
| `button-outline` | color | `light_dark(background, input × 30%)` |  |
| `button-outline-foreground` | color | `foreground` |  |
| `button-outline-border` | color | `light_dark(border, input)` |  |
| `button-outline-hover` | color | `light_dark(accent, input × 50%)` |  |
| `button-outline-hover-foreground` | color | `accent-foreground` |  |
| `button-outline-shadow` | number | `1` | 1 |
| `button-ghost-foreground` | color | `foreground` |  |
| `button-ghost-hover` | color | `light_dark(accent, accent × 50%)` |  |
| `button-ghost-hover-foreground` | color | `accent-foreground` |  |
| `button-destructive` | color | `light_dark(destructive, destructive × 60%)` |  |
| `button-destructive-foreground` | color | `destructive-foreground` |  |
| `button-destructive-hover` | color | `destructive × 90%` |  |
| `button-destructive-ring` | color | `light_dark(destructive × 20%, destructive × 40%)` |  |
| `button-link-foreground` | color | `primary` |  |
| `button-border-focus` | color | `ring` |  |
| `button-border-invalid` | color | `destructive` |  |
| `button-ring` | color | `ring × ring-opacity` |  |
| `button-ring-invalid` | color | `ring-invalid` |  |
<!-- /tokens -->

Other tokens it reads: `spacing` (gaps, and the padding taken off beside an icon), `font-size-*`, `icon-size-*`,
`font-weight-medium`, `border-width`, `ring-width`, `opacity-disabled`, `touch-target` and `shadow`.

## Examples

TSX:

```tsx
import { Button } from "nsh-longbridge";

<Button onPress={save}>Save</Button>
<Button variant="outline" icon="plus">New file</Button>
<Button variant="secondary" size="sm" iconEnd="chevron-down">Options</Button>
<Button variant="destructive" icon="trash-2" onPress={remove}>Delete</Button>
<Button variant="ghost" iconOnly icon="plus" accessibilityLabel="Add" />
<Button loading={saving()} onPress={save}>Saving…</Button>
<Button variant="link" onPress={showTerms}>Terms of use</Button>

// Parts: a semibold label, a blue icon.
<Button variant="outline" labelClass="font-semibold" iconClass="text-blue-600" icon="arrow-right">Next</Button>
```

With NSH 0.7.0, write a string child inside `<Text>` for now (see G0).

Rust:

```rust
use nsh_core::prelude::*;
use nsh_longbridge::{ButtonVariant, IconName, Size, button};

button()
    .variant(ButtonVariant::Outline)
    .size(Size::Sm)
    .icon(IconName::Plus)
    .on_press(move || add())
    .child("New file")

button().variant(ButtonVariant::Ghost).icon_only(true).icon(IconName::Plus).accessibility_label("Add")
```

## NSH gaps

- **G0: string children fail in NSH 0.7.0.** A TSX string child of any Rust component (`nsh/base`'s `Button`
  included) fails with `invalid op: UnknownNode` and shows nothing. Fixed on NSH main (#3324), not yet released.
  Until then, wrap the string: `<Button><Text>Save</Text></Button>`. Rust's `.child("Save")` is not affected.
- **G1: a component cannot tell a string child from an element.** `Children` (and each `Child` of
  `children_each`) is a node handle. So every child gets a `label` text: a string's text inherits the label's
  styles and `labelClass` / `labelStyle` work on it; an element child is wrapped too, mostly without effect. If NSH
  let a `Child` say whether it is text, only strings would be wrapped.
- **G2: font weight is not bindable.** `font_weight` takes a static value, so `font-weight-medium` is read when the
  button is built; a `setTheme` that changes it applies once the button is built again. Every other token is a
  binding and applies at once. Likewise the underline (`underline()` takes no binding) and the cursor follow the
  `variant` the button was built with; a `variant` changed later to or from `link` keeps them until it is built
  again.
- **G3: the shadow is an inline style.** A `BoxShadow`'s color is fixed and `.shadow()` is not bindable. The library
  computes the layers in a binding, from the current appearance and the `shadow` token, and sets them as the root's
  inline `Style`. The app's `style` still merges after it.

## Decisions

- **`iconOnly`, not `size="icon"`.** shadcn v4 has `size` values `icon`, `icon-xs`, `icon-sm` and `icon-lg`. Here
  `size` is foundation's four-step `Size` in every component; putting a shape into it would make its values differ
  by component. `iconOnly` is orthogonal: `<Button iconOnly size="sm">` is shadcn's `icon-sm`, with the same height,
  icon and press area as a text button of that size, so they line up in a row. shadcn's `icon` (36) is `md` (32)
  here, one density step down.
- **`loading` replaces the leading icon and does not dim.** With an `icon` the width stays the same; dimming would read
  as disabled.
- **The cursor is an arrow** except for `link`, as desktop apps do.
