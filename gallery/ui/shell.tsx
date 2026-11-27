// The gallery's frame: the header (the title bar on macOS), the sidebar
// (behind a button in a sheet below 900 points), the page and its pager.
// The header is the site's, drawn as nsh.dev's: NSH's mark and the name
// (with the way back to Native Shell), then the theme, the appearance and
// GitHub.

import { keymap, openUrl, platform, Scroll, signal, Text, View, type JSX } from "nsh";
import { TitleBar, Sheet } from "nsh/base";
import {
  Button,
  DropdownMenu,
  DropdownMenuRadioGroup,
  DropdownMenuRadioItem,
  Input,
  type InputRef,
  Kbd,
  Popover,
  ThemeScope,
  Tooltip,
} from "nsh-longbridge";
import { ArrowRight, ArrowUpRight, ChevronDown, ChevronsUpDown } from "nsh-icon-lucide";
import { catalog, type PageEntry, pageById, pages } from "../catalog";
import { navigate, route } from "../router";
import { appearance, type Appearance, colorPreset, cssColor } from "../theme";
import { Brand, Logo, PRODUCT } from "./brand";
import { GalleryToaster } from "./toaster";
import { compact, keyboardUsed, narrow, pagePadding, pageScroll, tiny } from "./screen";
import { GitHub, Glyph, Menu, Monitor, Moon, Search, Sun } from "./icons";
import { DensityTiles, densityLabel, PresetSwatch, PresetSwatches, presetLabel } from "./theme-picker";
import { DISPLAY, NSH_SITE, REPOSITORY, site } from "./site";
import { ink, Segmented } from "./site-ui";

export { REPOSITORY };

const macos = platform.os === "macos";

/** The pages drawn edge to edge on the site's ground, without the sidebar: the home page and the theme studio. */
const fullBleed = (id: string) => id === "home" || id === "customize";

/** A square button of the header, quiet until hovered, drawn in the site's colors. */
function HeaderIcon(props: { icon: string; label: string; onPress?: () => void; size?: number; side?: number }) {
  const side = props.side ?? 32;
  return (
    <View
      class="items-center justify-center"
      style={{ width: side, height: side, borderRadius: 8, cursor: "default" }}
      hoverStyle={{ backgroundColor: site().hover }}
      role="button"
      accessibilityLabel={props.label}
      onPress={props.onPress}
    >
      <Glyph icon={props.icon} size={props.size ?? 16} class={ink(site().muted)} />
    </View>
  );
}

/** One project of the NSH family, as the brand's menu lists it. */
function Project(props: { name: string; line: string; label?: string; current?: boolean; external?: boolean; onPress: () => void }) {
  return (
    <View
      class="flex-row items-center gap-3 rounded-lg px-2.5 py-2"
      style={{ backgroundColor: props.current ? site().hover : undefined, cursor: "default" }}
      hoverStyle={{ backgroundColor: site().hover }}
      role="link"
      accessibilityLabel={props.label ?? props.name}
      onPress={props.onPress}
    >
      <Logo size={28} />
      <View class="flex-1 gap-0.5">
        <Text style={{ color: site().ink, fontSize: 13.5, fontWeight: "600", letterSpacing: -0.2 }}>{props.name}</Text>
        <Text style={{ color: site().muted, fontSize: 12 }}>{props.line}</Text>
      </View>
      <If when={props.external}>
        <Glyph icon={ArrowUpRight} size={14} class={ink(site().faint)} />
      </If>
    </View>
  );
}

/**
 * The brand: "NSH / Longbridge". The mark and "NSH" lead to nsh.dev, the
 * family's site; "Longbridge" opens a menu of the family (this library, and
 * Native Shell). Narrow, the mark stands for "NSH".
 */
function BrandMenu() {
  const open = signal(false);
  const go = (f: () => void) => {
    open.set(false);
    f();
  };
  return (
    <View class="flex-row items-center">
      <View
        class="h-8 flex-row items-center"
        style={{ paddingHorizontal: 8, borderRadius: 9, cursor: "default" }}
        role="link"
        accessibilityLabel="NSH, nsh.dev"
        onPress={() => openUrl(NSH_SITE)}
      >
        <Brand name={!tiny()} />
      </View>
      <Text style={{ fontFamily: DISPLAY, color: site().slash, fontSize: 20, fontWeight: "300", marginHorizontal: 2 }} accessible={false}>
        /
      </Text>
      <Popover
        open={open()}
        onOpenChange={(e) => open.set(e.open)}
        placement="bottom"
        align="start"
        class="w-[360px] gap-0 p-1.5"
        style={{ backgroundColor: site().card, borderColor: site().border }}
        trigger={
          <View
            class="h-8 flex-row items-center"
            style={{ gap: 8, paddingLeft: 8, paddingRight: 7, borderRadius: 9, cursor: "default", backgroundColor: open() ? site().hover : undefined }}
            hoverStyle={{ backgroundColor: site().hover }}
            role="button"
            accessibilityLabel={`${PRODUCT}, other projects`}
          >
            <Text style={{ fontFamily: DISPLAY, color: site().ink, fontSize: 15, fontWeight: "500" }}>Longbridge</Text>
            <Glyph icon={ChevronsUpDown} size={14} class={ink(site().muted)} />
          </View>
        }
      >
        <Project
          name={PRODUCT}
          line="Components and design tokens for NSH"
          label={`${PRODUCT}, home`}
          current
          onPress={() => go(() => navigate("home"))}
        />
        <Project
          name="Native Shell"
          line="One native UI framework, every platform"
          external
          onPress={() => go(() => openUrl(NSH_SITE))}
        />
      </Popover>
    </View>
  );
}

/** The theme shown: the color preset and the density, behind one button. */
function ThemeMenu(props: { iconOnly?: boolean }) {
  const open = signal(false);
  const studio = () => {
    open.set(false);
    navigate("customize");
  };
  const s = () => site();
  return (
    <Popover
      open={open()}
      onOpenChange={(e) => open.set(e.open)}
      placement="bottom"
      align="end"
      class="w-[340px] gap-0 p-0"
      style={{ backgroundColor: s().card, borderColor: s().border }}
      trigger={
        <View
          class="h-8 flex-row items-center gap-2 rounded-lg"
          style={{ paddingLeft: props.iconOnly ? 0 : 8, paddingRight: props.iconOnly ? 0 : 6, width: props.iconOnly ? 32 : undefined, justifyContent: "center", cursor: "default" }}
          hoverStyle={{ backgroundColor: s().hover }}
          role="button"
          accessibilityLabel={`Theme: ${presetLabel()}, ${densityLabel()}`}
        >
          <PresetSwatch id={colorPreset()} size={14} />
          <If when={!props.iconOnly}>
            <Text style={{ color: s().ink, fontSize: 13.5, fontWeight: "500" }}>Theme</Text>
            <Glyph icon={ChevronDown} size={14} class={ink(s().faint)} />
          </If>
        </View>
      }
    >
      <View class="gap-1 px-5 pt-4 pb-4">
        <Text style={{ color: s().ink, fontSize: 14, fontWeight: "600", letterSpacing: -0.2 }}>Theme</Text>
        <Text style={{ color: s().muted, fontSize: 12.5, lineHeight: 18 }}>
          Recolors every demo on the site, in light and dark.
        </Text>
      </View>
      <View style={{ height: 1, backgroundColor: s().line }} />
      <View class="gap-3 px-5 pt-4 pb-5">
        <Text class="font-mono" style={{ color: s().faint, fontSize: 10.5, letterSpacing: 1.4 }}>COLOR</Text>
        <PresetSwatches labels spread />
      </View>
      <View style={{ height: 1, backgroundColor: s().line }} />
      <View class="gap-3 px-5 pt-4 pb-5">
        <Text class="font-mono" style={{ color: s().faint, fontSize: 10.5, letterSpacing: 1.4 }}>DENSITY</Text>
        <DensityTiles />
      </View>
      <View style={{ height: 1, backgroundColor: s().line }} />
      <View
        class="h-11 flex-row items-center justify-between px-5"
        style={{ cursor: "default" }}
        hoverStyle={{ backgroundColor: s().hover }}
        role="link"
        accessibilityLabel="Open the theme studio"
        onPress={studio}
      >
        <Text style={{ color: s().citrus, fontSize: 13, fontWeight: "550" }}>Every token, in the theme studio</Text>
        <Glyph icon={ArrowRight} size={14} class={ink(s().citrus)} />
      </View>
    </Popover>
  );
}

const appearances: { value: Appearance; label: string; icon: string }[] = [
  { value: "system", label: "System", icon: Monitor },
  { value: "light", label: "Light", icon: Sun },
  { value: "dark", label: "Dark", icon: Moon },
];

/** System, light or dark, as nsh.dev's header switches them: three icons in a segmented control. */
function AppearanceSwitch() {
  return (
    <Segmented
      accessibilityLabel="Appearance"
      segments={appearances}
      value={appearance()}
      onChange={(v) => appearance.set(v as Appearance)}
      size={24}
    />
  );
}

/** The same choice from one button, where the header has no room for three (phones). */
function AppearanceMenu() {
  const current = () => appearances.find((a) => a.value === appearance()) ?? appearances[0];
  return (
    <DropdownMenu
      align="end"
      size="sm"
      contentClass="w-36"
      trigger={
        <Button variant="ghost" size="sm" class="px-0 w-8" accessibilityLabel={`Appearance: ${current().label}`}>
          <Glyph icon={current().icon} class={ink(site().muted)} />
        </Button>
      }
    >
      <DropdownMenuRadioGroup value={appearance()} onChange={(e) => appearance.set(e.value as Appearance)}>
        {appearances.map((a) => (
          <DropdownMenuRadioItem value={a.value}>
            <Text>{a.label}</Text>
          </DropdownMenuRadioItem>
        ))}
      </DropdownMenuRadioGroup>
    </DropdownMenu>
  );
}

function GitHubButton() {
  return (
    <Button
      variant="ghost"
      size="sm"
      class="px-0 w-8"
      accessibilityLabel="GitHub"
      onPress={() => openUrl(REPOSITORY)}
      tooltip={
        <Tooltip placement="bottom">
          <Text>Source on GitHub</Text>
        </Tooltip>
      }
    >
      <Glyph icon={GitHub} class={ink(site().ink)} />
    </Button>
  );
}

/**
 * The header: the brand, then the theme, the appearance and GitHub. On the
 * site's own pages it is the site's ground under a hairline, as nsh.dev's.
 */
function Header(props: { onMenu: () => void }) {
  const own = () => fullBleed(route());
  const fill = () => (own() ? site().ground : cssColor("background"));
  const edge = () => (own() ? site().line : cssColor("border"));
  const content = () => (
    <View class="flex-1 flex-row items-center justify-between gap-2 pr-3 sm:pr-5" style={{ paddingLeft: macos ? 8 : narrow() ? 8 : 16 }}>
      <View class="flex-row items-center gap-1">
        <If when={compact()}>
          <Button variant="ghost" size="sm" class="px-0 w-8" accessibilityLabel="Pages" onPress={props.onMenu}>
            <Glyph icon={Menu} />
          </Button>
        </If>
        <BrandMenu />
      </View>
      <View class="flex-row items-center gap-1.5">
        <ThemeMenu iconOnly={narrow()} />
        {narrow() ? <AppearanceMenu /> : <AppearanceSwitch />}
        <GitHubButton />
      </View>
    </View>
  );
  return (
    <View style={{ backgroundColor: fill(), borderBottomWidth: 1, borderColor: edge(), boxShadow: own() ? `inset 0 1px 0 ${site().highlight}` : undefined }}>
      {macos ? (
        <TitleBar class="h-14 items-center">{content()}</TitleBar>
      ) : (
        <View class="h-14 flex-row items-center">{content()}</View>
      )}
    </View>
  );
}

function Sidebar(props: { onSelect?: () => void; class?: string }) {
  const query = signal("");
  const field = signal<InputRef>();
  keymap.bind("mod+k", () => field()?.focus(), { when: "always" });
  const matches = (p: PageEntry) => p.label.toLowerCase().includes(query().trim().toLowerCase());
  const groups = () =>
    catalog.map((g) => ({ title: g.title, pages: g.pages.filter(matches) })).filter((g) => g.pages.length > 0);
  const select = (id: string) => {
    navigate(id);
    props.onSelect?.();
  };
  return (
    <View class={`w-64 bg-background ${props.class ?? ""}`}>
      <View class="px-4 pt-4 pb-2">
        <ThemeScope tokens={{ "input-padding-x-sm": "30px" }}>
          <View>
            <Input
              ref={field}
              size="sm"
              placeholder="Filter"
              accessibilityLabel="Filter pages"
              value={query()}
              onChange={(e) => query.set(e.value)}
              onSubmit={() => {
                const first = groups()[0]?.pages[0];
                if (first) select(first.id);
              }}
            />
            <View class="absolute left-2.5 top-0 bottom-0 justify-center pointer-events-none">
              <Glyph icon={Search} size={14} class="text-muted-foreground" />
            </View>
            <If when={query() === ""}>
              <View class="absolute right-1.5 top-0 bottom-0 justify-center pointer-events-none">
                <Kbd>
                  <Text>{keymap.label("mod+k")}</Text>
                </Kbd>
              </View>
            </If>
          </View>
        </ThemeScope>
      </View>
      <Scroll class="flex-1" contentClass="px-4 pt-2 pb-8 gap-6">
        <Each of={groups()} key={(g) => g.title}>
          {(group) => (
            <View class="gap-0.5">
              <Text class="px-2 pb-1.5 text-xs font-medium text-muted-foreground">{group().title}</Text>
              <Each of={group().pages} key={(p) => p.id}>
                {(p) => (
                  <Button
                    size="sm"
                    variant={route() === p().id ? "secondary" : "ghost"}
                    class="w-full justify-start px-2"
                    labelClass={route() === p().id ? "font-medium" : "font-normal"}
                    onPress={() => select(p().id)}
                  >
                    <Text>{p().label}</Text>
                  </Button>
                )}
              </Each>
            </View>
          )}
        </Each>
        <If when={groups().length === 0}>
          <Text class="px-2 text-sm text-muted-foreground">No pages match.</Text>
        </If>
        <If when={narrow()}>
          <Button variant="ghost" size="sm" class="w-full justify-start px-2" onPress={() => openUrl(REPOSITORY)}>
            <Glyph icon={GitHub} size={14} />
            <Text>GitHub</Text>
          </Button>
        </If>
      </Scroll>
    </View>
  );
}

/** The pages before and after the current one, for reading in order. */
function Pager(props: { id: string }) {
  const index = () => pages.findIndex((p) => p.id === props.id);
  const prev = () => pages[index() - 1];
  const next = () => pages[index() + 1];
  return (
    <View class="mt-20 flex-row items-center justify-between gap-4 border-t border-border pt-6">
      <View>
        <If when={prev()}>
          <Button variant="ghost" size="sm" icon="chevron-left" onPress={() => navigate(prev()!.id)}>
            <Text>{prev()!.label}</Text>
          </Button>
        </If>
      </View>
      <View>
        <If when={next()}>
          <Button variant="ghost" size="sm" iconEnd="chevron-right" onPress={() => navigate(next()!.id)}>
            <Text>{next()!.label}</Text>
          </Button>
        </If>
      </View>
    </View>
  );
}

function Content() {
  return (
    <>
      {(() => {
        const entry = pageById(route());
        const Current = entry.page;
        pageScroll.set(0);
        // The theme studio lays out its own scrolling panes, on the site's ground.
        if (entry.id === "customize") {
          return (
            <View class="flex-1" style={{ backgroundColor: site().ground }}>
              <Current />
            </View>
          );
        }
        // The home page draws its own background and margins, edge to edge.
        if (entry.id === "home") {
          return (
            <Scroll
              class="flex-1"
              style={{ backgroundColor: site().ground }}
              scrollEventThrottle={100}
              onScroll={(e) => pageScroll.set(e.offset.y)}
            >
              <Current />
            </Scroll>
          );
        }
        return (
          <Scroll
            class="flex-1"
            contentClass="items-center"
            scrollEventThrottle={100}
            onScroll={(e) => pageScroll.set(e.offset.y)}
          >
            <View
              class={`w-full ${entry.wide ? "max-w-[1280px]" : "max-w-[960px]"}`}
              style={{ paddingHorizontal: pagePadding(), paddingTop: compact() ? 32 : 48, paddingBottom: 48 }}
            >
              <Current />
              <Pager id={entry.id} />
            </View>
          </Scroll>
        );
      })()}
    </>
  );
}

export function Shell(): JSX.Element {
  const menu = signal(false);
  return (
    <View class="flex-1 bg-background" onKeyDown={() => keyboardUsed.set(true)}>
      <Header onMenu={() => menu.set(true)} />
      <View class="flex-1 flex-row">
        <If when={!compact() && !fullBleed(route())}>
          <Sidebar class="border-r border-border" />
        </If>
        <Content />
      </View>
      <If when={compact()}>
        <Sheet
          side="left"
          open={menu()}
          onOpenChange={(e) => menu.set(e.open)}
          accessibilityLabel="Pages"
          popupClass="w-72 h-full border-r border-border bg-background shadow-lg"
          backdropStyle={{ backgroundColor: cssColor("overlay") }}
        >
          <Sidebar class="flex-1 w-full" onSelect={() => menu.set(false)} />
        </Sheet>
      </If>
      <GalleryToaster />
    </View>
  );
}
