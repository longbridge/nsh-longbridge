// Colors as the theme writes them (`oklch()`, hex, `color-mix()` in oklab,
// `transparent`), read into Oklab with alpha, so the gallery can show a
// derived token's value and give a picker a color it reads.

export interface Lab {
  l: number;
  a: number;
  b: number;
  alpha: number;
}

const TRANSPARENT: Lab = { l: 0, a: 0, b: 0, alpha: 0 };

/** A color value, or `null` when it is no color the gallery reads. */
export function parseColor(input: string): Lab | null {
  const s = input.trim().toLowerCase();
  if (s === "transparent") return TRANSPARENT;
  if (s.startsWith("#")) return fromHex(s);
  const fn = /^([a-z-]+)\((.*)\)$/.exec(s);
  if (!fn) return null;
  const [, name, body] = fn;
  if (name === "color-mix") return parseMix(body);
  const [main, alphaPart] = body.split("/");
  const parts = main.trim().split(/[\s,]+/).map(num);
  const alpha = alphaPart === undefined ? 1 : num(alphaPart.trim());
  if (parts.some(Number.isNaN) || Number.isNaN(alpha)) return null;
  if (name === "oklch") {
    const [l, c, h] = parts;
    const rad = (h * Math.PI) / 180;
    return { l, a: c * Math.cos(rad), b: c * Math.sin(rad), alpha };
  }
  if (name === "oklab") return { l: parts[0], a: parts[1], b: parts[2], alpha };
  if (name === "rgb" || name === "rgba") return fromRgb(parts[0] / 255, parts[1] / 255, parts[2] / 255, alpha);
  return null;
}

/** `"50%"` as 0.5, `"0.5"` as 0.5. */
function num(s: string): number {
  return s.endsWith("%") ? parseFloat(s) / 100 : parseFloat(s);
}

function fromHex(s: string): Lab | null {
  let h = s.slice(1);
  if (h.length === 3 || h.length === 4) h = [...h].map((c) => c + c).join("");
  if (h.length !== 6 && h.length !== 8) return null;
  const n = (i: number) => parseInt(h.slice(i, i + 2), 16) / 255;
  return fromRgb(n(0), n(2), n(4), h.length === 8 ? n(6) : 1);
}

/** Splits at the commas outside parentheses. */
function splitTop(s: string): string[] {
  const out: string[] = [];
  let depth = 0;
  let start = 0;
  for (let i = 0; i < s.length; i++) {
    if (s[i] === "(") depth++;
    else if (s[i] === ")") depth--;
    else if (s[i] === "," && depth === 0) {
      out.push(s.slice(start, i).trim());
      start = i + 1;
    }
  }
  out.push(s.slice(start).trim());
  return out;
}

/** `in oklab, <color> [p%], <color> [p%]`, with premultiplied alpha. */
function parseMix(body: string): Lab | null {
  const [space, first, second] = splitTop(body);
  if (!space?.startsWith("in ") || !first || !second) return null;
  const item = (s: string) => {
    const m = /^(.*?)\s+([\d.]+)%$/.exec(s);
    return m ? { color: parseColor(m[1]), p: parseFloat(m[2]) / 100 } : { color: parseColor(s), p: null };
  };
  const x = item(first);
  const y = item(second);
  if (!x.color || !y.color) return null;
  const px = x.p ?? (y.p === null ? 0.5 : 1 - y.p);
  const py = y.p ?? 1 - px;
  const alpha = x.color.alpha * px + y.color.alpha * py;
  if (alpha === 0) return TRANSPARENT;
  const mix = (k: "l" | "a" | "b") =>
    (x.color![k] * x.color!.alpha * px + y.color![k] * y.color!.alpha * py) / alpha;
  return { l: mix("l"), a: mix("a"), b: mix("b"), alpha: Math.min(1, alpha) };
}

const toLinear = (c: number) => (c <= 0.04045 ? c / 12.92 : ((c + 0.055) / 1.055) ** 2.4);
const toGamma = (c: number) => (c <= 0.0031308 ? 12.92 * c : 1.055 * c ** (1 / 2.4) - 0.055);

function fromRgb(r: number, g: number, b: number, alpha: number): Lab {
  const [lr, lg, lb] = [r, g, b].map(toLinear);
  const l = Math.cbrt(0.4122214708 * lr + 0.5363325363 * lg + 0.0514459929 * lb);
  const m = Math.cbrt(0.2119034982 * lr + 0.6806995451 * lg + 0.1073969566 * lb);
  const s = Math.cbrt(0.0883024619 * lr + 0.2817188376 * lg + 0.6299787005 * lb);
  return {
    l: 0.2104542553 * l + 0.793617785 * m - 0.0040720468 * s,
    a: 1.9779984951 * l - 2.428592205 * m + 0.4505937099 * s,
    b: 0.0259040371 * l + 0.7827717662 * m - 0.808675766 * s,
    alpha,
  };
}

/** `#rrggbb`, or `#rrggbbaa` when not opaque. */
export function toHex(c: Lab): string {
  const l = (c.l + 0.3963377774 * c.a + 0.2158037573 * c.b) ** 3;
  const m = (c.l - 0.1055613458 * c.a - 0.0638541728 * c.b) ** 3;
  const s = (c.l - 0.0894841775 * c.a - 1.291485548 * c.b) ** 3;
  const rgb = [
    4.0767416621 * l - 3.3077115913 * m + 0.2309699292 * s,
    -1.2684380046 * l + 2.6097574011 * m - 0.3413193965 * s,
    -0.0041960863 * l - 0.7034186147 * m + 1.707614701 * s,
  ];
  const byte = (v: number) =>
    Math.round(Math.min(1, Math.max(0, v)) * 255)
      .toString(16)
      .padStart(2, "0");
  const hex = "#" + rgb.map((v) => byte(toGamma(Math.min(1, Math.max(0, v))))).join("");
  return c.alpha >= 0.999 ? hex : hex + byte(c.alpha);
}

/** `oklch(L C H)` with three decimals, and `/ A%` when not opaque. */
export function toOklch(c: Lab): string {
  const chroma = Math.sqrt(c.a * c.a + c.b * c.b);
  let hue = (Math.atan2(c.b, c.a) * 180) / Math.PI;
  if (hue < 0) hue += 360;
  const r = (v: number, d = 3) => String(Number(v.toFixed(d)));
  const body = chroma < 0.0005 ? `${r(c.l)} 0 0` : `${r(c.l)} ${r(chroma)} ${r(hue)}`;
  return c.alpha >= 0.999 ? `oklch(${body})` : `oklch(${body} / ${Math.round(c.alpha * 100)}%)`;
}

/** Relative luminance-ish lightness, to pick a readable label over a swatch. */
export function isLight(c: Lab): boolean {
  return c.alpha < 0.5 ? true : c.l > 0.62;
}
