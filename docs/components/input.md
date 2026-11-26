# Input

> Status: API spec (P1, reviewed; revised after the 2026-11-26 review). Drawn by foundation's **field look** (built in P0:
> edge, fill, shadow, focus, invalid, readonly, disabled), which reads the `input-*` tokens under [Tokens](#tokens).
> [Textarea](textarea.md) and the later Select trigger, NumberInput and Combobox share it.

## Usage

Single-line text entry: a name, an email, a password, a search term, short text other than amounts. The look is
shadcn/ui new-york v4's `Input`.

## When not to use

| Need | Use instead |
|---|---|
| Several lines, notes, text with line breaks | [Textarea](textarea.md) |
| A number with steps, a range, arrow-key nudges | `nsh/base`'s `NumberInput` (wrapped by this library later, P3), not `Input` + `keyboardType="numeric"` with checks of the app's own |
| A choice from a fixed list | Select (P2) / Combobox (P3) |
| Icons, units or buttons before or after the field | InputGroup (shadcn v4's `input-group`, later); P1 gives `Input` no prefix / suffix |
| A verification code, one character per cell | `nsh/base`'s `OtpInput` |
| A field's label, description, error line | The label is a [Label](label.md); descriptions and errors wait for Form / Field (P3) |
| Picking a file (shadcn's `type="file"`) | Not supported: NSH has no file input; use the system's file dialog |

## Built on

NSH's host element `Input` (TSX imports it from `nsh/base`; Rust's is nsh-core's `element::input()`; natively AppKit,
UIKit, EditText, Win32, GTK or the browser's `<input>`). `nsh/base` has no styled text field component, and its
`NumberInput` uses this element directly; this library wraps it the same way. The component's root is the `input()`
element itself, with no view around it, so `class` / `style` land on the field, one element as in shadcn.

What the host element already does, and this library does **not** reimplement:

- Editing: the caret, the selection, input method composition (a `value` set during one is held natively and applied when
  it ends), undo, copy and paste, drag selection, the platform's edit menu, spell checking, autofill.
- Control: each native edit carries an edit count, and a `value` from the app applies only when based on the latest edit,
  so fast typing never jumps back.
- Keyboard, focus and screen readers: see [Accessibility](#accessibility).
- Software keyboards: `keyboardType`, `returnKeyType`, `autoCapitalize`, `autoCorrect`; a field in a scroll view moves
  clear of the keyboard natively.

This library adds only the look (tokens), `size`, `invalid`'s look and accessibility state, and `disabled` as a whole
(as `NumberInput` does: `readonly` and `disabled` set together, so the field neither edits nor takes focus, and
`disabled:` applies).

## API

### Props

| TSX | Rust (field / setter) | Type | Default | Notes |
|---|---|---|---|---|
| `value` | `value` | `string` / `Prop<Option<SharedString>>` | — | The controlled value: when set, the app owns it, `onChange` reports edits and the app sets them back. Unset, the field is uncontrolled and starts from `defaultValue` |
| `defaultValue` | `default_value` | `string` | `""` | The first value of an uncontrolled field |
| `placeholder` | `placeholder` | `string` | — | Shown while empty (the app's words) |
| `size` | `size` | `"xs" \| "sm" \| "md" \| "lg"` / `Size` | `"md"` | [Variants and sizes](#variants-and-sizes) |
| `disabled` | `disabled` | `boolean` | `false` | No editing, no focus, out of the Tab order; look under [States](#states) |
| `readonly` | `readonly` | `boolean` | `false` | Focusable, selectable and copyable, not editable; looks as at rest |
| `invalid` | `invalid` | `boolean` | `false` | The app's checks reject the value: a red edge, a red ring while focused; screen readers say invalid (`accessibilityState.invalid`, so `aria-invalid:` classes apply) |
| `masked` | `masked` | `boolean` | `false` | A password field (shadcn's `type="password"`) |
| `keyboardType` | `keyboard_type` | NSH's `KeyboardType` | `"default"` | The software keyboard (`email-address`, `url`, `phone-pad`, ...), passed through |
| `returnKeyType` | `return_key_type` | NSH's `ReturnKeyType` | the platform's | Passed through |
| `autoCapitalize` | `auto_capitalize` | NSH's `AutoCapitalize` | the platform's | Passed through |
| `autoCorrect` | `auto_correct` | `boolean` | the platform's | Passed through |
| `autoComplete` | `auto_complete` | NSH's `AutoFill` | — | What autofill offers (a user name, a password, an email, ...), passed through (the host's Rust setter is `auto_fill`) |
| `maxLength` | `max_length` | `number` | — | Passed through |
| `autoFocus` | `auto_focus` | `boolean` | `false` | Passed through; ignored while `disabled` |
| `selectTextOnFocus` | `select_text_on_focus` | `boolean` | `false` | Passed through |
| `accessibilityLabel` | `accessibility_label` | `string` | — | The name screen readers say. **Set it even with a visible label**: NSH has no labelledby between elements (G1 under [NSH gaps](#nsh-gaps)) |
| `class` | `class` | `string` | — | The root's (the field's) Tailwind classes, merged last, over the component's |
| `style` | `style` | `StyleProp<InputStyle>` / `InputPartStyle` | — | The root's inline style, over `class`; takes `placeholderColor`, `caretColor`, `selectionColor` |

No `type` prop: what shadcn tells apart with HTML's `type` is separate props in NSH (`masked`, `keyboardType`,
`autoComplete`), under NSH's names. No `variant`: shadcn's Input has one look.

### Events

| TSX | Rust (`InputEvent` variant / setter) | Data | Notes |
|---|---|---|---|
| `onChange` | `Change(String)` / `.on_change(move \|value: &String\| ..)` | `{ value }` | Every edit |
| `onSubmit` | `Submit(String)` / `.on_submit(..)` | `{ value }` | Return (per `returnKeyType`); a Return that commits an input method's composition does not count |
| `onFocus` | `Focus` / `.on_focus(..)` | — | The field took the keyboard focus |
| `onBlur` | `Blur` / `.on_blur(..)` | — | The field lost the keyboard focus |

The host element also has `onSelectionChange`, `onCaretMove`, `onPaste`, `onKeyPress`, `onCompositionChange` and more.
P1 does not pass them through (they are not part of shadcn's Input); an app that needs them uses `nsh/base`'s `Input`
directly for now (end of [Examples](#examples)). They are added later under their own names when a real need comes up.

### Ref

TSX: `let email: InputRef | undefined;` and `ref={email}` (NSH's ref form) give an `InputRef`; Rust: a
`ComponentRef<InputApi>`.

| Method | Notes |
|---|---|
| `focus()` | Takes the focus (showing the software keyboard); does nothing while `disabled` |
| `blur()` | Gives up the focus |
| `setSelection(start, end)` | The selection in UTF-16 indices, as the host's |
| `insertText(text)` | Inserts at the caret, as the host's |

### Parts

The root alone, no `<part>Class` / `<part>Style`. The placeholder, caret and selection colors are the field's style keys
(`placeholderColor` / `caretColor` / `selectionColor`, or the classes `placeholder:text-*` / `caret-*` / `selection:bg-*`),
set in the root's `class` / `style`.

## Variants and sizes

No variants. All four sizes; height, text size and horizontal padding change together:

| `size` | Height (pointer / touch) | Text (pointer / touch) | Horizontal padding | For |
|---|---|---|---|---|
| `xs` | 24 / 32 (`control-height-xs` / `-touch-xs`) | 12 / 14 | 8 (spacing × 2) | Editing in a table cell, a filter in a toolbar |
| `sm` | 28 / 36 | 13 / 14 | 10 (spacing × 2.5) | Compact forms, sidebars |
| `md` | 32 / 40 | 14 / 16 | 12 (spacing × 3) | The default |
| `lg` | 36 / 48 | 14 / 16 | 12 (spacing × 3) | With a pointer, shadcn's `h-9 px-3 md:text-sm` |

Unlike shadcn, which has one height (`h-9`, 36): the default `md` is 32, as tall as Button's `md` (medium desktop density,
gpui-kit's Design Guides). For shadcn's height use `size="lg"`, or `control-height: 36` in the theme. An Input and a
Button of the same `size` line up in a row. Text of 16 on touch screens at `md` / `lg` is conventions.md's rule (iOS
Safari zooms the page into fields with smaller text; shadcn's `text-base` on narrow screens too).

Width: `w-full min-w-0` (fills its parent, shrinks in a flex row), as shadcn; a fixed width goes in `class`. Every size
uses `input-radius` (Button's step; if P0's Button gives `xs` a smaller radius, Input follows the same rule). One line of
text is centered vertically in the fixed height, with no line height set.

## States

Foundation's **field look** draws the states (built in P0, conventions.md "Appearance"); this component hands it `size`,
`invalid`, `readonly`, `disabled` and the focus. The table is the result with the default tokens, and what the gallery
and the screenshots are checked against:

| State | Edge | Fill | Shadow / ring | Other | shadcn's classes |
|---|---|---|---|---|---|
| Rest | `input-border-width` (1) in `input-border` | `input-background` (light: transparent; dark: `input` at 30%) | `input-shadow` = 1 (`xs`) | Text `input-foreground`, placeholder `input-placeholder-foreground` | `border border-input bg-transparent shadow-xs dark:bg-input/30 placeholder:text-muted-foreground` |
| hover | Unchanged (no hover state, as shadcn) | Unchanged | Unchanged | The I-beam cursor (native) | none |
| focus-visible | `input-border-focus` (`ring`) | Unchanged | A ring of `input-ring-width` (3) in `input-ring` (`ring` at 50%), outside the box, over the `xs` shadow | Touch: the edge color only, no ring | `focus-visible:border-ring focus-visible:ring-[3px] focus-visible:ring-ring/50` |
| invalid | `input-border-invalid` (`destructive`) | Unchanged | No ring while unfocused | Screen readers say invalid | `aria-invalid:border-destructive` |
| invalid + focus-visible | `input-border-invalid` (the red edge wins over `ring`'s) | Unchanged | A ring of 3 in `input-ring-invalid` (= `ring-invalid`: `destructive` at 20%, 40% in dark); none on touch | — | `aria-invalid:ring-destructive/20 dark:aria-invalid:ring-destructive/40` |
| readonly | As at rest | As at rest | As focus-visible while focused | Focusable, its text selectable and copyable, not editable | none (shadcn has no such state) |
| disabled | As at rest | As at rest | No ring (no focus) | `opacity-disabled` (0.5) over all, the arrow cursor | `disabled:pointer-events-none disabled:cursor-not-allowed disabled:opacity-50` |

Notes for this component:

- Precedence, later wins: rest < focus-visible < invalid < disabled. Invalid changes only the colors of the edge and the
  ring; the ring still shows only while focused.
- An NSH field is `focus-visible` whenever it is focused (a click included), so clicking into a field with a pointer shows
  the ring, as in a browser.
- The ring is drawn outside the box and takes no layout. A field against the edge of a scroll view, a dialog's body or an
  `overflow-hidden` container gets its ring clipped (seen in real apps): the container leaves `ring-width` of padding,
  and the gallery has an edge case to check it.
- Transitions: the edge color, the fill and the shadow / ring transition over `duration`, none with reduced motion.
  shadcn transitions only `color, box-shadow` and switches the edge color at once; NSH groups transitions by colors and
  shadows, which cannot be split, so the edge and the ring transition together.
- Selection: shadcn's Input sets `selection:bg-primary selection:text-primary-foreground`. NSH sets only the selection's
  fill, not the selected text's color, and each platform adds its own alpha (Android 55%, Linux 30%, iOS a tint), so
  copying it would put dark text on a dark fill in light mode. **`selectionColor` is left unset: the platform's
  selection** (G4 under [NSH gaps](#nsh-gaps)). shadcn's Textarea sets none either.
- Caret: shadcn sets none; neither does this library (the platform's).
- Loading: a field has no loading state (shadcn has none); an asynchronous check's spinner belongs in an InputGroup
  (later).

## Pointer and touch

| | Pointer (desktops, the desktop web) | Touch |
|---|---|---|
| Height | `control-height-*`: 24 / 28 / 32 / 36 | `control-height-touch-*`: 32 / 36 / 40 / 48 |
| Text | 12 / 13 / 14 / 14 | 14 / 14 / 16 / 16 |
| Hit area | The field's box | Below `touch-target` (44) the field gets a vertical hit slop (`hitSlop` = (44 − height) / 2); the look is unchanged |
| Focus | The edge in `ring` and a 3pt ring | The edge color only, no ring (as iOS's and Android's system fields; an iPad with a hardware keyboard too) |
| Cursor | The I-beam (native); the arrow while disabled | — |

The pointer / touch values come from foundation's `pointer_touch` derivation. The software keyboard, moving clear of it
and the edit menu are native; this library does not touch them.

## Accessibility

All from the host element; this library adds only the `invalid` state and the `disabled` handling.

- Keyboard: an editable field is always a Tab stop; Return raises `onSubmit` per `submitBehavior`; editing keys belong to
  the field, and an ancestor's shortcuts never take them. A disabled field is no Tab stop.
- Focus: `focus-visible:` **always matches a focused editable field** (NSH's rule: a field shows its focus whether it was
  clicked or tabbed into), so the focus ring hangs on `focus-visible:` as in shadcn, with no keyboard / pointer split.
- Screen readers: the text field role, the value, `accessibilityLabel`; `invalid` maps on every platform (macOS
  `AXInvalid`, Windows `IsDataValidForForm`, AT-SPI's invalid entry, Android `setContentInvalid`, the web's
  `aria-invalid`, iOS in the value); `disabled` reads as dimmed.
- Name: `accessibilityLabel`, which the app sets even when a visible [Label](label.md) names the field (G1).

## Tokens

Named per tokens.md. These are every knob of the field look; Textarea reads the same ones (no `textarea-border` and the
like), so changing `input-border` in a theme changes both; to change one, use a `ThemeScope` or `class`. For a token
whose pointer and touch values differ, an app changes the touch value alone with `<name>-touch`.

| Token | Type | Derived default | Light / dark, pointer / touch |
|---|---|---|---|
| `input-radius` | length | `radius-md` (8) | — |
| `input-height-xs` / `-sm` / `-md` / `-lg` | length | `pointer_touch(control-height-xs, control-height-touch-xs)` and so on (24 / 28 / 32 / 36; touch 32 / 36 / 40 / 48) | per pointer / touch |
| `input-padding-x-xs` / `-sm` / `-md` / `-lg` | length | `spacing` × 2 / × 2.5 / × 3 / × 3 (8 / 10 / 12 / 12) | — |
| `input-font-size-xs` / `-sm` / `-md` / `-lg` | length | pointer `font-size-xs/sm/md/lg` (12 / 13 / 14 / 14); touch 14 / 14 / 16 / 16 | per pointer / touch |
| `input-border-width` | length | `border-width` (1) | — |
| `input-border` | color | `input` | as `input` |
| `input-border-focus` | color | `ring` | as `ring` |
| `input-border-invalid` | color | `destructive` | as `destructive` |
| `input-background` | color | `light_dark(transparent, input at 30%)` | per light / dark |
| `input-foreground` | color | `foreground` | as `foreground` |
| `input-placeholder-foreground` | color | `muted-foreground` | as `muted-foreground` |
| `input-ring` | color | `ring` × `ring-opacity` (50%) | as `ring` |
| `input-ring-invalid` | color | `ring-invalid` (`destructive` at 20%, 40% in dark) | as `ring-invalid` |
| `input-ring-width` | length | `ring-width` (3) | — |
| `input-shadow` | number | 1 (`xs`) | — |

The touch text sizes 14 / 16 are fixed values outside the `font-size` seed: tokens.md has no semantic token for touch
text sizes, so they live in these derivations; a theme changes them as `input-font-size-md-touch` and so on. Disabled
uses the semantic `opacity-disabled` directly, with no component token (as Button).

The Rust declarations (a sketch; names match the table one to one):

```rust
tokens! {
    /// The field's corner radius.
    INPUT_RADIUS: Length = "input-radius" => |t| t.length(RADIUS_MD),
    /// The field's height at `Size::Md`.
    INPUT_HEIGHT_MD: Length = "input-height-md" =>
        |t| t.pointer_touch(t.length(CONTROL_HEIGHT_MD), t.length(CONTROL_HEIGHT_TOUCH_MD)),
    /// The text size at `Size::Md`: 16 on touch screens, where smaller fields get zoomed.
    INPUT_FONT_SIZE_MD: Length = "input-font-size-md" => |t| t.pointer_touch(t.length(FONT_SIZE_MD), px(16.0)),
    /// The field's edge at rest.
    INPUT_BORDER: Color = "input-border" => |t| t.color(INPUT),
    /// The field's fill: none in light, `input` at 30% in dark.
    INPUT_BACKGROUND: Color = "input-background" => |t| t.light_dark(TRANSPARENT, t.color(INPUT).opacity(0.3)),
    /// The edge while the field has the keyboard focus.
    INPUT_BORDER_FOCUS: Color = "input-border-focus" => |t| t.color(RING),
    /// The edge while the app's checks reject the value; wins over the focus edge.
    INPUT_BORDER_INVALID: Color = "input-border-invalid" => |t| t.color(DESTRUCTIVE),
    /// The focus ring around the field.
    INPUT_RING: Color = "input-ring" => |t| t.color(RING).opacity(t.number(RING_OPACITY)),
    /// The focus ring while the value is invalid.
    INPUT_RING_INVALID: Color = "input-ring-invalid" => |t| t.color(RING_INVALID),
    // ...the rest as in the table
}
```

## Examples

TSX:

```tsx
import { signal, View } from "nsh";
import { Input, type InputRef, Label } from "nsh-longbridge";

function EmailField() {
  const email = signal("");
  const invalid = () => email().length > 0 && !email().includes("@");
  let field: InputRef | undefined;
  return (
    <View class="gap-2">
      <Label htmlFor={field}>Email</Label>
      <Input
        ref={field}
        value={email()}
        onChange={({ value }) => email.set(value)}
        placeholder="name@example.com"
        keyboardType="email-address"
        autoComplete="email"
        autoCapitalize="none"
        invalid={invalid()}
        accessibilityLabel="Email"
      />
    </View>
  );
}

// Sizes, a password, read-only, disabled
<Input size="sm" placeholder="Filter" />
<Input masked autoComplete="password" accessibilityLabel="Password" />
<Input readonly value={accountId()} accessibilityLabel="Account ID" />
<Input disabled value="Read from server" />
```

Rust:

```rust
use nsh::prelude::*;
use nsh_longbridge::{input, label, Size};

fn email_field() -> impl IntoElement {
    let email = signal(String::new());
    let invalid = move || email.with(|e| !e.is_empty() && !e.contains('@'));
    let field = ComponentRef::new();
    view()
        .gap_2()
        .child(label().html_for(field).child("Email"))
        .child(
            input()
                .component_ref(field)
                .value(move || email.get())
                .on_change(move |value: &String| email.set(value.clone()))
                .placeholder("name@example.com")
                .keyboard_type(KeyboardType::EmailAddress)
                .invalid(invalid)
                .accessibility_label("Email"),
        )
}

// A size
input().size(Size::Sm).placeholder("Filter")
```

An app that needs all of the host element (`onPaste`, `selectedRange`, `onCaretMove`, ...) uses `nsh/base`'s `Input`
with styles of its own for now; whether TSX exports the field look (`fieldLook`) is decided after P1 is built.

This library's `Input` and `nsh/base`'s `Input` share a name; an app using both aliases one, per conventions.md
(`import { Input as BaseInput } from "nsh/base"`). Likewise in Rust, `nsh_longbridge::input` and nsh-core's
`element::input` share a name: do not glob-import both.

## NSH gaps

- **G1 No labelledby between elements.** NSH has no relation by which one element names another for screen readers (its
  own API naming doc says so). Workaround: the app sets the Input's `accessibilityLabel` to the visible Label's text;
  the docs and the gallery examples all do. How Label handles it: [label.md](label.md).
- **G4 The selection takes only a fill.** The selected text's color cannot be set, the selection's alpha differs per
  platform, and iOS tints the selection and the caret with one color. Workaround: `selectionColor` is left unset (the
  platform's); an app that wants its brand color sets it in `style`.

## Decisions

Settled in the 2026-11-26 review:

- Text on touch screens: 16 at `md` / `lg`, 14 at `xs` / `sm`.
- The focus ring is drawn outside the box; on touch screens only the edge color changes, with no ring.
- Textarea shares the `input-*` tokens.
- Fields have no hover state.
- The field look is built in foundation (P0); this spec refers to it and keeps the token table.
- `readonly` is a state. TSX's `fieldLook` is not exported yet.
