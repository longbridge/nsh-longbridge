// The Icons page: the library's own icons at each size, where its
// components use them, and how an app adds its own.

import { Text, View } from "nsh";
import { Badge, Button, Icon, type IconName, type Size } from "nsh-longbridge";
import { Demo, Demos, Page, Section } from "../ui/layout";
import { columnWidth, contentWidth } from "../ui/screen";

const icons: { name: IconName; for: string }[] = [
  { name: "check", for: "Checkbox, checked items" },
  { name: "minus", for: "Checkbox, indeterminate" },
  { name: "chevron-down", for: "Select, opening a menu" },
  { name: "chevron-right", for: "Submenus, a next page" },
  { name: "chevron-left", for: "A sheet's back row" },
  { name: "x", for: "Closing a dialog or toast" },
  { name: "circle-check", for: "Toast, success" },
  { name: "info", for: "Toast, information" },
  { name: "triangle-alert", for: "Toast, warning" },
  { name: "octagon-x", for: "Toast, error" },
  { name: "loader-circle", for: "Spinner, loading" },
  { name: "ellipsis", for: "An overflow menu" },
  { name: "calendar", for: "DatePicker's field" },
  { name: "eye", for: "Showing a password" },
  { name: "eye-off", for: "Hiding a password again" },
  { name: "search", for: "A search field" },
  { name: "copy", for: "Copying, duplicating" },
  { name: "plus", for: "Adding" },
  { name: "arrow-right", for: "Going on" },
  { name: "trash-2", for: "Deleting" },
  { name: "settings", for: "Settings" },
  { name: "log-out", for: "Signing out" },
  { name: "layout-dashboard", for: "A dashboard" },
  { name: "activity", for: "Activity" },
  { name: "tag", for: "Tagging a selection" },
  { name: "user", for: "Assigning a selection" },
  { name: "archive", for: "Archiving" },
  { name: "download", for: "Downloading, exporting" },
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
      description="The few icons the library ships, from Lucide, sized by the control they are in and colored as its text (foreground outside the library's parts). An app brings its own set through nsh-icon, as this gallery does."
    >
      <Section title="The set" description="Each icon the library ships and what it is for. Any of them goes in an Icon, or in a Button's icon or iconEnd.">
        <View class="flex-row flex-wrap gap-4">
          {icons.map((i) => (
            <View class="items-center gap-4 rounded-xl border border-border px-3 py-6" style={{ width: tileWidth() }}>
              <View class="size-10 items-center justify-center rounded-lg bg-muted">
                <Icon name={i.name} size="lg" spin={i.name === "loader-circle"} />
              </View>
              <View class="items-center gap-1">
                <Text class="font-mono text-xs text-foreground">{i.name}</Text>
                <Text class="text-center text-xs text-muted-foreground">{i.for}</Text>
              </View>
            </View>
          ))}
        </View>
      </Section>

      <Section title="Sizes" description="The icon-size-xs to lg tokens: 12, 14, 16 and 16 points with the default font-size seed.">
        <View class="rounded-xl border border-border">
          {sizes.map((size, i) => (
            <View class={`min-h-14 flex-row items-center gap-6 px-5 ${i > 0 ? "border-t border-border" : ""}`}>
              <Text class="w-8 font-mono text-xs text-muted-foreground">{size}</Text>
              <View class="flex-1 flex-row flex-wrap items-center gap-x-4 gap-y-3 py-3">
                {icons.map((ic) => (
                  <Icon name={ic.name} size={size} />
                ))}
              </View>
            </View>
          ))}
        </View>
      </Section>

      <Section title="In components" description="Left without a size, an icon takes its control's. Its color is the text color of the part it sits in, foreground outside one, or a class color.">
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
