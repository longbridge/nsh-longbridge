// The home page in the browser: the theme (kept in `localStorage` and shared
// with the gallery's pages and demos on a `BroadcastChannel`, both ways, as
// `gallery/sync.ts` does), the header's menus, the demos (their iframes,
// posters, sizes, visibility and the keyboard rule), the token sliders, the
// install command's copy button and the sections' reveal; and the stage's
// flowing backdrop (`backdrop.ts`), which hands the pointer on to the hero.

import { startBackdrop } from './backdrop';

const KEY = 'nsh-longbridge:theme';
const CHANNEL = 'nsh-longbridge';

type Appearance = 'system' | 'light' | 'dark';
interface Theme {
  preset: string;
  density: string;
  appearance: Appearance;
  edits: { theme: Record<string, string>; light: Record<string, string>; dark: Record<string, string> };
}

const root = document.documentElement;
const channel = 'BroadcastChannel' in window ? new BroadcastChannel(CHANNEL) : null;
/** This page's id, which the demos it embeds answer to (two tabs never talk across). */
const pageId = Math.random().toString(36).slice(2, 10);
const reduced = matchMedia('(prefers-reduced-motion: reduce)');

// ---------------------------------------------------------------- the theme

function load(): Theme {
  let s: Partial<Theme> = {};
  try {
    s = JSON.parse(localStorage.getItem(KEY) || '{}') || {};
  } catch {}
  return {
    preset: s.preset || 'neutral',
    density: s.density || 'default',
    appearance: s.appearance === 'light' || s.appearance === 'dark' ? s.appearance : 'system',
    edits: { theme: {}, light: {}, dark: {}, ...(s.edits || {}) },
  };
}

let theme = load();
/** The preset the visitor chose: the hero's loop shows others in passing (`passing`), which are not saved. */
let chosen = theme.preset;
/** The edits' variables set on <html>, to clear when they change. */
let edited: string[] = [];

const isDark = () => root.classList.contains('dark');

/** The theme on the page: <html>'s attributes and the edits as variables, and the pickers' states. */
function paint() {
  root.dataset.preset = theme.preset;
  root.dataset.density = theme.density;
  root.dataset.appearance = theme.appearance;
  (window as unknown as { __applyAppearance?: () => void }).__applyAppearance?.();
  for (const name of edited) root.style.removeProperty(name);
  edited = [];
  const own = (v: string) => v.replace(/var\(--([a-z0-9-]+)/g, 'var(--lb-$1');
  for (const [k, v] of Object.entries({ ...theme.edits.theme, ...theme.edits[isDark() ? 'dark' : 'light'] })) {
    root.style.setProperty(`--lb-${k}`, own(v));
    edited.push(`--lb-${k}`);
  }
  const check = (sel: string, attr: string, value: string) =>
    document.querySelectorAll<HTMLElement>(`[${sel}]`).forEach((el) => el.setAttribute(attr, String(el.getAttribute(sel) === value)));
  check('data-preset-choice', 'aria-checked', theme.preset);
  check('data-density-choice', 'aria-checked', theme.density);
  check('data-appearance-choice', 'aria-checked', theme.appearance);
  check('data-preset-swatch', 'data-on', theme.preset);
}

/** A change made here: painted, saved and announced. */
function choose(change: Partial<Theme>) {
  theme = { ...theme, ...change };
  if (change.preset) chosen = change.preset;
  paint();
  try {
    localStorage.setItem(KEY, JSON.stringify({ ...theme, preset: chosen }));
  } catch {}
  // A preset only passing (the loop's) stays a passing one for this page's demos.
  channel?.postMessage(theme.preset === chosen ? { t: 'theme', ...theme } : { t: 'theme', ...theme, passing: true, page: pageId });
}

matchMedia('(prefers-color-scheme: dark)').addEventListener('change', () => paint());

// ---------------------------------------------------------------- the header's menus

/** Places popover `menu` under the button that opens it, at its start or end. */
function placeMenus() {
  document.querySelectorAll<HTMLButtonElement>('[popovertarget]').forEach((button) => {
    const menu = document.getElementById(button.getAttribute('popovertarget')!);
    if (!menu) return;
    menu.addEventListener('beforetoggle', (e) => {
      if ((e as ToggleEvent).newState !== 'open') return;
      const r = button.getBoundingClientRect();
      menu.style.top = `${Math.round(r.bottom + 4)}px`;
      if (button.dataset.anchor === 'start') {
        menu.style.left = `${Math.max(8, Math.round(r.left))}px`;
        menu.style.right = 'auto';
      } else {
        menu.style.right = `${Math.max(8, Math.round(innerWidth - r.right))}px`;
        menu.style.left = 'auto';
      }
    });
  });
  document.addEventListener('click', (e) => {
    const el = (e.target as HTMLElement).closest<HTMLElement>('[data-preset-choice],[data-density-choice],[data-appearance-choice]');
    if (!el) return;
    if (el.dataset.presetChoice) choose({ preset: el.dataset.presetChoice });
    if (el.dataset.densityChoice) choose({ density: el.dataset.densityChoice });
    if (el.dataset.appearanceChoice) {
      choose({ appearance: el.dataset.appearanceChoice as Appearance });
      el.closest<HTMLElement>('[popover]')?.hidePopover();
    }
  });
}

// ---------------------------------------------------------------- the demos

interface Demo {
  el: HTMLElement;
  name: string;
  frame?: HTMLIFrameElement;
  ready: boolean;
  height: number;
  visible: boolean;
  waiting?: boolean;
}
const demos = new Map<string, Demo>();

const send = (m: Record<string, unknown>) => channel?.postMessage({ ...m, page: pageId });

/** Whether the first demo (the hero's, `data-eager`) is drawn: the others wait for it, so it has the network to itself. */
let firstDrawn = !document.querySelector('[data-demo][data-eager]');
const afterFirst: (() => void)[] = [];
function releaseFirst() {
  if (firstDrawn) return;
  firstDrawn = true;
  afterFirst.splice(0).forEach((f) => f());
}
// Should the hero never draw, the others still come.
setTimeout(releaseFirst, 20000);

function startDemo(d: Demo) {
  if (d.frame) return;
  if (!firstDrawn && !d.el.dataset.eager) {
    if (!d.waiting) afterFirst.push(() => startDemo(d));
    d.waiting = true;
    return;
  }
  const frame = document.createElement('iframe');
  frame.className = 'demo__frame';
  frame.title = d.el.dataset.label ?? d.name;
  frame.setAttribute('allowtransparency', 'true');
  frame.src = `${import.meta.env.BASE_URL.replace(/\/?$/, '/')}embed/${d.name}?page=${pageId}`;
  d.el.append(frame);
  d.frame = frame;
}

/** The demo is drawn: it fades in over its poster, at the height it takes. */
function showDemo(d: Demo) {
  if (d.ready) return;
  d.ready = true;
  if (d.el.dataset.eager) releaseFirst();
  if (d.height) d.el.style.height = `${d.height}px`;
  d.el.classList.add('is-live');
  // The demo learns what the page knows already.
  send({ t: 'view', demo: d.name, visible: d.visible });
  if (keyUsed) send({ t: 'key' });
  if (d.name === 'tokens') send({ t: 'tokens', values: tokenValues() });
}

function watchDemos() {
  document.querySelectorAll<HTMLElement>('[data-demo]').forEach((el) => {
    const d: Demo = { el, name: el.dataset.demo!, ready: false, height: 0, visible: true };
    demos.set(d.name, d);
    if (el.dataset.eager) startDemo(d);
  });
  // Others start as they near the view; every one says when it is in view (the hero plays only then).
  const near = new IntersectionObserver(
    (entries) => {
      for (const e of entries) if (e.isIntersecting) startDemo(demos.get((e.target as HTMLElement).dataset.demo!)!);
    },
    { rootMargin: '600px 0px' },
  );
  const seen = new IntersectionObserver(
    (entries) => {
      for (const e of entries) {
        const d = demos.get((e.target as HTMLElement).dataset.demo!)!;
        d.visible = e.isIntersecting;
        if (d.ready) send({ t: 'view', demo: d.name, visible: d.visible });
      }
    },
    { rootMargin: '-160px 0px 0px 0px' },
  );
  for (const d of demos.values()) {
    near.observe(d.el);
    seen.observe(d.el);
  }
  channel?.addEventListener('message', (e) => {
    const m = e.data as { t?: string; page?: string; demo?: string; height?: number } & Partial<Theme>;
    if (!m) return;
    if (m.t === 'theme') {
      // From a demo here, or another tab of the site: follow it, without
      // announcing it again; another page's loop is not followed.
      const { t: _, page: __, passing, ...rest } = m as Record<string, unknown>;
      if (passing && m.page !== pageId) return;
      theme = { ...theme, ...(rest as Partial<Theme>) };
      if (!passing && theme.preset) chosen = theme.preset;
      paint();
      return;
    }
    if (m.page !== pageId || !m.demo) return;
    const d = demos.get(m.demo);
    if (!d) return;
    if (m.t === 'size' && m.height) {
      d.height = m.height;
      if (d.ready) d.el.style.height = `${m.height}px`;
    }
    if (m.t === 'ready') entered(d.el).then(() => showDemo(d));
  });
}

/**
 * When the stage's entrance (the poster's cards rising in one after another,
 * `stage-in` in the stylesheet) has played: the live demo is held behind the
 * poster until then, so the entrance plays once, whole, however soon the demo
 * is ready, and the demo takes over from the state it ends in. At once where
 * nothing plays (less motion, another demo).
 */
function entered(el: HTMLElement): Promise<unknown> {
  const poster = el.querySelector<HTMLElement>('.demo__poster');
  const playing = (poster?.getAnimations({ subtree: true }) ?? []).filter((a) => (a as CSSAnimation).animationName === 'stage-in');
  if (!playing.length) return Promise.resolve();
  // A cancelled one (the stage resized past a card's breakpoint) counts as played; two seconds at most.
  return Promise.race([Promise.all(playing.map((a) => a.finished.catch(() => {}))), new Promise((r) => setTimeout(r, 2000))]);
}

// The keyboard rule: a key pressed anywhere stops the hero's loop for good,
// as on the gallery (the loop's popups take the focus).
let keyUsed = false;
addEventListener('keydown', () => {
  if (keyUsed) return;
  keyUsed = true;
  send({ t: 'key' });
});

// ---------------------------------------------------------------- the token sliders

function tokenValues(): Record<string, string> {
  const out: Record<string, string> = {};
  document.querySelectorAll<HTMLInputElement>('[data-knob]').forEach((i) => (out[i.dataset.knob!] = `${i.value}px`));
  return out;
}

function watchKnobs() {
  const file = document.querySelector<HTMLElement>('[data-token-file] code');
  const preview = document.querySelector<HTMLElement>('.preview--tokens');
  const lines = file ? [...file.querySelectorAll<HTMLElement>('.code__line')] : [];
  document.querySelectorAll<HTMLInputElement>('[data-knob]').forEach((input, i) => {
    const update = () => {
      const name = input.dataset.knob!;
      const value = `${input.value}px`;
      document.querySelector(`[data-knob-value="${name}"]`)!.textContent = value;
      // The file's line for it: its value is the line's last string.
      const line = lines[3 + i];
      const strings = line?.querySelectorAll('.c-string');
      const last = strings?.[strings.length - 1];
      if (last) last.textContent = last.textContent!.replace(/"[^"]*"/, `"${value}"`);
      preview?.style.setProperty(`--lb-${name}`, value);
      input.style.setProperty('--at', `${((+input.value - +input.min) / (+input.max - +input.min)) * 100}%`);
      send({ t: 'tokens', values: { [name]: value } });
    };
    input.addEventListener('input', update);
    input.style.setProperty('--at', `${((+input.value - +input.min) / (+input.max - +input.min)) * 100}%`);
  });
}

// ---------------------------------------------------------------- small things

function watchCopy() {
  document.querySelectorAll<HTMLButtonElement>('[data-copy]').forEach((b) =>
    b.addEventListener('click', async () => {
      try {
        await navigator.clipboard.writeText(b.dataset.copy!);
      } catch {
        return;
      }
      b.classList.add('is-done');
      b.setAttribute('aria-label', 'Copied');
      setTimeout(() => {
        b.classList.remove('is-done');
        b.setAttribute('aria-label', 'Copy the command');
      }, 1600);
    }),
  );
}

/** Sections rise in as they come into view (none with less motion: they are shown as they are). */
function watchReveal() {
  const sections = document.querySelectorAll<HTMLElement>('.reveal');
  if (reduced.matches || !('IntersectionObserver' in window)) return;
  root.classList.add('can-reveal');
  const io = new IntersectionObserver(
    (entries) => {
      for (const e of entries)
        if (e.isIntersecting) {
          e.target.classList.add('is-in');
          io.unobserve(e.target);
        }
    },
    { rootMargin: '0px 0px -12% 0px' },
  );
  sections.forEach((s) => io.observe(s));
}

// ---------------------------------------------------------------- the stage

/** The backdrop behind the hero; the pointer over the stage goes on to the demo (its cards' depth), once a frame. */
function watchStage() {
  const stage = document.querySelector<HTMLElement>('.stage');
  if (!stage) return;
  let pending: { x: number; y: number } | null | undefined;
  let queued = false;
  startBackdrop(stage, (p) => {
    pending = p;
    if (queued) return;
    queued = true;
    requestAnimationFrame(() => {
      queued = false;
      send({ t: 'pointer', demo: 'hero', at: pending ?? null });
    });
  }).then((backdrop) => ((window as unknown as { __silk?: unknown }).__silk = backdrop));
}

export function start() {
  paint();
  watchStage();
  placeMenus();
  watchDemos();
  watchKnobs();
  watchCopy();
  watchReveal();
}
