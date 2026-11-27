# Toggle

Status: spec (P3), not implemented.

A button that is on or off: bold in a text toolbar, a favorite star, a view option, a mute. Its look is shadcn/ui
new-york v4's `toggle.tsx` (Radix Toggle); its behavior is `nsh/base`'s `Toggle`. This spec also owns the **toggle
look** (the `toggle-*` tokens), which [ToggleGroup](toggle-group.md)'s items share.

## Usage

- A formatting or view option that stays on until pressed again: bold, italic, underline; show grid; word wrap; a
  favorite or a pin in a toolbar.
- Search terms: toggle, toggle button, pressed button, on/off button, toolbar toggle, icon toggle, star, bookmark.

## When not to use

| Need | Use instead |
|---|---|
| A setting on or off that reads as a setting (a row in a settings page) | [Switch](switch.md) |
| An option a form submits, with a label beside a box | [Checkbox](checkbox.md) |
| One of a few options, or several, as a row of buttons | [ToggleGroup](toggle-group.md) |
| An action (it does something, it holds no state) | [Button](button.md) |
| Switching the views of a page | [Tabs](tabs.md) |

## Built on

The root is `nsh/base`'s `Toggle` (`crates/nsh-base/src/toggle.rs`). It has no look; this library adds it. What
`nsh/base` already does, not rewritten:

| Provided by `nsh/base` | |
|---|---|
| Presses | A tap, a click, Space or Return, a screen reader's activation report `onChange({ pressed })` with the opposite of `pressed`; the app owns the value and the toggle shows the one passed back |
| Disabled | No presses, no press feedback; disabled with a disabled `ToggleGroup` or `Toolbar` around it |
| Accessibility | The `button` role with its pressed state (a toggle button), `accessibilityLabel` |
| Toolbars | In a `ToggleGroup` or a `Toolbar`, one of its items: arrow keys move between them without pressing, the group is one Tab stop |
| Hit area | `hitSlop` |
| Terminal | `nsh/base`'s terminal look; this library's does not apply there |

This library adds the look (tokens), `variant`, `size`, the label part and the press sink.

### Implementation

- `#[component] fn Toggle` whose root is `nsh_base::toggle()`, `pressed`, `disabled`, `accessibilityLabel` and
  `hitSlop` passed through. The look is set on the root element with bindings (as Checkbox's root): the fill and text
  color read `pressed` (not `nsh/base`'s projected `aria-pressed:`, which holds only while not disabled, so a disabled
  pressed toggle would look off: Checkbox's NSH gap 2).
- Hover is the root's `hover:` refinement; the press sink is `foundation::press(0.97)` on the root; the focus ring is
  `foundation::focus_ring` (the edge turns `toggle-border-focus` in the outline variant).
- Children are wrapped in `label` parts (Button's G1). The toggle provides `icon-size-<size>` as the icon size
  (`provide_icon_size`: the library's `Icon` among the children follows it) and its text color (conventions'
  `provide_text_color()`: icons drawn in `currentColor` follow the pressed and hover colors). There is no `icon`
  prop: the library's `IconName` holds only the icons its own components draw, and a toggle's icon is the app's
  (bold, italic, a star), given as a child at `icon-size-<size>` (16 at `md`).

## API

### Props

| TSX | Rust | Type | Default | Description |
|---|---|---|---|---|
| `pressed` | `pressed` | `boolean` / `Prop<bool>` | `false` | On or off (controlled) |
| `variant` | `variant` | `"default"` \| `"outline"` (`ToggleVariant`) | `"default"` | See Variants |
| `size` | `size` | `Size` | `"md"` | Height, type, icon and padding together |
| `disabled` | `disabled` | `boolean` | `false` | No presses, no hover or press feedback, at `opacity-disabled` |
| `accessibilityLabel` | `accessibility_label` | `string` | — | What it turns on, when its content has no text (an icon alone): required then |
| `hitSlop` | `hit_slop` | `EdgeInsets` | automatic (Pointer and touch) | Given: used as is |
| `children` | `.child(..)` | string, elements | — | The content: an icon (the app's, or the library's `Icon`), a word, or both; each child in a `label` part |
| `class` / `style` | `class` / `style` | | | The root, merged last |
| `labelClass` / `labelStyle` | `label_class` / `label_style` | | | The `label` parts |

Controlled only, as `nsh/base`'s and the library's other on/off controls (`Checkbox`, `Switch`): no `defaultPressed`.
`nsh/base`'s `pressedStyle` is not exposed (the pressed colors are tokens), nor `onPressDown` / `onPressUp` (the
library draws the press feedback).

### Events

| TSX | Rust | When |
|---|---|---|
| `onChange({ pressed })` | `ToggleEvent::Change(bool)` (`#[args(pressed)]`); `.on_change(\|pressed: &bool\| ..)` | Pressed: the value it asks for. shadcn's `onPressedChange` |

### Parts

| Part | What | Customize with |
|---|---|---|
| root | `nsh/base`'s `Toggle`: a row, centered, `gap` by `size` | `class` / `style` |
| `label` | A text wrapping each child | `labelClass` / `labelStyle` |

## Variants and sizes

shadcn: `inline-flex items-center justify-center gap-2 rounded-md text-sm font-medium whitespace-nowrap
transition-[color,box-shadow] hover:bg-muted hover:text-muted-foreground focus-visible:border-ring
focus-visible:ring-[3px] focus-visible:ring-ring/50 disabled:opacity-50 data-[state=on]:bg-accent
data-[state=on]:text-accent-foreground [&_svg]:size-4`.

### Variants

| `variant` | At rest | Hover (pointer) | Pressed (on) | shadcn |
|---|---|---|---|---|
| `default` | No fill, text `toggle-foreground` | Fill `toggle-hover` (`muted`), text `toggle-hover-foreground` (`muted-foreground`) | Fill `toggle-pressed` (`accent`), text `toggle-pressed-foreground` (`accent-foreground`) | `bg-transparent` |
| `outline` | As `default`, and an edge `border-width` × `toggle-outline-border` (`input`), shadow tier `toggle-outline-shadow` (1, `xs`) | Fill `toggle-outline-hover` (`accent`), text `toggle-outline-hover-foreground` (`accent-foreground`) | As `default` | `border border-input bg-transparent shadow-xs hover:bg-accent hover:text-accent-foreground` |

Pressed wins over hover (shadcn's `data-[state=on]:` follows `hover:` in Tailwind 4's order), so an on toggle under
the pointer keeps its pressed fill. shadcn's toggle has no `dark:` classes: dark takes the same roles in their dark
values.

### Sizes

| `size` | Height (pointer / touch) | Min width | Horizontal padding | Gap | Type | Icon | shadcn |
|---|---|---|---|---|---|---|---|
| `xs` | 24 / 32 | = height | 4 (`spacing`) | 4 | 12 | 12 | — |
| `sm` | 28 / 36 | = height | 6 (`spacing × 1.5`) | 6 | 13 | 14 | `sm`: `h-8 min-w-8 px-1.5` |
| `md` | 32 / 40 | = height | 8 (`spacing × 2`) | 8 | 14 | 16 | `default`: `h-9 min-w-9 px-2` |
| `lg` | 36 / 48 | = height | 10 (`spacing × 2.5`) | 8 | 14 | 16 | `lg`: `h-10 min-w-10 px-2.5` |

- Heights are one step below shadcn's, as Button's (`md` is 32, a medium desktop density); padding keeps shadcn's for
  the size it stands in for (`md` takes `default`'s `px-2`). A Toggle and a Button of one `size` line up in a toolbar.
- The minimum width is the height, so an icon alone makes a square (shadcn's `min-w-9`).
- Gap and icon sizes are Button's (foundation's gap by size, `icon-size-<size>`); type `font-size-<size>` at
  `font-weight-medium`, one line.
- Corners `toggle-radius` (`radius-md`, 8) at every size.

## States

| State | Look | Behavior |
|---|---|---|
| hover (pointer) | Variants; the arrow cursor | — |
| active (pressed down) | The sink to 0.97 (`foundation::press`; none with reduced motion); the colors of hover | — |
| on (`pressed`) | Variants | Announced as pressed |
| focus-visible | A `ring-width` (3) ring of `toggle-ring` (`ring` at 50%) outside, no layout; the outline variant's edge `toggle-border-focus` (`ring`). Touch: the edge only | Keyboard focus only |
| disabled | `opacity-disabled` (0.5), no hover, no sink; the on look kept if pressed | No presses, no focus |
| invalid, loading | Not offered | shadcn's toggle carries the shared `aria-invalid:` classes but no form value; a toggle group in a form shows its error through [Field](field.md) |

Colors, the edge and the ring transition over `duration` (shadcn's `transition-[color,box-shadow]`; NSH groups colors
and shadows, so the fill transitions too); none with reduced motion. The sink changes at once (Checkbox's note: one
set of transitioned properties per view).

## Pointer and touch

| | Pointer | Touch |
|---|---|---|
| Height | `control-height-*`: 24 / 28 / 32 / 36 | `control-height-touch-*`: 32 / 36 / 40 / 48 |
| Hit area | The box | Below `touch-target` (44), a hit slop of (44 − side) / 2 each way (`foundation::hit_slop`), the look unchanged; in a `ToggleGroup` with no gap, vertical only (side by side, horizontal slop would take the neighbor's presses) |
| Hover | Yes | No: the pressed look shows the change |
| Focus | The ring | The edge only (outline), nothing else |

## Accessibility

`nsh/base`'s: a `button` with its pressed state ("toggle button" / "switch button" by platform), named by its text
or `accessibilityLabel`; dimmed while disabled; Space and Return press it. An icon child is read only if it has a
label of its own (leave it unlabelled and name the toggle). In a
`ToggleGroup` or a `Toolbar`, the arrow keys move between the toggles (see [ToggleGroup](toggle-group.md)).

## Tokens

The toggle look's tokens, declared in `toggle.rs` and read by `ToggleGroup`'s items; `ToggleGroup` adds only its own.
Generated from the code's declarations (`NSH_LONGBRIDGE_WRITE=1 cargo test --test tokens`); a test keeps it current.
Values are those of `nsh/base`'s default theme.

<!-- tokens:toggle -->
| Token | Type | Derived default | Value |
|---|---|---|---|
| `toggle-radius` | length | `radius-md` | 8 |
| `toggle-height-xs` | length | `pointer_touch(control-height-xs, control-height-touch-xs)` | 24 (touch 32) |
| `toggle-height-sm` | length | `pointer_touch(control-height-sm, control-height-touch-sm)` | 28 (touch 36) |
| `toggle-height-md` | length | `pointer_touch(control-height-md, control-height-touch-md)` | 32 (touch 40) |
| `toggle-height-lg` | length | `pointer_touch(control-height-lg, control-height-touch-lg)` | 36 (touch 48) |
| `toggle-padding-x-xs` | length | `spacing` | 4 |
| `toggle-padding-x-sm` | length | `spacing × 1.5` | 6 |
| `toggle-padding-x-md` | length | `spacing × 2` | 8 |
| `toggle-padding-x-lg` | length | `spacing × 2.5` | 10 |
| `toggle-foreground` | color | `foreground` |  |
| `toggle-hover` | color | `muted` |  |
| `toggle-hover-foreground` | color | `muted-foreground` |  |
| `toggle-pressed` | color | `accent` |  |
| `toggle-pressed-foreground` | color | `accent-foreground` |  |
| `toggle-outline-border` | color | `input` |  |
| `toggle-outline-hover` | color | `accent` |  |
| `toggle-outline-hover-foreground` | color | `accent-foreground` |  |
| `toggle-outline-shadow` | number | `1` | 1 |
| `toggle-border-focus` | color | `ring` |  |
| `toggle-ring` | color | `ring × ring-opacity` |  |
<!-- /tokens -->

- `pressed` takes the variant's place in the names (`toggle-pressed`), as `checkbox-checked`: it is the control's
  value, not an interaction state.
- An app that changes a touch value alone writes `<name>-touch` (tokens.md).
- Other tokens it reads: `icon-size-*`, `font-size-*`, `font-weight-medium`, `border-width`, `ring-width`,
  `opacity-disabled`, `touch-target`, `shadow`, `duration`.

## Examples

TSX:

```tsx
import { signal } from "nsh";
import { Icon } from "nsh-icon";
import { Bold, Bookmark, Italic, Underline } from "nsh-icon-lucide";
import { Toggle } from "nsh-longbridge";

const bold = signal(false);

<Toggle pressed={bold()} onChange={({ pressed }) => bold.set(pressed)} accessibilityLabel="Toggle bold">
  <Icon icon={Bold} size={16} />
</Toggle>

<Toggle variant="outline" size="sm" pressed={italic()} onChange={({ pressed }) => italic.set(pressed)} accessibilityLabel="Toggle italic">
  <Icon icon={Italic} size={14} />
</Toggle>

// With text
<Toggle pressed={saved()} onChange={({ pressed }) => saved.set(pressed)}>
  <Icon icon={Bookmark} size={16} />
  Bookmark
</Toggle>

<Toggle disabled accessibilityLabel="Toggle underline">
  <Icon icon={Underline} size={16} />
</Toggle>
```

With NSH 0.7.0, write a string child inside `<Text>` for now ([Button's G0](button.md#nsh-gaps)).

Rust:

```rust
use nsh_core::prelude::*;
use nsh_longbridge::{Size, ToggleVariant, toggle};

let bold = signal(false);

toggle()
    .child(bold_icon()) // the app's icon element, 14 at `sm`
    .variant(ToggleVariant::Outline)
    .size(Size::Sm)
    .pressed(move || bold.get())
    .on_change(move |pressed: &bool| bold.set(*pressed))
    .accessibility_label("Toggle bold")
```

`nsh/base` exports a `Toggle` too; an app using both renames one (`import { Toggle as BaseToggle } from "nsh/base"`).

## NSH gaps

1. **The projected `aria-pressed:` does not hold while disabled** (as Checkbox's gap 2): the pressed colors are
   bindings on `pressed`.
2. **Font weight and cursor are read once** (Button's G2).
3. **String children** (Button's G0).

## Decisions

To settle in review:

1. **Heights one step below shadcn's**, padding kept per shadcn size, as Button: toggles sit in desktop toolbars
   beside buttons.
2. **Controlled only**, as `nsh/base`'s `Toggle` and the library's `Checkbox` and `Switch`.
3. **No `icon` prop** (Button and `TabsTrigger` have one): the library's `IconName` has only its own components'
   icons, and a toggle's icon is the app's. If `IconName` grows into a general icon set, an `icon` prop follows.
