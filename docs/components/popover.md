# Popover

Status: spec (P2), not implemented. Rich content in a floating panel next to the element that opened it: a press on
the trigger opens it, a press outside or Escape closes it. Reference: shadcn/ui new-york v4 `popover.tsx` (Radix
Popover).

This spec also owns the **popup surface** every popup of the library shares (the fill, edge, corners, shadow and
open / close motion of `Popover`, `DropdownMenu`'s menus and `Select`'s list): see "The popup surface" under
"Variants and sizes". The layer rules (stacking, dismissal, focus, theme scope) are settled in
[dialog.md](dialog.md), "Overlays: what every layer shares".

## Usage

- A small form or a few settings next to a button (dimensions, a filter, a share panel), a date picker's calendar
  of the app's own, a "more about this" panel that must work on touch screens.
- Search terms: popover, popup panel, flyout, dropdown panel.

## When not to use

| Need | Use instead |
|---|---|
| A hint on hover, no interaction | `Tooltip` |
| A list of actions or options | `DropdownMenu`, `Select` (P2): their keys, typeahead and roles |
| Something that must be answered before going on | `Dialog`, `AlertDialog` |
| A preview shown while the pointer rests on a link | `nsh/base`'s `HoverCard` (this library's `HoverCard` later) |

## Built on

`nsh/base`'s `Popover` (`crates/nsh-base/src/popover.rs`). This library adds the look and the motion; all behavior
is `nsh/base`'s:

| Provided by `nsh/base` (not rewritten here) | |
|---|---|
| Opening | A press on the `trigger` slot's element toggles it (`"triggerPress"`); the trigger gets `expanded` and `hasPopup` for screen readers |
| Position | On the trigger's `placement` side, on the other side when only that one has room; `align` along it; `offset` from the trigger's box, at least `margin` from the safe area's edges; follows the trigger as it scrolls; above everything in its root view (the surface, or the dialog it is in) |
| Closing | A press outside (desktops: and the press goes on to what is under the pointer; touch screens: a transparent backdrop takes it), Escape and back; `closeOnPressOutside`, `closeOnEscape`; every attempt in `onCancel` |
| Focus | Not trapped; its content follows the trigger in the Tab order (Shift+Tab goes back to the trigger, Tab past the end goes on after the popover); as it closes the focus goes back to the trigger (opened by a pointer on macOS and iOS: back where it was), except after a press outside |
| Presence | `popupStartingClass` / `popupEndingClass`: its entrance from the starting look and its exit to the ending look, unmounted after; skipped with reduced motion |
| Control | `open` / `defaultOpen` / `onOpenChange({ open, reason })`, `onOpen`, `onClose`, `onCancel`, `onPosition({ placement, bounds, available })` |
| Toolbars | A trigger inside `nsh/base`'s `Toolbar` is one of its items; the content is not |

### Differs from shadcn's structure

shadcn has `Popover > PopoverTrigger + PopoverContent` (+ `PopoverAnchor`). Here, as `nsh/base`'s `Popover`:

- **No `PopoverTrigger`: a `trigger` slot**. NSH has no `asChild`, and `nsh/base` needs the trigger's element to place
  the popup against it and to give it the press and the accessibility state.
- **No `PopoverContent`: `Popover`'s children are the content**, and `Popover`'s `class` / `style` style the popup
  (the panel), as `Tooltip`'s style its bubble. shadcn's `PopoverContent` props `side`, `align`, `sideOffset` are
  `Popover`'s `placement`, `align` and the `popover-offset` token.
- **No `PopoverAnchor`**: `nsh/base` places the popup against its trigger; an anchor apart from the trigger is not
  offered (see NSH gaps 3).
- `PopoverHeader`, `PopoverTitle` and `PopoverDescription` are kept (shadcn's newer parts).

A child component cannot fill its parent's slot in NSH, so the trigger is a prop. From shadcn:

| shadcn | This library |
|---|---|
| `<Popover><PopoverTrigger asChild><Button>Open</Button></PopoverTrigger><PopoverContent>…</PopoverContent></Popover>` | `<Popover trigger={<Button>Open</Button>}>…</Popover>` |
| `<PopoverContent side="top" align="start" sideOffset={8} className="w-80">` | `<Popover placement="top" align="start" style={{ width: 320 }}>`; the offset is the `popover-offset` token |
| `PopoverHeader`, `PopoverTitle`, `PopoverDescription` | The same |
| `PopoverAnchor` | Not offered (NSH gaps 3) |
| `onOpenChange={(open) => …}` | `onOpenChange={({ open, reason }) => …}` |

### Implementation

- `#[component] fn Popover` whose body is `nsh_base::popover()` with the props passed through. The surface
  (`foundation::popup::surface(t)`: fill, edge, corners, shadow, width, padding, the text-style reset) is the
  popup's inline style (`popupStyle`), with the app's `style` merged after it; the app's `class` goes to
  `popupClass`. `popupStartingClass` / `popupEndingClass` are set by the library from the side (see "Motion") and
  `popover-animate`.
- `nsh/base`'s part styles take a style built once in Rust (Checkbox's NSH gap 3). The popup is made on each
  opening, so its surface reads the tokens as it opens: a `setTheme` or scope change while it is open applies at the
  next opening (accepted; NSH gaps 5).
- The app's `class` cannot override what the surface sets inline (fill, edge, corners, shadow, width, padding):
  inline styles win over classes. To change those, the app sets the tokens (in the theme or a `ThemeScope`) or
  passes `style`, merged after the library's. Classes still add what the surface leaves alone (a `max-h`, a gap).
- The popup's content is made by `nsh/base`'s `Popover`, inside this component: it reads the `ThemeScope` around the
  `Popover` (dialog.md, "The theme scope"). The popup resets inherited text styles (the overlay keeps its parent's).
- The library keeps the side the popup went on in a signal, set from `onPosition` (and passes the event on).

## API

### Components

| Component | TSX | Rust | Description |
|---|---|---|---|
| `Popover` | `<Popover>` | `popover()` | The root: trigger, popup, state; children are the content |
| `PopoverHeader` | `<PopoverHeader>` | `popover_header()` | A column for a title and a description |
| `PopoverTitle` | `<PopoverTitle>` | `popover_title()` | The panel's title (`font-medium`) |
| `PopoverDescription` | `<PopoverDescription>` | `popover_description()` | Muted text under the title |

`nsh/base` exports a `Popover` too (`nsh_base::popover()`); an app using both renames one.

### `Popover` props

| TSX | Rust | Type | Default | Description |
|---|---|---|---|---|
| `trigger` | `trigger` | element (slot) | — | What opens it and what it is placed against |
| `open` | `open` | `boolean?` | uncontrolled | Controlled open state |
| `defaultOpen` | `default_open` | `boolean` | `false` | Open at first, while `open` is not set |
| `placement` | `placement` | `"top"` \| `"bottom"` \| `"left"` \| `"right"` (`Placement`) | `"bottom"` | The side of the trigger (shadcn's `side`); flipped when only the other side has room |
| `align` | `align` | `"start"` \| `"center"` \| `"end"` (`Align`) | `"center"` | Alignment along that side; start / end follow the layout direction |
| `closeOnEscape` | `close_on_escape` | `boolean` | `true` | Escape and back close it; `onCancel` hears them either way |
| `closeOnPressOutside` | `close_on_press_outside` | `boolean` | `true` | A press outside closes it; `onCancel` hears it either way |
| `class` / `style` | `class` / `style` | | | The popup (the panel): `style` merged after the surface's inline look; `class` cannot override what the surface sets inline (see "Implementation") |
| `children` | `.child(..)` | elements | — | The content, made on each opening |

`offset` and `margin` are not props: they are the `popover-offset` (4, shadcn's `sideOffset`) and `popover-margin`
(4) tokens, the same across an app (as `Tooltip`'s). `popupStartingClass` / `popupEndingClass` are not exposed: the
motion is the library's look (`popover-animate: 0` turns it off).

### Events

All are `nsh/base`'s, passed on: `onOpenChange({ open, reason })` (`"triggerPress"`, `"pressOutside"`, `"cancel"`),
`onOpen`, `onClose`, `onCancel`, `onPosition({ placement, bounds, available })`. Rust: `PopoverEvent` with
`OpenChange(bool, OpenChangeReason)`, `Open`, `Close`, `Cancel`, `Position(Placement, Bounds<Points>,
Size<Points>)`.

### Parts

`PopoverHeader`, `PopoverTitle` and `PopoverDescription` take `children`, `class` and `style`; string children are
wrapped in a text that takes their styles. The popup is styled by `Popover`'s `class` / `style`.

## Variants and sizes

No `variant`, no `size`.

### Look

| Part | shadcn classes | This library |
|---|---|---|
| popup | `z-50 w-72 rounded-md border bg-popover p-4 text-popover-foreground shadow-md outline-hidden` | The popup surface (below), width `popover-width` (288), padding `popover-padding` (16); text `popover-foreground` at 14 / 20 (see Decisions 3); content in a column |
| `PopoverHeader` | `flex flex-col gap-1 text-sm` | A column, gap `popover-header-gap` (4), text 14 |
| `PopoverTitle` | `font-medium` | `font-weight-medium`; the inherited size and color |
| `PopoverDescription` | `text-muted-foreground` | Color `popover-description-foreground` (`muted-foreground`) |

The width is fixed, as shadcn's `w-72`; a content-sized popover sets `style={{ width: "auto" }}`, a wider one
`style={{ width: 360 }}` (or `popover-width` in a scope); a class cannot, as the width is inline (see
"Implementation").
The core lays an anchored popup out inside the safe area less the margin, so on a narrow screen it is never wider
than the screen.

### The popup surface (shared)

The look every popup of the library has; `Popover` owns its tokens, and `foundation` gets the function the popups
call (`foundation::popup::surface(t)`, defined and implemented with `Popover`: fill, edge, corners, shadow and the
text-style reset, as `foundation::field_look` is the fields'; `DropdownMenu`, `Select`'s list and later `Combobox`
call it), so a theme that changes `popover-radius` changes popovers, menus and
`Select`'s list together.

| | Value | Tokens | Used by |
|---|---|---|---|
| Fill / text | `popover` / `popover-foreground` | Semantic (shadcn's); no component token of the same name (tokens.md: `popover` is already the semantic name) | Popover, DropdownMenu, Select's list |
| Edge | `border-width` × `popover-border` (`border`) | `popover-border` | All three |
| Corners | `popover-radius` (`radius-md`, 8) | `popover-radius` | All three (shadcn: `rounded-md` on each) |
| Shadow | Tier `popover-shadow` (3, `md`) | `popover-shadow` | All three (shadcn: `shadow-md` on each) |
| Motion | See "Motion" | `popover-animate` | All three, on desktops (on touch screens menus and lists are `nsh/base`'s sheets, with their own motion) |
| Offset from the trigger | `popover-offset` (4) | `popover-offset` | All three (shadcn: `sideOffset={4}` on each) |
| Padding | Each its own: Popover `p-4` (`popover-padding`), the menus `p-1` (their tokens) | | |

Settled in review: the menus read these through `foundation::popup::surface`; the item look (heights, highlight,
separators, check and chevron icons, the destructive item) is the menu family's (`foundation::menu`, `menu-*`).

### Motion

| | shadcn (tw-animate-css) | This library |
|---|---|---|
| Entrance | `fade-in-0 zoom-in-95`, and `slide-in-from-{top,bottom,left,right}-2` by the side it went on (`data-[side=…]`), 150 ms, from Radix's transform origin (the trigger's side) | From `opacity-0 scale-95` and 8 toward the trigger (side `bottom`: `-translate-y-2`; `top`: `translate-y-2`; `right`: `-translate-x-2`; `left`: `translate-x-2`) to its look over `duration` (150), scale origin on the trigger's side (`origin-top` for `bottom`, …) |
| Exit | `fade-out-0 zoom-out-95` | To `opacity-0 scale-95` (no slide), then unmounted |
| Reduced motion | not handled | No transitions (`nsh/base`'s presence skips the states) |

The side is the one the popup went on (`onPosition`), not the requested `placement`: after a flip it slides from the
right side. Until the first report the requested `placement` is used. Whether `onPosition` comes before the first
display frame decides whether a flipped popup's very first frame starts from the wrong side; the first headless test
checks it. If it comes too late, the slide is dropped and only fade and zoom stay (Tooltip's choice).

## States

| State | Look |
|---|---|
| Entering / exiting | "Motion" |
| Trigger | `expanded` while open (`nsh/base` sets it): a library `Button` keeps its hover fill (button.md, "States"); screen readers hear it expanded |

No hover, focus or disabled look on the popup itself.

## Pointer and touch

| | Desktops and desktop browsers | Phones and tablets |
|---|---|---|
| Form | Anchored to the trigger | Anchored to the trigger (`nsh/base`: what it shows belongs next to it) |
| Press outside | Closes it and goes on to what is under the pointer | Only closes it (a transparent backdrop takes it) |
| Width | 288 | 288, never wider than the safe area less `popover-margin` on each side |
| Software keyboard | — | The core places anchored overlays above it |

No bottom-sheet form: `nsh/base` keeps a popover anchored everywhere, and phone apps show small anchored panels
(iPadOS and iOS popovers, Android's popup windows) for content tied to an element.

## Accessibility

- The trigger is announced as having a popup, expanded while open (`nsh/base`).
- The content is reached by Tab right after the trigger (`nsh/base`'s order); it is not a dialog for screen readers
  (Radix's popover content has `role="dialog"`; `nsh/base`'s has no role). See NSH gaps 2.
- `PopoverTitle` has no `header` role (shadcn's is a `div`; it is a label inside a panel, not a heading of the page).
- Escape closes it and the focus goes back to the trigger.

## Tokens

Proposed; once implemented, generated from `tokens!` (`<!-- tokens:popover -->`).

| Token | Type | Derived default | Value |
|---|---|---|---|
| `popover-border` | color | `border` |  |
| `popover-radius` | length | `radius-md` | 8 |
| `popover-shadow` | number | `3` | 3 |
| `popover-offset` | length | `spacing` | 4 |
| `popover-margin` | length | `spacing` | 4 |
| `popover-width` | length | `spacing × 72` | 288 |
| `popover-padding` | length | `spacing × 4` | 16 |
| `popover-header-gap` | length | `spacing` | 4 |
| `popover-description-foreground` | color | `muted-foreground` |  |
| `popover-animate` | number | `1` | 1 |

- `popover` and `popover-foreground` are shadcn's semantic tokens, used directly (as `card` by `Card`).
- `popover-margin` 4 is `nsh/base`'s `Tooltip`'s default margin, kept the same for every popup (Radix's
  `collisionPadding` is 0; a popup touching the window's edge looks cut).
- Other tokens it reads: `border-width`, `font-weight-medium`, `duration`, `shadow`.

## Examples

TSX:

```tsx
import { signal, View } from "nsh";
import { Button, Input, type InputRef, Label, Popover, PopoverDescription, PopoverHeader, PopoverTitle } from "nsh-longbridge";

const width = signal<InputRef>(); // read by the Label above it: kept in a signal
const open = signal(false);

<Popover trigger={<Button variant="outline">Open popover</Button>}>
  <View class="gap-4">
    <PopoverHeader>
      <PopoverTitle>Dimensions</PopoverTitle>
      <PopoverDescription>Set the dimensions for the layer.</PopoverDescription>
    </PopoverHeader>
    <View class="flex-row items-center gap-4">
      <Label htmlFor={width()}>Width</Label>
      <Input ref={width} size="sm" value="100%" />
    </View>
  </View>
</Popover>

// Above the trigger, aligned to its start, controlled.
<Popover
  placement="top"
  align="start"
  open={open()}
  onOpenChange={({ open: next }) => open.set(next)}
  trigger={<Button variant="ghost" iconOnly icon="plus" accessibilityLabel="Add" />}
>
  …
</Popover>
```

With NSH 0.7.0, write a string child inside `<Text>` for now (see [button.md, NSH gaps](button.md#nsh-gaps), G0).

Rust:

```rust
use nsh_core::{overlay::Placement, prelude::*};
use nsh_longbridge::{ButtonVariant, button, popover, popover_description, popover_header, popover_title};

popover()
    .placement(Placement::Bottom)
    .trigger(button().variant(ButtonVariant::Outline).child("Open popover"))
    .child(
        popover_header()
            .child(popover_title().child("Dimensions"))
            .child(popover_description().child("Set the dimensions for the layer.")),
    )
```

## NSH gaps

The ★ items are the first headless tests of the implementation; each has its fallback.

1. ★ **A trigger's tooltip above its open popover** (Tooltip's NSH gap 4, met now): a button with a `tooltip` opens a
   popover, the focus stays on the button, and the tooltip may show above the popover and take Escape first. The
   test reproduces it. If it happens, ask NSH for the fix (a hint not shown while a popup anchored to its element is
   open). Fallback in the library: its popups provide an "inside an open popup" context to their content and count
   open popups in a surface global; while one is open, a library `Tooltip` not inside an open popup is `disabled`.
   A tooltip of the app's own (not the library's) is not covered.
2. **No role on the popover's content.** Radix's popover content is a `dialog` for screen readers; `nsh/base`'s is a
   plain overlay. Noted; the library adds none (a non-modal `dialog` role would make screen readers treat it as
   modal on some platforms).
3. **No separate anchor.** `nsh/base`'s `Popover` places the popup against its trigger only (shadcn's
   `PopoverAnchor` places it against another element). An `anchor` prop on `nsh/base`'s `Popover` (the core's
   overlay already takes any ref) would allow it.
4. ★ **`onPosition` timing** decides the slide after a flip (see "Motion"). Fallback: no slide, fade and zoom only.
5. **Part styles built once (accepted).** `nsh/base`'s `popupStyle` takes a style built once in Rust; the surface is
   read as the popup opens, so a theme change while it is open shows at the next opening. Same for every popup on
   `foundation::popup::surface` (the menus, `Select`'s list).
6. ★ **The shadow on `nsh/base`'s popup.** The first test checks that the surface's shadow (an inline `BoxShadow`,
   as `Button`'s G3) survives on the popup overlay (not clipped, drawn on every platform). Fallback: popups draw no
   shadow, the border alone, and it is reported.

## Decisions

Settled in review.

1. **The API follows `nsh/base`'s `trigger` slot**, not shadcn's `PopoverTrigger` / `PopoverContent`; the panel is
   styled by `Popover`'s `class` / `style`.
2. **`Popover` owns the shared popup-surface tokens** (`popover-border`, `-radius`, `-shadow`, `-offset`,
   `-animate`), which the menus' group reads.
3. **The popup's text is 14 / 20**: shadcn's inherits the page's, NSH's would inherit the trigger's (a button's
   medium weight); the panel resets it to `text-sm`, the size of every shadcn part inside it.
4. **Anchored on touch screens too** (`nsh/base`'s choice), no sheet.
5. **The surface is inline, set by `foundation::popup::surface`**; the app changes it with tokens or `style`, not
   `class`.
6. **A `Button` trigger shows `expanded`** with its hover fill (button.md "States").
