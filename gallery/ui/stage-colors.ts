// The stage's colors from a preset's primary, as plain math (no NSH), for the
// gallery's own stage (`site.ts`'s `stageColors`).

import { type Lab, toHex } from "./color";

const lab = (l: number, chroma: number, hue: number, alpha = 1): Lab => ({
  l,
  a: chroma * Math.cos(hue),
  b: chroma * Math.sin(hue),
  alpha,
});

/** How much of the stage's tint a hue (in degrees) takes, so all hues read alike. */
const tintStops: [number, number][] = [
  [0, 0.8],
  [40, 0.5],
  [70, 0.36],
  [110, 0.42],
  [168, 0.55],
  [210, 0.75],
  [242, 0.9],
  [277, 0.88],
  [320, 0.85],
  [360, 0.8],
];

function tintScale(degrees: number): number {
  const h = ((degrees % 360) + 360) % 360;
  for (let i = 1; i < tintStops.length; i++) {
    const [h1, k1] = tintStops[i];
    if (h <= h1) {
      const [h0, k0] = tintStops[i - 1];
      return k0 + ((k1 - k0) * (h - h0)) / (h1 - h0);
    }
  }
  return 1;
}

/**
 * The stage's colors from the preset's primary. Only its hue: the backdrop
 * is a deep, quiet tint of it (low chroma, ink-dark in both appearances)
 * with a softer light of the hue from the top, so the components on the
 * stage carry the color and the backdrop only hints at it. A gray primary
 * (Neutral) gives the graphite stage.
 *
 * The eye reads chroma unevenly across hues: at the same chroma a deep
 * green or amber looks louder (and amber turns to mud) than a deep blue.
 * So the tint's chroma scales by hue, and every preset's stage reads as the
 * same quiet ink.
 */
export function stagePalette(primary: Lab | null, s: "light" | "dark") {
  const p = primary ?? { l: 0.2, a: 0, b: 0, alpha: 1 };
  const hue = Math.atan2(p.b, p.a);
  const gray = Math.hypot(p.a, p.b) < 0.03;
  if (gray) {
    // Gray stages lean to the site's ink blue.
    const l = s === "light" ? 0.24 : 0.29;
    const h = -1.45;
    return {
      base: toHex(lab(l, 0.012, h)),
      glow: toHex(lab(l + 0.13, 0.0126, h)),
      floor: toHex(lab(l - 0.07, 0.0108, h)),
      line: "#ffffff10",
      mark: "#ffffff59",
    };
  }
  const l = s === "light" ? 0.27 : 0.25;
  const k = tintScale((hue * 180) / Math.PI);
  return {
    base: toHex(lab(l, 0.045 * k, hue)),
    glow: toHex(lab(l + 0.13, 0.085 * k, hue)),
    floor: toHex(lab(l - 0.07, 0.03 * k, hue)),
    line: "#ffffff0d",
    mark: "#ffffff59",
  };
}
