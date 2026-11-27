// The DatePicker page: a meeting scheduled with a date and a time and a
// report's period, sizes beside fields, states, ranges and presets, dates
// and times, the app's own trigger.

import { signal, Text, View } from "nsh";
import {
  Button,
  DatePicker,
  type DatePickerValue,
  type DateRangePreset,
  Input,
  Label,
  Select,
  type SelectItem,
  type Size,
} from "nsh-longbridge";
import { Demo, Demos, Page, Row, Section } from "../ui/layout";
import { TokensSection } from "../ui/tokens";

const sizes: Size[] = ["xs", "sm", "md", "lg"];

const rooms: SelectItem[] = [
  { value: "atlas", title: "Atlas (8 seats)" },
  { value: "harbor", title: "Harbor (4 seats)" },
  { value: "summit", title: "Summit (12 seats)" },
];

/** `YYYY-MM-DD` of a date's local fields. */
function isoDate(d: Date): string {
  return `${d.getFullYear()}-${String(d.getMonth() + 1).padStart(2, "0")}-${String(d.getDate()).padStart(2, "0")}`;
}

/** The ISO date `n` days after `iso`. */
function addDays(iso: string, n: number): string {
  const [y, m, d] = iso.split("-").map(Number);
  return isoDate(new Date(y, m - 1, d + n));
}

const states: { label: string; note: string; value?: string; invalid?: boolean; disabled?: boolean }[] = [
  { label: "Start date", note: "Placeholder" },
  { label: "Due date", note: "With a value", value: "2026-12-15" },
  { label: "Delivery", note: "Invalid", invalid: true },
  { label: "Signed on", note: "Disabled", value: "2026-10-02", disabled: true },
];

export function DatePickerPage() {
  return (
    <Page
      eyebrow="Components"
      title="Date Picker"
      description="A date, a range of dates or a date and time, picked from a calendar that a field opens: a popup at the field on desktops, a sheet at the bottom on touch screens. Presets set the value in one press; a time row edits the time."
    >
      <Demo hero>
        <MeetingForm />
      </Demo>

      <Section
        title="Sizes"
        description="The field is an Input of the same size, so a date picker, a select, a field and a button line up; the calendar's cells take the size too."
      >
        <Demo class="items-stretch">
          {sizes.map((size) => (
            <Row label={size}>
              <DatePicker class="w-48" size={size} defaultValue="2026-11-26" accessibilityLabel={`Date, ${size}`} />
              <View class="flex-1 min-w-32 max-w-48">
                <Input size={size} placeholder="Title" accessibilityLabel={`Title, ${size}`} />
              </View>
              <Button size={size}>
                <Text>Save</Text>
              </Button>
            </Row>
          ))}
        </Demo>
      </Section>

      <Section
        title="States"
        description="Without a value the text and the icon are muted. Invalid turns the edge and the ring destructive; disabled dims it and it does not open. Open, the field keeps its focus edge."
      >
        <Demo class="items-stretch">
          <View class="w-full flex-row flex-wrap gap-x-6 gap-y-6">
            {states.map((s) => (
              <View class="flex-1 min-w-44 gap-2">
                <Label disabled={s.disabled}>
                  <Text>{s.label}</Text>
                </Label>
                <DatePicker
                  class="w-full"
                  defaultValue={s.value}
                  invalid={s.invalid}
                  disabled={s.disabled}
                  accessibilityLabel={s.label}
                />
                <Text class={`text-xs ${s.invalid ? "text-destructive" : "text-muted-foreground"}`}>
                  {s.invalid ? "Pick a delivery day." : s.note}
                </Text>
              </View>
            ))}
          </View>
        </Demo>
      </Section>

      <Section
        title="Ranges and presets"
        description="With range the first press starts the range and the second ends it and closes the popup. Presets sit beside the calendar on desktops and above it in a sheet; past days are out of reach here."
      >
        <PeriodDemo />
      </Section>

      <Section
        title="A date and a time"
        description="timePrecision adds a time row under the calendar: type the digits, or move with the arrows. On desktops the popup stays open for the time and a press on the picked day closes it. The hours follow the user's settings unless hourCycle sets them."
      >
        <Demos>
          <Demo label={'hourCycle="h12"'}>
            <DatePicker
              class="w-64"
              timePrecision="minute"
              hourCycle="h12"
              defaultValue="2026-11-26T14:30"
              accessibilityLabel="Reminder"
            />
          </Demo>
          <Demo label={'hourCycle="h23", seconds, dateFormat="%Y-%m-%d"'}>
            <DatePicker
              class="w-64"
              timePrecision="second"
              hourCycle="h23"
              dateFormat="%Y-%m-%d"
              defaultValue="2026-11-26T09:05:30"
              accessibilityLabel="Log from"
            />
          </Demo>
        </Demos>
      </Section>

      <Section
        title="The app's trigger"
        description="The trigger prop places the app's own element, here an icon button in a toolbar; it opens the popup and screen readers hear it as the date field."
      >
        <ToolbarDemo />
      </Section>

      <TokensSection
        prefix="date-picker"
        component="DatePicker"
        note="The field takes the Input tokens, the popup the Popover tokens and the calendar the Calendar tokens, shared with them."
      />
    </Page>
  );
}

/** A meeting: a title, a room, a start with its time. */
function MeetingForm() {
  const start = signal<DatePickerValue>("2026-12-03T10:00");
  const room = signal<string | null>("harbor");
  return (
    <View class="w-full max-w-[420px] gap-5">
      <View class="gap-1">
        <Text class="text-sm font-medium text-foreground">New meeting</Text>
        <Text class="text-sm text-muted-foreground">Everyone invited gets the time in their own zone.</Text>
      </View>
      <View class="gap-2">
        <Label>
          <Text>Title</Text>
        </Label>
        <Input defaultValue="Quarterly planning" accessibilityLabel="Title" />
      </View>
      <View class="flex-row flex-wrap gap-3">
        <View class="flex-1 min-w-52 gap-2">
          <Label>
            <Text>Starts</Text>
          </Label>
          <DatePicker
            class="w-full"
            timePrecision="minute"
            value={start()}
            onChange={(e) => start.set(e.value)}
            accessibilityLabel="Starts"
          />
        </View>
        <View class="flex-1 min-w-36 max-w-44 gap-2">
          <Label>
            <Text>Room</Text>
          </Label>
          <Select class="w-full" items={rooms} value={room()} onChange={(e) => room.set(e.value)} accessibilityLabel="Room" />
        </View>
      </View>
      <View class="flex-row justify-end gap-2 pt-1">
        <Button variant="ghost">
          <Text>Cancel</Text>
        </Button>
        <Button disabled={start() === null}>
          <Text>Send invites</Text>
        </Button>
      </View>
    </View>
  );
}

/** A report's period: two months, presets from today. */
function PeriodDemo() {
  const today = isoDate(new Date());
  const period = signal<DatePickerValue>([addDays(today, 7), addDays(today, 20)]);
  const presets: DateRangePreset[] = [
    { label: "Today", value: today },
    { label: "Next 7 days", value: [today, addDays(today, 6)] },
    { label: "Next 14 days", value: [today, addDays(today, 13)] },
    { label: "Next 30 days", value: [today, addDays(today, 29)] },
  ];
  return (
    <Demo label={`value: ${JSON.stringify(period())}`}>
      <DatePicker
        class="w-[300px]"
        range
        numberOfMonths={2}
        value={period()}
        onChange={(e) => period.set(e.value)}
        presets={presets}
        disabledMatcher={{ before: today }}
        accessibilityLabel="Report period"
      />
    </Demo>
  );
}

/** A toolbar jumping to a date. */
function ToolbarDemo() {
  const day = signal<DatePickerValue>("2026-11-26");
  return (
    <Demo label={`showing: ${day() ?? "nothing"}`}>
      <View class="flex-row items-center gap-1 rounded-lg border border-border p-1">
        <Button variant="ghost" size="sm">
          <Text>Today</Text>
        </Button>
        <DatePicker
          value={day()}
          onChange={(e) => day.set(e.value)}
          accessibilityLabel="Jump to date"
          trigger={<Button variant="ghost" size="sm" iconOnly icon="calendar" accessibilityLabel="Jump to date" />}
        />
        <Text class="px-2 text-sm text-foreground">{typeof day() === "string" ? (day() as string) : ""}</Text>
      </View>
    </Demo>
  );
}
