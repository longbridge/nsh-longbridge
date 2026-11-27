// The theme the visitor chose, kept and shared on the web: the color preset,
// the density, light / dark / system and the studio's edits are saved in the
// site's `localStorage` (one key), so a component page opened from the home
// page shows the same look, and are announced on a `BroadcastChannel`, so
// the website's home page (Astro, the same origin) and the demos it embeds
// follow each other live, both ways, without a reload. The website posts the
// same message (`website/src/lib/theme.ts`). The channel also carries the
// embeds' own messages (`embed.tsx`). Native apps have neither: there this
// does nothing.
//
// The home page's hero loop moves the preset on by itself (`showPreset`):
// that preset is only passing, so it is not saved, and only the page that
// embeds the loop follows it (the message is marked `passing`, with the
// page's id); the gallery's other tabs keep the visitor's. Once the visitor
// takes the loop over, the preset it shows becomes theirs (`adoptTheme`).

import { batch, effect, untracked } from "nsh";
import { type Appearance, appearance, colorPreset, colorPresets, densityPreset, densityPresets, edits } from "./theme";

const KEY = "nsh-longbridge:theme";

/** The channel's name, which the website uses too. */
export const CHANNEL = "nsh-longbridge";

interface Shared {
  preset: string;
  density: string;
  appearance: Appearance;
  edits: ReturnType<typeof edits>;
}

// The browser's globals, which the web runtime keeps (the app runs on the
// browser's engine); absent on native platforms.
const web = globalThis as unknown as {
  localStorage?: { getItem(k: string): string | null; setItem(k: string, v: string): void };
  BroadcastChannel?: new (name: string) => Channel;
};

export interface Channel {
  postMessage(data: unknown): void;
  addEventListener(type: "message", listener: (e: { data: unknown }) => void): void;
}

/** The site's channel, or null off the web. */
export const channel: Channel | null = (() => {
  try {
    return web.BroadcastChannel ? new web.BroadcastChannel(CHANNEL) : null;
  } catch {
    return null;
  }
})();

const snapshot = (): Shared => ({ preset: colorPreset(), density: densityPreset(), appearance: appearance(), edits: edits() });

/** Sets the signals to `s`, skipping what it does not name or the gallery does not know. */
function apply(s: Partial<Shared>) {
  batch(() => {
    if (s.preset && colorPresets.some((p) => p.id === s.preset)) colorPreset.set(s.preset);
    if (s.density && densityPresets.some((p) => p.id === s.density)) densityPreset.set(s.density);
    if (s.appearance === "light" || s.appearance === "dark" || s.appearance === "system") appearance.set(s.appearance);
    if (s.edits && typeof s.edits === "object") {
      const e: Partial<Shared["edits"]> = s.edits;
      edits.set({ theme: e.theme ?? {}, light: e.light ?? {}, dark: e.dark ?? {} });
    }
  });
}

/** The saved theme, applied: call before the address's options, which win. */
export function restoreTheme() {
  try {
    const saved = web.localStorage?.getItem(KEY);
    if (saved) apply(JSON.parse(saved));
  } catch {}
}

/** The preset the hero's loop shows for now, and the one the visitor chose. */
let passing: string | null = null;
let chosen = "";
/** The embedding page's id, when this is one of the home page's demos. */
let embedPage = "";

/** The hero's loop shows preset `id`, without making it the visitor's. */
export function showPreset(id: string) {
  passing = id;
  colorPreset.set(id);
}

/** The visitor took the loop over: the preset it shows is theirs now. */
export function adoptTheme() {
  if (passing === null) return;
  passing = null;
  chosen = untracked(colorPreset);
  save(untracked(snapshot));
}

function save(now: Shared) {
  const stored = { ...now, preset: passing !== null && now.preset === passing ? chosen : now.preset };
  try {
    web.localStorage?.setItem(KEY, JSON.stringify(stored));
  } catch {}
  const quiet = stored.preset !== now.preset;
  channel?.postMessage(quiet ? { t: "theme", ...now, passing: true, page: embedPage } : { t: "theme", ...now });
}

/**
 * Saves and announces each change from here, and takes the changes announced
 * elsewhere. `page`: the embedding page's id, for one of its demos.
 */
export function shareTheme(page = "") {
  if (!channel && !web.localStorage) return;
  embedPage = page;
  chosen = untracked(colorPreset);
  let last = JSON.stringify(untracked(snapshot));
  channel?.addEventListener("message", (e) => {
    const m = e.data as { t?: string; passing?: boolean; page?: string } & Partial<Shared>;
    if (m?.t !== "theme") return;
    // Another page's loop: none of this page's business.
    if (m.passing && (!embedPage || m.page !== embedPage)) return;
    const { t: _, passing: __, page: ___, ...shared } = m;
    if (!m.passing && shared.preset) chosen = shared.preset;
    last = JSON.stringify({ ...untracked(snapshot), ...shared });
    apply(shared);
  });
  effect(() => {
    const now = snapshot();
    const json = JSON.stringify(now);
    if (json === last) return;
    last = json;
    if (passing !== null && now.preset !== passing) passing = null;
    if (passing === null) chosen = now.preset;
    save(now);
  });
}
