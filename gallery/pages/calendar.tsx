// The Calendar page: a stay booked over two months, sizes, a single date
// with days that cannot be picked, ranges, a calendar in a card, the app's
// labels under the days, the week's first day.

import { signal, Text, View } from "nsh";
import {
  Badge,
  Button,
  Calendar,
  type CalendarValue,
  Card,
  CardContent,
  CardDescription,
  CardFooter,
  CardHeader,
  CardTitle,
  type Size,
} from "nsh-longbridge";
import { Demo, Demos, Page, Row, Section } from "../ui/layout";
import { TokensSection } from "../ui/tokens";

const sizes: Size[] = ["xs", "sm", "md", "lg"];

/** `YYYY-MM-DD` of a date's local fields. */
function isoDate(d: Date): string {
  return `${d.getFullYear()}-${String(d.getMonth() + 1).padStart(2, "0")}-${String(d.getDate()).padStart(2, "0")}`;
}

/** The local date of an ISO date. */
function fromIsoDate(s: string): Date {
  const [y, m, d] = s.split("-").map(Number);
  return new Date(y, m - 1, d);
}

/** The nights between two ISO dates. */
function nights(start: string, end: string): number {
  return Math.round((fromIsoDate(end).getTime() - fromIsoDate(start).getTime()) / 86_400_000);
}

/** `Dec 20` of an ISO date. */
function short(s: string): string {
  const months = ["Jan", "Feb", "Mar", "Apr", "May", "Jun", "Jul", "Aug", "Sep", "Oct", "Nov", "Dec"];
  const d = fromIsoDate(s);
  return `${months[d.getMonth()]} ${d.getDate()}`;
}

export function CalendarPage() {
  return (
    <Page
      eyebrow="Components"
      title="Calendar"
      description="A month's days to pick a date or a range from, on the page itself: months side by side, days that cannot be picked, the month and year views behind the toggles, and the week starting on the user's day."
    >
      <Demo hero>
        <StayCard />
      </Demo>

      <Section
        title="Sizes"
        description="The cells are the control heights, so a calendar of a size lines up with a button of it. On touch screens every cell is at least 44 tall."
      >
        <Demo class="items-stretch">
          <View class="w-full flex-row flex-wrap items-start gap-6">
            {sizes.map((size) => (
              <View class="gap-2">
                <Text class="text-xs text-muted-foreground">{size}</Text>
                <Calendar class="rounded-lg border border-border" size={size} defaultValue="2026-11-12" />
              </View>
            ))}
          </View>
        </Demo>
      </Section>

      <Section
        title="A single date"
        description="A press picks a day; today has its own square. Days that cannot be picked are dimmed and skipped by the keys: here weekends, and every day before today."
      >
        <SingleDemo />
      </Section>

      <Section
        title="Ranges"
        description="The first press starts a range and the second ends it; between them the pointer previews it. The band rounds at each week's ends, and a range of one day is a single square."
      >
        <Demos>
          <RangeDemo />
          <Demo label="A one-day range">
            <Calendar class="rounded-lg border border-border" range defaultValue={["2026-11-18", "2026-11-18"]} />
          </Demo>
        </Demos>
      </Section>

      <Section
        title="In a card"
        description="Inside a Card or a Popover the calendar has no fill of its own: the surface around it shows through."
      >
        <Demo>
          <Card class="w-fit">
            <CardHeader>
              <CardTitle>
                <Text>Publish date</Text>
              </CardTitle>
              <CardDescription>
                <Text>The post goes live at 9:00 on this day.</Text>
              </CardDescription>
            </CardHeader>
            <CardContent>
              <Calendar defaultValue="2026-12-03" />
            </CardContent>
          </Card>
        </Demo>
      </Section>

      <Section
        title="The app's labels"
        description="renderItem draws a day's label, here a price under the number; the library still draws the cell, the band and every state."
      >
        <PricesDemo />
      </Section>

      <Section
        title="The week's first day"
        description="Left out, the week starts on the user's day from the system's settings. firstDayOfWeek sets it: Sunday 0, Monday 1."
      >
        <Demos>
          {[0, 1].map((first) => (
            <Demo label={first === 0 ? "firstDayOfWeek={0}" : "firstDayOfWeek={1}"}>
              <Calendar class="rounded-lg border border-border" size="sm" firstDayOfWeek={first} defaultValue="2026-11-26" />
            </Demo>
          ))}
        </Demos>
      </Section>

      <TokensSection
        prefix="calendar"
        component="Calendar"
        note="The day's hover, corners and focus ring are the ghost Button's tokens. DatePicker draws its calendar with these tokens too."
      />
    </Page>
  );
}

/** A stay over two months: the nights so far, and a reservation. */
function StayCard() {
  const stay = signal<CalendarValue>(["2026-12-20", "2027-01-03"]);
  const range = () => {
    const v = stay();
    return Array.isArray(v) && v[0] && v[1] ? { start: v[0], end: v[1] } : null;
  };
  return (
    <Card class="w-fit">
      <CardHeader>
        <CardTitle>
          <Text>Cabin by the lake</Text>
        </CardTitle>
        <CardDescription>
          <Text>Pick your arrival and departure.</Text>
        </CardDescription>
      </CardHeader>
      <CardContent>
        <Calendar range numberOfMonths={2} value={stay()} onChange={(e) => stay.set(e.value)} />
      </CardContent>
      <CardFooter class="justify-between gap-4">
        <View class="gap-0.5">
          <Text class="text-sm font-medium text-foreground">
            {range() ? `${short(range()!.start)} – ${short(range()!.end)}` : "No dates yet"}
          </Text>
          <Text class="text-sm text-muted-foreground">
            {range() ? `${nights(range()!.start, range()!.end)} nights` : "Choose two days"}
          </Text>
        </View>
        <Button disabled={!range()}>
          <Text>Reserve</Text>
        </Button>
      </CardFooter>
    </Card>
  );
}

/** One date, weekends and past days out of reach. */
function SingleDemo() {
  const today = isoDate(new Date());
  const day = signal<CalendarValue>(null);
  return (
    <Demo label={`value: ${day() ?? "null"}`}>
      <Calendar
        class="rounded-lg border border-border"
        value={day()}
        onChange={(e) => day.set(e.value)}
        disabledMatcher={(date) => date < today || [0, 6].includes(fromIsoDate(date).getDay())}
      />
    </Demo>
  );
}

/** A range, reported once it is complete. */
function RangeDemo() {
  const period = signal<CalendarValue>(["2026-11-09", "2026-11-20"]);
  const label = () => {
    const v = period();
    return Array.isArray(v) && v[0] && v[1] ? `${v[0]} → ${v[1]}` : "none";
  };
  return (
    <Demo label={`value: ${label()}`}>
      <Calendar class="rounded-lg border border-border" range value={period()} onChange={(e) => period.set(e.value)} />
    </Demo>
  );
}

/** Fares by day: cheaper midweek. */
function PricesDemo() {
  const day = signal<CalendarValue>("2026-11-18");
  const fare = (iso: string | null | undefined) => {
    if (!iso) return "";
    const weekday = fromIsoDate(iso).getDay();
    return weekday === 2 || weekday === 3 ? "$89" : weekday === 5 || weekday === 6 ? "$149" : "$119";
  };
  return (
    <Demo label={`fare: ${fare(typeof day() === "string" ? (day() as string) : null)}`}>
      <Row label="Lisbon">
        <Calendar
          class="rounded-lg border border-border"
          size="lg"
          value={day()}
          onChange={(e) => day.set(e.value)}
          renderItem={(entry) => (
            <View class="items-center">
              <Text>{String(entry.value)}</Text>
              <Text class="text-[10px] leading-3 opacity-70">{fare(entry.date)}</Text>
            </View>
          )}
        />
        <Badge variant="secondary">
          <Text>Midweek from $89</Text>
        </Badge>
      </Row>
    </Demo>
  );
}
