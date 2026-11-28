// The stage backdrop's nebula (`backdrop.ts`): clouds over a deep star field,
// after the Nocturne gradient, drifting slowly. Two passes:
//
// - the field, at a reduced size (the look is soft): a gently domain-warped
//   noise, its height kept at sixteen bits in two channels, so the light read
//   from its slopes has no steps;
// - the picture, at the canvas's size: the clouds cut from the field by a
//   density, lit along their slopes (a broad light and a soft rim on the
//   edges that face it), a few soft bands along the warp and a glow around
//   the bright parts (the field's own mipmaps), quieter in the middle; that
//   value is looked up in the ramp (interpolated in Oklab on the CPU, its
//   chroma calmed). Then a sparse field of stars in the dark, a fine noise
//   laid over in soft light and a still film grain.
//
// The motion is continuous: each layer of the warp reads its noise at a
// point turning slowly round its own ellipse (stretched along the flow, 30°),
// each at its own pace, so the clouds slide and churn without ever jumping,
// turning back or repeating.
//
// A hand can stir it: a trail of pushes (`stroke`), kept small and coarse,
// shifts where the field reads its noise, so the clouds are dragged along a
// finger's path and settle back as the trail spreads and fades.

const VERTEX = `#version 300 es
in vec2 a;
out vec2 vUv;
void main() { vUv = a * 0.5 + 0.5; gl_Position = vec4(a, 0.0, 1.0); }`;

/** The look, after the preset (getgradients' "nebula" generator, seed 311). */
export const NOCTURNE = {
  scale: 1.2,
  density: 0.45,
  warp: 0.75,
  filaments: 0.25,
  rim: 1.2,
  /** Where the light comes from, in radians. */
  light: -1.2,
  stars: 0.5,
  glow: 0.4,
  /**
   * The flow's direction, in degrees, and its reach. The preset loops in 8 s;
   * the page instead lets each layer of the warp turn on its own slow path
   * (47 to 89 s a turn), calmer behind the demo and never repeating.
   */
  direction: 30,
  intensity: 0.5,
  /** The view's offset, in parts of the canvas. */
  shift: [-0.0666, 0.0186] as [number, number],
  /** The ramp's colors, at 0, ¼, ½, ¾ and 1 (the page sets each preset's own, `nebula-colors.ts`). */
  stops: ['#0b0a1f', '#3b1d8f', '#ff4d8d', '#ff9e5e', '#1b6cff'],
  noise: { amount: 0.16, scale: 90 },
  grain: { amount: 0.08 },
};

const NOISE = `
vec2 grad(vec2 p) {
  p = vec2(dot(p, vec2(127.1, 311.7)), dot(p, vec2(269.5, 183.3)));
  return fract(sin(p) * 43758.5453) * 2.0 - 1.0;
}
// Gradient noise, about -0.7 to 0.7.
float noise(vec2 p) {
  vec2 i = floor(p), f = fract(p);
  vec2 u = f * f * f * (f * (f * 6.0 - 15.0) + 10.0);
  return mix(mix(dot(grad(i), f), dot(grad(i + vec2(1.0, 0.0)), f - vec2(1.0, 0.0)), u.x),
             mix(dot(grad(i + vec2(0.0, 1.0)), f - vec2(0.0, 1.0)), dot(grad(i + vec2(1.0, 1.0)), f - vec2(1.0, 1.0)), u.x), u.y);
}
// Three octaves, each turned a little, the last two faint: broad, smooth forms; 0 to 1, mostly 0.3 to 0.7.
float fbm(vec2 p) {
  float v = 0.0, amp = 0.6;
  mat2 turn = mat2(1.6, 1.2, -1.2, 1.6);
  for (int i = 0; i < 3; i++) { v += amp * noise(p); p = turn * p; amp *= 0.3; }
  return 0.5 + v;
}`;

/**
 * The field: the warped noise's height (R the high byte, G the low one) and
 * the filaments (B), where the warp folds.
 */
const FIELD = `#version 300 es
precision highp float;
in vec2 vUv;
out vec4 color;
uniform float uAspect;
uniform float uScale;
uniform float uWarp;
uniform float uTime;
uniform vec2 uFlow;
uniform vec2 uShift;
uniform float uReach;
uniform sampler2D uTrail;
uniform float uPush;
${NOISE}
// Layer k's offset now: a slow turn round an ellipse stretched along the
// flow, each layer at its own pace (once in \`period\` seconds), so the
// layers slide over one another and the whole never repeats nor turns back.
vec2 orbit(float period, float phase) {
  float a = uTime * 6.2831853 / period + phase;
  vec2 across = vec2(-uFlow.y, uFlow.x);
  return (uFlow * cos(a) + across * 0.45 * sin(a)) * uReach;
}
void main() {
  vec2 p = ((vUv - 0.5) * vec2(uAspect, 1.0) - uShift) * uScale * 1.1 + vec2(3.11, 1.7);
  // Dragged by the hand: the clouds read from behind the trail's pushes, so they move with it.
  vec4 t = texture(uTrail, vUv);
  p -= (vec2(t.r + t.g / 255.0, t.b + t.a / 255.0) * 2.0 - 1.0) * uPush;
  // A gentle warp (low, broad), so nothing folds into fine, quick lines.
  float w = uWarp * 2.2;
  vec2 q = vec2(fbm(p + orbit(47.0, 0.0)), fbm(p + vec2(5.2, 1.3) + orbit(61.0, 2.1)));
  vec2 r = vec2(fbm(p + w * q + vec2(1.7, 9.2) + orbit(73.0, 4.2)), fbm(p + w * q + vec2(8.3, 2.8) + orbit(89.0, 1.0)));
  float h = clamp(fbm(p + w * r + orbit(53.0, 3.1) * 0.5), 0.0, 1.0);
  // The filaments: a few broad, soft bands across the second layer of the warp.
  float band = abs(fract(r.x * 3.0 - h * 1.5) - 0.5) * 2.0;
  float fil = smoothstep(0.2, 1.0, band) * smoothstep(0.2, 0.5, length(q - r));
  float hi = floor(h * 255.0) / 255.0;
  color = vec4(hi, (h - hi) * 255.0, fil, 1.0);
}`;

/**
 * The trail of pushes, a step on: spread a little to its neighbours, faded,
 * and the stroke since the last step added along its path (a soft brush).
 * Signed pushes, -1 to 1, kept at sixteen bits in RGBA8 (x in R and G, y
 * in B and A, each the high byte then the low), so they fade smoothly.
 */
const TRAIL = `#version 300 es
precision highp float;
in vec2 vUv;
out vec4 color;
uniform sampler2D uPrev;
uniform vec2 uTexel;
uniform float uKeep;
uniform float uAspect;
uniform vec2 uFrom;
uniform vec2 uTo;
uniform vec2 uMove;
uniform float uRadius;
vec2 at(vec2 uv) { vec4 t = texture(uPrev, uv); return vec2(t.r + t.g / 255.0, t.b + t.a / 255.0) * 2.0 - 1.0; }
vec2 hiLo(float v) { float u = v * 0.5 + 0.5; float hi = floor(u * 255.0) / 255.0; return vec2(hi, (u - hi) * 255.0); }
void main() {
  vec2 c = at(vUv) * 0.8 + 0.05 * (at(vUv + vec2(uTexel.x, 0.0)) + at(vUv - vec2(uTexel.x, 0.0)) + at(vUv + vec2(0.0, uTexel.y)) + at(vUv - vec2(0.0, uTexel.y)));
  c *= uKeep;
  // The stroke: its distance from the path, in stage heights.
  vec2 k = vec2(uAspect, 1.0);
  vec2 pa = (vUv - uFrom) * k, ba = (uTo - uFrom) * k;
  float d = length(pa - ba * clamp(dot(pa, ba) / max(dot(ba, ba), 1e-8), 0.0, 1.0));
  c += uMove * exp(-d * d / (uRadius * uRadius));
  c = clamp(c, -1.0, 1.0);
  color = vec4(hiLo(c.x), hiLo(c.y));
}`;

const PICTURE = `#version 300 es
precision highp float;
in vec2 vUv;
out vec4 color;
uniform sampler2D uField;
uniform sampler2D uRamp;
uniform vec2 uFieldRes;
uniform vec2 uRes;
uniform float uDensity;
uniform float uRim;
uniform vec2 uLight;
uniform float uFilaments;
uniform float uStars;
uniform float uGlow;
uniform float uNoise;
uniform float uNoiseCell;
uniform float uGrain;
uniform float uPoint;
uniform float uDebug;

float height(vec2 uv) { vec4 f = texture(uField, uv); return f.r + f.g / 255.0; }
float broad(vec2 uv) { vec4 f = textureLod(uField, uv, 1.5); return f.r + f.g / 255.0; }
float hash(vec2 p) {
  vec3 q = fract(vec3(p.xyx) * 0.1031);
  q += dot(q, q.yzx + 33.33);
  return fract((q.x + q.y) * q.z);
}
float vnoise(vec2 p) {
  vec2 i = floor(p), f = fract(p);
  vec2 u = f * f * (3.0 - 2.0 * f);
  return mix(mix(hash(i), hash(i + vec2(1.0, 0.0)), u.x), mix(hash(i + vec2(0.0, 1.0)), hash(i + vec2(1.0, 1.0)), u.x), u.y);
}

void main() {
  vec2 uv = vUv;
  float h = height(uv);
  // The slope, in height per stage height, broad: on the field softened, a
  // hundredth of the stage's height each way.
  vec2 d = vec2(uFieldRes.y / uFieldRes.x, 1.0) * 0.01;
  vec2 slope = vec2(broad(uv + vec2(d.x, 0.0)) - broad(uv - vec2(d.x, 0.0)), broad(uv + vec2(0.0, d.y)) - broad(uv - vec2(0.0, d.y))) / 0.02;
  if (uDebug > 0.5) { color = vec4(h, length(slope) * 0.1, 0.0, 1.0); return; }
  // The clouds: the field over the density's line (more density, a lower
  // line), with a soft edge; higher, a little brighter.
  float e = h - (0.845 - uDensity);
  float body = smoothstep(-0.05, 0.12, e);
  float v = body * (0.2 + 0.45 * clamp(e, 0.0, 0.4));
  // The light: the slopes that face it lit in a broad rim, the others a little darker.
  float facing = dot(-slope, uLight);
  v += uRim * 0.32 * smoothstep(0.5, 5.0, facing) * body;
  v -= 0.04 * smoothstep(0.0, 4.0, -facing) * body;
  // The highest parts, warmer.
  v += 0.1 * smoothstep(0.18, 0.4, e);
  v += uFilaments * (textureLod(uField, uv, 1.5).b - 0.25) * body * 0.3;
  // The glow: the field blurred (a small mip), lifting the bright parts' surroundings.
  vec4 soft = textureLod(uField, uv, 4.0);
  float hs = soft.r + soft.g / 255.0 - (0.845 - uDensity);
  v += uGlow * smoothstep(0.0, 0.3, hs) * 0.15;
  // Quieter in the middle, where the cards are; and the top of the ramp (the
  // accent) reached only in the hottest cores.
  float mid = length(vec2((uv.x - 0.5) * 1.6, (uv.y - 0.5) * 0.7));
  v *= mix(0.62, 1.0, smoothstep(0.12, 0.7, mid)) * 0.95;
  v = clamp(v, 0.0, 1.0);
  vec3 c = texture(uRamp, vec2(v * (255.0 / 256.0) + 0.5 / 256.0, 0.5)).rgb;

  // The stars, in the dark: one cell in many holds one, a pixel or two wide.
  vec2 px = gl_FragCoord.xy / uPoint;
  vec2 cell = floor(px / 7.0);
  float s = hash(cell * 1.37 + 11.0);
  if (s > 1.0 - 0.06 * uStars) {
    vec2 at = (cell + 0.2 + 0.6 * vec2(hash(cell + 3.1), hash(cell + 7.7))) * 7.0;
    float d = length(px - at);
    float star = smoothstep(0.9, 0.2, d) * (0.35 + 0.65 * hash(cell + 1.9));
    c += star * (1.0 - body * 0.85) * 0.75;
  }

  // The noise, in soft light: laid over as a layer of the picture itself
  // with the noise mixed in, so it deepens the darks and the colors as much
  // as it mottles them. A cell is a 290th of the stage's width.
  c = clamp(c, 0.0, 1.0);
  float nz = vnoise(gl_FragCoord.xy / uNoiseCell);
  nz = 0.5 + (nz - 0.5) * 1.15;
  vec3 b = mix(c, vec3(nz), uNoise);
  vec3 dc = mix(((16.0 * c - 12.0) * c + 4.0) * c, sqrt(c), step(0.25, c));
  vec3 lit = mix(c - (1.0 - 2.0 * b) * c * (1.0 - c), c + (2.0 * b - 1.0) * (dc - c), step(0.5, b));
  c = uNoise > 0.0 ? lit : c;
  // The grain: on each pixel, still (a grain that moves each frame reads as
  // a shimmer), about 1.3% either way.
  float g = hash(gl_FragCoord.xy) + hash(gl_FragCoord.xy * 1.3 + 517.0) - 1.0;
  c += g * uGrain * 0.4;
  color = vec4(c, 1.0);
}`;

export type Rgb = [number, number, number];

function srgbToLinear(c: number) {
  return c <= 0.04045 ? c / 12.92 : ((c + 0.055) / 1.055) ** 2.4;
}
function linearToSrgb(c: number) {
  return c <= 0.0031308 ? 12.92 * c : 1.055 * c ** (1 / 2.4) - 0.055;
}
function toOklab([r, g, b]: Rgb): Rgb {
  const l = Math.cbrt(0.4122214708 * r + 0.5363325363 * g + 0.0514459929 * b);
  const m = Math.cbrt(0.2119034982 * r + 0.6806995451 * g + 0.1073969566 * b);
  const s = Math.cbrt(0.0883024619 * r + 0.2817188376 * g + 0.6299787005 * b);
  return [0.2104542553 * l + 0.793617785 * m - 0.0040720468 * s, 1.9779984951 * l - 2.428592205 * m + 0.4505937099 * s, 0.0259040371 * l + 0.7827717662 * m - 0.808675766 * s];
}
function fromOklab([L, A, B]: Rgb): Rgb {
  const l = (L + 0.3963377774 * A + 0.2158037573 * B) ** 3;
  const m = (L - 0.1055613458 * A - 0.0638541728 * B) ** 3;
  const s = (L - 0.0894841775 * A - 1.291485548 * B) ** 3;
  return [4.0767416621 * l - 3.3077115913 * m + 0.2309699292 * s, -1.2684380046 * l + 2.6097574011 * m - 0.3413193965 * s, -0.0041960863 * l - 0.7034186147 * m + 1.707614701 * s];
}
function parseHex(h: string): Rgb {
  const n = (i: number) => parseInt(h.slice(1 + i, 3 + i), 16) / 255;
  return [n(0), n(2), n(4)];
}

/** A hex color in Oklab. */
export function oklab(hex: string): Rgb {
  return toOklab(parseHex(hex).map(srgbToLinear) as Rgb);
}

/**
 * The ramp: 256 sRGB texels, five Oklab colors at 0, ¼, ½, ¾ and 1,
 * interpolated in Oklab. Calmed on the way: the chroma at 70%, and the bright
 * stops a little dimmer, so it rests the eye behind the demo.
 */
export function ramp(stops: Rgb[]): Uint8Array {
  const colors = stops.map(([l, a, b], i) => [l * (i >= 2 ? 0.9 : 1), a * 0.7, b * 0.7] as Rgb);
  const out = new Uint8Array(256 * 4);
  const last = colors.length - 1;
  for (let i = 0; i < 256; i++) {
    const t = (i / 255) * last;
    const k = Math.min(last - 1, Math.floor(t));
    const f = t - k;
    const rgb = fromOklab(colors[k].map((v, j) => v + (colors[k + 1][j] - v) * f) as Rgb);
    rgb.forEach((v, j) => (out[i * 4 + j] = Math.round(255 * Math.min(1, Math.max(0, linearToSrgb(Math.max(0, v)))))));
    out[i * 4 + 3] = 255;
  }
  return out;
}

/**
 * Links each `[vertex, fragment]` pair without holding the page: where the
 * driver compiles in the background (`KHR_parallel_shader_compile`) they are
 * polled every 16 ms until done; elsewhere they are waited for at once. Null
 * for one that fails. Attribute 0 is `a`.
 */
export async function linkPrograms(gl: WebGL2RenderingContext, pairs: [string, string][]): Promise<(WebGLProgram | null)[]> {
  const parallel = gl.getExtension('KHR_parallel_shader_compile') as { COMPLETION_STATUS_KHR: number } | null;
  const shaders = new Map<string, WebGLShader>();
  const shader = (type: number, source: string) => {
    let sh = shaders.get(source);
    if (!sh) {
      sh = gl.createShader(type)!;
      gl.shaderSource(sh, source);
      gl.compileShader(sh);
      shaders.set(source, sh);
    }
    return sh;
  };
  const programs = pairs.map(([v, f]) => {
    const pr = gl.createProgram()!;
    gl.attachShader(pr, shader(gl.VERTEX_SHADER, v));
    gl.attachShader(pr, shader(gl.FRAGMENT_SHADER, f));
    gl.bindAttribLocation(pr, 0, 'a');
    gl.linkProgram(pr);
    return pr;
  });
  if (parallel) {
    while (!programs.every((pr) => gl.getProgramParameter(pr, parallel.COMPLETION_STATUS_KHR))) await new Promise((r) => setTimeout(r, 16));
  }
  return programs.map((pr) => (gl.getProgramParameter(pr, gl.LINK_STATUS) ? pr : null));
}

export interface Nebula {
  /**
   * Draws the moment `time` (seconds, any: the motion never repeats) into
   * the bound canvas at `width` by `height` pixels, `point` of them to a CSS
   * pixel, over a field `fieldWidth` by `fieldHeight`.
   */
  draw(o: { time: number; width: number; height: number; point: number; fieldWidth: number; fieldHeight: number }): void;
  /**
   * The trail, `dt` seconds on: the hand's path since the last step, `from`
   * and `to` in the stage's unit square (y up), or null (it fades). False
   * once it has faded away (nothing to step until the next stroke).
   */
  stroke(dt: number, path: { from: [number, number]; to: [number, number] } | null, size: { width: number; height: number }): boolean;
  /** The ramp's five colors, in Oklab. */
  setColors(colors: Rgb[]): void;
}

/** How long a push lasts (s, to 1/e), the brush's radius (stage heights), and how far a stroke drags (of its length). */
const TRAIL_KEEP = 1.6;
const BRUSH = 0.12;
const DRAG = 3;

/**
 * The nebula in `gl`, drawn with the full-screen triangle the caller binds to
 * attribute 0; null if its programs do not link. Its programs compile in the
 * background where the driver can (`linkPrograms`).
 */
export async function makeNebula(gl: WebGL2RenderingContext, look: typeof NOCTURNE = NOCTURNE): Promise<Nebula | null> {
  const [field, picture, trail] = await linkPrograms(gl, [
    [VERTEX, FIELD],
    [VERTEX, PICTURE],
    [VERTEX, TRAIL],
  ]);
  if (!field || !picture || !trail) return null;
  const u = (pr: WebGLProgram, names: string[]) => Object.fromEntries(names.map((n) => [n, gl.getUniformLocation(pr, n)]));
  const fu = u(field, ['uAspect', 'uScale', 'uWarp', 'uTime', 'uFlow', 'uShift', 'uReach', 'uTrail', 'uPush']);
  const pu = u(picture, ['uField', 'uRamp', 'uFieldRes', 'uRes', 'uDensity', 'uRim', 'uLight', 'uFilaments', 'uStars', 'uGlow', 'uNoise', 'uNoiseCell', 'uGrain', 'uPoint', 'uDebug']);
  const tu = u(trail, ['uPrev', 'uTexel', 'uKeep', 'uAspect', 'uFrom', 'uTo', 'uMove', 'uRadius']);

  const texture = (filter: number) => {
    const t = gl.createTexture();
    gl.bindTexture(gl.TEXTURE_2D, t);
    gl.texParameteri(gl.TEXTURE_2D, gl.TEXTURE_MIN_FILTER, filter);
    gl.texParameteri(gl.TEXTURE_2D, gl.TEXTURE_MAG_FILTER, gl.LINEAR);
    gl.texParameteri(gl.TEXTURE_2D, gl.TEXTURE_WRAP_S, gl.CLAMP_TO_EDGE);
    gl.texParameteri(gl.TEXTURE_2D, gl.TEXTURE_WRAP_T, gl.CLAMP_TO_EDGE);
    return t;
  };
  const rampTexture = texture(gl.LINEAR);
  gl.texImage2D(gl.TEXTURE_2D, 0, gl.RGBA8, 256, 1, 0, gl.RGBA, gl.UNSIGNED_BYTE, ramp(look.stops.map(oklab)));

  // The field: RGBA8 (renderable everywhere), with mipmaps for the broad
  // slopes and the glow; its storage is fixed, so a new size takes a new one.
  const framebuffer = gl.createFramebuffer();
  let fieldTexture: WebGLTexture | null = null;
  let fw = 0;
  let fh = 0;
  const fit = (w: number, h: number) => {
    if (fieldTexture && w === fw && h === fh) return;
    if (fieldTexture) gl.deleteTexture(fieldTexture);
    fieldTexture = texture(gl.LINEAR_MIPMAP_LINEAR);
    fw = w;
    fh = h;
    gl.texStorage2D(gl.TEXTURE_2D, Math.floor(Math.log2(Math.max(w, h))) + 1, gl.RGBA8, w, h);
  };

  // The trail: two small RGBA8 textures, one read while the other is drawn,
  // an eighth of the stage's size; 127 and 128 (a half) is no push. A 1×1
  // one stands in at rest.
  const rest = texture(gl.LINEAR);
  gl.texImage2D(gl.TEXTURE_2D, 0, gl.RGBA8, 1, 1, 0, gl.RGBA, gl.UNSIGNED_BYTE, new Uint8Array([127, 128, 127, 128]));
  const trails = [texture(gl.LINEAR), texture(gl.LINEAR)];
  let tw = 0;
  let th = 0;
  let current = 0;
  let alive = 0;
  const clearTrails = () => {
    for (const t of trails) {
      gl.bindFramebuffer(gl.FRAMEBUFFER, framebuffer);
      gl.framebufferTexture2D(gl.FRAMEBUFFER, gl.COLOR_ATTACHMENT0, gl.TEXTURE_2D, t, 0);
      gl.viewport(0, 0, tw, th);
      gl.clearColor(127 / 255, 128 / 255, 127 / 255, 128 / 255);
      gl.clear(gl.COLOR_BUFFER_BIT);
    }
  };

  const flow = (look.direction * Math.PI) / 180;
  return {
    stroke(dt, path, { width, height }) {
      if (!path && alive <= 0) return false;
      const w = Math.max(8, Math.round(width / 8));
      const h = Math.max(8, Math.round(height / 8));
      if (w !== tw || h !== th) {
        tw = w;
        th = h;
        for (const t of trails) {
          gl.bindTexture(gl.TEXTURE_2D, t);
          gl.texImage2D(gl.TEXTURE_2D, 0, gl.RGBA8, w, h, 0, gl.RGBA, gl.UNSIGNED_BYTE, null);
        }
        clearTrails();
      }
      // A stroke keeps it going for eight seconds after; then it is all but gone.
      alive = path ? 8 : alive - dt;
      if (alive <= 0) {
        clearTrails();
        return false;
      }
      const aspect = width / height;
      const from = path?.from ?? [0, 0];
      const to = path?.to ?? [0, 0];
      // The drag: the stroke's length (stage heights), capped, along its way.
      let mx = (to[0] - from[0]) * aspect * DRAG;
      let my = (to[1] - from[1]) * DRAG;
      const m = Math.hypot(mx, my);
      if (m > 0.6) (mx *= 0.6 / m), (my *= 0.6 / m);
      gl.bindFramebuffer(gl.FRAMEBUFFER, framebuffer);
      gl.framebufferTexture2D(gl.FRAMEBUFFER, gl.COLOR_ATTACHMENT0, gl.TEXTURE_2D, trails[1 - current], 0);
      gl.viewport(0, 0, tw, th);
      gl.useProgram(trail);
      gl.activeTexture(gl.TEXTURE0);
      gl.bindTexture(gl.TEXTURE_2D, trails[current]);
      gl.uniform1i(tu.uPrev, 0);
      gl.uniform2f(tu.uTexel, 1 / tw, 1 / th);
      gl.uniform1f(tu.uKeep, Math.exp(-dt / TRAIL_KEEP));
      gl.uniform1f(tu.uAspect, aspect);
      gl.uniform2f(tu.uFrom, from[0], from[1]);
      gl.uniform2f(tu.uTo, to[0], to[1]);
      gl.uniform2f(tu.uMove, path ? mx : 0, path ? my : 0);
      gl.uniform1f(tu.uRadius, BRUSH);
      gl.drawArrays(gl.TRIANGLES, 0, 3);
      current = 1 - current;
      return true;
    },
    setColors(colors) {
      gl.bindTexture(gl.TEXTURE_2D, rampTexture);
      gl.texSubImage2D(gl.TEXTURE_2D, 0, 0, 0, 256, 1, gl.RGBA, gl.UNSIGNED_BYTE, ramp(colors));
    },
    draw({ time, width, height, point, fieldWidth, fieldHeight }) {
      fit(fieldWidth, fieldHeight);
      gl.bindFramebuffer(gl.FRAMEBUFFER, framebuffer);
      gl.framebufferTexture2D(gl.FRAMEBUFFER, gl.COLOR_ATTACHMENT0, gl.TEXTURE_2D, fieldTexture, 0);
      gl.viewport(0, 0, fw, fh);
      gl.useProgram(field);
      gl.activeTexture(gl.TEXTURE0);
      gl.bindTexture(gl.TEXTURE_2D, alive > 0 ? trails[current] : rest);
      gl.uniform1i(fu.uTrail, 0);
      gl.uniform1f(fu.uPush, 1.6);
      gl.uniform1f(fu.uAspect, width / height);
      gl.uniform1f(fu.uScale, look.scale);
      gl.uniform1f(fu.uWarp, look.warp);
      gl.uniform1f(fu.uTime, time);
      // The flow, y up (the preset's angle is on a screen, y down).
      gl.uniform2f(fu.uFlow, Math.cos(flow), Math.sin(flow));
      gl.uniform2f(fu.uShift, look.shift[0] * (width / height), -look.shift[1]);
      gl.uniform1f(fu.uReach, look.intensity * 1.1);
      gl.drawArrays(gl.TRIANGLES, 0, 3);
      gl.bindTexture(gl.TEXTURE_2D, fieldTexture);
      gl.generateMipmap(gl.TEXTURE_2D);

      gl.bindFramebuffer(gl.FRAMEBUFFER, null);
      gl.viewport(0, 0, width, height);
      gl.useProgram(picture);
      gl.activeTexture(gl.TEXTURE1);
      gl.bindTexture(gl.TEXTURE_2D, rampTexture);
      gl.activeTexture(gl.TEXTURE0);
      gl.bindTexture(gl.TEXTURE_2D, fieldTexture);
      gl.uniform1i(pu.uField, 0);
      gl.uniform1i(pu.uRamp, 1);
      gl.uniform2f(pu.uFieldRes, fw, fh);
      gl.uniform2f(pu.uRes, width, height);
      gl.uniform1f(pu.uDensity, look.density);
      gl.uniform1f(pu.uRim, look.rim);
      gl.uniform2f(pu.uLight, Math.cos(look.light), Math.sin(look.light));
      gl.uniform1f(pu.uFilaments, look.filaments);
      gl.uniform1f(pu.uStars, look.stars);
      gl.uniform1f(pu.uGlow, look.glow);
      gl.uniform1f(pu.uNoise, look.noise.amount);
      // The noise's cell: a 290th of the width, as getgradients' scale 90 shows on its canvas.
      gl.uniform1f(pu.uNoiseCell, Math.max(1.5, width / 290));
      gl.uniform1f(pu.uGrain, look.grain.amount);
      gl.uniform1f(pu.uPoint, point);
      gl.uniform1f(pu.uDebug, (look as { debug?: number }).debug ?? 0);
      gl.drawArrays(gl.TRIANGLES, 0, 3);
    },
  };
}
