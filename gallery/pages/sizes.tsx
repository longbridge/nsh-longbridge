// The Radius and sizes page: the radii, the control heights per size with a
// pointer and on touch screens, icon sizes, spacing, lines and focus.

import { Text, View } from "nsh";
import { Button, Checkbox, Input, type Size, Switch } from "nsh-longbridge";
import { formatValue, numberOf } from "../theme";
import { Page, Section } from "../ui/layout";
import { Glyph, Search } from "../ui/icons";

const radii = [
  { name: "radius-xs", derived: "radius − 8, at least 0", for: "A dialog's close button" },
  { name: "radius-sm", derived: "radius − 4", for: "Checkboxes, keys" },
  { name: "radius-md", derived: "radius − 2", for: "Buttons, fields, popovers, menus" },
  { name: "radius-lg", derived: "radius", for: "Dialogs, tabs, toasts" },
  { name: "radius-xl", derived: "radius + 4", for: "Cards" },
];

const sizes: Size[] = ["xs", "sm", "md", "lg"];

const spacing = [1, 2, 3, 4, 6, 8, 12, 16, 24];

const lines = [
  { name: "border-width", note: "Every edge" },
  { name: "ring-width", note: "The focus ring, drawn outside" },
  { name: "ring-opacity", note: "The ring's strength" },
  { name: "opacity-disabled", note: "Disabled controls" },
  { name: "touch-target", note: "The least target on touch screens" },
];

export function SizesPage() {
  return (
    <Page
      eyebrow="Foundation"
      title="Radius and sizes"
      description="The radius, control-height, control-height-touch, spacing and font-size seeds, and what derives from them. The values are this theme's; change a seed on the Customize page and they follow."
    >
      <Section title="Radii" description="Five radii from one seed, shadcn v4's and Tailwind's rounded-xs. Badges and switches are pills whatever the seed. Nested surfaces step down a radius, so their corners stay concentric.">
        <View class="flex-row flex-wrap gap-4">
          {radii.map((r) => (
            <View class="flex-1 min-w-40 gap-3">
              <View
                class="h-24 border border-border bg-muted"
                style={{ borderRadius: numberOf(r.name) }}
              />
              <View class="gap-0.5">
                <View class="flex-row items-baseline justify-between">
                  <Text class="font-mono text-xs text-foreground">{r.name}</Text>
                  <Text class="font-mono text-xs tabular-nums text-muted-foreground">{formatValue(r.name)}</Text>
                </View>
                <Text class="text-xs text-muted-foreground">{`${r.derived} · ${r.for}`}</Text>
              </View>
            </View>
          ))}
        </View>
      </Section>

      <Section
        title="Control heights"
        description="A control's size sets its height, with a pointer and on touch screens. Controls of one size line up in a row, whatever they are."
      >
        <View class="rounded-xl border border-border">
          <View class="h-10 flex-row items-center gap-4 border-b border-border px-5">
            <Text class="w-10 text-xs font-medium text-muted-foreground">Size</Text>
            <Text class="w-20 text-xs font-medium text-muted-foreground">Pointer</Text>
            <Text class="w-20 text-xs font-medium text-muted-foreground">Touch</Text>
            <Text class="flex-1 text-xs font-medium text-muted-foreground">With a pointer</Text>
          </View>
          {sizes.map((size, i) => (
            <View class={`min-h-16 flex-row items-center gap-4 px-5 py-3 ${i > 0 ? "border-t border-border" : ""}`}>
              <Text class="w-10 font-mono text-xs text-foreground">{size}</Text>
              <Text class="w-20 font-mono text-xs tabular-nums text-foreground">{formatValue(`control-height-${size}`)}</Text>
              <Text class="w-20 font-mono text-xs tabular-nums text-muted-foreground">{formatValue(`control-height-touch-${size}`)}</Text>
              <View class="flex-1 flex-row flex-wrap items-center gap-3">
                <Button size={size}>
                  <Text>Save</Text>
                </Button>
                <View class="w-32">
                  <Input size={size} placeholder="Search" accessibilityLabel={`Search, ${size}`} />
                </View>
                <Checkbox size={size} checked accessibilityLabel={`Checkbox, ${size}`} />
                <Switch size={size} checked accessibilityLabel={`Switch, ${size}`} />
              </View>
            </View>
          ))}
        </View>
      </Section>

      <Section title="Icons in controls" description="Paired with the control's text: font-size − 2, ± 0, + 2, + 2.">
        <View class="flex-row flex-wrap gap-4">
          {sizes.map((size) => (
            <View class="flex-1 min-w-32 items-center gap-3 rounded-xl border border-border py-6">
              <View class="h-6 items-center justify-center">
                <Glyph icon={Search} size={numberOf(`icon-size-${size}`)} class="text-foreground" />
              </View>
              <View class="items-center gap-0.5">
                <Text class="font-mono text-xs text-foreground">{`icon-size-${size}`}</Text>
                <Text class="font-mono text-xs tabular-nums text-muted-foreground">{formatValue(`icon-size-${size}`)}</Text>
              </View>
            </View>
          ))}
        </View>
      </Section>

      <Section title="Spacing" description={`Paddings and gaps are steps of the spacing seed (${formatValue("spacing")}), as Tailwind 4's.`}>
        <View class="gap-2.5 rounded-xl border border-border p-5">
          {spacing.map((n) => (
            <View class="flex-row items-center gap-4">
              <Text class="w-10 font-mono text-xs text-muted-foreground">{String(n)}</Text>
              <View class="h-2.5 rounded-[2px] bg-primary" style={{ width: n * numberOf("spacing") }} />
              <Text class="font-mono text-xs tabular-nums text-muted-foreground">{String(n * numberOf("spacing"))}</Text>
            </View>
          ))}
        </View>
      </Section>

      <Section title="Lines, focus and states">
        <View class="rounded-xl border border-border">
          {lines.map((l, i) => (
            <View class={`h-12 flex-row items-center gap-4 px-5 ${i > 0 ? "border-t border-border" : ""}`}>
              <Text class="w-40 font-mono text-xs text-foreground">{l.name}</Text>
              <Text class="flex-1 text-sm text-muted-foreground">{l.note}</Text>
              <Text class="font-mono text-xs tabular-nums text-foreground">{formatValue(l.name)}</Text>
            </View>
          ))}
        </View>
      </Section>
    </Page>
  );
}
