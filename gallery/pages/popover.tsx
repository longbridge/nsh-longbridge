// The Popover page: a layer's dimensions held open, the four sides and the
// alignments, a share panel, a filter, controlled.

import { signal, Text, View } from "nsh";
import {
  Badge,
  Button,
  Checkbox,
  Input,
  type InputRef,
  Label,
  Popover,
  PopoverDescription,
  PopoverHeader,
  PopoverTitle,
  Separator,
  Switch,
} from "nsh-longbridge";
import { Demo, Demos, Page, Section } from "../ui/layout";
import { TokensSection } from "../ui/tokens";

const sides = ["top", "right", "bottom", "left"] as const;
const aligns = ["start", "center", "end"] as const;
const title = (s: string) => s[0].toUpperCase() + s.slice(1);

const statuses = ["Backlog", "In progress", "In review", "Done"];
const people = [
  { initials: "SD", name: "Sofia Davis", access: "Can edit" },
  { initials: "JL", name: "Jackson Lee", access: "Can view" },
];

export function PopoverPage() {
  return (
    <Page
      eyebrow="Components"
      title="Popover"
      description="Rich content in a floating panel next to the button that opened it. A press outside or Escape closes it; on touch screens it stays anchored."
    >
      <Demo hero class="pb-64 justify-start pt-16">
        <DimensionsPopover />
      </Demo>

      <Section title="Sides and alignment" description="On the trigger's side, flipped to the other when only that one has room; aligned to its start, center or end. Press each.">
        <Demos>
          <Demo row label="placement">
            {sides.map((p) => (
              <Popover placement={p} trigger={<Button variant="outline"><Text>{title(p)}</Text></Button>}>
                <PopoverHeader>
                  <PopoverTitle>
                    <Text>{`Placed ${p}`}</Text>
                  </PopoverTitle>
                  <PopoverDescription>
                    <Text>It moves 8 points toward the trigger as it opens.</Text>
                  </PopoverDescription>
                </PopoverHeader>
              </Popover>
            ))}
          </Demo>
          <Demo row label="align">
            {aligns.map((a) => (
              <Popover align={a} class="w-56" trigger={<Button variant="secondary"><Text>{title(a)}</Text></Button>}>
                <Text class="text-sm">{`Aligned to the trigger's ${a}.`}</Text>
              </Popover>
            ))}
          </Demo>
        </Demos>
      </Section>

      <Section title="In a product" description="A share panel from a document's header, and a filter from a list's toolbar. The panel's class or style changes its width and padding.">
        <Demos>
          <Demo class="items-stretch">
            <ShareScene />
          </Demo>
          <Demo class="items-stretch">
            <FilterScene />
          </Demo>
        </Demos>
      </Section>

      <Section title="Controlled" description="The app owns open: here a switch elsewhere opens it too, and onOpenChange says why it closed.">
        <ControlledDemo />
      </Section>

      <TokensSection
        prefix="popover"
        component="Popover"
        note="The first six are the popup surface's: the menus and Select's list read them too."
      />
    </Page>
  );
}

/** shadcn's example: a layer's dimensions, open as the page shows. */
function DimensionsPopover() {
  const fields = [
    { label: "Width", value: "100%" },
    { label: "Max. width", value: "300px" },
    { label: "Height", value: "25px" },
    { label: "Max. height", value: "none" },
  ];
  return (
    <Popover defaultOpen class="w-80" trigger={<Button variant="outline"><Text>Open popover</Text></Button>}>
      <View class="gap-4">
        <PopoverHeader>
          <PopoverTitle>
            <Text>Dimensions</Text>
          </PopoverTitle>
          <PopoverDescription>
            <Text>Set the dimensions for the layer.</Text>
          </PopoverDescription>
        </PopoverHeader>
        <View class="gap-2">
          {fields.map((f) => {
            const field = signal<InputRef>();
            return (
              <View class="flex-row items-center gap-4">
                <View class="w-24">
                  <Label htmlFor={field()}>
                    <Text>{f.label}</Text>
                  </Label>
                </View>
                <View class="flex-1">
                  <Input ref={field} size="sm" defaultValue={f.value} />
                </View>
              </View>
            );
          })}
        </View>
      </View>
    </Popover>
  );
}

function ShareScene() {
  return (
    <View class="w-full gap-3 rounded-lg border border-border p-4">
      <View class="flex-row items-center gap-3">
        <View class="flex-1 gap-0.5">
          <Text class="text-sm font-medium text-foreground">Q3 planning</Text>
          <Text class="text-xs text-muted-foreground">Edited 5 minutes ago</Text>
        </View>
        <Popover align="end" class="w-80" trigger={<Button size="sm" icon="plus"><Text>Share</Text></Button>}>
          <View class="gap-4">
            <PopoverHeader>
              <PopoverTitle>
                <Text>Share this document</Text>
              </PopoverTitle>
              <PopoverDescription>
                <Text>Anyone with the link can view it.</Text>
              </PopoverDescription>
            </PopoverHeader>
            <View class="flex-row gap-2">
              <View class="flex-1">
                <Input size="sm" defaultValue="https://example.com/d/q3-plan" accessibilityLabel="Link" readonly />
              </View>
              <Button size="sm" variant="secondary">
                <Text>Copy</Text>
              </Button>
            </View>
            <Separator />
            <View class="gap-3">
              <Text class="text-sm font-medium">People with access</Text>
              {people.map((p) => (
                <View class="flex-row items-center gap-3">
                  <Badge variant="secondary" class="size-8 rounded-full px-0 py-0">
                    <Text>{p.initials}</Text>
                  </Badge>
                  <Text class="flex-1 text-sm">{p.name}</Text>
                  <Text class="text-xs text-muted-foreground">{p.access}</Text>
                </View>
              ))}
            </View>
          </View>
        </Popover>
      </View>
      <View class="gap-2">
        <View class="h-2.5 w-3/4 rounded-full bg-muted" />
        <View class="h-2.5 w-2/3 rounded-full bg-muted" />
        <View class="h-2.5 w-1/2 rounded-full bg-muted" />
      </View>
    </View>
  );
}

function FilterScene() {
  const picked = signal<string[]>(["In progress", "In review"]);
  return (
    <View class="w-full gap-3 rounded-lg border border-border p-4">
      <View class="flex-row items-center gap-2">
        <Text class="flex-1 text-sm font-medium text-foreground">Issues</Text>
        <Popover
          align="end"
          class="w-56 p-2"
          trigger={
            <Button size="sm" variant="outline" iconEnd="chevron-down">
              <Text>{picked().length === 0 ? "Status" : `Status · ${picked().length}`}</Text>
            </Button>
          }
        >
          <View class="gap-1">
            {statuses.map((s) => (
              <View class="rounded-sm px-2 py-1.5">
                <Checkbox
                  size="sm"
                  checked={picked().includes(s)}
                  onChange={(e) => picked.set(e.checked ? [...picked(), s] : picked().filter((x) => x !== s))}
                >
                  <Text>{s}</Text>
                </Checkbox>
              </View>
            ))}
            <Separator class="my-1" />
            <Button size="xs" variant="ghost" onPress={() => picked.set([])}>
              <Text>Clear filters</Text>
            </Button>
          </View>
        </Popover>
      </View>
      {statuses
        .filter((s) => picked().length === 0 || picked().includes(s))
        .map((s, i) => (
          <View class="flex-row items-center gap-2">
            <Text class="font-mono text-xs text-muted-foreground">{`ENG-${120 + i}`}</Text>
            <Text class="flex-1 text-sm text-foreground">{`A task that is ${s.toLowerCase()}`}</Text>
            <Badge variant="outline">
              <Text>{s}</Text>
            </Badge>
          </View>
        ))}
    </View>
  );
}

function ControlledDemo() {
  const open = signal(false);
  const reason = signal("—");
  return (
    <Demo row class="gap-6 pb-40 justify-start pt-12" label={`open: ${open()} · last change: ${reason()}`}>
      <Popover
        open={open()}
        onOpenChange={(e) => {
          open.set(e.open);
          reason.set(e.reason);
        }}
        trigger={<Button variant="outline"><Text>Notifications</Text></Button>}
      >
        <PopoverHeader>
          <PopoverTitle>
            <Text>3 unread</Text>
          </PopoverTitle>
          <PopoverDescription>
            <Text>Escape or a press outside closes it.</Text>
          </PopoverDescription>
        </PopoverHeader>
      </Popover>
      <Switch checked={open()} onChange={(e) => open.set(e.checked)}>
        <Text>Open</Text>
      </Switch>
    </Demo>
  );
}
