# Component conventions

Every component follows these conventions. Change this document before changing a convention.

## What decides what

| Aspect | Decided by |
|---|---|
| The look (colors, radii, type, shadows, spacing) | A quiet, neutral look, its values as tokens ([tokens.md](tokens.md)) |
| Behavior, keys, focus, accessibility | `nsh/base`: a component wraps its `nsh/base` counterpart where there is one, never rewriting its behavior |
| The form of an interaction | The platform's conventions (a popup on a desktop, a sheet on a touch screen); what `nsh/base` already does per platform is used as is |
| Desktop conventions | gpui-kit's Design Guides: the arrow cursor on buttons, the pointing hand on links only; medium density by default; concentric radii; every state visible |

Web habits not to copy: critical actions that appear only under the pointer, a pointing hand everywhere, a
mobile-first single column.

## API

- **One component per file**, `src/components/<name>.rs`, registered in the root's `#[native_module]`. The JS export
  is the type's name.
- **Props are a struct**, events an enum, choices a `PropEnum`. The TypeScript declarations are generated from them,
  so every field has an English doc comment.
- **Controlled**: the app owns the value and the component reports changes. Events are named as NSH names them
  (`nsh/base`, NSH's `docs/dev/api-naming.md`): `onChange({ checked })`, `onChange({ value })`,
  `onOpenChange({ open })`, `onPress`. Apps use this library beside `nsh/base`, so one thing has one name.
- **Exports named like NSH's** (`Input`, `Button`) keep their names; an app using both renames one
  (`import { Input as BaseInput } from "nsh/base"`).
- **`variant` and `size`**: variants have the usual names (`default`, `secondary`, `outline`, `ghost`, `destructive`,
  `link`; only those a component needs). `size` is `foundation`'s `Size` (`xs`, `sm`, `md`, `lg`; `md` by default),
  which changes the height, type, icon and padding together; there is no separate height prop.
- **Styled parts**: the root takes `class` and `style` (merged last, over the component's own). Each meaningful part
  takes `<part>Class` and `<part>Style` (`labelClass`, `iconStyle`), named in the spec. Every child is wrapped
  in a text part named `label`, which gives a string child its text style (a component cannot tell a string child
  from an element).
- **Group names** a component uses inside start with `nlb-` (`group/nlb-checkbox`), leaving other names to the app
  and to `nsh/base`.
- **Context instead of ancestor selectors**: a part that looks different inside a container (a `Kbd` inside a
  `Tooltip`) asks the container's context (`in_tooltip()`), which the container provides; NSH has no ancestor or
  `:has()` selectors.
- **Composition over switches**: structure is made of parts (`Card`, `CardHeader`, `CardTitle`, `CardContent`), not
  of boolean props that show or hide pieces.
- **Content is slots and children**, not fixed markup.
- **A component's own text** (accessibility labels, "Loading") is English by default; an app translates it with
  `labels::set_translator` in Rust or `setTranslator` in TSX.
- **Examples** write a string child as a string (`<Button>Save</Button>`). Until NSH releases its fix for string
  children of Rust components (NSH 0.7.0 shows none), the gallery wraps them in `<Text>` and each spec says so once.
- **In TSX examples**, an element that refers to an element after it keeps the ref in a signal (JSX reads a plain
  variable once, before the later element sets it), and text is cut with `class="truncate"` (the `numberOfLines`
  prop is not read on a Rust component).
- **A new component, or a new or changed public API, is reviewed**: its spec is written before its code.

## Drawing

- Colors, radii and sizes are read from the `theme()` handle. A component declares its tokens with `tokens!` in its
  file, their defaults derived from semantic tokens. A color literal in `src/components/` fails a test.
- States: a component has the states its role or the platform gives it, and no others (fields and checkboxes have no
  hover: a field's edge already says where it is). The states are: hover; active (pressed); focus-visible (a ring in
  `ring` at `ring-width` and `ring-opacity`, the edge turning `ring`, drawn outside the control); disabled
  (`opacity-disabled`, no hover or press, the arrow cursor kept); read-only (focusable and selectable, drawn as at
  rest); invalid (a `destructive` edge, and the `ring-invalid` ring only while focused); loading (not pressable, not
  dimmed).
- On a touch screen focus changes only the edge color, without a ring (as iOS's and Android's own controls).
- A press goes through `foundation`'s `press(scale)`: 0.97 for buttons, 0.9 for small controls such as checkboxes and
  radios; no scale with reduced motion. Color transitions take `duration`.
- Fields (`Input`, `Textarea`, and later a `Select`'s trigger, `NumberInput`, `Combobox`) share `foundation`'s field
  look (edge, fill, shadow, focus, invalid, disabled) on the `input-*` tokens, rather than each drawing its own.
- Icons: sized by `icon-size-*` with the control's `size`, from the context when not given; colored as their text,
  `foreground` outside the library's parts (below).
- A part that colors its own text provides that color to what is inside it (`provide_text_color()`, as it provides
  the icon size): icons and spinners follow it, and outside the library's parts fall back to `foreground` (NSH gives
  a plain view black text in both appearances).
- "As wide as its content" is `w-fit` (an internal `app_class`), never `align-self`, which would override the row's
  alignment around it.
- Styles NSH cannot bind (font weight, underline, cursor) are read once when the component is made; the spec's NSH
  gaps section names them.
- Shadows come from `foundation`'s `shadow()` as the root's inline style; the app's `style` is merged after it.
- Line heights come from `foundation`'s `line_height(size)`; components do not set their own.
- Touch screens: controls take `control-height-touch-*`; one smaller than `touch-target` gets a hit slop, not a larger
  look. A checkbox-like control with a label is hit through its label and gets none. A field's `md` and `lg` text is
  16 on a touch screen (iOS Safari zooms the page into a field with smaller text).
- A part of an `nsh/base` component whose Rust style is built once (`popupStyle`, `itemStyle`, a part's
  `*_style`) gets the library's look as a class generated from the resolved tokens, placed before the app's class;
  the app's `style` is merged last. So a token change or a `ThemeScope` reaches it, and the app still overrides it.
- Any look the app must be able to override with a class is such a generated class, never a typed style: a typed
  style beats every class, the app's included. `foundation::class::Classes` builds them from the resolved tokens (a
  color with its `dark:` value, a state as a variant: `hover:`, `selected:`), the app's class appended last.
- A rule about a child's place uses NSH's `first:` / `last:` variants on the child (`last:border-b-0`, `first:ml-0`),
  not a count the parent keeps of its children: the variants follow children as they are added and removed.
- Slot content (`trigger`, `overflow`, `fallback`) is made where the app writes it and does not see the context a
  component provides in its body. What depends on that context (a group's size) is drawn on the slot's box, which the
  component places, not by the content.
- A part of an `nsh/base` component that the app must be able to restyle (`<part>Style` from Rust takes only fixed
  styles there) is drawn by the library, on `nsh/base`'s behavior, rather than styled through `nsh/base`'s part.
- Decorative components (`Separator`, `Skeleton`) are hidden from accessibility: `accessible(false)` and
  `accessibility_elements_hidden(true)`.
- Both appearances must look right; dark has its own treatment, not an inverted light (an outline button in dark:
  an `input` edge on `input` at 30%).

## Overlays

Every overlay follows these rules; [dialog.md](components/dialog.md) has the full version.

- **Two kinds of layers.** A modal layer (`Dialog`, `AlertDialog`, later `Sheet`, `Drawer`) is `nsh/base`'s
  `Dialog` with the library's own motion and closing: it dims and blocks the page, traps the focus and gives it back.
  A popup layer (`Popover`, `Tooltip`, `DropdownMenu`, `Select`'s list, later `Combobox`, `ContextMenu`) is the
  `nsh/base` component's overlay: it does not trap the focus, and on the desktops a press outside closes it and goes
  on.
- **The trigger is a `trigger` slot** of the component, not a `…Trigger` part (an NSH child component cannot fill
  its parent's slot). A trigger shows that its layer is open (`Button`'s `expanded:` look, a field's focus edge).
- **Stacking**: the page and its popups; each modal, the newest on top; the popups opened inside a modal; toasts,
  drawn in the topmost modal of this library.
- **Dismissal**: Escape and back go to the topmost layer; a popup closes on a press outside, a modal on its overlay
  (never an `AlertDialog`). Every request is reported once as `onOpenChange({ open, reason })` with `nsh/base`'s
  reasons, before the state changes; a controlled layer waits for the app's `open`.
- **The theme scope** of an overlay's content is where the app put the component: a `ThemeScope` around a
  `Dialog`, `Popover` or trigger reaches its layer.
- **Text styles**: each surface resets the inherited ones (as `Tooltip`'s bubble does) and sets its own.
- **Surfaces**: modal panels take `radius-lg` and shadow tier 4; popup surfaces take `foundation::popup::surface`
  (`popover` fill, the `popover-*` tokens: `radius-md`, shadow tier 3), which every popup shares; menu-like lists
  (`DropdownMenu`, `Select`, later `Combobox`, `ContextMenu`, `Command`) share `foundation::menu` and the `menu-*`
  tokens for their rows. A surface is built when it opens, so a theme change while it is open shows at the next open.
- **Motion**: popups fade and zoom from 95% and slide 8 from the trigger's side over `duration`; modals fade (and
  zoom their panel) over `dialog-duration`; nothing moves with reduced motion; `<component>-animate` 0 turns it off.
- **Touch screens**: a layer keeps its form unless the platform's habit differs and `nsh/base` already does it
  (menus and `Select`'s list become bottom sheets; dialogs stay centered panels; toasts span a narrow window).

## What each component delivers

1. `docs/components/<name>.md`, starting with a Status line, with these sections as needed, in this order: Usage, When not to use, Built on, API
   (props, events, parts), Variants and sizes, States, Pointer and touch, Accessibility, Tokens, Examples (TSX and
   Rust), NSH gaps, Decisions.
2. `src/components/<name>.rs`: the component and its `tokens!`.
3. A gallery page: variants × sizes × states, in light and dark, with the component's tokens editable on the page.
4. Tests, in `tests/<component>.rs`: headless tests of behavior (press, disabled, controlled values, keys, accessibility roles and states, pointer and touch sizes, a scoped token
   reaching the look); the
   token table and `themes/default.json` agree with the code.
5. Screenshots for review: web and macOS, light and dark, in the pull request.
6. A row in the component index (`docs/components/README.md`).

## Out of scope

- Domain components (quotes, trading, accounts) are not part of this library.
- No wrapper that only renames a component; no variant for one screen (use `class` or scoped tokens).
- No component checks for a particular product or brand.
