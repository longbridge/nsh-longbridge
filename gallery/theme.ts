// The gallery's theme: a color preset, a density preset and the Theme
// page's tweaks, merged into one theme file and applied with `setTheme`.

import { computed, effect, setColorScheme, setTheme, signal } from "nsh";
import blue from "../themes/blue.json";
import compact from "../themes/compact.json";
import rose from "../themes/rose.json";

export type Vars = Record<string, string>;
export interface ThemeFile {
  name?: string;
  cssVars: { theme?: Vars; light?: Vars; dark?: Vars };
}

export interface Preset {
  id: string;
  label: string;
  file: ThemeFile | null;
}

export const colorPresets: Preset[] = [
  { id: "neutral", label: "Neutral", file: null },
  { id: "blue", label: "Blue", file: blue as ThemeFile },
  { id: "rose", label: "Rose", file: rose as ThemeFile },
];

export const densityPresets: Preset[] = [
  { id: "default", label: "Default", file: null },
  { id: "compact", label: "Compact", file: compact as ThemeFile },
];

/** Primary colors the Theme page offers (Tailwind's 600 in light, 500 in dark). */
export const swatches = [
  { id: "violet", label: "Violet", light: "oklch(0.541 0.281 293.009)", dark: "oklch(0.606 0.25 292.717)" },
  { id: "green", label: "Green", light: "oklch(0.627 0.194 149.214)", dark: "oklch(0.723 0.219 149.579)" },
  { id: "orange", label: "Orange", light: "oklch(0.646 0.222 41.116)", dark: "oklch(0.705 0.213 47.604)" },
  { id: "sky", label: "Sky", light: "oklch(0.588 0.158 241.966)", dark: "oklch(0.685 0.169 237.323)" },
];

export type Appearance = "light" | "dark" | "system";

export const appearance = signal<Appearance>("system");
export const colorPreset = signal("neutral");
export const densityPreset = signal("default");
export const radius = signal<number | null>(null);
export const controlHeight = signal<number | null>(null);
export const primary = signal<string | null>(null);

/** `radius` and `control-height` before any tweak. */
export const RADIUS_DEFAULT = 10;
export const controlHeightDefault = () => (densityPreset() === "compact" ? 28 : 32);

export function resetTweaks() {
  radius.set(null);
  controlHeight.set(null);
  primary.set(null);
}

function merge(files: (ThemeFile | null)[]): ThemeFile {
  const out: ThemeFile = { cssVars: {} };
  for (const file of files) {
    if (!file) continue;
    for (const section of ["theme", "light", "dark"] as const) {
      const vars = file.cssVars[section];
      if (vars) out.cssVars[section] = { ...out.cssVars[section], ...vars };
    }
  }
  return out;
}

function tweaks(): ThemeFile | null {
  const theme: Vars = {};
  const light: Vars = {};
  const dark: Vars = {};
  const r = radius();
  if (r !== null) theme.radius = `${r}px`;
  const h = controlHeight();
  if (h !== null) theme["control-height"] = `${h}px`;
  const swatch = swatches.find((s) => s.id === primary());
  if (swatch) {
    light.primary = swatch.light;
    dark.primary = swatch.dark;
    light["primary-foreground"] = dark["primary-foreground"] = "oklch(0.985 0 0)";
    light.ring = swatch.light;
    dark.ring = swatch.dark;
  }
  const empty = (v: Vars) => Object.keys(v).length === 0;
  if (empty(theme) && empty(light) && empty(dark)) return null;
  return { cssVars: { theme, light, dark } };
}

const find = (presets: Preset[], id: string) => presets.find((p) => p.id === id)?.file ?? null;

/** The theme shown: the presets, then the tweaks. */
export const currentTheme = computed(() =>
  merge([find(colorPresets, colorPreset()), find(densityPresets, densityPreset()), tweaks()]),
);

/** The theme shown as a file an app's `nsh.json` can name. */
export const themeJson = computed(() =>
  JSON.stringify({ name: "my-theme", ...currentTheme() }, null, 2),
);

/** Keeps the app's theme and color scheme on the gallery's choices. */
export function followTheme() {
  effect(() => {
    const theme = currentTheme();
    const empty = Object.keys(theme.cssVars).length === 0;
    setTheme(empty ? null : theme);
  });
  effect(() => setColorScheme(appearance()));
}
