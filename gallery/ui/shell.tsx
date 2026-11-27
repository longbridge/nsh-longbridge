// The gallery's frame: the header (the title bar on macOS), the sidebar
// (behind a button in a sheet below 900 points), the page and its pager.

import { keymap, openUrl, platform, Scroll, signal, Text, View, type JSX } from "nsh";
import { TitleBar, Sheet } from "nsh/base";
import { Button, Input, type InputRef, Kbd, Separator, ThemeScope, Tooltip } from "nsh-longbridge";
import { catalog, type PageEntry, pageById, pages } from "../catalog";
import { navigate, route } from "../router";
import {
  appearance,
  type Appearance,
  colorPreset,
  colorPresets,
  cssColor,
  densityPreset,
  densityPresets,
  presetPrimary,
  scheme,
} from "../theme";
import { Segmented, Select } from "./controls";
import { GalleryToaster } from "./toaster";
import { compact, narrow, pagePadding } from "./screen";
import { ChevronRight, GitHub, Glyph, Menu, Monitor, Moon, Search, Sun } from "./icons";

export const REPOSITORY = "https://github.com/longbridge/nsh-longbridge";


const macos = platform.os === "macos";

function Dot(props: { color: string }) {
  return <View class="size-3 rounded-full border border-border" style={{ backgroundColor: props.color }} />;
}

/** The gallery's controls of the theme shown. */
export function ThemeControls(props: { narrow?: boolean }) {
  return (
    <View class="flex-row items-center gap-2">
      <Select
        accessibilityLabel="Color preset"
        items={colorPresets.map((p) => ({ value: p.id, title: p.label }))}
        value={colorPreset}
        onChange={(id) => colorPreset.set(id)}
        leading={(id) => <Dot color={presetPrimary(id)} />}
      />
      <If when={!props.narrow}>
        <Segmented
          accessibilityLabel="Density"
          options={densityPresets.map((p) => ({ id: p.id, label: p.label }))}
          value={densityPreset}
          onChange={(id) => densityPreset.set(id)}
        />
      </If>
      <Segmented
        accessibilityLabel="Appearance"
        options={[
          { id: "light", icon: Sun, name: "Light" },
          { id: "dark", icon: Moon, name: "Dark" },
          { id: "system", icon: Monitor, name: "System" },
        ]}
        value={appearance}
        onChange={(id) => appearance.set(id as Appearance)}
      />
    </View>
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
      <Glyph icon={GitHub} />
    </Button>
  );
}

function Header(props: { onMenu: () => void }) {
  const content = () => (
    <View class="flex-1 flex-row items-center justify-between gap-4 pr-4" style={{ paddingLeft: macos ? 8 : 16 }}>
      <View class="flex-row items-center gap-2">
        <If when={compact()}>
          <Button variant="ghost" size="sm" class="px-0 w-8" accessibilityLabel="Pages" onPress={props.onMenu}>
            <Glyph icon={Menu} />
          </Button>
        </If>
        <Button variant="ghost" size="sm" class="px-2" onPress={() => navigate("home")} accessibilityLabel="Home">
          <Text class="text-sm font-semibold tracking-tight text-foreground">nsh-longbridge</Text>
        </Button>
      </View>
      <View class="flex-row items-center gap-3">
        <ThemeControls narrow={narrow()} />
        <If when={!narrow()}>
          <Separator orientation="vertical" class="h-5" />
          <GitHubButton />
        </If>
      </View>
    </View>
  );
  return (
    <View class="border-b border-border bg-background">
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
          <Button variant="ghost" size="sm" onPress={() => navigate(prev()!.id)}>
            <View style={{ transform: [{ rotate: "180deg" }] }}>
              <Glyph icon={ChevronRight} size={14} />
            </View>
            <Text>{prev()!.label}</Text>
          </Button>
        </If>
      </View>
      <View>
        <If when={next()}>
          <Button variant="ghost" size="sm" onPress={() => navigate(next()!.id)}>
            <Text>{next()!.label}</Text>
            <Glyph icon={ChevronRight} size={14} />
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
        return (
          <Scroll class="flex-1" contentClass="items-center">
            <View
              class={`w-full ${entry.wide ? "max-w-[1280px]" : "max-w-[960px]"}`}
              style={{ paddingHorizontal: pagePadding(), paddingTop: compact() ? 32 : 48, paddingBottom: 48 }}
            >
              <Current />
              <If when={entry.id !== "home"}>
                <Pager id={entry.id} />
              </If>
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
    <View class="flex-1 bg-background">
      <Header onMenu={() => menu.set(true)} />
      <View class="flex-1 flex-row">
        <If when={!compact()}>
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
