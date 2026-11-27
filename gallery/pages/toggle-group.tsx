// The ToggleGroup page: a document toolbar, variants and spacing, sizes,
// words, several values, disabled.

import { signal, Text, View } from "nsh";
import { type Size, ToggleGroup, ToggleGroupItem } from "nsh-longbridge";
import {
  AlignCenter,
  AlignLeft,
  AlignRight,
  Bold,
  Calendar,
  Glyph,
  Italic,
  LayoutGrid,
  LayoutList,
  Underline,
} from "../ui/icons";
import { Demo, Demos, Page, Row, Section } from "../ui/layout";
import { TokensSection } from "../ui/tokens";

const sizes: Size[] = ["xs", "sm", "md", "lg"];
const iconSizes: Record<Size, number> = { xs: 12, sm: 14, md: 16, lg: 18 };

const aligns = [
  { value: "left", label: "Align left", icon: AlignLeft },
  { value: "center", label: "Align center", icon: AlignCenter },
  { value: "right", label: "Align right", icon: AlignRight },
];

const marks = [
  { value: "bold", label: "Bold", icon: Bold },
  { value: "italic", label: "Italic", icon: Italic },
  { value: "underline", label: "Underline", icon: Underline },
];

export function ToggleGroupPage() {
  return (
    <Page
      eyebrow="Components"
      title="Toggle Group"
      description="A row of toggles sharing one value, or several: an alignment, a view, the formats of a selection. Joined into one shape, or apart."
    >
      <Demo hero>
        <ToolbarDemo />
      </Demo>

      <Section
        title="Variants and spacing"
        description="Toggle's two variants. With spacing 0 the items join: inner corners square, one edge and shadow around the row. A spacing apart gives each item its own shape."
      >
        <Demos>
          <Demo label="default, joined">
            <AlignGroup />
          </Demo>
          <Demo label="outline, joined">
            <AlignGroup variant="outline" />
          </Demo>
          <Demo label="outline, spacing 2">
            <AlignGroup variant="outline" spacing={2} />
          </Demo>
        </Demos>
      </Section>

      <Section title="Sizes" description="Each item is a Toggle of the group's size.">
        <Demo class="items-stretch">
          {sizes.map((size) => (
            <Row label={size}>
              <AlignGroup variant="outline" size={size} />
            </Row>
          ))}
        </Demo>
      </Section>

      <Section title="Words" description="One value from a few named options. A press on the chosen item asks for none; the app keeps one by ignoring an empty value.">
        <ViewsDemo />
      </Section>

      <Section title="Several values" description="multiple lets each item be on or off on its own; the value is the list of those on.">
        <Demos>
          <Demo>
            <MarksDemo />
          </Demo>
          <Demo label="Disabled: every item, and an enclosing Field's or FieldSet's disabled reaches it too">
            <ToggleGroup variant="outline" disabled value={["bold"]} accessibilityLabel="Text style, disabled">
              {marks.map((m) => (
                <ToggleGroupItem value={m.value} accessibilityLabel={m.label}>
                  <Glyph icon={m.icon} />
                </ToggleGroupItem>
              ))}
            </ToggleGroup>
          </Demo>
        </Demos>
      </Section>

      <TokensSection
        prefix="toggle-group"
        component="Toggle Group"
        note="Its items read Toggle's tokens; these are the group's own."
      />
    </Page>
  );
}

function AlignGroup(props: { variant?: "default" | "outline"; size?: Size; spacing?: number }) {
  const align = signal("left");
  return (
    <ToggleGroup
      variant={props.variant}
      size={props.size}
      spacing={props.spacing}
      value={[align()]}
      onChange={(e) => e.value[0] && align.set(e.value[0])}
      accessibilityLabel={`Text alignment${props.size ? `, ${props.size}` : ""}`}
    >
      {aligns.map((a) => (
        <ToggleGroupItem value={a.value} accessibilityLabel={a.label}>
          <Glyph icon={a.icon} size={iconSizes[props.size ?? "md"]} />
        </ToggleGroupItem>
      ))}
    </ToggleGroup>
  );
}

function ToolbarDemo() {
  const align = signal("left");
  const style = signal<string[]>(["bold"]);
  const textClass = () =>
    [
      style().includes("bold") ? "font-semibold" : "",
      style().includes("italic") ? "italic" : "",
      style().includes("underline") ? "underline" : "",
      align() === "center" ? "text-center" : align() === "right" ? "text-right" : "text-left",
    ].join(" ");
  return (
    <View class="w-full max-w-[460px] rounded-xl border border-border bg-card">
      <View class="flex-row items-center gap-2 border-b border-border p-2">
        <ToggleGroup
          multiple
          size="sm"
          value={style()}
          onChange={(e) => style.set(e.value)}
          accessibilityLabel="Text style"
        >
          {marks.map((m) => (
            <ToggleGroupItem value={m.value} accessibilityLabel={m.label}>
              <Glyph icon={m.icon} size={14} />
            </ToggleGroupItem>
          ))}
        </ToggleGroup>
        <View class="h-5 w-px bg-border" />
        <ToggleGroup
          size="sm"
          value={[align()]}
          onChange={(e) => e.value[0] && align.set(e.value[0])}
          accessibilityLabel="Text alignment"
        >
          {aligns.map((a) => (
            <ToggleGroupItem value={a.value} accessibilityLabel={a.label}>
              <Glyph icon={a.icon} size={14} />
            </ToggleGroupItem>
          ))}
        </ToggleGroup>
      </View>
      <View class="gap-2 p-5">
        <Text class="text-xs font-medium text-muted-foreground">Announcement</Text>
        <Text class={`text-base leading-7 text-card-foreground ${textClass()}`}>
          The office is closed on Friday. Shared drives stay online, and support answers within the hour.
        </Text>
      </View>
    </View>
  );
}

function ViewsDemo() {
  const view = signal("list");
  return (
    <Demos>
      <Demo>
        <ToggleGroup
          variant="outline"
          value={[view()]}
          onChange={(e) => e.value[0] && view.set(e.value[0])}
          accessibilityLabel="View"
        >
          <ToggleGroupItem value="list">
            <Glyph icon={LayoutList} />
            <Text>List</Text>
          </ToggleGroupItem>
          <ToggleGroupItem value="board">
            <Glyph icon={LayoutGrid} />
            <Text>Board</Text>
          </ToggleGroupItem>
          <ToggleGroupItem value="calendar">
            <Glyph icon={Calendar} />
            <Text>Calendar</Text>
          </ToggleGroupItem>
        </ToggleGroup>
        <Text class="text-sm text-muted-foreground">{`Showing the ${view()} view.`}</Text>
      </Demo>
      <Demo>
        <DensityDemo />
      </Demo>
    </Demos>
  );
}

function DensityDemo() {
  const density = signal("default");
  return (
    <ToggleGroup
      size="sm"
      variant="outline"
      value={[density()]}
      onChange={(e) => e.value[0] && density.set(e.value[0])}
      accessibilityLabel="Density"
    >
      <ToggleGroupItem value="compact">
        <Text>Compact</Text>
      </ToggleGroupItem>
      <ToggleGroupItem value="default">
        <Text>Default</Text>
      </ToggleGroupItem>
      <ToggleGroupItem value="comfortable">
        <Text>Comfortable</Text>
      </ToggleGroupItem>
    </ToggleGroup>
  );
}

function MarksDemo() {
  const on = signal<string[]>(["bold", "underline"]);
  return (
    <View class="items-center gap-3">
      <ToggleGroup multiple spacing={1} value={on()} onChange={(e) => on.set(e.value)} accessibilityLabel="Text style">
        {marks.map((m) => (
          <ToggleGroupItem value={m.value} accessibilityLabel={m.label}>
            <Glyph icon={m.icon} />
          </ToggleGroupItem>
        ))}
      </ToggleGroup>
      <Text class="text-sm text-muted-foreground">{on().length ? on().join(", ") : "None"}</Text>
    </View>
  );
}
