# Textarea

Status: implemented (P1). Drawn by foundation's **field look** with [Input](input.md)'s `input-*` tokens; this spec
covers only what is particular to several lines.

## Usage

Multi-line text entry: notes, descriptions, feedback, a message draft. The look:
Input's frame, growing with its content (`field-sizing-content`) from a least height of 64.

## When not to use

| Need | Use instead |
|---|---|
| One line is enough (a name, an email, a search term) | [Input](input.md) |
| Rich text, Markdown editing, code | `nsh/base`'s `RichEditor` / `Editor` |
| A chat composer (a send button, attachments, Return sends) | A Textarea in an [InputGroup](input-group.md) with a `blockEnd` toolbar (and `submitBehavior="submit"`); a later composer block (`src/blocks/`) for more |
| A character count, a description, an error line | [Field](field.md) around it |

## Built on

As Input: NSH's host element `Input`, with `multiline` on (natively UITextView, NSTextView, a multi-line EditText,
`<textarea>`, ...). The component's root is that element. What the host does and this library does not reimplement:
[Input's Built on](input.md#built-on); with several lines, also:

- Content taller than the box scrolls inside it (natively), and the wheel goes to the field first.
- The host element is measured by its text as it is laid out: without a height of its own it is as tall as its text,
  in the same layout as the edit. This library grows the field that way ([States](#states)), not through
  `onContentSizeChange`, which comes after the layout.

## API

### Props

These are [Input's props](input.md#props), with the same meaning, type and default: `value`, `defaultValue`,
`placeholder`, `size`, `disabled`, `readonly`, `invalid`, `autoCapitalize`, `autoCorrect`, `maxLength`, `autoFocus`,
`accessibilityLabel`, `class`, `style`.

Left out: `masked`, `keyboardType`, `returnKeyType`, `autoComplete`, `selectTextOnFocus` (of no use for several lines, or
inconsistent across platforms there).

Particular to several lines:

| TSX | Rust | Type | Default | Notes |
|---|---|---|---|---|
| `submitBehavior` | `submit_behavior` | NSH's `SubmitBehavior`: `"newline"` / `"submit"` / `"blurAndSubmit"` | `"newline"` | What Return does, passed through (the host's rules) |

No `rows` / `minRows` / `maxRows`: only `min-h-16` and the content's height. The app gives a
maximum height with `class` (`max-h-40`) or `style` (`maxHeight`), past which the field scrolls inside. For a fixed
height write `h-32`: the app's `class` / `style` merge last, over the height the component computes, which turns growing
off.

### Events

As Input: `onChange({ value })`, `onSubmit({ value })` (only when `submitBehavior` is not `"newline"`), `onFocus`,
`onBlur`. In Rust, `TextareaEvent`'s `Change(String)` / `Submit(String)` / `Focus` / `Blur`.

### Ref

As Input: `TextareaRef` / `ComponentRef<TextareaApi>`, with `focus()`, `blur()`, `setSelection(start, end)`,
`insertText(text)`.

### Inside a `Field`

As Input ([Input's](input.md#inside-a-field)): the field's `invalid`, `disabled` and description or error; its
`FieldLabel` focuses it.

### Inside an `InputGroup`

As Input ([Input's](input.md#inside-an-inputgroup)): the group draws the frame and sets the size. In a group with a
`blockStart` or `blockEnd` addon (a header, a toolbar) it spans the column and keeps growing with its text.

### Parts

The root alone, no `<part>Class` / `<part>Style` (as Input).

## Variants and sizes

No variants. All four sizes, which change **only the text size, the line height and the horizontal padding**, so an
Input and a Textarea in one form line up their text; the height follows the content, not `size`:

| `size` | Text (pointer / touch) | Horizontal padding | Vertical padding |
|---|---|---|---|
| `xs` | 12 / 14 | 8 | 8 |
| `sm` | 13 / 14 | 10 | 8 |
| `md` | 14 / 16 | 12 | 8 |
| `lg` | 14 / 16 | 12 | 8 |

- The text size and the horizontal padding are Input's `input-font-size-*` and `input-padding-x-*` (the text size per
  pointer / touch, as Input).
- The line height comes from foundation's text size → line height function (14 → 20, `text-sm`); the component
  sets none, and it is no token.
- The vertical padding is `textarea-padding-y` (8, `py-2`), the same at every size and on touch screens.

Why sizes: conventions.md makes sizes go through `Size`, and a 14-point Textarea next to
a `size="sm"` Input in a form would not line up. The least height `textarea-min-height` does not change with the size
(one `min-h-16`).

Width: `w-full`.

## States

The edge, fill, shadow, focus ring, invalid, readonly, disabled, transitions, selection and caret are **all
[Input's States](input.md#states)** (foundation's field look: no hover, the ring outside the box, the edge color alone on
touch), with the same `input-*` tokens and the same classes (`border-input bg-transparent shadow-xs
focus-visible:border-ring focus-visible:ring-[3px] focus-visible:ring-ring/50 aria-invalid:... disabled:opacity-50
dark:bg-input/30`).

Particular to several lines:

- **Height**: `max(textarea-min-height, text height + 2 × textarea-padding-y + 2 × input-border-width)`. The text
  height is what the host measures as it lays the field out (the placeholder's while empty, at least one line); the
  component sets a least height and the paddings, no height. The app's `max-h-*` / `maxHeight` caps it, past which the
  field scrolls inside; the app's `h-*` / `height` overrides the whole rule.
- **Text starts at the top left** (not centered vertically).
- A height change is **not animated** (CSS's `field-sizing-content` is instant too), so reduced motion changes nothing.
- Scroll bars: the host's default (overlay bars that show while scrolling on most platforms); this library sets none.

## Pointer and touch

| | Pointer | Touch |
|---|---|---|
| Text | 12 / 13 / 14 / 14 | 14 / 14 / 16 / 16 (as Input) |
| Least height | 64 (`textarea-min-height`) | 64, above `touch-target`, so no hit slop |
| Focus | The edge in `ring` and a 3pt ring | The edge color only, no ring |
| Vertical padding | 8 | 8 (not larger) |
| Keyboard | — | The platform labels the Return key per `submitBehavior`; a field in a scroll view moves clear of the software keyboard natively |

## Accessibility

As [Input's](input.md#accessibility), with a multi-line text field's role. Keys particular to several lines:

- Return inserts a line break; with `submitBehavior="submit"` it submits, and Shift / Option / Alt+Return still break the
  line; a Return that commits an input method's composition never submits.

## Tokens

Every field look token is Input's (`input-radius`, `input-border*`, `input-background`, `input-foreground`,
`input-placeholder-foreground`, `input-ring*`, `input-shadow`, `input-font-size-*`, `input-padding-x-*`):
[Input's Tokens](input.md#tokens) (decided in review: shared, no `textarea-*` copies). Textarea's own:

Generated from the code's declarations (`NSH_LONGBRIDGE_WRITE=1 cargo test --test tokens`); a test keeps it current.

<!-- tokens:textarea -->
| Token | Type | Derived default | Value |
|---|---|---|---|
| `textarea-min-height` | length | `spacing × 16` | 64 |
| `textarea-padding-y` | length | `spacing × 2` | 8 |
<!-- /tokens -->

`textarea-min-height` is `min-h-16`, `textarea-padding-y` is `py-2`.


## Examples

TSX:

```tsx
import { signal, View } from "nsh";
import { Label, Textarea, type TextareaRef } from "nsh-longbridge";

function Feedback() {
  const text = signal("");
  const field = signal<TextareaRef>();
  return (
    <View class="gap-2">
      <Label htmlFor={field()}>Your message</Label>
      <Textarea
        ref={field}
        class="max-h-40"
        value={text()}
        onChange={({ value }) => text.set(value)}
        placeholder="Type your message here."
        maxLength={500}
        accessibilityLabel="Your message"
      />
    </View>
  );
}

// Return sends, Shift+Return breaks the line
<Textarea submitBehavior="submit" onSubmit={({ value }) => send(value)} />
// A fixed height, disabled
<Textarea class="h-32" disabled value={log()} />
```

Rust:

```rust
use nsh_core::prelude::*;
use nsh_longbridge::{Textarea, label, textarea};

fn feedback() -> impl IntoElement {
    let text = signal(String::new());
    let field = component_ref::<Textarea>();
    view()
        .gap_2()
        .child(label().html_for(field).child("Your message"))
        .child(
            textarea()
                .component_ref(&field)
                .max_h(px(160.0))
                .value(move || Some(text.get().into()))
                .on_change(move |value: &String| text.set(value.clone()))
                .placeholder("Type your message here.")
                .max_length(500)
                .accessibility_label("Your message"),
        )
}
```

## NSH gaps

G1 (the app names the field for screen readers) and G4 (the selection) are [Input's](input.md#nsh-gaps); a ring clipped
at a container's edge is handled as under [Input's States](input.md#states). With several lines, also:

- **G8 Growing lags a frame** (checked when building: it does not). `onContentSizeChange` is reported after layout, so
  growing with it would take a second layout. The host input is measured by its text in the core's layout, so the
  component sets no height and the field grows in the layout of the edit; a headless test types line breaks and
  checks the height right after each. No line is kept in reserve.
- **G9 Growing on the web.** The web's `<textarea>` is laid out by the core's measurement too; the web screenshots
  check that the browser's text fits the height the core gives it.

## Decisions

Settled in the 2026-11-26 review:

- Textarea has `size`, which changes only the text size, the line height and the horizontal padding
  ([Variants and sizes](#variants-and-sizes)).
- The vertical padding is not larger on touch screens.
- It shares the `input-*` tokens; the touch text sizes, the focus and the absent hover are Input's.
