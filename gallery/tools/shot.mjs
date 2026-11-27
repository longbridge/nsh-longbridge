// Screenshots of the gallery's web build through Chrome's DevTools protocol
// (Node's own WebSocket, no packages): `node tools/shot.mjs <url> <out.png>
// [--dark] [--size 1280x860] [--wait 6000] [--click x,y]... [--hover x,y]
// [--key Tab]... [--frames 3]` (points in CSS pixels; `CHROME` names another
// browser). With `--frames n`, n shots scrolled a screen apart, written as
// `<out>-1.png` … `<out>-n.png` (a tall page in frames, as a very tall
// window renders blank).
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
  `--user-data-dir=${profile}`, `--window-size=${width},${height}`, "about:blank"], { stdio: "ignore" });
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
await send("Emulation.setDeviceMetricsOverride", { width, height, deviceScaleFactor: 2, mobile: false });
await send("Emulation.setEmulatedMedia", { features: [{ name: "prefers-color-scheme", value: flag("--dark") ? "dark" : "light" }] });
await send("Page.enable");
await send("Page.navigate", { url });
await sleep(wait);
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
const hover = opt("--hover");
if (hover) { const [x, y] = hover.split(",").map(Number); await send("Input.dispatchMouseEvent", { type: "mouseMoved", x, y }); await sleep(500); }
const frames = Number(opt("--frames", "0"));
const capture = async (path) => {
  const shot = await send("Page.captureScreenshot", { format: "png" });
  writeFileSync(path, Buffer.from(shot.result.data, "base64"));
  console.log(`wrote ${path}`);
};
if (frames > 0) {
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
ws.close();
proc.kill();
