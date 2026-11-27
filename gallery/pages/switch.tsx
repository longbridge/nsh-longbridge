// The Switch page: a settings panel, states, sizes, a track alone in a
// table, parts.

import { signal, Text, View } from "nsh";
import { Badge, Separator, type Size, Switch } from "nsh-longbridge";
import { Demo, Demos, Page, Row, Section } from "../ui/layout";
import { TokensSection } from "../ui/tokens";

const sizes: Size[] = ["xs", "sm", "md", "lg"];

const settings = [
  { id: "sync", title: "Sync across devices", description: "Your settings follow you to every signed-in device." },
  { id: "updates", title: "Install updates automatically", description: "Restart when you are ready; nothing is lost." },
  { id: "beta", title: "Try features early", description: "Turn on what is still being built. It may change." },
];

const states: { label: string; checked?: boolean; disabled?: boolean }[] = [
  { label: "Off" },
  { label: "On", checked: true },
  { label: "Disabled", disabled: true },
  { label: "Disabled, on", disabled: true, checked: true },
];

const flags = [
  { name: "new-checkout", env: "Production", on: true },
  { name: "dark-mode-v2", env: "Staging", on: false },
  { name: "ai-summaries", env: "Preview", on: true },
];

export function SwitchPage() {
  return (
    <Page
      eyebrow="Components"
      title="Switch"
      description="A setting that takes effect at once. Four sizes, the label pressable, the thumb stretching while held."
    >
      <Demo hero>
        <SettingsDemo />
      </Demo>

      <Section title="States" description="Disabled dims the track and its label together. Tab shows the ring on the track.">
        <Demo row class="gap-x-10 gap-y-4">
          {states.map((s) => (
            <Switch checked={s.checked} disabled={s.disabled}>
              <Text>{s.label}</Text>
            </Switch>
          ))}
        </Demo>
      </Section>

      <Section
        title="Sizes"
        description="The thumb is half the control height of its size, the track twice as wide (md: 32 × 18, shadcn's), the side of a Checkbox of the same size."
      >
        <SizesDemo />
      </Section>

      <Section title="A track alone" description="In a table row, without a label: it names itself with accessibilityLabel. On touch screens it takes presses up to 44 points around it.">
        <FlagsDemo />
      </Section>

      <Section title="Parts" description="trackClass styles the track, thumbClass the thumb, labelClass the label; class styles the row.">
        <Demos>
          <Demo row label="A muted label, a raised thumb">
            <Switch checked labelClass="font-normal text-muted-foreground" thumbClass="shadow-md">
              <Text>Quiet hours</Text>
            </Switch>
          </Demo>
          <Demo row label="The label first: class reverses the row">
            <Switch checked class="flex-row-reverse">
              <Text>Wi-Fi</Text>
            </Switch>
          </Demo>
        </Demos>
      </Section>

      <TokensSection prefix="switch" component="Switch" />
    </Page>
  );
}

function SettingsDemo() {
  const on = signal<string[]>(["sync", "updates"]);
  return (
    <View class="w-full max-w-[440px] rounded-xl border border-border bg-card">
      {settings.map((s, i) => (
        <View>
          <If when={i > 0}>
            <Separator />
          </If>
          <View class="flex-row items-center gap-6 p-4">
            <View class="flex-1 gap-1">
              <View class="flex-row items-center gap-2">
                <Text class="text-sm font-medium text-card-foreground">{s.title}</Text>
                <If when={s.id === "beta"}>
                  <Badge variant="outline">
                    <Text>Beta</Text>
                  </Badge>
                </If>
              </View>
              <Text class="text-sm text-muted-foreground">{s.description}</Text>
            </View>
            <Switch
              checked={on().includes(s.id)}
              accessibilityLabel={s.title}
              onChange={(e) => on.set(e.checked ? [...on(), s.id] : on().filter((x) => x !== s.id))}
            />
          </View>
        </View>
      ))}
    </View>
  );
}

function SizesDemo() {
  const on = signal(true);
  return (
    <Demo class="items-stretch">
      {sizes.map((size) => (
        <Row label={size}>
          <View class="flex-row items-center gap-8">
            <Switch size={size} checked={on()} onChange={(e) => on.set(e.checked)}>
              <Text>Airplane mode</Text>
            </Switch>
            <Switch size={size}>
              <Text>Bluetooth</Text>
            </Switch>
          </View>
        </Row>
      ))}
    </Demo>
  );
}

function FlagsDemo() {
  const on = signal(flags.filter((f) => f.on).map((f) => f.name));
  return (
    <Demo class="items-stretch">
      <View class="w-full max-w-[520px] self-center rounded-lg border border-border">
        <View class="h-10 flex-row items-center gap-4 border-b border-border px-4">
          <Text class="flex-1 text-xs font-medium text-muted-foreground">Flag</Text>
          <Text class="w-24 text-xs font-medium text-muted-foreground">Environment</Text>
          <Text class="w-12 text-right text-xs font-medium text-muted-foreground">On</Text>
        </View>
        {flags.map((f, i) => (
          <View class={`h-12 flex-row items-center gap-4 px-4 ${i > 0 ? "border-t border-border" : ""}`}>
            <Text class="flex-1 font-mono text-sm text-foreground">{f.name}</Text>
            <View class="w-24 items-start">
              <Badge variant="secondary">
                <Text>{f.env}</Text>
              </Badge>
            </View>
            <View class="w-12 items-end">
              <Switch
                size="sm"
                checked={on().includes(f.name)}
                accessibilityLabel={f.name}
                onChange={(e) => on.set(e.checked ? [...on(), f.name] : on().filter((x) => x !== f.name))}
              />
            </View>
          </View>
        ))}
      </View>
    </Demo>
  );
}
