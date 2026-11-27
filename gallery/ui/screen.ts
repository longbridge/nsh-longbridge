// The window's width classes the gallery lays out by.

import { windowSize } from "nsh";

/** Below this width the sidebar goes behind a button and tables drop a column. */
export const compact = () => windowSize().width < 900;

/** Phone widths: the header keeps its essentials. */
export const narrow = () => windowSize().width < 640;

/** The page's side margins and the sidebar's width, as the shell lays them out. */
export const pagePadding = () => (compact() ? 20 : 48);
const sidebarWidth = 256;

/** The width a page's content has: the window less the sidebar and the margins (`wide` pages go to 1280). */
export const contentWidth = (wide = false) =>
  Math.min(wide ? 1280 : 960, windowSize().width - (compact() ? 0 : sidebarWidth)) - 2 * pagePadding();

/** The width of each of `columns` columns `gap` apart in `width`. */
export const columnWidth = (width: number, columns: number, gap: number) => Math.floor((width - gap * (columns - 1)) / columns);
