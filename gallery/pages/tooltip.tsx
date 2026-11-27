// The Tooltip page: an editor's toolbar, the four sides, shortcuts and long
// text, delays, a disabled trigger, controlled.

import { keymap, signal, Text, View } from "nsh";
import { Button, Kbd, Separator, Tooltip } from "nsh-longbridge";
import { Demo, Demos, Page, Section } from "../ui/layout";
import { TokensSection } from "../ui/tokens";

const placements = ["top", "right", "bottom", "left"] as const;
const tools = [
  { icon: "plus", label: "New page", keys: "mod+n" },
  { icon: "check", label: "Mark done", keys: "mod+enter" },
  { icon: "arrow-right", label: "Move to…", keys: "mod+shift+m" },
  { icon: "trash-2", label: "Delete", keys: "mod+backspace" },
] as const;
const delays = [0, 500, 1000];
const long =
  "Quarterly planning notes, including the hiring plan, the roadmap for the second half and the open questions from the offsite";

export function TooltipPage() {
  // Each scene holding its tooltips open shows them while it is in the window.
  const hero = signal(true);
  const sides = signal(false);
  const save = signal(false);
  const title = signal(false);
  return (
    <Page
      eyebrow="Components"
      title="Tooltip"
      description="A hint beside an element while the pointer rests on it or the keyboard focus is on it. Inverted, no arrow; hardly ever shown on touch screens."
    >
      <Demo hero class="pt-20" inView={hero.set}>
        <Toolbar shown={hero()} />
      </Demo>

      <Section title="Sides" description="Shown here held open. A tooltip flips to the other side when only that one has room.">
        <Demo row class="gap-x-24 gap-y-20 py-20" inView={sides.set}>
          {placements.map((p) => (
            <Button
              variant="outline"
              tooltip={
                <Tooltip placement={p} open={sides()}>
                  <Text>{p[0].toUpperCase() + p.slice(1)}</Text>
                </Tooltip>
              }
            >
              <Text>{p[0].toUpperCase() + p.slice(1)}</Text>
            </Button>
          ))}
        </Demo>
      </Section>

      <Section title="Shortcuts and long text" description="A Kbd inside takes the tooltip's colors. Long text, such as a cut label's whole title, wraps at tooltip-max-width (320).">
        <Demos>
          <Demo row label="Save, with its shortcut" class="pt-20" inView={save.set}>
            <Button
              tooltip={
                <Tooltip open={save()}>
                  <Text>Save</Text>
                  <Kbd>
                    <Text>{keymap.label("mod+s")}</Text>
                  </Kbd>
                </Tooltip>
              }
            >
              <Text>Save</Text>
            </Button>
          </Demo>
          <Demo label="A title cut short, whole in the tooltip" class="pt-28" inView={title.set}>
            <View class="w-56 flex-row items-center gap-2 rounded-md border border-border px-3 py-2">
              <Text
                class="flex-1 truncate text-sm text-foreground"
                tooltip={
                  <Tooltip open={title()} placement="top">
                    <Text>{long}</Text>
                  </Tooltip>
                }
              >
                {long}
              </Text>
            </View>
          </Demo>
        </Demos>
      </Section>

      <Section title="Delay" description="The tooltip-delay token (500) by default. Once one shows, the next shows at once as the pointer moves along. 0 for what the user is there to read.">
        <Demo row>
          {delays.map((d) => (
            <Button
              variant="outline"
              tooltip={
                <Tooltip delay={d}>
                  <Text>{`Shown after ${d} ms`}</Text>
                </Tooltip>
              }
            >
              <Text>{`${d} ms`}</Text>
            </Button>
          ))}
        </Demo>
      </Section>

      <Section title="States" description="A disabled button still shows its tooltip, which may say why. disabled on the tooltip shows none. The app can own open.">
        <Demos>
          <Demo row label="Rest the pointer on each">
            <Button
              disabled
              tooltip={
                <Tooltip>
                  <Text>Nothing to save yet</Text>
                </Tooltip>
              }
            >
              <Text>Save</Text>
            </Button>
            <Button
              variant="outline"
              tooltip={
                <Tooltip disabled>
                  <Text>Never shown</Text>
                </Tooltip>
              }
            >
              <Text>No tooltip</Text>
            </Button>
          </Demo>
          <ControlledDemo />
        </Demos>
      </Section>

      <TokensSection prefix="tooltip" component="Tooltip" />
    </Page>
  );
}

/** An editor's toolbar: icon buttons named by their tooltips, the first held open while `shown`. */
function Toolbar(props: { shown: boolean }) {
  return (
    <View class="flex-row items-center gap-1 rounded-lg border border-border bg-background p-1 shadow-xs">
      {tools.map((t, i) => (
        <Button
          variant="ghost"
          size="sm"
          iconOnly
          icon={t.icon}
          accessibilityLabel={t.label}
          tooltip={
            <Tooltip open={i === 0 ? props.shown : undefined}>
              <Text>{t.label}</Text>
              <Kbd>
                <Text>{keymap.label(t.keys)}</Text>
              </Kbd>
            </Tooltip>
          }
        />
      ))}
      <Separator orientation="vertical" class="mx-1 h-5" />
      <Button variant="ghost" size="sm" iconEnd="chevron-down">
        <Text>More</Text>
      </Button>
    </View>
  );
}

function ControlledDemo() {
  const open = signal(false);
  return (
    <Demo row class="pt-16" label={`open: ${open()}`}>
      <View
        tooltip={
          <Tooltip open={open()} onOpenChange={(e) => open.set(e.open)}>
            <Text>Synced 2 minutes ago</Text>
          </Tooltip>
        }
      >
        <Button variant="outline">
          <Text>Status</Text>
        </Button>
      </View>
      <Button variant="ghost" onPress={() => open.set(!open())}>
        <Text>{open() ? "Hide" : "Show"}</Text>
      </Button>
    </Demo>
  );
}
