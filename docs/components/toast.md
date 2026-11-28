# Toast

Status: implemented (P2). Brief messages that appear at a corner of the window, stack, and go by
themselves: "Event created", "Copied", a failed upload with "Retry".

Two exports: `Toaster`, placed once, draws the toasts; `toast(..)`, called from anywhere, shows one.

## Usage

- The result of an action the user started and can ignore: saved, copied, sent, failed (with "Retry"), deleted
  (with "Undo").
- Progress of background work that the user may want to see end: a loading toast replaced by its result.
- Search terms: toast, snackbar, notification, flash message.

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
| `ToastStack` | Measures the toasts and lays them out from the placement's edge, the newest nearest it: collapsed, overlapping layers `toast-stack-peek` (12) apart, each 5% narrower, 3 shown, the content of those behind the newest hidden by the library (NSH gap 6); expanded (pointer over it, a finger down on it), a list `toast-stack-gap` (12) apart; arrivals, departures and expansion move with springs (`ToastMotion`); `onExpandedChange` pauses the timeouts |
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
- The pause starts and ends the moment the stack expands and collapses: the lifecycle advances once then, so the
  time counted is the time the stack was collapsed, not rounded to a tick.
- A toast's remaining time is kept beside `nsh/base`'s manager, which counts it down without telling it (NSH gaps 7):
  the library's count starts, stops and starts over at the same moments as the manager's, and the remaining-time
  bar reads it.
- Toasts pushed before a `Toaster` is mounted wait in the queue and show once one is; their default time is
  `toast-duration`'s default (4000), as the `Toaster`'s `duration` is not known yet. A second `Toaster` logs a
  warning in development builds and draws nothing.
- The lifecycle's ticks that change nothing on screen notify nothing: the queue tells its readers only when a toast
  is added, changes phase or goes.

### Above dialogs

A toast must show above an open dialog (a message the user must not miss), but NSH's overlays show in their
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
| `id` | `string` | a new one | Names the toast. Showing one with the `id` of a toast still shown changes that toast in place (a loading toast becoming its result) and starts its time over |
| `description` | `string` | — | A second line under the title |
| `kind` | `"default"` \| `"success"` \| `"info"` \| `"warning"` \| `"error"` \| `"loading"` | `"default"` | The icon before the text (`toast.success(…)` and the others elsewhere); `"loading"` spins and never times out |
| `duration` | `number` (ms) | `Toaster`'s `duration` (4000) | How long it stays; `Infinity` until dismissed. Paused while the stack is expanded |
| `action` | `{ id: string, label: string }` | — | A button after the text (`Button` `xs`, default variant); a press closes the toast and resolves `{ action: id }` |
| `cancel` | `{ id: string, label: string }` | — | A secondary button before the action (`Button` `xs`, secondary); a press closes it and resolves `{ action: id }` |
| `closeButton` | `boolean` | `Toaster`'s `closeButton` | The small "×" at its corner |
| `timer` | `boolean` | `Toaster`'s `timer`; left out there too, shown when the toast has an `action` or a `cancel` | The remaining-time bar along its edge; never on a toast that does not time out (`loading`, `Infinity`) |

The promise resolves with `{ action: null }` when the toast goes any other way (its time ran out, a swipe, the "×",
`dismissToast`). Every call that showed or changed the same `id` resolves with the same answer.

Where the API differs from the common toast API (`toast.success(…)`, `onClick` callbacks), and why:

- **`toast.success(…)` and the others are `kind`**: an nshm exports functions and modules, and a function with
  methods of its own (`toast` callable and `toast.success`) is not one of them (see NSH gaps 2). `kind` is the
  option NSH's own gallery toast uses.
- **Buttons answer with an `id` instead of an `onClick` callback**: a JS callback kept by Rust to be called later
  from a press is not how NSH crosses the boundary (the library's `setTranslator` calls its function only during the
  call); the answer is a promise, as `nsh/base`'s `AlertDialog.show` resolves `{ action }`. An app awaits it:
  `const { action } = await toast("Deleted", { action: { id: "undo", label: "Undo" } }); if (action === "undo") …`.
- **No `toast.promise`**: write it with `id` (`toast("Saving…", { id, kind: "loading" })`, then
  `toast("Saved", { id, kind: "success" })`).
- **No rich colors, `invert`, JSX content or `unstyled`** in P2: one look, colored by the theme. A toast of the
  app's own layout is `nsh/base`'s `Toast`.

Coming from the common toast API:

| Common API | This library |
|---|---|
| `<Toaster position="top-center" />` | `<Toaster placement="topCenter" />` |
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
label)`, `cancel(id, label)`, `close_button(bool)`, `timer(bool)`, then `show()`. Dropping the answer does not dismiss the toast.
`ToastAnswer` is `IntoFuture`, not a `Future` itself, so a toast shown for its message alone is a plain statement
(`toast("Copied").show();`) without an unused-future warning.

### `Toaster` props

| TSX | Rust | Type | Default | Description |
|---|---|---|---|---|
| `placement` | `placement` | `"topLeft"` \| `"topCenter"` \| `"topRight"` \| `"bottomLeft"` \| `"bottomCenter"` \| `"bottomRight"` (`ToastPlacement`) | `"bottomRight"` | Where the stack is (in `nsh/base`'s `Anchor` names; its left and right middles are not offered, so the library has its own `ToastPlacement`) |
| `visibleToasts` | `visible_toasts` | `number` (`u32`) | `3` | How many are mounted at once; older ones wait |
| `duration` | `duration` | `number` (ms; Rust `Option<f64>`) | `toast-duration` (4000) | The default time a toast stays |
| `closeButton` | `close_button` | `boolean` | `false` | Every toast shows the "×" |
| `timer` | `timer` | `boolean` (Rust `Option<bool>`) | left out: shown on toasts with a button | `true`: every toast that times out shows the remaining-time bar; `false`: none does. A toast's own `timer` wins |
| `class` / `style` | `class` / `style` | | | The stack |
| `toastClass` / `toastStyle` | `toast_class` / `toast_style` | | | Each toast's box |
| `iconClass` / `iconStyle` | `icon_class` / `icon_style` | | | The kind's icon |
| `titleClass` / `titleStyle` | `title_class` / `title_style` | | | The title text |
| `descriptionClass` / `descriptionStyle` | `description_class` / `description_style` | | | The description text |
| `actionClass` / `actionStyle` | `action_class` / `action_style` | | | The action button |
| `cancelClass` / `cancelStyle` | `cancel_class` / `cancel_style` | | | The cancel button |
| `closeClass` / `closeStyle` | `close_class` / `close_style` | | | The "×" button |
| `timerClass` / `timerStyle` | `timer_class` / `timer_style` | | | The remaining-time bar (its fill, its height); its scale and transition are the library's |

Each part (`toast`, `icon`, `title`, `description`, `action`, `cancel`, `close`, `timer`) has the conventions' `<part>Class` /
`<part>Style`. The appearance is the `ThemeScope` (or the app's), the offset and the gap are tokens; rich colors, an
always-expanded stack, `invert`, a hotkey and a text direction are not offered (see "Decisions" and NSH gaps 3).

### Events

None on `Toaster`. A toast's outcome is its promise.

### Parts

```
Toaster
└─ overlay                       at the placement's edge, `toast-offset` (plus the safe area) from it; pointer-events box-none
   └─ stack (nsh/base ToastStack) `toast-width` wide; `class` / `style`
      └─ toast (nsh/base Toast)    a row; `toastClass` / `toastStyle`
         ├─ timer clip             absolute over the box, the box's inner corners, clipping; hidden from accessibility
         │  └─ timer               the remaining-time bar at the placement's edge (`timerClass`)
         ├─ icon                   the kind's icon (`iconClass`)
         ├─ content                a column: title (`titleClass`), description (`descriptionClass`)
         ├─ cancel                 Button xs secondary (`cancelClass`)
         ├─ action                 Button xs default (`actionClass`)
         └─ close                  the "×", last in the row at the top-end corner (`closeClass`)
```

## Variants and sizes

No `variant` and no `size`: one look, with `kind` changing only the icon (no colors per kind: a toast's words carry
its kind, and the theme's colors stay calm).

### Look

A toast is a popup surface (`popover`, `popover-foreground`, `border`) with `radius-lg` corners.

| Part | Look |
|---|---|
| toast | A row centered vertically, gap `toast-gap` (12) between the icon, the content and the buttons, padding `toast-padding` (16), as wide as the stack (`toast-width`, 356); fill `toast` (`popover`), text `toast-foreground` (`popover-foreground`), 13 (`toast-font-size`); border `border-width` × `toast-border`; corners `toast-radius` (`radius-lg`, 10); shadow tier `toast-shadow` (3, `md`) |
| icon | lucide `circle-check` / `info` / `triangle-alert` / `octagon-x` / `loader-2`: `toast-icon-size` (16) in a 16 box at the padding's edge (no margins of its own), the text color; `"loading"` turns with `foundation::motion::spin` (it keeps turning with reduced motion, as `Spinner`) |
| content | A column, gap `spacing` (4) between the title and the description, `flex: 1`, shrinkable (`min-w-0`) |
| title | `font-weight-medium`, line height from foundation (13 → 18) |
| description | Regular, line height 18, color `toast-description-foreground` (`muted-foreground`: see Decisions 3) |
| action | The library's `Button` `size="xs"` `variant="default"` (24 high, 8 padding, 12 / medium), pushed to the end (`ml-auto` when it is the first button) |
| cancel | `Button` `size="xs"` `variant="secondary"`; the cancel and the action are a row of their own, `spacing × 2` (8) apart (Decisions 11) |
| close | A 20 square (`toast-close-size`), no border or fill, corners `radius-sm`, `x` icon 14 in `toast-description-foreground`; last in the row, `toast-gap` after the buttons, at the top-end corner: centered on the first line of the text, its end pulled `spacing` (4) into the padding so the glyph lines up with the padding's edge (Decisions 13); under the pointer the fill turns `accent` and the icon `toast-foreground` |
| timer | A bar `toast-timer-height` (2) high along the toast's edge on the placement's side (the bottom edge of a bottom stack, the top edge of a top one), just inside the border, filled `toast-timer`; the full width when the time starts, shrinking toward the inline start (the left, the right in a right-to-left layout) at a constant pace to nothing as the time runs out. It is clipped by the toast's inner corners (`toast-radius` less `border-width`), so its ends follow the curve. No track: the toast's fill is what is left |

The action and cancel are the library's buttons (corners `button-radius`, 8; colors `primary` / `secondary`), so a
brand theme colors a toast's action as its other main buttons; the shadow is the library's tier 3 (`md`).

### Placement and geometry

| | Pointer (desktop) | Touch, or a window narrower than 600 |
|---|---|---|
| Stack width | `toast-width` (356) | The window less `toast-offset` on each side (the phone layout) |
| From the edges | `toast-offset` (24 with a pointer, also on a narrow window), inside the safe area | `toast-offset` (16) |
| Horizontal place | The placement's (left, center, right) | Centered (full width) |
| Collapsed / expanded | `nsh/base`'s stack: 3 layers `toast-stack-peek` (12) apart, each 5% narrower; expanded under the pointer, a list `toast-stack-gap` (12) apart | Expanded while a finger is down on it |

`ToastMotion` is `nsh/base`'s default toast motion (400 ms springs, 200 ms exit, scale step 0.05, 3 shown) with the
peek and the gap from `toast-stack-peek` and `toast-stack-gap` (12 each, one content group's gap, on the spacing
scale).

### Motion

| | |
|---|---|
| Entrance | `startingClass="opacity-0 translate-y-full"` (`-translate-y-full` at the top), `transition-all` over 400 ms (the stack's `ToastMotion` duration) |
| Exit | `endingClass="opacity-0 translate-y-full"` (`-translate-y-full` at the top), over the stack's exit (200 ms) |
| Swipe | Follows the finger, out past the threshold: `nsh/base`'s swipe; `swipingClass="transition-none"` |
| Reduced motion | No starting / ending classes: toasts show and go at once; the stack's layers are set at once (`nsh/base`'s, see NSH gaps 4) |
| Remaining-time bar | One linear transition of its horizontal scale, from what is left to nothing over the time left, started when the toast is fully in (its time starts then) and again as the stack collapses; while paused it stands at what is left; a toast changed in place by `id` fills it again over `duration` (150) and it starts again once the toast is in. With reduced motion there is no bar (Decisions 8) |

`toast-animate: 0` turns the entrance and exit off.

Swipe direction: right for `*Right`, left for `*Left`, down for
`bottomCenter`, up for `topCenter`; on touch screens, down at the bottom and up at the top.

## States

| State | Look |
|---|---|
| Entering / exiting / swiped | "Motion" |
| Expanded stack | `nsh/base`'s layout; timeouts paused |
| "×" hover | Fill `accent`, icon `toast-foreground` (pointer only) |
| Buttons | `Button`'s states |
| Loading | The spinning icon; no timeout until changed to another kind, so no remaining-time bar |
| Time running | The remaining-time bar shrinks (toasts with a button, or `timer`) |
| Paused (stack expanded) | The bar stands still at what is left; every toast's time stands still |
| Changed in place (`id`) | The bar fills again; the time starts over once the toast is in |

## Pointer and touch

| | Desktops and desktop browsers | Phones and tablets |
|---|---|---|
| Where | A corner (default bottom right), 356 wide | Full width less 16, at the placement's top or bottom |
| Pause | Pointer over the stack (expands it); the remaining-time bar stops | A finger down on it (expands it); the bar stops |
| Dismiss | "×", a button, a swipe with the mouse, its time | A swipe, "×", a button, its time |
| Hit areas | Buttons 24 high | The buttons take their touch heights (`control-height-touch-xs`, 32) and hit slop; the "×" gets a hit slop to 44 |

Android's Snackbar and iOS's banners sit at the bottom and the top of a phone, full width; the phone layout
matches them.

## Accessibility

- Each toast is an `alert` and is read once as it shows: title, then description (`nsh/base`'s announcement), on
  every platform.
- Its buttons are reachable by Tab where the stack's overlay is in the Tab order (after the page's content, or the
  dialog's). A hotkey to jump to the toasts is not offered (NSH gaps 3).
- The "×" is a `button` named "Close" (`labels`, shared with `Dialog`).
- A toast with an action stays long enough to reach: an action toast's default `duration` is still 4000;
  apps give important actions `Infinity` or a longer time.
- The remaining-time bar is decorative: hidden from accessibility (`accessible(false)`,
  `accessibilityElementsHidden`), no role, no live value. A screen reader hears the toast once as it shows, and
  reaching its buttons expands the stack only for a pointer or a finger; a bar that announced its value would
  interrupt what the user is doing for something the time itself already decides. Apps whose action matters give it
  a longer `duration`, as above.
- The bar does not change the timing: a toast goes when its time runs out whether the bar is shown or not.
- Escape does not dismiss toasts (Escape belongs to the dialog or popup in front): the stack's
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
| `toast-gap` | length | `spacing × 3` | 12 |
| `toast-font-size` | length | `13` | 13 |
| `toast-icon-size` | length | `16` | 16 |
| `toast-close-size` | length | `20` | 20 |
| `toast-stack-gap` | length | `spacing × 3` | 12 |
| `toast-stack-peek` | length | `spacing × 3` | 12 |
| `toast-offset` | length | `pointer_touch(spacing × 6, spacing × 4)` | 24 (touch 16) |
| `toast-duration` | number | `4000` | 4000 |
| `toast-animate` | number | `1` | 1 |
| `toast-timer` | color | `primary` |  |
| `toast-timer-height` | length | `2` | 2 |
<!-- /tokens -->
- `toast` and `toast-foreground` follow the popup surface (`popover`) but are the
  toast's own, so a theme can make toasts inverted or colored without touching popovers.
- `toast-radius` is `radius-lg`, one step rounder than a popover.
- `toast-width` and `toast-font-size` are fixed values (356, 13), not derived.
- A window narrower than 600 spans the stack across it (a fixed breakpoint); with a pointer it keeps the
  pointer's `toast-offset` (24), so a theme's offset holds there too (a fixed 16 would ignore the token).
- `toast-timer` is `primary`, the action button's fill, so the bar reads as the action's time and a brand theme
  colors both; `toast-timer-height` is a fixed 2, thin enough not to read as a progress bar of the toast's own
  task. A calmer bar is `toast-timer: var(--muted-foreground)`.

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

// A retry: the bar shows how long "Retry" stays (it has an action).
const { action } = await toast("Message not sent", {
  kind: "error",
  description: "The connection dropped.",
  action: { id: "retry", label: "Retry" },
  duration: 8000,
});
if (action === "retry") resend();

// No bar on this one; a bar on a toast without buttons.
toast("Archived", { action: { id: "undo", label: "Undo" }, timer: false });
toast("Copied", { timer: true });
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
   the `toast.success` spelling could be added without breaking.
3. **No app-wide hotkey to reach the toasts** (such as `alt+T` focusing the toaster region) and no landmark role for
   the region. Not offered; could be added with `keymap.bind` and a `focus()` on the newest toast's first button.
4. **Reduced motion and the stack's springs**: verified on NSH 0.7.0, `nsh/base`'s stack sets its layers' places at
   once with reduced motion (its springs run only without it), and its `Toast` counts as entered from the start, so
   no starting look shows; the library passes no starting or ending classes then. A test checks that a toast's
   first frame is its look with reduced motion (and its starting look without). No fallback is needed; it is
   undocumented in NSH, so a note there would keep it.
5. **Expanded by default**: `nsh/base`'s stack expands only for the pointer or a finger, so
   an always-expanded stack is not offered.
6. **Collapsed layers keep their own height.** The toasts behind the newest should take the newest's height and hide
   their content, so they read as cards behind it; `nsh/base`'s stack lays each layer out at its measured height. The
   library hides the content of the layers behind the newest while the stack is collapsed (fading in as it spreads out;
   a test guards it); a taller layer behind still peeks further. A collapsed height option on the stack would close the
   gap.
7. **The manager does not tell a toast's remaining time.** `nsh/base`'s `ToastManager` counts each toast's timeout
   down but keeps it private. Workaround: the library keeps its own count beside it, started and stopped as the
   manager's is (when the toast is fully in, while the stack is collapsed), started over when the toast is shown
   again by `id`; a test checks that the bar runs out as the toast goes. A `remaining(id)` (and the total) on the
   manager would remove the second count.

## Decisions

Settled in review.

1. **`toast()` resolves with an `id`** instead of taking callbacks (see "Departures").
2. **`kind` instead of `toast.success(…)`** (NSH gap 2).
3. **Description color `muted-foreground`**: the semantic role for secondary text, so the description follows the
   theme rather than a fixed grey.

4. **The action and cancel are the library's `Button`s** (radius 8, `primary` / `secondary`).
5. **Escape does not dismiss toasts.**
6. **Toasts move into the topmost library dialog** while one is open; under `nsh/base`'s own `Dialog` and `Sheet`
   they stay hidden (NSH gaps 1, accepted).
7. **The remaining-time bar shows by default on toasts with a button** (`action` or `cancel`) that time out. On
   such a toast the time is the window to act ("Undo", "Retry"), and without the bar the user cannot tell how long
   it stays; on a plain message ("Copied") the time only clears the screen, and a moving line would draw the eye to
   something that needs nothing. `timer` forces it on or off, per toast or for the `Toaster`. This adds a visual
   to existing action toasts in 0.1.x (no behavior or timing changes); `timer={false}` on the `Toaster` restores
   the old look.
8. **No bar with reduced motion.** NSH runs no transitions then, so a shrinking bar would have to be redrawn on
   every tick, which is the motion the user asked to avoid; a bar standing still would say the time is not
   running. The timeout is unchanged. (A spinner keeps turning because a still one looks stuck; a missing bar does
   not look broken.)
9. **The bar is on the placement's edge** (the bottom of a bottom stack): the opposite edge is where the layers
   behind peek out of a collapsed stack, and a line there would read as a seam between the cards. It shrinks toward
   the inline start, as reading order runs, with no track, inside the border, clipped by the inner corners.
10. **A transition, not a tick.** The bar is one linear transform transition per run (no layout, the renderer's own
    interpolation), restyled only when the toast is fully in, as the stack expands or collapses and when it is
    changed in place: nothing runs on the lifecycle's 50 ms timer for it, and a paused bar costs nothing.
11. **Three groups on the spacing scale**: the icon, the message and the buttons are distinct groups, `toast-gap` (12)
    apart, as gpui-kit's notifications (16 padding, a 12 column gap); the title and description are parts of one
    message (`spacing`, 4) and the two buttons closely related controls (8). The icon used to be pulled 3 out and
    pushed 4 in with margins of its own, around a 6 gap, so a toast with an icon started 3 points left of one without
    and its text 10 from the icon, both off the scale.
12. **The stack's distances are tokens on the spacing scale.** `nsh/base`'s default motion keeps collapsed layers and
    expanded toasts 14 apart; the library passes `toast-stack-peek` and `toast-stack-gap` (12 each) instead, so the
    expanded list is spaced as one content group and a theme can change both.
13. **The "×" sits inside the toast, at the top-end corner.** It used to be a bordered circle on the top-start
    corner, moved out over the border by 35% of its size: it covered the border and the corner radius, stood away
    from the title it closes and, on the start side, read before the message. Last in the row, it takes its own room,
    so it never covers the text or the buttons, and it mirrors with the layout direction.
