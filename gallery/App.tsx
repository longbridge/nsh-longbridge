// The gallery of nsh-longbridge: every component, the theme editor and the
// foundation, built with the library itself.

import { mount, platform } from "nsh";
import { Window } from "nsh/desktop";
import { pages } from "./catalog";
import { Embed, embedOf, launchAddress } from "./embed";
import { launchParams, startRouter } from "./router";
import { restoreTheme, shareTheme } from "./sync";
import { type Appearance, appearance, colorPreset, densityPreset, followTheme } from "./theme";
import { Shell } from "./ui/shell";

// On the web: the theme chosen earlier on the site, then the address, which
// names the page and may choose the theme shown (links to a look,
// screenshots). An address under `embed/` is one of the home page's demos,
// which the website embeds (`embed.tsx`).
const embed = embedOf(launchAddress());
restoreTheme();
if (!embed) startRouter(pages.map((p) => p.id));
if (launchParams.preset) colorPreset.set(launchParams.preset);
if (launchParams.density) densityPreset.set(launchParams.density);
if (launchParams.appearance) appearance.set(launchParams.appearance as Appearance);

function App() {
  followTheme();
  shareTheme(embed?.page);
  return embed ? <Embed name={embed.name} page={embed.page} /> : <Shell />;
}

mount(() => (
  <Window id="main" titleBar={platform.os === "macos" ? { style: "hidden", height: 56 } : undefined}>
    {App}
  </Window>
));
