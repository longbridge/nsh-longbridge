// The Checkbox page: states, sizes, "select all", a box alone, parts.

import { signal, Text } from "nsh";
import { Checkbox, Size } from "nsh-longbridge";
import { Caption, Page, Preview, Section } from "../ui";

const sizes: Size[] = ["xs", "sm", "md", "lg"];

const states: { label: string; checked?: boolean; indeterminate?: boolean; disabled?: boolean; invalid?: boolean }[] = [
  { label: "Unchecked" },
  { label: "Checked", checked: true },
  { label: "Indeterminate", indeterminate: true },
  { label: "Invalid", invalid: true },
  { label: "Invalid, checked", invalid: true, checked: true },
  { label: "Disabled", disabled: true },
  { label: "Disabled, checked", disabled: true, checked: true },
];

const notifications = ["Mentions", "Approvals", "Weekly summary"];

const stateCode = (s: (typeof states)[number]) => {
  const props = [
    s.checked ? " checked" : "",
    s.indeterminate ? " indeterminate" : "",
    s.invalid ? " invalid" : "",
    s.disabled ? " disabled" : "",
  ].join("");
  return `<Checkbox${props}>${s.label}</Checkbox>`;
};

export function CheckboxPage() {
  return (
    <Page
      title="Checkbox"
      description="An option the user turns on or off, taking effect when the form is submitted. shadcn/ui's checkbox on nsh/base's Checkbox: checked, unchecked or indeterminate, four sizes, the label pressable."
    >
      <Section title="Usage" description="The app owns the value: onChange reports the next one. A press on the label checks the box too.">
        <TermsDemo />
      </Section>

      <Section
        title="States"
        description="Invalid marks the edge of an unchecked box and the focus ring. Disabled dims the box and its label together. Press Tab to see the focus ring on the box."
      >
        <Preview class="flex-col items-start" code={states.map(stateCode).join("\n")}>
          {states.map((s) => (
            <Checkbox checked={s.checked} indeterminate={s.indeterminate} invalid={s.invalid} disabled={s.disabled}>
              <Text>{s.label}</Text>
            </Checkbox>
          ))}
        </Preview>
      </Section>

      <Section
        title="Sizes"
        description="The box is half the control height of its size (12 / 14 / 16 / 18; 16 / 18 / 20 / 24 on touch screens), the label's text follows."
      >
        {sizes.map((size) => (
          <Preview
            code={`<Checkbox size="${size}">Label</Checkbox>\n<Checkbox size="${size}" checked>Label</Checkbox>\n<Checkbox size="${size}" indeterminate>Label</Checkbox>`}
          >
            <Caption>{size}</Caption>
            <Checkbox size={size}>
              <Text>Unchecked</Text>
            </Checkbox>
            <Checkbox size={size} checked>
              <Text>Checked</Text>
            </Checkbox>
            <Checkbox size={size} indeterminate>
              <Text>Indeterminate</Text>
            </Checkbox>
          </Preview>
        ))}
      </Section>

      <Section title="Select all" description="Indeterminate while only some items are selected; a press selects all.">
        <SelectAllDemo />
      </Section>

      <Section title="A box alone" description="Without a label, give it an accessibilityLabel. On touch screens it takes presses up to 44 points around it.">
        <BoxAloneDemo />
      </Section>

      <Section title="Long labels" description="One line sits level with the box; more lines start at its top.">
        <Preview class="flex-col items-start" code={`<Checkbox class="max-w-80">By continuing you agree to …</Checkbox>`}>
          <Checkbox class="max-w-80">
            <Text>By continuing you agree to the terms of service and the privacy policy, and to receive occasional emails.</Text>
          </Checkbox>
        </Preview>
      </Section>

      <Section title="Parts" description="indicatorClass styles the box, markClass the check, labelClass the label; class styles the row.">
        <Preview
          code={[
            `<Checkbox checked indicatorClass="rounded-full" labelClass="font-normal text-muted-foreground">`,
            `  Round box, muted label`,
            `</Checkbox>`,
          ].join("\n")}
        >
          <Checkbox checked indicatorClass="rounded-full" labelClass="font-normal text-muted-foreground">
            <Text>Round box, muted label</Text>
          </Checkbox>
        </Preview>
      </Section>
    </Page>
  );
}

function TermsDemo() {
  const agreed = signal(false);
  return (
    <Preview
      code={[
        `const agreed = signal(false);`,
        `<Checkbox checked={agreed()} onChange={({ checked }) => agreed.set(checked)}>`,
        `  Accept terms and conditions`,
        `</Checkbox>`,
      ].join("\n")}
    >
      <Checkbox checked={agreed()} onChange={({ checked }) => agreed.set(checked)}>
        <Text>Accept terms and conditions</Text>
      </Checkbox>
    </Preview>
  );
}

function SelectAllDemo() {
  const selected = signal<string[]>(["Mentions"]);
  const toggle = (item: string, on: boolean) =>
    selected.set(on ? [...selected(), item] : selected().filter((s) => s !== item));
  return (
    <Preview
      class="flex-col items-start"
      code={[
        `const all = ["Mentions", "Approvals", "Weekly summary"];`,
        `<Checkbox`,
        `  checked={selected().length === all.length}`,
        `  indeterminate={selected().length > 0 && selected().length < all.length}`,
        `  onChange={({ checked }) => selected.set(checked ? all : [])}`,
        `>`,
        `  All notifications`,
        `</Checkbox>`,
      ].join("\n")}
    >
      <Checkbox
        checked={selected().length === notifications.length}
        indeterminate={selected().length > 0 && selected().length < notifications.length}
        onChange={({ checked }) => selected.set(checked ? [...notifications] : [])}
      >
        <Text>All notifications</Text>
      </Checkbox>
      {notifications.map((item) => (
        <Checkbox
          class="ml-6"
          checked={selected().includes(item)}
          onChange={({ checked }) => toggle(item, checked)}
        >
          <Text>{item}</Text>
        </Checkbox>
      ))}
    </Preview>
  );
}

function BoxAloneDemo() {
  const checked = signal(true);
  return (
    <Preview
      code={`<Checkbox checked={checked()} onChange={({ checked }) => checked.set(checked)} accessibilityLabel="Select row" />`}
    >
      {sizes.map((size) => (
        <Checkbox
          size={size}
          checked={checked()}
          onChange={(e) => checked.set(e.checked)}
          accessibilityLabel="Select row"
        />
      ))}
    </Preview>
  );
}
