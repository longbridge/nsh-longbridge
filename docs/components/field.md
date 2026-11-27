# Field

Status: implemented (P3).

The layout of a form: a control with its label, its description and its error, alone (`Field`), in a column of
fields (`FieldGroup`) or in a titled set (`FieldSet` with a `FieldLegend`), vertical, horizontal or switching between
the two with the room it has. A `Field` also hands its `invalid` and
`disabled` state, and its description or error for screen readers, to the control inside it, so the app writes them
once.

## Usage

- Every labelled control of a form or a settings page: an `Input` with a label and a help line, a `Select`, a
  `Textarea` with a count, a `Switch` with a title and a description beside it, a set of `Checkbox`es under a legend,
  a `Slider`, a `Combobox`, a `RadioGroup`.
- Choice cards: a whole bordered card that is one option of a `RadioGroup` or a `Checkbox` (a plan, a delivery
  speed).
- Search terms: form field, form item, form row, label and help text, field description, error message, validation
  message, fieldset, legend, form section, settings row, choice card.

## When not to use

| Need | Use instead |
|---|---|
| A label alone beside a control, no description or error | [Label](label.md) is enough (or the control's children: `Checkbox`, `Switch`, `RadioGroupItem`) |
| The form's state, validation rules, submission | The app's own code (or a form library): `Field` draws, it does not validate. No `Form` part binds a form library; see Decisions 1 |
| A message about the whole form or page ("Could not save") | A `Toast`, or an Alert (later) above the form |
| A titled box around settings | [Card](card.md), with `Field`s in its content |
| A table of values to edit | Table (P3), with controls in its cells |

## Built on

No `nsh/base` component: `nsh/base` has no form layout. Every part is a view (or a text) of this library's. What it
uses from NSH and from the library, rather than rewriting:

| Used as is | |
|---|---|
| The controls | `Input`, `Textarea`, `Select`, `Checkbox`, `Switch`, `RadioGroup`, `Combobox`, `Slider`, `ToggleGroup`: each keeps its own behavior, look and props |
| `Label` | `FieldLabel` is a [Label](label.md) (its look, `select-none`, a press focusing its field) with the field's additions |
| `Separator` | `FieldSeparator`'s line is a [Separator](separator.md) |
| NSH's `accessibilityDescribedBy` | The element whose text describes a control, read after its name (nsh-core's `accessibility_described_by`; ARIA's `aria-describedby`). [Dialog](dialog.md) uses it for its description |
| `foundation::field_ref` | The registry by which a label finds its field (`FieldRef`, Label's `htmlFor`), extended here (see Implementation) |

### Structure

The parts nest as `FieldSet > FieldLegend + FieldDescription + FieldGroup > Field > FieldLabel + control +
FieldDescription + FieldError`, with `FieldContent` and `FieldTitle` in horizontal fields and choice cards and
`FieldSeparator` between groups. How they work together, and why:

- **The state goes on `Field` once.** On the web an app marks the field invalid for the label's color and the control
  invalid for its edge, pairs an `id` on the control with `htmlFor` on the label, and disables the control besides
  dimming the label. Here `Field`'s `invalid` and `disabled` reach the label and the control, and `FieldLabel` finds
  the field's control without a ref: one place to write each, so the label and the control cannot disagree.
- **No `id` / `htmlFor` wiring needed.** `FieldLabel`'s `htmlFor` defaults to the `Field`'s control (the first control
  of the library made inside it). It still takes a ref, as `Label`'s, for a label that names a control elsewhere.
- **The description and the error describe the control.** Rather than leaving the described-by relation to the app,
  the `Field`'s control is described by its `FieldError` while that shows, else by its `FieldDescription`, so screen
  readers read them by default.
- **Responsive without container queries.** `orientation="responsive"` switches at the width of the nearest
  `FieldGroup`; NSH has no container queries, so `FieldGroup` measures its width and tells its fields (NSH gaps 3).
- **What the children are is told through context.** A `Field` aligns its row to the top when it holds a
  `FieldContent`, and a `FieldLabel` becomes a card when it holds a `Field`; NSH has no `has-` or `peer-` variants to
  select on children, so the inner part registers itself with the outer one.

### Implementation

The field integration (`foundation::field`, the widened `foundation::field_ref`, `SelectRef` and each existing
control's `field::join`) lands first, in one self-contained commit (as `popup::surface` did in P2), so other
components (DatePicker) take it before the Field parts exist.

- **Context.** `Field` starts a field with `field::provide(invalid, disabled)`, which gives what is made inside it a
  `FieldState`: its `invalid` and `disabled` (its own, or an enclosing `FieldSet`'s `disabled`), and three registers
  filled while their parts are mounted: its control (the host node, as `foundation::field_ref` keys it), its
  description's element and its error's element (`register_description`, `register_error`). Parts read it with
  `field::current()` (`None` outside a `Field`). `FieldSet` provides its `disabled` (`field::provide_set`); an overlay's
  content placed in a field ends it (`field::clear`). The component layer adds the `orientation` (responsive resolved
  to vertical or horizontal), `FieldGroup`'s measured width and the choice card, which reads its field's control's
  checked state through `FieldRef`.
- **Layout the control reads.** `Field` tells its `FieldState` two facts of its layout, which a control reads from its
  `Joined`: `in_row` (the field is horizontal) and `beside_content` (horizontal with a `FieldContent`). NSH's layout,
  as React Native's, neither shrinks a view nor keeps it at its content's width unless told to, so in a row a
  full-width control (Input, Textarea, Combobox, Slider) and the label side (`FieldContent` or a horizontal
  `FieldLabel`) each take `flex: 1 1 0`, an equal share of the row, so the rows of a group line up in columns
  whatever their descriptions say; the label side keeps CSS's automatic minimum (`min-width: auto`, its longest word);
  a horizontal `FieldLabel` stacks its children (a column), since a text in a row does not shrink and would run under
  the control instead of wrapping;
  beside a `FieldContent`, a choice control (Checkbox, Switch, a radio) centers itself on the text's first line as on
  its own label's (Decisions 5).
- **Joining a field.** Every control of the library calls `field::join(host, kind, disabled)` in its body, once (with
  its own `disabled`): it registers what a label's press does to it, and inside a `Field` whose control slot is empty,
  it takes it, and it gets back the field's `invalid`, `disabled` and described-by target (a `Joined`; inside a
  `FieldSet` alone, the set's `disabled`), which it combines with its own props:
  - `invalid` = the control's `invalid` or the field's; `disabled` = the control's `disabled` or the field's (a field
    cannot re-enable a control the app disabled).
  - The described-by target goes on the element screen readers reach where the library can set it: Input's and
    Textarea's host field, Select's trigger (the view `nsh/base` makes the combobox), Checkbox's root through
    `nsh/base`'s `accessibilityDescribedBy`, DatePicker's trigger; a control's own `accessibilityDescribedBy`, where it
    has one, wins. Switch, RadioGroup, ToggleGroup, Combobox and Slider cannot carry it (NSH gaps 5).
  - `kind` says what a label's press does: `Focus` (Input, Textarea, Select's trigger, DatePicker's trigger),
    `Activate` (Checkbox, Switch, a RadioGroupItem: the control's own press, reporting its `onChange`) or `Inert` (a
    RadioGroup, ToggleGroup, Slider, and Combobox, whose input `nsh/base` gives no handle on: NSH gaps 7; a label's
    press does not reach them; they join for `invalid` where they have it and `disabled`).
  The controls that join: Input, Textarea, Select, Checkbox, Switch, RadioGroup (and its items in choice cards),
  Combobox, Slider, ToggleGroup, DatePicker. Joining changes nothing outside a `Field`: the P1 and P2 controls behave
  as today.
- **`foundation::field_ref`** grows from "a field to focus" to "a field's control": each entry keeps the element to
  focus and, for a choice control, its activation and its checked state. `FieldRef`'s TSX type widens to the refs of
  every control a label can focus or press (`InputRef | TextareaRef | SelectRef | CheckboxRef | SwitchRef |
  DatePickerRef`; `SelectRef` and `DatePickerRef` are new, each with `focus()`; no `ComboboxRef`: NSH gaps 7). Label's
  `htmlFor` keeps working and takes the new refs (Decisions 3).
- **Parts are views of this library's**, styled with bindings on the tokens, the app's `class` / `style` merged last.
  String children are wrapped in a text that takes the part's text style (Button's G1).

## API

### Components

| Component | TSX | Rust | What it is |
|---|---|---|---|
| `Field` | `<Field>` | `field()` | One control with its label, description and error; holds the state |
| `FieldLabel` | `<FieldLabel>` | `field_label()` | The control's label: a `Label` that knows its field; around a `Field`, a choice card |
| `FieldDescription` | `<FieldDescription>` | `field_description()` | Muted help text under a label, a control or a legend |
| `FieldError` | `<FieldError>` | `field_error()` | The validation message(s), in `destructive`; shown only with content |
| `FieldContent` | `<FieldContent>` | `field_content()` | A column for a label (or title) and a description beside a control, in a horizontal field |
| `FieldTitle` | `<FieldTitle>` | `field_title()` | A label-looking title that is no label (inside a choice card, where the card is the label) |
| `FieldGroup` | `<FieldGroup>` | `field_group()` | A column of fields; the width responsive fields switch on |
| `FieldSet` | `<FieldSet>` | `field_set()` | A set of related fields under a legend (a browser's `<fieldset>`) |
| `FieldLegend` | `<FieldLegend>` | `field_legend()` | The set's title |
| `FieldSeparator` | `<FieldSeparator>` | `field_separator()` | A line between groups, optionally with a word on it ("Or continue with") |

### `Field`

| TSX | Rust | Type | Default | Description |
|---|---|---|---|---|
| `orientation` | `orientation` | `"vertical"` \| `"horizontal"` \| `"responsive"` (`FieldOrientation`) | `"vertical"` | Label above the control, beside it, or above it until the enclosing `FieldGroup` is at least `field-responsive-width` (448) wide |
| `invalid` | `invalid` | `boolean` / `Prop<bool>` | `false` | The value is rejected: the label turns `destructive`, the control takes its invalid look and state |
| `disabled` | `disabled` | `boolean` / `Prop<bool>` | `false` | The label and title dim; the control is disabled (or'ed with its own `disabled`) |
| `children` | `.child(..)` | elements | — | The parts and the control, in the order shown |
| `class` / `style` | `class` / `style` | | | The root, merged last |

No events: the control reports its own.

### `FieldLabel`

`Label`'s props ([label.md](label.md)): `children`, `htmlFor`, `disabled`, `class` / `style`. Inside a `Field`:

| TSX | Rust | Default inside a `Field` | Description |
|---|---|---|---|
| `htmlFor` | `html_for` | The field's control | What a press focuses (`focus` controls) or presses (`activate` controls: a checkbox toggles, as a browser's `<label>`). A ref given wins |
| `disabled` | `disabled` | The field's `disabled` | Or'ed with the field's |

A `FieldLabel` around a `Field` is a **choice card** (see Variants): a press anywhere on it presses that field's control.

### `FieldDescription`, `FieldError`, `FieldContent`, `FieldTitle`

| Component | Props | Notes |
|---|---|---|
| `FieldDescription` | `children`, `class` / `style` | Registers its element as the field's description (the first one, if several) |
| `FieldError` | `children`, `errors`, `class` / `style` | Shows `children` if any, else `errors` (below); with neither it is hidden, taking no room and no gap. Registers its element as the field's error while it shows |
| `FieldContent` | `children`, `class` / `style` | Tells its `Field` it is there (the row aligns to the top) |
| `FieldTitle` | `children`, `class` / `style` | Follows the field's `invalid` and `disabled` as `FieldLabel` |

`errors` (for validation libraries): TSX `({ message?: string | null } | null | undefined)[]`, Rust `FieldErrors`
(`Vec<SharedString>`, from an iterator of messages). Messages are deduplicated in order and empty ones skipped; one
message shows as text, several as a bulleted list. A Zod issue list or a react-hook-form error passes as is.

### `FieldGroup`, `FieldSet`, `FieldLegend`, `FieldSeparator`

| Component | Props | Notes |
|---|---|---|
| `FieldGroup` | `children`, `class` / `style` | Measures its width for responsive fields inside it (not through a nested `FieldGroup`, which measures its own) |
| `FieldSet` | `children`, `disabled`, `accessibilityLabel`, `class` / `style` | `disabled` disables every field inside it (a browser's `<fieldset disabled>`); `accessibilityLabel` names the set for screen readers (set it to the legend's text: NSH gaps 1) |
| `FieldLegend` | `children`, `variant`, `class` / `style` | `variant`: `"legend"` (16, the default) or `"label"` (14, for a set that reads as one field: a group of checkboxes) (`FieldLegendVariant`) |
| `FieldSeparator` | `children`, `class` / `style`, `contentClass` / `contentStyle` | `children`: the word on the line, optional |

### Parts

| Part | What | Customize with |
|---|---|---|
| Each component's root | A view (a text for `FieldDescription`, `FieldTitle`, `FieldLegend`) | Its `class` / `style` |
| `FieldError`'s items | The rows of a list of several errors | `FieldError`'s `class` / `style` (their text inherits it) |
| `FieldSeparator`'s content | The word on the line, on the page's fill | `contentClass` / `contentStyle` |

## Variants and sizes

No `size`: the field parts have one size; the control inside keeps its own `size`.

### Layout

The reference look as Tailwind classes, and this library's values (all lengths are tokens, multiples of `spacing`, so a compact theme shrinks
them):

| Part | Classes | This library |
|---|---|---|
| `Field` vertical | `group/field flex w-full flex-col gap-3 [&>*]:w-full` | A column, children stretched to its width, `field-gap` (8: a label and its control are closely related controls) apart; a description or an error `field-content-gap` (4) under what is above it |
| `Field` horizontal | `flex-row items-center [&>[data-slot=field-label]]:flex-auto`, `has-[>[data-slot=field-content]]:items-start` | A row, centered, `field-gap` apart; a `FieldLabel` or `FieldContent` in it and a full-width control share the row equally (`flex: 1 1 0`, so a group's rows line up in columns; the label side never narrower than its longest word, and a `FieldLabel` there stacks its children, so a long label wraps); beside a switch or a box the label side takes the room left (Implementation); with a `FieldContent`, the row aligns to the top and the content's first line is level with the control's: a choice control (Checkbox, Switch, a radio) or a slider's track, shorter than the line, is centered on it; beside a taller text control (Input, Textarea, DatePicker's field) the content moves down until its first line is level with the control's text (Decisions 5) |
| `Field` responsive | vertical; `@md/field-group:` horizontal | Vertical while the enclosing `FieldGroup` is narrower than `field-responsive-width` (448, Tailwind's `@md` container, 28rem), horizontal from it; vertical outside a `FieldGroup` (no container) |
| `FieldContent` | `flex flex-1 flex-col gap-1.5 leading-snug` | A column, `flex: 1 1 0` (as a horizontal `FieldLabel`: half the row beside a full-width control, the room left beside a switch or a box), `field-content-gap` (4: a label and its description are parts of one text) apart |
| `FieldGroup` | `flex w-full flex-col gap-7`, a nested group `gap-4` | A column, full width, `field-group-gap` (16: separate groups of a form) apart; a `FieldGroup` directly in another (not through a `Field`) `field-group-nested-gap` (12) |
| `FieldSet` | `flex flex-col gap-6` | A column, `field-set-gap` (12) apart: the legend is closer to its fields than the fields are to each other. A tighter `gap-3` around a checkbox or radio group is the app's `class="gap-3"` (NSH gaps 4) |
| `FieldLegend` | `mb-3 font-medium`, `text-base` / `text-sm` | Margin below `field-legend-margin-bottom` (0: the set's gap is the legend's space), `font-weight-medium`, `field-legend-font-size` (16) or `label-font-size` (14) with `variant="label"`, color `field-legend-foreground`, line height from foundation (24 / 20) |
| `FieldLabel` | Label + `flex w-fit gap-2 leading-snug` | Label's look; as wide as its content (`w-fit`) in a vertical field, line height from foundation (20, rather than `leading-snug`'s 19.25, which is off the 4-point grid; Label's own `leading-none` is dropped here so wrapped labels do not collide) |
| `FieldTitle` | `flex w-fit items-center gap-2 text-sm leading-snug font-medium` | `label-font-size`, `label-font-weight`, `label-foreground`, line height 20, `w-fit` |
| `FieldDescription` | `text-sm leading-normal font-normal text-muted-foreground`, `last:mt-0 nth-last-2:-mt-1`, after a legend `-mt-1.5` | `field-description-font-size` (14), line height 20, regular, `field-description-foreground`; `field-content-gap` (4) under what is above it, a label, a control or a legend (a negative top margin of the parent's gap less `field-content-gap`), wherever it stands, as the first child none |
| `FieldError` | `text-sm font-normal text-destructive`; several: `ml-4 flex list-disc flex-col gap-1` | `field-error-font-size` (14), line height 20, regular, `field-error-foreground`; `field-content-gap` (4) under what is above it, as a description; several: a column 4 apart, each row a bullet (`•`) in a 16-wide column, then the message |
| `FieldSeparator` | `relative -my-2 h-5 text-sm`; the line `absolute inset-0 top-1/2`; the word `mx-auto w-fit bg-background px-2 text-muted-foreground` | 20 tall (`spacing × 5`), `-my-2`; a `Separator` across its middle; the word centered on it, `w-fit`, padding 8 at the sides, fill `field-separator-content-background` (`background`), text 14 `field-separator-content-foreground` |

Text in `FieldDescription` is not balanced (`text-balance` has no NSH counterpart); links in it are
the app's own elements.

### Choice card

A `FieldLabel` whose child is a `Field`: a bordered card the whole of which is the option.

| | Classes | This library |
|---|---|---|
| Card | `w-full flex-col rounded-md border`, the inner field `p-4` | Full width, a column, `field-choice-card-radius` (8), `border-width` × `field-choice-card-border` (`border`); the inner `Field` padded `field-choice-card-padding` (16) |
| Checked (the control inside is on) | `border-primary bg-primary/5 dark:bg-primary/10` | Edge `field-choice-card-checked-border` (`primary`), fill `field-choice-card-checked` (`primary` at 5%, 10% in dark) |

The inner `Field` is usually horizontal, with a `FieldContent` (a `FieldTitle` and a `FieldDescription`) and the
control. Its `FieldTitle` is used rather than a `FieldLabel`, the card being the label.

## States

| State | Look | Classes |
|---|---|---|
| Rest | As Layout | — |
| invalid (`Field`'s) | `FieldLabel`, `FieldTitle` text `field-label-foreground-invalid` (`destructive`); the control's invalid look (its own spec); `FieldDescription` unchanged | `data-[invalid=true]:text-destructive` on the field |
| disabled (`Field`'s or `FieldSet`'s) | `FieldLabel`, `FieldTitle` at `opacity-disabled`; the control disabled (its own look); a label's press does nothing | `group-data-[disabled=true]/field:opacity-50` |
| Choice card checked | As Choice card | `has-data-[state=checked]:…` |
| Choice card pressed, hovered, focused | No look of its own: the control inside shows its press, and its focus ring when the keyboard is on it | none |

No hover, focus or press look on any part: they are text and layout. The choice card's edge and fill change at once
with the control's value (no transition), as the checkbox's own fill does.

## Pointer and touch

| | Pointer (desktops, desktop web) | Touch |
|---|---|---|
| Spacing | As Layout | The same (the controls are taller on touch; their gaps are not) |
| `FieldLabel` press | Focuses or presses the field's control (on release), as a browser's `<label>` | The same; a scrolling drag does not trigger it |
| Label hit area | The label's box, not padded (Label's rule: padding would take presses from the rows around) | The same |
| Choice card | The whole card is the control's press target; the arrow cursor | The whole card (at least 44 tall with its padding); no hit slop added |
| Horizontal fields | As asked | As asked: on a narrow phone screen use `responsive`, which falls back to vertical under 448 |

## Accessibility

- **The control's name**: NSH has no labelledby (Input's G1), so the control's `accessibilityLabel` is still the app's
  to set (to the `FieldLabel`'s text). The label is read as text, as Label's.
- **Description and error**: the control is described by its field's `FieldError` while that shows, else by its
  `FieldDescription` (NSH's `accessibilityDescribedBy` names one element: NSH gaps 2). Screen readers read it after the
  name and the value.
- **Invalid and disabled** reach the control's accessibility state through its own props (`invalid` is announced,
  `disabled` reads as dimmed).
- **`FieldError`** has the `alert` role and an `assertive` live region: an error that
  appears is announced at once.
- **`Field`** has the `group` role, unnamed. **`FieldSet`** has the `group` role, named by
  its `accessibilityLabel` (NSH has no legend relation: NSH gaps 1).
- **`FieldLegend`**, `FieldTitle`, `FieldDescription`: plain text. `FieldSeparator`'s line is hidden from screen
  readers (Separator's rule); its word is read.
- **Choice card**: the control inside keeps its role and state (radio, checkbox, switch); the card adds none. A press
  on the card presses the control, so VoiceOver's and TalkBack's activation of the control and a press anywhere on the
  card do the same.
- Keyboard: nothing of its own; Tab goes through the controls in order.

## Tokens

Generated from the code's declarations (`NSH_LONGBRIDGE_WRITE=1 cargo test --test tokens`); a test keeps it current.
Values are those of `nsh/base`'s default theme.

<!-- tokens:field -->
| Token | Type | Derived default | Value |
|---|---|---|---|
| `field-gap` | length | `spacing × 2` | 8 |
| `field-content-gap` | length | `spacing` | 4 |
| `field-group-gap` | length | `spacing × 4` | 16 |
| `field-group-nested-gap` | length | `spacing × 3` | 12 |
| `field-set-gap` | length | `spacing × 3` | 12 |
| `field-responsive-width` | length | `448` | 448 |
| `field-legend-margin-bottom` | length | `0` | 0 |
| `field-legend-font-size` | length | `text-base` | 16 |
| `field-legend-foreground` | color | `foreground` |  |
| `field-label-foreground-invalid` | color | `destructive` |  |
| `field-description-font-size` | length | `text-sm` | 14 |
| `field-description-foreground` | color | `muted-foreground` |  |
| `field-error-font-size` | length | `text-sm` | 14 |
| `field-error-foreground` | color | `destructive` |  |
| `field-separator-content-foreground` | color | `muted-foreground` |  |
| `field-separator-content-background` | color | `background` |  |
| `field-choice-card-radius` | length | `radius-md` | 8 |
| `field-choice-card-padding` | length | `spacing × 4` | 16 |
| `field-choice-card-border` | color | `border` |  |
| `field-choice-card-checked` | color | `light_dark(primary × 5%, primary × 10%)` |  |
| `field-choice-card-checked-border` | color | `primary` |  |
<!-- /tokens -->

- The label and the title read Label's tokens (`label-font-size`, `label-font-weight`, `label-foreground`); only the
  invalid color is the field's. The separator's line reads Separator's (`separator`, `separator-thickness`).
- `field-responsive-width` is Tailwind's fixed `@md` container width (28rem), not a multiple of `spacing`: it is a
  breakpoint, not a gap.
- `field-separator-content-background` must match what the separator is drawn on; inside a `Card` set it to `card` in
  a scope, or `contentClass="bg-card"`.
- Other tokens it reads: `border-width`, `opacity-disabled`, `font-weight-medium`.

## Examples

TSX:

```tsx
import { signal } from "nsh";
import {
  Button, Checkbox, Field, FieldContent, FieldDescription, FieldError, FieldGroup, FieldLabel, FieldLegend,
  FieldSeparator, FieldSet, FieldTitle, Input, RadioGroup, RadioGroupItem, Switch,
} from "nsh-longbridge";

const name = signal("");
const nameError = () => (name().trim() === "" ? "Enter your name." : null);

<FieldSet accessibilityLabel="Payment method">
  <FieldLegend>Payment method</FieldLegend>
  <FieldDescription>All transactions are secure and encrypted.</FieldDescription>
  <FieldGroup>
    <Field invalid={nameError() !== null}>
      <FieldLabel>Name on card</FieldLabel>
      <Input value={name()} onChange={({ value }) => name.set(value)} placeholder="Evil Rabbit" accessibilityLabel="Name on card" />
      <FieldError>{nameError()}</FieldError>
    </Field>
    <Field>
      <FieldLabel>Card number</FieldLabel>
      <Input placeholder="1234 5678 9012 3456" accessibilityLabel="Card number" />
      <FieldDescription>Enter your 16-digit card number.</FieldDescription>
    </Field>
  </FieldGroup>
</FieldSet>

<FieldSeparator>Or</FieldSeparator>

// A setting beside its switch; the label's press toggles the switch.
<Field orientation="horizontal">
  <FieldContent>
    <FieldLabel>Share across devices</FieldLabel>
    <FieldDescription>Focus is shared across devices, and turns off when you leave the app.</FieldDescription>
  </FieldContent>
  <Switch checked={shared()} onChange={({ checked }) => shared.set(checked)} accessibilityLabel="Share across devices" />
</Field>

// A checkbox with its label after it.
<Field orientation="horizontal">
  <Checkbox checked={same()} onChange={({ checked }) => same.set(checked)} accessibilityLabel="Same as shipping address" />
  <FieldLabel>Same as shipping address</FieldLabel>
</Field>

// Choice cards: each card is the whole option.
<RadioGroup value={plan()} onChange={({ value }) => plan.set(value)} accessibilityLabel="Compute environment">
  <FieldLabel>
    <Field orientation="horizontal">
      <FieldContent>
        <FieldTitle>Kubernetes</FieldTitle>
        <FieldDescription>Run GPU workloads on a K8s cluster.</FieldDescription>
      </FieldContent>
      <RadioGroupItem value="kubernetes" accessibilityLabel="Kubernetes" />
    </Field>
  </FieldLabel>
</RadioGroup>

// Errors from a validation library.
<FieldError errors={issues()} />

// Responsive: label beside the control once the group is 448 wide.
<FieldGroup>
  <Field orientation="responsive">
    <FieldContent>
      <FieldLabel>Name</FieldLabel>
      <FieldDescription>Provide your full name for identification.</FieldDescription>
    </FieldContent>
    <Input placeholder="Evil Rabbit" accessibilityLabel="Name" />
  </Field>
  <Field orientation="horizontal">
    <Button>Submit</Button>
    <Button variant="outline">Cancel</Button>
  </Field>
</FieldGroup>
```

With NSH 0.7.0, a string child is written inside `<Text>` for now ([Button's G0](button.md#nsh-gaps)). A
`FieldError` whose child is `null` (no error) makes nothing, and shows once the child is not: verified headless
(`tests/field.rs`, a TSX app whose `FieldError` child is `null` until the value is cleared).

Rust:

```rust
use nsh_core::prelude::*;
use nsh_longbridge::{
    FieldErrors, FieldOrientation, field, field_description, field_error, field_group, field_label, input,
};

let name = signal(String::new());
let error = move || name.with(|n| n.trim().is_empty()).then_some("Enter your name.");

field_group()
    .child(
        field()
            .invalid(move || error().is_some())
            .child(field_label().child("Name on card"))
            .child(
                input()
                    .value(move || Some(name.get().into()))
                    .on_change(move |value: &String| name.set(value.clone()))
                    .accessibility_label("Name on card"),
            )
            // Empty while there is no error: the part makes nothing.
            .child(field_error().errors(move || FieldErrors::from_iter(error()))),
    )
    .child(
        field()
            .orientation(FieldOrientation::Responsive)
            .child(field_label().child("Card number"))
            .child(input().accessibility_label("Card number"))
            .child(field_description().child("Enter your 16-digit card number.")),
    )
```

## NSH gaps

1. **No labelledby, no legend relation** (Input's G1). A `FieldLabel` cannot name its control and a `FieldLegend`
   cannot name its set: the app sets the control's and the `FieldSet`'s `accessibilityLabel`. An
   `accessibilityLabelledBy` taking an element, as `accessibilityDescribedBy` does, would let `Field` wire both.
2. **`accessibilityDescribedBy` names one element.** ARIA lists several ids; NSH takes one. Workaround: the error while
   it shows, else the description (the error is what the user must act on). A list of targets, or a target whose text
   is the joined text of several, would let both be read.
3. **No container queries.** `responsive` reads the `FieldGroup`'s width from its layout (`on_layout`), so it is known
   one layout pass after the group is first laid out. Verified headless: the width arrives before the first frame is
   sent, and a wide group's responsive fields are never placed vertical (`tests/field.rs` watches every frame), so no
   hiding is needed. A `@container` variant in NSH's Tailwind would make it a class.
4. **No `has-`, `peer-` or `nth-last-` variants.** What would be selected by the children (a `FieldContent` in a
   field, a `Field` in a label, a checkbox group in a set) is told through context instead; `FieldSet`'s tighter gap
   around a checkbox or radio group is the app's `class="gap-3"`; `FieldDescription`'s and `FieldError`'s pull toward
   the part above is the same wherever they stand (`first:` drops it), so no `nth-last-` is needed.
5. **Controls of `nsh/base` without a described-by prop.** The element screen readers reach is `nsh/base`'s in
   `Switch` and `RadioGroup` (their root), `ToggleGroup` (its root), `Combobox` (its input) and `Slider` (its thumbs),
   and those components take no `accessibilityDescribedBy` (only `Checkbox` does); a described-by set on a component
   from outside does not reach its root (verified headless). Inside a `Field` they join for `invalid`, `disabled` and
   the label's press as above, but screen readers do not read the field's description or error with them.
   Workaround: none; the description is still read in reading order, and `FieldError` announces itself. The prop on
   `nsh/base`'s `Switch`, `RadioGroup`, `ToggleGroup`, `Combobox` and `Slider`, passed to that element, would close
   it.
6. **String children** (Button's G0) in TSX until NSH's fix is released.
7. **No handle on `nsh/base`'s `Combobox` input** (Combobox's NSH gaps): no ref, no focus method. So there is no
   `ComboboxRef`, a `FieldLabel`'s press does not reach a combobox (it joins as `Inert`), and a `Label`'s `htmlFor`
   cannot name one. A ref on the input with `focus()` would let it join as `Focus`.

## Decisions

Settled in the P3 review:

1. **No `Form`**: a part binding one form library would tie the library to it; NSH apps keep form state in signals
   or their own library. `Field` takes `invalid` and the error text from it.
2. **`Field`'s `invalid` and `disabled` reach the control** (they are or'ed with the control's own), so the app does
   not write both; Label's G3 already promised this.
3. **`FieldRef` grows to every control** (`foundation::field_ref`, `kind` focus or activate): a press on a label
   focuses a text control and presses a checkbox-like one, as a browser's `<label>`; Label's `htmlFor` takes the new
   refs too. Additive: Label's TSX type widens, each control's body gains one `field::join` call. It lands first.
   Combobox cannot take part yet (NSH gaps 7).
4. **The spacing follows the spatial grammar of gpui-kit's Design Guides**, as tokens derived from `spacing`: 4 between
   the parts of one text (a label or a legend and its description, a control and its description or error), 8 between
   a label and its control, 12 between a legend and its fields, 16 between fields (12 in a nested group). Apps tighten
   them with `spacing` (`spacing: 3` gives 3 / 6 / 9 / 12). No separate desktop values. (The P3 review settled the
   reference's 12 / 6 / 28 / 24, which are off the 4 / 8 / 12 / 16 / 24 scale; the sweep of the fields moved them onto
   it, and a description's place no longer depends on whether a part, even an empty `FieldError`, follows it.)
5. **A choice control in a horizontal field with content is centered on the content's first line** (the label's 20
   line), as Checkbox centers its box on its own label, rather than a fixed `mt-px`, which only fits one text size.
   The same holds the other way: a control taller than the line tells its field where its first line of text is
   (`field::Joined::report_first_line`: an input's middle, a textarea's first line under its padding), and the
   `FieldContent` beside it moves down until its first line is level with it, so a label reads on the line of the text
   it names rather than 6 above it. A slider's track, shorter than the line, is centered on it as a choice control.
6. **`FieldLabel` uses foundation's line heights** (20), not Label's `leading-none`: field labels wrap more often
   (long settings titles), and a label that wraps under `leading-none` collides with itself.
