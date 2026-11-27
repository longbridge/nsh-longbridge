# Website

The site's home page, built with [Astro](https://astro.build): HTML and CSS around three live demos. The demos are the
gallery's web build (`gallery/`, NSH) shown in iframes (`embed/hero`, `embed/tokens`, `embed/sample`). The component
pages and the theme studio are the gallery itself, at their usual addresses.

## Commands

Uses [Bun](https://bun.sh) (`bun install` once).

```sh
bun run dev     # the home page alone, without the demos (their posters show)
bun run build   # the home page alone, into dist/
bun run site    # the whole Pages site into site/: the gallery's web build and this page
```

`bun run site` runs `scripts/build-site.sh [out-dir]`. It builds the gallery with
`nsh package --target web`, which needs clang and `llvm-ar` for wasm32 (on macOS:
`PATH=/opt/homebrew/opt/llvm/bin:$PATH AR_wasm32_unknown_unknown=llvm-ar`). Then it writes:

- every gallery page as `<id>.html`, with `<base href>`;
- `404.html`;
- the demos as `embed/<name>.html`;
- this page as `index.html`.

`SITE_BASE` sets the base (default `/nsh-longbridge/`). `PUBLIC_SITE_URL` sets the origin used in the share links.

To preview it as Pages serves it:

```sh
node gallery/tools/serve.mjs website/site --port 8080 --prefix /nsh-longbridge/ --pages
```
