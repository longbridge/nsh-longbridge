# DatePicker

Status: implemented (P3). A date, a range of dates or a date and time, picked from a
[Calendar](calendar.md) that a field opens: a popup at the field on desktops, a sheet at the bottom on touch
screens. The trigger is drawn by foundation's **field look** (the `input-*` tokens, as [Input](input.md) and
[Select](select.md)); the popup by the **popup surface** (the `popover-*` tokens, owned by [Popover](popover.md));
the calendar by the **calendar look** (the `calendar-*` tokens, owned by [Calendar](calendar.md)).

## Usage

- A date in a form: a due date, a birth date, a delivery day; a range: a report's period, a stay; a date and time: a
  meeting's start, a reminder.
- `confirm`: a choice that costs something to change (a report's period that reloads a dashboard, a filter over many
  rows): nothing is committed until Apply, the popup holds typed Start and End fields, and each preset shows the dates
  it stands for.
- Search terms: date picker, date input, date field, date range picker, date time picker, calendar dropdown.

## When not to use

| Need | Use instead |
|---|---|
| The days always shown on the page | [Calendar](calendar.md) |
| The system's own wheel or dialog on a phone | `nsh/mobile`'s `DateTimePicker` (the system's look, not this library's) |
| A time of day without a date | `nsh/base`'s `TimeField` (this library's later; NSH gaps 7) |
| Typing a date as text in the field itself, or words ("next Friday") | An [Input](input.md) of the app's own with its own parsing; the field is not editable as text (NSH gaps 6). With `confirm` the popup has typed fields (desktops) |
| A month or a year alone | A [Select](select.md) of months or years |

## Built on

`nsh/base`'s `DatePicker` (`crates/nsh-base/src/date_picker.rs`), which holds `nsh/base`'s calendar grid
(`calendar.rs`) and, with `timePrecision`, its time field (`time_field.rs`). It has no look; this library draws the
trigger and every part on its behavior, which is not rewritten:

| Provided by `nsh/base` (not rewritten here) | |
|---|---|
| Presentation | `Select`'s popup (`popup::dropdown`): desktops, a popup at the trigger, at least the trigger's width, growing with the calendar; touch screens, a sheet at the bottom over a backdrop |
| Opening and closing | A press on the trigger or Return opens and closes it; a press outside, the backdrop, Escape and back close it (`onCancel`) |
| Picking | A complete date (a day, a range's end) is the value and closes it. With `timePrecision` (a single date): on desktops the popup stays open for the time, every change is reported, a press on the selected day again closes it; on touch screens a tap on a day picks it at its time and closes the sheet |
| Presets | `presets` (`{ label, value }`): buttons beside the calendar; a press sets the value and closes it; not Tab stops |
| Clearing | Delete and Backspace on the closed trigger clear the value (`onChange(null)`) |
| Value | `value` (controlled) / `defaultValue`, `onChange({ value })`: ISO dates, `[start, end]`, with `timePrecision` dates and times; `defaultTime` for a date picked before any time |
| Focus | Opened on a desktop, the keyboard focus goes to the calendar's stop day; closing gives it back to the trigger (opened by a pointer on macOS and iOS: back where it was), except after a press outside |
| The calendar | Every `Calendar` behavior (keys, views, disabled days, the range's preview) |
| The time field | `TimeField`'s editing (segments, digits, Up / Down, `a` / `p`, a native field behind for the keyboards and screen readers) |
| Accessibility | The trigger is a `combobox` (`expanded`, `hasPopup`), its value the value in `dateFormat` |
| Control | `open` / `defaultOpen` / `onOpenChange({ open, reason })`, `onOpen`, `onClose`, `onCancel` |
| Terminal | A trigger with `▾`, the terminal calendar; this library's look does not apply there |

### One component, not a composition

On the web a date picker is often composed by the app: `Popover > PopoverTrigger > Button` and `PopoverContent >
Calendar`, the app formatting the date (date-fns `format(date, "PPP")`). Here it is one component, because
`nsh/base`'s `DatePicker` already joins the trigger, the popup and the calendar with the platform's presentation (a
sheet on touch screens, where a popover would not be the habit) and the keys between them:

| A web composition | This library |
|---|---|
| `<Popover><PopoverTrigger asChild><Button variant="outline">…</Button></PopoverTrigger><PopoverContent className="w-auto p-0"><Calendar mode="single" …/></PopoverContent></Popover>` | `<DatePicker value onChange placeholder="Pick a date" />` |
| The trigger's text: `format(date, "PPP")`, `<span>Pick a date</span>` muted | `dateFormat` (`chrono`'s) and `placeholder`, drawn by the library in the field |
| `<CalendarIcon />` before the text | The `calendar` icon, drawn by the library |
| `Calendar`'s props (`mode`, `numberOfMonths`, `disabled`) | The same props on `DatePicker` (`range`, `numberOfMonths`, `disabledMatcher`), passed to its calendar |
| A `Select` of presets above the calendar | `presets` (`nsh/base`'s): buttons beside the calendar |
| A separate `<Input type="time">` beside the picker | `timePrecision`: `nsh/base`'s time field in the popup, under the calendar |
| `PopoverContent align="start"` | `align` (default `"start"`) |
| `open` / `onOpenChange={setOpen}` | `open` / `onOpenChange({ open, reason })` |

Why not this library's `Popover` with a `Calendar` in it: that composition would have to rebuild what
`nsh/base`'s `DatePicker` already does (the sheet on touch screens, the focus moving into the days and back, the
trigger's combobox role and value, Return and Delete on the trigger, the time's keep-open rule), against
conventions ("a component wraps its `nsh/base` counterpart where there is one"). The popup still looks exactly like a
`Popover`: the same surface, tokens and motion (`foundation::popup`). An app that wants the composition literally (a
`Button` that opens a `Popover` holding a `Calendar`) can write it with this library's three components.

### Implementation

- `#[component] fn DatePicker` whose root is `nsh_base::date_picker()`, behavior props passed through; `offset` is
  `popover-offset`, `margin` `popover-margin`; `firstDayOfWeek` the app's, else the user's (Calendar's rule);
  `hourCycle` the app's, else `"h12"` where `locale_settings().hour_cycle.twelve_hour()`, else `"h23"`.
- **The value**: the library passes `nsh/base` a controlled `value`, its own signal when the app gives none (from
  `defaultValue`), so the trigger it draws reads it (as `Select`). `onChange` is forwarded; the own signal follows it.
- **The open state**: likewise controlled, the library's own signal when the app gives none, so a library `Button`
  as the app's `trigger` can open it (Popover's NSH gaps 7, `foundation::trigger`), and the popup counts among the
  open popups while open (`foundation::popup::hold_open`, so the trigger's tooltip hides).
- **The trigger** fills `nsh/base`'s `trigger` slot: the library's field (below) unless the app gives `trigger`.
  `nsh/base` binds the role, value, states, focus, keys and press on the slot and places the popup against it.
- **Joining a `Field`** ([Field](field.md)): the body calls `field::join(host, focus)` once. Inside a `Field` it takes
  the field's control slot, so a `FieldLabel`'s press focuses the trigger; `invalid` and `disabled` are its own or the
  field's (either one); the field's described-by target goes on the trigger (the combobox screen readers reach).
  Outside a `Field` nothing changes.
- **The popup and the sheet**: `foundation::popup::surface` on `nsh/base`'s `popupStyle`, as `Select`'s
  (`foundation::popup::set_popup_style`, `PopupStyleProp` implemented for `DatePickerProps`), no padding of
  its own (the calendar's padding is the popup's); in the sheet, the menu's sheet look
  (`menu-sheet-radius`, the top edge, the bottom safe area) and the `overlay` backdrop, as `Select`'s.
- **The calendar** is `nsh/base`'s grid inside the popup: the library writes the calendar look into its parts
  (`headerStyle` … `itemClass`, its `renderItem`) exactly as `Calendar` does, through functions `Calendar`'s file
  shares (one look, not two copies), transparent and with `calendar-padding` around it.
- **Presets, the time row and the segments** are `nsh/base`'s parts, whose Rust styles are built once: their look
  and states follow conventions.md's rule (Drawing: a class generated from the resolved tokens, before the app's
  class; the app's style last). The library fills `timeLabel` with "Time" (`labels`).

### Confirmed mode (`confirm`): composed from `nsh/base`'s parts

`nsh/base`'s `DatePicker` commits a complete date as it is picked, and its popup holds a fixed set of parts (the
presets, the calendar, the time row of a single date): no place for typed fields, a footer, or a line under a preset
(NSH gaps 9–11). So with `confirm` the library composes the popup from `nsh/base`'s public parts, as thin as that
allows, and keeps every behavior it can from them:

| | `nsh/base`'s `DatePicker` (default) | With `confirm` |
|---|---|---|
| Desktop popup | `popup::dropdown` at the trigger | `nsh/base`'s `Popover` at the trigger: the same anchoring, `placement` / `align`, flipping to the other side when only it has room, `popover-offset` / `popover-margin`, at least the field's width |
| Touch screens | A sheet at the bottom over the `overlay` backdrop | `nsh/base`'s `Sheet` at the bottom, the same look (`menu-sheet-radius`, a top edge, the safe area) and backdrop; the footer is the sheet's `footer` |
| Motion | Fade and zoom from the middle (no `onPosition`), `popover-animate` | The same classes (no slide) on desktops; the sheet's own motion |
| Opening | A press on the field or Return on it | The same: a press (`Popover`'s trigger, or the field on touch screens), Return bound on the focused field as `nsh/base` binds it |
| Closing | A complete pick, a preset, Escape, back, a press outside, the backdrop, the field again | Apply; Cancel, Escape, back, a press outside, the backdrop, the field again (all dropping the pending value). A pick or a preset does not close it |
| Clearing | Delete / Backspace on the closed field (`onChange(null)`) | The same, bound the same way |
| Focus | Opened on a desktop: into the calendar's stop day; closing gives it back to the field, except after a press outside; opened by a pointer on macOS and iOS: back where it was | Opened on a desktop: the first typed field (no API moves it into `nsh/base`'s calendar, NSH gaps 13); the rest as `Popover`'s, which follows the same rules |
| The field's accessibility | `combobox`, `expanded`, `hasPopup`, its value in `dateFormat`, `invalid` | The same, set by the library (the `combobox` role and value) and `Popover` (`expanded`, `hasPopup`) |
| The popup's accessibility | No role of its own; the calendar's grid, the presets as buttons | The same; the typed fields are named "Start date" / "End date" ("Date" for a single date), Cancel and Apply are buttons, Apply dimmed until there is a value to apply |
| The calendar | `nsh/base`'s grid inside the date picker | `nsh/base`'s `Calendar` (the same grid, keys, views, disabled days and range preview), controlled with the pending value, in the same look (`calendar::look`) |
| The time | `nsh/base`'s time field, in the time row | `nsh/base`'s `TimeField`, in the same row and look |
| Terminal | `nsh/base`'s terminal picker | The default path: `confirm` does not apply in a terminal |

The library writes only what has no `nsh/base` counterpart: the pending value and Apply / Cancel, the typed fields'
parsing, the presets' column (buttons with their dates) and the footer. It does not reimplement the calendar.

## API

### Components

| Component | TSX | Rust | Description |
|---|---|---|---|
| `DatePicker` | `<DatePicker>` | `date_picker()` | The field, the popup or sheet with the calendar, presets and the time |

`nsh/base` exports a `DatePicker` too; an app using both renames one.

### Props

| TSX | Rust | Type | Default | Description |
|---|---|---|---|---|
| `value` | `value` | `DatePickerValue` / `DatePickerValue` (`nsh/base`'s) | uncontrolled | An ISO date, `[start, end]` with `range`, a date and time with `timePrecision` (`"2026-10-28T09:30"`), `null` for none ([Calendar, "Dates in TSX and Rust"](calendar.md#dates-in-tsx-and-rust)) |
| `defaultValue` | `default_value` | `DatePickerValue` / `DateTime` | `null` | The first value while uncontrolled |
| `range` | `range` | `boolean` | `false` | Pick a range: the first press starts it, the second ends it and closes the popup (with `confirm`, keeps it open) |
| `confirm` | `confirm` | `boolean` | `false` | Nothing is committed until Apply: a pick or a preset fills the calendar and the typed fields, Cancel and Apply at the popup's foot, Escape, a press outside and Cancel drop the pending value; desktops get typed Start / End fields (Date for a single date), each preset its dates. Read once, when the picker is made; no effect in a terminal |
| `placeholder` | `placeholder` | `string` | "Pick a date" (`labels`) | What the field shows without a value |
| `dateFormat` | `date_format` | `string` | `"%b %-d, %Y"` (`Oct 28, 2026`) | How the field shows the value and screen readers hear it, in `chrono`'s format; a range's ends joined by ` - `; with `timePrecision`, the time after a space (`%H:%M`, `%I:%M %p` by `hourCycle`, with `:%S` at seconds) |
| `size` | `size` | `Size` | `"md"` | The field's height, type, padding and icon, and the calendar's cells |
| `disabled` | `disabled` | `boolean` | `false` | It does not open, takes no focus, is dimmed; also while its `Field` is disabled |
| `invalid` | `invalid` | `boolean` | `false` | The app's checks reject the value: the field look's red edge and ring; screen readers say invalid; also while its `Field` is invalid |
| `open` / `defaultOpen` | `open` / `default_open` | `boolean` | uncontrolled / `false` | The popup's open state |
| `timePrecision` | `time_precision` | `"minute"` \| `"second"` / `Option<TimePrecision>` | — | Edit the time of day too (a single date only): a time row under the calendar |
| `hourCycle` | `hour_cycle` | `"h23"` \| `"h12"` / `Option<HourCycle>` | the user's | How the time field counts the hours; unset, `localeSettings()`'s (12-hour where the user's is) |
| `defaultTime` | `default_time` | `string` / `Option<TimeOfDay>` | `"00:00"` | The time a date gets before the user sets one |
| `presets` | `presets` | `DateRangePreset[]` / `Vec<DateRangePreset>` | `[]` | Buttons beside the calendar that set the value: `{ label, value }` ("Today", "Last 7 days"); with `confirm` they fill the calendar instead, and on desktops show the dates they stand for under the label |
| `numberOfMonths` | `number_of_months` | `number` | `1` | As Calendar's (one below `md`) |
| `firstDayOfWeek` | `first_day_of_week` | `number` / `Option<u32>` | the user's | As Calendar's |
| `disabledMatcher` | `disabled_matcher` | `DateMatcher` / `Matcher` | — | As Calendar's |
| `yearRange` | `year_range` | `[number, number]` / `YearRange` | 50 years either side | As Calendar's |
| `placement` | `placement` | `Placement` | `"bottom"` | Desktops: the side of the field the popup goes on |
| `align` | `align` | `Align` | `"start"` | Desktops: the alignment along it: the popup starts where the field does |
| `trigger` | `trigger` | element (slot) | the library's field | The app's own trigger in place of the field (an icon `Button` in a toolbar); it is placed against and announced as the combobox |
| `accessibilityLabel` | `accessibility_label` | `string` | — | What screen readers call it; set it even beside a `Label` (Input's G1) |
| `timeAccessibilityLabel` | `time_accessibility_label` | `string` | "Time" (`labels`) | What screen readers call the time field |
| `renderItem` | `render_item` | as Calendar's | — | As Calendar's: a day's label inside the library's cell (days only) |
| `class` / `style` | `class` / `style` | | | The root, the box the field fills: width and layout (`w-[240px]`, `w-full`) |
| `triggerClass` / `triggerStyle` | `trigger_class` / `trigger_style` | | | The field, merged over the field look |
| `valueClass` / `valueStyle` | `value_class` / `value_style` | | | The field's text (the value or the placeholder) |
| `iconClass` / `iconStyle` | `icon_class` / `icon_style` | | | The field's `calendar` icon |
| `contentClass` / `contentStyle` | `content_class` / `content_style` | | | The popup or sheet surface, as Select's: `contentStyle` wins over the library's look, `contentClass` only adds to it |
| `calendarClass` / `calendarStyle` | `calendar_class` / `calendar_style` | | | The calendar's box in the popup |
| `itemClass` / `itemStyle` | `item_class` / `item_style` | | | The calendar's items, as Calendar's |
| `presetsClass` / `presetsStyle` | `presets_class` / `presets_style` | | | The presets' column (row in a sheet) |
| `presetClass` / `presetStyle` | `preset_class` / `preset_style` | | | Each preset button |
| `presetDescriptionClass` / `presetDescriptionStyle` | `preset_description_class` / `preset_description_style` | | | `confirm`, desktops: each preset's dates line (`hidden` hides it) |
| `fieldsClass` / `fieldsStyle` | `fields_class` / `fields_style` | | | `confirm`, desktops: the row of typed fields above the calendar |
| `footerClass` / `footerStyle` | `footer_class` / `footer_style` | | | `confirm`: the footer with Cancel and Apply (and, in the sheet, the pending value) |
| `timeClass` / `timeStyle` | `time_class` / `time_style` | | | The time row: its label and the time field |
| `timeFieldClass` / `timeFieldStyle` | `time_field_class` / `time_field_style` | | | The time field (the field look around the segments) |
| `segmentClass` / `segmentStyle` | `segment_class` / `segment_style` | | | Each segment of the time (hours, minutes, seconds, AM / PM) |

Not exposed in P3, though `nsh/base` has them: `cleanable` and its clear button (as Select's: added later with a
look; Delete and Backspace on the trigger clear the value), `timeLabel` (the library's "Time"),
`renderSegment`, the calendar's `headerClass` … `rowStyle` (the calendar look; `calendarClass` and `itemClass`
reach what an app restyles), `popupStartingClass` / `popupEndingClass` (the library's motion; `popover-animate` 0 turns
it off), `backdropClass` (the `overlay` token), `offset` / `margin` (the `popover-offset` / `popover-margin`
tokens), the previous / next labels (Calendar's defaults; `labels`).

`DatePickerValue`, `DateRangePreset`, `DateMatcher` and `CalendarEntry` are `nsh/base`'s TypeScript types, declared
again by the library's declarations; in Rust `DateTime`, `DatePickerValue`, `PickedDate`, `DateRangePreset`,
`TimePrecision`, `HourCycle` (`nsh/base`'s two-valued one, not `nsh_core::platform::HourCycle`) and `TimeOfDay` are
`nsh/base`'s, re-exported.

### Events

| TSX | Rust (`DatePickerEvent`) | Payload | When |
|---|---|---|---|
| `onChange` | `Change(PickedDate)` | `{ value: DatePickerValue }` | The user changed the value: a day, a range's end, a preset, the time, Delete / Backspace; with `confirm`, only Apply (and Delete / Backspace) |
| `onOpenChange` | `OpenChange(open, reason)` | `{ open, reason }` | The user asks to open or close it (`"triggerPress"`, `"select"`, `"pressOutside"`, `"cancel"`) |
| `onOpen` / `onClose` | `Open` / `Close` | — | The popup is all there / all gone |
| `onCancel` | `Cancel` | — | Closed without a choice: Escape, back, a press outside or on the backdrop; with `confirm`, Cancel too |

`nsh/base`'s, forwarded (with `confirm`, `Popover`'s and `Sheet`'s, as the same events). Apply reports
`onOpenChange({ open: false, reason: "select" })`, Cancel `"cancel"`. In Rust `PickedDate` holds the `value` (a `DateTime`) and the picker's `time_precision`.

### Ref

TSX: `ref={due}` gives a `DatePickerRef`, into a variable or a signal (`const due = signal<DatePickerRef>()`); Rust: a
`ComponentRef<DatePicker>` (`component_ref::<DatePicker>()`), whose `api::<DatePickerApi>()` has the methods.
`DatePickerRef` joins `FieldRef`'s union, so a `Label`'s `htmlFor` or a `FieldLabel` focuses it (Field, Decisions 3).

| Method | Notes |
|---|---|
| `focus()` | Gives the trigger the keyboard focus, without opening it; does nothing while disabled |

### Parts

| Part | What | Customize with |
|---|---|---|
| root | `nsh/base`'s view: the box the field fills | `class` / `style` |
| trigger | The field: the icon and the value, or the app's `trigger` | `triggerClass` / `triggerStyle` |
| value | The value in `dateFormat`, or the placeholder | `valueClass` / `valueStyle` |
| icon | The `calendar` icon | `iconClass` / `iconStyle` |
| content | The popup or the sheet | `contentClass` / `contentStyle` |
| calendar | The calendar in it | `calendarClass` / `calendarStyle`, `itemClass` / `itemStyle`, `renderItem` |
| presets, preset | The presets' column and each button | `presetsClass` / `presetsStyle`, `presetClass` / `presetStyle` |
| time, time field, segment | The time row, its field, each segment | `timeClass` / `timeStyle`, `timeFieldClass` / `timeFieldStyle`, `segmentClass` / `segmentStyle` |
| preset description | `confirm`, desktops: the dates under a preset's label | `presetDescriptionClass` / `presetDescriptionStyle` |
| fields | `confirm`, desktops: the typed fields' row, each a label over an [Input](input.md) | `fieldsClass` / `fieldsStyle` |
| footer | `confirm`: Cancel and Apply ([Button](button.md)s), in the sheet the pending value above them | `footerClass` / `footerStyle` |

## Variants and sizes

No variants.

### The field

The field look (as `Select`'s trigger): an outline button's edge, fill and shadow, plus the focus ring, `invalid`
and the open state:

| `size` | Height (pointer / touch) | Text (pointer / touch) | Padding | Icon | Calendar cell |
|---|---|---|---|---|---|
| `xs` | 24 / 32 | 12 / 14 | 8 | 12 | 24 / 44 |
| `sm` | 28 / 36 | 13 / 14 | 10 | 14 | 28 / 44 |
| `md` | 32 / 40 | 14 / 16 | 12 | 16 | 32 / 44 |
| `lg` | 36 / 48 | 14 / 16 | 12 | 16 | 36 / 48 |

- Height, text and padding are Input's (`input-height-<size>`, `input-font-size-<size>`, `input-padding-x-<size>`), so
  a DatePicker, a Select and an Input of a `size` line up in a row.
- A row: the `calendar` icon (`icon-size-<size>`), `spacing × 2` (8) after it, then the value (one line, cut with an
  ellipsis, `flex: 1`, start-aligned). Text and icon are `input-foreground` with a value, both
  `input-placeholder-foreground` without one (the whole field reads empty). Regular weight.
- Width: as wide as its content, as Select's (`w-fit`); an app sets a width (`w-[240px]`, `w-[280px]`) with `class`
  so that the field keeps its width as the value changes. Beside a label in a horizontal `Field` it shrinks to share
  the row (`Joined::in_row`, [Field](field.md)), its value truncated.
- The `calendar` icon is lucide's `calendar` (added to `foundation`'s icons).

### The popup

| Part | Look |
|---|---|
| Surface | The popup surface (`popover`, `popover-border`, `popover-radius`, shadow tier `popover-shadow` 3), no padding; at least the field's width, otherwise the calendar's |
| Distance | `popover-offset` (4) from the field, `popover-margin` (4) from the safe area |
| Calendar | The calendar look at the picker's `size`, transparent, padding `calendar-padding` (12) |
| Presets (desktops) | A column before the calendar: padding `calendar-padding`, gap 4, at least `menu-min-width` (128) wide, an edge of `border-width` in `border` between it and the calendar; each preset a ghost button `button-height-sm` tall, padding `menu-item-padding-x` (8), text 13 start-aligned, hover `button-ghost-hover`. Beside the calendar is `nsh/base`'s place (a desktop range picker's habit) |
| Time row | Under the calendar: an edge of `border-width` in `border` above, padding `calendar-padding`, a row with "Time" at the start (`text-sm` / 20, `font-weight-medium`, as `Label`) and the time field at the end, gap 8 |
| Time field | The field look (as Input) at the picker's `size`, as wide as its segments plus `input-padding-x-<size>`; its edge color is a generated class (`focus-within:` changes it, which an inline color would override), the focus edge and (with a pointer) ring on `focus-within:` while the native field inside has the focus |
| Segments | Padding 2 at the sides, corners `radius-xs`, text `input-font-size-<size>` with tabular figures; the edited one (`focus:`) `date-picker-segment-focus` / `-foreground`; `:` between them in `input-placeholder-foreground` |

With `confirm`, the popup's parts are the same, and:

| Part | Look |
|---|---|
| Presets (desktops) | The same column and buttons, each two lines when it has a value: the label, and under it its dates in the field's format (`dateFormat`, a range's ends joined by ` - `, a range of one day as that day) at `font-size-xs` in `date-picker-preset-description`; the button grows to fit (padding 6 above and below, at least `button-height-sm`). The column is as wide as its widest preset, each line cut with an ellipsis if the app narrows it. The preset whose value is the pending one is lit: the ghost hover fill and its text, `selected` for screen readers |
| Fields (desktops) | A row above the calendar, padding `calendar-padding` and none under it (the calendar's follows), gap 8; each field a column: its name ("Start date", "End date"; "Date") at `text-sm` / 20 medium, as `Label`, gap 6, then an [Input](input.md) at the picker's `size`, the two sharing the row |
| Footer (desktops) | Under the calendar (and the time row): an edge of `border-width` in `border` above, padding `calendar-padding`, Cancel (`outline`) and Apply (`default`) at the end, `sm`, gap 8 |
| Footer (sheet) | The sheet's `footer`: an edge above, padding `calendar-padding` and the bottom safe area; the pending value at the start in `text-sm`, muted while there is none ("Pick a date"), then Cancel and Apply sharing the width, `md` (40 tall on touch) |

### Typed fields (`confirm`, desktops)

Typing goes into the field as it is: it is never cleared or rewritten while the field has the focus.

| Rule | |
|---|---|
| Shown | The pending start and end in the date part of the field's format (`dateFormat`, default `%b %-d, %Y`: `Oct 28, 2026`); empty while unset. A pick, a preset or Cancel rewrites them; the placeholder is today in that format, as an example |
| Read | `chrono`'s parsing with that format (month names in any case, short or long, as `chrono` reads `%b` / `%B`; spaces trimmed), else an ISO date (`2026-10-28`), which every format accepts. A format's words are English (NSH gaps 1), so a numeric `dateFormat` is the way to typed dates in another language |
| A date read | Becomes that end of the pending value at once (the calendar turns to the range's start, NSH gaps 14): the start before a later end; the start after the end drops the end (the range starts again there, as a pick there would); an end before the start is refused (invalid) |
| Refused | A date that does not read, an end before the start, a day `disabledMatcher` disables: the pending value keeps that end as it was; Apply is dimmed while a field shows invalid |
| Invalid shown | When the field loses the focus or Return is pressed in it with refused text: the field look's invalid edge (and ring while focused), `invalid` for screen readers. Typing again clears it once the text reads |
| Empty | Unsets that end (Apply waits for it) |
| Leaving the field | A date read is rewritten in the format (`2026-10-28` → `Oct 28, 2026`); refused text stays as typed, invalid |
| Return | Reads the field as leaving it does, then applies when the pending value is complete and no field is invalid |

### Motion

Select's (every popup's): fade and zoom from 95% over `duration`, none with reduced motion, `popover-animate` 0 to turn
it off; no slide and a centered origin (`nsh/base`'s `DatePicker` has no `onPosition`, as `Select`). The sheet has
`nsh/base`'s motion.

## States

The field takes the field look's states (Input's table): rest, focus-visible (`input-border-focus` edge and the
`input-ring` ring), invalid, invalid + focus-visible, disabled (`opacity-disabled`, the arrow cursor). In addition:

| State | Look |
|---|---|
| Hover (pointer) | Fill `date-picker-trigger-hover` (dark only: `input` at 50%; light unchanged), as Select's trigger |
| Placeholder shown | Text and icon in `input-placeholder-foreground` |
| Open (`expanded:`) | Edge `input-border-focus`, no ring (Select's Decisions 2) |
| A half range | The calendar shows its start and the preview (Calendar's states); closing drops it, the value unchanged |
| Preset hover / press | Ghost hover fill (outline hover in the sheet); touch: the press as hover |
| Segment edited | `date-picker-segment-focus` fill and its foreground |
| Pending (`confirm`) | The calendar, the fields and the lit preset show the pending value; the field still shows the committed one |
| Nothing to apply (`confirm`) | Apply disabled (`opacity-disabled`): no date, a range without its end, or a field shown invalid |
| Typed field refused (`confirm`) | Input's invalid state (Typed fields) |

The field's colors, edge and ring transition over `duration` (the field look's); the cursor is the arrow (a date
picker is not text).

## Pointer and touch

| | Desktops and desktop browsers | Phones and tablets |
|---|---|---|
| Field | `input-height-<size>`: 24 / 28 / 32 / 36 | 32 / 36 / 40 / 48, and below `touch-target` a vertical hit slop of (44 − height) / 2, as Input |
| Focus | The edge and the ring | The edge only |
| Form | A popup at the field (`placement`, `align`) | A sheet at the bottom, the menu's sheet look (`menu-sheet-radius`, a top edge, the bottom safe area), the `overlay` backdrop; the calendar spans the sheet, its cells sharing the width (44 tall at `md`) |
| Months | `numberOfMonths` side by side (one below `md`, Calendar's rule) | One in the sheet |
| Presets | A column before the calendar | A wrapping row above the calendar: outline buttons `button-height-sm` (36 on touch), gap 8, padding `calendar-padding` (none under it: the calendar's own padding follows) |
| Picking with a time | The popup stays for the time; the selected day again closes it | A tap on a day picks it at its time and closes the sheet; opening it again edits the time (`nsh/base`) |
| Time field | Typed on the keyboard | The numeric keypad (`nsh/base`'s native field) |
| `confirm` | A popup with typed fields above the calendar, presets with their dates, Cancel / Apply at its foot | A sheet with presets (labels only) above one month, the time row, and a footer: the pending value, then Cancel / Apply sharing the width. No typed fields: a phone's keyboard would cover the calendar, and a tap is the faster way to a day |

The sheet on touch screens is `nsh/base`'s choice and every touch platform's habit for a picker of this size (iOS's
and Android's date pickers come up from the bottom or as a dialog, not anchored to the field).

## Accessibility

`nsh/base`'s (Built on): the field is a `combobox`, expanded while open, its value the value in `dateFormat`;
the calendar's (Calendar's Accessibility); the time field's native field with its segments. The library adds
`invalid` to the field's accessibility state and hides the icon from screen readers. `accessibilityLabel` names the
picker (Input's G1: no labelledby between elements); `timeAccessibilityLabel` the time field. Inside a `Field`, its
description and error describe the trigger ([Field](field.md)).

With `confirm`: the typed fields are text fields named "Start date" and "End date" ("Date"), `invalid` while refused;
each preset is a button named by its label, its dates its hint, `selected` while its value is the pending one; Cancel
and Apply are buttons, Apply `disabled` while there is nothing to apply. Tab goes from the fields to the calendar's stop
day and its arrows, then Cancel and Apply (the presets are not Tab stops, as `nsh/base`'s). In the sheet, the pending
value line is read before the buttons.

## Tokens

The field reads the `input-*` tokens ([Input](input.md#tokens)), the popup the `popover-*` tokens
([Popover](popover.md#tokens)), the sheet and the presets' row geometry the `menu-*` tokens
([DropdownMenu](dropdown-menu.md#tokens)), the calendar the `calendar-*` tokens ([Calendar](calendar.md#tokens)), the
presets Button's ghost and outline tokens. Its own tokens are what the picker alone has. This table is generated
(`<!-- tokens:date-picker -->`).

<!-- tokens:date-picker -->
| Token | Type | Derived default | Value |
|---|---|---|---|
| `date-picker-trigger-hover` | color | `select-trigger-hover` |  |
| `date-picker-segment-focus` | color | `menu-item-focus` |  |
| `date-picker-segment-focus-foreground` | color | `menu-item-focus-foreground` |  |
| `date-picker-preset-description` | color | `muted-foreground` |  |
<!-- /tokens -->

- `date-picker-trigger-hover` follows Select's trigger: two fields that open a list look the same under the pointer.
- The segment's highlight is the menu's row highlight (`accent`), the "the keys act here" color across the library.
- `date-picker-preset-description` is the muted text of descriptions across the library (`popover-description-foreground`,
  a field's description).
- No `date-picker-radius`, `-height-*` or `-border`: the field is a field, the popup a popup, the calendar a calendar;
  a different date picker uses `triggerClass`, `contentStyle` or a scope.

## Examples

TSX:

```tsx
import { signal } from "nsh";
import { Button, DatePicker, type DatePickerValue, Field, FieldError, FieldLabel } from "nsh-longbridge";

const due = signal<DatePickerValue>(null);

<DatePicker
  class="w-[240px]"
  value={due()}
  onChange={({ value }) => due.set(value)}
  accessibilityLabel="Due date"
/>

// A range over two months, from Monday, with presets.
const stay = signal<DatePickerValue>(["2026-12-20", "2027-01-03"]);
const today = isoDate(new Date()); // Calendar, "Dates in TSX and Rust"

<DatePicker
  class="w-[300px]"
  range
  numberOfMonths={2}
  firstDayOfWeek={1}
  value={stay()}
  onChange={({ value }) => stay.set(value)}
  disabledMatcher={{ before: today }}
  presets={[
    { label: "This week", value: [weekStart(today), weekEnd(today)] },
    { label: "Next 7 days", value: [today, addDays(today, 6)] },
  ]}
  accessibilityLabel="Stay"
/>

// A date and time, 24-hour whatever the user's settings, a numeric format.
<DatePicker
  timePrecision="minute"
  hourCycle="h23"
  dateFormat="%Y-%m-%d"
  defaultTime="09:00"
  defaultValue="2026-11-26T09:00"
  accessibilityLabel="Meeting start"
/>

// In a Field: the label focuses it, the field's error describes it and makes it invalid.
<Field invalid={due() === null}>
  <FieldLabel>Due date</FieldLabel>
  <DatePicker value={due()} onChange={({ value }) => due.set(value)} accessibilityLabel="Due date" />
  <FieldError>Pick a due date.</FieldError>
</Field>

// An icon button in a toolbar opens it.
<DatePicker
  trigger={<Button variant="ghost" iconOnly icon="calendar" accessibilityLabel="Jump to date" />}
  onChange={({ value }) => jumpTo(value)}
/>

// A report's period: two months, presets showing their dates, typed fields, nothing reloads until Apply.
const period = signal<DatePickerValue>([addDays(today, -6), today]);

<DatePicker
  class="w-[280px]"
  range
  confirm
  numberOfMonths={2}
  value={period()}
  onChange={({ value }) => period.set(value)}
  disabledMatcher={{ after: today }}
  presets={[
    { label: "Last 7 days", value: [addDays(today, -6), today] },
    { label: "Last 30 days", value: [addDays(today, -29), today] },
    { label: "Last month", value: [monthStart(addMonths(today, -1)), monthEnd(addMonths(today, -1))] },
    { label: "Year to date", value: [yearStart(today), today] },
  ]}
  accessibilityLabel="Report period"
/>
```

`weekStart`, `weekEnd`, `addDays`, `addMonths`, `monthStart`, `monthEnd` and `yearStart` are the app's helpers on ISO strings. With NSH 0.7.0, a string child is written
inside `<Text>` for now ([Button's G0](button.md#nsh-gaps)).

Rust:

```rust
use nsh_core::prelude::*;
use nsh_longbridge::{
    DatePickerValue, DateTime, PickedDate, Size, TimePrecision, chrono::NaiveDate, date_picker,
};

let due = signal(DateTime::Single(None));

date_picker()
    .app_class(Some("w-[240px]".into()))
    .value(move || DatePickerValue::Controlled(due.get()))
    .on_change(move |picked: &PickedDate| due.set(picked.value))
    .accessibility_label("Due date")

// A date and time, small.
date_picker()
    .size(Size::Sm)
    .time_precision(Some(TimePrecision::Minute))
    .default_value(DateTime::from(
        NaiveDate::from_ymd_opt(2026, 11, 26).unwrap().and_hms_opt(9, 0, 0).unwrap(),
    ))
    .accessibility_label("Meeting start")

// A report's period, applied with Apply.
let period = signal(DateTime::from((d(2026, 9, 1), d(2026, 9, 30))));
date_picker()
    .range(true)
    .confirm(true)
    .number_of_months(2)
    .value(move || DatePickerValue::Controlled(period.get()))
    .on_change(move |picked: &PickedDate| period.set(picked.value))
    .presets(vec![
        DateRangePreset::range("Last month", d(2026, 10, 1), d(2026, 10, 31)),
        DateRangePreset::range("This quarter", d(2026, 10, 1), d(2026, 12, 31)),
    ])
    .accessibility_label("Report period")
```

## NSH gaps

1. **The field's text is `chrono`'s, in English.** `dateFormat` is a `chrono` format, whose month and weekday names
   are English (`%b` → `Oct`), and `nsh/base` hears the value in the same format, so screen and screen reader agree.
   Workaround: an app in another language sets a numeric format (`"%Y-%m-%d"`, `"%d.%m.%Y"`, `"%Y/%m/%d"`); the
   default is English (`LLL dd, y` without the zero). The NSH fix is Calendar's NSH gaps 3 (locale
   names in Rust), with a locale-aware default format (`Intl`'s `dateStyle: "medium"`).
2. **The parts are `nsh/base`'s** (Select's gap 1): `popupStyle`, the calendar's part styles, the presets', the time
   row's and the segments' styles from Rust are built once; their look follows conventions.md's rule (Drawing).
   ★ Verified: a headless test changes a token while the popup is open and the popup, a day and a segment follow
   it (`tests/date_picker.rs`); no fallback needed.
3. **No `onPosition`**: no slide toward the field and a centered zoom origin, as `Select` (Tooltip's gap 3).
4. **The app's pressable as `trigger`** takes the press (Popover's NSH gaps 7): handled for this library's `Button`
   by `foundation::trigger` with the open state the library holds; an app's own pressable is not covered.
5. **The time field's focus is inside it**: `nsh/base`'s time field keeps a native text field behind the segments,
   which takes the focus; the field look's focus states are drawn on `focus-within:` of the time field's view, not
   `focus-visible:`. ★ Verified: a headless test focuses the time field and its view takes the focus edge and the
   ring, and the rest edge again once Tab leaves it; no fallback needed.
6. **Not editable as text.** `nsh/base`'s trigger is a button-like combobox; the web's "input" date picker (an `Input`
   that parses what is typed, with a calendar button) has no counterpart. An app builds it from `Input` and a
   `Popover` with a `Calendar`; a `DateField` of segments (React Aria's) would be the NSH answer, as `TimeField` is
   for times.
7. **No `TimeField` of this library yet.** `nsh/base` has one (used here inside the popup); a standalone `TimeField`
   with the field look and the segments' look of this spec comes later, not in P3.
8. **The sheet's condition is `nsh/base`'s** (`!is_desktop()`, private); the library computes the same
   (`foundation::menu::list_as_sheet`), as Select (DropdownMenu's gap 2).
9. **No parts of the app's or the library's in `nsh/base`'s popup.** Its content is fixed: the presets, the calendar,
   the time row (a single date with a time). A footer (Cancel / Apply) or typed fields need a `header` and a `footer`
   slot (or a `renderContent`) on `nsh/base`'s `DatePicker`, and a way to keep it open after a complete pick (a
   `closeOnSelect: false`, its picks reported as the pending value). Workaround: `confirm` composes the popup from
   `Popover`, `Sheet`, `Calendar` and `TimeField` (Built on). With those in `nsh/base`, `confirm` would go back to
   `nsh/base`'s `DatePicker` without an API change.
10. **A preset is a label only.** `nsh/base`'s preset button draws its `label` as one text, so the default path cannot
    show the dates under it in a muted line. Needs a `description` on `DateRangePreset` (drawn as a second part with
    `presetDescriptionClass` / `-Style`), or a `renderPreset` slot. Until then the dates show only with `confirm`, where
    the library draws the presets.
11. **No date field of segments.** `nsh/base` has a `TimeField` but no `DateField` (React Aria's), so the typed
    fields are text [Input](input.md)s parsed by the library (Typed fields), not segments with Up / Down per part and
    a locale's order. A `DateField` (and its range pair) in `nsh/base` would replace them.
12. **A half range is not reported.** `nsh/base`'s `Calendar` reports a complete value only (`onChange`); while a
    range is half picked its start lives inside the grid. With `confirm`, the typed fields and Apply follow the
    calendar once the range is complete: after a first press the fields still show the previous range and Apply would
    apply it. Needs the half value reported (`onChange` with `[start, null]`, or an `onPendingChange`).
13. **No focus method on `Calendar`.** `nsh/base`'s `DatePicker` moves the focus into its grid as it opens
    (`focus_days`, private); a `Calendar` has no ref method for it. With `confirm` the focus goes to the first typed
    field instead (a good start for typing; Tab reaches the days).
14. **A value turns the calendar to its start.** `nsh/base`'s grid shows the month of the value's start when the value
    changes; there is no way to show the end's month (a typed end far from the start). With two months side by side
    the end is often in view; otherwise the user turns the months. Needs a `month` (the shown month) prop on
    `Calendar`.

## Decisions

Settled in review:

1. **Built on `nsh/base`'s `DatePicker`, not this library's `Popover` + `Calendar`** (see "One component, not a
   composition"): the platform's sheet, the focus and the keys come with it; the popup looks like a `Popover`
   (`foundation::popup`).
2. **The trigger is the field look**, not an outline `Button`: the same edge, fill and shadow, plus the focus
   ring, `invalid`, the open state and Input's heights, so it lines up with fields in a form. The app's own trigger
   is the `trigger` slot.
3. **`dateFormat` defaults to `"%b %-d, %Y"`** (`Oct 28, 2026`) for single dates and ranges: date-fns' `PPP`
   (`October 28th, 2026`) needs ordinals `chrono` has not, and a range in it overflows a 300 wide field.
4. **The calendar takes the picker's `size`**, as Select's rows take its `size`.
5. **`hourCycle` and `firstDayOfWeek` default to the user's** (`localeSettings()`), where `nsh/base` defaults to 24-hour
   and Sunday.
6. **Presets beside the calendar on desktops, a row above it in a sheet** (`nsh/base`'s place; the look of the common
   preset buttons).
7. **`cleanable` waits**, as Select's (Decisions 4); Delete and Backspace clear.
8. **It joins `Field`** (`field::join`, kind `focus`) and has a `DatePickerRef` with `focus()`.
9. **No `TimeField` component in P3.**

Settled in implementation:

9. **One month in the sheet**, whatever `numberOfMonths` and the window's width: the sheet is a phone's or a
   tablet's, where months side by side under one header row do not read (Calendar's NSH gaps 2).
10. **The time field's edge color is a class**, with its `focus-within:` twin: an inline color would win over the
    focus class, so the focus edge would never show.
11. **Presets in the sheet are outline buttons**, a wrapping row with no padding under it (the calendar's padding
    follows), so the row and the calendar keep one gap.

Settled in implementation (range presets, Apply, typed fields):

12. **`confirm`, an opt-in boolean, for every kind of value** (a date, a range, a date and time), not a range-only
    `applyMode`: the date and time picker benefits most (on desktops `nsh/base` reports every change of the time and
    closes only on a second press of the selected day, which nobody finds; on touch screens it closes on the first
    tap, and the time needs a second opening). The default path does not change.
13. **Preset dates are automatic with `confirm`** (a preset stands for dates, and seeing them is the point of a
    dashboard's "Last month"); `presetDescriptionClass="hidden"` hides them. Not in the sheet: two-line buttons in a
    wrapping row would take half a phone's height, and the footer's pending value shows what a tap picked.
14. **Typed fields on desktops only.** On touch screens the keyboard would cover the calendar of the sheet.
15. **Invalid is shown on leaving the field or Return**, never while typing (every date is invalid halfway through
    being typed), and typed text is never cleared (the user fixes it).
16. **`numberOfMonths` keeps its default of 1**, with `confirm` too: a default does not change silently. A desktop
    range picker should set `numberOfMonths={2}` (the examples do); one month shows below `md` and in the sheet.
17. **Apply reports `onOpenChange` with `"select"` and Cancel with `"cancel"`** (and `onCancel`), `nsh/base`'s reasons
    for a choice and a dismissal, so an app's handling of the two paths is one.
18. **Return in the calendar picks a day** (`nsh/base`'s key), not Apply; Return in a typed field applies.
