// A local preview of the gallery's web build as the site serves it, the
// pages at paths (`/nsh-longbridge/combobox`): `node tools/serve.mjs <dir>
// [--port 8080] [--prefix /nsh-longbridge/]`, where `<dir>` is what
// `nsh package --target web` wrote (the folder holding index.html). Node's
// own modules only. A path under the prefix with no file behind it gets
// index.html, and every index.html served gets `<base href="<prefix>">` at the
// start of its <head>, so a page one level down still loads the site's files.
// `/` moves to the prefix. Listens on 127.0.0.1 only.
//
// `--pages` serves a folder as GitHub Pages does instead (the whole site from
// `website/scripts/build-site.sh`): a path is its file, `<path>.html` or
// `<path>/index.html`, else `404.html` with a 404; the files as they are,
// gzipped when the browser takes it (as Pages does), cached in memory.

import { createServer } from "node:http";
import { readFile, stat } from "node:fs/promises";
import { extname, join, normalize, resolve, sep } from "node:path";
import { gzipSync } from "node:zlib";
import { createHash } from "node:crypto";

const args = process.argv.slice(2);
const opt = (name, d) => { const i = args.indexOf(name); return i >= 0 ? args[i + 1] : d; };
const root = resolve(args.find((a, i) => !a.startsWith("--") && !args[i - 1]?.startsWith("--")) ?? ".");
const port = Number(opt("--port", "8080"));
const pages = args.includes("--pages");
const zipped = new Map();
const prefix = `/${opt("--prefix", "/nsh-longbridge/").replace(/^\/+|\/+$/g, "")}/`.replace(/^\/\/$/, "/");

const types = {
  ".html": "text/html; charset=utf-8",
  ".js": "text/javascript; charset=utf-8",
  ".mjs": "text/javascript; charset=utf-8",
  ".css": "text/css; charset=utf-8",
  ".json": "application/json",
  ".webmanifest": "application/manifest+json",
  ".wasm": "application/wasm",
  ".svg": "image/svg+xml",
  ".png": "image/png",
  ".webp": "image/webp",
  ".avif": "image/avif",
  ".txt": "text/plain; charset=utf-8",
  ".xml": "application/xml",
  ".jpg": "image/jpeg",
  ".ico": "image/x-icon",
  ".woff2": "font/woff2",
  ".ttf": "font/ttf",
  ".otf": "font/otf",
};

/** The file under `root` a site path names, or null (none, a folder without index.html, or outside `root`). */
async function fileOf(path) {
  const file = normalize(join(root, path));
  if (file !== root && !file.startsWith(root + sep)) return null;
  try {
    const s = await stat(file);
    if (s.isFile()) return file;
    if (s.isDirectory()) {
      const index = join(file, "index.html");
      if ((await stat(index)).isFile()) return index;
    }
  } catch {}
  return null;
}

const withBase = (html) => html.replace(/<head([^>]*)>/i, `<head$1>\n<base href="${prefix}">`);

createServer(async (req, res) => {
  const path = decodeURIComponent((req.url ?? "/").split(/[?#]/)[0]);
  if (!path.startsWith(prefix)) {
    if (`${path}/` === prefix || path === "/") {
      res.writeHead(302, { location: prefix });
      return res.end();
    }
    res.writeHead(404, { "content-type": "text/plain" });
    return res.end("Not under the site's prefix\n");
  }
  const rest = path.slice(prefix.length);
  if (pages) {
    const found = (await fileOf(rest)) ?? (rest && !rest.endsWith("/") ? await fileOf(`${rest}.html`) : null);
    const file = found ?? join(root, "404.html");
    try {
      let body = await readFile(file);
      const headers = { "content-type": types[extname(file)] ?? "application/octet-stream", "cache-control": "max-age=600" };
      if (/\bgzip\b/.test(req.headers["accept-encoding"] ?? "") && !/\.(png|jpe?g|webp|avif|woff2?)$/.test(file)) {
        const key = createHash("sha1").update(body).digest("hex");
        body = zipped.get(key) ?? zipped.set(key, gzipSync(body, { level: 9 })).get(key);
        headers["content-encoding"] = "gzip";
      }
      res.writeHead(found ? 200 : 404, headers);
      return res.end(req.method === "HEAD" ? undefined : body);
    } catch {
      res.writeHead(404, { "content-type": "text/plain" });
      return res.end("Not found\n");
    }
  }
  // A path with no file behind it is a page: the app reads it from the address.
  const file = (await fileOf(rest)) ?? join(root, "index.html");
  try {
    let body = await readFile(file);
    const type = types[extname(file)] ?? "application/octet-stream";
    if (extname(file) === ".html") body = Buffer.from(withBase(body.toString("utf8")));
    res.writeHead(200, { "content-type": type, "cache-control": "no-store" });
    res.end(req.method === "HEAD" ? undefined : body);
  } catch {
    res.writeHead(404, { "content-type": "text/plain" });
    res.end("Not found\n");
  }
}).listen(port, "127.0.0.1", () => console.log(`http://127.0.0.1:${port}${prefix} serves ${root}`));
