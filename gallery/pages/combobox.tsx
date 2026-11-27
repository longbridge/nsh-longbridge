// The Combobox page: a project's settings, several values as chips, the
// matched part of each title, values made from the query, recent choices
// while nothing is typed, sizes, groups and a long list, states, the empty
// list.

import { signal, Text, View } from "nsh";
import { Button, Combobox, Input, Label, type SelectItem, type Size } from "nsh-longbridge";
import { Demo, Demos, Page, Row, Section } from "../ui/layout";
import { TokensSection } from "../ui/tokens";

const sizes: Size[] = ["xs", "sm", "md", "lg"];

const frameworks: SelectItem[] = [
  { value: "next", title: "Next.js" },
  { value: "sveltekit", title: "SvelteKit" },
  { value: "nuxt", title: "Nuxt.js" },
  { value: "remix", title: "Remix" },
  { value: "astro", title: "Astro" },
  { value: "solid", title: "SolidStart" },
];

const regions: SelectItem[] = [
  { value: "iad", title: "Washington, D.C. (iad1)", group: "Americas" },
  { value: "sfo", title: "San Francisco (sfo1)", group: "Americas" },
  { value: "gru", title: "São Paulo (gru1)", group: "Americas" },
  { value: "fra", title: "Frankfurt (fra1)", group: "Europe" },
  { value: "lhr", title: "London (lhr1)", group: "Europe" },
  { value: "cdg", title: "Paris (cdg1)", group: "Europe" },
  { value: "hnd", title: "Tokyo (hnd1)", group: "Asia Pacific" },
  { value: "sin", title: "Singapore (sin1)", group: "Asia Pacific" },
  { value: "syd", title: "Sydney (syd1)", group: "Asia Pacific" },
];

const labels: SelectItem[] = [
  { value: "bug", title: "Bug" },
  { value: "feature", title: "Feature" },
  { value: "docs", title: "Documentation" },
  { value: "perf", title: "Performance" },
  { value: "design", title: "Design" },
  { value: "security", title: "Security" },
];

// Airports, by city: several start alike ("San"), some have letters whose
// case folds change their length ("İ").
const airports: SelectItem[] = [
  ["sfo", "San Francisco (SFO)"], ["sjc", "San José (SJC)"], ["san", "San Diego (SAN)"], ["scl", "Santiago (SCL)"],
  ["gru", "São Paulo (GRU)"], ["sin", "Singapore (SIN)"], ["icn", "Seoul (ICN)"], ["syd", "Sydney (SYD)"],
  ["pvg", "Shanghai (PVG)"], ["hnd", "Tokyo (HND)"], ["ist", "İstanbul (IST)"], ["zrh", "Zürich (ZRH)"],
  ["lhr", "London (LHR)"], ["cdg", "Paris (CDG)"], ["fra", "Frankfurt (FRA)"], ["muc", "München (MUC)"],
  ["dxb", "Dubai (DXB)"], ["hkg", "Hong Kong (HKG)"], ["lax", "Los Angeles (LAX)"], ["jfk", "New York (JFK)"],
  ["kef", "Reykjavík (KEF)"], ["ams", "Amsterdam (AMS)"],
].map(([value, title]) => ({ value, title }));

const topics: SelectItem[] = [
  { value: "design", title: "Design" },
  { value: "engineering", title: "Engineering" },
  { value: "research", title: "Research" },
  { value: "marketing", title: "Marketing" },
  { value: "sales", title: "Sales" },
];

// Enough to scroll: the list is capped at combobox-list-max-height.
const countries: SelectItem[] = [
  "Argentina", "Australia", "Austria", "Belgium", "Brazil", "Canada", "Chile", "China", "Colombia", "Czechia",
  "Denmark", "Egypt", "Finland", "France", "Germany", "Greece", "Hungary", "India", "Indonesia", "Ireland", "Israel",
  "Italy", "Japan", "Kenya", "Malaysia", "Mexico", "Netherlands", "New Zealand", "Nigeria", "Norway", "Peru",
  "Philippines", "Poland", "Portugal", "Singapore", "South Africa", "South Korea", "Spain", "Sweden", "Switzerland",
  "Thailand", "Turkey", "Ukraine", "United Kingdom", "United States", "Vietnam",
].map((c) => ({ value: c.toLowerCase().replace(/ /g, "-"), title: c }));

export function ComboboxPage() {
  return (
    <Page
      eyebrow="Components"
      title="Combobox"
      description="A field that filters a list as you type: one value, or several as chips. The field is an Input's, the list a Select's; Arrow keys move through the rows and Enter chooses."
    >
      <Demo hero>
        <ProjectDemo />
      </Demo>

      <Section title="Several values" description="multiple shows the values as chips whose buttons remove them; choosing a row toggles it and the list stays open.">
        <Demo>
          <LabelsDemo />
        </Demo>
      </Section>

      <Section
        title="Matching"
        description="The part of each title the query matches is drawn heavier, so the reason a row is there shows at a glance. It matches as the filter does: ignoring case, the first place only."
      >
        <MatchingDemo />
      </Section>

      <Section
        title="Creating values"
        description="creatable adds a last row, Create “…”, while no item is called what was typed. Return on it reports onCreate with the query and the app adds the item. With several values the query empties for the next one; with one the list closes."
      >
        <Demos>
          <Demo label="Tags, several">
            <TagsDemo />
          </Demo>
          <Demo label="A folder, one">
            <FolderDemo />
          </Demo>
        </Demos>
      </Section>

      <Section
        title="Recent choices"
        description="Items are data, so while nothing is typed the app can lead with what was chosen last, under a group of its own. It watches the query with onQueryChange; once a query filters the list, every airport shows."
      >
        <Demo>
          <RecentDemo />
        </Demo>
      </Section>

      <Section title="Sizes" description="The field is an Input of the same size, so a combobox, a field and a button line up.">
        <Demo class="items-stretch">
          {sizes.map((size) => (
            <Row label={size}>
              <View class="w-52">
                <Combobox size={size} items={frameworks} placeholder="Framework" accessibilityLabel={`Framework, ${size}`} />
              </View>
              <View class="flex-1 min-w-32 max-w-48">
                <Input size={size} placeholder="Project name" accessibilityLabel={`Project name, ${size}`} />
              </View>
              <Button size={size}>
                <Text>Create</Text>
              </Button>
            </Row>
          ))}
        </Demo>
      </Section>

      <Section title="Groups and long lists" description="Items with a group are listed under its heading. A long list scrolls inside a capped height.">
        <Demos>
          <Demo label="Grouped">
            <View class="w-full max-w-[280px]">
              <Combobox items={regions} placeholder="Select a region" accessibilityLabel="Region" />
            </View>
          </Demo>
          <Demo label="46 countries">
            <View class="w-full max-w-[280px]">
              <Combobox items={countries} placeholder="Select a country" cleanable accessibilityLabel="Country" />
            </View>
          </Demo>
        </Demos>
      </Section>

      <Section title="States" description="Invalid turns the edge and the ring destructive; disabled dims it and does not open. With nothing matching, the list says so, in the app's words if it gives them.">
        <StatesDemo />
      </Section>

      <TokensSection prefix="combobox" component="Combobox" note="The field reads Input's tokens and the list the menu's; these are the combobox's own." />
    </Page>
  );
}

function ProjectDemo() {
  const framework = signal<string[]>(["astro"]);
  const region = signal<string[]>([]);
  return (
    <View class="w-full max-w-[380px] gap-5 rounded-xl border border-border bg-card p-6">
      <View class="gap-1">
        <Text class="text-base font-semibold text-card-foreground">New project</Text>
        <Text class="text-sm text-muted-foreground">Pick a framework and where it runs. Type to filter.</Text>
      </View>
      <View class="gap-2">
        <Label>
          <Text>Framework</Text>
        </Label>
        <Combobox
          items={frameworks}
          value={framework()}
          onChange={(e) => framework.set(e.value)}
          placeholder="Select a framework"
          cleanable
          accessibilityLabel="Framework"
        />
      </View>
      <View class="gap-2">
        <Label>
          <Text>Region</Text>
        </Label>
        <Combobox
          items={regions}
          value={region()}
          onChange={(e) => region.set(e.value)}
          placeholder="Select a region"
          accessibilityLabel="Region"
        />
      </View>
      <Button disabled={framework().length === 0 || region().length === 0}>
        <Text>Deploy</Text>
      </Button>
    </View>
  );
}

function LabelsDemo() {
  const picked = signal<string[]>(["bug", "perf"]);
  return (
    <View class="w-full max-w-[400px] gap-2">
      <Label>
        <Text>Labels</Text>
      </Label>
      <Combobox
        multiple
        items={labels}
        value={picked()}
        onChange={(e) => picked.set(e.value)}
        placeholder="Add labels"
        accessibilityLabel="Labels"
      />
      <Text class="text-xs text-muted-foreground">{`${picked().length} of ${labels.length} labels`}</Text>
    </View>
  );
}

function StatesDemo() {
  const owner = signal<string[]>([]);
  return (
    <Demo class="items-stretch">
      <View class="w-full flex-row flex-wrap gap-6">
        <View class="flex-1 min-w-44 gap-2">
          <Label>
            <Text>Owner</Text>
          </Label>
          <Combobox
            items={frameworks}
            value={owner()}
            onChange={(e) => owner.set(e.value)}
            invalid={owner().length === 0}
            placeholder="Required"
            accessibilityLabel="Owner"
          />
          <Text class={`text-xs ${owner().length === 0 ? "text-destructive" : "text-muted-foreground"}`}>
            {owner().length === 0 ? "Choose one to go on." : "Invalid until chosen"}
          </Text>
        </View>
        <View class="flex-1 min-w-44 gap-2">
          <Label disabled>
            <Text>Locked</Text>
          </Label>
          <Combobox items={frameworks} defaultValue={["next"]} disabled accessibilityLabel="Locked" />
          <Text class="text-xs text-muted-foreground">Disabled</Text>
        </View>
        <View class="flex-1 min-w-44 gap-2">
          <Label>
            <Text>Search</Text>
          </Label>
          <Combobox
            items={frameworks}
            defaultQuery="Ember"
            placeholder="Type a name"
            empty={<Text>No framework by that name.</Text>}
            accessibilityLabel="Search"
          />
          <Text class="text-xs text-muted-foreground">The app's empty words</Text>
        </View>
      </View>
    </Demo>
  );
}

/**
 * Destinations, the list held open on "san" while the scene is in the window
 * (as any list, it closes on Escape or a press outside, the query then
 * emptied by the app, which keeps it).
 */
function MatchingDemo() {
  const shown = signal(false);
  const open = signal(true);
  const query = signal("san");
  const picked = signal<string[]>(["sin"]);
  return (
    <Demo class="items-stretch justify-start" inView={shown.set}>
      <View class="w-full max-w-[340px] min-h-[300px] gap-2">
        <Label>
          <Text>Destinations</Text>
        </Label>
        <Combobox
          multiple
          items={airports}
          query={query()}
          onQueryChange={(e) => query.set(e.query)}
          open={open() && shown()}
          onOpenChange={(e) => {
            open.set(e.open);
            if (!e.open) query.set("");
          }}
          value={picked()}
          onChange={(e) => picked.set(e.value)}
          placeholder="Search airports"
          accessibilityLabel="Destinations"
        />
      </View>
    </Demo>
  );
}

let made = 0;

function TagsDemo() {
  const items = signal<SelectItem[]>(topics);
  const tags = signal<string[]>(["design"]);
  return (
    <View class="w-full max-w-[320px] gap-2">
      <Label>
        <Text>Topics</Text>
      </Label>
      <Combobox
        multiple
        creatable
        items={items()}
        value={tags()}
        onChange={(e) => tags.set(e.value)}
        onCreate={(e) => {
          const value = `topic-${++made}`;
          items.set([...items(), { value, title: e.query }]);
          tags.set([...tags(), value]);
        }}
        placeholder="Add a topic"
        accessibilityLabel="Topics"
      />
      <Text class="text-xs text-muted-foreground">Type a topic that is not listed, then Enter.</Text>
    </View>
  );
}

function FolderDemo() {
  const folders = signal<SelectItem[]>([
    { value: "inbox", title: "Inbox" },
    { value: "receipts", title: "Receipts" },
    { value: "travel", title: "Travel" },
  ]);
  const folder = signal<string[]>(["inbox"]);
  return (
    <View class="w-full max-w-[280px] gap-2">
      <Label>
        <Text>Move to</Text>
      </Label>
      <Combobox
        creatable
        items={folders()}
        value={folder()}
        onChange={(e) => folder.set(e.value)}
        onCreate={(e) => {
          const value = `folder-${++made}`;
          folders.set([...folders(), { value, title: e.query }]);
          folder.set([value]);
        }}
        placeholder="Choose or name a folder"
        accessibilityLabel="Folder"
      />
      <Text class="text-xs text-muted-foreground">A new name makes a folder and moves it there.</Text>
    </View>
  );
}

/** The airports with the recent ones first, under "Recent", and the rest under "All airports": one value, one row. */
function withRecent(recent: string[]): SelectItem[] {
  const first = recent.flatMap((v) => airports.filter((a) => a.value === v)).map((a) => ({ ...a, group: "Recent" }));
  const rest = airports.filter((a) => !recent.includes(a.value)).map((a) => ({ ...a, group: "All airports" }));
  return [...first, ...rest];
}

function RecentDemo() {
  const typed = signal("");
  const recent = signal<string[]>(["sin", "hnd", "lhr"]);
  const airport = signal<string[]>([]);
  return (
    <View class="w-full max-w-[320px] gap-2">
      <Label>
        <Text>Flying from</Text>
      </Label>
      <Combobox
        items={typed().trim() === "" ? withRecent(recent()) : airports}
        onQueryChange={(e) => typed.set(e.query)}
        value={airport()}
        onChange={(e) => {
          airport.set(e.value);
          recent.set([...e.value, ...recent().filter((v) => !e.value.includes(v))].slice(0, 3));
        }}
        placeholder="Search airports"
        cleanable
        accessibilityLabel="Flying from"
      />
      <Text class="text-xs text-muted-foreground">Open it with nothing typed: the last three come first.</Text>
    </View>
  );
}
