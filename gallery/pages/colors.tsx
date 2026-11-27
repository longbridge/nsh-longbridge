// The Colors page: every semantic color, in light and in dark side by side,
// as the theme shown resolves it.

import { Text, View } from "nsh";
import { colorGroups, colorOf, cssColor, formatValue, type Scheme } from "../theme";
import { Page, Section } from "../ui/layout";
import { columnWidth, contentWidth } from "../ui/screen";

/** Two or three tiles a row, by the page's width. */
const tileWidth = () => {
  const w = contentWidth();
  return columnWidth(w, w >= 760 ? 3 : w >= 500 ? 2 : 1, 16);
};

const notes: Record<string, string> = {
  Base: "The page and its text.",
  Surfaces: "Cards, and popups over the page.",
  Roles: "Fills with the text that goes on them: the main action, quieter ones, hover and selection, danger.",
  Status: "This library's additions for outcomes: done, careful, for your information.",
  "Lines and focus": "Edges, the edges of fields, the focus ring, and the ring of a focused invalid field.",
  Charts: "Series in charts, in order.",
  Sidebar: "An app's sidebar, which may differ from the page.",
  Overlays: "Behind a modal, and the color of drop shadows.",
};

export function ColorsPage() {
  return (
    <Page
      eyebrow="Foundation"
      title="Colors"
      description="The semantic colors and this library's additions, as the preset and your changes resolve them, light on the left of each swatch and dark on the right. Components read these roles, never a color of their own."
    >
      {colorGroups.map((g) => (
        <Section title={g.title} description={notes[g.title]}>
          <View class="flex-row flex-wrap gap-4">
            {g.pairs.map(([fill, text]) => (
              <ColorTile fill={fill} text={text} width={tileWidth()} />
            ))}
          </View>
        </Section>
      ))}
    </Page>
  );
}

/** Text that reads on token `fill` in `s`: its pair, else the theme's dark or light foreground. */
function inkOn(fill: string, text: string | null, s: Scheme): string {
  if (text) return cssColor(text, s);
  const c = colorOf(fill, s);
  const light = !c || c.alpha < 0.5 ? s === "light" : c.l > 0.62;
  return cssColor("foreground", light ? "light" : "dark");
}

/** A color in light and in dark: a split swatch with each value on it, then its names. */
function ColorTile(props: { fill: string; text: string | null; width: number }) {
  // Each half sits on its appearance's background, so a translucent color shows as it would there.
  const half = (s: Scheme) => (
    <View class="flex-1" style={{ backgroundColor: cssColor("background", s) }}>
      <View
        class="flex-1 justify-between p-3"
        style={{ backgroundColor: cssColor(props.fill, s) }}
        accessibilityLabel={`${props.fill}, ${s}`}
      >
        <Text class="text-base font-medium" style={{ color: inkOn(props.fill, props.text, s), opacity: props.text ? 1 : 0 }}>
          Aa
        </Text>
        <Text class="font-mono text-xs tabular-nums" style={{ color: inkOn(props.fill, props.text, s) }}>
          {formatValue(props.fill, s)}
        </Text>
      </View>
    </View>
  );
  return (
    <View class="overflow-hidden rounded-xl border border-border" style={{ width: props.width }}>
      <View class="h-24 flex-row">
        {half("light")}
        {half("dark")}
      </View>
      <View class="gap-1 border-t border-border px-3 py-2.5">
        <Text class="truncate font-mono text-xs text-foreground">{props.fill}</Text>
        <Text class="truncate font-mono text-xs text-muted-foreground">{props.text ?? " "}</Text>
      </View>
    </View>
  );
}
