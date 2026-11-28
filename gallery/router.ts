// The page shown, kept in the address's path on the web (`<base>button`, the
// home page at the base itself), so a page can be linked and the browser's
// back and forward move between pages. The base is wherever the site is
// served from (a local preview, Pages, a domain of its own), found from the
// address at launch. Addresses of the old form (`#/button`) move to the path.
// Native apps have no address: the page lives in `route` alone.
//
// On the web the home page is the website's (Astro, at the base): the
// gallery's own home page is the native app's, and going home on the web
// loads the website's page instead (a full navigation).

import { signal, UrlScheme } from "nsh";

/** An address in its parts: `origin` (`https://host`), `path`, `query` (`?…`) and `hash` (`#…`), each "" when absent. */
function parse(url: string) {
  const m = /^([a-z][a-z0-9+.-]*:\/\/[^/?#]*)?([^?#]*)(\?[^#]*)?(#.*)?$/i.exec(url);
  return { origin: m?.[1] ?? "", path: m?.[2] || "/", query: m?.[3] ?? "", hash: m?.[4] ?? "" };
}

/** `?a=1&b=2` (or `a=1&b=2`) as its pairs; a bare name (`?solo`) is the name with "". */
const paramsOf = (query: string): Record<string, string> =>
  Object.fromEntries(
    query
      .replace(/^\?/, "")
      .split("&")
      .filter(Boolean)
      .map((kv) => [...kv.split("=").map(decodeURIComponent), ""].slice(0, 2) as [string, string]),
  );

/** The page and the query an old address's hash names (`#/button?preset=blue`), or null. */
function fromHash(hash: string): { id: string; query: string } | null {
  const m = /^#\/?([^?]*)(\?.*)?$/.exec(hash);
  return m && m[1] ? { id: m[1].replace(/\/+$/, ""), query: m[2] ?? "" } : null;
}

let ids = new Set<string>();
let origin = "";
let base = "/";

/** The page a path shows: its last segment when that is a page's id (a slash after it allowed), else the home page. */
function idOf(path: string): string {
  const last = path.split("/").filter(Boolean).pop() ?? "";
  return ids.has(last) ? last : "home";
}

/** The full address of page `id` (the home page is the base itself), with `query`. */
const urlOf = (id: string, query = "") => `${origin}${base}${id === "home" ? "" : id}${query}`;

/** The options the address gave at launch (`?preset=blue&appearance=dark`; an old `#/home?…` too). Set by `startRouter`. */
export const launchParams: Record<string, string> = {};

/**
 * Whether the address asked for the page alone (`?solo`): no header, sidebar or pager around it, to look at one
 * component (or shoot it) without the gallery's frame. Read once, at launch, like the other options.
 */
export const solo = () => "solo" in launchParams && launchParams.solo !== "0";

export const route = signal<string>("home");

/** The document the app runs in (on the web the app's realm is an iframe of it), or null off the web. */
const page = (): { location: { assign(url: string): void; replace(url: string): void } } | null => {
  const w = globalThis as unknown as { parent?: unknown; location?: unknown };
  const doc = (w.parent && w.parent !== w ? w.parent : w) as { location?: { assign(url: string): void; replace(url: string): void } };
  return doc.location ? (doc as never) : null;
};

/** Loads the website's home page in place of the gallery (the web only); false where there is none. */
function leaveForHome(replace: boolean): boolean {
  const doc = UrlScheme.url() ? page() : null;
  if (!doc) return false;
  const url = urlOf("home");
  if (replace) doc.location.replace(url);
  else doc.location.assign(url);
  return true;
}

/**
 * Reads the address once, at launch: the site's base, the page and the
 * options. `pageIds`: every page's id, which a path's last segment is
 * matched against. An old `#/…` address is replaced by its path.
 */
export function startRouter(pageIds: string[]) {
  ids = new Set(pageIds);
  const url = UrlScheme.url();
  if (!url) return;
  const at = parse(url);
  origin = at.origin;
  const segments = at.path.split("/").filter(Boolean);
  if (ids.has(segments[segments.length - 1] ?? "")) segments.pop();
  base = `/${segments.map((s) => `${s}/`).join("")}`;
  const old = fromHash(at.hash);
  const id = old && ids.has(old.id) ? old.id : idOf(at.path);
  Object.assign(launchParams, paramsOf(at.query), old ? paramsOf(old.query) : {});
  if (id === "home" && leaveForHome(true)) return;
  route.set(id);
  if (old) UrlScheme.setUrl(urlOf(id, old.query || at.query), { history: "replace" });
}

/** Shows page `id`, as a new entry of the browser's history. */
export function navigate(id: string) {
  if (id === "home" && leaveForHome(false)) return;
  if (id === route()) return;
  route.set(id);
  if (UrlScheme.url()) UrlScheme.setUrl(urlOf(id), { history: "push" });
}

UrlScheme.on("change", (e) => {
  const at = parse(e.url);
  const old = fromHash(at.hash);
  if (old && ids.has(old.id)) {
    route.set(old.id);
    UrlScheme.setUrl(urlOf(old.id, old.query || at.query), { history: "replace" });
    return;
  }
  const id = idOf(at.path);
  if (id === "home" && leaveForHome(true)) return;
  route.set(id);
});
