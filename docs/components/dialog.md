# Dialog

Status: spec (P2), not implemented. A modal window over the app: a panel in the middle of the window over a dimmed
overlay, keeping the keyboard focus and screen readers inside it until it closes. Reference: shadcn/ui new-york v4
`dialog.tsx` (Radix Dialog).

This spec also settles what every overlay of the library shares (the layers, stacking, dismissal, focus, the theme
scope, shadows, radii and motion): see "Overlays: what every layer shares" under "Built on". `AlertDialog`,
`Popover` and `Toast` refer to it, and so do the menus (`DropdownMenu`, `Select`).

## Usage

- A form or a decision that interrupts the current task and needs an answer before the user goes on: edit a
  profile, rename a file, configure an export, a short wizard step.
- Search terms: dialog, modal, modal window, popup window, lightbox.

## When not to use

| Need | Use instead |
|---|---|
| Confirm a destructive or irreversible action ("Delete 3 files?") | `AlertDialog`: no close by the overlay, the focus starts on Cancel |
| The platform's own alert (an `NSAlert`, a `UIAlertController`), with no look of this library | `nsh/base`'s `AlertDialog.show({ … })` |
| Content next to what opened it, that does not block the page | `Popover` |
| A panel sliding from an edge, a bottom sheet with detents | `nsh/base`'s `Sheet` (this library's `Sheet` / `Drawer` come later) |
| A message that needs no answer ("Saved") | `toast()` |
| A full page on a phone (settings, a compose screen) | `nsh/base`'s `NavStack` page, or `nsh/base`'s `Dialog` with `presentation="pageSheet"` |

## Built on

The modal layer is `nsh/base`'s `Dialog` (Rust `nsh_base::dialog()`, the core's `Modal`): a native layer over the
whole window (a transparent layer over the window on the desktops, Android and the web; a presented view controller
with a clear background on iOS), above every page and the surface's overlays. The library uses it as `nsh/base`'s
`Sheet` does: with no platform animation and no native closing, so the look, the motion and the closing are the
library's, while the behavior below stays NSH's.

| Provided by NSH (not rewritten here) | |
|---|---|
| Layer | The modal's native layer, above the window's content and the overlays shown there; a newer modal above an older one |
| Focus trap | The newest modal is the focus scope: Tab and Shift+Tab stay inside it, Return and Space press only stops inside it, the window's own keys give way to it |
| Focus return | When the modal goes, the keyboard focus goes back to where it was when it opened (the trigger, usually), on every platform |
| Escape and back | Escape, the Android back button and VoiceOver's escape gesture reach the modal's `onCancel` |
| Screen readers | Role `dialog` (`alertdialog` with `role`), named by its `label`, described by `accessibilityDescribedBy`; what is under it is out of reach |
| Inside | Overlays inside it (a `Popover`, a `Select`'s list, a `Tooltip`) show in the modal's root view, above it |
| Terminal | A modal in the terminal's look; this library's look does not apply |

What this library adds: the open state (controlled or not) and its events, the trigger, the overlay and the panel
with their look, the motion, the close button, the closing on Escape and on the overlay, the title and description
wired to the modal's name and description, the layout parts.

### Structure

```
Dialog                        root: the open state, the trigger, the modal while open (draws no view of its own)
├─ trigger                    the app's element (a slot): pressing it opens the dialog
└─ modal (nsh/base Dialog)    made while open or closing; animation none, closeOnEscape false, closeOnSwipe false
   └─ DialogContent           the overlay and the panel
      ├─ overlay              inset 0, `dialog-overlay` fill; presses: a close attempt ("backdropPress")
      └─ layer                inset 0, centered, pointer-events box-none, bottom padded by the software keyboard
         └─ panel             the dialog's box: DialogHeader, content, DialogFooter, the close button
            └─ close          the "×" button at the top right (`showCloseButton`)
```

`DialogHeader`, `DialogTitle`, `DialogDescription`, `DialogFooter` and `DialogClose` are parts the app places in
`DialogContent`, in its order.

### Differs from shadcn's structure

- **No `DialogTrigger`: a `trigger` slot on `Dialog`**, as `nsh/base`'s `Popover` and `Sheet` take theirs and this
  library's `Popover` does. NSH has no `asChild`: a `DialogTrigger` wrapping the app's `Button` would be a second
  pressable around the first, which takes the press. The slot's element gets the press listener and the
  accessibility state (`hasPopup`, `expanded`) as `nsh/base`'s `Popover` gives its trigger. A dialog opened by the
  app (`open`) needs no trigger.
- **No `DialogPortal`, no `DialogOverlay` component**: the modal is the portal, and the overlay is a part of
  `DialogContent` (`overlayClass` / `overlayStyle`), as shadcn's `DialogContent` renders its `DialogOverlay`.
- **`DialogClose` is a `Button`** that closes the dialog (shadcn's footer `Close` is a `Button` inside a
  `DialogPrimitive.Close asChild`): it takes `Button`'s props, `variant="outline"` by default.

From shadcn, for an app written after its docs:

| shadcn | This library |
|---|---|
| `<Dialog><DialogTrigger asChild><Button>Open</Button></DialogTrigger><DialogContent>…</DialogContent></Dialog>` | `<Dialog trigger={<Button>Open</Button>}><DialogContent>…</DialogContent></Dialog>` |
| `<DialogClose asChild><Button variant="outline">Cancel</Button></DialogClose>` | `<DialogClose>Cancel</DialogClose>` (a `Button`, `outline` by default) |
| `<DialogContent showCloseButton={false}>`, `DialogHeader`, `DialogTitle`, `DialogDescription`, `DialogFooter` | The same |
| `DialogOverlay`, `DialogPortal` | `DialogContent`'s `overlayClass` / `overlayStyle`; no portal |
| `onOpenChange={(open) => …}` | `onOpenChange={({ open, reason }) => …}` (NSH's event shape) |

A child component cannot fill its parent's slot in NSH, so the trigger is a prop, not a `DialogTrigger` child.

**The trigger shows that it is open.** While the dialog is open the trigger has `accessibilityState.expanded` (as
`nsh/base`'s `Popover` and `Menu` set on theirs), which screen readers announce and NSH's `expanded` styles follow
(`aria-expanded:` in classes, `.expanded_style(..)` in Rust). The library's `Button` keeps its hover fill while
expanded (button.md, "States"), so a `Button` trigger stays lit while its dialog, popover or menu is open. An app's
own trigger styles `aria-expanded:` itself.

### Overlays: what every layer shares

Every overlay of the library (`Dialog`, `AlertDialog`, `Popover`, `Toast`, and the menus `DropdownMenu` and
`Select`, specified by their own group) follows these rules. `docs/conventions.md`'s "Overlays" section is the short
form; this is the full one.

**Two kinds of layers.**

| | Modal layer | Popup layer |
|---|---|---|
| Who | `Dialog`, `AlertDialog` (later `Sheet`, `Drawer`, `Command`'s dialog) | `Popover`, `DropdownMenu`, `Select`'s list, `Tooltip` (later `HoverCard`, `ContextMenu`, `Combobox`) |
| NSH element | `nsh/base`'s `Dialog` (the core's `Modal`), with the library's own motion and closing, as `nsh/base`'s `Sheet` | The core's overlay, through the `nsh/base` component (`Popover`, `Menu`, `Select`, `Tooltip`), whose presence (`popupStartingClass` / `popupEndingClass`) runs the motion |
| Blocks the page | Yes: the overlay dims it and takes its presses; focus and screen readers stay inside | No: Tab goes on past it (its content follows its trigger in the Tab order); on the desktops a press outside closes it and goes on to what is under the pointer |
| Surface | `dialog` fill (`background`), `border`, `radius-lg`, shadow 4 (`lg`) | `popover` fill, `border`, `radius-md`, shadow 3 (`md`); owned by `Popover`'s tokens (see [popover.md](popover.md)) |
| Motion | Overlay fades; panel fades and zooms from 95%; `dialog-duration` (200) | Fades and zooms from 95%, slides 8 from the trigger's side; `duration` (150) |

**Stacking**, bottom to top, each layer above the ones opened before it:

1. The page, and the popups opened on it (overlays of the surface's root view).
2. Each open modal, the newest on top (native layers).
3. The popups opened inside a modal (overlays of that modal's root view: a `Select` in a dialog shows above it).
4. Toasts: the `Toaster` draws its stack in the topmost open modal of this library, else on the page (see
   [toast.md](toast.md), "Built on"), so a toast is never under a dialog's overlay.

Within one root view NSH orders overlays by when they were shown (no z-index): a popup opened after the toasts
showed covers them. A tooltip shows above the popup it belongs to only if NSH puts it there: see "NSH gaps".

**Dismissal.** Each layer closes on its own, the topmost first:

- Escape and the Android back button go to the topmost layer listening (NSH routes them so): Escape in a `Select`
  inside a dialog closes the list, a second Escape the dialog. Each layer has `closeOnEscape` (default true) and
  reports every attempt in `onCancel`, closed or not.
- A press outside: a popup closes (`closeOnPressOutside`), on the desktops letting the press go on, on touch
  screens taking it (`nsh/base`'s transparent backdrop). A modal's overlay press closes it (`closeOnBackdropPress`,
  default true; false and not a prop in `AlertDialog`).
- The app closes any layer by its `open` prop at any time; its exit runs.
- Every request is reported once, as `onOpenChange({ open, reason })` with `nsh/base`'s `OpenChangeReason`
  (`"triggerPress"`, `"pressOutside"`, `"backdropPress"`, `"cancel"`, `"select"`, …), before the state changes; a
  controlled layer waits for the app's `open`. `onOpen` follows when it is all there (refs inside it name their
  views, so `focus()` there works), `onClose` when it is all gone, after its exit.

**Focus.** A modal traps the focus (NSH's newest-modal focus scope) and gives it back as it goes (NSH). A popup does
not trap it: `nsh/base` gives it back to the trigger as the popup starts to go, except after a press outside (which
put it where it pressed). The library moves the focus in only where shadcn's primitive does and NSH does not
already: `AlertDialog` focuses its `AlertDialogCancel` as it opens (Radix's default). Otherwise, the app focuses
what it wants in `onOpen` (an `Input`'s `autoFocus` also works).

**The theme scope.** An overlay's content is made by the component, under the component's place in the tree: its
reactive owner and context are those of where the app put the component, whatever native layer it is drawn in
(NSH's overlays and modals keep their parent's owner and context). So `theme()` in the content finds the
`ThemeScope` around the `Dialog` / `Popover` / trigger, and a dark scope gets a dark dialog. The `Toaster`'s stack,
drawn inside a modal, provides the `Toaster`'s own scope to its toasts (see [toast.md](toast.md)). Guarded by tests
(a scope around the trigger reaches the panel; a forced appearance reaches it).

**Text styles.** Overlays and modals keep their parent's inherited text styles (a popover in a bold label would be
bold). Each surface resets them as `Tooltip`'s bubble does (normal wrapping, regular weight, no underline, no case
change, no letter spacing) and sets its own size, line height and color.

**Shadows and radii.** Modal panels take shadow tier 4 (`lg`) and `radius-lg`; popup surfaces tier 3 (`md`) and
`radius-md`; a toast tier 3 and `radius-lg` (Sonner's `--border-radius: var(--radius)`). Each is its component's
token (`dialog-shadow`, `popover-shadow`, `toast-shadow`), drawn by `foundation`'s `shadow()` as an inline style.

**Motion.** Entrances and exits use the transitions `foundation::motion` already has (`transition_all`'s easing,
`cubic-bezier(0.4, 0, 0.2, 1)`); a popup's over `duration` (150, shadcn's `tw-animate-css` default), a modal's over
`dialog-duration` (200, shadcn's `duration-200`). With reduced motion every overlay shows and goes at once (the
conventions' rule; `nsh/base`'s presence skips the states). Each component's `<component>-animate` token (1 / 0)
turns its motion off, as `tooltip-animate` does.

**Touch screens.** A layer keeps its form unless the platform's habit differs and `nsh/base` already does it: menus
and `Select`'s list become bottom sheets (`nsh/base`); a `Popover` stays anchored (`nsh/base`); a `Dialog` stays a
centered panel (iOS and Android alerts and dialogs are centered; a bottom-sheet form is a different component, the
later `Drawer`); toasts span the width of a narrow window (Sonner's mobile layout).

## API

### Components

| Component | TSX | Rust | Description |
|---|---|---|---|
| `Dialog` | `<Dialog>` | `dialog()` | The root: open state, trigger, the modal while open |
| `DialogContent` | `<DialogContent>` | `dialog_content()` | The overlay and the panel; its children are the panel's content |
| `DialogHeader` | `<DialogHeader>` | `dialog_header()` | A column holding the title and the description |
| `DialogTitle` | `<DialogTitle>` | `dialog_title()` | The title: the dialog's accessible name; a `header` |
| `DialogDescription` | `<DialogDescription>` | `dialog_description()` | What the dialog is about: its accessible description |
| `DialogFooter` | `<DialogFooter>` | `dialog_footer()` | The row of buttons at the bottom |
| `DialogClose` | `<DialogClose>` | `dialog_close()` | A `Button` that closes the dialog |

`nsh/base` also exports a `Dialog` (the bare modal) and Rust's `nsh_base::dialog()`; an app using both renames one
(`import { Dialog as BaseDialog } from "nsh/base"`), as the conventions say.

### `Dialog` props

| TSX | Rust | Type | Default | Description |
|---|---|---|---|---|
| `open` | `open` | `boolean?` / `Prop<Option<bool>>` | uncontrolled | Open (controlled): the app owns it and changes it in `onOpenChange` |
| `defaultOpen` | `default_open` | `boolean` | `false` | Open at first, while `open` is not set |
| `trigger` | `trigger` | element (slot) | — | What opens it: pressing it requests `open: true` (`"triggerPress"`) |
| `closeOnEscape` | `close_on_escape` | `boolean` | `true` | Escape and back close it; either way `onCancel` hears them |
| `closeOnBackdropPress` | `close_on_backdrop_press` | `boolean` | `true` | A press on the overlay closes it; either way `onCancel` hears it |
| `accessibilityLabel` | `accessibility_label` | `string?` | the title's text | The dialog's name for screen readers, when there is no `DialogTitle` or it should say something else |
| `children` | `.child(..)` | elements | — | Usually one `DialogContent`; made on each opening (lazy) |

No `class` / `style`: the root draws nothing (shadcn's `Dialog` renders no element). The trigger is the app's
element and takes its own classes. Radix's `modal={false}` is not offered: a non-modal panel is a `Popover`.

### `DialogContent` props

| TSX | Rust | Type | Default | Description |
|---|---|---|---|---|
| `showCloseButton` | `show_close_button` | `boolean` | `true` | The "×" button at the top right |
| `class` / `style` | `class` / `style` | | | The panel, merged last |
| `overlayClass` / `overlayStyle` | `overlay_class` / `overlay_style` | | | The overlay behind the panel |
| `closeClass` / `closeStyle` | `close_class` / `close_style` | | | The "×" button |
| `children` | `.child(..)` | elements | — | The panel's content: header, body, footer |

`DialogContent` outside a `Dialog` renders its panel in place (no modal) and logs a warning in development builds,
as `CardAction` outside a `CardHeader`.

### Other parts' props

`DialogHeader`, `DialogTitle`, `DialogDescription` and `DialogFooter` take `children`, `class` and `style` (each part
is a component, so there are no `headerClass`-like props, as `Card`'s). String children of `DialogTitle` and
`DialogDescription` are wrapped in a text that takes their styles.

`DialogClose` takes `Button`'s props (`variant` defaulting to `"outline"`, `size`, `icon`, `disabled`, `loading`,
…) and `onPress`. A press requests `open: false` with `reason` `"select"` (the app's own button closed it), then
calls the app's `onPress`.

### Events (`Dialog`)

| TSX | Rust (`DialogEvent`) | Payload | When |
|---|---|---|---|
| `onOpenChange` | `OpenChange(open, reason)` | `{ open, reason }`: `"triggerPress"` (the trigger), `"cancel"` (Escape, back, the "×" button), `"backdropPress"` (the overlay), `"select"` (a `DialogClose`) | Every request of the user to open or close, before it applies; not the app's own `open` changes |
| `onOpen` | `Open` | — | It is all there, after its entrance started (the commit after its content was made: refs inside name their views) |
| `onClose` | `Close` | — | It is all gone, after its exit, whoever closed it |
| `onCancel` | `Cancel` | — | Each attempt to close it without choosing (Escape, back, the overlay, the "×" button), whether or not `closeOnEscape` / `closeOnBackdropPress` let it close |

The "×" button is reported as `"cancel"`, as Escape: both close without an answer. These are `nsh/base`'s popups'
names and reasons; no reason is added.

### Rust

```rust
dialog()
    .open(move || Some(open.get()))
    .on_open_change(move |e: &DialogEvent| if let DialogEvent::OpenChange(next, _) = e { open.set(*next) })
    .trigger(button().variant(ButtonVariant::Outline).child("Edit profile"))
    .child(dialog_content().child(/* … */))
```

The builder setters follow the props; `trigger` takes any element, as `nsh/base`'s `popover().trigger(..)`.

## Variants and sizes

- No `variant` (shadcn has one).
- No `size`: the panel is as wide as `dialog-max-width` (512, shadcn's `sm:max-w-lg`) and no wider than the window
  less `dialog-margin` on each side (shadcn's `max-w-[calc(100%-2rem)]`). A wider dialog sets `class="max-w-[640px]"`
  on `DialogContent`, as shadcn's examples do (`sm:max-w-[425px]`).

### Look

| Part | shadcn classes | This library |
|---|---|---|
| overlay | `fixed inset-0 z-50 bg-black/50` | `inset-0`, fill `dialog-overlay` (`overlay`, black at 50%) |
| layer | `fixed top-[50%] left-[50%] translate-…` | `inset-0`, centered both ways, `pointer-events: box-none`; bottom padding `env(keyboard-inset-height)` so the panel stays centered above a software keyboard |
| panel | `grid w-full max-w-[calc(100%-2rem)] gap-4 rounded-lg border bg-background p-6 shadow-lg sm:max-w-lg` | A column, gap `dialog-gap` (16), padding `dialog-padding` (24), width 100% of the layer up to `dialog-max-width` (512) with `dialog-margin` (16) kept on each side; corners `dialog-radius` (`radius-lg`, 10); border `border-width` × `dialog-border`; fill `dialog`; text `dialog-foreground`, 14 (`text-sm`, see Decisions 4); shadow tier `dialog-shadow` (4) |
| close | `absolute top-4 right-4 rounded-xs opacity-70 transition-opacity hover:opacity-100 focus:ring-2 focus:ring-ring focus:ring-offset-2 … size-4` | An `nsh/base` `Button` with the `x` icon at `icon-size-md` (16), absolutely placed `dialog-close-offset` (16) from the panel's top and right; opacity `dialog-close-opacity` (0.7), 1 under the pointer and while focused; corners `radius-xs` (2, Tailwind's `rounded-xs`); focus ring: the library's (`ring-width` 3, `ring` at `ring-opacity`, outside), not shadcn's 2 + offset; named "Close" (`labels`) |
| `DialogHeader` | `flex flex-col gap-2 text-center sm:text-left` | A column, gap `dialog-header-gap` (8); text centered while the window is narrower than 640 (Tailwind's `sm`), start-aligned from 640 |
| `DialogTitle` | `text-lg leading-none font-semibold` | Size `dialog-title-font-size` (18), line height 1 (18, shadcn's `leading-none`), `font-weight-semibold` |
| `DialogDescription` | `text-sm text-muted-foreground` | Size `dialog-description-font-size` (14), line height from foundation (20), color `dialog-description-foreground` (`muted-foreground`) |
| `DialogFooter` | `flex flex-col-reverse gap-2 sm:flex-row sm:justify-end` | Narrower than 640: a column in reverse order (the first button at the bottom, each as wide as the panel), gap `dialog-footer-gap` (8); from 640: a row at the end |

- `sm:` is the window's width, as NSH's Tailwind breakpoints are (NSH supports `sm:` in classes; the library reads
  the window's width in a binding with the same 640).
- The panel's content is not scrolled by the dialog (shadcn's isn't either): a tall form puts its body in a `Scroll`
  with a `max-h`. A panel taller than the window is cut at the window's edges.

### Motion

| | shadcn (tw-animate-css) | This library |
|---|---|---|
| Overlay | `fade-in-0` / `fade-out-0`, 150 ms | Opacity 0 → 1 / 1 → 0 over `dialog-duration` |
| Panel | `fade-in-0 zoom-in-95` / `fade-out-0 zoom-out-95`, `duration-200` | Opacity 0 → 1 and scale 0.95 → 1, back on exit, over `dialog-duration` (200), from the panel's center |
| Reduced motion | not handled | No motion: shown and gone at once |

One duration for both (shadcn's overlay takes the default 150 while its panel takes 200: the difference cannot be
seen and one value is simpler to tune). The values are animated values (the core's `animated_value`, bound with
`bind_opacity` / `bind_scale` as `nsh/base`'s `Sheet` binds its own), so the motion runs on the platform's animation
graph. Easing: `foundation::motion`'s curve. Closing: the modal stays mounted (taking no presses: the overlay and
the panel turn `pointer-events: none`) until the exit's animation ends, then goes, and `onClose` follows. Opened
again on its way out, it animates back from where it is, and neither `onClose` nor `onOpen` fires. `dialog-animate:
0` turns the motion off.

## States

| State | Look |
|---|---|
| Entering / exiting | See "Motion" |
| Close button hover | Opacity 1 (pointer only) |
| Close button focus-visible | Opacity 1 and the focus ring |
| Close button pressed | Opacity 1; no scale (it is an icon, not a button look) |

The panel has no hover, focus or disabled state. The trigger is `expanded` while the dialog is open: a library
`Button` keeps its hover fill then (button.md, "States"), and screen readers hear it expanded.

## Pointer and touch

| | Desktops and desktop browsers | Phones and tablets |
|---|---|---|
| Form | A centered panel, 512 wide at most | The same panel, as wide as the window less 16 on each side; header centered and footer stacked below 640 wide |
| Close | Escape, the overlay, the "×", the app's buttons | Back (Android), the overlay, the "×", the app's buttons |
| "×" hit area | Its 16 square | `max(0, (touch-target − 16) / 2)` on each side (44 square); the look unchanged |
| Software keyboard | — | The panel stays centered in the room above it (`env(keyboard-inset-height)`) |
| Swipe | — | None (a centered panel is not swiped away on either platform; `nsh/base`'s `Sheet` is) |

The buttons in the footer take `control-height-touch-*` on touch screens by themselves.

## Accessibility

- The modal is a `dialog` for screen readers, named by `accessibilityLabel`, else by the text of its `DialogTitle`
  (read with `ElementRef::text_content` once the content is made, and again when the title's text changes), and
  described by its `DialogDescription` (`accessibilityDescribedBy` on the modal, the description's element). Radix
  links them with `aria-labelledby` / `aria-describedby`; see NSH gaps 1.
- `DialogTitle` has the `header` role (as `CardTitle`).
- A dialog without a title and without `accessibilityLabel` logs a warning in development builds (Radix warns too).
- The "×" button is a `button` named "Close" (`labels`: a new `Label::Close`, `"close"`).
- The focus is trapped and returned by NSH (see "Built on"); it is not moved in as the dialog opens (see NSH gaps
  2): the first Tab reaches the first stop inside.
- The overlay is hidden from screen readers (`accessible(false)`), as shadcn's overlay is a plain `div`.

## Tokens

Proposed; once implemented, the block is generated from `tokens!` (`<!-- tokens:dialog -->`) as in the other specs.

| Token | Type | Derived default | Value |
|---|---|---|---|
| `dialog` | color | `background` |  |
| `dialog-foreground` | color | `foreground` |  |
| `dialog-border` | color | `border` |  |
| `dialog-overlay` | color | `overlay` |  |
| `dialog-radius` | length | `radius-lg` | 10 |
| `dialog-shadow` | number | `4` | 4 |
| `dialog-padding` | length | `spacing × 6` | 24 |
| `dialog-gap` | length | `spacing × 4` | 16 |
| `dialog-margin` | length | `spacing × 4` | 16 |
| `dialog-max-width` | length | `spacing × 128` | 512 |
| `dialog-header-gap` | length | `spacing × 2` | 8 |
| `dialog-footer-gap` | length | `spacing × 2` | 8 |
| `dialog-title-font-size` | length | `18` | 18 |
| `dialog-description-font-size` | length | `14` | 14 |
| `dialog-description-foreground` | color | `muted-foreground` |  |
| `dialog-close-offset` | length | `spacing × 4` | 16 |
| `dialog-close-opacity` | number | `0.7` | 0.7 |
| `dialog-duration` | number | `duration × 4/3` | 200 |
| `dialog-animate` | number | `1` | 1 |

- `dialog` is not a semantic token, so the panel's fill is the component's (`background`, shadcn's `bg-background`)
  and the text on it `dialog-foreground`. A dialog the color of cards sets `"dialog": "var(--card)"`.
- `dialog-overlay` is the semantic `overlay` (black at 50%, shadcn's `bg-black/50`) unless a theme changes it for
  dialogs alone. `AlertDialog` uses the same tokens (it is drawn by the same parts).
- `dialog-title-font-size` and `dialog-description-font-size` are Tailwind's fixed sizes (`text-lg`, `text-sm`): a
  title and body text, not a control's (tokens.md).
- `dialog-duration` is a number of milliseconds derived from the `duration` seed (4/3 of 150), so a theme that
  slows everything slows dialogs too.
- Other tokens it reads: `border-width`, `radius-xs` (the semantic `max(radius − 8, 0)`, 2, Tailwind's
  `rounded-xs`; added with this component, see Decisions 5), `font-weight-semibold`, `icon-size-md`, `ring-width`,
  `ring`, `ring-opacity`, `touch-target`, `shadow`.

## Examples

TSX:

```tsx
import { signal, View } from "nsh";
import { Button, Dialog, DialogClose, DialogContent, DialogDescription, DialogFooter, DialogHeader, DialogTitle, Input, type InputRef, Label } from "nsh-longbridge";

const name = signal("Pedro Duarte");
const nameField = signal<InputRef>(); // read by the Label above it: kept in a signal

// Uncontrolled, opened by its trigger.
<Dialog trigger={<Button variant="outline">Edit profile</Button>}>
  <DialogContent class="max-w-[425px]">
    <DialogHeader>
      <DialogTitle>Edit profile</DialogTitle>
      <DialogDescription>Make changes to your profile here. Click save when you're done.</DialogDescription>
    </DialogHeader>
    <View class="gap-3">
      <Label htmlFor={nameField()}>Name</Label>
      <Input ref={nameField} value={name()} onChange={({ value }) => name.set(value)} />
    </View>
    <DialogFooter>
      <DialogClose>Cancel</DialogClose>
      <Button onPress={save}>Save changes</Button>
    </DialogFooter>
  </DialogContent>
</Dialog>

// Controlled, opened by the app, the first field focused once it is there.
const open = signal(false);
<Dialog open={open()} onOpenChange={({ open: next }) => open.set(next)} onOpen={() => nameField()?.focus()}>
  <DialogContent showCloseButton={false}>…</DialogContent>
</Dialog>
```

With NSH 0.7.0, write a string child inside `<Text>` for now (see [button.md, NSH gaps](button.md#nsh-gaps), G0).

Rust:

```rust
use nsh_core::prelude::*;
use nsh_longbridge::{
    ButtonVariant, DialogEvent, button, dialog, dialog_close, dialog_content, dialog_description, dialog_footer,
    dialog_header, dialog_title,
};

let open = signal(false);

dialog()
    .open(move || Some(open.get()))
    .on_open_change(move |e: &DialogEvent| {
        if let DialogEvent::OpenChange(next, _reason) = e {
            open.set(*next);
        }
    })
    .trigger(button().variant(ButtonVariant::Outline).child("Edit profile"))
    .child(
        dialog_content()
            .child(
                dialog_header()
                    .child(dialog_title().child("Edit profile"))
                    .child(dialog_description().child("Make changes to your profile here.")),
            )
            .child(/* fields */)
            .child(
                dialog_footer()
                    .child(dialog_close().child("Cancel"))
                    .child(button().on_press(move || save()).child("Save changes")),
            ),
    )
```

## NSH gaps

The ★ items are the first headless tests of the implementation; each has its fallback.

1. **No `accessibilityLabelledBy`.** A modal's name is a string (`label`); Radix names it by its title element.
   Workaround: the library reads the title's text (`ElementRef::text_content`) once the content is made and when the
   title changes, and sets the modal's `label`; `accessibilityLabel` wins. The description uses
   `accessibilityDescribedBy`, which NSH has. A `labelledBy` would remove the read-back.
2. **No "focus the first stop inside" (accepted).** Radix moves the focus to the first focusable element inside the
   content as the dialog opens; this library's `Dialog` does not: NSH's modal leaves the focus on its container, and
   NSH has no call to focus a subtree's first stop. So with a keyboard the first Tab enters the dialog, and a screen
   reader starts at the dialog itself. The app focuses a ref in `onOpen`, or gives an `Input` `autoFocus`;
   `AlertDialog` focuses its `AlertDialogCancel` through context (alert-dialog.md). A core call
   (`ElementRef::focus_first()`) would let `Dialog` do Radix's default; revisit then (Decisions 3).
3. **`nsh/base`'s popup machinery is private.** `OpenState`, `lifecycle`, `PopupPresence`, `keyboard_lift` and
   `focus_back` (hidden) live in `nsh-base`'s private `popup` module. The library re-implements a small open state
   (controlled or not, `request` with the reason), the presence (mounted while open or exiting) and its lifecycle
   events, following `popup.rs`'s rules, and pads by `env(keyboard-inset-height)` instead of `keyboard_lift`. Exposing
   them (or a styled-modal building block) would remove the copy.
4. ★ **Escape order with a popup inside the dialog.** NSH's overlay docs say a dialog's native Escape
   handling comes before the overlays inside it. The library's modal closes nothing natively (`closeOnEscape(false)`)
   but its `onCancel` may still hear Escape before an open `Select` list or `Popover` inside it, closing the dialog
   instead of the list. The first headless test opens a `Popover` in a `Dialog` and presses Escape. If the dialog
   hears it first, the workaround: the library's popups register in the nearest `Dialog`'s context while open, and the
   dialog ignores a cancel while any is open; NSH routing Escape to the topmost overlay first would be the real fix.
5. ★ **Does the modal keep the context of where the `Dialog` is placed?** NSH documents it for overlays (owner and
   context stay the parent's), not in so many words for modals. First headless test: a `ThemeScope` with a forced
   dark appearance around a `Dialog`; the panel must read the scope's tokens. Fallback: `Dialog` captures `theme()`'s
   scope (and the other contexts the library reads: `Size`, the "in a dialog" registry) in its body and provides it
   again inside the modal, before `DialogContent` is made.
6. **Inherited text styles.** The modal's content inherits the text styles where the `Dialog` is placed (as the
   tooltip's hint layer does, Tooltip's gap 7); the panel resets them (see "Text styles" above). Italics and
   alignment have no reset setter and still pass through.

## Decisions

Settled in review.

1. **One centered panel on every device**, not a bottom sheet on phones: shadcn's Dialog, iOS's and Android's own
   dialogs and alerts are centered; the bottom-sheet form is a separate component (later `Drawer`, on `nsh/base`'s
   `Sheet`), and choosing between them per device is the app's (shadcn's "responsive dialog" pattern).
2. **A `trigger` slot instead of `DialogTrigger`**, and **`DialogClose` as a `Button`**: NSH has no `asChild`
   (see "Differs from shadcn's structure").
3. **No initial focus move** in `Dialog` until NSH can focus a subtree's first stop (gap 2); to revisit then.
4. **The panel's base text is 14 (`text-sm`)**: shadcn's panel inherits the page's (16 in a browser), but its
   parts are `text-sm`; NSH's inherited text would be the trigger's surroundings, so the panel sets one size. The
   app's content may set its own.
5. **`radius-xs` for the close button** (2): a new semantic token `radius-xs` = `max(radius − 8, 0)` (tokens.md), for
   which `NumExpr` gets `max` / `min`; both are added with this component.
6. **`"select"` for `DialogClose`, `"cancel"` for "×"**: no reason is added to `nsh/base`'s set.
7. **One `dialog-duration` (200) for the overlay and the panel.**
8. **A `Button` trigger shows `expanded`** with its hover fill (button.md "States").
