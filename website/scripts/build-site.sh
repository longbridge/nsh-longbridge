#!/bin/sh
# Puts the whole Pages site together in one folder:
#   - the gallery's web build (gallery/, NSH wasm): every component page at
#     `<id>.html`, the theme studio, `404.html`, and the home page's live demos
#     at `embed/<name>.html`, each a copy of the gallery's entry with
#     `<base href>` so its relative files resolve from the site's root (the
#     embeds also get a transparent page, as the website draws behind them);
#   - the website's home page (website/, Astro) as `index.html`, its files under
#     `_site/`.
# Usage: website/scripts/build-site.sh [out-dir]   (default website/site)
# Env: SITE_BASE (default /nsh-longbridge/), PUBLIC_SITE_URL; the gallery's web
# build needs clang and llvm-ar for wasm32 (AR_wasm32_unknown_unknown=llvm-ar).
set -eu

website=$(cd "$(dirname "$0")/.." && pwd)
repo=$(dirname "$website")
out=${1:-"$website/site"}
base=${SITE_BASE:-/nsh-longbridge/}
tmp=$(mktemp -d)
trap 'rm -rf "$tmp"' EXIT

echo "› gallery (nsh package --target web)"
(cd "$repo/gallery" && nsh package --target web App.tsx -o "$tmp/web")
gallery=$(find "$tmp/web" -mindepth 1 -maxdepth 1 -type d | head -n 1)
# Pages compresses on its own; the precompressed copies only add weight.
find "$gallery" \( -name '*.br' -o -name '*.gz' \) -delete

rm -rf "$out"
mkdir -p "$out"
cp -R "$gallery"/. "$out"/
sed "s|<head>|<head><base href=\"$base\">|" "$out/index.html" > "$tmp/entry.html"
grep -q "<base href=\"$base\">" "$tmp/entry.html"
rm "$out/index.html"
for page in "$repo"/gallery/pages/*.tsx; do
  id=$(basename "$page" .tsx)
  [ "$id" = home ] || cp "$tmp/entry.html" "$out/$id.html"
done
cp "$tmp/entry.html" "$out/404.html"
mkdir -p "$out/embed"
sed 's|</head>|<style>:root{color-scheme:light!important}html,body{background:transparent!important}#status{display:none!important}</style></head>|' \
  "$tmp/entry.html" > "$tmp/embed.html"
for name in hero tokens sample; do cp "$tmp/embed.html" "$out/embed/$name.html"; done

echo "› website (astro build)"
(cd "$website" && PUBLIC_SITE_BASE="$base" SITE_OUT_DIR="$tmp/astro" bunx --bun astro build)
cp -R "$tmp/astro"/. "$out"/

echo "✓ $out"
