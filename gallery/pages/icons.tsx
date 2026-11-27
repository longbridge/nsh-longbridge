// The Icons page: the library's own icons at each size, where its
// components use them, and how an app adds its own.

import { Text, View } from "nsh";
import { Badge, Button, Icon, type IconName, type Size } from "nsh-longbridge";
import { Demo, Demos, Page, Section } from "../ui/layout";
import { columnWidth, contentWidth } from "../ui/screen";

const icons: { name: IconName; for: string }[] = [
  { name: "check", for: "Checkbox, Badge" },
  { name: "minus", for: "Checkbox, indeterminate" },
  { name: "plus", for: "Adding" },
  { name: "arrow-right", for: "Going on" },
  { name: "chevron-down", for: "Opening a menu" },
  { name: "trash-2", for: "Deleting" },
  { name: "loader-circle", for: "Spinner, loading" },
];

const sizes: Size[] = ["xs", "sm", "md", "lg"];

const tileWidth = () => {
  const w = contentWidth();
  return columnWidth(w, w >= 760 ? 4 : w >= 500 ? 3 : 2, 16);
};

export function IconsPage() {
  return (
    <Page
      eyebrow="Foundation"
      title="Icons"
      description="The few icons the library's components draw, from Lucide, sized by the control they are in and colored as its text. An app brings its own set through nsh-icon, as this gallery does."
    >
      <Section title="The set">
        <View class="flex-row flex-wrap gap-4">
          {icons.map((i) => (
            <View class="items-center gap-4 rounded-xl border border-border px-3 py-6" style={{ width: tileWidth() }}>
              <View class="size-10 items-center justify-center rounded-lg bg-muted">
                <Icon name={i.name} size="lg" class="text-foreground" spin={i.name === "loader-circle"} />
              </View>
              <View class="items-center gap-1">
                <Text class="font-mono text-xs text-foreground">{i.name}</Text>
                <Text class="text-center text-xs text-muted-foreground">{i.for}</Text>
              </View>
            </View>
          ))}
        </View>
      </Section>

      <Section title="Sizes" description="icon-size-xs to lg: 12, 14, 16 and 16 points with the default font-size seed.">
        <View class="rounded-xl border border-border">
          {sizes.map((size, i) => (
            <View class={`h-14 flex-row items-center gap-6 px-5 ${i > 0 ? "border-t border-border" : ""}`}>
              <Text class="w-8 font-mono text-xs text-muted-foreground">{size}</Text>
              <View class="flex-1 flex-row items-center gap-6">
                {icons.map((ic) => (
                  <Icon name={ic.name} size={size} class="text-foreground" />
                ))}
              </View>
            </View>
          ))}
        </View>
      </Section>

      <Section title="In components" description="Left without a size, an icon takes its control's; its color is always its text's.">
        <Demos>
          <Demo row label="In buttons of each size">
            {sizes.map((size) => (
              <Button size={size} variant="outline" icon="plus">
                <Text>Add</Text>
              </Button>
            ))}
          </Demo>
          <Demo row label="In badges, colored as their text">
            <Badge>
              <Icon name="check" />
              <Text>Done</Text>
            </Badge>
            <Badge variant="destructive">
              <Icon name="trash-2" />
              <Text>Removed</Text>
            </Badge>
            <Badge variant="outline">
              <Text>Next</Text>
              <Icon name="arrow-right" />
            </Badge>
          </Demo>
        </Demos>
      </Section>
    </Page>
  );
}
