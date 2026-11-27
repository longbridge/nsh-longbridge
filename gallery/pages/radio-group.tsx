// The RadioGroup page: a shipping choice, states, sizes, a row, cards and
// parts.

import { signal, Text, View } from "nsh";
import { Badge, RadioGroup, RadioGroupItem, type Size } from "nsh-longbridge";
import { Demo, Demos, Page, Row, Section } from "../ui/layout";
import { TokensSection } from "../ui/tokens";

const sizes: Size[] = ["xs", "sm", "md", "lg"];

const shipping = [
  { value: "standard", label: "Standard", note: "4 to 6 business days · Free" },
  { value: "express", label: "Express", note: "2 to 3 business days · $9" },
  { value: "overnight", label: "Overnight", note: "Next business day · $24" },
  { value: "pickup", label: "Pick up in store", note: "No store near this address", disabled: true },
];

const plans = [
  { value: "monthly", label: "Monthly" },
  { value: "yearly", label: "Yearly" },
  { value: "lifetime", label: "Lifetime", disabled: true },
];

const states: { label: string; value?: string; invalid?: boolean; disabled?: boolean }[] = [
  { label: "At rest", value: "monthly" },
  { label: "Nothing checked" },
  { label: "Invalid", invalid: true },
  { label: "Disabled", disabled: true, value: "yearly" },
];

const sorts = [
  { value: "name", label: "Name" },
  { value: "date", label: "Date" },
  { value: "size", label: "Size" },
];

const tiers = [
  { value: "starter", label: "Starter", price: "$0", note: "1 project, community support" },
  { value: "pro", label: "Pro", price: "$20", note: "Unlimited projects, email support", badge: "Popular" },
  { value: "scale", label: "Scale", price: "$90", note: "SSO, audit logs, an uptime promise" },
];

export function RadioGroupPage() {
  return (
    <Page
      eyebrow="Components"
      title="Radio Group"
      description="Exactly one of a small set of options, all visible. The group holds the value and gives its items their size, disabled and invalid states."
    >
      <Demo hero>
        <ShippingDemo />
      </Demo>

      <Section
        title="States"
        description="Invalid marks every circle until one is chosen. Disabled on the group disables every item; an item can be disabled alone. Tab shows the ring on the circle."
      >
        <Demo class="items-stretch">
          <View class="w-full flex-row flex-wrap gap-x-10 gap-y-6">
            {states.map((s) => (
              <View class="flex-1 min-w-36 gap-3">
                <Text class="text-xs font-medium text-muted-foreground">{s.label}</Text>
                <RadioGroup accessibilityLabel={s.label} value={s.value} invalid={s.invalid} disabled={s.disabled}>
                  {plans.map((p) => (
                    <RadioGroupItem value={p.value} disabled={p.disabled}>
                      <Text>{p.label}</Text>
                    </RadioGroupItem>
                  ))}
                </RadioGroup>
              </View>
            ))}
          </View>
        </Demo>
      </Section>

      <Section
        title="Sizes and rows"
        description={`Size is set on the group: the circle is half its control height, the side of a Checkbox of the same size. A row is class="flex-row"; both pairs of arrow keys move either way.`}
      >
        <SortDemo />
      </Section>

      <Section title="Cards" description="An item's class and labelClass make it a card: the whole card is pressable, the circle stays the group's.">
        <TierDemo />
      </Section>

      <Section title="Parts" description="The indicatorClass prop styles the circle, dotClass the dot, labelClass the label.">
        <Demos>
          <Demo row label="A destructive dot">
            <RadioGroup accessibilityLabel="Parts" value="a" class="flex-row gap-6">
              <RadioGroupItem value="a" indicatorClass="border-destructive" dotClass="bg-destructive">
                <Text>Delete</Text>
              </RadioGroupItem>
              <RadioGroupItem value="b">
                <Text>Keep</Text>
              </RadioGroupItem>
            </RadioGroup>
          </Demo>
          <Demo row label="A muted label">
            <RadioGroup accessibilityLabel="Muted" value="a" class="flex-row gap-6">
              <RadioGroupItem value="a" labelClass="font-normal text-muted-foreground">
                <Text>Quiet</Text>
              </RadioGroupItem>
              <RadioGroupItem value="b" labelClass="font-normal text-muted-foreground">
                <Text>Calm</Text>
              </RadioGroupItem>
            </RadioGroup>
          </Demo>
        </Demos>
      </Section>

      <TokensSection prefix="radio-group" component="Radio Group" />
    </Page>
  );
}

function ShippingDemo() {
  const choice = signal<string | undefined>("express");
  return (
    <View class="w-full max-w-[380px] gap-4">
      <View class="gap-1">
        <Text class="text-sm font-medium text-foreground">Delivery</Text>
        <Text class="text-sm text-muted-foreground">Arrives at 221B Baker Street.</Text>
      </View>
      <RadioGroup
        accessibilityLabel="Delivery"
        value={choice()}
        onChange={(e) => choice.set(e.value)}
        class="gap-4"
      >
        {shipping.map((s) => (
          <RadioGroupItem value={s.value} disabled={s.disabled}>
            <View class="gap-1">
              <Text>{s.label}</Text>
              <Text class="text-xs font-normal text-muted-foreground">{s.note}</Text>
            </View>
          </RadioGroupItem>
        ))}
      </RadioGroup>
    </View>
  );
}

function SortDemo() {
  const sort = signal("date");
  return (
    <Demo class="items-stretch">
      {sizes.map((size) => (
        <Row label={size}>
          <RadioGroup
            size={size}
            class="flex-row gap-6"
            accessibilityLabel={`Sort by, ${size}`}
            value={sort()}
            onChange={(e) => sort.set(e.value)}
          >
            {sorts.map((s) => (
              <RadioGroupItem value={s.value}>
                <Text>{s.label}</Text>
              </RadioGroupItem>
            ))}
          </RadioGroup>
        </Row>
      ))}
    </Demo>
  );
}

function TierDemo() {
  const tier = signal("pro");
  return (
    <Demo class="items-stretch">
      <RadioGroup
        accessibilityLabel="Plan"
        value={tier()}
        onChange={(e) => tier.set(e.value)}
        class="w-full flex-row flex-wrap gap-3"
      >
        {tiers.map((t) => (
          <RadioGroupItem
            value={t.value}
            class={`flex-1 min-w-44 items-start rounded-lg border p-4 ${
              tier() === t.value ? "border-primary bg-accent/40" : "border-border"
            }`}
          >
            <View class="flex-1 gap-1.5">
              <View class="flex-row items-center gap-2">
                <Text>{t.label}</Text>
                <If when={t.badge}>
                  <Badge>
                    <Text>{t.badge!}</Text>
                  </Badge>
                </If>
              </View>
              <Text class="text-2xl font-semibold tabular-nums">{t.price}</Text>
              <Text class="text-xs font-normal text-muted-foreground">{t.note}</Text>
            </View>
          </RadioGroupItem>
        ))}
      </RadioGroup>
    </Demo>
  );
}
