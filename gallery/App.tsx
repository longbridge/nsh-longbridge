// The gallery of nsh-longbridge: every component, the theme editor and the
// foundation, built with the library itself.

import { mount, platform } from "nsh";
import { Window } from "nsh/desktop";
import { launchParams } from "./router";
import { type Appearance, appearance, colorPreset, densityPreset, followTheme } from "./theme";
import { Shell } from "./ui/shell";

// The address may choose the theme shown (links to a look, screenshots).
if (launchParams.preset) colorPreset.set(launchParams.preset);
if (launchParams.density) densityPreset.set(launchParams.density);
if (launchParams.appearance) appearance.set(launchParams.appearance as Appearance);

function App() {
  followTheme();
  return <Shell />;
}

mount(() => (
  <Window id="main" titleBar={platform.os === "macos" ? { style: "hidden", height: 56 } : undefined}>
    {App}
  </Window>
));
