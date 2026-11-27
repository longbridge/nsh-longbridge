// What the hero demo's cards around the sheet show, and where they go, as
// plain data (no NSH): the demo (`hero-demo.tsx`) draws them with the
// library's components, and the website's poster of it (`Posters.astro`)
// draws the same in HTML.

/** A portrait drawn as an SVG: a backdrop, a head and shoulders. */
export function portrait(backdrop: string, skin: string, shirt: string): string {
  const svg = `<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 64 64"><rect width="64" height="64" fill="${backdrop}"/><circle cx="32" cy="26" r="12" fill="${skin}"/><path d="M10 64c2-14 11-21 22-21s20 7 22 21z" fill="${shirt}"/></svg>`;
  return `data:image/svg+xml;utf8,${encodeURIComponent(svg)}`;
}

/** Who has the invoice open: the first two from the start, the third joins as the loop picks the customer. */
export const viewers = [
  { name: "Maya Chen", photo: portrait("#c7d2fe", "#f2c9a0", "#4f46e5") },
  { name: "Leo Park", photo: portrait("#bbf7d0", "#d9a27a", "#15803d") },
  { name: "Ana Ruiz", photo: portrait("#fde68a", "#8d5a3b", "#b45309") },
];

/** The presence card's lines for the first `n` viewers. */
export const presence = (n: number) => ({
  title: n >= 3 ? "Ana joined" : "Maya and Leo",
  line: n >= 3 ? "3 people on this invoice" : "Viewing this invoice",
});

/** How many invoices this quarter came to each $200, from $0 to $6,000. */
export const amounts = [
  1, 2, 4, 6, 9, 12, 15, 17, 18, 17, 16, 14, 12, 11, 9, 8, 7, 6, 5, 5, 4, 3, 3, 2, 2, 2, 1, 1, 1, 2,
];
export const AMOUNT_MAX = 6000;
export const AMOUNT_STEP = 50;

/** The month's collections: collected of due, before this invoice is sent. */
export const COLLECTED = 31200;
export const DUE = 48000;

export const money = (n: number) => `$${Math.round(n).toLocaleString("en-US")}`;
export const percent = (collected: number) => `${Math.round((collected / DUE) * 100)}%`;

/**
 * The hero's layout at a stage `width`: the sheet's width, and how many of
 * the cards around it fit beside it (3 from a stage of 1100, the presence
 * and the amounts from 900, none below).
 */
export function heroLayout(width: number) {
  const phone = width < 560;
  const sheet = Math.min(440, width - (phone ? 24 : 64));
  const side = (width - sheet) / 2;
  const cards = phone ? 0 : side >= 330 ? 3 : side >= 230 ? 2 : 0;
  return { phone, sheet, side, cards };
}

/** How far the cards reach over the sheet's edges. */
export const OVERLAP = 16;
