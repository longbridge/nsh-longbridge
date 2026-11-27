// The Combobox page: a project's settings, several values as chips, sizes,
// groups and a long list, states, the empty list.

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
