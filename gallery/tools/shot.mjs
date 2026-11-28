// Screenshots of the gallery's web build through Chrome's DevTools protocol
// (Node's own WebSocket, no packages). Serve the build with `tools/serve.mjs`,
// then shoot a page by its path, with the theme in the query
// (`http://127.0.0.1:8080/nsh-longbridge/button?preset=blue&appearance=dark`;
// the site's root is the home page): `node tools/shot.mjs <url> <out.png>
// [--dark] [--size 1280x860] [--wait 6000] [--click x,y]... [--hover x,y]
// [--key Tab]... [--frames 3] [--times 2000,4500] [--reduce-motion]` (points in CSS pixels;
// `CHROME` names another browser). With `--frames n`, n shots scrolled a
// screen apart, written as `<out>-1.png` … `<out>-n.png` (a tall page in
// frames, as a very tall window renders blank). With `--times`, one shot
// at each of those milliseconds after the wait, `<out>-<ms>.png`: frames of
// something that moves by itself. Also: `--throttle kbps,latency` (a slow
// network: shots and times count from the start of the load with `--wait 0`),
// `--no-js`, `--pre <js>` (run in every document before its scripts),
// `--eval <js>`... (run in the page after the clicks), `--after <js>`... (after the shots), `--scroll y` and
// `--report` (bytes over the wire, the layout shift and the time to `load`), `--gl` (WebGL2 in software),
// `--moves "x,y …"` with `--probe <js>` (the pointer moved through points, the page probed at each) and
// `--sweep x1,y1,x2,y2` (with `--times`: the pointer crossing from one point to the other), `--warm <url>`
// (that address loaded first, for `--warm-wait` ms (4000): the same page leaves its files in the cache, any page
// leaves the browser's GPU process started, which in software takes seconds) and `--stamp <js>` (with `--times`:
// run and printed just before each shot), `--rate x` (the page's CSS animations and transitions played at x
// times their speed: slow ones shot frame by frame where shots are slow), `--cast ms` (the screen recorded
// from the start of the load for that long, every frame Chrome paints: `<out>-cast-<n>.png`, each listed with
// its time from the start, in ms).
import { spawn } from "node:child_process";
import { mkdtempSync, readFileSync, writeFileSync } from "node:fs";
import { tmpdir } from "node:os";
import { join } from "node:path";

const args = process.argv.slice(2);
const [url, out] = args;
const flag = (name) => args.includes(name);
const opt = (name, d) => { const i = args.indexOf(name); return i >= 0 ? args[i + 1] : d; };
const all = (name) => args.flatMap((a, i) => (a === name ? [args[i + 1]] : []));
const [width, height] = opt("--size", "1280x860").split("x").map(Number);
const wait = Number(opt("--wait", "6000"));
const chrome = process.env.CHROME ?? "/Applications/Google Chrome.app/Contents/MacOS/Google Chrome";
// Port 0: Chrome picks a free port and writes it to DevToolsActivePort in
// its profile, so runs side by side never reach each other's browser.
const profile = mkdtempSync(join(tmpdir(), "nlb-chrome-"));
const proc = spawn(chrome, ["--headless=new", "--remote-debugging-port=0", "--hide-scrollbars",
  `--user-data-dir=${profile}`, `--window-size=${width},${height}`,
  // `--gl`: WebGL2 through the software renderer (plain headless Chrome has none).
  ...(flag("--gl") ? ["--use-angle=swiftshader", "--enable-unsafe-swiftshader"] : []),
  "about:blank"], { stdio: "ignore" });
const sleep = (ms) => new Promise((r) => setTimeout(r, ms));
let port, target;
for (let i = 0; i < 50 && !target; i++) {
  try {
    port ??= Number(readFileSync(join(profile, "DevToolsActivePort"), "utf8").split("\n")[0]) || undefined;
    if (port) target = (await (await fetch(`http://127.0.0.1:${port}/json/list`)).json()).find((t) => t.type === "page");
  } catch {}
  if (!target) await sleep(200);
}
if (!target) { proc.kill(); throw new Error("Chrome did not start its DevTools endpoint"); }
const ws = new WebSocket(target.webSocketDebuggerUrl);
await new Promise((r) => ws.addEventListener("open", r));
let id = 0;
const pending = new Map();
ws.addEventListener("message", (e) => { const m = JSON.parse(e.data); if (m.id && pending.has(m.id)) { pending.get(m.id)(m); pending.delete(m.id); } });
const send = (method, params = {}) => new Promise((r) => { const i = ++id; pending.set(i, r); ws.send(JSON.stringify({ id: i, method, params })); });
await send("Emulation.setDeviceMetricsOverride", { width, height, deviceScaleFactor: Number(opt("--scale", "2")), mobile: false });
await send("Emulation.setEmulatedMedia", {
  features: [
    { name: "prefers-color-scheme", value: flag("--dark") ? "dark" : "light" },
    { name: "prefers-reduced-motion", value: flag("--reduce-motion") ? "reduce" : "no-preference" },
  ],
});
await send("Page.enable");
await send("Network.enable");
let bytes = 0, requests = 0;
const urls = new Map(), loaded = [];
ws.addEventListener("message", (e) => {
  const m = JSON.parse(e.data);
  if (m.method === "Network.requestWillBeSent") urls.set(m.params.requestId, m.params.request.url);
  if (m.method === "Network.loadingFinished") { bytes += m.params.encodedDataLength; requests++; loaded.push([urls.get(m.params.requestId), m.params.encodedDataLength]); }
});
const throttle = opt("--throttle");
if (throttle) {
  const [kbps, latency] = throttle.split(",").map(Number);
  await send("Network.emulateNetworkConditions", { offline: false, latency, downloadThroughput: (kbps * 1000) / 8, uploadThroughput: (kbps * 1000) / 8 });
}
if (flag("--no-js")) await send("Emulation.setScriptExecutionDisabled", { value: true });
await send("Page.addScriptToEvaluateOnNewDocument", { source: `window.__cls=0;try{new PerformanceObserver(l=>{for(const e of l.getEntries())if(!e.hadRecentInput)window.__cls+=e.value}).observe({type:'layout-shift',buffered:true})}catch{}` });
for (const pre of all("--pre")) await send("Page.addScriptToEvaluateOnNewDocument", { source: pre });
const warm = opt("--warm");
if (warm) { await send("Page.navigate", { url: warm }); await sleep(Number(opt("--warm-wait", "4000"))); }
const rate = opt("--rate");
if (rate) { await send("Animation.enable"); await send("Animation.setPlaybackRate", { playbackRate: Number(rate) }); }
const cast = Number(opt("--cast", "0"));
const castFrames = [];
if (cast) {
  ws.addEventListener("message", (e) => {
    const m = JSON.parse(e.data);
    if (m.method !== "Page.screencastFrame") return;
    castFrames.push({ at: m.params.metadata.timestamp * 1000, data: m.params.data });
    ws.send(JSON.stringify({ id: ++id, method: "Page.screencastFrameAck", params: { sessionId: m.params.sessionId } }));
  });
  await send("Page.startScreencast", { format: "png", everyNthFrame: 1 });
}
const started = Date.now();
await send("Page.navigate", { url });
if (cast) {
  await sleep(Math.max(0, started + cast - Date.now()));
  await send("Page.stopScreencast");
  const stem = out.replace(/\.png$/, "");
  castFrames.forEach((f, i) => { writeFileSync(`${stem}-cast-${i}.png`, Buffer.from(f.data, "base64")); console.log(`cast ${i} ${Math.round(f.at - started)} abs ${Math.round(f.at)}`); });
}
await sleep(wait);
const evaluate = async (expression) => (await send("Runtime.evaluate", { expression, awaitPromise: true, returnByValue: true })).result?.result?.value;
for (const c of all("--click")) {
  const [x, y] = c.split(",").map(Number);
  for (const type of ["mouseMoved", "mousePressed", "mouseReleased"]) {
    await send("Input.dispatchMouseEvent", { type, x, y, button: "left", clickCount: 1 });
  }
  await sleep(700);
}
for (const k of all("--key")) {
  await send("Input.dispatchKeyEvent", { type: "keyDown", key: k, code: k, windowsVirtualKeyCode: k === "Tab" ? 9 : 0 });
  await send("Input.dispatchKeyEvent", { type: "keyUp", key: k, code: k, windowsVirtualKeyCode: k === "Tab" ? 9 : 0 });
  await sleep(300);
}
for (const js of all("--eval")) { const v = await evaluate(js); if (v !== undefined) console.log(`eval ${typeof v === "string" ? v : JSON.stringify(v)}`); await sleep(Number(opt("--eval-wait", "700"))); }
const scroll = opt("--scroll");
if (scroll) { await evaluate(`window.scrollTo(0, ${scroll})`); await sleep(900); }
// `--moves "x,y x,y …"`: the pointer moved through those points, `--move-wait` ms apart (300),
// each followed by every `--probe <js>`, printed.
const moves = opt("--moves");
if (moves) {
  for (const pt of moves.trim().split(/\s+/)) {
    const [x, y] = pt.split(",").map(Number);
    await send("Input.dispatchMouseEvent", { type: "mouseMoved", x, y });
    await sleep(Number(opt("--move-wait", "300")));
    for (const js of all("--probe")) console.log(`probe ${x},${y} ${await evaluate(js)}`);
  }
}
const hover = opt("--hover");
if (hover) { const [x, y] = hover.split(",").map(Number); await send("Input.dispatchMouseEvent", { type: "mouseMoved", x, y }); await sleep(500); }
const frames = Number(opt("--frames", "0"));
const capture = async (path) => {
  const shot = await send("Page.captureScreenshot", { format: "png" });
  writeFileSync(path, Buffer.from(shot.result.data, "base64"));
  console.log(`wrote ${path}`);
};
const times = opt("--times");
if (times) {
  const stem = out.replace(/\.png$/, "");
  // Each from the same start, however long the captures before it took.
  const from = Date.now();
  // `--sweep x1,y1,x2,y2`: the pointer moves from one point to the other across the times, in steps of 25 ms.
  const sweep = opt("--sweep")?.split(",").map(Number);
  const list = times.split(",").map(Number);
  const end = list[list.length - 1] || 1;
  for (const t of list) {
    if (sweep) {
      while (Date.now() < from + t) {
        const k = Math.min(1, (Date.now() - from) / end);
        await send("Input.dispatchMouseEvent", { type: "mouseMoved", x: sweep[0] + (sweep[2] - sweep[0]) * k, y: sweep[1] + (sweep[3] - sweep[1]) * k });
        await sleep(Math.min(25, Math.max(0, from + t - Date.now())));
      }
    } else await sleep(Math.max(0, from + t - Date.now()));
    for (const js of all("--stamp")) console.log(`stamp ${t} ${await evaluate(js)}`);
    await capture(`${stem}-${t}.png`);
  }
} else if (frames > 0) {
  const stem = out.replace(/\.png$/, "");
  for (let i = 1; i <= frames; i++) {
    await capture(`${stem}-${i}.png`);
    if (i < frames) {
      // The pointer over the page first: the scroll view takes the wheel
      // under the pointer.
      const [x, y] = [width / 2 + 120, height / 2];
      await send("Input.dispatchMouseEvent", { type: "mouseMoved", x, y });
      await sleep(150);
      await send("Input.dispatchMouseEvent", { type: "mouseWheel", x, y, deltaX: 0, deltaY: height - 120 });
      await sleep(900);
    }
  }
} else {
  await capture(out);
}
for (const js of all("--after")) { const v = await evaluate(js); if (v !== undefined) console.log(`after ${typeof v === "string" ? v : JSON.stringify(v)}`); }
if (flag("--report")) {
  const t = await evaluate("JSON.stringify({cls: window.__cls, load: performance.getEntriesByType('navigation')[0]?.loadEventEnd})");
  if (flag("--requests")) for (const [u, b] of loaded) console.log(`  ${String(b).padStart(9)} ${u}`);
  console.log(`report ${JSON.stringify({ bytes, requests, after: Date.now() - started, ...JSON.parse(t || "{}") })}`);
}
ws.close();
proc.kill();
