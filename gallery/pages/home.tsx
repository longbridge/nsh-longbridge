// The home page, the library's front door, kept quiet: a sentence, one
// button and the install command over a stage where a "New invoice" sheet
// fills itself in and the theme moves on (ui/hero-demo.tsx), which a visitor
// can take over at any point. Then three short sections, each one live demo:
// a few tokens restyling a screen, a component's TSX beside it running, and
// the components as a compact list. The site's chrome keeps its own colors
// (a page of the NSH family); everything shown running follows the theme.

import { Clipboard, openUrl, Scroll, signal, Text, View, type JSX } from "nsh";
import { componentPages } from "../catalog";
import { navigate } from "../router";
import { Logo } from "../ui/brand";
import { Code } from "../ui/code";
import { HeroDemo } from "../ui/hero-demo";
import { Check, Copy, Glyph } from "../ui/icons";
import { INVITE_FILE, INVITE_SOURCE, InviteCard } from "../ui/sample";
import { pageScroll } from "../ui/screen";
import { DISPLAY, gutter, NSH_SITE, REPOSITORY, site, siteWidth } from "../ui/site";
import { cardStyle, ink, SiteButton } from "../ui/site-ui";
import { TokenDemo } from "../ui/token-demo";
import { cssColor } from "../theme";

const INSTALL = "nsh add github:longbridge/nsh-longbridge";

const phone = () => siteWidth() < 600;

/** A section of the page: the site's column, a lot of air around it. */
function Band(props: { children?: JSX.Element }) {
  return (
    <View
      class="items-center"
      style={{ backgroundColor: site().ground, paddingTop: phone() ? 72 : 128, paddingHorizontal: gutter() }}
    >
      <View style={{ width: siteWidth() }}>{props.children}</View>
    </View>
  );
}

/** A section's heading: one short line, centered, and one sentence under it. */
function Heading(props: { title: string; line: string }) {
  const size = () => (phone() ? 30 : 40);
  return (
    <View class="items-center gap-3" style={{ marginBottom: phone() ? 32 : 48 }}>
      <Text
        class="text-center"
        style={{ fontFamily: DISPLAY, color: site().ink, fontSize: size(), lineHeight: Math.round(size() * 1.12), fontWeight: "650", letterSpacing: -size() * 0.03 }}
      >
        {props.title}
      </Text>
      <Text class="text-center" style={{ color: site().muted, fontSize: phone() ? 15.5 : 17, lineHeight: phone() ? 24 : 27, maxWidth: 560 }}>
        {props.line}
      </Text>
    </View>
  );
}

// ---------------------------------------------------------------- hero

function InstallCommand() {
  const copied = signal(false);
  const copy = async () => {
    await Clipboard.writeText(INSTALL);
    copied.set(true);
    setTimeout(() => copied.set(false), 1600);
  };
  const s = () => site();
  return (
    <View
      class="h-10 min-w-0 flex-row items-center gap-3 pl-3.5 pr-1"
      style={{ borderRadius: 10, backgroundColor: s().inset, boxShadow: `${s().insetShadow}, inset 0 0 0 1px ${s().line}` }}
    >
      <Text class="font-mono" style={{ color: s().citrus, fontSize: 13 }}>
        $
      </Text>
      <Text class="flex-1 truncate font-mono" style={{ color: s().ink, fontSize: phone() ? 11.5 : 13, letterSpacing: phone() ? -0.3 : 0 }}>
        {INSTALL}
      </Text>
      <View
        class="size-8 items-center justify-center"
        style={{ borderRadius: 7, cursor: "default" }}
        hoverStyle={{ backgroundColor: s().hover }}
        role="button"
        accessibilityLabel={copied() ? "Copied" : "Copy the command"}
        onPress={copy}
      >
        <Glyph icon={copied() ? Check : Copy} size={14} class={ink(copied() ? s().citrus : s().muted)} />
      </View>
    </View>
  );
}

function Hero() {
  // The loop plays while the stage is on screen: up to its last 160 points.
  const bottom = signal(10_000);
  const size = () => (siteWidth() >= 1100 ? 58 : phone() ? 38 : 50);
  return (
    <View
      class="items-center"
      onLayout={(e) => bottom.set(e.bounds.origin.y + e.bounds.size.height)}
      style={{
        backgroundImage: `linear-gradient(180deg, ${site().heroTop}, ${site().heroBottom})`,
        backgroundColor: site().ground,
        paddingHorizontal: gutter(),
        paddingTop: phone() ? 40 : 56,
      }}
    >
      <View class="items-center" style={{ width: siteWidth() }}>
        <Text
          class="text-center"
          style={{
            fontFamily: DISPLAY,
            color: site().ink,
            fontSize: size(),
            lineHeight: Math.round(size() * 1.06),
            fontWeight: "650",
            letterSpacing: -size() * 0.032,
            maxWidth: phone() ? undefined : size() * 13,
          }}
        >
          {"Native apps that look finished from day\u00a0one."}
        </Text>
        <Text
          class="text-center"
          style={{ color: site().muted, fontSize: phone() ? 16 : 18, lineHeight: phone() ? 25 : 28, maxWidth: 560, marginTop: phone() ? 16 : 18 }}
        >
          {`${componentPages.length} components for NSH apps, in TSX or Rust. One theme file restyles every one of them.`}
        </Text>
        <View
          class={phone() ? "w-full gap-3" : "flex-row items-center gap-3"}
          style={{ marginTop: phone() ? 24 : 28 }}
        >
          <SiteButton label="Browse components" onPress={() => navigate(componentPages[0].id)} role="link" />
          <View style={phone() ? undefined : { width: 404 }}>
            <InstallCommand />
          </View>
        </View>
        <View class="w-full" style={{ marginTop: phone() ? 32 : 44 }}>
          <HeroDemo width={siteWidth()} inView={pageScroll() < bottom() - 160} />
        </View>
      </View>
    </View>
  );
}

// ---------------------------------------------------------------- code beside its result

/** The traffic lights of a macOS window, which read the same in both appearances. */
function Lights() {
  return (
    <View class="flex-row gap-2">
      {["#ff5f57", "#febc2e", "#28c840"].map((c) => (
        <View class="size-3 rounded-full" style={{ backgroundColor: c, boxShadow: "inset 0 0 0 0.5px #00000026" }} />
      ))}
    </View>
  );
}

/** A label over a pane: what it shows, and a citrus dot when it is running. */
function PaneLabel(props: { children: string; live?: boolean }) {
  return (
    <View class="h-10 flex-row items-center gap-2 px-5" style={{ borderBottomWidth: 1, borderColor: site().line }}>
      <If when={props.live}>
        <View class="size-1.5 rounded-full" style={{ backgroundColor: site().citrusFill, boxShadow: `0 0 0 3px ${site().citrusFill}33` }} />
      </If>
      <Text class="font-mono" style={{ color: site().faint, fontSize: 10.5, letterSpacing: 1.4 }}>
        {props.children.toUpperCase()}
      </Text>
    </View>
  );
}

function CodeAndResult() {
  const split = () => siteWidth() >= 980;
  // The result stands on the theme's own background, a faint blueprint grid on it.
  const grid = () => `${cssColor("border")}66`;
  const preview = () => (
    <View class="flex-1 min-w-0">
      <PaneLabel live>Running</PaneLabel>
      <View
        class="flex-1 items-center justify-center"
        style={{
          backgroundColor: cssColor("muted"),
          backgroundImage: [
            `repeating-linear-gradient(90deg, ${grid()} 0px, ${grid()} 1px, transparent 1px, transparent 24px)`,
            `repeating-linear-gradient(0deg, ${grid()} 0px, ${grid()} 1px, transparent 1px, transparent 24px)`,
          ].join(", "),
          paddingVertical: phone() ? 28 : 48,
          paddingHorizontal: phone() ? 14 : 40,
        }}
      >
        <View style={{ width: phone() ? "100%" : 380 }}>
          <InviteCard class="shadow-lg" />
        </View>
      </View>
    </View>
  );
  const code = () => (
    <View class="min-w-0" style={split() ? { width: "56%", borderRightWidth: 1, borderColor: site().line } : { borderBottomWidth: 1, borderColor: site().line }}>
      <PaneLabel>Source</PaneLabel>
      {split() ? (
        <View class="flex-1 px-5 py-5" style={{ backgroundColor: site().code.bg }}>
          <Code text={INVITE_SOURCE} size={12} />
        </View>
      ) : (
        <Scroll horizontal style={{ backgroundColor: site().code.bg }} contentClass="px-4 py-4">
          <Code text={INVITE_SOURCE} size={phone() ? 11 : 12} />
        </Scroll>
      )}
    </View>
  );
  return (
    <View>
      <Heading title="Write it once." line="A component is a function in TSX. The card beside the file is that file, running: type an email and Send invite wakes up." />
      <View class="overflow-hidden" style={cardStyle()}>
        <View class="h-11 flex-row items-center px-4" style={{ backgroundColor: site().raised, borderBottomWidth: 1, borderColor: site().line }}>
          <Lights />
          <Text class="flex-1 text-center font-mono" style={{ color: site().muted, fontSize: 12, marginRight: 52 }}>
            {INVITE_FILE}
          </Text>
        </View>
        <View class={split() ? "flex-row" : ""}>
          {code()}
          {preview()}
        </View>
      </View>
    </View>
  );
}

// ---------------------------------------------------------------- components

type Kind = "Actions" | "Forms" | "Overlays" | "Display";

/** Each kind's dot: the site's citrus, its info blue and warning amber, and a quiet gray (no purple: nsh.dev keeps it for code). */
const kindColors = (): Record<Kind, string> =>
  site().ground === "#0e1016"
    ? { Actions: "#c8f26d", Forms: "#7ea8ff", Overlays: "#ffb547", Display: "#6b6f76" }
    : { Actions: "#7cb015", Forms: "#3d63d1", Overlays: "#d08a1c", Display: "#9a9da3" };

const kinds: Kind[] = ["Actions", "Forms", "Overlays", "Display"];

const about: Record<string, [Kind, string]> = {
  "alert-dialog": ["Overlays", "Confirms what can't be undone."],
  avatar: ["Display", "A face or initials, alone or grouped."],
  badge: ["Display", "A small pill for a status or a count."],
  button: ["Actions", "Six variants, four sizes, icons."],
  calendar: ["Forms", "A month to pick a day or a range in."],
  card: ["Display", "A surface with header, body, actions."],
  checkbox: ["Forms", "On, off or mixed, with its label."],
  combobox: ["Forms", "Filters a long list as you type."],
  "data-table": ["Display", "Sort, select and resize columns."],
  "date-picker": ["Forms", "A field that opens a calendar."],
  dialog: ["Overlays", "A modal panel that holds the focus."],
  "dropdown-menu": ["Actions", "Commands and settings in a menu."],
  field: ["Forms", "Label, control, help and error."],
  input: ["Forms", "One line of text, in all its states."],
  kbd: ["Display", "A key or a shortcut: ⌘K, Ctrl+K."],
  label: ["Forms", "Names a control; a press focuses it."],
  pagination: ["Actions", "Moves through the pages of a list."],
  popover: ["Overlays", "Rich content beside its trigger."],
  progress: ["Display", "How far along a task is."],
  "radio-group": ["Forms", "One choice of a few, all in view."],
  select: ["Forms", "One choice from a list, searchable."],
  separator: ["Display", "A hairline between groups of content."],
  skeleton: ["Display", "The shape of content while it loads."],
  slider: ["Forms", "A value or a range along a track."],
  spinner: ["Display", "Work under way, with no known end."],
  switch: ["Forms", "A setting that takes effect at once."],
  table: ["Display", "Rows and columns the app writes."],
  tabs: ["Actions", "Views of one page, one at a time."],
  textarea: ["Forms", "Lines of text that grow as you type."],
  toast: ["Overlays", "A brief note, and the time it has left."],
  toggle: ["Actions", "A button that stays on or off."],
  "toggle-group": ["Actions", "A set of toggles, one or several on."],
  tooltip: ["Overlays", "A short hint where the pointer rests."],
};

function Components() {
  const columns = () => (siteWidth() >= 900 ? 4 : 2);
  const groups = () => kinds.map((k) => ({ kind: k, pages: componentPages.filter((p) => (about[p.id]?.[0] ?? "Display") === k) }));
  const rows = () => {
    const out: ReturnType<typeof groups>[] = [];
    for (let i = 0; i < kinds.length; i += columns()) out.push(groups().slice(i, i + columns()));
    return out;
  };
  return (
    <View>
      <Heading title={`${componentPages.length} components.`} line="Each with its own page: the variants, the states and the tokens it reads." />
      <View class="gap-10">
        {rows().map((row) => (
          <View class="flex-row" style={{ gap: phone() ? 16 : 32 }}>
            {row.map((g) => (
              <View class="flex-1 min-w-0 gap-1">
                <View class="flex-row items-center gap-2 pb-3" style={{ borderBottomWidth: 1, borderColor: site().line, marginBottom: 6 }}>
                  <View class="size-2 rounded-full" style={{ backgroundColor: kindColors()[g.kind] }} />
                  <Text style={{ color: site().muted, fontSize: 12.5, fontWeight: "550" }}>{g.kind}</Text>
                </View>
                {g.pages.map((p) => (
                  <Text
                    class="py-1"
                    style={{ color: site().ink, fontSize: 14.5, cursor: "default" }}
                    hoverStyle={{ color: site().muted }}
                    role="link"
                    accessibilityHint={about[p.id]?.[1]}
                    onPress={() => navigate(p.id)}
                  >
                    {p.label}
                  </Text>
                ))}
              </View>
            ))}
          </View>
        ))}
      </View>
    </View>
  );
}

// ---------------------------------------------------------------- footer

function FooterLink(props: { label: string; onPress: () => void }) {
  return (
    <Text
      style={{ color: site().muted, fontSize: 13.5, cursor: "default" }}
      hoverStyle={{ color: site().ink }}
      role="link"
      onPress={props.onPress}
    >
      {props.label}
    </Text>
  );
}

function Footer() {
  const s = () => site();
  // Side by side only where the six links fit beside the name.
  const stacked = () => siteWidth() < 1000;
  return (
    <View
      class="items-center"
      style={{ borderTopWidth: 1, borderColor: s().line, backgroundColor: s().ground, paddingHorizontal: gutter(), paddingTop: 40, paddingBottom: 48 }}
    >
      <View class={stacked() ? "gap-8" : "flex-row justify-between gap-16"} style={{ width: siteWidth() }}>
        <View class="gap-2" style={{ maxWidth: 460 }}>
          <View class="flex-row items-center gap-2">
            <Logo size={20} />
            <Text style={{ color: s().ink, fontSize: 14, fontWeight: "640", letterSpacing: -0.3 }}>NSH Longbridge</Text>
          </View>
          <Text style={{ color: s().muted, fontSize: 13.5, lineHeight: 21 }}>Components and design tokens for NSH apps.</Text>
          <Text style={{ color: s().muted, fontSize: 13.5, lineHeight: 21 }}>
            Code licensed under Apache-2.0. Developed by Longbridge.
          </Text>
        </View>
        <View class="flex-row flex-wrap" style={{ columnGap: 20, rowGap: 8, justifyContent: stacked() ? "flex-start" : "flex-end", maxWidth: 680 }}>
          <FooterLink label="Components" onPress={() => navigate(componentPages[0].id)} />
          <FooterLink label="Theme studio" onPress={() => navigate("customize")} />
          <FooterLink label="Colors" onPress={() => navigate("colors")} />
          <FooterLink label="Source code" onPress={() => openUrl(REPOSITORY)} />
          <FooterLink label="Report an issue" onPress={() => openUrl(`${REPOSITORY}/issues`)} />
          <FooterLink label="Native Shell" onPress={() => openUrl(NSH_SITE)} />
        </View>
      </View>
    </View>
  );
}

export function HomePage() {
  return (
    <View style={{ backgroundColor: site().ground }}>
      <Hero />
      <Band>
        <Heading title="One file restyles all of it." line="Every radius, size and color is a token. Move one and every component that reads it follows." />
        <TokenDemo width={siteWidth()} />
      </Band>
      <Band>
        <CodeAndResult />
      </Band>
      <Band>
        <Components />
      </Band>
      <View style={{ height: phone() ? 72 : 128, backgroundColor: site().ground }} />
      <Footer />
    </View>
  );
}
