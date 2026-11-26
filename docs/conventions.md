# Component conventions

Every component follows these conventions. Change this document before changing a convention.

## What decides what

| Aspect | Decided by |
|---|---|
| The look (colors, radii, type, shadows, spacing) | shadcn/ui new-york v4, its values as tokens ([tokens.md](tokens.md)) |
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
  (`nsh/base`, NSH's `docs/dev/api-naming.md`), not as Radix does: `onChange({ checked })`, `onChange({ value })`,
  `onOpenChange({ open })`, `onPress`. Apps use this library beside `nsh/base`, so one thing has one name.
- **Exports named like NSH's** (`Input`, `Button`) keep shadcn's names; an app using both renames one
  (`import { Input as BaseInput } from "nsh/base"`).
- **`variant` and `size`**: variants have shadcn's names (`default`, `secondary`, `outline`, `ghost`, `destructive`,
  `link`; only those a component needs). `size` is `foundation`'s `Size` (`xs`, `sm`, `md`, `lg`; `md` by default),
  which changes the height, type, icon and padding together; there is no separate height prop.
- **Styled parts**: the root takes `class` and `style` (merged last, over the component's own). Each meaningful part
  takes `<part>Class` and `<part>Style` (`labelClass`, `iconStyle`), named in the spec. The text a string child
  becomes is a part named `label`.
- **Group names** a component uses inside start with `nlb-` (`group/nlb-checkbox`), leaving other names to the app
  and to `nsh/base`.
- **Composition over switches**: structure is made of parts (`Card`, `CardHeader`, `CardTitle`, `CardContent`), not
  of boolean props that show or hide pieces.
- **Content is slots and children**, not fixed markup.
- **A component's own text** (accessibility labels, "Loading") is English by default; a host translates it with
  `labels::set_translator`.
- **A new component, or a new or changed public API, is reviewed**: its spec is written before its code.

## Drawing

- Colors, radii and sizes are read from the `theme()` handle. A component declares its tokens with `tokens!` in its
  file, their defaults derived from semantic tokens. A color literal in `src/components/` fails a test.
- States: a component has the states shadcn or the platform gives it, and no others (fields and checkboxes have no
  hover, as in shadcn). The states are: hover; active (pressed); focus-visible (a ring in `ring` at `ring-width` and
  `ring-opacity`, the edge turning `ring`, drawn outside the control); disabled (`opacity-disabled`, no hover or press,
  the arrow cursor kept); read-only (focusable and selectable, drawn as at rest); invalid (a `destructive` edge, and
  the `ring-invalid` ring only while focused); loading (not pressable, not dimmed).
- On a touch screen focus changes only the edge color, without a ring (as iOS's and Android's own controls).
- A press goes through `foundation`'s `press(scale)`: 0.97 for buttons, 0.9 for small controls such as checkboxes and
  radios; no scale with reduced motion. Color transitions take `duration`.
- Fields (`Input`, `Textarea`, and later a `Select`'s trigger, `NumberInput`, `Combobox`) share `foundation`'s field
  look (edge, fill, shadow, focus, invalid, disabled) on the `input-*` tokens, rather than each drawing its own.
- Icons: sized by `icon-size-*` with the control's `size`, from the context when not given; colored as their text.
- Line heights come from `foundation`'s `line_height(size)`; components do not set their own.
- Touch screens: controls take `control-height-touch-*`; one smaller than `touch-target` gets a hit slop, not a larger
  look. A checkbox-like control with a label is hit through its label and gets none. A field's `md` and `lg` text is
  16 on a touch screen (iOS Safari zooms the page into a field with smaller text).
- Both appearances must look right; dark follows shadcn's dark treatment (an outline button in dark: an `input`
  edge on `input` at 30%).

## What each component delivers

1. `docs/components/<name>.md`, with these sections as needed, in this order: Usage, When not to use, Built on, API
   (props, events, parts), Variants and sizes, States, Pointer and touch, Accessibility, Tokens, Examples (TSX and
   Rust), NSH gaps, Decisions.
2. `src/components/<name>.rs`: the component and its `tokens!`.
3. A gallery page: variants × sizes × states, in light and dark, with the component's tokens editable on the page.
4. Tests: headless tests of behavior (press, disabled, controlled values, keys, accessibility roles and states); the
   token table and `themes/default.json` agree with the code.
5. Screenshots for review: web and macOS, light and dark, in the pull request.
6. A row in the README's component index.

## Out of scope

- Domain components (quotes, trading, accounts) are not part of this library.
- No wrapper that only renames a component; no variant for one screen (use `class` or scoped tokens).
- No component checks for a particular product or brand.
