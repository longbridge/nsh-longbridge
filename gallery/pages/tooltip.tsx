// The Tooltip page: on buttons, the four sides, shortcuts, long text,
// delays, a disabled trigger, controlled.

import { signal, Text, View } from "nsh";
import { Button, Kbd, Tooltip } from "nsh-longbridge";
import { Caption, Page, Preview, Section } from "../ui";

const placements = ["top", "right", "bottom", "left"] as const;
const tools = [
  { icon: "plus", label: "Add" },
  { icon: "arrow-right", label: "Forward" },
  { icon: "trash-2", label: "Delete" },
] as const;
const delays = [0, 500, 1000];
const long =
  "Shows the full text of a label cut short, wrapping at tooltip-max-width (320) rather than running out of the window.";

export function TooltipPage() {
  return (
    <Page
      title="Tooltip"
      description="A hint beside an element while the pointer rests on it or the keyboard focus is on it. Inverted, no arrow; hardly ever shown on touch screens."
    >
      <Section title="Icon buttons" description="Rest the pointer on a tool, then move along: the next shows at once. Tab shows them too.">
        <Preview
          code={tools
            .map(
              (t) =>
                `<Button variant="ghost" iconOnly icon="${t.icon}" accessibilityLabel="${t.label}" tooltip={<Tooltip>${t.label}</Tooltip>} />`,
            )
            .join("\n")}
        >
          {tools.map((t) => (
            <Button
              variant="ghost"
              iconOnly
              icon={t.icon}
              accessibilityLabel={t.label}
              tooltip={
                <Tooltip>
                  <Text>{t.label}</Text>
                </Tooltip>
              }
            />
          ))}
        </Preview>
      </Section>

      <Section title="Placement" description="Shown here with open. Flipped to the other side when only that one has room.">
        <Preview
          class="py-20 gap-16"
          code={placements.map((p) => `<Button variant="outline" tooltip={<Tooltip placement="${p}">${p}</Tooltip>}>…</Button>`).join("\n")}
        >
          {placements.map((p) => (
            <Button
              variant="outline"
              tooltip={
                <Tooltip placement={p} open>
                  <Text>{p}</Text>
                </Tooltip>
              }
            >
              <Text>{p}</Text>
            </Button>
          ))}
        </Preview>
      </Section>

      <Section title="Shortcut and long text" description="A Kbd takes the tooltip's colors; long text wraps at 320.">
        <Preview
          class="pt-28 gap-28"
          code={[
            `<Button tooltip={<Tooltip>Save <Kbd>⌘S</Kbd></Tooltip>}>Save</Button>`,
            `<Text class="truncate" tooltip={<Tooltip delay={0}>{long}</Tooltip>}>{long}</Text>`,
          ].join("\n")}
        >
          <Button
            tooltip={
              <Tooltip open>
                <Text>Save</Text>
                <Kbd>
                  <Text>⌘S</Text>
                </Kbd>
              </Tooltip>
            }
          >
            <Text>Save</Text>
          </Button>
          <View class="w-48">
            <Text
              class="truncate text-sm text-foreground"
              tooltip={
                <Tooltip open placement="top">
                  <Text>{long}</Text>
                </Tooltip>
              }
            >
              {long}
            </Text>
          </View>
        </Preview>
      </Section>

      <Section title="Delay" description="tooltip-delay (500) by default; 0 for what the user is there to read.">
        <Preview code={delays.map((d) => `<Button variant="secondary" tooltip={<Tooltip delay={${d}}>…</Tooltip>}>${d} ms</Button>`).join("\n")}>
          {delays.map((d) => (
            <Button
              variant="secondary"
              tooltip={
                <Tooltip delay={d}>
                  <Text>{`Shown after ${d} ms`}</Text>
                </Tooltip>
              }
            >
              <Text>{`${d} ms`}</Text>
            </Button>
          ))}
        </Preview>
      </Section>

      <Section title="States" description="A disabled button still shows its tooltip (it may say why); disabled on the tooltip shows none.">
        <Preview
          code={[
            `<Button disabled tooltip={<Tooltip>Nothing to save</Tooltip>}>Save</Button>`,
            `<Button variant="outline" tooltip={<Tooltip disabled>Never shown</Tooltip>}>No tooltip</Button>`,
          ].join("\n")}
        >
          <Button
            disabled
            tooltip={
              <Tooltip>
                <Text>Nothing to save</Text>
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
        </Preview>
      </Section>

      <Section title="Controlled" description="The app owns open and follows onOpenChange.">
        <ControlledDemo />
      </Section>
    </Page>
  );
}

function ControlledDemo() {
  const open = signal(false);
  const reason = signal("—");
  return (
    <Preview
      class="pt-16"
      code={[
        `const open = signal(false);`,
        `<View tooltip={<Tooltip open={open()} onOpenChange={(e) => open.set(e.open)}>Synced 2 min ago</Tooltip>}>`,
        `  <Button variant="outline">Status</Button>`,
        `</View>`,
      ].join("\n")}
    >
      <View
        tooltip={
          <Tooltip
            open={open()}
            onOpenChange={(e) => {
              open.set(e.open);
              reason.set(e.reason);
            }}
          >
            <Text>Synced 2 min ago</Text>
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
      <Caption>{`open: ${open()}, last reason: ${reason()}`}</Caption>
    </Preview>
  );
}
