// The site's own colors (its surfaces, text, accent and code), as plain data
// (no NSH), so the website writes the same ones as CSS (`site.ts` explains
// what they are for; `website/src/lib/site-css.ts`).

/** nsh.dev's surfaces, text and accent, per appearance. */
export const sites = {
  light: {
    /** The page. */
    ground: "#ffffff",
    /** The hero's ground, top to bottom: a cool gray-white. */
    heroTop: "#f6f7f9",
    heroBottom: "#fbfbfc",
    card: "#ffffff",
    /** Recessed surfaces: tracks, code, the demo stage. */
    inset: "#f2f2f4",
    /** Raised controls and tinted bands. */
    raised: "#f7f7f8",
    /** Side panels. */
    sidebar: "#fafafa",
    /** Tinted bands of the home page. */
    band: "#f7f7f8",
    border: "#e2e2e6",
    input: "#d8d8dd",
    line: "#ececef",
    ink: "#101319",
    muted: "#5f636a",
    faint: "#9a9da3",
    /** The brand's slash, between NSH and the product. */
    slash: "#c3c7cf",
    /** The accent as text and lines. */
    citrus: "#4a840a",
    /** The accent as a small fill: dots, a live light. */
    citrusFill: "#9fd23a",
    /** The site's primary button: ink on the page, and the text on it. */
    solid: "#15181f",
    solidHover: "#2a2e37",
    onSolid: "#ffffff",
    highlight: "#ffffff",
    hover: "#0000000a",
    /** A card's soft shadow under its top light. */
    cardShadow: "inset 0 1px 0 #ffffff, 0 1px 2px #101e1e0d, 0 8px 24px -12px #101e1e14",
    raisedShadow: "inset 0 1px 0 #ffffff, inset 0 -1px 0 #00000006, 0 1px 2px #101e1e0d",
    insetShadow: "inset 0 1px 2px #1014200d",
    thumb: "linear-gradient(180deg, #ffffff, #ffffffcf)",
    thumbShadow: "inset 0 1px 0 #ffffff, inset 0 -1px 0 #0000000f, 0 1px 3px #10141e24",
    code: {
      bg: "#f7f8fa",
      fg: "#1d2129",
      keyword: "#4a7d06",
      string: "#2a7166",
      comment: "#8a909a",
      fn: "#2c53a8",
      type: "#3c5f9e",
      number: "#a35f00",
      property: "#5b4a2a",
      punct: "#6b7079",
    },
  },
  dark: {
    ground: "#0e1016",
    heroTop: "#0e1016",
    heroBottom: "#0e1016",
    card: "#161920",
    inset: "#0b0d12",
    raised: "#1c1f27",
    sidebar: "#0c0e13",
    band: "#12141b",
    border: "#282c35",
    input: "#30343e",
    line: "#1f222a",
    ink: "#eceef1",
    muted: "#9a9ea5",
    faint: "#6b6f76",
    slash: "#3a3f4a",
    citrus: "#c8f26d",
    citrusFill: "#c6ef6a",
    solid: "#eceef1",
    solidHover: "#ffffff",
    onSolid: "#0e1016",
    highlight: "#ffffff0d",
    hover: "#ffffff09",
    cardShadow: "inset 0 1px 0 #ffffff0d, 0 1px 2px #0000004d, 0 12px 32px -16px #00000080",
    raisedShadow: "inset 0 1px 0 #ffffff14, 0 1px 2px #00000073",
    insetShadow: "inset 0 1px 3px #00000059",
    thumb: "linear-gradient(180deg, #ffffff20, #ffffff16)",
    thumbShadow: "inset 0 1px 0 #ffffff38, inset 0 -1px 0 #00000033, 0 1px 3px #00000059",
    code: {
      bg: "#0a0c10",
      fg: "#d3d7de",
      keyword: "#c8f26d",
      string: "#93cdbf",
      comment: "#6b717c",
      fn: "#8fb2f5",
      type: "#b6c8f0",
      number: "#e9b872",
      property: "#d9c9a3",
      punct: "#8a8f98",
    },
  },
};

