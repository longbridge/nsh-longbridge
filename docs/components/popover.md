# Popover

Status: implemented (P2). Rich content in a floating panel next to the element that opened it: a press on
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

- `#[component] fn Popover` whose body is `nsh_base::popover()` with the props passed through and the trigger slot
  forwarded to it. The popup (the box the core places) carries the motion: its starting and ending looks
  (`popupStartingClass` / `popupEndingClass`, set by the library from the side and `popover-animate`), its transitions
  and its scale origin. Inside it, the library draws a **panel** of its own: the surface
  (`foundation::popup::surface(el, t)`: fill, edge, corners, shadow, the text-style reset), the width and the padding,
  then the app's `class` and `style`, merged last as on any root.
- The panel, not `nsh/base`'s `popupStyle`, because a part style of `nsh/base` takes a style built once in Rust (Checkbox's
  NSH gap 3): it cannot take the app's `style` passed through. With the panel, the app's `class` and `style` both
  follow the library's look and override it (`class="w-80 p-0"`, `style={{ width: 320 }}`), as on every component.
- The panel is made on each opening, so its look reads the tokens and the scope then; its bindings keep following
  them while it is open.
- The popup's content is made by `nsh/base`'s `Popover`, inside this component: it reads the `ThemeScope` around the
  `Popover` (dialog.md, "The theme scope"; a test guards it). The panel resets inherited text styles (the overlay keeps
  its parent's).
- The library keeps the side the popup went on in a signal, set from `onPosition` (and passes the event on).
- While the panel is there it counts among the surface's open popups (`foundation::popup::hold_open`), which keeps a
  library `Tooltip` outside every open popup hidden (NSH gaps 1).

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
| `class` / `style` | `class` / `style` | | | The panel (the popup's box), merged after its own look (see "Implementation") |
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

The width is fixed, as shadcn's `w-72`; a content-sized popover sets `class="w-auto"`, a wider one `class="w-80"`,
`style={{ width: 360 }}` or `popover-width` in a scope.
The core lays an anchored popup out inside the safe area less the margin, so on a narrow screen it is never wider
than the screen.

### The popup surface (shared)

The look every popup of the library has; `Popover` owns its tokens, and `foundation` gets the function the popups
call (`foundation::popup::surface(el, t)`, on a view or on a part's style: fill, edge, corners, shadow, the
transitions and the text-style reset, as `foundation::field_look` is the fields'; `DropdownMenu`, `Select`'s list and
later `Combobox` call it), so a theme that changes `popover-radius` changes popovers, menus and `Select`'s list
together. The surface's tokens are declared in `foundation::popup` (as the field look's in `foundation::field`);
`foundation::popup` also has the entrance and exit looks by side (`starting_class`, `ending_class`, `origin`) and the
count of open popups (`hold_open`).

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
right side. `onPosition` comes before the first display frame (a test checks a flipped popup's first frame), so the
slide is kept.

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

Generated from the code's declarations (`NSH_LONGBRIDGE_WRITE=1 cargo test --test tokens`); a test keeps it current.
Values are those of `nsh/base`'s default theme. The first six are the popup surface's (`foundation::popup`), which
the menus read too; the rest are `Popover`'s own.

<!-- tokens:popover -->
| Token | Type | Derived default | Value |
|---|---|---|---|
| `popover-border` | color | `border` |  |
| `popover-radius` | length | `radius-md` | 8 |
| `popover-shadow` | number | `3` | 3 |
| `popover-offset` | length | `spacing` | 4 |
| `popover-margin` | length | `spacing` | 4 |
| `popover-animate` | number | `1` | 1 |
| `popover-width` | length | `spacing × 72` | 288 |
| `popover-padding` | length | `spacing × 4` | 16 |
| `popover-header-gap` | length | `spacing` | 4 |
| `popover-description-foreground` | color | `muted-foreground` |  |
<!-- /tokens -->
- `popover` and `popover-foreground` are shadcn's semantic tokens, used directly (as `card` by `Card`).
- `popover-margin` 4 is `nsh/base`'s `Tooltip`'s default margin, kept the same for every popup (Radix's
  `collisionPadding` is 0; a popup touching the window's edge looks cut).
- `popover-animate` is a switch: at 0 the library passes no starting or ending look, and the popup shows and goes in
  one frame.
- Other tokens it reads: `border-width`, `text-sm`, `font-weight-medium`, `duration`, `shadow`.

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

1. **A trigger's tooltip above its open popover** (Tooltip's NSH gap 4; reproduced on NSH 0.7.0). A button with a
   tooltip, focused from the keyboard, opens a popover: the tooltip stays shown over it (Escape still reaches the
   popover first). The library's fallback is in place: its popups count themselves open in a surface global and
   provide an "inside an open popup" context to their content (`foundation::popup::hold_open`); while one is open, a
   library `Tooltip` not inside an open popup is `disabled`. A test guards it. A tooltip of the app's own (not the
   library's) is not covered; the NSH fix would be a hint not shown while a popup anchored to its element is open.
2. **No role on the popover's content.** Radix's popover content is a `dialog` for screen readers; `nsh/base`'s is a
   plain overlay. Noted; the library adds none (a non-modal `dialog` role would make screen readers treat it as
   modal on some platforms).
3. **No separate anchor.** `nsh/base`'s `Popover` places the popup against its trigger only (shadcn's
   `PopoverAnchor` places it against another element). An `anchor` prop on `nsh/base`'s `Popover` (the core's
   overlay already takes any ref) would allow it.
4. **`onPosition` timing** decides the slide after a flip (see "Motion"): verified on NSH 0.7.0, it comes before the
   first display frame, so a flipped popup slides from the right side; a test guards it.
5. **Part styles built once.** `nsh/base`'s `popupStyle` takes a style built once in Rust, so it cannot take the
   app's `style` passed through: the library draws the surface on a panel of its own inside the popup (see
   "Implementation"). A popup drawn on `popupStyle` (a menu's) reads its surface as it opens.
6. **The shadow on `nsh/base`'s popup**: verified headless on NSH 0.7.0, the surface's shadow reaches the panel and
   nothing around it clips (a test guards it). Whether each platform draws it is for the web and macOS screenshots.

## Decisions

Settled in review.

1. **The API follows `nsh/base`'s `trigger` slot**, not shadcn's `PopoverTrigger` / `PopoverContent`; the panel is
   styled by `Popover`'s `class` / `style`.
2. **`Popover` owns the shared popup-surface tokens** (`popover-border`, `-radius`, `-shadow`, `-offset`,
   `-animate`), which the menus' group reads.
3. **The popup's text is 14 / 20**: shadcn's inherits the page's, NSH's would inherit the trigger's (a button's
   medium weight); the panel resets it to `text-sm`, the size of every shadcn part inside it.
4. **Anchored on touch screens too** (`nsh/base`'s choice), no sheet.
5. **The surface is the library's panel inside the popup**, drawn by `foundation::popup::surface`; the app changes
   it with tokens, `class` or `style` (changed in implementation: `nsh/base`'s `popupStyle` cannot take the app's
   style from Rust).
6. **A `Button` trigger shows `expanded`** with its hover fill (button.md "States").
