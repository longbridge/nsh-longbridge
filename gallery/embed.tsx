// The home page's live demos, one per address, for the website to embed
// (`embed/hero`, `embed/tokens`, `embed/sample`): the demo's components
// alone, edge to edge, on nothing (the website draws the stage and the panels
// behind the frame, and the page's styles are made transparent when the site
// is put together), with no header, sidebar or page around them. The website (Astro) shows
// each in an iframe over a poster of it and talks to it on the site's channel
// (`sync.ts`), each message carrying the page's id (`?page=` in the address)
// so two tabs never answer each other:
//
// - the demo says `ready` once it has drawn, and its `size` (the height it
//   takes at the frame's width) whenever that changes, so the frame fits it;
// - the page says when the frame is in `view` (the hero's loop plays only
//   then; the first one comes as the page shows the demo over its poster), when a `key` was pressed (the hero's loop stops for good, as on
//   the gallery), the token demo's `tokens` as its sliders move, and the
//   `pointer` over the hero's stage (its cards lean from it).

import { signal, UrlScheme, View, windowSize } from "nsh";
import { channel } from "./sync";
import { HeroDemo } from "./ui/hero-demo";
import { InviteCard } from "./ui/sample";
import { keyboardUsed } from "./ui/screen";
import { knobs, WorkspaceCard } from "./ui/token-demo";

export type EmbedName = "hero" | "tokens" | "sample";

export const embedNames: EmbedName[] = ["hero", "tokens", "sample"];

/** The embed an address names (`…/embed/hero?page=…`), with the page's id, or null. */
export function embedOf(url: string | null): { name: EmbedName; page: string } | null {
  const m = url && /\/embed\/([a-z]+)(?:\.html)?\/?(?:\?([^#]*))?/.exec(url);
  if (!m || !embedNames.includes(m[1] as EmbedName)) return null;
  const page = /(?:^|&)page=([^&]*)/.exec(m[2] ?? "")?.[1] ?? "";
  return { name: m[1] as EmbedName, page: decodeURIComponent(page) };
}

/**
 * The address the app was opened at. In a frame (the website's demos) the
 * runtime gives none, so it is read from the document the app's realm runs in.
 */
export function launchAddress(): string | null {
  const url = UrlScheme.url();
  if (url) return url;
  const w = globalThis as unknown as { parent?: { location?: { href?: string } }; location?: { href?: string } };
  try {
    return (w.parent && w.parent !== (w as unknown) ? w.parent : w).location?.href ?? null;
  } catch {
    return null;
  }
}

const startTokens = () => Object.fromEntries(knobs.map((k) => [k.name, `${k.start}px`]));

/** The demo `name`, filling the frame's width; `page`: the embedding page's id. */
export function Embed(props: { name: EmbedName; page: string }) {
  // Not until the page says so: it shows the demo (`view`) once its poster's
  // entrance has played, and the hero's loop starts from the top then.
  const inView = signal(false);
  const pointer = signal<{ x: number; y: number } | null>(null);
  const tokens = signal<Record<string, string>>(startTokens());
  const send = (m: Record<string, unknown>) => channel?.postMessage({ ...m, page: props.page, demo: props.name });

  channel?.addEventListener("message", (e) => {
    const m = e.data as {
      t?: string;
      page?: string;
      demo?: string;
      visible?: boolean;
      values?: Record<string, string>;
      at?: { x: number; y: number } | null;
    };
    if (!m || m.page !== props.page) return;
    if (m.t === "key") keyboardUsed.set(true);
    if (m.t === "view" && m.demo === props.name) inView.set(!!m.visible);
    if (m.t === "pointer" && m.demo === props.name) pointer.set(m.at ?? null);
    if (m.t === "tokens" && props.name === "tokens" && m.values) tokens.set({ ...tokens(), ...m.values });
  });

  // The height drawn, and `ready` after the first one is on screen.
  let ready = false;
  const measured = (height: number) => {
    if (height <= 0) return;
    send({ t: "size", height: Math.ceil(height) });
    if (!ready) {
      ready = true;
      setTimeout(() => send({ t: "ready" }), 50);
    }
  };

  const width = () => windowSize().width;
  return (
    <View onKeyDown={() => keyboardUsed.set(true)}>
      <View onLayout={(e) => measured(e.bounds.size.height)}>
        {props.name === "hero" ? (
          <HeroDemo width={width()} inView={inView()} pointer={pointer()} bare />
        ) : props.name === "tokens" ? (
          <View class="items-center justify-center" style={{ paddingVertical: width() >= 480 ? 40 : 24, paddingHorizontal: width() >= 480 ? 40 : 14 }}>
            <WorkspaceCard tokens={tokens()} />
          </View>
        ) : (
          <View
            class="items-center justify-center"
            style={{
              paddingVertical: width() >= 480 ? 48 : 28,
              paddingHorizontal: width() >= 480 ? 40 : 14,
            }}
          >
            <View style={{ width: width() >= 480 ? 380 : "100%" }}>
              <InviteCard class="shadow-lg" />
            </View>
          </View>
        )}
      </View>
    </View>
  );
}
