// The home page's hero demo: a "New invoice" sheet on the stage that fills
// itself in, on a loop. It types a customer's name and picks the match,
// picks a due date, slides the amount up, sends, and a note confirms; then
// the theme moves on (the next color preset, another radius) and it starts
// over, so a visitor sees the same components restyled without doing a
// thing. Every step is the components' real state, set from timers: a
// press, a drag or a pick by the visitor, or any control under the sheet,
// stops the loop and leaves everything live. It does not play while the
// user asks for less motion, nor while the stage is scrolled out of view.
//
// The sheet and the note are the library's components inside a
// `ThemeScope`, which carries the radius and the light or dark look the
// controls set; the color is the gallery's preset, as everywhere, though the
// loop's is only shown, not saved, until the visitor takes over (`sync.ts`).

import { batch, effect, type JSX, reduceMotion, signal, Text, untracked, View } from "nsh";
import {
  Button,
  Card,
  CardContent,
  CardDescription,
  CardFooter,
  CardHeader,
  CardTitle,
  Combobox,
  DatePicker,
  Label,
  type SelectItem,
  Slider,
  Switch,
  ThemeScope,
} from "nsh-longbridge";
import { CircleCheck, Moon, Pause, Play, Sun } from "nsh-icon-lucide";
import { adoptTheme, showPreset } from "../sync";
import { colorPreset, colorPresets, cssColor, scheme } from "../theme";
import { Glyph } from "./icons";
import { AmountsCard, CollectedCard, Floating, type Pointer, PresenceCard } from "./hero-cards";
import { COLLECTED, DUE, heroLayout, money, OVERLAP } from "./hero-data";
import { floatShadow, stageFill, windowShadow } from "./stage";
import { keyboardUsed } from "./screen";
import { PresetSwatches } from "./theme-picker";

const customers: SelectItem[] = [
  { value: "harbor", title: "Harbor & Pine" },
  { value: "halden", title: "Halden Studio" },
  { value: "lumen", title: "Lumen Labs" },
  { value: "northwind", title: "Northwind Rail" },
  { value: "kestrel", title: "Kestrel Health" },
  { value: "orbit", title: "Orbit Freight" },
];

/** The radii the loop and the controls step through: sharp, the default, round. */
const radii = [
  { label: "Sharp corners", value: "0.25rem", glyph: 2 },
  { label: "Default corners", value: "0.625rem", glyph: 4.5 },
  { label: "Round corners", value: "1rem", glyph: 7 },
];

const months = ["Jan", "Feb", "Mar", "Apr", "May", "Jun", "Jul", "Aug", "Sep", "Oct", "Nov", "Dec"];
const shortDate = (iso: string) => `${months[Number(iso.slice(5, 7)) - 1]} ${Number(iso.slice(8, 10))}`;

const START_AMOUNT = 1200;
const END_AMOUNT = 3400;

/** One run of the loop, in milliseconds from its start. */
const CYCLE = 9000;

/**
 * The demo, `width` points wide; `inView` says whether the stage is on
 * screen; `bare`: without the stage's backdrop and corners (an embed: the
 * website draws them); `pointer`: where the pointer is over the stage, each
 * axis -1 to 1 (the cards around the sheet lean from it).
 */
export function HeroDemo(props: { width: number; inView: boolean; bare?: boolean; pointer?: Pointer }) {
  // The sheet's state, which the loop and the visitor both set.
  const query = signal("");
  const open = signal(false);
  const customer = signal<string[]>([]);
  const due = signal<string | null>(null);
  const dueOpen = signal(false);
  const amount = signal(START_AMOUNT);
  const email = signal(true);
  const sent = signal(false);
  const sending = signal(false);
  const radius = signal(1);
  const look = signal<"light" | "dark" | null>(null);
  // Each run's combobox is a new one: one cleared from outside keeps the text it showed.
  const run = signal(0);
  // The cards around the sheet: who has it open, the month's collections, and whether each is shown.
  const viewing = signal(2);
  const collected = signal(COLLECTED);
  const shown = signal([true, true, true]);
  // The loop clears the calendar's side a moment before it opens it.
  const makeWay = signal(false);

  const playing = signal(!untracked(reduceMotion) && !untracked(keyboardUsed));
  let ownPreset = untracked(colorPreset);
  let noteTimer: number | undefined;

  /** The visitor took over: the loop stops where it is, and closes what it opened. */
  const takeOver = () => {
    if (!playing()) return;
    adoptTheme();
    batch(() => {
      playing.set(false);
      open.set(false);
      dueOpen.set(false);
      makeWay.set(false);
      shown.set([true, true, true]);
    });
  };

  const reset = () =>
    batch(() => {
      run.set(run() + 1);
      query.set("");
      open.set(false);
      customer.set([]);
      due.set(null);
      dueOpen.set(false);
      amount.set(START_AMOUNT);
      sent.set(false);
      sending.set(false);
      viewing.set(2);
      collected.set(COLLECTED);
    });

  const showNote = (ms: number) => {
    clearTimeout(noteTimer);
    sent.set(true);
    collected.set(Math.min(DUE, collected() + amount()));
    noteTimer = setTimeout(() => sent.set(false), ms);
  };

  // Not playing from the start (less motion, the keyboard in use): the sheet as the loop leaves it.
  if (!untracked(playing)) {
    customer.set(["harbor"]);
    due.set("2026-12-12");
    amount.set(END_AMOUNT);
    viewing.set(3);
    collected.set(COLLECTED + END_AMOUNT);
  }

  // A key pressed anywhere stops it: the loop's popups take the focus as they open.
  effect(() => {
    if (keyboardUsed()) untracked(takeOver);
  });

  // A choice of preset made anywhere else (the header's Theme menu) stops the loop too.
  effect(() => {
    const p = colorPreset();
    if (p !== ownPreset) untracked(takeOver);
  });

  // The loop, while it plays and the stage is in view.
  effect(() => {
    if (!playing() || !props.inView) return;
    const timers: number[] = [];
    const at = (ms: number, step: () => void) => timers.push(setTimeout(step, ms));
    const loop = () => {
      timers.length = 0;
      reset();
      // The cards come back one after another (from the second run: the first starts with them).
      [0, 1, 2].forEach((i) => at(250 + i * 110, () => shown.set(shown().map((v, j) => (j === i ? true : v)))));
      at(500, () => open.set(true));
      ["H", "Ha", "Har"].forEach((q, i) => at(800 + i * 170, () => query.set(q)));
      at(1700, () => batch(() => (customer.set(["harbor"]), open.set(false), query.set(""))));
      at(2000, () => viewing.set(3));
      at(2050, () => makeWay.set(true));
      at(2300, () => dueOpen.set(true));
      at(3200, () => due.set("2026-12-12"));
      at(3500, () => batch(() => (dueOpen.set(false), makeWay.set(false))));
      for (let i = 1; i <= 8; i++) at(3900 + i * 70, () => amount.set(START_AMOUNT + ((END_AMOUNT - START_AMOUNT) * i) / 8));
      at(5000, () => sending.set(true));
      at(5200, () => batch(() => (sending.set(false), showNote(2600))));
      // The cards leave before the theme moves on, and come back in it.
      [0, 1, 2].forEach((i) => at(7650 + i * 110, () => shown.set(shown().map((v, j) => (j === i ? false : v)))));
      at(8100, () =>
        batch(() => {
          const i = colorPresets.findIndex((p) => p.id === colorPreset());
          ownPreset = colorPresets[(i + 1) % colorPresets.length].id;
          showPreset(ownPreset);
          radius.set((radius() + 1) % radii.length);
        }),
      );
      at(CYCLE, loop);
    };
    untracked(loop);
    return () => {
      for (const t of timers) clearTimeout(t);
      untracked(() => batch(() => (open.set(false), dueOpen.set(false), makeWay.set(false))));
    };
  });

  const layout = () => heroLayout(props.width);
  const phone = () => layout().phone;
  const sheetWidth = () => layout().sheet;
  const cards = () => layout().cards;
  // The calendar opens beside the sheet, on its left: the cards there make way.
  const leftShown = (i: number) => shown()[i] && !((dueOpen() || makeWay()) && room());
  const matches = () => customers.filter((c) => c.title.toLowerCase().includes(query().toLowerCase()));
  // Beside the sheet where the stage has room for a popup (the calendar, the note), over it where not.
  const room = () => (props.width - sheetWidth()) / 2 - 32 >= 300;
  // What the loop types shows where the field's placeholder is, in the text's color: an input
  // shows its query only while it has the focus, and the loop never takes the visitor's.
  const typing = () => playing() && open() && query() !== "";
  const customerName = () => customers.find((c) => c.value === customer()[0])?.title ?? "the customer";

  return (
    <View
      class="items-center overflow-hidden"
      style={{
        borderRadius: props.bare ? 0 : phone() ? 20 : 28,
        paddingTop: phone() ? 24 : 40,
        paddingBottom: phone() ? 20 : 28,
        paddingHorizontal: 12,
        ...(props.bare ? {} : stageFill()),
      }}
    >
      <ThemeScope tokens={{ radius: radii[radius()].value }} appearance={look()} class="relative mb-5">
        <Card style={{ width: sheetWidth(), boxShadow: windowShadow(), transitionProperty: "all", transitionDuration: 250 }}>
          <CardHeader>
            <CardTitle>
              <Text>New invoice</Text>
            </CardTitle>
            <CardDescription>
              <Text>INV-2042, billed from Northwind Studio</Text>
            </CardDescription>
          </CardHeader>
          <CardContent class="gap-5">
            <View class="gap-2">
              <Label>
                <Text>Customer</Text>
              </Label>
              <Each of={[run()]}>
                {() => (
                  <Combobox
                    items={matches()}
                    value={customer()}
                    onChange={(e) => (takeOver(), customer.set(e.value))}
                    query={query()}
                    onQueryChange={(e) => (takeOver(), query.set(e.query))}
                    open={open()}
                    onOpenChange={(e) => (takeOver(), open.set(e.open))}
                    placeholder={typing() ? query() : "Search customers"}
                    inputStyle={typing() ? { placeholderColor: cssColor("foreground") } : undefined}
                    accessibilityLabel="Customer"
                  />
                )}
              </Each>
            </View>
            <View class={phone() ? "gap-5" : "flex-row gap-4"}>
              <View class="flex-1 gap-2">
                <Label>
                  <Text>Due date</Text>
                </Label>
                <DatePicker
                  class="w-full"
                  value={due()}
                  onChange={(e) => (takeOver(), due.set(e.value as string | null))}
                  open={dueOpen()}
                  onOpenChange={(e) => (takeOver(), dueOpen.set(e.open))}
                  placement={room() ? "left" : "bottom"}
                  align={room() ? "center" : "start"}
                  placeholder="Pick a date"
                  accessibilityLabel="Due date"
                />
              </View>
              <View class="flex-1 gap-2">
                <View class="flex-row items-center justify-between">
                  <Label>
                    <Text>Amount</Text>
                  </Label>
                  <Label class="tabular-nums">
                    <Text>{money(amount())}</Text>
                  </Label>
                </View>
                <View class="h-8 justify-center">
                  <Slider
                    min={0}
                    max={6000}
                    step={50}
                    value={amount()}
                    onChange={(e) => (takeOver(), amount.set(e.value as number))}
                    accessibilityLabel="Amount"
                  />
                </View>
              </View>
            </View>
            <View class="flex-row items-center justify-between gap-4">
              <Label>
                <Text>Email it to the customer</Text>
              </Label>
              <Switch checked={email()} onChange={(e) => (takeOver(), email.set(e.checked))} accessibilityLabel="Email it to the customer" />
            </View>
          </CardContent>
          <CardFooter class="justify-end gap-2">
            <Button variant="outline" onPress={() => (takeOver(), reset())}>
              <Text>Cancel</Text>
            </Button>
            <Button
              loading={sending()}
              disabled={customer().length === 0}
              onPress={() => (takeOver(), showNote(3200))}
            >
              <Text>Send invoice</Text>
            </Button>
          </CardFooter>
        </Card>

        {/* Around the sheet, over its edges: who has it open, the amount among the quarter's, the month's collections. */}
        <If when={cards() >= 2}>
          <Floating place={{ right: sheetWidth() - OVERLAP, top: 24 }} shown={leftShown(0)} from={[-14, 0]} depth={6} pointer={props.pointer ?? null}>
            <PresenceCard count={viewing()} />
          </Floating>
          <Floating place={{ left: sheetWidth() - OVERLAP, top: 52 }} shown={shown()[1]} from={[14, 0]} depth={9} pointer={props.pointer ?? null}>
            <AmountsCard
              width={cards() === 3 ? 280 : Math.min(260, layout().side - 8)}
              amount={amount()}
              onAmount={(v) => (takeOver(), amount.set(v))}
            />
          </Floating>
        </If>
        <If when={cards() === 3}>
          <Floating place={{ right: sheetWidth() - OVERLAP, bottom: 36 }} shown={leftShown(2)} from={[-14, 0]} depth={7} pointer={props.pointer ?? null}>
            <CollectedCard width={256} collected={collected()} />
          </Floating>
        </If>

        {/* The note the send leaves: over the sheet's foot at its end, or over the sheet. */}
        <View
          class="absolute"
          style={cards() === 3 ? { left: sheetWidth() - 16, bottom: 14 } : { left: 0, right: 0, bottom: 72, alignItems: "center" }}
          accessibilityElementsHidden={!sent()}
        >
          <Card
            class="flex-row items-center gap-3 py-3 px-4"
            style={{
              width: Math.min(300, sheetWidth() - 24),
              boxShadow: floatShadow,
              opacity: sent() ? 1 : 0,
              translate: [0, sent() ? 0 : 10],
              transitionProperty: "opacity, translate",
              transitionDuration: 220,
              transitionTimingFunction: "ease-out",
            }}
          >
            <Glyph icon={CircleCheck} size={18} class="text-primary" />
            <View class="flex-1 min-w-0 gap-0.5">
              <CardTitle class="text-sm">
                <Text>{`Invoice sent to ${customerName()}`}</Text>
              </CardTitle>
              <CardDescription class="text-xs">
                <Text>{`${money(amount())}${due() ? `, due ${shortDate(due()!)}` : ""}`}</Text>
              </CardDescription>
            </View>
          </Card>
        </View>
      </ThemeScope>

      <Controls
        radius={radius()}
        onRadius={(i) => (takeOver(), radius.set(i))}
        look={look() ?? scheme()}
        onLook={(l) => (takeOver(), look.set(l === scheme() ? null : l))}
        playing={playing()}
        onPlay={() => {
          if (playing()) takeOver();
          else {
            ownPreset = colorPreset();
            playing.set(true);
          }
        }}
        phone={phone()}
      />
    </View>
  );
}

// ---------------------------------------------------------------- the controls

/** A square button on the stage's glass, lit while `on`. */
function GlassButton(props: { on?: boolean; label: string; onPress: () => void; children?: JSX.Element }) {
  return (
    <View
      class="size-7 items-center justify-center rounded-full"
      style={{ backgroundColor: props.on ? "#ffffff2e" : "transparent", cursor: "default" }}
      hoverStyle={props.on ? undefined : { backgroundColor: "#ffffff14" }}
      role="button"
      accessibilityLabel={props.label}
      accessibilityState={props.on === undefined ? undefined : { selected: props.on }}
      onPress={props.onPress}
    >
      {props.children}
    </View>
  );
}

const Divider = () => <View style={{ width: 1, height: 18, backgroundColor: "#ffffff26" }} />;

/** Under the sheet, on the glass: the color presets, the radius, light or dark, and the loop's play / pause. */
function Controls(props: {
  radius: number;
  onRadius: (i: number) => void;
  look: "light" | "dark";
  onLook: (l: "light" | "dark") => void;
  playing: boolean;
  onPlay: () => void;
  phone: boolean;
}) {
  const rest = () => (
    <View class="flex-row items-center gap-2">
      <View class="flex-row items-center gap-0.5" role="radiogroup" accessibilityLabel="Corners">
        {radii.map((r, i) => (
          <GlassButton on={props.radius === i} label={r.label} onPress={() => props.onRadius(i)}>
            <View style={{ width: 13, height: 13, borderWidth: 1.5, borderColor: "#ffffff", borderRadius: r.glyph }} />
          </GlassButton>
        ))}
      </View>
      <Divider />
      <View class="flex-row items-center gap-0.5" role="radiogroup" accessibilityLabel="Appearance of the demo">
        <GlassButton on={props.look === "light"} label="Light" onPress={() => props.onLook("light")}>
          <Glyph icon={Sun} size={14} class="text-white" />
        </GlassButton>
        <GlassButton on={props.look === "dark"} label="Dark" onPress={() => props.onLook("dark")}>
          <Glyph icon={Moon} size={14} class="text-white" />
        </GlassButton>
      </View>
      <Divider />
      <GlassButton label={props.playing ? "Pause the demo" : "Play the demo"} onPress={props.onPlay}>
        <Glyph icon={props.playing ? Pause : Play} size={13} class="text-white" />
      </GlassButton>
    </View>
  );
  return (
    <View
      class={props.phone ? "items-center gap-2 rounded-2xl px-3 py-2" : "flex-row items-center gap-3 rounded-full py-1.5 pl-3 pr-1.5"}
      style={{ backgroundColor: "#ffffff14", boxShadow: "inset 0 0 0 1px #ffffff1f", backdropFilter: "blur(12px)" }}
    >
      <PresetSwatches tone="stage" size={18} gap={8} />
      <If when={!props.phone}>
        <Divider />
      </If>
      {rest()}
    </View>
  );
}
