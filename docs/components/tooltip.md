# Tooltip

A hint bubble: a line of explanation beside an element while the pointer rests on it or the keyboard focus is on it.
The library's first popup. Reference: shadcn/ui new-york v4 `tooltip.tsx` (Radix Tooltip).

## Usage

- The name of an icon button ("Refresh"), the full text of a truncated label, a control's shortcut (with `Kbd`).
- Search terms: tooltip, hint, hover hint.

## When not to use

- **Information the user must see**: tooltips hardly ever show on touch screens (see "Pointer and touch"), and screen
  readers may not read them. Write it in the interface, or use `Popover` (P2) for "tap to see more".
- Content with something pressable (a link, a button): a tooltip takes no pointer events, so it cannot be pressed. Use
  `Popover` (P2) or the later `HoverCard`.
- Several paragraphs, images, preview cards: the later `HoverCard`.
- Form validation errors: under the field (P3's Field), not in a tooltip.
- As the accessible name: an icon button's name goes in its own `accessibilityLabel`; the tooltip is for sighted users.

## Built on

`nsh/base`'s `Tooltip` (`crates/nsh-base/src/tooltip.rs`). This library adds the look and the motion; all behavior is
`nsh/base`'s:

| Provided by `nsh/base` (not rewritten here) | |
|---|---|
| Showing | After the pointer rests on the element for `delay` ms; leaving or pressing it hides the tooltip |
| Reading along | When the pointer leaves an element **whose tooltip showed** and enters another within 300 ms, that one shows at once (the core's fixed grace period, Radix's default `skipDelayDuration`), so the tools of a toolbar can be swept in turn; a press or Escape ends the grace period |
| Keyboard | Shows **at once** when the keyboard moves the focus to the element (Tab, an arrow, a shortcut, the app's `focus()` after a key) and hides when the focus leaves; focus from a click does not count (as `focus-visible:`), so the tooltip does not linger after a click; Escape hides it |
| Position | On the element's `placement` side, flipped to the opposite side when only that one has room; aligned along the side by `align`; `offset` from the element and at least `margin` from the safe area's edges; above everything |
| Out of the way | Takes no pointer events (`pointer-events: none`); the element stays pressable and hoverable |
| Control | `open` / `defaultOpen` / `onOpenChange`, `disabled` |
| Entrance and exit | `startingClass` / `endingClass`: shown, it transitions from the starting look to its look; hidden, to the ending look, then unmounts; transitions are skipped when the user asks for reduced motion |
| Touch screens | No hover: it shows only when a hardware keyboard moves the focus to the element, otherwise only when the app sets `open` |
| Terminal | A line on the status line in the terminal's colors; this library's look does not apply |

### Differs from shadcn's structure

shadcn (Radix) has four pieces: `TooltipProvider > Tooltip > TooltipTrigger + TooltipContent`. In NSH a tooltip is an
**element's `tooltip` prop** (as GPUI's `.tooltip(..)`): the trigger is the element with the prop, the tooltip is its
value. This library follows NSH's model:

- No `TooltipTrigger`: any element (the library's `Button`, the app's `View`) with `tooltip={<Tooltip>…</Tooltip>}` is
  the trigger.
- No `TooltipContent`: `Tooltip` itself is the bubble.
- No `TooltipProvider`: shadcn uses it to set `delayDuration` for all; here the `tooltip-delay` token does, and NSH
  fixes the grace period.

The reason: NSH only recognizes a tooltip given as an element's `tooltip` prop (the core makes it a hint layer on the
element); a tooltip placed among children would not follow hover and focus.

### Implementation

- **Structure**: `#[component] fn Tooltip` whose root is `nsh_base::tooltip()`, with the props passed through and the
  look set on the root (the bubble) from `theme()` tokens; `startingClass` / `endingClass` are set by this library
  according to `tooltip-animate`; it provides the "inside a tooltip" context `Kbd` reads.
- **Whether the core recognizes the TSX wrapper as a hint layer**: the core takes "an element in a `tooltip` prop whose
  root is an overlay with a `hint`" (`Core::adopt_hint_tooltips`). This library's `Tooltip` is a component around
  `nsh/base`'s, and the host node takes its root's kind (`Core::retag`; nested components share one node), so it should
  be recognized. **The first headless test of the P1 implementation checks this**: `<View tooltip={<Tooltip>…</Tooltip>}>`
  shows after 500 ms of hover, above the element, with `delay` honored. If it is not recognized, the fallback is a
  function instead of a component, returning `nsh/base`'s `Tooltip` with the look preset (TSX: a `tooltipLook()` giving
  the `class`), and the issue is reported as an NSH gap.
- **Rust's `IntoTooltip`**: the blanket implementation for `ComponentElement<C>` treats it as a plain element and wraps
  it in a hint layer of the core's own (core defaults for position and delay; every prop of this library ignored). So
  the library implements `IntoTooltip` for its own `TooltipElement`, calling `nsh_core::element::__hint_tooltip` as
  `nsh/base` does. That function is `#[doc(hidden)]`; see "NSH gaps" 1.
- **Scope**: the hint layer lives in the trigger (the core places it among the element's children), so it reads the
  `ThemeScope` the trigger is in: a button in a dark scope gets a dark tooltip. Guarded by tests.

## API

### Attaching

```tsx
<Button variant="ghost" size="sm" accessibilityLabel="Refresh" tooltip={<Tooltip>Refresh</Tooltip>}>
  <Icon name="refresh-cw" />
</Button>
```

```rust
button().variant(ButtonVariant::Ghost).accessibility_label("Refresh")
    .tooltip(tooltip().child("Refresh"))
    .child(icon("refresh-cw"))
```

In TSX `tooltip` is a host prop every component and every `View` has (`ComponentHostProps`); in Rust it is
`Interactive::tooltip`, and the library's `TooltipElement` implements `IntoTooltip` (see "Implementation").

**Attach it to the focusable layer**: the tooltip follows the focus being on the element or inside it. On a `View`
around a button that works; on the icon **inside** a button it does not show when the button gets the keyboard focus
(the focus is outside the icon).

### Props

| TSX | Rust | Type | Default | Description |
|---|---|---|---|---|
| `children` | `.child(..)` | Text, `Kbd`, elements | | What it says; strings become the `label` part |
| `placement` | `placement` | `"top"` \| `"bottom"` \| `"left"` \| `"right"` (`nsh_core::overlay::Placement`) | `"top"` | The side it goes on; flipped when it does not fit |
| `align` | `align` | `"start"` \| `"center"` \| `"end"` (`Align`) | `"center"` | Alignment along that side; `start` / `end` follow the layout direction (reversed in RTL) |
| `delay` | `delay` | `number` (ms) | `tooltip-delay` (500) | How long the pointer rests before it shows; `0` at once; `Infinity`: neither the pointer nor the focus shows it, only the app's `open`. The keyboard focus always shows it at once regardless (except with `Infinity`) |
| `open` | `open` | `boolean` | Uncontrolled | Controlled: the app owns it and changes it in `onOpenChange` |
| `defaultOpen` | `default_open` | `boolean` | `false` | Shown at first while uncontrolled |
| `disabled` | `disabled` | `boolean` | `false` | No tooltip and no `onOpenChange` |
| `class` / `style` | `class` / `style` | | | The bubble (the root) |
| `labelClass` / `labelStyle` | `label_class` / `label_style` | | | The text made from string children |

`offset` (distance from the element) and `margin` (distance from the screen's edges) are not props: they are the
`tooltip-offset` token and `nsh/base`'s default of 4. shadcn exposes `sideOffset`, but it should be the same across an
app; change the token.

`startingClass` / `endingClass` are not exposed: the entrance and exit are this library's look (see "Motion"). To turn
the motion off, set `tooltip-animate: 0` in a scope (see "Tokens").

### Events

| TSX | Rust (`TooltipEvent`) | Payload | When |
|---|---|---|---|
| `onOpenChange` | `OpenChange(open, reason)` | `{ open: boolean, reason: "hover" \| "focus" \| "cancel" }` | The user asks to open or close: the pointer rested long enough, left or pressed (`"hover"`); the keyboard focus came or went (`"focus"`); Escape (`"cancel"`). The app changing `open` itself is not reported |
| `onOpen` | `Open` | | It is shown |
| `onClose` | `Close` | | It is entirely gone (after its exit), whoever closed it |

All are `nsh/base` `Tooltip`'s events, passed through.

### Parts

| Part | Props | What it is |
|---|---|---|
| Bubble | `class` / `style` | The root: fill, corners, padding |
| Label | `labelClass` / `labelStyle` | The `text` made from string children; not made for element children, where these props do nothing |

No arrow part (decided in review; see "Decisions" 1).

## Variants and sizes

- No `variant`: shadcn has one (the inverted bubble).
- No `size` (none of `xs` / `sm` / `md` / `lg`): a tooltip's size does not follow its trigger.

### Look

shadcn: `w-fit rounded-md bg-foreground px-3 py-1.5 text-xs text-balance text-background`, with an arrow
`size-2.5 rotate-45 rounded-[2px] bg-foreground` and `sideOffset` 0 (the arrow fills the gap).

| | Value |
|---|---|
| Fill / text | `tooltip` / `tooltip-foreground`: inverted by default (dark on a light theme, light on a dark one), as shadcn |
| Corners | `tooltip-radius` (`radius-md`, 8) |
| Padding | `tooltip-padding-x` (12) horizontally, `tooltip-padding-y` (6) vertically |
| Text | `tooltip-font-size` (12), line height from the foundation's font size → line height (12 → 16), regular weight |
| Layout | A row, centered vertically, `spacing` × 2 (8) apart: the gap before a trailing `Kbd` |
| Width | As wide as its content, at most `tooltip-max-width` (320), wrapping beyond |
| Distance from the element | `tooltip-offset` (4) |
| Shadow | None (shadcn has none) |
| One-line height | 16 + 6 × 2 = 28 |

Differences from shadcn:

- **No arrow**, so `offset` is 4 instead of 0. See "Decisions" 1.
- **A maximum width**: shadcn has none (it relies on `text-balance` for even lines). Without one a long hint spreads to
  the window's edge (seen in a real app: a long description running out of the window). 320 at 12 points is about 24
  CJK or 50 Latin characters per line. Whether NSH has `text-balance` is checked in the implementation; without it the
  text wraps normally.
- Regular weight, as shadcn (gpui-kit uses medium; not followed).

### Motion

| | shadcn (tw-animate-css) | This library |
|---|---|---|
| Entrance | `fade-in-0 zoom-in-95`, plus `slide-in-from-*-2` (8) by the actual side, 150 ms | From `opacity-0 scale-95` to its look over `duration` (150 ms), no slide |
| Exit | `fade-out-0 zoom-out-95` | To `opacity-0 scale-95`, then unmounted |
| Scale origin | The point toward the trigger Radix computes | The bubble's center |
| Reduced motion | Not handled | No transition: shown and hidden at once (built into `nsh/base`) |

No slide and a centered origin, because both need the side the bubble **finally** went on (it may flip), and
`nsh/base`'s `Tooltip` does not report its position (`Popover` and `HoverCard` have `onPosition`; `Tooltip` has not).
Sliding by the requested `placement` would slide the wrong way after a flip. See "NSH gaps".

Reading along (moving from one tooltip to the next within the grace period) runs the entrance again for the new one;
gpui-kit's bubble that glides from the previous position to the next is not done, as `nsh/base` cannot.

## States

| State | Look |
|---|---|
| Entering / exiting | See "Motion" |
| Trigger hovered / focus-visible | The tooltip does not change the trigger's look; those are the trigger's own states |
| Trigger disabled | As `nsh/base`: the tooltip follows the pointer and the focus, regardless of the element being disabled. A disabled button rarely gets the focus, so only hover shows it. A tooltip may explain why it cannot be pressed; to have none, set `disabled`. A headless test confirms that hovering a disabled `Button` shows its tooltip |

No invalid / loading.

## Pointer and touch

| | Desktops (and desktop browsers) | Phones and tablets |
|---|---|---|
| Shows | After the pointer rests for `delay`; at once on keyboard focus | At once when a hardware keyboard moves the focus to it; otherwise only on the app's `open` |
| Hides | Pointer leaves or presses; focus leaves; Escape | Focus leaves; Escape; the app sets `open=false` |
| Long press | — | **Does not show it.** NSH does not do it, and neither do Radix or the iOS and Android system controls (Android's long-press hint on toolbar icons is system behavior the app does not draw); a long press means other things on touch screens (context menus, dragging) |
| Form | A bubble | The same bubble, same size (it only shows with a keyboard attached, when the user sits about as far from the screen as at a desktop) |
| Hit area | The trigger itself | Same |

So on phones tooltips are **almost never seen**: whatever a tooltip explains needs another way in on touch screens (the
accessible name, text in the interface, a Popover). Do not imitate "tap to see more" by opening a tooltip with a
controlled `open` on press: the press also triggers the element's own action, and the next press asks to close it
again. Use `Popover` (P2) for that.

## Accessibility

- The tooltip is no Tab stop and never takes the focus; Tab moves between triggers as usual, each showing its tooltip.
- Escape hides the current tooltip (`onOpenChange` with `reason` `"cancel"`); the focus stays.
- Screen readers: see "NSH gaps" 2. Until that is confirmed, this library requires **the trigger to carry its full
  accessible name** (an icon button's `accessibilityLabel`) and any extra explanation in `accessibilityHint`
  (`nsh/base`'s `Button` has it), without relying on the tooltip being read.

## Tokens

| Token | Type | Derived default | Light / dark |
|---|---|---|---|
| `tooltip` | color | `foreground` | Near black in light, near white in dark (follows `foreground`: shadcn's inverted bubble) |
| `tooltip-foreground` | color | `background` | The reverse |
| `tooltip-radius` | length | `radius-md` (8) | Same |
| `tooltip-padding-x` | length | `spacing` × 3 (12) | Same |
| `tooltip-padding-y` | length | `spacing` × 1.5 (6) | Same |
| `tooltip-font-size` | length | 12 (Tailwind's `text-xs`) | Same |
| `tooltip-max-width` | length | 320 | Same |
| `tooltip-offset` | length | `spacing` (4) | Same |
| `tooltip-delay` | number | 500 (ms) | Same |
| `tooltip-animate` | number | 1 (1: entrance and exit, 0: none) | Same |

- `tooltip-font-size` is Tailwind's fixed 12, not `font-size-xs`: a tooltip is explanatory text, not a control, and
  tokens.md gives such text Tailwind's values.
- For a light tooltip in the style of gpui-kit and the system (`popover` fill, `border` edge), set
  `"tooltip": "var(--popover)"` and `"tooltip-foreground": "var(--popover-foreground)"` in the theme and add
  `class="border"`. No separate variant.
- `tooltip-animate` is a switch, not a duration: the duration follows the `duration` seed. At 0 the library passes no
  `startingClass` / `endingClass` to `nsh/base`, and showing and hiding happen in one frame.

## Examples

```tsx
import { keymap } from "nsh";
import { Button, Icon, Kbd, Tooltip } from "nsh-longbridge";

// An icon button
<Button variant="ghost" size="sm" accessibilityLabel="Add to favorites" tooltip={<Tooltip>Add to favorites</Tooltip>}>
  <Icon name="plus" />
</Button>

// With a shortcut, below the trigger
<Button
  variant="outline"
  tooltip={
    <Tooltip placement="bottom">
      Save <Kbd>{keymap.label("mod+s")}</Kbd>
    </Tooltip>
  }
>
  Save
</Button>

// The full text of a truncated label, at once
<Text numberOfLines={1} tooltip={<Tooltip delay={0}>{fullName}</Tooltip>}>{fullName}</Text>

// Controlled
const [open, setOpen] = useState(false);
<View tooltip={<Tooltip open={open} onOpenChange={(e) => setOpen(e.open)}>Synced 2 min ago</Tooltip>}>…</View>
```

```rust
use nsh_core::{element::key_label, overlay::Placement};
use nsh_longbridge::{button, kbd, tooltip, ButtonVariant, TooltipEvent};

button().variant(ButtonVariant::Outline)
    .tooltip(
        tooltip().placement(Placement::Bottom)
            .child("Save")
            .child(kbd().child(key_label("mod+s"))),
    )
    .child("Save")

let open = signal(false);
view().tooltip(
    tooltip().open(move || Some(open.get()))
        .on_open_change(move |e: &TooltipEvent| {
            if let TooltipEvent::OpenChange(next, _reason) = e {
                open.set(*next);
            }
        })
        .child("Synced 2 min ago"),
)
```

## NSH gaps

1. **A third-party Rust component can only be a tooltip through hidden functions**: `IntoTooltip::apply_tooltip` and
   `__hint_tooltip` are `#[doc(hidden)]`, and the blanket implementation for `ComponentElement` adds a layer. Workaround:
   call `__hint_tooltip` as `nsh/base` does, pin the NSH version, guard upgrades with tests. A public way would help
   (for example, a component whose root is a hint layer being recognized as a tooltip).
2. **Unconfirmed whether screen readers read the tooltip**: Radix uses `aria-describedby` so that the tooltip is read as
   the trigger's description. NSH's docs do not say whether a tooltip becomes the trigger's accessibility description.
   Workaround: triggers carry their own name / `accessibilityHint` (see "Accessibility"); tested with VoiceOver on macOS
   during the implementation, with the result reported.
3. **`Tooltip` has no `onPosition`**: the side it ends up on after a flip is unknown, so there is no arrow pointing at the
   element, no slide by side and no scale origin toward the element. Workaround: no arrow, no slide, a centered origin.
   `Tooltip` reporting `onPosition` (as `Popover` and `HoverCard` do) would lift this.
4. **A tooltip above a popup opened later** (met in P2; noted now): seen in a real app: a button opens a Popover, menu or
   dialog, the focus stays on the button, and the button's tooltip shows **above** the popup and takes Escape first. An
   app can work around it by counting popup layers. When P2 builds `Popover`, reproduce it on NSH first, then decide
   whether NSH fixes it or this library works around it.
5. **No hover tooltips on an iPad with a pointer**: `nsh/base` treats every tablet as a touch screen, while iPadOS shows
   pointer hints with a trackpad. Low priority; noted.

## Decisions

1. **No arrow; `offset` 4.** `Tooltip` does not report its actual side (gap 3), so after a flip an arrow would point the
   wrong way; gpui-kit's and the NSH gallery's tooltips have no arrow either. Revisit once NSH reports `onPosition`; the
   arrow would then be an `arrowClass` / `arrowStyle` part.
2. **Default delay 500 ms** (`tooltip-delay`). shadcn's `TooltipProvider` sets 0, a web habit; on a desktop a pointer
   crossing a toolbar should not flash tooltips all the way, and the grace period already makes reading along
   immediate. Where the user is there to read it (the full text of a truncation), set `delay={0}` there.
3. **shadcn's inverted colors** (`foreground` fill). The light style (`popover` fill and border) is the two-token theme
   change described under "Tokens".
4. **The API follows NSH's `tooltip` prop**, not shadcn's four pieces. Whether the core recognizes the TSX wrapper as a
   hint layer is the first headless test of the implementation.
