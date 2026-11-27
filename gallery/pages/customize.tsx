// The theme studio: a control panel (presets, density, the seeds, every
// semantic color, the file's copy and reset) beside a live preview on the
// stage, then the theme file with how to use it, then ThemeScope. Wide
// windows keep the panel in place while the preview scrolls; narrow ones
// stack them.

import { Clipboard, Scroll, signal, Text, View, windowSize, type JSX } from "nsh";
import { Badge, Button, Separator, Slider, Tabs, TabsList, TabsTrigger, ThemeScope } from "nsh-longbridge";
import { ChevronDown, ChevronRight, FileText } from "nsh-icon-lucide";
import {
  appearance,
  colorGroups,
  cssColor,
  editCount,
  formatValue,
  isEdited,
  numberOf,
  resetAll,
  resetToken,
  scheme,
  sectionOf,
  seeds,
  setSeed,
  setToken,
  themeJson,
} from "../theme";
import { ColorEdit } from "../ui/color-edit";
import { Check, Copy, Glyph, RotateCcw } from "../ui/icons";
import { NotificationsCard, PlanCard } from "../ui/showcase";
import { Code as Source } from "../ui/code";
import { DISPLAY, gutter, site } from "../ui/site";
import { cardStyle, ink, Kicker } from "../ui/site-ui";
import { Stage } from "../ui/stage";
import { densityLabel, DensityTiles, presetLabel, PresetSwatches } from "../ui/theme-picker";

/** The colors a scope demo sets: a green brand and square corners. */
const brandScope = {
  primary: "oklch(0.596 0.145 163.225)",
  "primary-foreground": "oklch(0.985 0 0)",
  ring: "oklch(0.696 0.17 162.48)",
  radius: "2px",
};

const PANEL = 372;

/** The panel beside the preview (wide windows) or above it (narrow ones). */
const split = () => windowSize().width >= 1080;

/** The width the preview column has. */
const mainWidth = () =>
  split() ? Math.min(1120, windowSize().width - PANEL - 2 * 56) : Math.min(1200, windowSize().width - 2 * gutter());

const phone = () => mainWidth() < 600;

// ---------------------------------------------------------------- the file

const copied = signal(false);

async function copyTheme() {
  await Clipboard.writeText(themeJson());
  copied.set(true);
  setTimeout(() => copied.set(false), 1600);
}

const changes = () => (editCount() === 0 ? "No changes" : `${editCount()} ${editCount() === 1 ? "change" : "changes"}`);

function CopyButton(props: { size?: "sm" | "md"; class?: string }) {
  return (
    <Button size={props.size ?? "sm"} class={props.class} onPress={copyTheme}>
      <Glyph icon={copied() ? Check : Copy} size={14} />
      <Text>{copied() ? "Copied" : "Copy theme"}</Text>
    </Button>
  );
}

function ResetButton(props: { size?: "sm" | "md" }) {
  return (
    <Button size={props.size ?? "sm"} variant="outline" disabled={editCount() === 0} onPress={resetAll}>
      <Glyph icon={RotateCcw} size={13} />
      <Text>Reset</Text>
    </Button>
  );
}

// ---------------------------------------------------------------- the panel

function PanelSection(props: { title: string; aside?: JSX.Element; note?: string; children?: JSX.Element; last?: boolean }) {
  return (
    <View
      class="gap-4 px-6 pt-5 pb-6"
      style={{ borderBottomWidth: props.last ? 0 : 1, borderColor: site().line }}
    >
      <View class="gap-1">
        <View class="min-h-6 flex-row items-center justify-between gap-3">
          <Kicker quiet>{props.title}</Kicker>
          {props.aside}
        </View>
        <If when={props.note}>
          <Text class="text-xs leading-5" style={{ color: site().muted }}>{props.note!}</Text>
        </If>
      </View>
      {props.children}
    </View>
  );
}

/** The panel's head: the file, its changes, Copy and Reset. */
function FileBar() {
  return (
    <View class="gap-3.5 px-6 pt-5 pb-5" style={{ borderBottomWidth: 1, borderColor: site().line }}>
      <View class="flex-row items-center justify-between">
        <View class="flex-row items-center gap-2">
          <Glyph icon={FileText} size={14} class={ink(site().muted)} />
          <Text class="font-mono text-[13px]" style={{ color: site().ink }}>my-theme.json</Text>
        </View>
        <Badge variant={editCount() > 0 ? "default" : "secondary"}>
          <Text>{changes()}</Text>
        </Badge>
      </View>
      <View class="flex-row gap-2">
        <CopyButton class="flex-1" />
        <ResetButton />
      </View>
    </View>
  );
}

function Seeds() {
  return (
    <View class="gap-5">
      {seeds.map((s) => (
        <View class="gap-3">
          <View class="h-5 flex-row items-center justify-between">
            <View class="flex-row items-center gap-1.5">
              <Text class="text-sm" style={{ color: site().ink }}>{s.label}</Text>
              <If when={isEdited(s.name)}>
                <Button
                  variant="ghost"
                  size="xs"
                  class="h-5 w-5 px-0"
                  accessibilityLabel={`Reset ${s.label}`}
                  onPress={() => resetToken(s.name)}
                >
                  <Glyph icon={RotateCcw} size={12} />
                </Button>
              </If>
            </View>
            <Text class="font-mono text-xs tabular-nums" style={{ color: site().muted }}>{formatValue(s.name)}</Text>
          </View>
          <Slider
            accessibilityLabel={s.label}
            min={s.min}
            max={s.max}
            step={s.step}
            value={numberOf(s.name)}
            onChange={({ value }) => setSeed(s.name, value as number)}
          />
        </View>
      ))}
    </View>
  );
}

/** Light or dark: the appearance whose colors the panel edits (it also switches the page). */
function AppearanceTabs() {
  return (
    <Tabs value={scheme()} onChange={(e) => appearance.set(e.value as "light" | "dark")}>
      <TabsList size="xs" accessibilityLabel="Appearance to edit">
        <TabsTrigger value="light">
          <Text>Light</Text>
        </TabsTrigger>
        <TabsTrigger value="dark">
          <Text>Dark</Text>
        </TabsTrigger>
      </TabsList>
    </Tabs>
  );
}

/** A few of a group's colors, overlapping, ringed in the panel's color. */
function SwatchStack(props: { names: string[] }) {
  return (
    <View class="flex-row">
      {props.names.slice(0, 5).map((name, i) => (
        <View
          class="size-3.5 rounded-full"
          style={{
            marginLeft: i === 0 ? 0 : -4,
            backgroundColor: cssColor(name),
            boxShadow: `0 0 0 1.5px ${site().sidebar}, inset 0 0 0 1px ${scheme() === "dark" ? "#ffffff1f" : "#0000001a"}`,
          }}
        />
      ))}
    </View>
  );
}

/** The semantic colors in folding groups, each row a color and the text on it. */
function ColorGroups(props: { open: () => string[]; toggle: (title: string) => void }) {
  return (
    <View class="gap-2">
      {colorGroups.map((g) => {
        const isOpen = () => props.open().includes(g.title);
        const names = g.pairs.flatMap(([a, b]) => (b ? [a, b] : [a]));
        return (
          <View
            class="overflow-hidden rounded-xl"
            style={{
              backgroundColor: isOpen() ? site().card : "transparent",
              boxShadow: `inset 0 0 0 1px ${site().line}`,
            }}
          >
            <View
              class="h-11 flex-row items-center gap-2.5 px-3 hover:bg-foreground/[0.03]"
              style={{ cursor: "default" }}
              role="button"
              accessibilityLabel={g.title}
              accessibilityState={{ expanded: isOpen() }}
              onPress={() => props.toggle(g.title)}
            >
              <Glyph icon={isOpen() ? ChevronDown : ChevronRight} size={14} class={ink(site().muted)} />
              <Text class="flex-1 text-sm" style={{ color: site().ink }}>{g.title}</Text>
              <If when={names.some(isEdited)}>
                <View class="size-1.5 rounded-full bg-primary" accessibilityLabel="Edited" />
              </If>
              <Text class="w-5 text-right font-mono text-[11px] tabular-nums" style={{ color: site().muted }}>
                {String(g.pairs.length)}
              </Text>
              <SwatchStack names={g.pairs.map(([fill]) => fill)} />
            </View>
            <If when={isOpen()}>
              <View style={{ borderTopWidth: 1, borderColor: site().line }}>
                {g.pairs.map(([fill, text], i) => (
                  <View>
                    <If when={i > 0}>
                      <Separator />
                    </If>
                    <ColorRow fill={fill} text={text} />
                  </View>
                ))}
              </View>
            </If>
          </View>
        );
      })}
    </View>
  );
}

/** A color and the text that goes on it: a swatch each, the color's name and value. */
function ColorRow(props: { fill: string; text: string | null }) {
  const names = props.text ? [props.fill, props.text] : [props.fill];
  const edited = () => names.some(isEdited);
  return (
    <View class="h-11 flex-row items-center gap-3 px-3">
      <View class="flex-row gap-1.5">
        {names.map((name) => (
          <ColorEdit name={name} onChange={(v) => setToken(sectionOf(name), name, v)} />
        ))}
        <If when={!props.text}>
          <View class="size-6" />
        </If>
      </View>
      <View class="flex-1 min-w-0 flex-row items-baseline gap-1.5">
        <Text class="truncate font-mono text-xs" style={{ color: site().ink }}>{props.fill}</Text>
        <If when={props.text}>
          <Text class="font-mono text-[11px]" style={{ color: site().muted }}>+ text</Text>
        </If>
      </View>
      <If when={edited()}>
        <Button
          variant="ghost"
          size="xs"
          class="h-5 w-5 px-0"
          accessibilityLabel={`Reset ${props.fill}`}
          onPress={() => names.forEach(resetToken)}
        >
          <Glyph icon={RotateCcw} size={12} />
        </Button>
      </If>
      <Text class="font-mono text-[11px] tabular-nums" style={{ color: site().muted }}>{formatValue(props.fill)}</Text>
    </View>
  );
}

const openGroups = signal<string[]>([]);
const toggleGroup = (title: string) =>
  openGroups.set(openGroups().includes(title) ? openGroups().filter((t) => t !== title) : [...openGroups(), title]);

/** The controls, in sections: the file, the preset, the density, the seeds, the colors. */
function PanelBody() {
  return (
    <>
      <FileBar />
      <PanelSection title="Preset" aside={<Text class="text-xs" style={{ color: site().muted }}>{presetLabel()}</Text>}>
        <PresetSwatches spread />
      </PanelSection>
      <PanelSection title="Density" aside={<Text class="text-xs" style={{ color: site().muted }}>{densityLabel()}</Text>}>
        <DensityTiles width={72} />
      </PanelSection>
      <PanelSection title="Seeds" note="Each one moves every component in proportion.">
        <Seeds />
      </PanelSection>
      <PanelSection
        title="Colors"
        aside={<AppearanceTabs />}
        note="Every role, per appearance. A swatch opens its picker; the second is the text on it."
        last
      >
        <ColorGroups open={openGroups} toggle={toggleGroup} />
      </PanelSection>
    </>
  );
}

// ---------------------------------------------------------------- the preview column

function Heading(props: { eyebrow: string; title: string; lead: string; size?: number }) {
  const size = props.size ?? (phone() ? 30 : 40);
  return (
    <View class="gap-4" style={{ maxWidth: 640 }}>
      <Kicker>{props.eyebrow}</Kicker>
      <Text
        style={{ fontFamily: DISPLAY, color: site().ink, fontSize: size, lineHeight: Math.round(size * 1.1), fontWeight: "600", letterSpacing: -size * 0.03 }}
      >
        {props.title}
      </Text>
      <Text class="text-base leading-7" style={{ color: site().muted }}>{props.lead}</Text>
    </View>
  );
}

function Intro() {
  const size = () => (phone() ? 38 : 56);
  return (
    <View class="gap-5">
      <Kicker>Theme studio</Kicker>
      <Text
        style={{ fontFamily: DISPLAY, color: site().ink, fontSize: size(), lineHeight: Math.round(size() * 1.06), fontWeight: "650", letterSpacing: -size() * 0.032 }}
      >
        Make it yours.
      </Text>
      <Text style={{ color: site().muted, fontSize: phone() ? 16 : 17, lineHeight: phone() ? 25 : 27, maxWidth: 560 }}>
        Start from a preset, move the seeds, recolor any role. The preview is the library itself, so what you see here
        is what your app gets.
      </Text>
    </View>
  );
}

/** The cards that stand on the stage under the window. */
function StageCards(props: { width: number }) {
  return (
    <View class={props.width >= 560 ? "flex-row items-start gap-5" : "gap-4"}>
      <View class="flex-1 min-w-0">
        <PlanCard />
      </View>
      <View class="flex-1 min-w-0">
        <NotificationsCard />
      </View>
    </View>
  );
}

/** The theme file as it stands, keys quieter than their values. */
function Code(props: { text: string; maxHeight?: number }) {
  const lines = () => props.text.split("\n");
  return (
    <Scroll style={{ maxHeight: props.maxHeight ?? 440 }} contentClass="px-5 py-4">
      {lines().map((line) => {
        const m = /^(\s*)("[^"]*")(:\s?)?(.*)$/.exec(line);
        return m && m[3] ? (
          <View class="flex-row">
            <Text class="font-mono text-[12.5px] leading-[21px]" style={{ color: site().code.property }}>{`${m[1]}${m[2]}`}</Text>
            <Text class="font-mono text-[12.5px] leading-[21px]" style={{ color: site().code.punct }}>:</Text>
            <Text class="flex-1 font-mono text-[12.5px] leading-[21px]" style={{ color: /^"/.test(m[4]) ? site().code.string : site().code.number }}>{` ${m[4]}`}</Text>
          </View>
        ) : (
          <Text class="font-mono text-[12.5px] leading-[21px]" style={{ color: site().code.punct }}>{line || " "}</Text>
        );
      })}
    </Scroll>
  );
}

const nshJson = `{
  "name": "My App",
  "theme": "my-theme.json"
}`;

function Step(props: { n: string; children?: JSX.Element }) {
  return (
    <View class="flex-row gap-4">
      <Text class="w-5 pt-px font-mono text-xs" style={{ color: site().citrus }}>{props.n}</Text>
      <View class="flex-1 min-w-0 gap-3">{props.children}</View>
    </View>
  );
}

function Mono(props: { children: string }) {
  return <Text class="font-mono text-[13px]" style={{ color: site().ink }}>{props.children}</Text>;
}

function Export() {
  const wide = () => mainWidth() >= 860;
  const frame = () => cardStyle();
  return (
    <View class="gap-10">
      <Heading
        eyebrow="Export"
        title="Take it with you."
        lead="The preset you chose and every token you changed, as one theme file. Tokens you left alone stay out of it: they derive from the library's defaults."
      />
      <View class={wide() ? "flex-row items-start gap-10" : "gap-10"}>
        <View class="flex-1 min-w-0 overflow-hidden rounded-2xl" style={frame()}>
          <View
            class="h-12 flex-row items-center gap-3 pl-5 pr-2"
            style={{ borderBottomWidth: 1, borderColor: site().line }}
          >
            <Text class="flex-1 font-mono text-[13px]" style={{ color: site().ink }}>my-theme.json</Text>
            <Text class="text-xs" style={{ color: site().muted }}>{changes()}</Text>
            <Button size="xs" variant="ghost" disabled={editCount() === 0} onPress={resetAll}>
              <Text>Reset</Text>
            </Button>
            <Button size="xs" variant="outline" onPress={copyTheme}>
              <Glyph icon={copied() ? Check : Copy} size={12} />
              <Text>{copied() ? "Copied" : "Copy"}</Text>
            </Button>
          </View>
          <View style={{ backgroundColor: site().code.bg }}>
            <Code text={themeJson()} />
          </View>
        </View>
        <View class="gap-6" style={wide() ? { width: 320 } : undefined}>
          <Step n="01">
            <Text class="text-sm leading-6" style={{ color: site().ink }}>Save the file beside your app's nsh.json.</Text>
          </Step>
          <Step n="02">
            <Text class="text-sm leading-6" style={{ color: site().ink }}>
              Name it in nsh.json's "theme", as a path from nsh.json:
            </Text>
            <View class="overflow-hidden px-4 py-3" style={{ ...frame(), borderRadius: 12, backgroundColor: site().code.bg }}>
              <Source text={nshJson} size={12} numbers={false} />
            </View>
          </Step>
          <Step n="03">
            <Text class="text-sm leading-6" style={{ color: site().ink }}>
              Every component takes it, and what derives from a token follows. For one part of a screen, use a
              ThemeScope instead.
            </Text>
          </Step>
        </View>
      </View>
    </View>
  );
}

function ScopeCell(props: { title: string; line: string; children?: JSX.Element }) {
  return (
    <View class="flex-1 min-w-0 overflow-hidden" style={cardStyle()}>
      <View class="gap-1 px-6 pt-5 pb-5">
        <Text style={{ color: site().ink, fontSize: 14, fontWeight: "600" }}>{props.title}</Text>
        <Text style={{ color: site().muted, fontSize: 13.5 }}>{props.line}</Text>
      </View>
      <View
        class="flex-1 items-center"
        style={{ backgroundColor: cssColor("muted"), borderTopWidth: 1, borderColor: site().line, paddingVertical: 24, paddingHorizontal: phone() ? 12 : 24 }}
      >
        {props.children}
      </View>
    </View>
  );
}

function Scopes() {
  return (
    <View class="gap-10">
      <Heading
        eyebrow="ThemeScope"
        title="Or theme part of a screen."
        lead="Tokens for one part of a screen. The library's components inside take the scope's values, and what derives from them follows; scopes nest, the inner one winning."
      />
      <View class={mainWidth() >= 760 ? "flex-row gap-4" : "gap-4"}>
        <ScopeCell title="Its own appearance" line="The dark appearance, with its own surface.">
          <ThemeScope appearance="dark" surface class="w-full max-w-[380px] rounded-xl p-4">
            <NotificationsCard />
          </ThemeScope>
        </ScopeCell>
        <ScopeCell title="Its own brand" line="A green primary and a radius of 2, nothing else.">
          <ThemeScope tokens={brandScope} class="w-full max-w-[380px]">
            <PlanCard />
          </ThemeScope>
        </ScopeCell>
      </View>
    </View>
  );
}

function PreviewColumn() {
  const gap = () => (phone() ? 72 : 112);
  return (
    <View style={{ width: mainWidth(), gap: gap() }}>
      <Stage width={mainWidth()} presets={false} below={<StageCards width={mainWidth()} />} />
      <Export />
      <Scopes />
    </View>
  );
}

export function CustomizePage() {
  return (
    <>
      {split() ? (
        <View class="flex-1 flex-row">
          <View style={{ width: PANEL, borderRightWidth: 1, borderColor: site().line, backgroundColor: site().sidebar }}>
            <Scroll class="flex-1">
              <PanelBody />
            </Scroll>
          </View>
          <Scroll class="flex-1" contentClass="items-center">
            <View style={{ width: mainWidth(), paddingTop: 72, paddingBottom: 96, gap: 56 }}>
              <Intro />
              <PreviewColumn />
            </View>
          </Scroll>
        </View>
      ) : (
        <Scroll class="flex-1" contentClass="items-center">
          <View style={{ width: mainWidth(), paddingTop: phone() ? 40 : 64, paddingBottom: 72, gap: phone() ? 40 : 56 }}>
            <Intro />
            <View class="overflow-hidden" style={cardStyle()}>
              <PanelBody />
            </View>
            <PreviewColumn />
          </View>
        </Scroll>
      )}
    </>
  );
}
