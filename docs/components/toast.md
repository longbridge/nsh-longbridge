# Toast

Status: implemented (P2). Brief messages that appear at a corner of the window, stack, and go by
themselves: "Event created", "Copied", a failed upload with "Retry". Reference: shadcn/ui new-york v4 `sonner.tsx`
(shadcn's toast is Sonner, styled with shadcn's tokens) and Sonner's own defaults for what `sonner.tsx` leaves to it.

Two exports, as shadcn's: `Toaster`, placed once, draws the toasts; `toast(..)`, called from anywhere, shows one.

## Usage

- The result of an action the user started and can ignore: saved, copied, sent, failed (with "Retry"), deleted
  (with "Undo").
- Progress of background work that the user may want to see end: a loading toast replaced by its result.
- Search terms: toast, sonner, snackbar, notification, flash message.

## When not to use

| Need | Use instead |
|---|---|
| An answer is required before going on | `AlertDialog` |
| An error tied to a field or a form | The field's error line (P3's `Field`) |
| A persistent state of the page (offline, read-only) | A notice in the page (the later `Alert`) or a `Badge` |
| A system notification outside the app's window | NSH's notifications module, not this |
| Information the user must not miss: toasts go by themselves and are easy to miss | Put it in the interface |

## Built on

`nsh/base`'s toast pieces (`crates/nsh-base/src/toast/`), which NSH leaves the imperative layer and the look to a
styled library (`docs/dev/components.md` "Toasts"; NSH's gallery builds its own the same way):

| Provided by `nsh/base` (not rewritten here) | |
|---|---|
| `ToastManager` | Ordered storage and the lifecycle of each toast: entering, present, its timeout, paused, ending, removed; `visible(limit)`, `push`, `dismiss`, `dismissAll`, `advance(now, paused)` |
| `ToastStack` | Measures the toasts and lays them out from the placement's edge, the newest nearest it: collapsed, overlapping layers 14 apart, each 5% narrower, 3 shown; expanded (pointer over it, a finger down on it), a list 14 apart; arrivals, departures and expansion move with springs (Sonner's `ToastMotion`); `onExpandedChange` pauses the timeouts |
| `Toast` | The semantic root (role `alert`), read once by screen readers as it shows (`accessibilityLabel`); `transitionStatus` with `startingClass` / `endingClass` / `swipingClass` for the entrance and exit; a swipe toward `swipeDirection` past 50 points, or a flick, asks to dismiss it |
| Terminal | Only the newest toast, on the status line; nothing stacks or moves |

What this library adds: the global queue behind `toast()` (per surface), the `Toaster` that places the stack and
draws each toast (icon, title, description, action and cancel buttons, close button), the look, the motion classes,
the per-toast duration and the answer (`toast()` resolves with the button that closed it).

### How `toast()` reaches the `Toaster`

- The queue is a surface global (`nsh_core`'s `global`, as `labels`' translator): one `ToastManager` per app
  surface, the content of each toast by id, and the pending answers. `toast()` pushes to it and works from anywhere
  in the app's runtime (a component, a listener, a task, JS).
- `Toaster` reads the global and renders `manager.visible(visibleToasts)`. While a toast is mounted it advances the
  lifecycle on a 50 ms timer (`nsh/base`'s pattern), paused while the stack is expanded, and stops when the queue is
  empty.
- Toasts pushed before a `Toaster` is mounted wait in the queue and show once one is; their default time is
  `toast-duration`'s default (4000), as the `Toaster`'s `duration` is not known yet. A second `Toaster` logs a
  warning in development builds and draws nothing.
- The lifecycle's ticks that change nothing on screen notify nothing: the queue tells its readers only when a toast
  is added, changes phase or goes.

### Above dialogs

A toast must show above an open dialog (Sonner's toaster has the topmost z-index), but NSH's overlays show in their
root view and a dialog is a native layer above the page's. So the stack is drawn where the topmost layer is:

- No dialog of this library open: the `Toaster` draws its stack in an overlay of the page (its root view).
- While `Dialog` / `AlertDialog` of this library are open, the topmost one draws the stack inside its modal (an
  overlay in the modal's root view), and the `Toaster`'s own draws nothing. The library keeps the open modals in a
  surface global, newest last; a modal draws the outlet while it is last.
- Moving the stack rebuilds its toasts under the new parent; `nsh/base`'s `Toast` does not read a moved toast again
  (its test covers "a stack drawn in the topmost modal"). A toast moved during its entrance starts its entrance again.
- The toasts read the `Toaster`'s `ThemeScope`, not the dialog's: the `Toaster` publishes its drawing of the stack
  with its scope captured, and the outlet provides that scope again. A test opens a dialog in another scope, shows a
  toast from it and checks that the toast is drawn inside the modal, once, in the `Toaster`'s scope, and that it
  moves back to the page as the dialog closes.
- `nsh/base`'s own modals (its `Dialog`, `Sheet`, a `Select` as a sheet on a phone) are not known to the library: a
  toast shows under them. See NSH gaps 1.

## API

### Exports

| Export | TSX | Rust | Description |
|---|---|---|---|
| `Toaster` | `<Toaster />` | `toaster()` | Draws the toasts; place it once, at the app's root |
| `toast` | `toast(title, options?)` | `toast(title)…show()` | Shows a toast; resolves once it is gone, with the button that closed it |
| `dismissToast` | `dismissToast(id?)` | `dismiss_toast(id)` | Closes the toast `id`, or every toast without one; its exit runs |

### `toast(title, options?)`

TSX: `toast(title: string, options?: ToastOptions): Promise<{ action: string | null }>`.

| Option | Type | Default | Description |
|---|---|---|---|
| `id` | `string` | a new one | Names the toast. Showing one with the `id` of a toast still shown changes that toast in place (a loading toast becoming its result) and starts its time over, as Sonner's `toast(…, { id })` |
| `description` | `string` | — | A second line under the title |
| `kind` | `"default"` \| `"success"` \| `"info"` \| `"warning"` \| `"error"` \| `"loading"` | `"default"` | The icon before the text (Sonner's `toast.success(…)` and the others); `"loading"` spins and never times out |
| `duration` | `number` (ms) | `Toaster`'s `duration` (4000) | How long it stays; `Infinity` until dismissed. Paused while the stack is expanded |
| `action` | `{ id: string, label: string }` | — | A button after the text (`Button` `xs`, default variant); a press closes the toast and resolves `{ action: id }` |
| `cancel` | `{ id: string, label: string }` | — | A secondary button before the action (`Button` `xs`, secondary); a press closes it and resolves `{ action: id }` |
| `closeButton` | `boolean` | `Toaster`'s `closeButton` | The small "×" at its corner |

The promise resolves with `{ action: null }` when the toast goes any other way (its time ran out, a swipe, the "×",
`dismissToast`). Every call that showed or changed the same `id` resolves with the same answer.

Departures from Sonner's API, with the reason:

- **`toast.success(…)` and the others are `kind`**: an nshm exports functions and modules, and a function with
  methods of its own (`toast` callable and `toast.success`) is not one of them (see NSH gaps 2). `kind` is the
  option NSH's own gallery toast uses.
- **Buttons answer with an `id` instead of an `onClick` callback**: a JS callback kept by Rust to be called later
  from a press is not how NSH crosses the boundary (the library's `setTranslator` calls its function only during the
  call); the answer is a promise, as `nsh/base`'s `AlertDialog.show` resolves `{ action }`. An app awaits it:
  `const { action } = await toast("Deleted", { action: { id: "undo", label: "Undo" } }); if (action === "undo") …`.
- **No `toast.promise`**: write it with `id` (`toast("Saving…", { id, kind: "loading" })`, then
  `toast("Saved", { id, kind: "success" })`).
- **No rich colors, `invert`, JSX content or `unstyled`** in P2 (shadcn's `sonner.tsx` uses none). A toast of the
  app's own layout is `nsh/base`'s `Toast`.

From shadcn / Sonner:

| shadcn / Sonner | This library |
|---|---|
| `<Toaster />` from `@/components/ui/sonner`, `position="top-center"` | `<Toaster />`, `placement="topCenter"` |
| `toast("Saved", { description })` | The same |
| `toast.success("Saved")`, `.error`, `.warning`, `.info`, `.loading` | `toast("Saved", { kind: "success" })`, … |
| `toast("Deleted", { action: { label: "Undo", onClick: undo } })` | `const { action } = await toast("Deleted", { action: { id: "undo", label: "Undo" } }); if (action === "undo") undo();` |
| `toast.dismiss(id)` | `dismissToast(id)` |

Rust:

```rust
let answer = toast("Event created")
    .description("Sunday, December 03, 2023 at 9:00 AM")
    .kind(ToastKind::Success)
    .action("undo", "Undo")
    .show();                               // a `ToastAnswer`: awaited, `ToastResult { action: Option<SharedString> }`
spawn(async move {
    if answer.await.action.as_deref() == Some("undo") { undo() }
});
dismiss_toast(Some("upload"));
```

Builder methods: `id`, `description`, `kind`, `duration(Option<Duration>)` (`None`: until dismissed), `action(id,
label)`, `cancel(id, label)`, `close_button(bool)`, then `show()`. Dropping the answer does not dismiss the toast.
`ToastAnswer` is `IntoFuture`, not a `Future` itself, so a toast shown for its message alone is a plain statement
(`toast("Copied").show();`) without an unused-future warning.

### `Toaster` props

| TSX | Rust | Type | Default | Description |
|---|---|---|---|---|
| `placement` | `placement` | `"topLeft"` \| `"topCenter"` \| `"topRight"` \| `"bottomLeft"` \| `"bottomCenter"` \| `"bottomRight"` (`ToastPlacement`) | `"bottomRight"` | Where the stack is (Sonner's `position`, in `nsh/base`'s `Anchor` names; its left and right middles are not offered, so the library has its own `ToastPlacement`) |
| `visibleToasts` | `visible_toasts` | `number` (`u32`) | `3` | How many are mounted at once; older ones wait (Sonner's) |
| `duration` | `duration` | `number` (ms; Rust `Option<f64>`) | `toast-duration` (4000) | The default time a toast stays (Sonner's `duration`) |
| `closeButton` | `close_button` | `boolean` | `false` | Every toast shows the "×" (Sonner's) |
| `class` / `style` | `class` / `style` | | | The stack |
| `toastClass` / `toastStyle` | `toast_class` / `toast_style` | | | Each toast's box |
| `iconClass` / `iconStyle` | `icon_class` / `icon_style` | | | The kind's icon |
| `titleClass` / `titleStyle` | `title_class` / `title_style` | | | The title text |
| `descriptionClass` / `descriptionStyle` | `description_class` / `description_style` | | | The description text |
| `actionClass` / `actionStyle` | `action_class` / `action_style` | | | The action button |
| `cancelClass` / `cancelStyle` | `cancel_class` / `cancel_style` | | | The cancel button |
| `closeClass` / `closeStyle` | `close_class` / `close_style` | | | The "×" button |

The parts are Sonner's `toastOptions.classNames` (`toast`, `icon`, `title`, `description`, `actionButton`,
`cancelButton`, `closeButton`) as the conventions' `<part>Class` / `<part>Style`. Sonner's `theme` is the
`ThemeScope` (or the app's appearance), `offset` and `gap` are tokens, `richColors`, `expand`, `invert`, `hotkey` and
`dir` are not offered (see "Decisions" and NSH gaps 3).

### Events

None on `Toaster`. A toast's outcome is its promise.

### Parts

```
Toaster
└─ overlay                       at the placement's edge, `toast-offset` (plus the safe area) from it; pointer-events box-none
   └─ stack (nsh/base ToastStack) `toast-width` wide; `class` / `style`
      └─ toast (nsh/base Toast)    a row; `toastClass` / `toastStyle`
         ├─ close                  the "×", at the top-start corner (`closeClass`)
         ├─ icon                   the kind's icon (`iconClass`)
         ├─ content                a column: title (`titleClass`), description (`descriptionClass`)
         ├─ cancel                 Button xs secondary (`cancelClass`)
         └─ action                 Button xs default (`actionClass`)
```

## Variants and sizes

No `variant` and no `size`: Sonner's one look, with `kind` changing only the icon (shadcn's `sonner.tsx` sets the
icons and no colors per kind).

### Look

shadcn's `sonner.tsx` sets `--normal-bg: var(--popover)`, `--normal-text: var(--popover-foreground)`,
`--normal-border: var(--border)`, `--border-radius: var(--radius)` and the icons; the rest is Sonner's stylesheet.

| Part | Sonner | This library |
|---|---|---|
| toast | `padding: 16px; background: var(--normal-bg); border: 1px solid var(--normal-border); color: var(--normal-text); border-radius: var(--border-radius); box-shadow: 0 4px 12px rgba(0,0,0,.1); width: 356px; font-size: 13px; display: flex; align-items: center; gap: 6px` | A row centered vertically, gap `toast-gap` (6), padding `toast-padding` (16), as wide as the stack (`toast-width`, 356); fill `toast` (`popover`), text `toast-foreground` (`popover-foreground`), 13 (`toast-font-size`); border `border-width` × `toast-border`; corners `toast-radius` (`radius-lg`, 10); shadow tier `toast-shadow` (3, `md`) |
| icon | lucide `circle-check` / `info` / `triangle-alert` / `octagon-x` / `loader-2` (spinning), `size-4`; Sonner's icon box `16×16`, margins −3 / 4 | `toast-icon-size` (16) in a 16 box, the start margin −3 and end margin 4 (Sonner's), the text color; `"loading"` turns with `foundation::motion::spin` (it keeps turning with reduced motion, as `Spinner`) |
| content | `flex-direction: column; gap: 2px` | A column, gap 2, `flex: 1`, shrinkable (`min-w-0`) |
| title | `font-weight: 500; line-height: 1.5` | `font-weight-medium`, line height from foundation (13 → 18) |
| description | `font-weight: 400; line-height: 1.4`; `#3f3f3f` (dark: `hsl(0 0% 91%)`) | Regular, line height 18, color `toast-description-foreground` (`muted-foreground`: see Decisions 3) |
| action | `height: 24px; padding: 0 8px; font-size: 12px; font-weight: 500; border-radius: 4px; background: var(--normal-text); color: var(--normal-bg); margin-left: auto` | The library's `Button` `size="xs"` `variant="default"` (24 high, 8 padding, 12 / medium), pushed to the end (`ml-auto` when it is the first button) |
| cancel | as the action, `background: rgba(0,0,0,.08)` (dark `rgba(255,255,255,.3)`), `color: var(--normal-text)` | `Button` `size="xs"` `variant="secondary"` |
| close | `20×20`, circle, `border: 1px solid var(--gray4)`, `background: var(--normal-bg)`, icon 12, at the top-left corner moved out by 35% | A 20 square circle (`toast-close-size`), border `toast-border`, fill `toast`, `x` icon 12, at the toast's top-start corner moved out by 35% of its size (7, from inside the toast's edge, as Sonner's `left: 0; top: 0` with its translate); opacity 1; under the pointer the fill turns `accent` |

Departures: the action and cancel are the library's buttons (corners `button-radius`, 8, not 4; colors `primary` /
`secondary`, which are Sonner's near-black and 8% grey on shadcn's neutral theme), so a brand theme colors a toast's
action as its other main buttons; the shadow is the library's tier 3 (`md`), the nearest to Sonner's
`0 4px 12px` at 10%.

### Placement and geometry

| | Pointer (desktop) | Touch, or a window narrower than 600 |
|---|---|---|
| Stack width | `toast-width` (356) | The window less `toast-offset` on each side (Sonner's mobile layout: `@media (max-width: 600px)`) |
| From the edges | `toast-offset` (24 with a pointer, also on a narrow window), inside the safe area | `toast-offset` (16) |
| Horizontal place | The placement's (left, center, right) | Centered (full width) |
| Collapsed / expanded | `nsh/base`'s stack: 3 layers 14 apart, each 5% narrower; expanded under the pointer, a list 14 apart | Expanded while a finger is down on it |

`ToastMotion` is `nsh/base`'s `sonner()` (400 ms springs, 200 ms exit, peek 14, gap 14, scale step 0.05, 3 shown).

### Motion

| | Sonner | This library |
|---|---|---|
| Entrance | From `translateY(100%)` (bottom placements; −100% at the top) and opacity 0, 400 ms ease | `startingClass="opacity-0 translate-y-full"` (`-translate-y-full` at the top), `transition-all` over 400 ms (Sonner's; the stack's `ToastMotion` duration) |
| Exit | Back toward the edge and opacity 0 | `endingClass="opacity-0 translate-y-full"` (`-translate-y-full` at the top), over the stack's exit (200 ms) |
| Swipe | Follows the finger, out past the threshold | `nsh/base`'s swipe; `swipingClass="transition-none"` |
| Reduced motion | not handled | No starting / ending classes: toasts show and go at once; the stack's layers are set at once (`nsh/base`'s, see NSH gaps 4) |

`toast-animate: 0` turns the entrance and exit off.

Swipe direction (Sonner's default is the placement's sides): right for `*Right`, left for `*Left`, down for
`bottomCenter`, up for `topCenter`; on touch screens, down at the bottom and up at the top.

## States

| State | Look |
|---|---|
| Entering / exiting / swiped | "Motion" |
| Expanded stack | `nsh/base`'s layout; timeouts paused |
| "×" hover | Fill `accent` (pointer only) |
| Buttons | `Button`'s states |
| Loading | The spinning icon; no timeout until changed to another kind |

## Pointer and touch

| | Desktops and desktop browsers | Phones and tablets |
|---|---|---|
| Where | A corner (default bottom right), 356 wide | Full width less 16, at the placement's top or bottom |
| Pause | Pointer over the stack (expands it) | A finger down on it (expands it) |
| Dismiss | "×", a button, a swipe with the mouse, its time | A swipe, "×", a button, its time |
| Hit areas | Buttons 24 high | The buttons take their touch heights (`control-height-touch-xs`, 32) and hit slop; the "×" gets a hit slop to 44 |

Android's Snackbar and iOS's banners sit at the bottom and the top of a phone, full width; Sonner's mobile layout
matches them.

## Accessibility

- Each toast is an `alert` and is read once as it shows: title, then description (`nsh/base`'s announcement), on
  every platform.
- Its buttons are reachable by Tab where the stack's overlay is in the Tab order (after the page's content, or the
  dialog's). Sonner's `alt+T` hotkey to jump to the toasts is not offered (NSH gaps 3).
- The "×" is a `button` named "Close" (`labels`, shared with `Dialog`).
- A toast with an action stays long enough to reach: an action toast's default `duration` is still 4000, as Sonner;
  apps give important actions `Infinity` or a longer time.
- Escape does not dismiss toasts (Sonner does not, and Escape belongs to the dialog or popup in front): the stack's
  `closeOnEscape` is false.

## Tokens

Generated from the code's declarations (`NSH_LONGBRIDGE_WRITE=1 cargo test --test tokens`); a test keeps it current.
Values are those of `nsh/base`'s default theme.

<!-- tokens:toast -->
| Token | Type | Derived default | Value |
|---|---|---|---|
| `toast` | color | `popover` |  |
| `toast-foreground` | color | `popover-foreground` |  |
| `toast-border` | color | `border` |  |
| `toast-description-foreground` | color | `muted-foreground` |  |
| `toast-radius` | length | `radius-lg` | 10 |
| `toast-shadow` | number | `3` | 3 |
| `toast-width` | length | `356` | 356 |
| `toast-padding` | length | `spacing × 4` | 16 |
| `toast-gap` | length | `spacing × 1.5` | 6 |
| `toast-font-size` | length | `13` | 13 |
| `toast-icon-size` | length | `16` | 16 |
| `toast-close-size` | length | `20` | 20 |
| `toast-offset` | length | `pointer_touch(spacing × 6, spacing × 4)` | 24 (touch 16) |
| `toast-duration` | number | `4000` | 4000 |
| `toast-animate` | number | `1` | 1 |
<!-- /tokens -->
- `toast` and `toast-foreground` follow the popup surface (`popover`, as shadcn's `--normal-bg`) but are the
  toast's own, so a theme can make toasts inverted or colored without touching popovers.
- `toast-radius` is `radius-lg` (shadcn's `--border-radius: var(--radius)`), one step rounder than a popover.
- `toast-width` and `toast-font-size` are Sonner's fixed values (356, 13), not derived.
- A window narrower than 600 spans the stack across it (Sonner's breakpoint, fixed); with a pointer it keeps the
  pointer's `toast-offset` (24), so a theme's offset holds there too (Sonner's mobile layout uses 16; changed in
  implementation: a fixed 16 would ignore the token).

## Examples

TSX:

```tsx
import { Button, Toaster, dismissToast, toast } from "nsh-longbridge";

// Once, at the root of the app.
<Toaster />

<Button
  variant="outline"
  onPress={async () => {
    const { action } = await toast("Event has been created", {
      description: "Sunday, December 03, 2023 at 9:00 AM",
      action: { id: "undo", label: "Undo" },
    });
    if (action === "undo") undoCreate();
  }}
>
  Show Toast
</Button>

// A loading toast replaced by its result.
toast("Uploading report.pdf…", { id: "upload", kind: "loading" });
try {
  await upload();
  toast("Uploaded", { id: "upload", kind: "success" });
} catch {
  toast("Upload failed", { id: "upload", kind: "error", description: "Check your connection.", duration: Infinity, closeButton: true });
}

dismissToast("upload");
```

With NSH 0.7.0, write a string child inside `<Text>` for now (see [button.md, NSH gaps](button.md#nsh-gaps), G0).

Rust:

```rust
use nsh_core::prelude::*;
use nsh_longbridge::{ToastKind, toast, toaster};

// At the root.
view().flex_1().child(app_content()).child(toaster())

// Anywhere in the app's runtime.
let answer = toast("Event has been created")
    .description("Sunday, December 03, 2023 at 9:00 AM")
    .kind(ToastKind::Success)
    .action("undo", "Undo")
    .show();
spawn(async move {
    if answer.await.action.as_deref() == Some("undo") {
        undo_create();
    }
});
```

## NSH gaps

1. **No layer above every modal (accepted as a known limitation).** NSH shows an overlay in its root view, and each modal is a native layer above;
   nothing floats above all of them. Workaround: the library's dialogs draw the stack while topmost ("Above
   dialogs"); under `nsh/base`'s own `Dialog`, `Sheet` and phone sheets toasts stay hidden. A "top layer" overlay
   (above every modal of the surface, taking no focus) would remove the workaround and cover `nsh/base`'s modals.
2. **No callable namespace.** An nshm exports components, functions and modules; `toast` cannot also have
   `toast.success`. Workaround: the `kind` option. If NSH's JS face allowed a module function with sub-functions,
   Sonner's spelling could be added without breaking.
3. **No app-wide hotkey to reach the toasts** (Sonner's `alt+T` focuses the toaster region) and no landmark role for
   the region. Not offered; could be added with `keymap.bind` and a `focus()` on the newest toast's first button.
4. **Reduced motion and the stack's springs**: verified on NSH 0.7.0, `nsh/base`'s stack sets its layers' places at
   once with reduced motion (its springs run only without it), and its `Toast` counts as entered from the start, so
   no starting look shows; the library passes no starting or ending classes then. A test checks that a toast's
   first frame is its look with reduced motion (and its starting look without). No fallback is needed; it is
   undocumented in NSH, so a note there would keep it.
5. **Expanded by default** (Sonner's `expand`): `nsh/base`'s stack expands only for the pointer or a finger, so
   `expand` is not offered.

## Decisions

Settled in review.

1. **`toast()` resolves with an `id`** instead of taking callbacks (see "Departures").
2. **`kind` instead of `toast.success(…)`** (NSH gap 2).
3. **Description color `muted-foreground`**: Sonner's `#3f3f3f` / 91% white are greys no shadcn token names; the
   nearest semantic role is `muted-foreground` (lighter than Sonner's in light, about the same in dark).
4. **The action and cancel are the library's `Button`s** (radius 8, `primary` / `secondary`).
5. **Escape does not dismiss toasts.**
6. **Toasts move into the topmost library dialog** while one is open; under `nsh/base`'s own `Dialog` and `Sheet`
   they stay hidden (NSH gaps 1, accepted).
