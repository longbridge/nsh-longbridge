// The page shown, kept in the address on the web (`#/button`), so a page
// can be linked and the browser's back and forward move between pages.

import { signal, UrlScheme } from "nsh";

const fromUrl = (url: string | null): string | null => {
  const hash = url?.split("#")[1]?.split("?")[0];
  return hash ? hash.replace(/^\/+/, "") : null;
};

/** The options the address gave at launch (`#/home?preset=blue&appearance=dark`). */
export const launchParams: Record<string, string> = Object.fromEntries(
  (UrlScheme.url()?.split("#")[1]?.split("?")[1] ?? "")
    .split("&")
    .filter((kv) => kv.includes("="))
    .map((kv) => kv.split("=").map(decodeURIComponent) as [string, string]),
);

export const route = signal<string>(fromUrl(UrlScheme.url()) ?? "home");

/** Shows page `id`, as a new entry of the browser's history. */
export function navigate(id: string) {
  if (id === route()) return;
  route.set(id);
  const url = UrlScheme.url();
  if (url) UrlScheme.setUrl(`${url.split("#")[0]}#/${id}`, { history: "push" });
}

UrlScheme.on("change", (e) => {
  const id = fromUrl(e.url);
  route.set(id ?? "home");
});
