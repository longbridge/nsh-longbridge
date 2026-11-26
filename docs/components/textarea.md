# Textarea

> Status: API spec (P1, reviewed; revised after the 2026-11-26 review). Drawn by foundation's **field look** with
> [Input](input.md)'s `input-*` tokens; this spec covers only what is particular to several lines.

## Usage

Multi-line text entry: notes, descriptions, feedback, a message draft. The look is shadcn/ui new-york v4's `Textarea`:
Input's frame, growing with its content (shadcn's `field-sizing-content`) from a least height of 64.

## When not to use

| Need | Use instead |
|---|---|
| One line is enough (a name, an email, a search term) | [Input](input.md) |
| Rich text, Markdown editing, code | `nsh/base`'s `RichEditor` / `Editor` |
| A chat composer (a send button, attachments, Return sends) | A later composer block (`src/blocks/`), not in P1; until then a Textarea with `submitBehavior="submit"` |
| A character count, a description, an error line | Form / Field (P3) |

## Built on

As Input: NSH's host element `Input`, with `multiline` on (natively UITextView, NSTextView, a multi-line EditText,
`<textarea>`, ...). The component's root is that element. What the host does and this library does not reimplement:
[Input's Built on](input.md#built-on); with several lines, also:

- Content taller than the box scrolls inside it (natively), and the wheel goes to the field first.
- `onContentSizeChange` reports the content's size after each change of the text or the width; this library grows the
  field with it ([States](#states)).

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

No `rows` / `minRows` / `maxRows`: shadcn uses no `rows`, only `min-h-16` and the content's height. The app gives a
maximum height with `class` (`max-h-40`) or `style` (`maxHeight`), past which the field scrolls inside. For a fixed
height write `h-32`: the app's `class` / `style` merge last, over the height the component computes, which turns growing
off.

### Events

As Input: `onChange({ value })`, `onSubmit({ value })` (only when `submitBehavior` is not `"newline"`), `onFocus`,
`onBlur`. In Rust, `TextareaEvent`'s `Change(String)` / `Submit(String)` / `Focus` / `Blur`.

### Ref

As Input: `TextareaRef` / `ComponentRef<TextareaApi>`, with `focus()`, `blur()`, `setSelection(start, end)`,
`insertText(text)`.

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
- The line height comes from foundation's text size → line height function (14 → 20, shadcn's `text-sm`); the component
  sets none, and it is no token.
- The vertical padding is `textarea-padding-y` (8, shadcn's `py-2`), the same at every size and on touch screens.

Unlike shadcn, whose Textarea has no sizes: conventions.md makes sizes go through `Size`, and a 14-point Textarea next to
a `size="sm"` Input in a form would not line up. The least height `textarea-min-height` does not change with the size
(shadcn has one `min-h-16`).

Width: `w-full`, as shadcn.

## States

The edge, fill, shadow, focus ring, invalid, readonly, disabled, transitions, selection and caret are **all
[Input's States](input.md#states)** (foundation's field look: no hover, the ring outside the box, the edge color alone on
touch), with the same `input-*` tokens and the same shadcn classes (`border-input bg-transparent shadow-xs
focus-visible:border-ring focus-visible:ring-[3px] focus-visible:ring-ring/50 aria-invalid:... disabled:opacity-50
dark:bg-input/30`).

Particular to several lines:

- **Height**: `max(textarea-min-height, content height + 2 × textarea-padding-y + 2 × input-border-width)`. The content
  height is the host's `onContentSizeChange` (one line while empty). The app's `max-h-*` / `maxHeight` caps it, past
  which the field scrolls inside; the app's `h-*` / `height` overrides the whole rule.
- **Text starts at the top left** (not centered vertically), as shadcn.
- A height change is **not animated** (shadcn's `field-sizing-content` is instant too), so reduced motion changes nothing.
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

| Token | Type | Derived default | Light / dark |
|---|---|---|---|
| `textarea-min-height` | length | `spacing` × 16 (64, shadcn's `min-h-16`) | — |
| `textarea-padding-y` | length | `spacing` × 2 (8, shadcn's `py-2`) | — |

```rust
tokens! {
    /// The textarea's least height; it grows with its content from there.
    TEXTAREA_MIN_HEIGHT: Length = "textarea-min-height" => |t| t.length(SPACING) * 16.0,
    /// The space above and below the text.
    TEXTAREA_PADDING_Y: Length = "textarea-padding-y" => |t| t.length(SPACING) * 2.0,
}
```

## Examples

TSX:

```tsx
import { signal, View } from "nsh";
import { Label, Textarea, type TextareaRef } from "nsh-longbridge";

function Feedback() {
  const text = signal("");
  let field: TextareaRef | undefined;
  return (
    <View class="gap-2">
      <Label htmlFor={field}>Your message</Label>
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
use nsh::prelude::*;
use nsh_longbridge::{label, textarea};

fn feedback() -> impl IntoElement {
    let text = signal(String::new());
    let field = ComponentRef::new();
    view()
        .gap_2()
        .child(label().html_for(field).child("Your message"))
        .child(
            textarea()
                .component_ref(field)
                .max_h(px(160.0))
                .value(move || text.get())
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

- **G8 Growing lags a frame.** `onContentSizeChange` is reported after layout, and the component's new height takes
  another layout, so on a line break the text may scroll in the old height before the field grows. Workaround: check it
  first in headless tests and on macOS; if it shows, keep one line in reserve (content height + one line, up to the cap),
  which needs a decision on whether a blank line below the content is acceptable. The gallery has a repeated-Return case
  to check it.
- **G9 Growing on the web (to verify).** Growing always goes through `onContentSizeChange`, never CSS's `field-sizing`
  (no other platform has it); screenshots on the web confirm that the content height NSH reports matches the browser's
  layout.

## Decisions

Settled in the 2026-11-26 review:

- Textarea has `size`, which changes only the text size, the line height and the horizontal padding
  ([Variants and sizes](#variants-and-sizes)).
- The vertical padding is not larger on touch screens.
- It shares the `input-*` tokens; the touch text sizes, the focus and the absent hover are Input's.
