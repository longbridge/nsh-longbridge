import { mount, Scroll, signal, Text, View } from "nsh";
import { Window } from "nsh/desktop";
import { Button } from "nsh-longbridge";
import { BadgePage } from "./pages/badge";
import { ButtonPage } from "./pages/button";
import { CardPage } from "./pages/card";
import { CheckboxPage } from "./pages/checkbox";
import { InputPage } from "./pages/input";
import { KbdPage } from "./pages/kbd";
import { LabelPage } from "./pages/label";
import { RadioGroupPage } from "./pages/radio-group";
import { SeparatorPage } from "./pages/separator";
import { SkeletonPage } from "./pages/skeleton";
import { SpinnerPage } from "./pages/spinner";
import { SwitchPage } from "./pages/switch";
import { TextareaPage } from "./pages/textarea";
import { ThemePage } from "./pages/theme";
import { TooltipPage } from "./pages/tooltip";
import { appearance, Appearance, colorPreset, colorPresets, densityPreset, densityPresets, followTheme } from "./theme";
import { Segmented } from "./ui";

type PageId = "badge" | "button" | "card" | "checkbox" | "input" | "kbd" | "label" | "radio-group" | "separator" | "skeleton" | "spinner" | "switch" | "textarea" | "tooltip" | "theme";

const catalog: { group: string; pages: { id: PageId; label: string; page: () => JSX.Element }[] }[] = [
  {
    group: "Components",
    pages: [
      { id: "badge", label: "Badge", page: BadgePage },
      { id: "button", label: "Button", page: ButtonPage },
      { id: "card", label: "Card", page: CardPage },
      { id: "checkbox", label: "Checkbox", page: CheckboxPage },
      { id: "input", label: "Input", page: InputPage },
      { id: "kbd", label: "Kbd", page: KbdPage },
      { id: "label", label: "Label", page: LabelPage },
      { id: "radio-group", label: "RadioGroup", page: RadioGroupPage },
      { id: "separator", label: "Separator", page: SeparatorPage },
      { id: "skeleton", label: "Skeleton", page: SkeletonPage },
      { id: "spinner", label: "Spinner", page: SpinnerPage },
      { id: "switch", label: "Switch", page: SwitchPage },
      { id: "textarea", label: "Textarea", page: TextareaPage },
      { id: "tooltip", label: "Tooltip", page: TooltipPage },
    ],
  },
  { group: "Foundation", pages: [{ id: "theme", label: "Theme", page: ThemePage }] },
];

const pageOf = (id: PageId) => catalog.flatMap((g) => g.pages).find((p) => p.id === id)!.page;

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
          {(() => {
            const Current = pageOf(page());
            return <Current />;
          })()}
        </Scroll>
      </View>
    </View>
  );
}

mount(() => <Window id="main">{App}</Window>);
