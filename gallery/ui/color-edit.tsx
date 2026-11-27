// A color edited in place: its swatch opens a popover with `nsh/base`'s
// color picker (area, hue, the value as oklch) and the theme's own colors.

import { Text, View } from "nsh";
import { ColorArea, ColorField, ColorPicker, ColorSlider, ColorSwatch, Popover } from "nsh/base";
import { colorOf, cssColor, scheme } from "../theme";
import { toHex } from "./color";

const quick = ["foreground", "primary", "secondary", "muted", "accent", "destructive", "border", "chart-1", "chart-2", "chart-3", "chart-4", "chart-5"];

/** The swatch of token `name` in the shown appearance; pressing it edits it. */
export function ColorEdit(props: { name: string; onChange: (oklch: string) => void; size?: "sm" | "md" }) {
  const hex = () => {
    const c = colorOf(props.name, scheme());
    return c ? toHex(c) : "#00000000";
  };
  return (
    <Popover
      placement="bottom"
      align="end"
      offset={6}
      popupClass="w-64 rounded-lg border border-border bg-popover p-3 shadow-md"
      trigger={
        <View
          accessibilityLabel={`Edit ${props.name}`}
          class={`${props.size === "md" ? "size-8 rounded-md" : "size-6 rounded-[5px]"} border border-border focus-visible:ring-[3px] focus-visible:ring-ring/50`}
          style={{ backgroundColor: cssColor(props.name) }}
        />
      }
    >
      <ColorPicker
        value={hex()}
        format="oklch"
        onChange={(e) => props.onChange(e.formatted)}
        class="gap-3"
      >
        <View class="flex-row items-center justify-between">
          <Text class="font-mono text-xs text-popover-foreground">{props.name}</Text>
          <Text class="text-xs text-muted-foreground">{scheme() === "dark" ? "Dark" : "Light"}</Text>
        </View>
        <ColorArea
          class="h-36"
          panelClass="rounded-md"
          thumbClass="size-4 rounded-full border-2 border-background shadow-sm"
          accessibilityLabel="Saturation and brightness"
        />
        <ColorSlider
          colorSpace="oklch"
          channel="hue"
          class="h-4 justify-center"
          trackClass="h-3 rounded-full"
          thumbClass="size-4 rounded-full border-2 border-background shadow-sm"
          accessibilityLabel="Hue"
        />
        <ColorField
          format="oklch"
          accessibilityLabel="Value"
          class="h-8 rounded-md border border-input bg-background px-2 font-mono text-xs text-foreground dark:bg-input/30 focus:border-ring"
        />
        <View class="flex-row flex-wrap gap-1.5">
          {quick.map((name) => (
            <ColorSwatch
              color={cssColor(name)}
              accessibilityLabel={name}
              class="size-5 rounded-[4px] border border-border"
            />
          ))}
        </View>
      </ColorPicker>
    </Popover>
  );
}
