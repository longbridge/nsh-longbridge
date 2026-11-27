// The gallery's theme pickers, one visual language wherever the theme is
// chosen (the header's Theme popover, the home page's stage, the theme
// studio): presets as solid round swatches of the preset's primary (a
// hairline inside, a faint top light) and densities as small tiles drawing
// rows at their spacing. The chosen one wears a ring set off from it by a
// gap.

import { Text, View, type JSX } from "nsh";
import { colorPreset, colorPresets, densityPreset, densityPresets, presetPrimary, scheme } from "../theme";
import { site } from "./site";

/** Where a picker sits: on the page's surfaces, or on the stage's colored glass. */
export type PickerTone = "surface" | "stage";

const ringColor = (tone: PickerTone, on: boolean) =>
  on ? (tone === "stage" ? "#ffffff" : site().ink) : "transparent";

/** A ring around `children`, a `gap` away from them, drawn while `on`. */
function Ring(props: { on: boolean; tone: PickerTone; radius: number; children?: JSX.Element }) {
  return (
    <View
      hoverStyle={props.on ? undefined : { borderColor: props.tone === "stage" ? "#ffffff66" : site().input }}
      style={{
        padding: 2.5,
        borderWidth: 1.5,
        borderRadius: props.radius + 4,
        borderColor: ringColor(props.tone, props.on),
      }}
    >
      {props.children}
    </View>
  );
}

/** One preset's swatch: its primary, solid, with a hairline inside and a faint light from the top. */
export function PresetSwatch(props: { id: string; size?: number; tone?: PickerTone }) {
  const side = props.size ?? 32;
  const edge = () => (props.tone === "stage" ? "#ffffff40" : scheme() === "dark" ? "#ffffff26" : "#0000001f");
  return (
    <View
      class="rounded-full"
      style={{
        width: side,
        height: side,
        backgroundColor: presetPrimary(props.id),
        backgroundImage: "linear-gradient(180deg, #ffffff2e 0%, #ffffff00 55%)",
        boxShadow: `inset 0 0 0 1px ${edge()}`,
      }}
    />
  );
}

/**
 * The color presets as swatches. `labels`: each name under its swatch
 * (otherwise the caller names the chosen one). `size`: the swatch's side.
 * `gap`: the space between them, or `spread` to share out the row's width.
 */
export function PresetSwatches(props: { tone?: PickerTone; size?: number; labels?: boolean; gap?: number; spread?: boolean }) {
  const tone = props.tone ?? "surface";
  const size = props.size ?? 28;
  return (
    <View
      class={props.spread ? "flex-row justify-between" : "flex-row"}
      style={props.spread ? undefined : { gap: props.gap ?? 10 }}
      role="radiogroup"
      accessibilityLabel="Color preset"
    >
      {colorPresets.map((p) => {
        const on = () => colorPreset() === p.id;
        return (
          <View
            class="items-center gap-2"
            style={{ cursor: "default" }}
            role="radio"
            accessibilityState={{ checked: on() }}
            accessibilityLabel={p.label}
            onPress={() => colorPreset.set(p.id)}
          >
            <Ring on={on()} tone={tone} radius={size / 2}>
              <PresetSwatch id={p.id} size={size} tone={tone} />
            </Ring>
            <If when={props.labels}>
              <Text
                style={{
                  fontSize: 12,
                  fontWeight: on() ? "550" : "400",
                  color: tone === "stage" ? (on() ? "#ffffff" : "#ffffffa6") : on() ? site().ink : site().muted,
                }}
              >
                {p.label}
              </Text>
            </If>
          </View>
        );
      })}
    </View>
  );
}

/** The chosen color preset's name. */
export const presetLabel = () => colorPresets.find((p) => p.id === colorPreset())?.label ?? "";

/** The chosen density's name. */
export const densityLabel = () => densityPresets.find((p) => p.id === densityPreset())?.label ?? "";

const rows: Record<string, { gap: number; bar: number }> = {
  default: { gap: 5, bar: 5 },
  compact: { gap: 2.5, bar: 4 },
};

/** The densities as tiles, each drawing three rows at its spacing, with its name under it. */
export function DensityTiles(props: { width?: number; labels?: boolean }) {
  const width = props.width ?? 64;
  return (
    <View class="flex-row gap-3" role="radiogroup" accessibilityLabel="Density">
      {densityPresets.map((p) => {
        const on = () => densityPreset() === p.id;
        const r = rows[p.id] ?? rows.default;
        return (
          <View
            class="items-center gap-2"
            style={{ cursor: "default" }}
            role="radio"
            accessibilityState={{ checked: on() }}
            accessibilityLabel={p.label}
            onPress={() => densityPreset.set(p.id)}
          >
            <Ring on={on()} tone="surface" radius={8}>
              <View
                class="justify-center rounded-lg px-2.5"
                style={{
                  width,
                  height: 40,
                  gap: r.gap,
                  backgroundColor: site().card,
                  borderWidth: 1,
                  borderColor: site().border,
                }}
              >
                {[1, 0.7, 0.85].map((w, i) => (
                  <View
                    class={`rounded-full ${i === 0 ? "bg-primary" : ""}`}
                    style={{ height: r.bar, width: `${w * 100}%`, backgroundColor: i === 0 ? undefined : site().input }}
                  />
                ))}
              </View>
            </Ring>
            <If when={props.labels !== false}>
              <Text style={{ fontSize: 12, fontWeight: on() ? "550" : "400", color: on() ? site().ink : site().muted }}>
                {p.label}
              </Text>
            </If>
          </View>
        );
      })}
    </View>
  );
}
