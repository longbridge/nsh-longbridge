import { mount, Scroll, signal, Text, View } from "nsh";
import { Window } from "nsh/desktop";
import { Button } from "nsh-longbridge";
import { ButtonPage } from "./pages/button";
import { InputPage } from "./pages/input";
import { LabelPage } from "./pages/label";
import { TextareaPage } from "./pages/textarea";
import { ThemePage } from "./pages/theme";
import { appearance, Appearance, colorPreset, colorPresets, densityPreset, densityPresets, followTheme } from "./theme";
import { Segmented } from "./ui";

type PageId = "button" | "input" | "label" | "textarea" | "theme";

const catalog: { group: string; pages: { id: PageId; label: string }[] }[] = [
  {
    group: "Components",
    pages: [
      { id: "button", label: "Button" },
      { id: "input", label: "Input" },
      { id: "label", label: "Label" },
      { id: "textarea", label: "Textarea" },
    ],
  },
  { group: "Foundation", pages: [{ id: "theme", label: "Theme" }] },
];

function TopBar() {
  return (
    <View class="h-14 flex-row items-center justify-between border-b border-border px-5">
      <View class="flex-row items-baseline gap-2">
        <Text class="text-base font-semibold text-foreground">nsh-longbridge</Text>
        <Text class="text-sm text-muted-foreground">Gallery</Text>
      </View>
      <View class="flex-row items-center gap-5">
        <Segmented label="Color" options={colorPresets} value={colorPreset} onChange={(id) => colorPreset.set(id)} />
        <Segmented
          label="Density"
          options={densityPresets}
          value={densityPreset}
          onChange={(id) => densityPreset.set(id)}
        />
        <Segmented
          label="Appearance"
          options={[
            { id: "light", label: "Light" },
            { id: "dark", label: "Dark" },
            { id: "system", label: "System" },
          ]}
          value={appearance}
          onChange={(id) => appearance.set(id as Appearance)}
        />
      </View>
    </View>
  );
}

function Sidebar(props: { page: () => PageId; onSelect: (id: PageId) => void }) {
  return (
    <View class="w-56 border-r border-border px-3 py-5 gap-5">
      {catalog.map((group) => (
        <View class="gap-1">
          <Text class="px-2 pb-1 text-xs font-medium text-muted-foreground">{group.group}</Text>
          {group.pages.map((p) => (
            <Button
              size="sm"
              variant={props.page() === p.id ? "secondary" : "ghost"}
              class="w-full justify-start"
              onPress={() => props.onSelect(p.id)}
            >
              <Text>{p.label}</Text>
            </Button>
          ))}
        </View>
      ))}
    </View>
  );
}

function App() {
  followTheme();
  const page = signal<PageId>("button");
  return (
    <View class="flex-1 bg-background">
      <TopBar />
      <View class="flex-1 flex-row">
        <Sidebar page={page} onSelect={(id) => page.set(id)} />
        <Scroll class="flex-1" contentClass="px-12 py-10 max-w-[1040px]">
          <If when={page() === "button"}>
            <ButtonPage />
          </If>
          <If when={page() === "input"}>
            <InputPage />
          </If>
          <If when={page() === "label"}>
            <LabelPage />
          </If>
          <If when={page() === "textarea"}>
            <TextareaPage />
          </If>
          <If when={page() === "theme"}>
            <ThemePage />
          </If>
        </Scroll>
      </View>
    </View>
  );
}

mount(() => <Window id="main">{App}</Window>);
