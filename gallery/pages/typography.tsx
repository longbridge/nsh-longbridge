// The Typography page: the type scale with its line heights, the controls'
// sizes by `size`, the weights and the two families.

import { Text, View } from "nsh";
import { Badge, Button, Input, type Size } from "nsh-longbridge";
import { formatValue, numberOf } from "../theme";
import { Page, Section } from "../ui/layout";
import { compact } from "../ui/screen";

/** Tailwind's scale, as NSH's classes and `foundation`'s line heights pair them. */
const scale = [
  { name: "text-xs", size: 12, line: 16, sample: "Captions, badges, key caps" },
  { name: "text-sm", size: 14, line: 20, sample: "Body text in controls and tables" },
  { name: "text-base", size: 16, line: 24, sample: "Reading text and descriptions" },
  { name: "text-lg", size: 18, line: 28, sample: "A lead paragraph" },
  { name: "text-xl", size: 20, line: 28, sample: "A section heading" },
  { name: "text-2xl", size: 24, line: 32, sample: "A figure in a card" },
  { name: "text-3xl", size: 30, line: 36, sample: "A page title" },
  { name: "text-4xl", size: 36, line: 40, sample: "A hero" },
];

const sizes: Size[] = ["xs", "sm", "md", "lg"];

const weights = [
  { name: "Regular", class: "font-normal", value: "400" },
  { name: "Medium", class: "font-medium", value: "font-weight-medium" },
  { name: "Semibold", class: "font-semibold", value: "font-weight-semibold" },
];

export function TypographyPage() {
  return (
    <Page
      eyebrow="Foundation"
      title="Typography"
      description="One scale for the whole library: Tailwind's sizes with their line heights for text, and four sizes for controls that follow the font-size seed."
    >
      <Section title="The scale" description="Text classes keep Tailwind's names and values, so an app's own text agrees with the components around it.">
        <View class="rounded-xl border border-border">
          {scale.map((s, i) => (
            <View class={`flex-row items-center gap-6 px-5 py-4 ${i > 0 ? "border-t border-border" : ""}`}>
              <View class="w-28 gap-0.5">
                <Text class="font-mono text-xs text-foreground">{s.name}</Text>
                <Text class="font-mono text-xs tabular-nums text-muted-foreground">{`${s.size} / ${s.line}`}</Text>
              </View>
              <Text class="flex-1 truncate text-foreground" style={{ fontSize: s.size, lineHeight: s.line }}>
                {s.sample}
              </Text>
            </View>
          ))}
        </View>
      </Section>

      <Section
        title="In controls"
        description="A control's size sets its text: font-size − 2, − 1, ± 0, ± 0. Move the font-size seed on the Customize page and every row follows."
      >
        <View class="rounded-xl border border-border">
          {sizes.map((size, i) => (
            <View class={`flex-row items-center gap-6 px-5 py-4 ${i > 0 ? "border-t border-border" : ""}`}>
              <View class="w-28 gap-0.5">
                <Text class="font-mono text-xs text-foreground">{`font-size-${size}`}</Text>
                <Text class="font-mono text-xs tabular-nums text-muted-foreground">{formatValue(`font-size-${size}`)}</Text>
              </View>
              <View class="flex-1 flex-row flex-wrap items-center gap-3">
                <Button size={size} variant="outline">
                  <Text>Button</Text>
                </Button>
                <If when={!compact()}>
                  <View class="w-44">
                    <Input size={size} placeholder="Input" accessibilityLabel={`Input, ${size}`} />
                  </View>
                </If>
              </View>
            </View>
          ))}
        </View>
      </Section>

      <Section title="Weights" description="Three weights: regular for reading, medium for labels and controls, semibold for titles.">
        <View class="flex-row flex-wrap gap-4">
          {weights.map((w) => (
            <View class="flex-1 min-w-48 gap-4 rounded-xl border border-border p-5">
              <Text class={`text-4xl text-foreground ${w.class}`}>Ag</Text>
              <View class="gap-0.5">
                <Text class="text-sm font-medium text-foreground">{w.name}</Text>
                <Text class="font-mono text-xs text-muted-foreground">
                  {w.value === "400" ? "400" : `${w.value} · ${numberOf(w.value)}`}
                </Text>
              </View>
            </View>
          ))}
        </View>
      </Section>

      <Section title="Families" description="The system's own faces by default: font-sans and font-mono in the theme change them.">
        <View class="flex-row flex-wrap gap-4">
          <View class="flex-1 min-w-64 gap-3 rounded-xl border border-border p-5">
            <Badge variant="secondary">
              <Text>font-sans</Text>
            </Badge>
            <Text class="text-2xl text-foreground">The quick brown fox jumps over the lazy dog</Text>
            <Text class="text-sm text-muted-foreground">显示名称 · 表示名 · 표시 이름</Text>
          </View>
          <View class="flex-1 min-w-64 gap-3 rounded-xl border border-border p-5">
            <Badge variant="secondary">
              <Text>font-mono</Text>
            </Badge>
            <Text class="font-mono text-xl text-foreground">{"0123456789 { } [ ] ( ) => !="}</Text>
            <Text class="font-mono text-sm text-muted-foreground">INV-001 · $15,231.89 · v2.4.0</Text>
          </View>
        </View>
      </Section>
    </Page>
  );
}
