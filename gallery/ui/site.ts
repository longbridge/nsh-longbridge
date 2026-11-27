// The site's own look, outside the library's tokens. The home page, the
// theme studio and the header belong to the NSH family by a few shared
// traits only: Figtree headlines, NSH's cool grays and stepped surfaces,
// its mark, and citrus as a small accent (labels, links, dots). Buttons are
// neutral. What the site demonstrates (the scene, the windows, every
// component) follows the theme instead, so a preset recolors the demos and
// never the site around them.

import { windowSize } from "nsh";
import Figtree from "../fonts/Figtree-Variable.woff2";
import { colorOf, scheme } from "../theme";
import { sites } from "./site-colors";
import { stagePalette } from "./stage-colors";

export { sites };

/** The site's colors in the shown appearance. */
export const site = () => sites[scheme()];

/** The headlines' face, Figtree (shipped with the gallery, OFL), the system's sans after it. */
export const DISPLAY = `${Figtree}, -apple-system, system-ui, sans-serif`;

/** The side margin: 16 on a phone, 24 above (nsh.dev's gutter). */
export const gutter = () => (windowSize().width < 640 ? 16 : 24);

/** The width of the site's column: 1280 at most, as nsh.dev's. */
export const siteWidth = () => Math.min(1280, windowSize().width - 2 * gutter());

/**
 * The stage's colors from the preset's primary in the shown appearance
 * (`stage-colors.ts`).
 */
export const stageColors = () => stagePalette(colorOf("primary", scheme()), scheme());

export const REPOSITORY = "https://github.com/longbridge/nsh-longbridge";
export const NSH_SITE = "https://nsh.dev";
