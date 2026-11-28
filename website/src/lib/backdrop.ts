// The stage's flowing backdrop behind the hero, in WebGL2: a liquid of the
// preset's hue under a pane of frosted glass. Two passes:
//
// - the liquid (`liquid.ts`): a small fluid simulation whose dye holds the
//   preset's tones over a deep ink, drawn as glossy liquids meeting at clean
//   edges, at the stage's size (at most 1.5 pixels a point). Where the GPU
//   cannot render to half floats, the silk instead (at half that size):
//   domain-warped noise in the same tones, soft and low in detail;
// - the glass, at the screen's own pixels (up to 2 per point): the silk
//   stretched and softened, a faint white sheen from the top, a diffuse
//   light, and a still grain on each pixel, a little stronger in the darks,
//   that the stretching never blurs. The pane's edge (a lit inner line and a
//   shadow) is the stylesheet's, so the CSS gradient without WebGL2 has it
//   too.
//
// - The liquid never stops: a few unseen currents drift on slow paths, each
//   pushing it along and leaving a little of one tone, so it churns and folds
//   gently. The pointer stirs it: its motion pushes the liquid along its path
//   and leaves a little dye, by its speed, and the liquid swirls on behind
//   it; a soft glow of the preset's light follows it. (The silk instead
//   drifts, almost still, and streams at five to six times that while
//   hovered, dragged and rippled by the pointer.)
// - Its colors are the stage's `--silk-*` (`theme-css.ts`), read again when
//   the page's theme changes, and crossfaded.
// - It draws only while the stage is on screen and the tab is shown (at rest
//   at most 30 times a second); with less motion it draws one still frame
//   and runs nothing; without WebGL2 it draws nothing and the stage keeps its
//   CSS gradient.
//
// The demo's frame lies over the stage, so the pointer is read from the page
// and from the frame's window (the same origin), and handed on (`onPointer`)
// for the demo's own depth.

import { linkPrograms, makeLiquid, type Splat } from './liquid';

const VERTEX = `#version 300 es
in vec2 a;
void main() { gl_Position = vec4(a, 0.0, 1.0); }`;

const SILK = `#version 300 es
precision highp float;
uniform vec2 uRes;
uniform float uTime;
uniform vec2 uPointer;
uniform float uHover;
uniform float uPhase;
uniform vec2 uDrag;
uniform vec3 uInk;
uniform vec3 uLow;
uniform vec3 uHigh;
uniform vec3 uSheen;
out vec4 color;

float hash(vec2 p) {
  p = fract(p * vec2(123.34, 456.21));
  p += dot(p, p + 45.32);
  return fract(p.x * p.y);
}
float noise(vec2 p) {
  vec2 i = floor(p), f = fract(p);
  vec2 u = f * f * (3.0 - 2.0 * f);
  return mix(mix(hash(i), hash(i + vec2(1.0, 0.0)), u.x), mix(hash(i + vec2(0.0, 1.0)), hash(i + vec2(1.0, 1.0)), u.x), u.y);
}
// Three octaves, the last one faint: broad forms, no fine detail.
float fbm(vec2 p) {
  float v = 0.0, amp = 0.55;
  mat2 turn = mat2(1.6, 1.2, -1.2, 1.6);
  for (int i = 0; i < 3; i++) { v += amp * noise(p); p = turn * p; amp *= 0.4; }
  return v / 0.844;
}

void main() {
  vec2 uv = gl_FragCoord.xy / uRes;
  float aspect = uRes.x / uRes.y;
  vec2 p = vec2(uv.x * aspect, uv.y) * 0.7;
  vec2 m = vec2(uPointer.x * aspect, uPointer.y) * 0.7;

  // Around the pointer the silk is drawn along with it (its motion, uDrag),
  // swells toward it and ripples a little.
  vec2 d = p - m;
  float r2 = dot(d, d);
  float near = exp(-r2 * 28.0) * uHover;
  float wide = exp(-r2 * 8.0) * uHover;
  p -= uDrag * exp(-r2 * 4.0) * 0.35;
  p -= d * wide * 0.18;
  p += d * sin(sqrt(r2) * 16.0 - uPhase * 2.5) * wide * 0.03;

  float t = uTime;
  float warp = 1.7 + uHover * 0.6;
  vec2 q = vec2(fbm(p * 0.8 + vec2(0.0, t * 0.6)), fbm(p * 0.8 + vec2(5.2, 1.3) - t * 0.45));
  vec2 r = vec2(fbm(p + warp * q + vec2(1.7, 9.2) + t * 0.3), fbm(p + warp * q + vec2(8.3, 2.8) - t * 0.25));
  float f = fbm(p + warp * r);

  // Three tones over the ink; depth from light and dark more than from color.
  vec3 c = mix(uInk, uLow, smoothstep(0.2, 0.8, f));
  c = mix(c, uHigh, smoothstep(0.42, 0.95, f * 0.55 + r.x * 0.65) * 0.9);
  // The silk: broad, soft bands bent by the field, lit where they crest; they
  // stream on their own clock (uPhase), quick while hovered.
  float band = 0.5 + 0.5 * sin((p.x * 0.8 + p.y * 0.5) * 3.6 + f * 5.5 + r.x * 1.6 + uPhase);
  c += uSheen * smoothstep(0.55, 1.0, band) * (0.12 + 0.08 * uHover) * (0.4 + f);
  c -= c * smoothstep(0.4, 0.0, band) * 0.16;

  // Quieter toward the middle and the floor, where the sheet and the controls
  // are; less so while hovered, so the light and the flow read there too.
  float centre = smoothstep(0.0, 0.55, length((uv - vec2(0.5, 0.48)) * vec2(1.0, 1.25)));
  c = mix(c, uInk, (1.0 - centre) * 0.25 * (1.0 - 0.75 * uHover));
  c = mix(uInk, c, mix(0.6 + 0.4 * smoothstep(0.0, 0.65, uv.y), 1.0, 0.7 * uHover));

  // The pointer's light, last: a wide glow of the high tone and a brighter
  // core of the sheen, wide enough to rim the sheet when the pointer is on it.
  vec3 glow = mix(uHigh, uSheen, 0.35);
  c += glow * wide * 0.42 + mix(glow, uSheen, 0.6) * near * 0.3;
  color = vec4(c, 1.0);
}`;

const GLASS = `#version 300 es
precision highp float;
uniform sampler2D uSilk;
uniform vec2 uRes;
uniform vec2 uSilkRes;
uniform float uAspect;
out vec4 color;

// A hash per pixel: white noise, summed twice for a film's softer spread.
float hash(vec2 p) {
  vec3 q = fract(vec3(p.xyx) * 0.1031);
  q += dot(q, q.yzx + 33.33);
  return fract((q.x + q.y) * q.z);
}

void main() {
  vec2 uv = gl_FragCoord.xy / uRes;
  // The silk, stretched and softened: four taps a texel apart.
  vec2 o = 0.8 / uSilkRes;
  vec3 c = 0.25 * (texture(uSilk, uv + vec2(o.x, o.y)).rgb + texture(uSilk, uv + vec2(-o.x, o.y)).rgb
    + texture(uSilk, uv + vec2(o.x, -o.y)).rgb + texture(uSilk, uv - o).rgb);

  // The pane: a faint white sheen from the top, a diffuse light toward the upper left.
  c += vec3(0.045) * smoothstep(0.3, 1.0, uv.y) + vec3(0.02) * smoothstep(0.9, 1.0, uv.y);
  vec2 l = (uv - vec2(0.28, 1.0)) * vec2(uAspect, 1.0);
  c += vec3(0.035) * exp(-dot(l, l) * 1.2);

  // The grain: still, on each pixel, about 4% (5 in the darks, 3 in the lights).
  float n = hash(gl_FragCoord.xy) + hash(gl_FragCoord.xy + 71.3) - 1.0;
  float luma = dot(c, vec3(0.2126, 0.7152, 0.0722));
  c += n * mix(0.05, 0.03, smoothstep(0.05, 0.45, luma));
  color = vec4(c, 1.0);
}`;

type Rgb = [number, number, number];
const NAMES = ['ink', 'low', 'high', 'sheen'] as const;

/**
 * The currents that keep the liquid moving at rest: unseen splats drifting on
 * slow Lissajous paths (`at` plus `reach` times the sine of `speed` radians a
 * second), each pushing the liquid its way and leaving a little of its tone
 * (the weights of low, high and sheen).
 */
const CURRENTS = [
  { at: [0.2, 0.62], reach: [0.14, 0.22], speed: [0.21, 0.16], phase: [0, 1.3], tone: [1, 0.1, 0] },
  { at: [0.8, 0.5], reach: [0.14, 0.24], speed: [0.15, 0.23], phase: [2.1, 0.4], tone: [0.15, 1, 0.1] },
  { at: [0.5, 0.22], reach: [0.34, 0.1], speed: [0.11, 0.19], phase: [4, 2], tone: [0.2, 0.35, 0.9] },
  { at: [0.45, 0.82], reach: [0.3, 0.1], speed: [0.17, 0.12], phase: [1, 3], tone: [0.75, 0.55, 0] },
] as const;
/** How hard a current pushes (cells a second, each second) and how fast it leaves its tone. */
const CURRENT_PUSH = 9;
const CURRENT_DYE = 0.5;
/** How much of the pointer's speed the liquid takes, and how much dye it leaves at a stage's width a second. */
const STIR = 0.4;
const STIR_DYE = 0.22;

function rgb(hex: string): Rgb {
  const h = hex.trim().replace('#', '');
  const n = (i: number) => parseInt(h.slice(i, i + 2), 16) / 255;
  return h.length >= 6 ? [n(0), n(2), n(4)] : [0, 0, 0];
}

export interface Backdrop {
  /** What draws it (the liquid, or the silk without half floats), the frames drawn so far and the size they are drawn at. */
  stats(): { mode: 'liquid' | 'silk'; frames: number; width: number; height: number; sim?: [number, number]; dye?: [number, number] };
  /** Whether the pointer is over the stage, and the hover's eased amount (0 to 1). */
  state(): { over: boolean; hover: number; pointer: { x: number; y: number }; aim: { x: number; y: number } };
  /** How much the backdrop moves: the mean absolute difference (0-255, per channel) of two frames `ms` apart (500), hovered or at rest. The liquid is moved on by it. */
  flow(hovered: boolean, at?: { x: number; y: number }, ms?: number): { meanAbsDiff: number; hover: boolean; width: number; height: number };
  /** Draws `n` frames one after another, each waited for (a pixel read back), and gives the average time of one in ms. */
  bench(n: number): { ms: number; width: number; height: number };
}

/**
 * Starts the backdrop in `stage` (a `canvas.stage__silk` in it); null without
 * WebGL2. It begins after the page's first paint and compiles its shaders in
 * the background where the driver can, so the page and its entrance never
 * wait for it; the stage shows its CSS gradient until it fades in. `onPointer`
 * gets the pointer over the stage, each axis -1 to 1, or null as it leaves.
 */
export async function startBackdrop(stage: HTMLElement, onPointer?: (p: { x: number; y: number } | null) => void): Promise<Backdrop | null> {
  await new Promise((r) => requestAnimationFrame(() => setTimeout(r, 0)));
  const canvas = stage.querySelector<HTMLCanvasElement>('canvas.stage__silk');
  const gl = canvas?.getContext('webgl2', { antialias: false, alpha: false, depth: false, stencil: false, powerPreference: 'low-power' });
  if (!canvas || !gl) return null;

  // The liquid's programs and the silk's and the glass's, compiled side by side.
  const [[silk, glass], fluid] = await Promise.all([linkPrograms(gl, [[VERTEX, SILK], [VERTEX, GLASS]]), makeLiquid(gl)]);
  if (!silk || !glass) return null;
  gl.bindBuffer(gl.ARRAY_BUFFER, gl.createBuffer());
  gl.bufferData(gl.ARRAY_BUFFER, new Float32Array([-1, -1, 3, -1, -1, 3]), gl.STATIC_DRAW);
  gl.enableVertexAttribArray(0);
  gl.vertexAttribPointer(0, 2, gl.FLOAT, false, 0, 0);
  const u = (pr: WebGLProgram, name: string) => gl.getUniformLocation(pr, name);
  const at = { res: u(silk, 'uRes'), time: u(silk, 'uTime'), pointer: u(silk, 'uPointer'), hover: u(silk, 'uHover'), phase: u(silk, 'uPhase'), drag: u(silk, 'uDrag') };
  const colorAt = Object.fromEntries(NAMES.map((n) => [n, u(silk, `u${n[0].toUpperCase()}${n.slice(1)}`)])) as Record<(typeof NAMES)[number], WebGLUniformLocation | null>;
  const glassAt = { silk: u(glass, 'uSilk'), res: u(glass, 'uRes'), silkRes: u(glass, 'uSilkRes'), aspect: u(glass, 'uAspect') };
  // The liquid where half floats render (`fluid`); the silk where not.

  // The silk is drawn into a small texture, which the glass pass stretches.
  const texture = gl.createTexture();
  gl.bindTexture(gl.TEXTURE_2D, texture);
  gl.texParameteri(gl.TEXTURE_2D, gl.TEXTURE_MIN_FILTER, gl.LINEAR);
  gl.texParameteri(gl.TEXTURE_2D, gl.TEXTURE_MAG_FILTER, gl.LINEAR);
  gl.texParameteri(gl.TEXTURE_2D, gl.TEXTURE_WRAP_S, gl.CLAMP_TO_EDGE);
  gl.texParameteri(gl.TEXTURE_2D, gl.TEXTURE_WRAP_T, gl.CLAMP_TO_EDGE);
  const framebuffer = gl.createFramebuffer();

  const reduced = matchMedia('(prefers-reduced-motion: reduce)');
  const read = () => {
    const css = getComputedStyle(stage);
    return NAMES.map((n) => rgb(css.getPropertyValue(`--silk-${n}`))) as Rgb[];
  };

  // The colors: shown, and the ones they move to.
  let from = read();
  let to = from;
  let mixStart = 0;
  const MIX = 500;
  const colors = (now: number): Rgb[] => {
    const k = Math.min(1, (now - mixStart) / MIX);
    const e = k * k * (3 - 2 * k);
    return from.map((c, i) => c.map((v, j) => v + (to[i][j] - v) * e) as Rgb);
  };

  // The pointer, in the stage's own unit square (y up), eased; the hover, eased from 0 to 1.
  let pointer = { x: 0.5, y: 0.6 };
  let aim = { ...pointer };
  let hover = 0;
  let over = false;
  let time = 7.3;
  // The bands' own clock, and the silk dragged along by the pointer's motion (in the field's units).
  let phase = 0;
  let drag = { x: 0, y: 0 };
  // The liquid's clock (the currents' paths), and where the pointer was at the last step (null: just in, or out).
  let clock = 0;
  let trail: { x: number; y: number } | null = null;

  // The silk's size, and the canvas's (the screen's pixels, at most 2 per point).
  let width = 0;
  let height = 0;
  let silkWidth = 0;
  let silkHeight = 0;
  const size = () => {
    const r = stage.getBoundingClientRect();
    const dpr = devicePixelRatio || 1;
    width = Math.max(1, Math.round(r.width * Math.min(dpr, 2)));
    height = Math.max(1, Math.round(r.height * Math.min(dpr, 2)));
    // At most 1.5 pixels a point: the liquid at the stage's full size (its
    // edges and highlights are fine lines), the silk at half.
    const scale = Math.min(dpr, 1.5) * (fluid ? 1 : 0.5);
    const sw = Math.max(1, Math.round(r.width * scale));
    const sh = Math.max(1, Math.round(r.height * scale));
    fluid?.fit(r.width / Math.max(1, r.height));
    if (canvas.width !== width || canvas.height !== height) {
      canvas.width = width;
      canvas.height = height;
    }
    if (sw !== silkWidth || sh !== silkHeight) {
      silkWidth = sw;
      silkHeight = sh;
      gl.bindTexture(gl.TEXTURE_2D, texture);
      gl.texImage2D(gl.TEXTURE_2D, 0, gl.RGBA8, sw, sh, 0, gl.RGBA, gl.UNSIGNED_BYTE, null);
      gl.bindFramebuffer(gl.FRAMEBUFFER, framebuffer);
      gl.framebufferTexture2D(gl.FRAMEBUFFER, gl.COLOR_ATTACHMENT0, gl.TEXTURE_2D, texture, 0);
      gl.bindFramebuffer(gl.FRAMEBUFFER, null);
    }
  };

  let frames = 0;
  const draw = (now: number) => {
    if (fluid) {
      const [ink, low, high, sheen] = colors(now);
      fluid.draw(framebuffer, silkWidth, silkHeight, { ink, low, high, sheen, pointer, hover });
    } else drawSilk(now);

    gl.bindFramebuffer(gl.FRAMEBUFFER, null);
    gl.viewport(0, 0, width, height);
    gl.useProgram(glass);
    gl.activeTexture(gl.TEXTURE0);
    gl.bindTexture(gl.TEXTURE_2D, texture);
    gl.uniform1i(glassAt.silk, 0);
    gl.uniform2f(glassAt.res, width, height);
    gl.uniform2f(glassAt.silkRes, silkWidth, silkHeight);
    gl.uniform1f(glassAt.aspect, width / height);
    gl.drawArrays(gl.TRIANGLES, 0, 3);
    frames++;
  };
  const drawSilk = (now: number) => {
    gl.bindFramebuffer(gl.FRAMEBUFFER, framebuffer);
    gl.viewport(0, 0, silkWidth, silkHeight);
    gl.useProgram(silk);
    gl.uniform2f(at.res, silkWidth, silkHeight);
    gl.uniform1f(at.time, time);
    gl.uniform2f(at.pointer, pointer.x, pointer.y);
    gl.uniform1f(at.hover, hover);
    gl.uniform1f(at.phase, phase);
    gl.uniform2f(at.drag, drag.x, drag.y);
    colors(now).forEach((c, i) => gl.uniform3f(colorAt[NAMES[i]], c[0], c[1], c[2]));
    gl.drawArrays(gl.TRIANGLES, 0, 3);
  };

  let visible = true;
  let raf = 0;
  let last = 0;
  let drawn = 0;
  const still = () => reduced.matches;
  /**
   * The motion over `dt` seconds. The hover eases in over about 400 ms and
   * out over about 800; the light follows the pointer in about 200 ms. At
   * rest the field and the bands barely drift; hovered they stream at five
   * to six times that.
   */
  const step = (dt: number) => {
    const target = over ? 1 : 0;
    hover += (target - hover) * (1 - Math.exp(-dt / (over ? 0.13 : 0.27)));
    const k = 1 - Math.exp(-dt / 0.07);
    const px = pointer.x;
    const py = pointer.y;
    pointer.x += (aim.x - pointer.x) * k;
    pointer.y += (aim.y - pointer.y) * k;
    // The pointer's speed drags the silk; the drag settles back in about 600 ms.
    const aspect = width / height || 1;
    drag.x = drag.x * Math.exp(-dt / 0.3) + (pointer.x - px) * aspect * 0.7 * hover;
    drag.y = drag.y * Math.exp(-dt / 0.3) + (pointer.y - py) * 0.7 * hover;
    time += dt * (0.02 + hover * 0.09);
    phase += dt * (0.25 + hover * 1.15);
    if (fluid && dt > 0) stir(dt);
  };
  /** The liquid moved on by `dt`: the currents, then the pointer's path since the last step. */
  const stir = (dt: number) => {
    clock += dt;
    const velocity: Splat[] = [];
    const dye: Splat[] = [];
    for (const c of CURRENTS) {
      const x = c.at[0] + c.reach[0] * Math.sin(c.speed[0] * clock + c.phase[0]);
      const y = c.at[1] + c.reach[1] * Math.sin(c.speed[1] * clock + c.phase[1]);
      // Its way: the path's direction, in cells.
      const vx = c.reach[0] * c.speed[0] * Math.cos(c.speed[0] * clock + c.phase[0]) * aspectOf();
      const vy = c.reach[1] * c.speed[1] * Math.cos(c.speed[1] * clock + c.phase[1]);
      const n = Math.hypot(vx, vy) || 1;
      velocity.push({ x, y, radius: 0.02, value: [(vx / n) * CURRENT_PUSH, (vy / n) * CURRENT_PUSH, 0], amount: dt });
      dye.push({ x, y, radius: 0.03, value: [...c.tone], amount: CURRENT_DYE * dt });
    }
    if (over && trail) {
      const dx = aim.x - trail.x;
      const dy = aim.y - trail.y;
      const across = Math.hypot(dx * aspectOf(), dy);
      if (across > 1e-4) {
        const { sim } = fluid!.size();
        // The pointer's speed in cells a second, taken in part and capped.
        let vx = (dx / dt) * sim[0] * STIR;
        let vy = (dy / dt) * sim[1] * STIR;
        const v = Math.hypot(vx, vy);
        if (v > 600) (vx *= 600 / v), (vy *= 600 / v);
        // Its speed in stage widths a second sets the dye it leaves.
        const ink = Math.min(0.4, (across / aspectOf() / dt) * STIR_DYE);
        // Along the path, a splat every 3% of the stage's height.
        const n = Math.min(6, Math.ceil(across / 0.03));
        for (let i = 1; i <= n; i++) {
          const x = trail.x + (dx * i) / n;
          const y = trail.y + (dy * i) / n;
          velocity.push({ x, y, radius: 0.01, value: [vx, vy, 0], amount: 1 / n });
          dye.push({ x, y, radius: 0.006, value: [0.15, 0.85, 0.75], amount: ink });
        }
      }
    }
    trail = over ? { ...aim } : null;
    fluid!.step(dt, velocity, dye);
  };
  const aspectOf = () => width / height || 1;
  const tick = (now: number) => {
    raf = 0;
    if (fluid) {
      // The liquid moves on each frame while the pointer is about, at rest at
      // most 30 times a second; a long gap (a busy page) counts as 50 ms.
      const gap = last ? (now - last) / 1000 : 1 / 60;
      if (!last || over || hover > 0.005 || now - mixStart < MIX || gap >= 0.03) {
        last = now;
        step(Math.min(0.05, gap));
        draw(now);
      }
      schedule();
      return;
    }
    // Frames may come slowly (a busy page): a long gap is taken as at most 100 ms.
    const dt = Math.min(0.1, last ? (now - last) / 1000 : 0);
    last = now;
    step(dt);
    // At rest the drift is slow enough for a frame in three.
    if (hover > 0.005 || Math.abs(drag.x) + Math.abs(drag.y) > 1e-4 || now - drawn > 45 || now - mixStart < MIX) {
      drawn = now;
      draw(now);
    }
    schedule();
  };
  const schedule = () => {
    if (raf || !visible || document.hidden) return;
    if (still()) {
      // One still frame, drawn again only for a change of colors or size.
      draw(performance.now() + MIX);
      return;
    }
    raf = requestAnimationFrame(tick);
  };
  const redraw = () => {
    if (still()) draw(performance.now() + MIX);
    else schedule();
  };

  size();
  draw(performance.now());
  stage.classList.add('is-silk');
  schedule();

  new ResizeObserver(() => {
    size();
    redraw();
  }).observe(stage);
  new IntersectionObserver((entries) => {
    visible = entries[entries.length - 1].isIntersecting;
    last = 0;
    schedule();
  }).observe(stage);
  document.addEventListener('visibilitychange', () => {
    last = 0;
    schedule();
  });
  reduced.addEventListener('change', redraw);
  // The theme changes on <html>: the colors cross to the new ones.
  new MutationObserver(() => {
    const next = read();
    if (next.every((c, i) => c.every((v, j) => Math.abs(v - to[i][j]) < 1e-3))) return;
    from = colors(performance.now());
    to = next;
    mixStart = performance.now();
    redraw();
  }).observe(document.documentElement, { attributes: true, attributeFilter: ['class', 'data-preset', 'data-appearance'] });

  // The pointer, from the page and from the demo's frame over the stage.
  let leaving = 0;
  const point = (x: number, y: number) => {
    const r = stage.getBoundingClientRect();
    const inside = x >= r.left && x <= r.right && y >= r.top && y <= r.bottom;
    clearTimeout(leaving);
    if (inside) {
      aim = { x: (x - r.left) / r.width, y: 1 - (y - r.top) / r.height };
      if (!over) last = 0;
      over = !still();
      onPointer?.({ x: aim.x * 2 - 1, y: 1 - aim.y * 2 });
    } else {
      leave();
    }
    // With less motion the still frame stands: nothing to draw for the pointer.
    if (!still()) schedule();
  };
  const leave = () => {
    if (!over) return;
    over = false;
    onPointer?.(null);
    if (!still()) schedule();
  };
  addEventListener('pointermove', (e) => point(e.clientX, e.clientY), { passive: true });
  document.documentElement.addEventListener('mouseleave', () => leave());
  const hooked = new WeakSet<Document>();
  const attach = (frame: HTMLIFrameElement) => {
    const hook = () => {
      try {
        const doc = frame.contentDocument;
        if (!doc || hooked.has(doc)) return;
        hooked.add(doc);
        doc.addEventListener(
          'pointermove',
          (e) => {
            const r = frame.getBoundingClientRect();
            point(r.left + e.clientX, r.top + e.clientY);
          },
          { passive: true, capture: true },
        );
        // Out of the frame: to the page (its own moves follow) or out of the window.
        doc.documentElement.addEventListener('mouseleave', () => {
          leaving = window.setTimeout(leave, 60);
        });
      } catch {}
    };
    // It may have loaded before the backdrop started (that waits for the first paint and its shaders).
    frame.addEventListener('load', hook);
    if (frame.contentDocument?.readyState === 'complete') hook();
  };
  stage.querySelectorAll('iframe').forEach((f) => attach(f as HTMLIFrameElement));
  new MutationObserver((records) => {
    for (const r of records) r.addedNodes.forEach((n) => n instanceof HTMLIFrameElement && attach(n));
  }).observe(stage, { childList: true, subtree: true });

  const pixel = new Uint8Array(4);
  return {
    stats: () => ({ mode: fluid ? 'liquid' : 'silk', frames, width, height, ...(fluid ? fluid.size() : {}) }),
    state: () => ({ over, hover, pointer: { ...pointer }, aim: { ...aim } }),
    flow: (hovered, at = { x: 0.3, y: 0.6 }, ms = 500) => {
      // Two frames `ms` apart on a steady 60 fps clock, after a second's
      // settling, with the pointer over the stage (still) or away from it.
      const saved = { over, hover, time, phase, pointer: { ...pointer }, aim: { ...aim }, drag: { ...drag } };
      over = hovered;
      aim = { ...at };
      pointer = { ...at };
      const read = () => {
        const out = new Uint8Array(width * height * 4);
        gl.readPixels(0, 0, width, height, gl.RGBA, gl.UNSIGNED_BYTE, out);
        return out;
      };
      for (let i = 0; i < 60; i++) step(1 / 60);
      draw(performance.now() + MIX);
      const a = read();
      for (let i = 0; i < Math.round(ms * 0.06); i++) step(1 / 60);
      draw(performance.now() + MIX);
      const b = read();
      let sum = 0;
      for (let i = 0; i < a.length; i += 4) sum += Math.abs(a[i] - b[i]) + Math.abs(a[i + 1] - b[i + 1]) + Math.abs(a[i + 2] - b[i + 2]);
      ({ over, hover, time, phase } = saved);
      pointer = saved.pointer;
      aim = saved.aim;
      drag = saved.drag;
      draw(performance.now() + MIX);
      return { meanAbsDiff: sum / ((a.length / 4) * 3), hover: hovered, width, height };
    },
    bench: (n) => {
      const start = performance.now();
      for (let i = 0; i < n; i++) {
        time += 0.01;
        if (fluid) stir(1 / 60);
        draw(performance.now());
        gl.readPixels(0, 0, 1, 1, gl.RGBA, gl.UNSIGNED_BYTE, pixel);
      }
      return { ms: (performance.now() - start) / n, width, height };
    },
  };
}
