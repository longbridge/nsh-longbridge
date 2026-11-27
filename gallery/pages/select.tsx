// The Select page: a report's schedule, sizes beside fields and buttons,
// states, groups and a long list, a search field and a clear button, the
// app's rows, over the trigger or beside it.

import { signal, Text, View } from "nsh";
import { Badge, type BadgeVariant, Button, Input, Label, Select, type SelectItem, type Size } from "nsh-longbridge";
import { Demo, Demos, Page, Row, Section } from "../ui/layout";
import { TokensSection } from "../ui/tokens";

const sizes: Size[] = ["xs", "sm", "md", "lg"];

const zones: SelectItem[] = [
  { value: "est", title: "Eastern Time (ET)", group: "North America" },
  { value: "cst", title: "Central Time (CT)", group: "North America" },
  { value: "pst", title: "Pacific Time (PT)", group: "North America" },
  { value: "gmt", title: "Greenwich Mean Time (GMT)", group: "Europe" },
  { value: "cet", title: "Central European Time (CET)", group: "Europe" },
  { value: "eet", title: "Eastern European Time (EET)", group: "Europe" },
  { value: "ist", title: "India Standard Time (IST)", group: "Asia" },
  { value: "sgt", title: "Singapore Time (SGT)", group: "Asia" },
  { value: "jst", title: "Japan Standard Time (JST)", group: "Asia" },
];

const frequencies: SelectItem[] = [
  { value: "daily", title: "Every day" },
  { value: "weekly", title: "Every week" },
  { value: "monthly", title: "Every month" },
  { value: "hourly", title: "Every hour (Enterprise)", disabled: true },
];

const formats: SelectItem[] = [
  { value: "pdf", title: "PDF" },
  { value: "csv", title: "CSV" },
  { value: "xlsx", title: "Excel" },
];

const fruits: SelectItem[] = [
  { value: "apple", title: "Apple" },
  { value: "banana", title: "Banana" },
  { value: "blueberry", title: "Blueberry" },
  { value: "grapes", title: "Grapes", disabled: true },
  { value: "pineapple", title: "Pineapple" },
];

// Countries by region, for a searchable list in groups.
const regions: Record<string, string[]> = {
  Africa: ["Algeria", "Egypt", "Ethiopia", "Ghana", "Kenya", "Morocco", "Nigeria", "Rwanda", "Senegal", "South Africa", "Tanzania", "Tunisia", "Uganda"],
  Americas: ["Argentina", "Brazil", "Canada", "Chile", "Colombia", "Costa Rica", "Ecuador", "Mexico", "Panama", "Peru", "United States", "Uruguay"],
  Asia: ["China", "India", "Indonesia", "Israel", "Japan", "Malaysia", "Nepal", "Philippines", "Saudi Arabia", "Singapore", "South Korea", "Thailand", "Turkey", "United Arab Emirates", "Vietnam"],
  Europe: ["Austria", "Belgium", "Denmark", "Finland", "France", "Germany", "Greece", "Iceland", "Ireland", "Italy", "Netherlands", "Norway", "Poland", "Portugal", "Spain", "Sweden", "Switzerland", "United Kingdom"],
  Oceania: ["Australia", "Fiji", "New Zealand", "Papua New Guinea", "Samoa"],
};
const countries: SelectItem[] = Object.entries(regions).flatMap(([region, names]) =>
  names.map((name) => ({ value: name.toLowerCase().replaceAll(" ", "-"), title: name, group: region })),
);

const people: SelectItem[] = [
  { value: "ada", title: "Ada Lovelace" },
  { value: "alan", title: "Alan Turing" },
  { value: "barbara", title: "Barbara Liskov" },
  { value: "edsger", title: "Edsger Dijkstra" },
  { value: "grace", title: "Grace Hopper" },
  { value: "ken", title: "Ken Thompson" },
  { value: "margaret", title: "Margaret Hamilton" },
  { value: "radia", title: "Radia Perlman" },
];

const issueStatuses: SelectItem[] = [
  { value: "backlog", title: "Backlog" },
  { value: "todo", title: "Todo" },
  { value: "progress", title: "In progress" },
  { value: "done", title: "Done" },
  { value: "canceled", title: "Canceled" },
];

const priorities: SelectItem[] = [
  { value: "urgent", title: "Urgent" },
  { value: "high", title: "High" },
  { value: "medium", title: "Medium" },
  { value: "low", title: "Low" },
];

// Ten thousand rows: the search narrows them, only those in view are made.
const many: SelectItem[] = Array.from({ length: 10000 }, (_, i) => ({ value: String(i + 1), title: `Item ${i + 1}` }));

// Enough rows to scroll: only those in view are made.
const years: SelectItem[] = Array.from({ length: 120 }, (_, i) => {
  const year = String(2026 - i);
  return { value: year, title: year };
});

const statuses: { value: string; title: string; variant: BadgeVariant; note: string }[] = [
  { value: "backlog", title: "Backlog", variant: "outline", note: "Not planned yet" },
  { value: "todo", title: "Todo", variant: "secondary", note: "Planned for this cycle" },
  { value: "progress", title: "In progress", variant: "default", note: "Someone is on it" },
  { value: "canceled", title: "Canceled", variant: "destructive", note: "Will not be done" },
];

const states: { label: string; note: string; value?: string; invalid?: boolean; disabled?: boolean; cleanable?: boolean }[] = [
  { label: "Fruit", note: "Placeholder" },
  { label: "Favorite", note: "With a value, cleanable", value: "banana", cleanable: true },
  { label: "Snack", note: "Invalid", invalid: true },
  { label: "Dessert", note: "Disabled, cleanable", value: "apple", disabled: true, cleanable: true },
];

export function SelectPage() {
  return (
    <Page
      eyebrow="Components"
      title="Select"
      description="One value from a list: a time zone, a format, a status. A trigger drawn as a field opens the list at it (over it on macOS, the chosen row on the trigger), or a sheet at the bottom of a phone."
    >
      <Demo hero>
        <ScheduleForm />
      </Demo>

      <Section
        title="Sizes"
        description="The trigger is an Input of the same size, so a select, a field and a button line up; the list's rows take the size too."
      >
        <Demo class="items-stretch">
          {sizes.map((size) => (
            <Row label={size}>
              <View class="w-44">
                <Select class="w-full" size={size} items={formats} defaultValue="pdf" accessibilityLabel={`Format, ${size}`} />
              </View>
              <View class="flex-1 min-w-32 max-w-56">
                <Input size={size} placeholder="File name" accessibilityLabel={`File name, ${size}`} />
              </View>
              <Button size={size}>
                <Text>Export</Text>
              </Button>
            </Row>
          ))}
        </Demo>
      </Section>

      <Section
        title="States"
        description="The placeholder is muted until a value is chosen. Invalid turns the edge and the ring destructive; disabled dims and does not open; an item can be disabled alone (Grapes). With cleanable, a cross takes the chevron's place while a value is set; disabled dims it too."
      >
        <Demo class="items-stretch">
          <View class="w-full flex-row flex-wrap gap-x-6 gap-y-6">
            {states.map((s) => (
              <View class="flex-1 min-w-36 gap-2">
                <Label disabled={s.disabled}>
                  <Text>{s.label}</Text>
                </Label>
                <Select
                  class="w-full"
                  items={fruits}
                  defaultValue={s.value}
                  placeholder="Select a fruit"
                  invalid={s.invalid}
                  disabled={s.disabled}
                  cleanable={s.cleanable}
                  accessibilityLabel={s.label}
                />
                <Text class={`text-xs ${s.invalid ? "text-destructive" : "text-muted-foreground"}`}>
                  {s.invalid ? "Choose a fruit to go on." : s.note}
                </Text>
              </View>
            ))}
          </View>
        </Demo>
      </Section>

      <Section
        title="Groups and long lists"
        description="Adjacent items of one group show under its header. A long list scrolls and makes only the rows in view; type to jump to an item."
      >
        <Demos>
          <Demo label="Time zones in groups">
            <View class="w-64">
              <Select class="w-full" items={zones} placeholder="Select a time zone" accessibilityLabel="Time zone" />
            </View>
          </Demo>
          <Demo label="120 years: type 19 to reach the 1900s">
            <View class="w-40">
              <Select class="w-full" items={years} defaultValue="1990" accessibilityLabel="Year of birth" />
            </View>
          </Demo>
        </Demos>
      </Section>

      <Section
        title="Search and clear"
        description="searchable puts a search field at the top of the list, focused on opening: typing narrows the options by title, groups without a match go, the arrows move the highlight and Return chooses. cleanable adds a cross while a value is set; Delete or Backspace on the focused trigger clears it too."
      >
        <Demos>
          <Demo label="Countries in groups: type to narrow">
            <CountryDemo />
          </Demo>
          <Demo label="10 000 items: only the rows in view are made">
            <View class="w-56">
              <Select class="w-full" items={many} searchable searchPlaceholder="Search items" placeholder="Select an item" accessibilityLabel="Item" />
            </View>
          </Demo>
        </Demos>
        <FilterBar />
      </Section>

      <Section
        title="The app's rows"
        description="The renderItem prop draws a row's content; the library still draws the row, its highlight and the check on the chosen one."
      >
        <StatusDemo />
      </Section>

      <Section
        title="Over the trigger or beside it"
        description="Without cover, the list takes the platform's way: on macOS the list opens with the chosen row over the trigger, elsewhere below it. cover sets either everywhere."
      >
        <Demos>
          {[true, false].map((cover) => (
            <Demo label={cover ? "cover={true}: the chosen row over the trigger" : "cover={false}: below the trigger"} class="pt-24">
              <View class="w-48">
                <Select class="w-full" items={fruits} defaultValue="blueberry" cover={cover} accessibilityLabel={cover ? "Covering" : "Below"} />
              </View>
            </Demo>
          ))}
        </Demos>
      </Section>

      <TokensSection
        prefix="select"
        component="Select"
        note="The trigger takes the Input tokens and the list the menu tokens (DropdownMenu's), shared with them."
      />
    </Page>
  );
}

/** A report's schedule: a time zone in groups, a frequency with an item for another plan, a format. */
function ScheduleForm() {
  const zone = signal<string | null>("cet");
  const frequency = signal<string | null>("weekly");
  const format = signal<string | null>(null);
  const ready = () => zone() !== null && frequency() !== null && format() !== null;
  return (
    <View class="w-full max-w-[400px] gap-5">
      <View class="gap-1">
        <Text class="text-sm font-medium text-foreground">Schedule the revenue report</Text>
        <Text class="text-sm text-muted-foreground">Sent by email to the finance team.</Text>
      </View>
      <View class="gap-2">
        <Label>
          <Text>Time zone</Text>
        </Label>
        <Select class="w-full" items={zones} value={zone()} onChange={(e) => zone.set(e.value)} accessibilityLabel="Time zone" />
      </View>
      <View class="flex-row gap-3">
        <View class="flex-1 gap-2">
          <Label>
            <Text>Frequency</Text>
          </Label>
          <Select class="w-full" items={frequencies} value={frequency()} onChange={(e) => frequency.set(e.value)} accessibilityLabel="Frequency" />
        </View>
        <View class="flex-1 gap-2">
          <Label>
            <Text>Format</Text>
          </Label>
          <Select
            class="w-full"
            items={formats}
            value={format()}
            onChange={(e) => format.set(e.value)}
            placeholder="Choose…"
            accessibilityLabel="Format"
          />
        </View>
      </View>
      <View class="flex-row justify-end gap-2 pt-1">
        <Button variant="ghost">
          <Text>Cancel</Text>
        </Button>
        <Button disabled={!ready()}>
          <Text>Schedule</Text>
        </Button>
      </View>
    </View>
  );
}

/** A country from a searchable list in groups, with the app's empty state. */
function CountryDemo() {
  const country = signal<string | null>("japan");
  return (
    <View class="w-64 gap-2">
      <Select
        class="w-full"
        items={countries}
        value={country()}
        onChange={(e) => country.set(e.value)}
        searchable
        searchPlaceholder="Search countries"
        empty={<Text>No country matches.</Text>}
        placeholder="Select a country"
        accessibilityLabel="Country"
      />
      <Text class="text-xs text-muted-foreground">{`country: ${country() ?? "none"}`}</Text>
    </View>
  );
}

/** A list's filters: each an optional choice the cross takes back to any. */
function FilterBar() {
  const owner = signal<string | null>("grace");
  const status = signal<string | null>(null);
  const priority = signal<string | null>("high");
  const shown = () =>
    [owner(), status(), priority()].filter((v) => v !== null).length === 0
      ? "Showing all issues"
      : `Filtered by ${[owner() && "owner", status() && "status", priority() && "priority"].filter(Boolean).join(", ")}`;
  return (
    <Demo label="A filter bar: optional choices, cleared with the cross" class="items-stretch">
      <View class="w-full gap-3">
        <View class="flex-row flex-wrap items-center gap-2">
          <Select
            class="w-44"
            size="sm"
            items={people}
            value={owner()}
            onChange={(e) => owner.set(e.value)}
            searchable
            searchPlaceholder="Search people"
            cleanable
            placeholder="Owner"
            accessibilityLabel="Owner"
          />
          <Select class="w-36" size="sm" items={issueStatuses} value={status()} onChange={(e) => status.set(e.value)} cleanable placeholder="Status" accessibilityLabel="Status" />
          <Select class="w-36" size="sm" items={priorities} value={priority()} onChange={(e) => priority.set(e.value)} cleanable placeholder="Priority" accessibilityLabel="Priority" />
        </View>
        <Text class="text-xs text-muted-foreground">{shown()}</Text>
      </View>
    </Demo>
  );
}

/** An issue's status: each row a badge and a note. */
function StatusDemo() {
  const status = signal<string | null>("progress");
  const items: SelectItem[] = statuses.map((s) => ({ value: s.value, title: s.title }));
  return (
    <Demo label={`status: ${status() ?? "none"}`}>
      <View class="w-80">
        <Select
          class="w-full"
          items={items}
          value={status()}
          onChange={(e) => status.set(e.value)}
          accessibilityLabel="Status"
          renderItem={(entry) => {
            const s = statuses.find((x) => x.value === entry.value)!;
            return (
              <View class="flex-row items-center gap-2">
                <Badge variant={s.variant}>
                  <Text>{s.title}</Text>
                </Badge>
                <Text class="min-w-0 flex-1 truncate text-xs text-muted-foreground">{s.note}</Text>
              </View>
            );
          }}
        />
      </View>
    </Demo>
  );
}
