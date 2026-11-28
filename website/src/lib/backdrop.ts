// The stage's backdrop behind the hero, in WebGL2: a nebula (`nebula.ts`)
// after the Nocturne gradient, clouds over a deep star field drifting slowly
// and continuously (its clock is the page's own, never stepped nor wrapped).
//
// - Its colors are the preset's (`--nebula-*`, `nebula-colors.ts`), read
//   again when the theme changes and crossfaded in Oklab over 500 ms. The
//   stage stays dark in the light appearance too.
// - Its field is drawn at 0.6 pixels a point (the look is soft), the picture
//   at the screen's pixels (up to 2 per point), so the grain stays fine.
// - A hand stirs it: the pointer moved over the stage, or a finger dragged on
//   it, pulls the clouds along its path, and they settle back as the trail
//   spreads and fades (in about three seconds). The hand's speed is eased
//   (about 80 ms) before it pushes, so the pull follows it without a jolt and
//   dies away when it stops. The demo's frame lies over
//   the stage, so the pointer and the touches are read from the page and from
//   the frame's window (the same origin); the pointer is also handed on
//   (`onPointer`) for the demo's own depth.
// - It draws only while the stage is on screen and the tab is shown, on every
//   frame the browser paints; with less motion it draws one still frame, runs
//   nothing and takes no strokes; without WebGL2 it draws nothing and the
//   stage keeps its CSS gradient (the stylesheet's likeness of the nebula).

import { makeNebula, NOCTURNE, oklab, type Rgb } from './nebula';

/** The field's pixels per CSS pixel, and the canvas's at most. */
const FIELD = 0.6;
const MAX_DPR = 2;
/** The still frame's moment (s), with less motion. */
const STILL_AT = 12;
/** How quickly the hand's eased speed follows it (s). */
const HAND_EASE = 0.08;
/** The colors' crossfade (ms). */
const MIX = 500;

export interface Backdrop {
  /** The frames drawn so far and the sizes they are drawn at (the canvas's and the field's, in pixels). */
  stats(): { frames: number; width: number; height: number; field: [number, number]; time: number };
  /** Whether the pointer is over the stage, and whether a trail is still settling. */
  state(): { over: boolean; trail: boolean };
  /** Stops the clock at `time` (s) and draws that moment; null starts it again from there. For comparisons. */
  seek(time: number | null): void;
  /** Draws `n` frames one after another, each waited for (a pixel read back), and gives the average time of one in ms. */
  bench(n: number): { ms: number; width: number; height: number; field: [number, number] };
}

/**
 * Starts the backdrop in `stage` (a `canvas.stage__nebula` in it); null
 * without WebGL2. It begins after the page's first paint and compiles its
 * shaders in the background where the driver can, so the page and its
 * entrance never wait for it; the stage shows its CSS gradient until it fades
 * in. `onPointer` gets the pointer over the stage, each axis -1 to 1, or null
 * as it leaves.
 */
export async function startBackdrop(stage: HTMLElement, onPointer?: (p: { x: number; y: number } | null) => void): Promise<Backdrop | null> {
  await new Promise((r) => requestAnimationFrame(() => setTimeout(r, 0)));
  const canvas = stage.querySelector<HTMLCanvasElement>('canvas.stage__nebula');
  const gl = canvas?.getContext('webgl2', { antialias: false, alpha: false, depth: false, stencil: false, powerPreference: 'low-power' });
  if (!canvas || !gl) return null;
  const nebula = await makeNebula(gl);
  if (!nebula) return null;
  gl.bindBuffer(gl.ARRAY_BUFFER, gl.createBuffer());
  gl.bufferData(gl.ARRAY_BUFFER, new Float32Array([-1, -1, 3, -1, -1, 3]), gl.STATIC_DRAW);
  gl.enableVertexAttribArray(0);
  gl.vertexAttribPointer(0, 2, gl.FLOAT, false, 0, 0);

  const reduced = matchMedia('(prefers-reduced-motion: reduce)');
  const still = () => reduced.matches;

  // The colors: shown, and the ones they move to (Oklab).
  const read = () => {
    const css = getComputedStyle(stage);
    return [0, 1, 2, 3, 4].map((i) => {
      const v = css.getPropertyValue(`--nebula-${i}`).trim();
      return oklab(/^#[0-9a-f]{6}$/i.test(v) ? v : NOCTURNE.stops[i]);
    });
  };
  let from = read();
  let to = from;
  let mixStart = -Infinity;
  const colors = (now: number): Rgb[] => {
    const k = Math.min(1, (now - mixStart) / MIX);
    const e = k * k * (3 - 2 * k);
    return from.map((c, i) => c.map((v, j) => v + (to[i][j] - v) * e) as Rgb);
  };
  nebula.setColors(to);

  // The canvas's size (the screen's pixels, at most 2 per point) and the field's.
  let width = 0;
  let height = 0;
  let point = 1;
  let field: [number, number] = [1, 1];
  const size = () => {
    const r = stage.getBoundingClientRect();
    point = Math.min(devicePixelRatio || 1, MAX_DPR);
    width = Math.max(1, Math.round(r.width * point));
    height = Math.max(1, Math.round(r.height * point));
    field = [Math.max(1, Math.round(r.width * FIELD)), Math.max(1, Math.round(r.height * FIELD))];
    if (canvas.width !== width || canvas.height !== height) {
      canvas.width = width;
      canvas.height = height;
    }
  };

  // The clock (s, the page's time while the stage is shown), and the hand:
  // where it is (the stage's unit square, y up), where it was at the last
  // frame, and its eased speed (units a second), which pushes.
  let time = 0;
  let frames = 0;
  let pinned: number | null = null;
  let over = false;
  let hand: [number, number] | null = null;
  let handWas: [number, number] | null = null;
  let push: [number, number] | null = null;
  let speed: [number, number] = [0, 0];
  let trailing = false;
  const draw = (now = performance.now()) => {
    if (now - mixStart < MIX + 50) nebula.setColors(colors(now));
    const t = pinned ?? (still() ? STILL_AT : time);
    nebula.draw({ time: t, width, height, point, fieldWidth: field[0], fieldHeight: field[1] });
    frames++;
  };
  /** The hand, `dt` on: its eased speed, and the stroke it makes (from where its push was to where it is now). */
  const stir = (dt: number) => {
    const raw: [number, number] = hand && handWas ? [(hand[0] - handWas[0]) / dt, (hand[1] - handWas[1]) / dt] : [0, 0];
    if (hand) push = hand;
    handWas = hand;
    const k = 1 - Math.exp(-dt / HAND_EASE);
    speed = [speed[0] + (raw[0] - speed[0]) * k, speed[1] + (raw[1] - speed[1]) * k];
    const moving = push && Math.hypot(speed[0], speed[1]) > 1e-3;
    const path = moving ? { from: [push![0] - speed[0] * dt, push![1] - speed[1] * dt] as [number, number], to: push! } : null;
    if (!moving && !hand) push = null;
    trailing = nebula.stroke(dt, path, { width, height });
  };

  let visible = true;
  let raf = 0;
  let last = 0;
  const tick = (now: number) => {
    raf = 0;
    // Every frame, on the frame's own time; a long gap (a hidden tab, a stall
    // over 250 ms) is skipped rather than played.
    const gap = last ? (now - last) / 1000 : 0;
    const dt = gap > 0.25 ? 1 / 60 : gap;
    last = now;
    if (pinned === null) time += dt;
    if (dt > 0) stir(dt);
    draw(now);
    schedule();
  };
  const schedule = () => {
    if (raf || !visible || document.hidden) return;
    if (still()) {
      // One still frame, drawn again for a change of colors or size.
      draw();
      if (performance.now() - mixStart < MIX) raf = requestAnimationFrame(() => ((raf = 0), schedule()));
      return;
    }
    // Pinned (a comparison), it waits for nothing but the trail and the colors.
    if (pinned !== null && !trailing && !hand && !push && performance.now() - mixStart > MIX) {
      draw();
      return;
    }
    raf = requestAnimationFrame(tick);
  };

  size();
  draw();
  stage.classList.add('is-nebula');
  schedule();

  new ResizeObserver(() => {
    size();
    if (!raf) draw();
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
  reduced.addEventListener('change', () => {
    hand = handWas = push = null;
    speed = [0, 0];
    schedule();
  });
  // The theme changes on <html>: the colors cross to the new ones.
  new MutationObserver(() => {
    const next = read();
    if (next.every((c, i) => c.every((v, j) => Math.abs(v - to[i][j]) < 1e-4))) return;
    const now = performance.now();
    from = colors(now);
    to = next;
    mixStart = now;
    schedule();
  }).observe(document.documentElement, { attributes: true, attributeFilter: ['class', 'data-preset', 'data-appearance'] });

  // The hand, from the page and from the demo's frame over the stage: the
  // pointer moving, or a finger dragging (its touches; the page still scrolls).
  let leaving = 0;
  const track = (x: number, y: number) => {
    const r = stage.getBoundingClientRect();
    const inside = x >= r.left && x <= r.right && y >= r.top && y <= r.bottom;
    clearTimeout(leaving);
    if (!inside) return leave();
    over = true;
    onPointer?.({ x: ((x - r.left) / r.width) * 2 - 1, y: ((y - r.top) / r.height) * 2 - 1 });
    if (still()) return;
    hand = [(x - r.left) / r.width, 1 - (y - r.top) / r.height];
    schedule();
  };
  const leave = () => {
    hand = handWas = null;
    if (!over) return;
    over = false;
    onPointer?.(null);
  };
  const listen = (target: Document | Window, offset: () => { left: number; top: number }, frame: boolean) => {
    const at = (x: number, y: number) => {
      const o = offset();
      track(o.left + x, o.top + y);
    };
    target.addEventListener('pointermove', (e) => at((e as PointerEvent).clientX, (e as PointerEvent).clientY), { passive: true, capture: frame });
    target.addEventListener(
      'touchmove',
      (e) => {
        const t = (e as TouchEvent).touches[0];
        if (t) at(t.clientX, t.clientY);
      },
      { passive: true, capture: frame },
    );
    // A finger lifted: the stroke eases out there (the next touch starts afresh).
    target.addEventListener('touchend', () => (hand = handWas = null), { passive: true, capture: frame });
    target.addEventListener('touchcancel', () => (hand = handWas = null), { passive: true, capture: frame });
  };
  listen(window, () => ({ left: 0, top: 0 }), false);
  document.documentElement.addEventListener('mouseleave', () => leave());
  const hooked = new WeakSet<Document>();
  const attach = (frame: HTMLIFrameElement) => {
    const hook = () => {
      try {
        const doc = frame.contentDocument;
        if (!doc || hooked.has(doc)) return;
        hooked.add(doc);
        listen(doc, () => frame.getBoundingClientRect(), true);
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
    stats: () => ({ frames, width, height, field: [...field] as [number, number], time }),
    state: () => ({ over, trail: trailing }),
    seek: (t) => {
      pinned = t;
      last = 0;
      if (raf) cancelAnimationFrame(raf);
      raf = 0;
      schedule();
    },
    bench: (n) => {
      const start = performance.now();
      for (let i = 0; i < n; i++) {
        nebula.draw({ time: time + i / 60, width, height, point, fieldWidth: field[0], fieldHeight: field[1] });
        gl.readPixels(0, 0, 1, 1, gl.RGBA, gl.UNSIGNED_BYTE, pixel);
      }
      draw();
      return { ms: (performance.now() - start) / n, width, height, field: [...field] as [number, number] };
    },
  };
}
