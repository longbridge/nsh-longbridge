// The Kbd page: a command menu's hints, keys, groups, in a button and in a
// tooltip.

import { keymap, signal, Text, View } from "nsh";
import { Button, Kbd, KbdGroup, Separator, Tooltip } from "nsh-longbridge";
import { Demo, Demos, Page, Section } from "../ui/layout";
import { Glyph, Search } from "../ui/icons";
import { TokensSection } from "../ui/tokens";

const keys = ["⌘", "⇧", "⌥", "⌃", "Esc", "Enter", "Tab", "←"];
const commands = [
  { group: "Files", items: [{ label: "New file", keys: "mod+n" }, { label: "Open…", keys: "mod+o" }, { label: "Save", keys: "mod+s" }] },
  { group: "View", items: [{ label: "Toggle sidebar", keys: "mod+b" }, { label: "Command palette", keys: "mod+shift+p" }] },
];

/** A shortcut's keys as caps: `keymap.label("mod+shift+p")` split into its keys. */
export function keyCaps(combo: string): string[] {
  return keymap
    .label(combo)
    .split(/(?<=[⌘⇧⌥⌃])|\+/)
    .filter((k) => k);
}

function Shortcut(props: { keys: string }) {
  return (
    <KbdGroup>
      {keyCaps(props.keys).map((k) => (
        <Kbd>
          <Text>{k}</Text>
        </Kbd>
      ))}
    </KbdGroup>
  );
}

export function KbdPage() {
  const tipShown = signal(false);
  return (
    <Page
      eyebrow="Components"
      title="Kbd"
      description="Key caps for shortcuts in menus, buttons and tooltips. It shows the text it is given: write keys with keymap.label so each platform reads its own."
    >
      <Demo hero>
        <CommandMenu />
      </Demo>

      <Section title="Keys" description="One character is a square; a name widens the cap.">
        <Demo row>
          {keys.map((k) => (
            <Kbd>
              <Text>{k}</Text>
            </Kbd>
          ))}
        </Demo>
      </Section>

      <Section title="Groups" description="Caps in a row, 4 apart. Words between them are the app's own text.">
        <Demos>
          <Demo row label="A chord, written out">
            <KbdGroup>
              <Kbd>
                <Text>Ctrl</Text>
              </Kbd>
              <Text class="text-xs text-muted-foreground">+</Text>
              <Kbd>
                <Text>B</Text>
              </Kbd>
            </KbdGroup>
          </Demo>
          <Demo row class="gap-6" label="This platform's labels, from keymap.label">
            <Shortcut keys="mod+k" />
            <Shortcut keys="mod+shift+p" />
            <Shortcut keys="alt+enter" />
          </Demo>
        </Demos>
      </Section>

      <Section title="In a button and a tooltip" description="Inside a tooltip a cap takes the tooltip's inverted colors by itself.">
        <Demos>
          <Demo row label="A search button with its shortcut">
            <Button variant="outline" class="w-72 justify-between" labelClass="font-normal">
              <View class="flex-row items-center gap-2">
                <Glyph icon={Search} size={14} class="text-muted-foreground" />
                <Text class="text-muted-foreground">Search documentation…</Text>
              </View>
              <Kbd>
                <Text>{keymap.label("mod+k")}</Text>
              </Kbd>
            </Button>
          </Demo>
          <Demo row label="A tooltip naming its shortcut" class="pt-16" inView={tipShown.set}>
            <View
              tooltip={
                <Tooltip open={tipShown()}>
                  <Text>Save</Text>
                  <Kbd>
                    <Text>{keymap.label("mod+s")}</Text>
                  </Kbd>
                </Tooltip>
              }
            >
              <Button>
                <Text>Save</Text>
              </Button>
            </View>
          </Demo>
        </Demos>
      </Section>

      <TokensSection prefix="kbd" component="Kbd" note="The kbd-tooltip colors are a cap's inside a tooltip." />
    </Page>
  );
}

/** A command menu: actions with their shortcuts on the right. */
function CommandMenu() {
  return (
    <View class="w-full max-w-[380px] rounded-lg border border-border bg-popover shadow-md">
      <View class="h-11 flex-row items-center gap-2 border-b border-border px-3">
        <Glyph icon={Search} size={16} class="text-muted-foreground" />
        <Text class="flex-1 text-sm text-muted-foreground">Type a command or search…</Text>
        <Kbd>
          <Text>Esc</Text>
        </Kbd>
      </View>
      <View class="p-1">
        {commands.map((g, i) => (
          <View>
            <If when={i > 0}>
              <Separator class="my-1" />
            </If>
            <Text class="px-2 pb-1 pt-2 text-xs font-medium text-muted-foreground">{g.group}</Text>
            {g.items.map((item, j) => (
              <View class={`h-8 flex-row items-center justify-between rounded-sm px-2 ${i === 0 && j === 0 ? "bg-accent" : ""}`}>
                <Text class={`text-sm ${i === 0 && j === 0 ? "text-accent-foreground" : "text-popover-foreground"}`}>{item.label}</Text>
                <Shortcut keys={item.keys} />
              </View>
            ))}
          </View>
        ))}
      </View>
    </View>
  );
}
