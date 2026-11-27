# Calendar

Status: spec (P3), not implemented. A month's days to pick a date or a range of dates from, with Previous and
Next, months side by side, disabled days, and the week starting on the user's day. Reference: shadcn/ui new-york v4
`calendar.tsx` (React DayPicker).

This spec owns the **calendar look** (the header, the weekdays, the day cells, the range band, the month and year
views) and its `calendar-*` tokens. [DatePicker](date-picker.md) draws its calendar with the same look and reads the
same tokens. It also settles **how dates cross TSX and Rust** (see "Dates in TSX and Rust" under API), which
DatePicker follows.

## Usage

- A date or a range of dates shown on the page itself: a booking's nights, a report's period, a schedule beside a
  list, a calendar in a `Card` or a `Popover` of the app's own.
- Search terms: calendar, date grid, month view, day picker, range calendar, date range.

## When not to use

| Need | Use instead |
|---|---|
| A date in a form field that opens a calendar | [DatePicker](date-picker.md) |
| The system's own date wheel or dialog on a phone | `nsh/mobile`'s `DateTimePicker` (not this library's look) |
| A time of day alone | `nsh/base`'s `TimeField` (wrapped later; see DatePicker's NSH gaps) |
| Several separate dates (shadcn's `mode="multiple"`) | Not offered yet: NSH gaps 1 |
| Events laid out on days (an agenda, a week planner) | An app's own view: this is a picker, not a scheduler |

## Built on

`nsh/base`'s `Calendar` (`crates/nsh-base/src/calendar.rs`). It has no look of its own; this library draws every
item and part on its behavior, which is not rewritten:

| Provided by `nsh/base` (not rewritten here) | |
|---|---|
| Value | `value` (controlled) / `defaultValue`; one date, or with `range` a start and an end. A press picks a day; with `range` the first press starts a range and the second ends it (an earlier day starts it again). `onChange({ value })` reports a complete value only; a half range shows until it completes |
| Views | The days of `numberOfMonths` months side by side; the twelve months of the shown year (the month toggle) or a page of twenty years of `yearRange` (the year toggle); picking a month or a year goes back to the days. Previous and Next turn the month, or the page of years |
| Disabled days | `disabledMatcher`: days of the week, before / after a date, a range, or a function, asked once per shown day |
| Keys | The days are one Tab stop (the selected day, else today, else the first); arrows move a day or a week, Home / End to the week's ends, Page Up / Down a month (Shift: a year), Return and Space pick; disabled days are skipped; moving past the shown months turns them. Left and Right swap in a right-to-left layout |
| States on each item | `selected:` (the date, every day of a range, the shown month or year, the open view's toggle), `disabled:`, `focus-visible:` (the day with the keyboard), and on the days the `data-*` attributes `data-range-start`, `data-range-end`, `data-range-middle`, `data-range-preview-start`, `-end`, `-middle` (a half range's start to the day under the pointer or the keyboard), `data-today`, `data-outside` |
| Drawing | Each item's content is drawn once (`renderItem` from a `CalendarEntry`: what it is, never its state); the keys, the pointer and a pick draw nothing again; a turn of the months draws only the days it brings |
| Layout | The grid fills the width it is given: the months share the body, the items of a row share it equally (`flex: 1`); in a popup that fits its content the items keep their own widths |
| Accessibility | Days are buttons, `selected` / `disabled`; weekdays are column headers; the months' titles headers; Previous / Next buttons named by their labels |
| Terminal | `cal`'s month: numbers on the grid, today bold, the other months muted; this library's look does not apply there |

### Differs from shadcn's structure

shadcn's `Calendar` is React DayPicker with classes (`mode`, `selected`, `onSelect`, `classNames`, `components`,
`formatters`). Here, as `nsh/base`'s `Calendar`:

| shadcn (React DayPicker) | This library |
|---|---|
| `mode="single"` / `"range"` | `range={false}` (default) / `range` |
| `mode="multiple"` | Not offered (NSH gaps 1) |
| `selected={date}` + `onSelect={setDate}` (a JS `Date`, `{ from, to }`) | `value` + `onChange({ value })`, ISO strings (see "Dates in TSX and Rust") |
| `numberOfMonths={2}` | `numberOfMonths={2}` |
| `weekStartsOn={1}` (the date-fns locale's by default) | `firstDayOfWeek={1}`; unset: the user's, from `localeSettings()` |
| `disabled={matcher}` | `disabledMatcher={matcher}` (`nsh/base`'s name: `disabled` is the whole control elsewhere in this library) |
| `captionLayout="dropdown"`, `startMonth` / `endMonth` | Always the month and year toggles, which open `nsh/base`'s month and year views; `yearRange={[start, end]}` |
| `showOutsideDays` (default `true`) | Always shown (`nsh/base`'s) |
| `defaultMonth`, `month` / `onMonthChange` | Not offered: the first shown month is the value's, else today's (NSH gaps 5) |
| `showWeekNumber`, `fixedWeeks`, `hidden`, `timeZone` | Not offered (`nsh/base` has none; dates have no time zone here) |
| `buttonVariant` | Not offered: the day cells are ghost buttons; a different look is tokens or `itemClass` |
| `classNames={{ day: … }}`, `components={{ DayButton }}` | Parts: `headerClass` … `itemClass`, `renderItem` (the content of an item, inside the library's cell) |
| `formatters` / `locale` (month and weekday names) | The library's words (`labels`), which the app translates (see "Names") |

### Implementation

- `#[component] fn Calendar` whose root is `nsh_base::calendar()`, with `value`, `defaultValue`, `range`,
  `numberOfMonths`, `disabledMatcher`, `yearRange` and `onChange` passed through. `firstDayOfWeek` is passed as the
  app's, else `locale_settings().first_day_of_week.number() % 7` in a binding (so it follows the user's settings).
- **The look is written into `nsh/base`'s part props, bound** (as `Select` writes its rows', Select's NSH gaps 1):
  `headerStyle`, `bodyStyle`, `monthStyle`, `rowStyle` and `itemStyle` from the tokens, the app's style of the same
  part merged after; `itemClass` generated in a binding from the resolved tokens (the states an inline style cannot
  take: `hover:`, `selected:`, `disabled:`, `focus-visible:`, `data-*:`, each appearance's value under `dark:`), the
  app's `itemClass` appended so its classes win.
- **`renderItem` is the library's**: it draws each item's content from its `CalendarEntry` (a day's band and square
  and number, a weekday's or a month's name, the toggles' text and chevron, Previous / Next's chevrons), calling the
  app's `renderItem` for the label when given (see Parts). The content follows the item's states through a named
  group, `group/nlb-calendar-item` on the item and `group-selected/nlb-calendar-item:`,
  `group-data-range-start/nlb-calendar-item:` … on the content, so a state change draws nothing again.
- The root view around `nsh/base`'s calendar takes the fill, the padding and the app's `class` / `style`.
- Inside a library `Popover` or `Card` (their contexts: `foundation::popup`'s "inside an open popup", a new
  `in_card()`) the fill is transparent, as shadcn's `[[data-slot=popover-content]_&]:bg-transparent` and
  `[[data-slot=card-content]_&]:bg-transparent`.

## API

### Components

| Component | TSX | Rust | Description |
|---|---|---|---|
| `Calendar` | `<Calendar>` | `calendar()` | The calendar: header, weekdays, days; the month and year views |

`nsh/base` exports a `Calendar` too (`nsh_base::calendar()`); an app using both renames one.

### Props

| TSX | Rust | Type | Default | Description |
|---|---|---|---|---|
| `value` | `value` | `CalendarValue` / `CalendarValue` (`nsh/base`'s) | uncontrolled | The value: an ISO date, `[start, end]` with `range`, `null` for none. Set, the app owns it and sets it in `onChange` |
| `defaultValue` | `default_value` | `CalendarValue` / `Date` | `null` | The first value while `value` is not set |
| `range` | `range` | `boolean` | `false` | Pick a range: the first press starts it, the second ends it |
| `numberOfMonths` | `number_of_months` | `number` / `u32` | `1` | Months side by side, at least 1; one in a window narrower than `md` (768), see "Several months" |
| `firstDayOfWeek` | `first_day_of_week` | `number` / `Option<u32>` | the user's | The weeks' first day, `0` Sunday to `6` Saturday; unset, `localeSettings().firstDayOfWeek % 7` |
| `disabledMatcher` | `disabled_matcher` | `DateMatcher` / `Matcher` (`nsh/base`'s) | — | The days that cannot be picked: `[0, 6]` (weekdays, Sunday 0), `{ before, after }`, `{ from, to }`, or `(date: string) => boolean` (pure: asked once per shown day) |
| `yearRange` | `year_range` | `[number, number]` / `YearRange` | 50 years either side of this one | The years the year view pages through, `[start, end)` |
| `size` | `size` | `Size` | `"md"` | The cells, the type and the icons; see Variants and sizes |
| `previousAccessibilityLabel` | `previous_accessibility_label` | `string` | "Go to the previous month" (`labels`) | What screen readers say for Previous |
| `nextAccessibilityLabel` | `next_accessibility_label` | `string` | "Go to the next month" (`labels`) | What screen readers say for Next |
| `renderItem` | `render_item` | `(entry: CalendarEntry) => element` / `Renderer<CalendarEntry>` | — | An item's label in place of the library's (a day's number with a price under it, an event dot); `undefined` keeps the library's for that item. The library still draws the cell, the band, the square and every state. `entry` is what the item is, never its state |
| `class` / `style` | `class` / `style` | | | The root: fill, padding, a border (`class="rounded-md border shadow-sm"`, shadcn's demo) |
| `headerClass` / `headerStyle` | `header_class` / `header_style` | | | The header: Previous, the toggles (or the months' titles), Next |
| `bodyClass` / `bodyStyle` | `body_class` / `body_style` | | | The body: the months side by side, or the month or year view |
| `monthClass` / `monthStyle` | `month_class` / `month_style` | | | Each month of the day view: its weekdays and weeks |
| `rowClass` / `rowStyle` | `row_class` / `row_style` | | | Each row: the weekdays, a week, a row of months or years |
| `itemClass` / `itemStyle` | `item_class` / `item_style` | | | Each item (a day cell, a weekday, a toggle, Previous / Next, a month, a year): `itemStyle` merged after the library's and wins; `itemClass` after the library's generated classes. States are `nsh/base`'s variants (Built on) |

`CalendarValue`, `DateMatcher` and `CalendarEntry` are `nsh/base`'s TypeScript types; the library's generated
declarations declare them too (the same types), so a TSX app imports them from `nsh-longbridge`, as `Select`'s
`SelectItem`.

### Events

| TSX | Rust (`CalendarEvent`) | Payload | When |
|---|---|---|---|
| `onChange` | `Change(Date)` | `{ value: CalendarValue }` | The user picked a complete value: a day, or a range's end. A `value` the app passes is never reported |

`nsh/base`'s, forwarded; shadcn's `onSelect`.

### Parts

| Part | What | Customize with |
|---|---|---|
| root | The calendar's box: fill and padding | `class` / `style` |
| header | Previous, the month and year toggles (one month) or the months' titles (several), Next | `headerClass` / `headerStyle` |
| body | The months, or the month or year view | `bodyClass` / `bodyStyle` |
| month | One month: its weekdays' row and its weeks | `monthClass` / `monthStyle` |
| row | A row of weekdays, days, months or years | `rowClass` / `rowStyle` |
| item | Every pressable or label: a day cell, a weekday, a toggle, a title, Previous, Next, a month, a year | `itemClass` / `itemStyle`, `renderItem` for its label |

Every part is `nsh/base`'s; the library writes its look into them. The day's band and square are the library's
content inside the item (not separate parts): `itemClass` with `group-*` variants restyles them only as far as an
app's class on the item can (a fill on the item shows behind the band); the tokens change them.

### Dates in TSX and Rust

One rule for every date component of this library ([DatePicker](date-picker.md) too), which is `nsh/base`'s:

- **TSX: ISO strings, the forms of HTML's inputs.** A date is `"2026-10-28"` (`<input type="date">`); a range is
  `[start, end]`, either `null` while unset; with a time (DatePicker) `"2026-10-28T09:30"` or `"2026-10-28T09:30:15"`
  (`<input type="datetime-local">`); `null` is none. A value that does not parse is a prop error (logged), as
  `nsh/base`'s.
- **Not JS `Date` objects.** A JS `Date` is an instant (milliseconds since 1970 UTC), not a day on a calendar: the
  same `Date` is the 27th in New York and the 28th in Tokyo, which is why shadcn's calendar needs a `timeZone` prop
  and warns of days off by one. A calendar picks a civil date, which an ISO date string is exactly; the app decides
  which zone it belongs to when it turns it into an instant.
- **Converting in TSX**: build the string from a `Date`'s local fields, never with `toISOString()` (which is UTC and
  can move the day):

  ```ts
  const isoDate = (d: Date) =>
    `${d.getFullYear()}-${String(d.getMonth() + 1).padStart(2, "0")}-${String(d.getDate()).padStart(2, "0")}`;
  const fromIsoDate = (s: string) => {
    const [y, m, d] = s.split("-").map(Number);
    return new Date(y, m - 1, d); // local midnight
  };
  ```

  ISO dates compare as strings (`a < b`) and sort as strings, which is what `disabledMatcher`'s function and a
  range's checks need.
- **Rust: `chrono`.** A date is `chrono::NaiveDate`, a date and time `chrono::NaiveDateTime`, a time
  `chrono::NaiveTime` (no zone, as the strings). The values are `nsh/base`'s enums, re-exported by this library:
  `Date::Single(Option<NaiveDate>)` / `Date::Range(Option<NaiveDate>, Option<NaiveDate>)` (with `From<NaiveDate>`
  and `From<(NaiveDate, NaiveDate)>`), DatePicker's `DateTime` the same over `NaiveDateTime`. The library re-exports
  `chrono` (`nsh_longbridge::chrono`), the version `nsh/base` uses, so a Rust app builds dates with the same crate.
- **Today** is the device's local date (`nsh/base`'s, from the engine's clock and the local offset), the same in
  TSX and Rust.

### Names

Month and weekday names are this library's own words (conventions: English by default, translated by the app):

| Where | English | `labels` keys |
|---|---|---|
| Weekdays over the days | `Su` `Mo` `Tu` `We` `Th` `Fr` `Sa` (React DayPicker's default, `EEEEEE`) | `weekday-0` … `weekday-6` (Sunday 0) |
| The month toggle, the months' titles | `January` … `December` | `month-1` … `month-12` |
| The month view | `Jan` … `Dec` (shadcn's `formatMonthDropdown`, `month: "short"`) | `month-short-1` … `month-short-12` |
| Previous / Next (screen readers) | `Go to the previous month` / `Go to the next month` (React DayPicker's) | `previous-month`, `next-month` |

Years and day numbers are written in ASCII digits. A TSX app translates the names with `Intl` in its translator:

```ts
import { locale } from "nsh";
import { setTranslator } from "nsh-longbridge";

const month = new Intl.DateTimeFormat(locale(), { month: "long" });
const short = new Intl.DateTimeFormat(locale(), { month: "short" });
const weekday = new Intl.DateTimeFormat(locale(), { weekday: "narrow" });
setTranslator((key) => {
  const [kind, n] = [key.replace(/-\d+$/, ""), Number(key.split("-").pop())];
  if (kind === "month") return month.format(new Date(2026, n - 1, 1));
  if (kind === "month-short") return short.format(new Date(2026, n - 1, 1));
  if (kind === "weekday") return weekday.format(new Date(2026, 1, 1 + n)); // 2026-02-01 is a Sunday
  return null; // the library's other words: English, or the app's table
});
```

A Rust app answers the same keys in `labels::set_translator` (from its own translations: NSH has no locale-aware
month names in Rust, NSH gaps 3).

## Variants and sizes

No variants. `size` sets the cell, the type and the icons together (shadcn has one size, `--cell-size: 32`, which is
`md`):

| `size` | Cell (pointer / touch) | Day text | Icons (chevrons) |
|---|---|---|---|
| `xs` | 24 / 44 | 12 | 12 |
| `sm` | 28 / 44 | 13 | 14 |
| `md` | 32 / 44 | 14 | 16 |
| `lg` | 36 / 48 | 14 | 16 |

The pointer cells are the control heights (`control-height-*`), so a `Calendar` of a `size` lines up with a `Button` of
it; touch cells are at least `touch-target` (days touch each other in a row, so a hit slop cannot help: the cells
themselves are big enough, as a menu sheet's rows). gpui-kit's calendar has 28 / 32 / 40 by size, the same density.

### Look

shadcn's values, by part (the cell is `calendar-cell-size-<size>`, 32 at `md`):

| Part | shadcn classes | This library |
|---|---|---|
| root | `group/calendar bg-background p-3 w-fit`; transparent in a popover or a card's content | Fill `calendar-background` (`background`; transparent inside a library `Popover` or `Card`), padding `calendar-padding` (12); as wide as its content (`w-fit`, an internal `app_class`, conventions) |
| header | `nav`: `absolute inset-x-0 top-0 flex w-full items-center justify-between gap-1`; `month_caption`: `flex h-(--cell-size) w-full items-center justify-center px-(--cell-size)` | A row as tall as a cell: Previous at the start, the toggles centered (one month; gap 4) or the titles (several, each `flex: 1`, centered), Next at the end; `calendar-gap` (16) under it |
| Previous / Next | `buttonVariants({ variant: "ghost" })`, `size-(--cell-size) p-0`, `aria-disabled:opacity-50`; `ChevronLeft` / `ChevronRight` `size-4`, rotated in RTL | A ghost button a cell square (`button-radius`, hover `button-ghost-hover` / `-foreground`), the `chevron-left` / `chevron-right` icon (`icon-size-<size>`; swapped in a right-to-left layout), `opacity-disabled` while disabled (the month and year views' ends) |
| month toggle, year toggle | `captionLayout="dropdown"`: `caption_label` `flex h-8 items-center gap-1 rounded-md pr-1 pl-2 text-sm font-medium` with `ChevronDown size-3.5 text-muted-foreground` | Ghost buttons a cell tall: padding start 8, end 4, gap 4, the name (`October`) or the year (`2026`) at `calendar-caption-font-size` (14) `font-weight-medium`, then `chevron-down` at 14 in `calendar-weekday-foreground`; `selected:` (its view open) keeps the hover fill |
| month title (several months) | `caption_label` `text-sm font-medium`, centered over its month | `October 2026` at 14 `font-weight-medium`, centered; a header for screen readers, no press, no hover |
| months | `flex flex-col gap-4 md:flex-row` | Side by side, `calendar-month-gap` (16) apart |
| weekdays row | `weekdays: flex`; `weekday: flex-1 rounded-md text-[0.8rem] font-normal text-muted-foreground` | Each a cell wide (`flex: 1` when wider), text `calendar-weekday-font-size` (12) / 16, `calendar-weekday-foreground` (`muted-foreground`), centered |
| weeks | `week: mt-2 flex w-full` | `calendar-row-gap` (8) above each week (under the weekdays too) |
| day cell (the item) | `day`: `relative aspect-square h-full w-full p-0 text-center`; the range ends' `rounded-l-md bg-accent` / `rounded-r-md bg-accent`, middle `rounded-none`, the week's first / last selected `rounded-l-md` / `rounded-r-md` | A cell square (`flex: 1` when wider); holds the **band** (below) and the **square** |
| day square | `CalendarDayButton`: `Button variant="ghost" size="icon"`, `aspect-square w-full min-w-(--cell-size) flex-col gap-1 leading-none font-normal` | A cell square centered in the cell, corners `button-radius`, the number at `font-size-<size>` regular, centered; with an app's `renderItem`, its label in a centered column, gap 4, second lines at 12 and 70% (shadcn's `[&>span]:text-xs [&>span]:opacity-70`) |
| month view, year view | (shadcn: native `<select>`s in the dropdown caption) | `nsh/base`'s grids: rows of 3 months or 5 years, `calendar-row-gap` apart, each a ghost button a cell tall sharing the row (`flex: 1`), `Jan` / `2026` at `font-size-<size>`; the shown one `selected:` as a selected day |

The header is one row over the months (`nsh/base`'s), not one caption per month; see NSH gaps 2 for several months.

### Day states

The band fills the cell behind the square; the square is the ghost button. Later rows win:

| State (`nsh/base` variant) | Band | Square fill / text | shadcn |
|---|---|---|---|
| Rest | none | none / `calendar-day-foreground` (`foreground`) | ghost button |
| Outside (`data-outside`) | — | — / `calendar-outside-foreground` (`muted-foreground`) | `outside: text-muted-foreground` |
| Today (`data-today`) | — | `calendar-today` (`accent`) / `calendar-today-foreground` (`accent-foreground`) | `today: rounded-md bg-accent text-accent-foreground` |
| Hover (pointer) | — | `button-ghost-hover` / `button-ghost-hover-foreground` | ghost `hover:bg-accent dark:hover:bg-accent/50` |
| Range middle (`data-range-middle`) | `calendar-range` (`accent`) across the cell | none / `calendar-range-foreground` (`accent-foreground`); outside days keep their muted text | `range_middle`, `data-[range-middle=true]:bg-accent text-accent-foreground rounded-none` |
| Range start / end (`data-range-start` / `-end`) | `calendar-range` from the middle of the cell toward the range (start: its end half; end: its start half) | `calendar-selected` (`primary`) / `calendar-selected-foreground` (`primary-foreground`) | `range_start: rounded-l-md bg-accent` + `bg-primary text-primary-foreground` |
| Selected single (`selected:` without a `data-range-*`) | none | `calendar-selected` / `calendar-selected-foreground` | `data-[selected-single=true]:bg-primary text-primary-foreground` |
| Preview (`data-range-preview-middle` / `-end`, a half range) | `calendar-range` as a range's | as a range's middle; the preview's end square as hover | none (React DayPicker has no preview; `nsh/base` gives it, React Aria's look) |
| Focus-visible (keyboard) | — | edge `button-border-focus`, a ring of `ring-width` (3) in `button-ring` (`ring` at 50%), drawn outside the square and over its neighbours | `group-data-[focused=true]/day:border-ring ring-[3px] ring-ring/50 z-10` |
| Disabled (`disabled:`) | — | no hover / `calendar-outside-foreground` at `opacity-disabled` | `disabled: text-muted-foreground opacity-50` |

- **The band's corners** are `button-radius` where the band ends: at a range's start and end (the square is round
  there anyway), and at the first and last column of each week (shadcn's `first-child` / `last-child` rounding), so a
  range wrapping across weeks reads as rows of pills. The column is the day's place in its week, from `entry.date`
  and `firstDayOfWeek`, computed in a binding in the content (no `first:` / `last:` variant needed).
- A range of one day (start = end) has no band: both `data-range-start` and `-end` are set, the band halves cancel.
- In a right-to-left layout the band's halves and corners mirror (`nsh/base` lays the weeks from the right).
- No transitions on the days (shadcn's day buttons transition nothing that shows; a move of the keyboard across the
  grid should not trail).

### Several months

- Side by side, `calendar-month-gap` apart; the header holds one title per month between Previous and Next.
- In a window narrower than `md` (768, Tailwind's breakpoint shadcn uses: `md:flex-row`) the library passes
  `nsh/base` one month: shadcn stacks its months there, but `nsh/base`'s header keeps every title in one row, which
  would leave them over the first month. Previous and Next still turn one month at a time.

## States

The day states are above. Elsewhere:

| Item | Hover | Selected | Disabled | Focus-visible |
|---|---|---|---|---|
| Previous / Next | ghost hover | — | `opacity-disabled`, no hover (the month view; the year view's ends) | ring as a day's |
| Month / year toggle | ghost hover | hover fill while its view is open | — | ring as a day's |
| Month / year of those views | ghost hover | `calendar-selected` / `-foreground` | — | ring as a day's |
| Weekday, month title | none (labels) | — | — | — |

The cursor is the arrow everywhere (gpui-kit's Design Guides: no pointing hand on buttons). Colors change at once.

## Pointer and touch

| | Pointer (desktops, desktop browsers) | Touch (phones, tablets) |
|---|---|---|
| Cell | `control-height-<size>`: 24 / 28 / 32 / 36, square | `max(control-height-touch-<size>, touch-target)`: 44 / 44 / 44 / 48 tall; as wide as the row's share |
| Width | Fits its content (`w-fit`) | The same; in a container that stretches it (a sheet, `class="w-full"`), the cells share the width and the squares stay centered in them, the band spanning the whole cell |
| Hover | The ghost hover on days, toggles and arrows; a half range previews to the day under the pointer | None; a half range previews to the last day the keyboard reached |
| Focus | Edge and ring on the keyboard's day | The edge color only (conventions) |
| Several months | Side by side from `md` | One month below `md` (a phone) |

## Accessibility

`nsh/base`'s (Built on): days are buttons, `selected` / `disabled` heard; weekdays column headers; titles headers;
Previous and Next buttons named by `previousAccessibilityLabel` / `nextAccessibilityLabel`, which this library
fills from `labels` by default (`nsh/base` has none, so a screen reader would say "‹"). The library's content (the
band, the square, the chevrons) is hidden from screen readers. What screen readers hear for a day, a weekday and a
toggle is `nsh/base`'s label, not the drawn name: NSH gaps 4.

## Tokens

The calendar look's tokens, owned by Calendar and read by DatePicker. The day square's hover and focus are Button's
ghost tokens (`button-radius`, `button-ghost-hover`, `button-ghost-hover-foreground`, `button-border-focus`,
`button-ring`; shadcn's day is a ghost `Button`), so a theme that changes ghost buttons changes the days. When
implemented, this table is generated (`<!-- tokens:calendar -->`).

<!-- tokens:calendar -->
| Token | Type | Derived default | Value |
|---|---|---|---|
| `calendar-background` | color | `background` |  |
| `calendar-padding` | length | `spacing × 3` | 12 |
| `calendar-gap` | length | `spacing × 4` | 16 |
| `calendar-month-gap` | length | `spacing × 4` | 16 |
| `calendar-row-gap` | length | `spacing × 2` | 8 |
| `calendar-cell-size-xs` | length | `pointer_touch(control-height-xs, max(control-height-touch-xs, touch-target))` | 24 (touch 44) |
| `calendar-cell-size-sm` | length | `pointer_touch(control-height-sm, max(control-height-touch-sm, touch-target))` | 28 (touch 44) |
| `calendar-cell-size-md` | length | `pointer_touch(control-height-md, max(control-height-touch-md, touch-target))` | 32 (touch 44) |
| `calendar-cell-size-lg` | length | `pointer_touch(control-height-lg, max(control-height-touch-lg, touch-target))` | 36 (touch 48) |
| `calendar-caption-font-size` | length | `text-sm` | 14 |
| `calendar-weekday-font-size` | length | `font-size-xs` | 12 |
| `calendar-weekday-foreground` | color | `muted-foreground` |  |
| `calendar-day-foreground` | color | `button-ghost-foreground` |  |
| `calendar-outside-foreground` | color | `muted-foreground` |  |
| `calendar-today` | color | `accent` |  |
| `calendar-today-foreground` | color | `accent-foreground` |  |
| `calendar-selected` | color | `primary` |  |
| `calendar-selected-foreground` | color | `primary-foreground` |  |
| `calendar-range` | color | `accent` |  |
| `calendar-range-foreground` | color | `accent-foreground` |  |
<!-- /tokens -->

- Names: the day's forms (`today`, `selected`, `range`, `outside`) go where a variant goes (tokens.md), the day being
  the control's own item: `calendar-selected` is the selected day's fill, `calendar-selected-foreground` its number.
- `calendar-weekday-font-size` is 12, not shadcn's `text-[0.8rem]` (12.8): 12 is on the type scale, which
  `line_height` pairs (12 → 16).
- `calendar-caption-font-size` is Tailwind's `text-sm` (14) at every `size`, as shadcn's.
- `calendar-background` is not `card` or `popover`: shadcn's calendar fills with `background` and turns transparent
  inside a card or a popover.
- No `calendar-radius`: the squares and the band read `button-radius`; the root has no corners of its own (shadcn's
  demo adds `rounded-md border` with `class`).
- Other tokens it reads: `font-size-<size>`, `icon-size-<size>`, `font-weight-medium`, `opacity-disabled`,
  `button-*` above.

## Examples

TSX:

```tsx
import { signal, Text, View } from "nsh";
import { Calendar, type CalendarValue } from "nsh-longbridge";

const day = signal<CalendarValue>("2026-11-26");

<Calendar
  class="rounded-md border shadow-sm"
  value={day()}
  onChange={({ value }) => day.set(value)}
/>

// A range over two months; weekends and past days cannot be picked.
const today = isoDate(new Date()); // see "Dates in TSX and Rust"
const trip = signal<CalendarValue>([null, null]);

<Calendar
  range
  numberOfMonths={2}
  value={trip()}
  onChange={({ value }) => trip.set(value)}
  disabledMatcher={(date) => date < today || [0, 6].includes(fromIsoDate(date).getDay())}
/>

// Weeks from Monday whatever the user's settings; small; a price under each day.
<Calendar
  size="sm"
  firstDayOfWeek={1}
  defaultValue="2026-12-01"
  renderItem={(entry) =>
    entry.kind === "day" ? (
      <View class="items-center">
        <Text>{entry.value}</Text>
        <Text>{priceOf(entry.date)}</Text>
      </View>
    ) : undefined
  }
/>
```

With NSH 0.7.0, write a string child inside `<Text>` for now ([Button's G0](button.md#nsh-gaps)); `renderItem`'s
elements follow the same rule. `renderItem` returning `undefined` keeps the library's label for that item.

Rust:

```rust
use nsh_core::prelude::*;
use nsh_longbridge::{CalendarValue, Date, Matcher, calendar, chrono::NaiveDate};

let day = signal(Date::from(NaiveDate::from_ymd_opt(2026, 11, 26).unwrap()));

calendar()
    .app_class(Some("rounded-md border shadow-sm".into()))
    .value(move || CalendarValue::Controlled(day.get()))
    .on_change(move |value: &Date| day.set(*value))

// A range over two months, weekends disabled.
calendar()
    .range(true)
    .number_of_months(2)
    .disabled_matcher(Some(Matcher::from(vec![0, 6])))
```

## NSH gaps

1. **No multiple selection.** `nsh/base`'s value is one date or a range (`Date::Single` / `Date::Range`); shadcn's
   `mode="multiple"` (several separate days) has no counterpart. No workaround inside the library (a day's
   `selected:` comes from the value only). The NSH fix: a `Date::Multiple(Vec<NaiveDate>)` (TS `string[]`) with a
   press toggling a day; the library then adds `multiple` (an addition, not a break).
2. **One header row for every month.** `nsh/base` puts every month's title in one header between Previous and Next,
   so with several months each title is centered in its share of the header, half a cell off its month's center
   (16 at `md`), where shadcn centers each caption over its month. Accepted for P3. The NSH fix: each month's title
   at the top of its month (React DayPicker's and gpui-kit's layout), Previous and Next over the first and last.
3. **No locale-aware date names in Rust.** NSH's JS runtime has `Intl.DateTimeFormat` and the language data behind
   it, but a Rust component cannot reach either, and `chrono`'s names are English. Workaround: the names are the
   library's words (`labels`), English by default, translated by the app (TSX with `Intl`, see "Names"). The NSH
   fix: the month and weekday names of `locale()` (the data `Intl` has) in `nsh-core`; the library would then default
   to them.
4. **Screen readers hear numbers and ISO dates.** `nsh/base` names a day by its ISO date (`2026-10-28`), a weekday
   by its number (`0` to `6`) and the month toggle by its number (`10`), and a library cannot change the label
   `nsh/base` sets on its items. React DayPicker says "Wednesday, October 28th, 2026", "Sunday", "October". The NSH
   fix: a label function for items (an `itemAccessibilityLabel` from a `CalendarEntry`), which the library would fill
   from its names.
5. **No shown month of the app's.** The first shown month is the value's (or today's); shadcn's `defaultMonth` /
   `month` + `onMonthChange` (open on a month without a value, follow the user's turns) have no counterpart. Accepted;
   an NSH `month` / `onMonthChange` pair would allow it.
6. **Today is read once.** `nsh/base` reads today when the calendar is made, so one left open over midnight marks
   yesterday until it is made again. Accepted.
7. **The rows and the items are `nsh/base`'s parts** (as Select's gap 1): their styles from Rust are built once, so
   the library writes the bound style props itself and generates the state classes from the resolved tokens (a
   scope's tokens and a forced appearance reach them, as Select's). Named groups with `data-*` variants
   (`group-data-range-start/nlb-calendar-item:`) are in NSH's class grammar (`docs/dev/styles.md`); a test guards
   that each state reaches the content.
8. **String children** do not apply (the labels are the library's); `renderItem`'s elements follow Button's G0.

## Decisions

To settle in review:

1. **`range` (a boolean, `nsh/base`'s), not shadcn's `mode`**: NSH has no multiple mode yet, and a `mode` with one
   value missing would mislead. When NSH adds multiple selection, a `multiple` boolean beside `range` (as
   `nsh/base` names it) or a `mode` (shadcn's): to decide then.
2. **ISO strings in TSX, `chrono` in Rust**, never JS `Date` (see "Dates in TSX and Rust"); the library re-exports
   `chrono`.
3. **The week starts on the user's day** (`localeSettings()`), where `nsh/base` defaults to Sunday: React DayPicker
   takes it from the locale too, and a Monday-first user sees Monday first without the app doing anything.
4. **`size` with four cells**, shadcn's 32 as `md`, touch cells at least 44.
5. **The names are the library's words** (`labels`, translated by the app), until NSH gives Rust locale names.
6. **One month below `md`**, rather than stacked months under one header row (NSH gaps 2).
7. **The day's hover and focus are Button's ghost tokens**; the day's own forms (`today`, `selected`, `range`) are
   `calendar-*` tokens.
