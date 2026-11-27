// The stage's flowing backdrop behind the hero, in WebGL2: a silk of the
// preset's hue under a pane of frosted glass. Two small passes:
//
// - the silk: domain-warped noise in three tones over a deep ink, soft and
//   low in detail, drawn small (half the stage's size, at most 1.5 times the
//   screen's density);
// - the glass, at the screen's own pixels (up to 2 per point): the silk
//   stretched and softened, a faint white sheen from the top, a diffuse
//   light, and a still grain on each pixel, a little stronger in the darks,
//   that the stretching never blurs. The pane's edge (a lit inner line and a
//   shadow) is the stylesheet's, so the CSS gradient without WebGL2 has it
//   too.
//
// - At rest it drifts, almost still; with the pointer over the stage it
//   slowly comes alive (the flow and the warp ease up) and its light leans
//   to the pointer; it eases back as the pointer leaves.
// - Its colors are the stage's `--silk-*` (`theme-css.ts`), read again when
//   the page's theme changes, and crossfaded.
// - It draws only while the stage is on screen and the tab is shown; with
//   less motion it draws one still frame; without WebGL2 it draws nothing
//   and the stage keeps its CSS gradient.
//
// The demo's frame lies over the stage, so the pointer is read from the page
// and from the frame's window (the same origin), and handed on (`onPointer`)
// for the demo's own depth.

const VERTEX = `#version 300 es
in vec2 a;
void main() { gl_Position = vec4(a, 0.0, 1.0); }`;

const SILK = `#version 300 es
precision highp float;
uniform vec2 uRes;
uniform float uTime;
uniform vec2 uPointer;
uniform float uHover;
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

  // The light leans to the pointer: the field swells toward it.
  vec2 d = p - m;
  float near = exp(-dot(d, d) * 7.0) * uHover;
  p -= d * near * 0.3;

  float t = uTime;
  float warp = 1.7 + uHover * 0.5;
  vec2 q = vec2(fbm(p * 0.8 + vec2(0.0, t * 0.6)), fbm(p * 0.8 + vec2(5.2, 1.3) - t * 0.45));
  vec2 r = vec2(fbm(p + warp * q + vec2(1.7, 9.2) + t * 0.3), fbm(p + warp * q + vec2(8.3, 2.8) - t * 0.25));
  float f = fbm(p + warp * r);

  // Three tones over the ink; depth from light and dark more than from color.
  vec3 c = mix(uInk, uLow, smoothstep(0.2, 0.8, f));
  c = mix(c, uHigh, smoothstep(0.42, 0.95, f * 0.55 + r.x * 0.65) * 0.9);
  // The silk: broad, soft bands bent by the field, lit where they crest.
  float band = 0.5 + 0.5 * sin((p.x * 0.8 + p.y * 0.5) * 3.6 + f * 5.5 + r.x * 1.6 + t * 1.4);
  c += uSheen * smoothstep(0.55, 1.0, band) * 0.12 * (0.4 + f);
  c -= c * smoothstep(0.4, 0.0, band) * 0.16;
  c += mix(uHigh, uSheen, 0.5) * near * 0.22;

  // Quieter toward the middle and the floor, where the sheet and the controls are.
  float centre = smoothstep(0.0, 0.55, length((uv - vec2(0.5, 0.48)) * vec2(1.0, 1.25)));
  c = mix(c, uInk, (1.0 - centre) * 0.25);
  c = mix(uInk, c, 0.6 + 0.4 * smoothstep(0.0, 0.65, uv.y));
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

function rgb(hex: string): Rgb {
  const h = hex.trim().replace('#', '');
  const n = (i: number) => parseInt(h.slice(i, i + 2), 16) / 255;
  return h.length >= 6 ? [n(0), n(2), n(4)] : [0, 0, 0];
}

export interface Backdrop {
  /** The frames drawn so far and the size they are drawn at. */
  stats(): { frames: number; width: number; height: number };
  /** Draws `n` frames one after another, each waited for (a pixel read back), and gives the average time of one in ms. */
  bench(n: number): { ms: number; width: number; height: number };
}

/**
 * Starts the backdrop in `stage` (a `canvas.stage__silk` in it); null without
 * WebGL2. `onPointer` gets the pointer over the stage, each axis -1 to 1, or
 * null as it leaves.
 */
export function startBackdrop(stage: HTMLElement, onPointer?: (p: { x: number; y: number } | null) => void): Backdrop | null {
  const canvas = stage.querySelector<HTMLCanvasElement>('canvas.stage__silk');
  const gl = canvas?.getContext('webgl2', { antialias: false, alpha: false, depth: false, stencil: false, powerPreference: 'low-power' });
  if (!canvas || !gl) return null;

  const shader = (type: number, source: string) => {
    const sh = gl.createShader(type)!;
    gl.shaderSource(sh, source);
    gl.compileShader(sh);
    return sh;
  };
  const link = (fragment: string) => {
    const pr = gl.createProgram()!;
    gl.attachShader(pr, shader(gl.VERTEX_SHADER, VERTEX));
    gl.attachShader(pr, shader(gl.FRAGMENT_SHADER, fragment));
    gl.bindAttribLocation(pr, 0, 'a');
    gl.linkProgram(pr);
    return gl.getProgramParameter(pr, gl.LINK_STATUS) ? pr : null;
  };
  const silk = link(SILK);
  const glass = link(GLASS);
  if (!silk || !glass) return null;
  gl.bindBuffer(gl.ARRAY_BUFFER, gl.createBuffer());
  gl.bufferData(gl.ARRAY_BUFFER, new Float32Array([-1, -1, 3, -1, -1, 3]), gl.STATIC_DRAW);
  gl.enableVertexAttribArray(0);
  gl.vertexAttribPointer(0, 2, gl.FLOAT, false, 0, 0);
  const u = (pr: WebGLProgram, name: string) => gl.getUniformLocation(pr, name);
  const at = { res: u(silk, 'uRes'), time: u(silk, 'uTime'), pointer: u(silk, 'uPointer'), hover: u(silk, 'uHover') };
  const colorAt = Object.fromEntries(NAMES.map((n) => [n, u(silk, `u${n[0].toUpperCase()}${n.slice(1)}`)])) as Record<(typeof NAMES)[number], WebGLUniformLocation | null>;
  const glassAt = { silk: u(glass, 'uSilk'), res: u(glass, 'uRes'), silkRes: u(glass, 'uSilkRes'), aspect: u(glass, 'uAspect') };

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

  // The silk's size, and the canvas's (the screen's pixels, at most 2 per point).
  let width = 0;
  let height = 0;
  let silkWidth = 0;
  let silkHeight = 0;
  const size = () => {
    const r = stage.getBoundingClientRect();
    const dpr = devicePixelRatio || 1;
    const sw = Math.max(1, Math.round(r.width * Math.min(dpr, 1.5) * 0.5));
    const sh = Math.max(1, Math.round(r.height * Math.min(dpr, 1.5) * 0.5));
    width = Math.max(1, Math.round(r.width * Math.min(dpr, 2)));
    height = Math.max(1, Math.round(r.height * Math.min(dpr, 2)));
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
    gl.bindFramebuffer(gl.FRAMEBUFFER, framebuffer);
    gl.viewport(0, 0, silkWidth, silkHeight);
    gl.useProgram(silk);
    gl.uniform2f(at.res, silkWidth, silkHeight);
    gl.uniform1f(at.time, time);
    gl.uniform2f(at.pointer, pointer.x, pointer.y);
    gl.uniform1f(at.hover, hover);
    colors(now).forEach((c, i) => gl.uniform3f(colorAt[NAMES[i]], c[0], c[1], c[2]));
    gl.drawArrays(gl.TRIANGLES, 0, 3);

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

  let visible = true;
  let raf = 0;
  let last = 0;
  let drawn = 0;
  const still = () => reduced.matches;
  const tick = (now: number) => {
    raf = 0;
    const dt = Math.min(0.05, last ? (now - last) / 1000 : 0);
    last = now;
    hover += ((over ? 1 : 0) - hover) * (1 - Math.exp(-dt / 0.45));
    pointer.x += (aim.x - pointer.x) * (1 - Math.exp(-dt / 0.35));
    pointer.y += (aim.y - pointer.y) * (1 - Math.exp(-dt / 0.35));
    time += dt * (0.016 + hover * 0.07);
    // At rest the drift is slow enough for a frame in three.
    if (hover > 0.01 || now - drawn > 45 || now - mixStart < MIX) {
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
    schedule();
  };
  const leave = () => {
    if (!over) return;
    over = false;
    onPointer?.(null);
    schedule();
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
    frame.addEventListener('load', hook);
  };
  stage.querySelectorAll('iframe').forEach((f) => attach(f as HTMLIFrameElement));
  new MutationObserver((records) => {
    for (const r of records) r.addedNodes.forEach((n) => n instanceof HTMLIFrameElement && attach(n));
  }).observe(stage, { childList: true, subtree: true });

  const pixel = new Uint8Array(4);
  return {
    stats: () => ({ frames, width, height }),
    bench: (n) => {
      const start = performance.now();
      for (let i = 0; i < n; i++) {
        time += 0.01;
        draw(performance.now());
        gl.readPixels(0, 0, 1, 1, gl.RGBA, gl.UNSIGNED_BYTE, pixel);
      }
      return { ms: (performance.now() - start) / n, width, height };
    },
  };
}
