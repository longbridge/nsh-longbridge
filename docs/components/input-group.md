# Input Group

Status: implemented (P3). Icons, words, key hints, spinners and buttons around a text field, inside one field look:
`InputGroup` draws the edge, the fill and the focus ring around this library's `Input` or `Textarea` and its addons
(`InputGroupAddon`, `InputGroupButton`, `InputGroupPasswordToggle`).

## Usage

- A search field: a leading search icon, a trailing clear button and a key hint (`⌘K`).
- A password with a button that shows it.
- A URL or an email with fixed words before or after it (`https://`, `@example.com`), an amount with its currency and
  unit (`$` … `USD`), a price filter with `$` before each bound.
- A field with an action on its edge: Copy, Send, Generate.
- A field checking itself (a user name's availability): a spinner while it checks, a check mark after.
- A message box: a `Textarea` with a toolbar under it (attach, a count of characters, Send) or a header over it.
- Search terms: input group, input addon, prefix, suffix, adornment, leading icon, trailing icon, inline button,
  password toggle, show password, reveal password, search field, clearable input, input with button.

## When not to use

| Need | Use instead |
|---|---|
| A field alone | [Input](input.md) / [Textarea](textarea.md) |
| A field and a button side by side, each with its own edge | A row (`flex-row gap-2`) of an `Input` and a `Button` |
| A choice from a list with an icon in the field | [Select](select.md) / [Combobox](combobox.md): they draw their own field; one inside a group draws a field in a field |
| A number with steps | `nsh/base`'s `NumberInput` (its own `prefix` / `suffix`) |
| The label, help and error of the field | [Field](field.md) around the group |
| Formatting what is typed (thousands separators, a mask) | The app's own code on `onChange`; an addon only shows fixed words |

## Built on

`nsh/base` has no input group; NSH has no `:has()` or ancestor selectors, so this library builds the group on
context (conventions, "Context instead of ancestor selectors"):

- `InputGroup` is a view that draws foundation's field look at rest (`field_rest`) and provides a group context: its
  `size`, and registers its control fills (the field, its focus, `invalid`, `disabled`), whether a block addon is in
  it, and whether its password is shown.
- The library's `Input` and `Textarea` read that context. Inside a group they draw no edge, fill, shadow or ring of
  their own (the group draws them), take the group's `size`, fill the room the addons leave, and report their focus,
  `invalid` and `disabled` to the group. Outside a group nothing changes. There is no `InputGroupInput` or
  `InputGroupTextarea`: they would only rename `Input` and `Textarea` (conventions, "Out of scope").
- The ring shows while **the field** has the keyboard focus, from the field's `onFocus` / `onBlur`, not NSH's
  `focus-within:` (which also matches a focused button inside): a focused `InputGroupButton` draws its own ring, so the
  two never show at once.
- Editing, focus, the caret and screen readers stay the host element's (`Input`'s "Built on"); buttons are
  `nsh/base`'s `Button` and `Toggle`.

### How it is put together

- The parts are laid out in the order they are written, as NSH has no `order`: write an `inlineStart` addon before
  the field and an `inlineEnd` addon after it. An addon's `align` says which edge it sits at (which side gets the
  field's padding); the field leaves room for an addon before or after it with NSH's `first:` / `last:` variants, so the
  group counts nothing.
- A `blockStart` or `blockEnd` addon turns the group into a column (header, field, footer) as wide as the group: the
  addon registers itself while mounted.
- A group has one field. A second `Input` in the same group draws as in a group but does not drive its look.

## API

### Components

| Component | TSX | Rust | Description |
|---|---|---|---|
| `InputGroup` | `<InputGroup>` | `input_group()` | The frame: the field look around its children |
| `InputGroupAddon` | `<InputGroupAddon>` | `input_group_addon()` | Icons, words, key caps, a spinner or buttons at one edge; a press on it (not on a button) focuses the field |
| `InputGroupButton` | `<InputGroupButton>` | `input_group_button()` | A small button sized to the field, in an addon |
| `InputGroupPasswordToggle` | `<InputGroupPasswordToggle>` | `input_group_password_toggle()` | The eye button that shows and hides the group's masked `Input` |

The field is this library's `Input` or `Textarea`, a child of the group.

### `InputGroup` props

| TSX | Rust | Type | Default | Description |
|---|---|---|---|---|
| `size` | `size` | `"xs" \| "sm" \| "md" \| "lg"` / `Size` | `"md"` | The size of the field, its addons, icons and buttons together ([Sizes](#sizes)). The field's own `size` is not read inside a group |
| `class` / `style` | `class` / `style` | | | The frame (the root): width (`max-w-80`), merged last |
| `children` | `.child(..)` | elements | | The field and its addons, in the order they show |

No `invalid`, `disabled` or `readonly`: the field's (or its `Field`'s) say it, and the group follows them. Set them
once, on the `Input`.

### `InputGroupAddon` props

| TSX | Rust | Type | Default | Description |
|---|---|---|---|---|
| `align` | `align` | `"inlineStart" \| "inlineEnd" \| "blockStart" \| "blockEnd"` / `InputGroupAlign` | `"inlineStart"` | Which edge it sits at: before or after the field in its row, or a row over or under it (the group becomes a column) |
| `class` / `style` | `class` / `style` | | | The addon, merged last |
| `children` | `.child(..)` | elements | | Its content: words, an `Icon`, a `Kbd`, a `Spinner`, `InputGroupButton`s. The addon sets the text style its words inherit; its children are not wrapped in a text part, so a button stays the addon's own child (its `first:` / `last:` place) |

Its words and icons are `input-group-addon-foreground` (muted) at the group's text and icon size, `input-group-gap`
apart. It is no control: no Tab stop, no press feedback, read by screen readers as text; a press on it moves the focus
into the field (as a `Label`), except while the field is disabled.

### `InputGroupButton` props

| TSX | Rust | Type | Default | Description |
|---|---|---|---|---|
| `variant` | `variant` | `ButtonVariant` | `"ghost"` | As `Button`'s; `ghost` takes the addon's muted color at rest |
| `iconOnly` | `icon_only` | `boolean` | `false` | A square showing `icon` alone; give it an `accessibilityLabel` |
| `icon` | `icon` | `IconName` | — | An icon before the content |
| `disabled` | `disabled` | `boolean` | `false` | No presses; also while the field is disabled |
| `accessibilityLabel` | `accessibility_label` | `string` | — | What screen readers say (an icon-only button's name) |
| `accessibilityHint` | `accessibility_hint` | `string` | — | What pressing it does |
| `labelClass` / `labelStyle` | `label_class` / `label_style` | | | The text part around each child |
| `iconClass` / `iconStyle` | `icon_class` / `icon_style` | | | The icon |
| `class` / `style` | `class` / `style` | | | The button, merged last |
| `children` | `.child(..)` | elements | | Its label ("Copy") |

Event: `onPress` (Rust `InputGroupButtonEvent::Press`), never while disabled.

### `InputGroupPasswordToggle` props

| TSX | Rust | Type | Default | Description |
|---|---|---|---|---|
| `shown` | `shown` | `boolean` (Rust `Option<bool>`) | — | Controlled: whether the password shows. Unset, the toggle keeps its own state, hidden at first |
| `disabled` | `disabled` | `boolean` | `false` | No presses; also while the field is disabled |
| `accessibilityLabel` | `accessibility_label` | `string` | "Show password" (`show-password`, translatable) | What screen readers say |
| `iconClass` / `iconStyle` | `icon_class` / `icon_style` | | | The eye |
| `class` / `style` | `class` / `style` | | | The button, merged last |

Event: `onChange({ shown })` (Rust `InputGroupPasswordToggleEvent::Change(bool)`): the state a press asks for, before
anything changes; an uncontrolled toggle then shows it, a controlled one waits for the app's `shown`.

While shown, the group's `Input` with `masked` shows its text: the toggle overrides `masked` through the group, so the
app keeps `masked` as it is. When the toggle goes away, the password is hidden again.

## Variants and sizes

No variants of the group. The group's `size` sets everything inside:

| `size` | Height (pointer / touch) | Text | Icons | Edge padding | Button side (pointer / touch) |
|---|---|---|---|---|---|
| `xs` | 24 / 32 (`input-height-xs`) | `input-font-size-xs` (12 / 14) | 12 | 8 | 16 / 24 |
| `sm` | 28 / 36 | 13 / 14 | 14 | 10 | 20 / 28 |
| `md` | 32 / 40 | 14 / 16 | 16 | 12 | 24 / 32 |
| `lg` | 36 / 48 | 14 / 16 | 16 | 12 | 28 / 40 |

- The height, text and padding are Input's tokens, so a group lines up with an `Input` and a `Button` of its size.
- An inline addon has the field's padding (`input-padding-x-*`) on its outer side and `input-group-gap` between its
  children; the field leaves `input-group-gap` toward an addon and the field's padding toward the edge.
- A button is as tall as the field less 4 above and below (`input-height-*` − 8), its corners `input-radius` − 4 (the
  frame's, inset), so it sits concentric in the frame. At the frame's edge it moves out to 4 from the frame's outer
  edge, the same inset as above and below it (a negative margin on the addon's first or last child), so its corners
  share the frame's corner centers.
- The block addons (`blockStart` / `blockEnd`) span the frame, a row as tall as the field (a button's side and the
  button inset above and below it), with the field's padding at the sides for words; their first and last buttons move
  out to the button inset, so a button in a corner of the frame is concentric with it, as in an inline addon. The
  column grows with a `Textarea`.
- Width: `w-full min-w-0`, as `Input`; in a horizontal `Field` the group takes the row's equal share.

## States

The field's states drawn on the frame: the same table as [Input's](input.md#states), on the group instead of the field.

| State | Frame | Inside |
|---|---|---|
| Rest | `input-border`, `input-background`, `input-shadow` | Addons muted; ghost buttons muted |
| Field focused | `input-border-focus` and the `input-ring` ring outside the frame (touch: the edge alone) | — |
| A button focused (Tab) | As at rest | The button's own ring (Button's `focus-visible`) |
| Button hover / press | — | Ghost: `button-ghost-hover` fill and its foreground; press sinks (0.97, none with reduced motion) |
| Invalid (the field's or its `Field`'s) | `input-border-invalid`; the ring `input-ring-invalid` while focused | — |
| Read-only | As at rest | Buttons still work (Copy a read-only key) |
| Disabled (the field's or its `Field`'s) | `opacity-disabled` over the whole group | Buttons and the toggle disabled (said to screen readers), addons do not focus the field; nothing dims twice |
| Password shown | — | The toggle's eye is crossed out (`eye-off`); screen readers say pressed |

No hover on the frame (fields have none). Transitions take `duration`, none with reduced motion.

## Pointer and touch

| | Pointer | Touch |
|---|---|---|
| Sizes | `input-height-*` | `input-height-*` touch values; text 16 at `md` / `lg` |
| Field hit area | The field's box | Field: vertical hit slop to `touch-target` (44); buttons: the same, above and below only (the field is beside them) |
| A press on an addon | Focuses the field | Focuses the field (as a label's tap) |
| A press on a button | Leaves the focus in the field (NSH: a press on a view that is not `focusable` keeps an editing field's focus), so the caret stays | iOS and Android keep the field's focus; on the mobile web a tap blurs it as the browser does |
| Cursor | The I-beam over addons (a press there edits), the arrow over buttons | — |

## Accessibility

- The field is the element screen readers name: give the `Input` an `accessibilityLabel` that says what the addons
  show ("Amount in US dollars", "Website address"), since NSH has no labelledby (Input's G1).
- The group itself has no role: it adds no stop and no extra "group" for a screen reader to step through.
- Addons are text (`role` text, no Tab stop); their icons are hidden; a `Spinner` says "Loading".
- `InputGroupButton` is a button with its `accessibilityLabel`; Tab reaches it after the field (written order).
- `InputGroupPasswordToggle` is a toggle button (the `button` role with a pressed state) named "Show password": the name
  says what it does when pressed, the state says whether it is on, and the name does not change with the state, as
  WAI-ARIA's toggle buttons ask. Tab reaches it; Space and Return press it.
- Disabled: the field and every button in the group are disabled for screen readers.

## Tokens

The frame reads the `input-*` tokens (foundation's field look, [Input's table](input.md#tokens)); buttons read
Button's (`button-ghost-hover`, `button-ring`, ...). The group adds:

Generated from the code's declarations (`NSH_LONGBRIDGE_WRITE=1 cargo test --test tokens`); a test keeps it current.
Values are those of `nsh/base`'s default theme.

<!-- tokens:input-group -->
| Token | Type | Derived default | Value |
|---|---|---|---|
| `input-group-addon-foreground` | color | `muted-foreground` |  |
| `input-group-gap` | length | `spacing × 2` | 8 |
<!-- /tokens -->

Other tokens it reads: `input-height-*`, `input-padding-x-*`, `input-font-size-*`, `input-radius`, `icon-size-*`,
`touch-target`, `opacity-disabled`.

## Examples

TSX:

```tsx
import { keymap, signal } from "nsh";
import {
  Icon, Input, InputGroup, InputGroupAddon, InputGroupButton, InputGroupPasswordToggle, Kbd, Spinner, Textarea,
} from "nsh-longbridge";

// A password with its eye
<InputGroup>
  <Input masked autoComplete="password" accessibilityLabel="Password" />
  <InputGroupAddon align="inlineEnd">
    <InputGroupPasswordToggle />
  </InputGroupAddon>
</InputGroup>

// A search: icon, clear button while it has text, key hint
const query = signal("");
<InputGroup>
  <InputGroupAddon>
    <Icon name="search" />
  </InputGroupAddon>
  <Input value={query()} onChange={({ value }) => query.set(value)} placeholder="Search" accessibilityLabel="Search" />
  <InputGroupAddon align="inlineEnd">
    {query() ? <InputGroupButton iconOnly icon="x" accessibilityLabel="Clear" onPress={() => query.set("")} /> : null}
    <Kbd>{keymap.label("mod+k")}</Kbd>
  </InputGroupAddon>
</InputGroup>

// Words before and after
<InputGroup>
  <InputGroupAddon>https://</InputGroupAddon>
  <Input placeholder="example.com" keyboardType="url" accessibilityLabel="Website address" />
</InputGroup>
<InputGroup>
  <InputGroupAddon>$</InputGroupAddon>
  <Input placeholder="0.00" keyboardType="decimal-pad" accessibilityLabel="Amount in US dollars" />
  <InputGroupAddon align="inlineEnd">USD</InputGroupAddon>
</InputGroup>

// An action, a check in progress
<InputGroup>
  <Input readonly value={apiKey} accessibilityLabel="API key" />
  <InputGroupAddon align="inlineEnd">
    <InputGroupButton icon="copy" onPress={copy}>Copy</InputGroupButton>
  </InputGroupAddon>
</InputGroup>
<InputGroup>
  <Input value={name()} onChange={check} accessibilityLabel="User name" />
  <InputGroupAddon align="inlineEnd">{checking() ? <Spinner /> : <Icon name="check" />}</InputGroupAddon>
</InputGroup>

// A message box with a toolbar
<InputGroup>
  <Textarea placeholder="Write a message" accessibilityLabel="Message" />
  <InputGroupAddon align="blockEnd">
    <Text class="flex-1">{count()} / 280</Text>
    <InputGroupButton variant="default" iconOnly icon="arrow-right" accessibilityLabel="Send" />
  </InputGroupAddon>
</InputGroup>
```

With NSH 0.7.0, a string child of a Rust component is written inside `<Text>` for now ([Button's G0](button.md#nsh-gaps)):
`<InputGroupAddon><Text>https://</Text></InputGroupAddon>`.

Rust:

```rust
use nsh_core::prelude::*;
use nsh_longbridge::{
    InputGroupAlign, IconName, input, input_group, input_group_addon, input_group_password_toggle,
};

input_group()
    .child(input().masked(true).accessibility_label("Password"))
    .child(
        input_group_addon()
            .align(InputGroupAlign::InlineEnd)
            .child(input_group_password_toggle()),
    )
```

## NSH gaps

- **G1 No `:has()` or ancestor selectors.** The frame cannot style itself from the field inside it (`has-[input:focus]`),
  nor the field from the group around it. Workaround: the group's context, which the field, the addons and the
  buttons read and write (conventions, "Context instead of ancestor selectors").
- **G2 No `order`.** NSH's layout has no `order`, so an addon cannot move itself before the field. Workaround: the parts
  show in the order they are written; `align` says which edge an addon belongs to.
- **G3 Changing `masked` rebuilds the macOS field.** AppKit has separate classes for secure and plain fields, so NSH
  replaces the field and puts the caret at the end of its text. Workaround: an `Input` made with `masked` keeps the last
  selection it saw and sets it again on the next frame after `masked` changes while it has the focus (a platform that
  keeps it gets the same selection again). Checked headless (the command is sent); not yet checked by hand in each
  platform's field.
- **G4 Context reaches overlays.** An overlay opened from an addon (a currency menu) is made inside the group, so a
  field in its content would read the group, as it would a `Field`'s. Until overlays end both contexts for their
  content, put no `Input` in an overlay opened from inside a group.

## Decisions

- **Parts are `InputGroup`, `InputGroupAddon`, `InputGroupButton`, `InputGroupPasswordToggle`.** No `InputGroupText`:
  an addon already gives its children the muted text style, so a text part would only rename it. No
  `InputGroupInput` / `InputGroupTextarea`: the library's `Input` and `Textarea` read the group.
- **State comes from the field.** The group has no `invalid` / `disabled` props: the `Input` (or its `Field`) says it
  once and the frame, buttons and addons follow, so the two can never disagree.
- **The ring is the field's focus, not `focus-within`**: a button focused by Tab shows its own ring, and the frame does
  not ring at the same time.
- **A ready password toggle, not only a recipe.** Nearly every sign-up and reset page has the eye; the part brings the
  icons, the translatable name, the toggle role and pressed state, and the reveal through the group without app state.
  A recipe stays possible (`InputGroupButton` and the app's `masked`). No `PasswordInput` component: it would repeat
  all of `Input`'s props and close the door on other addons (a lock icon, a strength meter).
- **The toggle's name is fixed ("Show password") and its pressed state changes**, rather than a name that switches to
  "Hide password": WAI-ARIA's toggle buttons keep their name, and a changing name with a pressed state would read
  "Hide password, pressed". There is no `hide-password` word.
- **Buttons are the field's height less 8**, not a `Button` size: a `Button` `xs` fits `md` but no size fits `sm` or
  `xs` fields, and the corners must be the frame's inset.
- **A press on a button does not move the focus** (NSH's rule for views that are not `focusable`): the caret stays in
  the field when the eye or Clear is clicked, while Tab still reaches every button.
