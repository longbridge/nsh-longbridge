// The stage backdrop's liquid (`backdrop.ts`): a small fluid on the GPU, the
// classic stable-fluids loop. A velocity field and a dye, both small (the
// velocity about 160 cells on the long side, the dye about 256), moved each
// frame by semi-Lagrangian advection, with vorticity confinement to keep the
// swirls and a pressure solve (Jacobi) to keep the flow incompressible.
//
// The dye is not a color: its three channels are how much of each of the
// preset's tones (low, high, sheen) lies there, over the ink. The colors are
// applied as it is drawn, so a change of preset crossfades the whole liquid,
// and the surface is lit from a normal taken from the dye's slope: a slow,
// thick sheen, not flat color.
//
// Its fields are half floats, which it renders to; without that (no
// `EXT_color_buffer_float` nor `EXT_color_buffer_half_float`, or a format that
// will not attach) it is not made and the backdrop keeps its silk.

const VERTEX = `#version 300 es
in vec2 a;
out vec2 vUv;
void main() { vUv = a * 0.5 + 0.5; gl_Position = vec4(a, 0.0, 1.0); }`;

const HEAD = `#version 300 es
precision highp float;
precision highp sampler2D;
in vec2 vUv;
out vec4 color;
`;

const NOISE = `
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
float fbm(vec2 p) {
  float v = 0.0, amp = 0.55;
  mat2 turn = mat2(1.6, 1.2, -1.2, 1.6);
  for (int i = 0; i < 3; i++) { v += amp * noise(p); p = turn * p; amp *= 0.4; }
  return v / 0.844;
}`;

/** The first state: the dye as broad, warped folds of the three tones; the velocity a gentle curl noise. */
const SEED = `${HEAD}
uniform float uAspect;
uniform float uMode;
${NOISE}
void main() {
  vec2 p = vec2(vUv.x * uAspect, vUv.y) * 0.7 + vec2(3.1, 7.3);
  if (uMode < 0.5) {
    vec2 q = vec2(fbm(p * 0.8), fbm(p * 0.8 + vec2(5.2, 1.3)));
    vec2 r = vec2(fbm(p + 1.7 * q + vec2(1.7, 9.2)), fbm(p + 1.7 * q + vec2(8.3, 2.8)));
    float f = fbm(p + 1.7 * r);
    color = vec4(smoothstep(0.25, 0.8, f), smoothstep(0.45, 0.95, f * 0.55 + r.x * 0.65), smoothstep(0.55, 0.9, r.y) * 0.6, 1.0);
  } else {
    // The curl of a noise: a flow without sources or sinks, in cells per second.
    vec2 s = p * 1.4;
    float e = 0.02;
    float dy = fbm(s + vec2(0.0, e)) - fbm(s - vec2(0.0, e));
    float dx = fbm(s + vec2(e, 0.0)) - fbm(s - vec2(e, 0.0));
    color = vec4(vec2(dy, -dx) / (2.0 * e) * 2.5, 0.0, 1.0);
  }
}`;

const ADVECT = `${HEAD}
uniform sampler2D uVelocity;
uniform sampler2D uSource;
uniform vec2 uTexel;
uniform float uDt;
uniform float uDissipation;
void main() {
  vec2 coord = vUv - uDt * texture(uVelocity, vUv).xy * uTexel;
  color = texture(uSource, coord) / (1.0 + uDissipation * uDt);
}`;

const CURL = `${HEAD}
uniform sampler2D uVelocity;
uniform vec2 uTexel;
void main() {
  float l = texture(uVelocity, vUv - vec2(uTexel.x, 0.0)).y;
  float r = texture(uVelocity, vUv + vec2(uTexel.x, 0.0)).y;
  float t = texture(uVelocity, vUv + vec2(0.0, uTexel.y)).x;
  float b = texture(uVelocity, vUv - vec2(0.0, uTexel.y)).x;
  color = vec4(0.5 * (r - l - t + b), 0.0, 0.0, 1.0);
}`;

const VORTICITY = `${HEAD}
uniform sampler2D uVelocity;
uniform sampler2D uCurl;
uniform vec2 uTexel;
uniform float uCurlAmount;
uniform float uDt;
void main() {
  float l = texture(uCurl, vUv - vec2(uTexel.x, 0.0)).x;
  float r = texture(uCurl, vUv + vec2(uTexel.x, 0.0)).x;
  float t = texture(uCurl, vUv + vec2(0.0, uTexel.y)).x;
  float b = texture(uCurl, vUv - vec2(0.0, uTexel.y)).x;
  float c = texture(uCurl, vUv).x;
  vec2 force = 0.5 * vec2(abs(t) - abs(b), abs(r) - abs(l));
  force /= length(force) + 0.0001;
  force *= uCurlAmount * c;
  force.y *= -1.0;
  vec2 v = texture(uVelocity, vUv).xy + force * uDt;
  color = vec4(clamp(v, -1000.0, 1000.0), 0.0, 1.0);
}`;

const DIVERGENCE = `${HEAD}
uniform sampler2D uVelocity;
uniform vec2 uTexel;
void main() {
  vec2 c = texture(uVelocity, vUv).xy;
  float l = texture(uVelocity, vUv - vec2(uTexel.x, 0.0)).x;
  float r = texture(uVelocity, vUv + vec2(uTexel.x, 0.0)).x;
  float t = texture(uVelocity, vUv + vec2(0.0, uTexel.y)).y;
  float b = texture(uVelocity, vUv - vec2(0.0, uTexel.y)).y;
  // The walls turn the flow back.
  if (vUv.x - uTexel.x < 0.0) l = -c.x;
  if (vUv.x + uTexel.x > 1.0) r = -c.x;
  if (vUv.y + uTexel.y > 1.0) t = -c.y;
  if (vUv.y - uTexel.y < 0.0) b = -c.y;
  color = vec4(0.5 * (r - l + t - b), 0.0, 0.0, 1.0);
}`;

/** A thick liquid's viscosity, cheaply: the velocity eased toward its neighbours' mean. */
const VISCOSITY = `${HEAD}
uniform sampler2D uVelocity;
uniform vec2 uTexel;
uniform float uAmount;
void main() {
  vec2 c = texture(uVelocity, vUv).xy;
  vec2 m = 0.25 * (texture(uVelocity, vUv - vec2(uTexel.x, 0.0)).xy + texture(uVelocity, vUv + vec2(uTexel.x, 0.0)).xy
    + texture(uVelocity, vUv + vec2(0.0, uTexel.y)).xy + texture(uVelocity, vUv - vec2(0.0, uTexel.y)).xy);
  color = vec4(mix(c, m, uAmount), 0.0, 1.0);
}`;

const SCALE = `${HEAD}
uniform sampler2D uSource;
uniform float uValue;
void main() { color = uValue * texture(uSource, vUv); }`;

const PRESSURE = `${HEAD}
uniform sampler2D uPressure;
uniform sampler2D uDivergence;
uniform vec2 uTexel;
void main() {
  float l = texture(uPressure, vUv - vec2(uTexel.x, 0.0)).x;
  float r = texture(uPressure, vUv + vec2(uTexel.x, 0.0)).x;
  float t = texture(uPressure, vUv + vec2(0.0, uTexel.y)).x;
  float b = texture(uPressure, vUv - vec2(0.0, uTexel.y)).x;
  color = vec4(0.25 * (l + r + b + t - texture(uDivergence, vUv).x), 0.0, 0.0, 1.0);
}`;

const GRADIENT = `${HEAD}
uniform sampler2D uPressure;
uniform sampler2D uVelocity;
uniform vec2 uTexel;
void main() {
  float l = texture(uPressure, vUv - vec2(uTexel.x, 0.0)).x;
  float r = texture(uPressure, vUv + vec2(uTexel.x, 0.0)).x;
  float t = texture(uPressure, vUv + vec2(0.0, uTexel.y)).x;
  float b = texture(uPressure, vUv - vec2(0.0, uTexel.y)).x;
  color = vec4(texture(uVelocity, vUv).xy - vec2(r - l, t - b), 0.0, 1.0);
}`;

/**
 * Up to eight splats in one pass: added (the velocity) or mixed in (the dye,
 * so it never runs past 1), each a soft round blot of `uRadii` (a squared
 * distance, in the stage's height).
 */
const SPLAT = `${HEAD}
uniform sampler2D uTarget;
uniform float uAspect;
uniform float uMix;
uniform int uCount;
uniform vec2 uPoints[8];
uniform vec4 uValues[8];
uniform float uRadii[8];
uniform float uAmounts[8];
void main() {
  vec4 c = texture(uTarget, vUv);
  for (int i = 0; i < 8; i++) {
    if (i >= uCount) break;
    vec2 d = vUv - uPoints[i];
    d.x *= uAspect;
    float f = exp(-dot(d, d) / uRadii[i]);
    if (uMix > 0.5) c.rgb = mix(c.rgb, uValues[i].rgb, clamp(f * uAmounts[i], 0.0, 1.0));
    else c.xy += uValues[i].xy * f * uAmounts[i];
  }
  color = c;
}`;

/**
 * The liquid as it is seen: the tones over the ink by the dye, lit from the
 * upper left by a normal from the dye's slope (a soft diffuse light and a
 * narrow sheen where it folds), quieter toward the middle and the floor, where
 * the sheet and the controls are, with the pointer's soft light last.
 */
const SHADE = `${HEAD}
uniform sampler2D uDye;
uniform vec2 uDyeTexel;
uniform float uAspect;
uniform vec2 uPointer;
uniform float uHover;
uniform vec3 uInk;
uniform vec3 uLow;
uniform vec3 uHigh;
uniform vec3 uSheen;
uniform float uRelief;

float height(vec2 uv) {
  vec3 w = clamp(texture(uDye, uv).rgb, 0.0, 1.0);
  return dot(w, vec3(0.7, 1.0, 0.6));
}

void main() {
  vec2 uv = vUv;
  vec3 w = clamp(texture(uDye, uv).rgb, 0.0, 1.0);
  vec3 k = w * w * (3.0 - 2.0 * w);
  vec3 c = mix(uInk, uLow, k.r * 0.95);
  c = mix(c, uHigh, k.g * 0.85);
  c = mix(c, mix(uHigh, uSheen, 0.4), k.b * 0.5);

  // The surface: its slope over about three cells, the straight and the
  // diagonal pairs together (a Sobel), smooth enough to light.
  vec2 o = uDyeTexel * 2.5;
  float tl = height(uv + vec2(-o.x, o.y)), tr = height(uv + o), bl = height(uv - o), br = height(uv + vec2(o.x, -o.y));
  float hx = 0.25 * (2.0 * (height(uv + vec2(o.x, 0.0)) - height(uv - vec2(o.x, 0.0))) + tr - tl + br - bl);
  float hy = 0.25 * (2.0 * (height(uv + vec2(0.0, o.y)) - height(uv - vec2(0.0, o.y))) + tl - bl + tr - br);
  vec3 n = normalize(vec3(-hx * uRelief, -hy * uRelief, 1.0));
  vec3 light = normalize(vec3(-0.45, 0.6, 0.66));
  float diffuse = dot(n, light) - light.z;
  vec3 half_ = normalize(light + vec3(0.0, 0.0, 1.0));
  float facing = max(dot(n, half_), 0.0);
  float sheen = pow(facing, 60.0);
  float broad = pow(facing, 12.0);
  c *= clamp(1.0 + diffuse * 1.4, 0.45, 1.6);
  c += uSheen * (sheen * 0.28 + broad * 0.06) * (0.45 + 0.55 * max(k.g, k.b));

  // Quieter toward the middle and the floor (less while hovered).
  float centre = smoothstep(0.0, 0.55, length((uv - vec2(0.5, 0.48)) * vec2(1.0, 1.25)));
  c = mix(c, uInk, (1.0 - centre) * 0.22 * (1.0 - 0.6 * uHover));
  c = mix(uInk, c, mix(0.62 + 0.38 * smoothstep(0.0, 0.65, uv.y), 1.0, 0.6 * uHover));

  // The pointer's light: wide and soft, the liquid's motion does the rest.
  vec2 d = (uv - uPointer) * vec2(uAspect, 1.0) * 0.7;
  float r2 = dot(d, d);
  vec3 glow = mix(uHigh, uSheen, 0.35);
  c += glow * exp(-r2 * 8.0) * uHover * 0.22 + mix(glow, uSheen, 0.6) * exp(-r2 * 28.0) * uHover * 0.12;
  color = vec4(c, 1.0);
}`;

type Rgb = [number, number, number];

/** What the liquid is drawn with: the tones, the pointer (the stage's unit square, y up) and the hover (0 to 1). */
export interface Look {
  ink: Rgb;
  low: Rgb;
  high: Rgb;
  sheen: Rgb;
  pointer: { x: number; y: number };
  hover: number;
}

/** A splat: where (the stage's unit square, y up), how wide, and what it adds (velocity, in cells a second) or mixes in (dye, the tones' weights). */
export interface Splat {
  x: number;
  y: number;
  radius: number;
  value: [number, number, number];
  amount: number;
}

export interface Liquid {
  /** The fields' sizes. */
  size(): { sim: [number, number]; dye: [number, number] };
  /** Fits the fields to the stage's shape (width over height), seeding them anew if their size changes. */
  fit(aspect: number): void;
  /** Moves the liquid on by `dt` seconds, after the splats given (velocity and dye). */
  step(dt: number, velocity: Splat[], dye: Splat[]): void;
  /** Draws it into `target` (`width` by `height`). */
  draw(target: WebGLFramebuffer | null, width: number, height: number, look: Look): void;
}

/** The liquid's feel: thick and slow, its color lingering and folding. */
const SIM = 160;
const DYE = 256;
const VELOCITY_DISSIPATION = 0.35;
const DYE_DISSIPATION = 0.018;
const PRESSURE_KEEP = 0.8;
const PRESSURE_ITERATIONS = 18;
const CURL_AMOUNT = 2.5;
const VISCOSITY_AMOUNT = 0.35;
/** How steep the dye's surface reads (its slope's scale, for the light). */
const RELIEF = 9;

interface Target {
  tex: WebGLTexture;
  fb: WebGLFramebuffer;
  w: number;
  h: number;
}
interface Pair {
  read: Target;
  write: Target;
  swap(): void;
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

/** The liquid in `gl` (its full-screen triangle bound to attribute 0), or null where it cannot render to half floats. */
export async function makeLiquid(gl: WebGL2RenderingContext): Promise<Liquid | null> {
  if (!gl.getExtension('EXT_color_buffer_float') && !gl.getExtension('EXT_color_buffer_half_float')) return null;

  type Format = { internal: number; format: number };
  const formats: Record<'rgba' | 'rg' | 'r', Format> = {
    rgba: { internal: gl.RGBA16F, format: gl.RGBA },
    rg: { internal: gl.RG16F, format: gl.RG },
    r: { internal: gl.R16F, format: gl.RED },
  };
  const target = (w: number, h: number, f: { internal: number; format: number }, filter: number): Target | null => {
    const tex = gl.createTexture()!;
    gl.bindTexture(gl.TEXTURE_2D, tex);
    gl.texParameteri(gl.TEXTURE_2D, gl.TEXTURE_MIN_FILTER, filter);
    gl.texParameteri(gl.TEXTURE_2D, gl.TEXTURE_MAG_FILTER, filter);
    gl.texParameteri(gl.TEXTURE_2D, gl.TEXTURE_WRAP_S, gl.CLAMP_TO_EDGE);
    gl.texParameteri(gl.TEXTURE_2D, gl.TEXTURE_WRAP_T, gl.CLAMP_TO_EDGE);
    gl.texImage2D(gl.TEXTURE_2D, 0, f.internal, w, h, 0, f.format, gl.HALF_FLOAT, null);
    const fb = gl.createFramebuffer()!;
    gl.bindFramebuffer(gl.FRAMEBUFFER, fb);
    gl.framebufferTexture2D(gl.FRAMEBUFFER, gl.COLOR_ATTACHMENT0, gl.TEXTURE_2D, tex, 0);
    const ok = gl.checkFramebufferStatus(gl.FRAMEBUFFER) === gl.FRAMEBUFFER_COMPLETE;
    gl.bindFramebuffer(gl.FRAMEBUFFER, null);
    if (!ok) {
      gl.deleteTexture(tex);
      gl.deleteFramebuffer(fb);
      return null;
    }
    return { tex, fb, w, h };
  };
  // One- and two-channel fields fall back to four channels where those will not attach.
  const probe = (f: { internal: number; format: number }) => {
    const t = target(4, 4, f, gl.LINEAR);
    if (t) {
      gl.deleteTexture(t.tex);
      gl.deleteFramebuffer(t.fb);
    }
    return !!t;
  };
  if (!probe(formats.rgba)) return null;
  if (!probe(formats.rg)) formats.rg = formats.rgba;
  if (!probe(formats.r)) formats.r = formats.rgba;

  const sources = { seed: SEED, advect: ADVECT, curl: CURL, vorticity: VORTICITY, viscosity: VISCOSITY, divergence: DIVERGENCE, scale: SCALE, pressure: PRESSURE, gradient: GRADIENT, splat: SPLAT, shade: SHADE };
  const names = Object.keys(sources) as (keyof typeof sources)[];
  const linked = await linkPrograms(gl, names.map((n) => [VERTEX, sources[n]]));
  if (linked.some((pr) => !pr)) return null;
  type Program = { pr: WebGLProgram; u: (name: string) => WebGLUniformLocation | null };
  const p = Object.fromEntries(
    names.map((n, i) => {
      const pr = linked[i]!;
      const cache = new Map<string, WebGLUniformLocation | null>();
      const u = (name: string) => {
        if (!cache.has(name)) cache.set(name, gl.getUniformLocation(pr, name));
        return cache.get(name)!;
      };
      return [n, { pr, u }];
    }),
  ) as Record<keyof typeof sources, Program>;

  const pair = (w: number, h: number, f: { internal: number; format: number }, filter: number): Pair => {
    const a = target(w, h, f, filter)!;
    const b = target(w, h, f, filter)!;
    const out: Pair = {
      read: a,
      write: b,
      swap() {
        [out.read, out.write] = [out.write, out.read];
      },
    };
    return out;
  };
  const free = (t: Target) => {
    gl.deleteTexture(t.tex);
    gl.deleteFramebuffer(t.fb);
  };

  let aspect = 0;
  let velocity: Pair | null = null;
  let dye: Pair | null = null;
  let pressure: Pair | null = null;
  let divergence: Target | null = null;
  let curl: Target | null = null;
  let simSize: [number, number] = [0, 0];
  let dyeSize: [number, number] = [0, 0];

  const grid = (long: number): [number, number] =>
    aspect >= 1 ? [long, Math.max(16, Math.round(long / aspect))] : [Math.max(16, Math.round(long * aspect)), long];

  /** Draws `prog` into `t` (null: the canvas, `w` by `h`). */
  const blit = (t: Target | null, w = t?.w ?? 0, h = t?.h ?? 0) => {
    gl.bindFramebuffer(gl.FRAMEBUFFER, t ? t.fb : null);
    gl.viewport(0, 0, w, h);
    gl.drawArrays(gl.TRIANGLES, 0, 3);
  };
  const use = (prog: Program) => {
    gl.useProgram(prog.pr);
    return prog;
  };
  const bind = (prog: Program, name: string, unit: number, t: Target) => {
    gl.activeTexture(gl.TEXTURE0 + unit);
    gl.bindTexture(gl.TEXTURE_2D, t.tex);
    gl.uniform1i(prog.u(name), unit);
  };

  const seed = () => {
    const s = use(p.seed);
    gl.uniform1f(s.u('uAspect'), aspect);
    gl.uniform1f(s.u('uMode'), 0);
    blit(dye!.read);
    gl.uniform1f(s.u('uMode'), 1);
    blit(velocity!.read);
    for (const t of [pressure!.read, pressure!.write]) {
      gl.bindFramebuffer(gl.FRAMEBUFFER, t.fb);
      gl.clearColor(0, 0, 0, 1);
      gl.clear(gl.COLOR_BUFFER_BIT);
    }
  };

  const fit = (next: number) => {
    aspect = next;
    const sim = grid(SIM);
    const dyeGrid = grid(DYE);
    if (velocity && sim[0] === simSize[0] && sim[1] === simSize[1] && dyeGrid[0] === dyeSize[0] && dyeGrid[1] === dyeSize[1]) return;
    for (const t of [velocity?.read, velocity?.write, dye?.read, dye?.write, pressure?.read, pressure?.write, divergence, curl]) if (t) free(t);
    simSize = sim;
    dyeSize = dyeGrid;
    velocity = pair(sim[0], sim[1], formats.rg, gl.LINEAR);
    dye = pair(dyeGrid[0], dyeGrid[1], formats.rgba, gl.LINEAR);
    pressure = pair(sim[0], sim[1], formats.r, gl.NEAREST);
    divergence = target(sim[0], sim[1], formats.r, gl.NEAREST);
    curl = target(sim[0], sim[1], formats.r, gl.NEAREST);
    seed();
  };

  const splat = (field: Pair, list: Splat[], mix: boolean) => {
    for (let i = 0; i < list.length; i += 8) {
      const batch = list.slice(i, i + 8);
      const s = use(p.splat);
      bind(s, 'uTarget', 0, field.read);
      gl.uniform1f(s.u('uAspect'), aspect);
      gl.uniform1f(s.u('uMix'), mix ? 1 : 0);
      gl.uniform1i(s.u('uCount'), batch.length);
      gl.uniform2fv(s.u('uPoints'), batch.flatMap((b) => [b.x, b.y]).concat(Array(16 - batch.length * 2).fill(0)));
      gl.uniform4fv(s.u('uValues'), batch.flatMap((b) => [...b.value, 0]).concat(Array(32 - batch.length * 4).fill(0)));
      gl.uniform1fv(s.u('uRadii'), batch.map((b) => b.radius).concat(Array(8 - batch.length).fill(1)));
      gl.uniform1fv(s.u('uAmounts'), batch.map((b) => b.amount).concat(Array(8 - batch.length).fill(0)));
      blit(field.write);
      field.swap();
    }
  };

  const step = (dt: number, velocitySplats: Splat[], dyeSplats: Splat[]) => {
    if (!velocity || !dye || !pressure || !divergence || !curl) return;
    gl.disable(gl.BLEND);
    if (velocitySplats.length) splat(velocity, velocitySplats, false);
    if (dyeSplats.length) splat(dye, dyeSplats, true);
    const texel = [1 / simSize[0], 1 / simSize[1]] as const;

    let s = use(p.curl);
    gl.uniform2f(s.u('uTexel'), ...texel);
    bind(s, 'uVelocity', 0, velocity.read);
    blit(curl);

    s = use(p.vorticity);
    gl.uniform2f(s.u('uTexel'), ...texel);
    bind(s, 'uVelocity', 0, velocity.read);
    bind(s, 'uCurl', 1, curl);
    gl.uniform1f(s.u('uCurlAmount'), CURL_AMOUNT);
    gl.uniform1f(s.u('uDt'), dt);
    blit(velocity.write);
    velocity.swap();

    s = use(p.viscosity);
    gl.uniform2f(s.u('uTexel'), ...texel);
    bind(s, 'uVelocity', 0, velocity.read);
    gl.uniform1f(s.u('uAmount'), VISCOSITY_AMOUNT);
    blit(velocity.write);
    velocity.swap();

    s = use(p.divergence);
    gl.uniform2f(s.u('uTexel'), ...texel);
    bind(s, 'uVelocity', 0, velocity.read);
    blit(divergence);

    // The last solve's pressure, a little faded, is where this one starts.
    s = use(p.scale);
    bind(s, 'uSource', 0, pressure.read);
    gl.uniform1f(s.u('uValue'), PRESSURE_KEEP);
    blit(pressure.write);
    pressure.swap();

    s = use(p.pressure);
    gl.uniform2f(s.u('uTexel'), ...texel);
    bind(s, 'uDivergence', 1, divergence);
    for (let i = 0; i < PRESSURE_ITERATIONS; i++) {
      bind(s, 'uPressure', 0, pressure.read);
      blit(pressure.write);
      pressure.swap();
    }

    s = use(p.gradient);
    gl.uniform2f(s.u('uTexel'), ...texel);
    bind(s, 'uPressure', 0, pressure.read);
    bind(s, 'uVelocity', 1, velocity.read);
    blit(velocity.write);
    velocity.swap();

    s = use(p.advect);
    gl.uniform2f(s.u('uTexel'), ...texel);
    gl.uniform1f(s.u('uDt'), dt);
    bind(s, 'uVelocity', 0, velocity.read);
    bind(s, 'uSource', 1, velocity.read);
    gl.uniform1f(s.u('uDissipation'), VELOCITY_DISSIPATION);
    blit(velocity.write);
    velocity.swap();

    bind(s, 'uVelocity', 0, velocity.read);
    bind(s, 'uSource', 1, dye.read);
    gl.uniform1f(s.u('uDissipation'), DYE_DISSIPATION);
    blit(dye.write);
    dye.swap();
  };

  const draw = (out: WebGLFramebuffer | null, width: number, height: number, look: Look) => {
    if (!dye) return;
    const s = use(p.shade);
    bind(s, 'uDye', 0, dye.read);
    gl.uniform2f(s.u('uDyeTexel'), 1 / dyeSize[0], 1 / dyeSize[1]);
    gl.uniform1f(s.u('uAspect'), aspect);
    gl.uniform2f(s.u('uPointer'), look.pointer.x, look.pointer.y);
    gl.uniform1f(s.u('uHover'), look.hover);
    gl.uniform3f(s.u('uInk'), ...look.ink);
    gl.uniform3f(s.u('uLow'), ...look.low);
    gl.uniform3f(s.u('uHigh'), ...look.high);
    gl.uniform3f(s.u('uSheen'), ...look.sheen);
    gl.uniform1f(s.u('uRelief'), RELIEF);
    gl.bindFramebuffer(gl.FRAMEBUFFER, out);
    gl.viewport(0, 0, width, height);
    gl.drawArrays(gl.TRIANGLES, 0, 3);
  };

  return { size: () => ({ sim: simSize, dye: dyeSize }), fit, step, draw };
}
