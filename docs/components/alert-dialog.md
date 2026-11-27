# AlertDialog

Status: implemented (P2). A modal dialog that interrupts the user with an important question and expects an
answer: a title, a sentence, and a Cancel and an action button. Reference: shadcn/ui new-york v4 `alert-dialog.tsx`
(Radix AlertDialog).

It is a [`Dialog`](dialog.md) with an alert's rules: the same modal layer, overlay, panel, motion and tokens, and the
shared overlay rules settled there ("Overlays: what every layer shares").

## Usage

- Confirming something destructive or irreversible: delete a file, sign out of every device, discard a draft.
- Search terms: alert dialog, confirm dialog, confirmation, "are you sure".

## When not to use

| Need | Use instead |
|---|---|
| A form, or a dialog the user may dismiss by pressing outside | `Dialog` |
| The platform's own alert (`NSAlert`, `UIAlertController`, Android's `AlertDialog`) with its look and placement | `nsh/base`'s `AlertDialog.show({ title, description, actions })` |
| An undoable action: delete and offer "Undo" | Do it, then `toast("Deleted", { action: { id: "undo", label: "Undo" } })` |
| A warning that needs no answer | A notice in the page (the later `Alert`) or a `toast()` |

`nsh/base` exports a module named `AlertDialog` (the system's alert). This library's `AlertDialog` is a component;
an app using both renames one (`import { AlertDialog as SystemAlert } from "nsh/base"`).

## Built on

The library's `Dialog` parts, which sit on `nsh/base`'s `Dialog` (the core's modal: focus trap and return, Escape
and back, screen readers; see dialog.md "Built on"). What differs from `Dialog`:

| | `Dialog` | `AlertDialog` (Radix's rules) |
|---|---|---|
| Role | `dialog` | `alertdialog` (the modal's `role`) |
| Press on the overlay | Closes (`closeOnBackdropPress`) | Never closes; still reported to `onCancel`. Not a prop |
| Escape and back | Close (`closeOnEscape`) | Close (`closeOnEscape`, default true), reported as `"cancel"` |
| Close button "×" | `showCloseButton` (default true) | None |
| Focus as it opens | Not moved (dialog.md, NSH gaps 2) | On the `AlertDialogCancel` (Radix focuses Cancel), else on the first `AlertDialogAction` |
| Buttons | `DialogClose` | `AlertDialogCancel` (outline) and `AlertDialogAction` (default), both `Button`s that close it |

Focusing Cancel uses context: `AlertDialogCancel` and `AlertDialogAction` register their element refs with the
nearest `AlertDialog`, which calls `focus()` on Cancel (else the first action) in its `onOpen` step, before the
app's `onOpen` (a test checks that Return right after opening presses Cancel, else the action). On touch screens the focus moves too (screen readers start there), with no visible ring (the
conventions' touch rule).

### Structure

```
AlertDialog                    root: open state, trigger, the modal (role alertdialog)
└─ AlertDialogContent          overlay + panel (the Dialog's), size, no close button
   ├─ AlertDialogHeader        grid: [media] title description
   │  ├─ AlertDialogMedia      optional icon tile
   │  ├─ AlertDialogTitle
   │  └─ AlertDialogDescription
   └─ AlertDialogFooter        Cancel and the action
      ├─ AlertDialogCancel
      └─ AlertDialogAction
```

As with `Dialog`, there is no `AlertDialogTrigger` (a `trigger` slot: a child component cannot fill its parent's
slot in NSH), no `AlertDialogPortal` and no `AlertDialogOverlay` component (`overlayClass` / `overlayStyle` on the
content). From shadcn:

| shadcn | This library |
|---|---|
| `<AlertDialog><AlertDialogTrigger asChild><Button>Delete</Button></AlertDialogTrigger><AlertDialogContent>…` | `<AlertDialog trigger={<Button>Delete</Button>}><AlertDialogContent>…` |
| `<AlertDialogContent size="sm">` | `<AlertDialogContent size="sm">` (`Size`: see "Variants and sizes") |
| `<AlertDialogAction variant="destructive" onClick={…}>` | `<AlertDialogAction variant="destructive" onPress={…}>` |
| `AlertDialogCancel`, `AlertDialogHeader`, `AlertDialogMedia`, `AlertDialogTitle`, `AlertDialogDescription`, `AlertDialogFooter` | The same |
| `onOpenChange={(open) => …}` | `onOpenChange={({ open, reason }) => …}` |

While open, the trigger is `expanded`; a library `Button` keeps its hover fill then (button.md "States").

### Implementation

`AlertDialog` is `Dialog`'s root with the role and the overlay rule fixed; `AlertDialogContent` is
`DialogContent` with `showCloseButton` false, the `size` and the narrower width; the other parts are their own
components because their layout differs from `Dialog`'s (grid header, media, two-column footer at `sm`). They read
the same `dialog-*` tokens, plus the few of their own below. `AlertDialogCancel` takes `DialogClose`'s props
(`outline` by default) and `AlertDialogAction` `Button`'s; both draw the library's `Button` with a press of their own.

## API

### Components

| Component | TSX | Rust | Description |
|---|---|---|---|
| `AlertDialog` | `<AlertDialog>` | `alert_dialog()` | The root |
| `AlertDialogContent` | `<AlertDialogContent>` | `alert_dialog_content()` | Overlay and panel |
| `AlertDialogHeader` | `<AlertDialogHeader>` | `alert_dialog_header()` | Media, title, description |
| `AlertDialogMedia` | `<AlertDialogMedia>` | `alert_dialog_media()` | An icon tile beside or above the title |
| `AlertDialogTitle` | `<AlertDialogTitle>` | `alert_dialog_title()` | The question: the dialog's name; a `header` |
| `AlertDialogDescription` | `<AlertDialogDescription>` | `alert_dialog_description()` | The consequences: the dialog's description |
| `AlertDialogFooter` | `<AlertDialogFooter>` | `alert_dialog_footer()` | The buttons |
| `AlertDialogCancel` | `<AlertDialogCancel>` | `alert_dialog_cancel()` | A `Button` (`outline`) that closes it without acting |
| `AlertDialogAction` | `<AlertDialogAction>` | `alert_dialog_action()` | A `Button` (`default`) that closes it; the app acts in `onPress` |

`alert_dialog()` in Rust is this library's component; `nsh_base::alert_dialog()` is the system alert's builder.

### `AlertDialog` props

| TSX | Rust | Type | Default | Description |
|---|---|---|---|---|
| `open` / `defaultOpen` | `open` / `default_open` | `boolean?` / `boolean` | uncontrolled / `false` | As `Dialog` |
| `trigger` | `trigger` | element (slot) | — | As `Dialog` |
| `closeOnEscape` | `close_on_escape` | `boolean` | `true` | Escape and back close it (Radix's alert dialog closes on Escape); either way `onCancel` hears them |
| `accessibilityLabel` | `accessibility_label` | `string?` | the title's text | As `Dialog` |
| `children` | `.child(..)` | elements | — | Usually one `AlertDialogContent`; made on each opening |

No `closeOnBackdropPress`: an alert dialog is answered with a button (Radix prevents the outside press).

### `AlertDialogContent` props

| TSX | Rust | Type | Default | Description |
|---|---|---|---|---|
| `size` | `size` | `Size` | `"md"` | `"sm"` (and `"xs"`): shadcn's `size="sm"`, a narrow, centered alert with the buttons side by side; `"md"` (and `"lg"`): shadcn's `default` |
| `class` / `style` | `class` / `style` | | | The panel |
| `overlayClass` / `overlayStyle` | `overlay_class` / `overlay_style` | | | The overlay |
| `children` | `.child(..)` | elements | — | Header and footer |

### Other parts

- `AlertDialogHeader`, `AlertDialogMedia`, `AlertDialogTitle`, `AlertDialogDescription`, `AlertDialogFooter`:
  `children`, `class`, `style`. `AlertDialogMedia`'s child is usually an `Icon`, which takes the media's icon size
  from context (`foundation::icon::provide_icon_side`; an `Icon` with its own `size` keeps it, and a control inside
  the media gives its icons its own size).
- `AlertDialogCancel`: `Button`'s props, `variant` defaulting to `"outline"`; a press requests `open: false` with
  `reason` `"cancel"`, then calls `onPress`.
- `AlertDialogAction`: `Button`'s props, `variant` defaulting to `"default"` (`"destructive"` for a delete); a press
  requests `open: false` with `reason` `"select"`, then calls `onPress`. An action that must finish before the dialog
  closes (a request to the server) uses a controlled `open`: set `loading` on the action, keep `open` true in
  `onOpenChange`, and close when the work ends.

### Events (`AlertDialog`)

As `Dialog`'s: `onOpenChange({ open, reason })` (`"triggerPress"`, `"cancel"` for Escape, back and
`AlertDialogCancel`, `"select"` for `AlertDialogAction`), `onOpen`, `onClose`, `onCancel` (Escape, back, and a press
on the overlay, which does not close it). Rust: `AlertDialogEvent`, the same variants.

## Variants and sizes

No `variant`; the action's look is the `Button`'s `variant`. `size` on `AlertDialogContent` is `foundation`'s
`Size`, whose four values draw shadcn's two sizes, two by two:

| `size` | Draws shadcn's | Panel width | Header | Footer |
|---|---|---|---|---|
| `xs` | `sm` | up to 320 | centered | two equal columns |
| `sm` | `sm` | up to 320 | centered | two equal columns |
| `md` (default) | `default` | up to 512 | centered below 640, start-aligned from 640 | reversed column below 640, a row at the end from 640 |
| `lg` | `default` | up to 512 | as `md` | as `md` |

`xs` looks the same as `sm`, and `lg` the same as `md`.

### Look

Panel, overlay and motion: `Dialog`'s (dialog.md "Look" and "Motion"), except:

| Part | shadcn classes | This library |
|---|---|---|
| panel, `md` | `max-w-[calc(100%-2rem)] … sm:max-w-lg` | As `Dialog`: up to `dialog-max-width` (512) |
| panel, `sm` | `data-[size=sm]:max-w-xs` | Up to `alert-dialog-max-width-sm` (320) at every window width |
| `AlertDialogHeader` | `grid grid-rows-[auto_1fr] place-items-center gap-1.5 text-center`, from `sm` (size default) `place-items-start text-left`; with media `grid-rows-[auto_auto_1fr] gap-x-6`, from `sm` the media spans two rows on the left | A grid of one column, row gap `alert-dialog-header-gap` (6). Centered (text, and the media by its auto margins) while the window is narrower than 640 or the size is `sm`; otherwise start-aligned. With an `AlertDialogMedia`: centered, the media is a row above the title; start-aligned, the columns are `auto minmax(0, 1fr)` and the rows `auto minmax(0, 1fr)` (see NSH gaps 2), the media spans both rows on the left and the title and description start at column 2, `spacing × 6` (24) from it |
| `AlertDialogMedia` | `mb-2 inline-flex size-16 items-center justify-center rounded-md bg-muted *:[svg]:size-8` | A square `alert-dialog-media-size` (64), content centered, corners `radius-md`, fill `alert-dialog-media` (`muted`); bottom margin 8 while centered above the title; icons `alert-dialog-media-icon-size` (32) |
| `AlertDialogTitle` | `text-lg font-semibold` | Size `dialog-title-font-size` (18), line height from foundation (28: no `leading-none`, unlike `DialogTitle`), `font-weight-semibold` |
| `AlertDialogDescription` | `text-sm text-muted-foreground` | As `DialogDescription` |
| `AlertDialogFooter` | `flex flex-col-reverse gap-2 sm:flex-row sm:justify-end`; size `sm`: `grid grid-cols-2` | `md`: as `DialogFooter` (a reversed column below 640, a row at the end from 640). `sm`: two equal columns at every width (Cancel left, action right), gap 8 |

- "Has media" (shadcn's `has-data-[slot=alert-dialog-media]:`) is context, not a selector (NSH has no `:has()`): the
  media registers with the header on mount and clears on cleanup, as `CardAction` does with `CardHeader`.
- "Size `sm`" (shadcn's `group-data-[size=sm]/alert-dialog-content:`) is context the content provides to the header
  and the footer.

## States

As `Dialog` (entering, exiting). The buttons have `Button`'s states. No close button.

## Pointer and touch

As `Dialog`. On a phone (narrower than 640) a `md` alert stacks its buttons full width, the action on top (shadcn's
`flex-col-reverse`), as iOS stacks more than two alert actions and Android's Material dialogs place the confirming
action nearest the thumb; a `sm` alert keeps them side by side (iOS's two-button alert).

## Accessibility

- Role `alertdialog`, named by its title (or `accessibilityLabel`), described by its description (dialog.md
  "Accessibility" and its NSH gap 1).
- The focus starts on `AlertDialogCancel`, so Return or Space right away does the safe thing (Radix). It stays
  trapped and goes back to the trigger as it closes (NSH).
- `AlertDialogMedia` is decorative: hidden from screen readers.
- Without `AlertDialogCancel` and without `AlertDialogAction`, nothing closes it but Escape and the app: a warning in
  development builds.

## Tokens

It reads every `dialog-*` token (dialog.md "Tokens"). Its own, generated from the code's declarations
(`NSH_LONGBRIDGE_WRITE=1 cargo test --test tokens`; a test keeps it current), with `nsh/base`'s default theme:

<!-- tokens:alert-dialog -->
| Token | Type | Derived default | Value |
|---|---|---|---|
| `alert-dialog-max-width-sm` | length | `spacing × 80` | 320 |
| `alert-dialog-header-gap` | length | `spacing × 1.5` | 6 |
| `alert-dialog-media` | color | `muted` |  |
| `alert-dialog-media-size` | length | `spacing × 16` | 64 |
| `alert-dialog-media-icon-size` | length | `spacing × 8` | 32 |
<!-- /tokens -->
- The panel, overlay, radius, shadow, padding, gaps, title and description of an alert dialog follow the `dialog-*`
  tokens: a theme that rounds dialogs rounds alerts too. A scope can still change them for alerts alone.
- The media's corners are `radius-md` (shadcn's `rounded-md`), not a token of its own.

## Examples

TSX:

```tsx
import { AlertDialog, AlertDialogAction, AlertDialogCancel, AlertDialogContent, AlertDialogDescription, AlertDialogFooter, AlertDialogHeader, AlertDialogMedia, AlertDialogTitle, Button, Icon } from "nsh-longbridge";

<AlertDialog trigger={<Button variant="outline">Show dialog</Button>}>
  <AlertDialogContent>
    <AlertDialogHeader>
      <AlertDialogTitle>Are you absolutely sure?</AlertDialogTitle>
      <AlertDialogDescription>
        This action cannot be undone. This will permanently delete your account and remove your data from our servers.
      </AlertDialogDescription>
    </AlertDialogHeader>
    <AlertDialogFooter>
      <AlertDialogCancel>Cancel</AlertDialogCancel>
      <AlertDialogAction onPress={deleteAccount}>Continue</AlertDialogAction>
    </AlertDialogFooter>
  </AlertDialogContent>
</AlertDialog>

// Small, with media, destructive.
<AlertDialog open={confirming()} onOpenChange={({ open }) => confirming.set(open)}>
  <AlertDialogContent size="sm">
    <AlertDialogHeader>
      <AlertDialogMedia><Icon name="trash-2" /></AlertDialogMedia>
      <AlertDialogTitle>Delete 3 files?</AlertDialogTitle>
      <AlertDialogDescription>They will be removed for everyone.</AlertDialogDescription>
    </AlertDialogHeader>
    <AlertDialogFooter>
      <AlertDialogCancel>Cancel</AlertDialogCancel>
      <AlertDialogAction variant="destructive" onPress={remove}>Delete</AlertDialogAction>
    </AlertDialogFooter>
  </AlertDialogContent>
</AlertDialog>
```

With NSH 0.7.0, write a string child inside `<Text>` for now (see [button.md, NSH gaps](button.md#nsh-gaps), G0).

Rust:

```rust
use nsh_longbridge::{
    ButtonVariant, alert_dialog, alert_dialog_action, alert_dialog_cancel, alert_dialog_content,
    alert_dialog_description, alert_dialog_footer, alert_dialog_header, alert_dialog_title, button,
};

alert_dialog()
    .trigger(button().variant(ButtonVariant::Outline).child("Show dialog"))
    .child(
        alert_dialog_content()
            .child(
                alert_dialog_header()
                    .child(alert_dialog_title().child("Are you absolutely sure?"))
                    .child(alert_dialog_description().child("This action cannot be undone.")),
            )
            .child(
                alert_dialog_footer()
                    .child(alert_dialog_cancel().child("Cancel"))
                    .child(alert_dialog_action().on_press(move || delete_account()).child("Continue")),
            ),
    )
```

## NSH gaps

Those of `Dialog` (dialog.md "NSH gaps", and focus not moved in a `Dialog`, which `AlertDialog` makes up for by
focusing Cancel through context), and:

1. **No `:has()`**: the header's layout with media is context (see "Look"), as `CardHeader`'s action.
2. **An `auto` grid row next to a `1fr` one** (found on NSH 0.7.0): with `grid-template-rows: auto 1fr` and an item
   spanning both rows (the 64-high media), the `auto` row (the title's) gets no height and its text is not shown.
   The library writes `auto minmax(0, 1fr)`, which lays out as shadcn's does (a test guards it).
3. **No `justify-items` setter in Rust** (the style property exists, for classes): the media is centered in its
   column by auto margins, the title and description by their text alignment.

## Decisions

Settled in review.

1. **A drawn alert, not the system's.** shadcn's AlertDialog is drawn in the app's look; `nsh/base`'s
   `AlertDialog.show` remains the way to the system's alert ("When not to use").
2. **`size` is `foundation`'s `Size`**, as the conventions ask: `xs` / `sm` draw shadcn's `sm`, `md` / `lg` its
   `default` (the table in "Variants and sizes"). A separate `"default" | "sm"` type would break "one `Size`
   everywhere"; the price is two sizes that look the same as two others.
3. **Focus on Cancel as it opens** (Radix), including on touch screens.
4. **`"cancel"` for `AlertDialogCancel`, `"select"` for `AlertDialogAction`** (no new reasons).
