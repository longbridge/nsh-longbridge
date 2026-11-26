// The Switch page: states, sizes, a track alone, parts.

import { signal, Text } from "nsh";
import { Size, Switch } from "nsh-longbridge";
import { Caption, Page, Preview, Section } from "../ui";

const sizes: Size[] = ["xs", "sm", "md", "lg"];

const states: { label: string; checked?: boolean; disabled?: boolean }[] = [
  { label: "Off" },
  { label: "On", checked: true },
  { label: "Disabled", disabled: true },
  { label: "Disabled, on", disabled: true, checked: true },
];

const settings = ["Airplane mode", "Bluetooth", "Do not disturb"];

const stateCode = (s: (typeof states)[number]) =>
  `<Switch${s.checked ? " checked" : ""}${s.disabled ? " disabled" : ""}>${s.label}</Switch>`;

export function SwitchPage() {
  return (
    <Page
      title="Switch"
      description="A setting that takes effect at once. shadcn/ui's switch on nsh/base's Switch: four sizes, the label pressable, the thumb stretching while held."
    >
      <Section title="Usage" description="The app owns the value: onChange reports the next one. A press on the label toggles it too; hold it to see the thumb stretch.">
        <SettingsDemo />
      </Section>

      <Section title="States" description="Disabled dims the track and its label together. Press Tab to see the focus ring on the track.">
        <Preview class="flex-col items-start" code={states.map(stateCode).join("\n")}>
          {states.map((s) => (
            <Switch checked={s.checked} disabled={s.disabled}>
              <Text>{s.label}</Text>
            </Switch>
          ))}
        </Preview>
      </Section>

      <Section
        title="Sizes"
        description="The thumb is half the control height of its size, the track twice as wide (md: 32 × 18, shadcn's). A Checkbox of the same size has the thumb's side."
      >
        {sizes.map((size) => (
          <Preview code={`<Switch size="${size}">Label</Switch>\n<Switch size="${size}" checked>Label</Switch>`}>
            <Caption>{size}</Caption>
            <Switch size={size}>
              <Text>Off</Text>
            </Switch>
            <Switch size={size} checked>
              <Text>On</Text>
            </Switch>
          </Preview>
        ))}
      </Section>

      <Section title="A track alone" description="Without a label, give it an accessibilityLabel. On touch screens it takes presses up to 44 points around it.">
        <TrackAloneDemo />
      </Section>

      <Section title="Parts" description="trackClass styles the track, thumbClass the thumb, labelClass the label; class styles the row.">
        <Preview
          code={`<Switch checked labelClass="font-normal text-muted-foreground" thumbClass="shadow-sm">Muted label</Switch>`}
        >
          <Switch checked labelClass="font-normal text-muted-foreground" thumbClass="shadow-sm">
            <Text>Muted label</Text>
          </Switch>
        </Preview>
      </Section>
    </Page>
  );
}

function SettingsDemo() {
  const on = signal<string[]>(["Bluetooth"]);
  return (
    <Preview
      class="flex-col items-start"
      code={[
        `const airplane = signal(false);`,
        `<Switch checked={airplane()} onChange={({ checked }) => airplane.set(checked)}>`,
        `  Airplane mode`,
        `</Switch>`,
      ].join("\n")}
    >
      {settings.map((s) => (
        <Switch
          checked={on().includes(s)}
          onChange={({ checked }) => on.set(checked ? [...on(), s] : on().filter((x) => x !== s))}
        >
          <Text>{s}</Text>
        </Switch>
      ))}
    </Preview>
  );
}

function TrackAloneDemo() {
  const on = signal(true);
  return (
    <Preview code={`<Switch checked={on()} onChange={({ checked }) => on.set(checked)} accessibilityLabel="Airplane mode" />`}>
      {sizes.map((size) => (
        <Switch size={size} checked={on()} onChange={(e) => on.set(e.checked)} accessibilityLabel="Airplane mode" />
      ))}
    </Preview>
  );
}
