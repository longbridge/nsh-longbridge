// The gallery's theme: a color preset, a density preset and the edits made
// on the Customize page and in the component pages' Tokens sections, merged
// into one theme file and applied with `setTheme`. It also resolves any
// token of the library as the library does, from `themes/default.json` (the
// registry's copy of every token with its derived default), so the gallery
// shows live values in both appearances.

import { colorScheme, computed, effect, setColorScheme, setTheme, signal } from "nsh";
import amber from "../themes/amber.json";
import blue from "../themes/blue.json";
import compact from "../themes/compact.json";
import defaults from "../themes/default.json";
import green from "../themes/green.json";
import indigo from "../themes/indigo.json";
import rose from "../themes/rose.json";
import { type Lab, parseColor, toHex } from "./ui/color";

export type Vars = Record<string, string>;
export type Scheme = "light" | "dark";
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
  { id: "indigo", label: "Indigo", file: indigo as ThemeFile },
  { id: "blue", label: "Blue", file: blue as ThemeFile },
  { id: "green", label: "Green", file: green as ThemeFile },
  { id: "amber", label: "Amber", file: amber as ThemeFile },
  { id: "rose", label: "Rose", file: rose as ThemeFile },
];

export const densityPresets: Preset[] = [
  { id: "default", label: "Default", file: null },
  { id: "compact", label: "Compact", file: compact as ThemeFile },
];

export type Appearance = "light" | "dark" | "system";

export const appearance = signal<Appearance>("system");
export const colorPreset = signal("neutral");
export const densityPreset = signal("default");

/** What the user changed, over the presets: seeds, colors, component tokens. */
export const edits = signal<Required<ThemeFile["cssVars"]>>({ theme: {}, light: {}, dark: {} });

/** The appearance shown. */
export const scheme = (): Scheme => colorScheme();

// ---------------------------------------------------------------- the file

function merge(files: (ThemeFile | null | undefined)[]): ThemeFile {
  const out: ThemeFile = { cssVars: {} };
  for (const file of files) {
    if (!file) continue;
    for (const section of ["theme", "light", "dark"] as const) {
      const vars = file.cssVars[section];
      if (vars && Object.keys(vars).length > 0) out.cssVars[section] = { ...out.cssVars[section], ...vars };
    }
  }
  return out;
}

const find = (presets: Preset[], id: string) => presets.find((p) => p.id === id)?.file ?? null;

/** The theme shown: the presets, then the edits. */
export const currentTheme = computed(() =>
  merge([find(colorPresets, colorPreset()), find(densityPresets, densityPreset()), { cssVars: edits() }]),
);

/** The theme shown as a file an app's `nsh.json` can name. */
export const themeJson = computed(() => JSON.stringify({ name: "my-theme", ...currentTheme() }, null, 2));

/** Keeps the app's theme and color scheme on the gallery's choices. */
export function followTheme() {
  effect(() => {
    const theme = currentTheme();
    const empty = Object.keys(theme.cssVars).length === 0;
    setTheme(empty ? null : theme);
  });
  effect(() => setColorScheme(appearance()));
}

// ---------------------------------------------------------------- edits

export type Section = "theme" | Scheme;

/** Sets token `name` in `section` (`null` takes the edit back). */
export function setToken(section: Section, name: string, value: string | null) {
  const next = { ...edits(), [section]: { ...edits()[section] } };
  if (value === null) delete next[section][name];
  else next[section][name] = value;
  edits.set(next);
}

/** The section a token's value lives in: colors per appearance. */
export const sectionOf = (name: string): Section => (kindOf(name) === "color" ? scheme() : "theme");

export const isEdited = (name: string) => {
  const e = edits();
  return name in e.theme || name in e.light || name in e.dark;
};

export function resetToken(name: string) {
  const e = edits();
  const drop = (v: Vars) => Object.fromEntries(Object.entries(v).filter(([k]) => k !== name));
  edits.set({ theme: drop(e.theme), light: drop(e.light), dark: drop(e.dark) });
}

export function resetTokens(names: string[]) {
  const e = edits();
  const drop = (v: Vars) => Object.fromEntries(Object.entries(v).filter(([k]) => !names.includes(k)));
  edits.set({ theme: drop(e.theme), light: drop(e.light), dark: drop(e.dark) });
}

export function resetAll() {
  edits.set({ theme: {}, light: {}, dark: {} });
}

export const editCount = () => {
  const e = edits();
  return new Set([...Object.keys(e.theme), ...Object.keys(e.light), ...Object.keys(e.dark)]).size;
};

// ---------------------------------------------------------------- seeds

export interface Seed {
  name: string;
  label: string;
  min: number;
  max: number;
  step: number;
}

export const seeds: Seed[] = [
  { name: "radius", label: "Radius", min: 0, max: 20, step: 1 },
  { name: "spacing", label: "Spacing", min: 3, max: 5, step: 0.5 },
  { name: "control-height", label: "Control height", min: 24, max: 40, step: 2 },
  { name: "font-size", label: "Font size", min: 12, max: 16, step: 1 },
];

export const setSeed = (name: string, value: number) => setToken("theme", name, `${value}px`);

// ---------------------------------------------------------------- resolving

const resolved = computed(() => merge([defaults as ThemeFile, currentTheme()]).cssVars);

/** A token's value as written, in `s` (colors) or the theme section. */
export function raw(name: string, s: Scheme = scheme()): string | undefined {
  const vars = resolved();
  return vars[s]?.[name] ?? vars.theme?.[name];
}

/** A token's default as the library derives it (before presets and edits). */
export function derived(name: string, s: Scheme = scheme()): string | undefined {
  const vars = (defaults as ThemeFile).cssVars;
  return vars[s]?.[name] ?? vars.theme?.[name];
}

/** `value` with every `var(--x)` replaced by x's value, recursively. */
function substitute(value: string, s: Scheme, depth = 0): string {
  if (depth > 12) return value;
  return value.replace(/var\(--([a-z0-9-]+)\)/g, (_, name: string) => {
    const v = raw(name, s);
    return v === undefined ? "transparent" : substitute(v, s, depth + 1);
  });
}

/** A color token in `s`, read. */
export function colorOf(name: string, s: Scheme = scheme()): Lab | null {
  const v = raw(name, s);
  return v === undefined ? null : parseColor(substitute(v, s));
}

/** A color token in `s` as a style value (`#rrggbbaa`). */
export function cssColor(name: string, s: Scheme = scheme()): string {
  const c = colorOf(name, s);
  return c ? toHex(c) : "transparent";
}

/** A length or number token, evaluated (`rem` as 16, `px` as points). */
export function numberOf(name: string, s: Scheme = scheme()): number {
  const v = raw(name, s);
  return v === undefined ? NaN : evaluate(substitute(v, s));
}

/** Evaluates `calc()` arithmetic of `px`, `rem` and plain numbers. */
function evaluate(expr: string): number {
  const tokens = expr.replace(/calc\(/g, "(").match(/[\d.]+(px|rem|ms|%)?|[-+*/()]/g) ?? [];
  let i = 0;
  const atom = (): number => {
    const t = tokens[i++];
    if (t === "(") {
      const v = sum();
      i++;
      return v;
    }
    if (t === "-") return -atom();
    const n = parseFloat(t);
    return t.endsWith("rem") ? n * 16 : n;
  };
  const product = (): number => {
    let v = atom();
    while (tokens[i] === "*" || tokens[i] === "/") v = tokens[i++] === "*" ? v * atom() : v / atom();
    return v;
  };
  const sum = (): number => {
    let v = product();
    while (tokens[i] === "+" || tokens[i] === "-") v = tokens[i++] === "+" ? v + product() : v - product();
    return v;
  };
  return sum();
}

/**
 * The length tokens that follow `seeds` (`{ radius: "16px" }`), evaluated
 * with them over the current theme. A `ThemeScope` that sets a seed must set
 * these too: the app's theme gives each of them a value of its own, made from
 * its own seed, and that wins over the scope's seed.
 */
export function followers(seeds: Vars): Vars {
  const vars = resolved().theme ?? {};
  const s = scheme();
  const names = Object.keys(seeds);
  const refs = (v: string) => [...v.matchAll(/var\(--([a-z0-9-]+)\)/g)].map((m) => m[1]);
  const known = new Map<string, boolean>();
  const follows = (name: string, depth = 0): boolean => {
    if (names.includes(name)) return true;
    if (known.has(name)) return known.get(name)!;
    const v = vars[name];
    const r = v !== undefined && depth < 12 && refs(v).some((n) => follows(n, depth + 1));
    known.set(name, r);
    return r;
  };
  const uses = (v: string) => refs(v).some((n) => follows(n));
  const sub = (v: string, depth = 0): string =>
    depth > 12
      ? v
      : v.replace(/var\(--([a-z0-9-]+)\)/g, (_, n: string) => {
          const x = seeds[n] ?? raw(n, s);
          return x === undefined ? "0" : sub(x, depth + 1);
        });
  const out: Vars = {};
  for (const [name, v] of Object.entries(vars)) {
    if (names.includes(name) || kindOf(name) !== "length" || !uses(v)) continue;
    const expr = sub(v);
    // `max()` and the like are left to the library.
    if (/[a-z]\(/.test(expr.replace(/calc\(/g, "("))) continue;
    const n = evaluate(expr);
    if (Number.isFinite(n)) out[name] = `${Number(n.toFixed(3))}px`;
  }
  return out;
}

export type TokenKind = "color" | "length" | "ms" | "tier" | "number";

/** What a token holds, from its section and its name. */
export function kindOf(name: string): TokenKind {
  const vars = (defaults as ThemeFile).cssVars;
  if (vars.light?.[name] !== undefined) return "color";
  if (/duration|delay/.test(name)) return "ms";
  if (/-shadow$/.test(name)) return "tier";
  if (/opacity|animate|weight/.test(name)) return "number";
  return "length";
}

/** A token's value for reading: `10`, `150 ms`, `sm`. */
export function formatValue(name: string, s: Scheme = scheme()): string {
  const kind = kindOf(name);
  if (kind === "color") return colorOf(name, s) ? toHex(colorOf(name, s)!) : "—";
  const n = numberOf(name, s);
  const r = String(Number(n.toFixed(2)));
  // Durations read in whole milliseconds (`duration × 4 / 3` of another seed is not).
  if (kind === "ms") return `${Math.round(n)} ms`;
  if (kind === "tier") return ["none", "xs", "sm", "md", "lg"][Math.round(n)] ?? r;
  return r;
}

/** A derived default as tokens.md writes it: `primary`, `button-default × 90%`, `radius − 2`. */
export function formatDerived(value: string | undefined): string {
  if (value === undefined) return "—";
  const v = value.trim();
  const mix = /^color-mix\(in oklab, var\(--([a-z0-9-]+)\) ([\d.]+)%, transparent\)$/.exec(v);
  if (mix) return `${mix[1]} × ${mix[2]}%`;
  const ref = /^var\(--([a-z0-9-]+)\)$/.exec(v);
  if (ref) return ref[1];
  const calc = /^calc\((.*)\)$/.exec(v);
  if (calc) {
    return calc[1]
      .replace(/var\(--([a-z0-9-]+)\)/g, "$1")
      .replace(/(\d)px/g, "$1")
      .replace(/ - /g, " − ")
      .replace(/ \* /g, " × ");
  }
  return v.replace(/(\d)px$/, "$1");
}

/** The primary color a color preset shows in the shown appearance, for its dot. */
export function presetPrimary(id: string): string {
  const file = colorPresets.find((p) => p.id === id)?.file;
  const s = scheme();
  const value = file?.cssVars[s]?.primary ?? derived("primary", s);
  const color = value ? parseColor(value) : null;
  return color ? toHex(color) : cssColor("primary", s);
}

// ---------------------------------------------------------------- the registry

/** Semantic colors, grouped as the Colors page and the editor show them. */
export const colorGroups: { title: string; pairs: [string, string | null][] }[] = [
  { title: "Base", pairs: [["background", "foreground"]] },
  { title: "Surfaces", pairs: [["card", "card-foreground"], ["popover", "popover-foreground"]] },
  {
    title: "Roles",
    pairs: [
      ["primary", "primary-foreground"],
      ["secondary", "secondary-foreground"],
      ["muted", "muted-foreground"],
      ["accent", "accent-foreground"],
      ["destructive", "destructive-foreground"],
    ],
  },
  {
    title: "Status",
    pairs: [
      ["success", "success-foreground"],
      ["warning", "warning-foreground"],
      ["info", "info-foreground"],
    ],
  },
  {
    title: "Lines and focus",
    pairs: [
      ["border", null],
      ["input", null],
      ["ring", null],
      ["ring-invalid", null],
    ],
  },
  {
    title: "Charts",
    pairs: [
      ["chart-1", null],
      ["chart-2", null],
      ["chart-3", null],
      ["chart-4", null],
      ["chart-5", null],
    ],
  },
  {
    title: "Sidebar",
    pairs: [
      ["sidebar", "sidebar-foreground"],
      ["sidebar-primary", "sidebar-primary-foreground"],
      ["sidebar-accent", "sidebar-accent-foreground"],
      ["sidebar-border", null],
      ["sidebar-ring", null],
    ],
  },
  { title: "Overlays", pairs: [["overlay", null], ["shadow", null]] },
];

export const semanticColors = colorGroups.flatMap((g) => g.pairs.flatMap(([a, b]) => (b ? [a, b] : [a])));

const allTokens = [
  ...Object.keys((defaults as ThemeFile).cssVars.theme ?? {}),
  ...Object.keys((defaults as ThemeFile).cssVars.light ?? {}),
].filter((n, i, list) => list.indexOf(n) === i);

/** The tokens of one kind of component: `button-*`, `checkbox` and `checkbox-*`. */
export function componentTokens(prefix: string): string[] {
  return allTokens.filter(
    (n) => (n === prefix || n.startsWith(`${prefix}-`)) && !semanticColors.includes(n) && n !== "input",
  );
}
