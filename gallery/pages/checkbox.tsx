// The Checkbox page: a terms row, "select all", states, sizes, a table's boxes.

import { signal, Text, View } from "nsh";
import { Badge, Checkbox, type Size, Table, TableBody, TableCell, TableHead, TableHeader, TableRow } from "nsh-longbridge";
import { Demo, Demos, Page, Row, Section } from "../ui/layout";
import { TokensSection } from "../ui/tokens";

const sizes: Size[] = ["xs", "sm", "md", "lg"];

const states: { label: string; checked?: boolean; indeterminate?: boolean; disabled?: boolean; invalid?: boolean }[] = [
  { label: "Unchecked" },
  { label: "Checked", checked: true },
  { label: "Indeterminate", indeterminate: true },
  { label: "Invalid", invalid: true },
  { label: "Disabled", disabled: true },
  { label: "Disabled, checked", disabled: true, checked: true },
];

const notifications = ["Mentions", "Approvals", "Weekly summary"];

const rows = [
  { id: "INV-001", customer: "Sofia Davis", amount: "$250.00", status: "Paid" },
  { id: "INV-002", customer: "Jackson Lee", amount: "$150.00", status: "Pending" },
  { id: "INV-003", customer: "Isabella Nguyen", amount: "$350.00", status: "Paid" },
];

export function CheckboxPage() {
  return (
    <Page
      eyebrow="Components"
      title="Checkbox"
      description="An option turned on or off, applied when the form is submitted: checked, unchecked or indeterminate, four sizes, the label pressable."
    >
      <HeroDemo />

      <Section title="Select all" description="Indeterminate while only some are selected; a press on it selects them all.">
        <SelectAllDemo />
      </Section>

      <Section
        title="States"
        description="Invalid marks the edge of an unchecked box and its focus ring. Disabled dims the box and its label together. Tab shows the ring on the box."
      >
        <Demo class="items-stretch">
          <View class="flex-row flex-wrap gap-x-10 gap-y-4 self-center">
            {states.map((s) => (
              <View class="w-44">
                <Checkbox checked={s.checked} indeterminate={s.indeterminate} invalid={s.invalid} disabled={s.disabled}>
                  <Text>{s.label}</Text>
                </Checkbox>
              </View>
            ))}
          </View>
        </Demo>
      </Section>

      <Section
        title="Sizes"
        description="The box is half the control height of its size (12, 14, 16, 18; larger on touch screens), and the label's text follows."
      >
        <Demo class="items-stretch">
          {sizes.map((size) => (
            <Row label={size}>
              <View class="flex-row items-center gap-8">
                <Checkbox size={size} checked>
                  <Text>Email me a copy</Text>
                </Checkbox>
                <Checkbox size={size} indeterminate>
                  <Text>Some files</Text>
                </Checkbox>
              </View>
            </Row>
          ))}
        </Demo>
      </Section>

      <Section title="A box alone" description="In a table row it names itself to screen readers; on touch screens it takes presses up to 44 points around it.">
        <TableDemo />
      </Section>

      <TokensSection prefix="checkbox" component="Checkbox" />
    </Page>
  );
}

function HeroDemo() {
  const agreed = signal(false);
  const news = signal(true);
  return (
    <Demo hero>
      <View class="w-full max-w-[360px] gap-6">
        <Checkbox checked={agreed()} onChange={(e) => agreed.set(e.checked)}>
          <Text>Accept terms and conditions</Text>
        </Checkbox>
        <Checkbox checked={news()} onChange={(e) => news.set(e.checked)}>
          <View class="gap-1.5">
            <Text>Product updates</Text>
            <Text class="font-normal text-muted-foreground">A short email when something new ships, once a month at most.</Text>
          </View>
        </Checkbox>
        <Checkbox disabled>
          <Text>Share usage data (managed by your organization)</Text>
        </Checkbox>
      </View>
    </Demo>
  );
}

function SelectAllDemo() {
  const selected = signal<string[]>(["Mentions"]);
  const toggle = (item: string, on: boolean) =>
    selected.set(on ? [...selected(), item] : selected().filter((s) => s !== item));
  return (
    <Demo>
      <View class="w-64 gap-3">
        <Checkbox
          checked={selected().length === notifications.length}
          indeterminate={selected().length > 0 && selected().length < notifications.length}
          onChange={({ checked }) => selected.set(checked ? [...notifications] : [])}
        >
          <Text>All notifications</Text>
        </Checkbox>
        <View class="ml-6 gap-3">
          {notifications.map((item) => (
            <Checkbox checked={selected().includes(item)} onChange={({ checked }) => toggle(item, checked)}>
              <Text>{item}</Text>
            </Checkbox>
          ))}
        </View>
      </View>
    </Demo>
  );
}

function TableDemo() {
  const picked = signal<string[]>(["INV-002"]);
  const all = () => picked().length === rows.length;
  return (
    <Demo class="items-stretch">
      <View class="w-full max-w-[560px] self-center overflow-hidden rounded-lg border border-border">
        <Table size="lg" accessibilityLabel="Invoices">
          <TableHeader>
            <TableRow>
              <TableHead class="flex-none pl-4">
                <Checkbox
                  accessibilityLabel="Select all"
                  checked={all()}
                  indeterminate={picked().length > 0 && !all()}
                  onChange={(e) => picked.set(e.checked ? rows.map((r) => r.id) : [])}
                />
              </TableHead>
              <TableHead class="w-24 flex-none">
                <Text>Invoice</Text>
              </TableHead>
              <TableHead>
                <Text>Customer</Text>
              </TableHead>
              <TableHead class="w-24 flex-none items-end pr-4">
                <Text>Amount</Text>
              </TableHead>
            </TableRow>
          </TableHeader>
          <TableBody>
            {rows.map((r) => (
              <TableRow selected={picked().includes(r.id)}>
                <TableCell class="flex-none pl-4">
                  <Checkbox
                    accessibilityLabel={`Select ${r.id}`}
                    checked={picked().includes(r.id)}
                    onChange={(e) => picked.set(e.checked ? [...picked(), r.id] : picked().filter((x) => x !== r.id))}
                  />
                </TableCell>
                <TableCell class="w-24 flex-none">
                  <Text>{r.id}</Text>
                </TableCell>
                <TableCell>
                  <View class="flex-row items-center gap-2">
                    <Text>{r.customer}</Text>
                    <If when={r.status === "Pending"}>
                      <Badge variant="secondary">
                        <Text>Pending</Text>
                      </Badge>
                    </If>
                  </View>
                </TableCell>
                <TableCell class="w-24 flex-none items-end pr-4">
                  <Text class="tabular-nums">{r.amount}</Text>
                </TableCell>
              </TableRow>
            ))}
          </TableBody>
        </Table>
      </View>
      <Text class="self-center text-xs text-muted-foreground">{`${picked().length} of ${rows.length} selected`}</Text>
    </Demo>
  );
}
