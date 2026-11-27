# Label

Status: implemented (P1). Used in forms with [Input](input.md) / [Textarea](textarea.md); a Label draws no frame and
follows only its field's `disabled`. The `label-*` tokens are also read by the `label` parts of Checkbox, Switch and
RadioGroup ([Tokens](#tokens)).

## Usage

The text naming a control: a form field's name, the words beside a Checkbox or a Switch. The look: 14, medium weight, a
line height of 1, in a row with icons or controls. Pressing a Label moves the focus to the field it names (`htmlFor`).

## When not to use

| Need | Use instead |
|---|---|
| The words beside a Checkbox, a Switch or a RadioGroup item | Their children (the `label` part of those components, in their specs): pressing the words toggles them and screen readers get the name; no separate Label beside them |
| A field's description, error, required mark, count | Form / Field (P3: FieldLabel, FieldDescription, FieldError); `Text` in P1 |
| Body text, headings, group titles | `Text` with Tailwind classes (`text-sm`, `font-semibold`, ...) |
| A small status or count | Badge |

## Built on

No `nsh/base` component and no host element: a Label is a `view` in a row around the app's children, written in this
library. It does what a web `<label>` does, and only that: pressing it focuses the control its `for` names, and a
double click selects no text:

- With `htmlFor` naming a control, pressing the Label focuses a text control (an `Input`, a `Textarea`, a `Select`'s
  trigger: as its ref's `focus()`) and presses a checkbox-like one (a `Checkbox` or a `Switch` toggles and reports
  `onChange`, as a browser's `<label>` does); not while the Label or the control is disabled.
- Its text is not selectable (`select-none`), so a double click selects nothing.

## API

### Props

| TSX | Rust | Type | Default | Notes |
|---|---|---|---|---|
| `children` | `.child(..)` / `.children(..)` | elements / text | — | The label: text, possibly with an icon or a Badge (spaced by `gap`) |
| `htmlFor` | `html_for` | TSX: `InputRef \| TextareaRef \| SelectRef \| CheckboxRef \| SwitchRef`; Rust: `FieldRef`, from a `ComponentRef` of one of those | — | What a press focuses or presses. Named as React's `htmlFor`. A Label before its control gets the ref through a signal (below). Inside a [Field](field.md), `FieldLabel` names the field's control without it |
| `disabled` | `disabled` | `boolean` | `false` | Dimmed (`opacity-disabled`), presses do nothing. Following the field would need CSS's `peer-disabled` / `group-data-[disabled]`; NSH has no `peer-*`, so the app passes it (G3) |
| `class` | `class` | `string` | — | The root's classes, merged last |
| `style` | `style` | `StyleProp<ViewStyle>` / `Style` | — | The root's style; a text `color` or `fontSize` here reaches the text inside |

**A ref made after the Label.** TSX makes elements in order, and reads a plain variable once: `htmlFor={field}` on a
Label above `<Input ref={field}>` reads it before the Input sets it, and stays empty. Keep the ref in a signal, which
`ref` fills and `htmlFor={field()}` follows:

```tsx
const field = signal<InputRef>();
<Label htmlFor={field()}>Email</Label>
<Input ref={field} accessibilityLabel="Email" />
```

A Label after its field may take a plain variable. Rust has no such order: a `ComponentRef` is found when the Label is
pressed.

No `size`, `variant`, `required` or `invalid`: a label is text, and those belong to the field. A required mark and red
text on errors belong to Field (P3; its label turns `text-destructive` when invalid).

### Events

None. (Focusing or pressing on a press is `htmlFor`'s built-in behavior and reports nothing; an app that wants something else uses
a `View` with `onPress`.)

### Parts

The root alone (a `view`, a row). The text styles (size, weight, color, line height) are set on the root and inherited
by the `Text` inside, so `class` / `style` on the root restyle the text; there is no `textClass`.

## Variants and sizes

No variants. **No `size`**: one size, `text-sm`, over fields of every size; a small label beside a
small field uses `class="text-xs"`.

## States

| State | Look | Classes |
|---|---|---|
| Rest | Text `label-font-size` (14), weight `label-font-weight` (500), color `label-foreground` (`foreground`), line height = text size (`leading-none`); a row, centered, children `label-gap` (8) apart; text not selectable | `flex items-center gap-2 text-sm leading-none font-medium select-none` |
| hover | Unchanged; the arrow cursor (not the hand: it is no link; gpui-kit's Design Guides) | none |
| active (pressed) | Unchanged, no press feedback (it is no button); the effect is the field taking the focus and showing its ring | none |
| focus-visible | Does not apply: a Label takes no focus | — |
| disabled | `opacity-disabled` (0.5) over all; a press focuses nothing | `peer-disabled:opacity-50 group-data-[disabled=true]:opacity-50 group-data-[disabled=true]:pointer-events-none` |
| invalid / loading | Do not apply ([API](#api)) | — |

`peer-disabled:cursor-not-allowed` is not copied: on desktops a disabled control's label keeps the arrow cursor (as a
disabled Input).

A line height of 1 needs checking with CJK text: under `leading-none`, CJK glyphs and Latin descenders (g, y) may be
clipped on some platforms. The gallery shows a Chinese label and a `gy` label, screenshotted in light and dark; if they
clip, the Label takes the line height of foundation's text size → line height function, and this spec records the
change. (A line height equal to the text size is a deliberate `leading-none`, so the Label does not
use that function: the one exception to "components set no line heights of their own".)

## Pointer and touch

| | Pointer | Touch |
|---|---|---|
| Text | 14 | 14 (not the fields' 16 on touch: a label is `text-sm` on narrow screens too; 16 only keeps a browser from zooming into a field) |
| Hit area | The label's box | The label's box, **not padded to 44**: it is no control of its own, its field already meets the target, and padding would take presses from the fields above and below |
| Press | A full click (on release) focuses the field, as a browser's `<label>` click | The same (on a tap's release), showing the software keyboard; a scrolling drag does not trigger it |

## Accessibility

- **No accessibility relation.** NSH has no labelledby ([Input's NSH gaps](input.md#nsh-gaps), G1), so a Label cannot be
  its field's screen reader name. The Label is read as plain text, and the field's name is the field's
  `accessibilityLabel`.
- The Label is not hidden from screen readers (see [Decisions](#decisions)).
- Not a Tab stop and not a button: no `button` role, never reached by the keyboard (the field it focuses is in the Tab
  order itself).

## Tokens

Generated from the code's declarations (`NSH_LONGBRIDGE_WRITE=1 cargo test --test tokens`); a test keeps it current.
Values are those of `nsh/base`'s default theme.

<!-- tokens:label -->
| Token | Type | Derived default | Value |
|---|---|---|---|
| `label-font-size` | length | `font-size-md` | 14 |
| `label-font-weight` | number | `font-weight-medium` | 500 |
| `label-foreground` | color | `foreground` |  |
| `label-gap` | length | `spacing × 2` | 8 |
<!-- /tokens -->

`label-font-size` follows `font-size-md` rather than Tailwind's fixed `text-sm` (14), so a theme that sets the
`font-size` seed to 13 for a compact density shrinks labels with their fields. Both default to 14, so nothing differs
visibly by default.

The `label` parts of the choice controls (Checkbox, Switch, RadioGroup items) read `label-foreground` and
`label-font-weight`, with their own `font-size-<size>` rather than `label-font-size` (they have `size`, a Label does
not). Changing `label-foreground` therefore changes those controls' label text too.

Disabled uses the semantic `opacity-disabled`, with no token of its own.

## Examples

TSX:

```tsx
import { signal, Text, View } from "nsh";
import { Badge, Checkbox, Input, type InputRef, Label } from "nsh-longbridge";

const name = signal<InputRef>();

<View class="gap-2">
  <Label htmlFor={name()}>Display name</Label>
  <Input ref={name} placeholder="Ada" accessibilityLabel="Display name" />
</View>

// With another element (Badge's API is its own spec's)
<Label htmlFor={name()}>
  API key <Badge variant="secondary">Beta</Badge>
</Label>

// Following the field's disabled state
<Label htmlFor={name()} disabled={locked()}>Display name</Label>
<Input ref={name} disabled={locked()} accessibilityLabel="Display name" />

// No Label beside a Checkbox: the words are its children
<Checkbox checked={terms()} onChange={({ checked }) => terms.set(checked)}>
  <Text>Accept terms and conditions</Text>
</Checkbox>
```

Rust:

```rust
use nsh_core::prelude::*;
use nsh_longbridge::{Input, input, label};

fn display_name(locked: Signal<bool>) -> impl IntoElement {
    let name = component_ref::<Input>();
    view()
        .gap_2()
        .child(label().html_for(name).disabled(locked).child("Display name"))
        .child(
            input()
                .component_ref(&name)
                .placeholder("Ada")
                .disabled(locked)
                .accessibility_label("Display name"),
        )
}
```

With NSH 0.7.0, write a string child inside `<Text>` for now ([Button's G0](button.md#nsh-gaps)).

(The Checkbox follows [checkbox.md](checkbox.md): its label is its children, its event NSH's `onChange({ checked })`.)

## NSH gaps

- **G1 No labelledby**: [Input's NSH gaps](input.md#nsh-gaps). To screen readers a Label is text.
- **G2 A component prop taking another component's ref** (verified when building). A TSX ref of a Rust component is
  an object carrying its host node as `node`; `htmlFor` takes it (`FieldRef`) and keeps the node. Rust cannot reach a
  component's methods from its node, so each control registers itself under its host while it is mounted
  (`foundation::field`'s `join`), with what a label's press does to it, and a press does that to the control found
  there ([Input's G2](input.md#nsh-gaps)). Rust converts a `ComponentRef` of each control, resolved on each press, so a
  ref put on the control after the Label is made works. In TSX a ref read before the control sets it is empty: the API
  above passes it through a signal.
- **G3 No `peer-*`.** NSH does not support `peer-*` variants, so a Label cannot learn its field's `disabled` by itself.
  Workaround: `disabled` is passed explicitly; inside a [Field](field.md), `FieldLabel` takes the field's `disabled`,
  which the control takes too, so the app writes it once.

## Decisions

Settled in the 2026-11-26 review:

- A Label is not hidden from screen readers: they read the label's text and then "Display name, text field", in exchange
  for a field that still has a name when the app forgets `accessibilityLabel`.
- `label-font-size` follows `font-size-md`, changing with the density seed.
- The choice controls' label parts read `label-foreground` and `label-font-weight` ([Tokens](#tokens)).
