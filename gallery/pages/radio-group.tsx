// The RadioGroup page: states, sizes, a row, parts.

import { signal, Text } from "nsh";
import { RadioGroup, RadioGroupItem, Size } from "nsh-longbridge";
import { Caption, Page, Preview, Section } from "../ui";

const sizes: Size[] = ["xs", "sm", "md", "lg"];

const plans: { value: string; label: string; disabled?: boolean }[] = [
  { value: "monthly", label: "Monthly" },
  { value: "yearly", label: "Yearly" },
  { value: "lifetime", label: "Lifetime", disabled: true },
];

const states: { label: string; value?: string; invalid?: boolean; disabled?: boolean }[] = [
  { label: "Rest", value: "monthly" },
  { label: "Nothing checked" },
  { label: "Invalid", invalid: true },
  { label: "Invalid, checked", invalid: true, value: "monthly" },
  { label: "Disabled", disabled: true, value: "monthly" },
];

const sorts = [
  { value: "name", label: "Name" },
  { value: "date", label: "Date" },
  { value: "size", label: "Size" },
];

const stateCode = (s: (typeof states)[number]) =>
  `<RadioGroup accessibilityLabel="Plan"${s.value ? ` value="${s.value}"` : ""}${s.invalid ? " invalid" : ""}${
    s.disabled ? " disabled" : ""
  }>…</RadioGroup>`;

export function RadioGroupPage() {
  return (
    <Page
      title="RadioGroup"
      description="Exactly one of a small set of options, all visible. shadcn/ui's radio group on nsh/base's RadioGroup and Radio: the group holds the value, its items follow its size, disabled and invalid states."
    >
      <Section
        title="Usage"
        description="The app owns the value: onChange reports the pressed item's. The group is one stop of Tab; the arrow keys move between items, skip disabled ones and check them."
      >
        <PlanDemo />
      </Section>

      <Section title="States" description="Invalid marks every circle, checked too. Disabled on the group disables every item. Press Tab to see the focus ring on the circle.">
        {states.map((s) => (
          <Preview code={stateCode(s)}>
            <Caption>{s.label}</Caption>
            <RadioGroup accessibilityLabel={s.label} value={s.value} invalid={s.invalid} disabled={s.disabled}>
              {plans.map((p) => (
                <RadioGroupItem value={p.value} disabled={p.disabled}>
                  <Text>{p.label}</Text>
                </RadioGroupItem>
              ))}
            </RadioGroup>
          </Preview>
        ))}
      </Section>

      <Section
        title="Sizes"
        description="Set on the group. The circle is half the control height of its size, the side of a Checkbox's box of the same size; the dot is half the circle."
      >
        {sizes.map((size) => (
          <Preview code={`<RadioGroup size="${size}" class="flex-row" value="name">…</RadioGroup>`}>
            <Caption>{size}</Caption>
            <RadioGroup size={size} class="flex-row" accessibilityLabel={`Sort by, ${size}`} value="name">
              {sorts.map((s) => (
                <RadioGroupItem value={s.value}>
                  <Text>{s.label}</Text>
                </RadioGroupItem>
              ))}
            </RadioGroup>
          </Preview>
        ))}
      </Section>

      <Section title="A row" description='class="flex-row" lays the items out in a row; both pairs of arrow keys work either way.'>
        <SortDemo />
      </Section>

      <Section title="Parts" description="indicatorClass styles the circle, dotClass the dot, labelClass the label; class styles the group.">
        <Preview
          code={`<RadioGroupItem value="a" indicatorClass="border-primary" dotClass="bg-destructive">Custom</RadioGroupItem>`}
        >
          <RadioGroup accessibilityLabel="Parts" value="a">
            <RadioGroupItem value="a" indicatorClass="border-primary" dotClass="bg-destructive">
              <Text>Custom</Text>
            </RadioGroupItem>
          </RadioGroup>
        </Preview>
      </Section>
    </Page>
  );
}

function PlanDemo() {
  const plan = signal<string | undefined>(undefined);
  return (
    <Preview
      class="flex-col items-start"
      code={[
        `const plan = signal<string | undefined>(undefined);`,
        `<RadioGroup accessibilityLabel="Plan" value={plan()} invalid={plan() === undefined} onChange={({ value }) => plan.set(value)}>`,
        `  <RadioGroupItem value="monthly">Monthly</RadioGroupItem>`,
        `  <RadioGroupItem value="yearly">Yearly</RadioGroupItem>`,
        `  <RadioGroupItem value="lifetime" disabled>Lifetime</RadioGroupItem>`,
        `</RadioGroup>`,
      ].join("\n")}
    >
      <RadioGroup
        accessibilityLabel="Plan"
        value={plan()}
        invalid={plan() === undefined}
        onChange={({ value }) => plan.set(value)}
      >
        {plans.map((p) => (
          <RadioGroupItem value={p.value} disabled={p.disabled}>
            <Text>{p.label}</Text>
          </RadioGroupItem>
        ))}
      </RadioGroup>
      <Caption>{plan() ? `Plan: ${plan()}` : "Choose a plan"}</Caption>
    </Preview>
  );
}

function SortDemo() {
  const sort = signal("name");
  return (
    <Preview
      code={`<RadioGroup class="flex-row" size="sm" accessibilityLabel="Sort by" value={sort()} onChange={({ value }) => sort.set(value)}>…</RadioGroup>`}
    >
      <RadioGroup class="flex-row" size="sm" accessibilityLabel="Sort by" value={sort()} onChange={({ value }) => sort.set(value)}>
        {sorts.map((s) => (
          <RadioGroupItem value={s.value}>
            <Text>{s.label}</Text>
          </RadioGroupItem>
        ))}
      </RadioGroup>
    </Preview>
  );
}
