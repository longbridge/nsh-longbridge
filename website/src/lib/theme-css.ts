// The page's colors as CSS, written at build time from the same sources the
// gallery draws with, so the HTML around the demos and the posters under
// them match what the demos draw:
//
// - the site's own colors (`gallery/ui/site-colors.ts`) as `--s-*`, per
//   appearance (`html.dark`);
// - the library's tokens the posters use (`themes/default.json`, then each
//   preset's file) as `--lb-*`, per appearance and preset
//   (`html[data-preset="…"]`).
// - the nebula behind the hero (`backdrop.ts`), per preset
//   (`nebula-colors.ts`) as `--nebula-0` to `--nebula-4`, the same in both
//   appearances.

import defaults from '../../../themes/default.json';
import amber from '../../../themes/amber.json';
import blue from '../../../themes/blue.json';
import green from '../../../themes/green.json';
import indigo from '../../../themes/indigo.json';
import rose from '../../../themes/rose.json';
import compact from '../../../themes/compact.json';
import { parseColor } from '../../../gallery/ui/color';
import { sites } from '../../../gallery/ui/site-colors';
import { nebulaColors } from './nebula-colors';

type Vars = Record<string, string>;
interface ThemeFile {
  cssVars: { theme?: Vars; light?: Vars; dark?: Vars };
}

/** The presets, in the order the pickers show them (`gallery/theme.ts`). */
export const presets: { id: string; label: string; file: ThemeFile | null }[] = [
  { id: 'neutral', label: 'Neutral', file: null },
  { id: 'indigo', label: 'Indigo', file: indigo as ThemeFile },
  { id: 'blue', label: 'Blue', file: blue as ThemeFile },
  { id: 'green', label: 'Green', file: green as ThemeFile },
  { id: 'amber', label: 'Amber', file: amber as ThemeFile },
  { id: 'rose', label: 'Rose', file: rose as ThemeFile },
];

export const densities = [
  { id: 'default', label: 'Default', file: null as ThemeFile | null },
  { id: 'compact', label: 'Compact', file: compact as ThemeFile },
];

/** The library's colors the posters draw with. */
const COLORS = [
  'background', 'foreground', 'card', 'card-foreground', 'popover', 'popover-foreground', 'primary',
  'primary-foreground', 'secondary', 'secondary-foreground', 'muted', 'muted-foreground', 'accent',
  'accent-foreground', 'border', 'input', 'ring',
];
/** Its seeds (lengths), which the posters derive their sizes from as the library does. */
const SEEDS = ['radius', 'spacing', 'font-size', 'control-height'];

const base = defaults as ThemeFile;

const block = (selector: string, vars: Vars) =>
  `${selector}{${Object.entries(vars).map(([k, v]) => `${k}:${v}`).join(';')}}`;

/** `--s-*` for each of the site's colors (`code` as `--s-code-*`). */
function siteVars(s: (typeof sites)['light']): Vars {
  const out: Vars = {};
  for (const [k, v] of Object.entries(s)) {
    if (typeof v === 'string') out[`--s-${k.replace(/[A-Z]/g, (c) => `-${c.toLowerCase()}`)}`] = v;
    else for (const [ck, cv] of Object.entries(v)) out[`--s-code-${ck}`] = cv;
  }
  return out;
}

function libraryVars(scheme: 'light' | 'dark', file: ThemeFile | null): Vars {
  const out: Vars = {};
  for (const name of COLORS) {
    const v = file?.cssVars[scheme]?.[name] ?? base.cssVars[scheme]?.[name];
    if (v) out[`--lb-${name}`] = v;
  }
  return out;
}

function nebulaVars(id: string, file: ThemeFile | null): Vars {
  const primary = file?.cssVars.light?.primary ?? base.cssVars.light?.primary ?? '';
  return Object.fromEntries(nebulaColors(id, parseColor(primary)).map((c, i) => [`--nebula-${i}`, c]));
}

/** The stylesheet: the site's colors, then each preset's in each appearance. */
export function themeCss(): string {
  const rules: string[] = [];
  const seeds: Vars = {};
  for (const name of SEEDS) seeds[`--lb-${name}`] = base.cssVars.theme?.[name] ?? '';
  rules.push(block(':root', { ...siteVars(sites.light), ...seeds }));
  rules.push(block('html.dark', siteVars(sites.dark)));
  for (const p of presets) {
    const sel = p.id === 'neutral' ? ':root' : `html[data-preset="${p.id}"]`;
    const darkSel = p.id === 'neutral' ? 'html.dark' : `html.dark[data-preset="${p.id}"]`;
    rules.push(block(sel, { ...libraryVars('light', p.file), ...nebulaVars(p.id, p.file) }));
    rules.push(block(darkSel, libraryVars('dark', p.file)));
  }
  for (const d of densities) {
    if (!d.file) continue;
    const vars: Vars = {};
    for (const [k, v] of Object.entries(d.file.cssVars.theme ?? {})) vars[`--lb-${k}`] = v;
    rules.push(block(`html[data-density="${d.id}"]`, vars));
  }
  return rules.join('\n');
}

/** Each preset's primary in each appearance, for the swatches (as hex, which every browser reads). */
export function swatches(): { id: string; label: string; light: string; dark: string }[] {
  const hex = (v: string | undefined) => v ?? '';
  return presets.map((p) => ({
    id: p.id,
    label: p.label,
    light: hex(p.file?.cssVars.light?.primary ?? base.cssVars.light?.primary),
    dark: hex(p.file?.cssVars.dark?.primary ?? base.cssVars.dark?.primary),
  }));
}
