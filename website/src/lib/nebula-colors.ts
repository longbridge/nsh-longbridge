// The nebula's colors for each preset (`backdrop.ts`, `nebula.ts`): five
// stops of a ramp read in Oklab, in the Nocturne gradient's order. They are:
//
// 1. a near black of the hue;
// 2. the deep tone of the primary;
// 3. a bright tone, the primary turned warmer;
// 4. a pale, warm light for the hottest parts;
// 5. an accent across the wheel, in the brightest cores.
//
// The lightness and the chroma follow Nocturne's (which Indigo keeps as it
// is); the hues start from the preset's primary. Neutral is graphite and
// silver with a little cool blue. They are the same in both appearances:
// the stage stays dark in the light one too, so the white sheet and the
// controls on it keep their contrast.

import { toHex, type Lab } from '../../../gallery/ui/color';

/** Nocturne, as getgradients wrote it. */
const NOCTURNE = ['#0b0a1f', '#3b1d8f', '#ff4d8d', '#ff9e5e', '#1b6cff'];

/** A stop: lightness, chroma, and the hue: degrees from the primary's, or fixed (`'=85'`). */
type Stop = [number, number, number | `=${number}`];

const SETS: Record<string, Stop[]> = {
  // Azure toward cyan, a cream light, a violet accent.
  blue: [[0.16, 0.04, 5], [0.36, 0.15, 8], [0.7, 0.15, -28], [0.88, 0.09, '=85'], [0.58, 0.21, '=295']],
  // Deep teal to mint, a pale lime light, an azure accent.
  green: [[0.16, 0.035, 10], [0.37, 0.1, 4], [0.75, 0.15, -14], [0.9, 0.12, '=110'], [0.6, 0.15, '=235']],
  // Rust to amber, a gold light, a blue accent (Nocturne's peach and blue, turned about).
  amber: [[0.16, 0.035, -25], [0.38, 0.12, -35], [0.72, 0.16, -12], [0.88, 0.13, 20], [0.6, 0.18, '=255']],
  // Wine and crimson, a coral rose, a peach light, a violet accent.
  rose: [[0.16, 0.045, -15], [0.36, 0.15, -15], [0.68, 0.2, 5], [0.82, 0.12, 45], [0.58, 0.2, '=295']],
  // Graphite and silver, a near white, a cool blue accent.
  neutral: [[0.15, 0.01, '=260'], [0.35, 0.018, '=255'], [0.7, 0.015, '=250'], [0.9, 0.012, '=240'], [0.64, 0.12, '=240']],
};

/** The nebula's five colors (hex) for the preset `id` with primary `primary`. */
export function nebulaColors(id: string, primary: Lab | null): string[] {
  if (id === 'indigo') return NOCTURNE;
  const gray = !primary || Math.hypot(primary.a, primary.b) < 0.03;
  const set = SETS[gray ? 'neutral' : id] ?? SETS.neutral;
  const hue = primary ? (Math.atan2(primary.b, primary.a) * 180) / Math.PI : 0;
  return set.map(([l, c, h]) => {
    const deg = typeof h === 'number' ? hue + h : Number(h.slice(1));
    const r = (deg * Math.PI) / 180;
    return toHex({ l, a: c * Math.cos(r), b: c * Math.sin(r), alpha: 1 });
  });
}
