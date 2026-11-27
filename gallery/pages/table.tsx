// The Table page: invoices with a footer and a caption, rows picked with
// boxes, a row's menu, sizes, a sticky header.

import { Scroll, signal, Text, View } from "nsh";
import {
  Badge,
  Button,
  Checkbox,
  DropdownMenu,
  DropdownMenuItem,
  DropdownMenuSeparator,
  type Size,
  Table,
  TableBody,
  TableCaption,
  TableCell,
  TableFooter,
  TableHead,
  TableHeader,
  TableRow,
} from "nsh-longbridge";
import { Demo, Page, Row, Section } from "../ui/layout";
import { TokensSection } from "../ui/tokens";

const sizes: Size[] = ["xs", "sm", "md", "lg"];

const invoices = [
  { id: "INV-001", status: "Paid", method: "Credit card", amount: "$250.00" },
  { id: "INV-002", status: "Pending", method: "PayPal", amount: "$150.00" },
  { id: "INV-003", status: "Unpaid", method: "Bank transfer", amount: "$350.00" },
  { id: "INV-004", status: "Paid", method: "Credit card", amount: "$450.00" },
  { id: "INV-005", status: "Paid", method: "PayPal", amount: "$550.00" },
];

// One class string per column, given to the title and every cell.
const col = {
  invoice: "w-[96px] flex-none",
  status: "flex-1",
  method: "flex-[1.4]",
  amount: "w-[96px] flex-none items-end",
};

const statusVariant = (s: string) => (s === "Paid" ? "secondary" : s === "Pending" ? "outline" : "destructive");

const members = [
  { name: "Sofia Davis", email: "sofia@example.com", role: "Owner" },
  { name: "Jackson Lee", email: "jackson@example.com", role: "Admin" },
  { name: "Isabella Nguyen", email: "isabella@example.com", role: "Member" },
  { name: "William Kim", email: "will@example.com", role: "Member" },
];

const trades = Array.from({ length: 24 }, (_, i) => ({
  time: `09:${String(30 + i).padStart(2, "0")}`,
  symbol: ["AAPL", "MSFT", "NVDA", "TSLA"][i % 4],
  side: i % 3 === 0 ? "Sell" : "Buy",
  qty: String(100 * ((i % 5) + 1)),
}));

export function TablePage() {
  return (
    <Page
      eyebrow="Components"
      title="Table"
      description="Rows the app writes: a header, a body, a footer and a caption; cells of any content, rows lit under the pointer and when selected. For sorting, resizing or thousands of rows, use DataTable."
    >
      <HeroDemo />

      <Section
        title="Picking rows"
        description="A box alone in a cell names itself to screen readers; its cell drops the padding after it. Picked rows take the selected fill."
      >
        <PickDemo />
      </Section>

      <Section title="A row's actions" description="A row holding an open menu stays lit while the menu is open.">
        <ActionsDemo />
      </Section>

      <Section
        title="Sizes"
        description="A body row is the control height of its size (24, 28, 32, 36; taller on touch screens), so a control of the same size fits it; the header is one spacing step taller."
      >
        <Demo class="items-stretch">
          {sizes.map((size) => (
            <Row label={size}>
              <View class="flex-1">
                <Table size={size} accessibilityLabel={`Invoices, ${size}`}>
                  <TableHeader>
                    <TableRow>
                      <TableHead class={col.invoice}>
                        <Text>Invoice</Text>
                      </TableHead>
                      <TableHead>
                        <Text>Method</Text>
                      </TableHead>
                      <TableHead class={col.amount}>
                        <Text>Amount</Text>
                      </TableHead>
                    </TableRow>
                  </TableHeader>
                  <TableBody>
                    {invoices.slice(0, 2).map((i) => (
                      <TableRow>
                        <TableCell class={col.invoice}>
                          <Text>{i.id}</Text>
                        </TableCell>
                        <TableCell>
                          <Text>{i.method}</Text>
                        </TableCell>
                        <TableCell class={col.amount}>
                          <Text>{i.amount}</Text>
                        </TableCell>
                      </TableRow>
                    ))}
                  </TableBody>
                </Table>
              </View>
            </Row>
          ))}
        </Demo>
      </Section>

      <Section
        title="Sticky header"
        description="In a scroll view the header stays at the top, filled so the rows do not show through it."
      >
        <Demo class="items-stretch">
          <View class="w-full max-w-[560px] self-center overflow-hidden rounded-lg border border-border">
            <Scroll class="h-[240px]">
              <Table accessibilityLabel="Trades">
                <TableHeader sticky>
                  <TableRow>
                    <TableHead class="w-[72px] flex-none">
                      <Text>Time</Text>
                    </TableHead>
                    <TableHead>
                      <Text>Symbol</Text>
                    </TableHead>
                    <TableHead>
                      <Text>Side</Text>
                    </TableHead>
                    <TableHead class="w-[80px] flex-none items-end">
                      <Text>Quantity</Text>
                    </TableHead>
                  </TableRow>
                </TableHeader>
                <TableBody>
                  {trades.map((t) => (
                    <TableRow>
                      <TableCell class="w-[72px] flex-none">
                        <Text class="tabular-nums">{t.time}</Text>
                      </TableCell>
                      <TableCell>
                        <Text class="font-medium">{t.symbol}</Text>
                      </TableCell>
                      <TableCell>
                        <Text>{t.side}</Text>
                      </TableCell>
                      <TableCell class="w-[80px] flex-none items-end">
                        <Text class="tabular-nums">{t.qty}</Text>
                      </TableCell>
                    </TableRow>
                  ))}
                </TableBody>
              </Table>
            </Scroll>
          </View>
        </Demo>
      </Section>

      <TokensSection prefix="table" component="Table" />
    </Page>
  );
}

function HeroDemo() {
  return (
    <Demo hero class="items-stretch">
      <View class="w-full max-w-[640px] self-center">
        <Table accessibilityLabel="Recent invoices">
          <TableHeader>
            <TableRow>
              <TableHead class={col.invoice}>
                <Text>Invoice</Text>
              </TableHead>
              <TableHead class={col.status}>
                <Text>Status</Text>
              </TableHead>
              <TableHead class={col.method}>
                <Text>Method</Text>
              </TableHead>
              <TableHead class={col.amount}>
                <Text>Amount</Text>
              </TableHead>
            </TableRow>
          </TableHeader>
          <TableBody>
            {invoices.map((i) => (
              <TableRow>
                <TableCell class={col.invoice}>
                  <Text class="font-medium">{i.id}</Text>
                </TableCell>
                <TableCell class={col.status}>
                  <Badge variant={statusVariant(i.status)}>
                    <Text>{i.status}</Text>
                  </Badge>
                </TableCell>
                <TableCell class={col.method}>
                  <Text>{i.method}</Text>
                </TableCell>
                <TableCell class={col.amount}>
                  <Text class="tabular-nums">{i.amount}</Text>
                </TableCell>
              </TableRow>
            ))}
          </TableBody>
          <TableFooter>
            <TableRow>
              <TableCell class="flex-[3.4]">
                <Text>Total</Text>
              </TableCell>
              <TableCell class={col.amount}>
                <Text class="tabular-nums">$1,750.00</Text>
              </TableCell>
            </TableRow>
          </TableFooter>
          <TableCaption>
            <Text>A list of your recent invoices.</Text>
          </TableCaption>
        </Table>
      </View>
    </Demo>
  );
}

function PickDemo() {
  const picked = signal<string[]>(["INV-002"]);
  const all = () => picked().length === invoices.length;
  const toggle = (id: string, on: boolean) => picked.set(on ? [...picked(), id] : picked().filter((x) => x !== id));
  return (
    <Demo class="items-stretch">
      <View class="w-full max-w-[560px] self-center gap-3">
        <Table accessibilityLabel="Invoices to export">
          <TableHeader>
            <TableRow>
              <TableHead class="flex-none">
                <Checkbox
                  accessibilityLabel="Select all"
                  checked={all()}
                  indeterminate={picked().length > 0 && !all()}
                  onChange={(e) => picked.set(e.checked ? invoices.map((i) => i.id) : [])}
                />
              </TableHead>
              <TableHead class={col.invoice}>
                <Text>Invoice</Text>
              </TableHead>
              <TableHead>
                <Text>Method</Text>
              </TableHead>
              <TableHead class={col.amount}>
                <Text>Amount</Text>
              </TableHead>
            </TableRow>
          </TableHeader>
          <TableBody>
            {invoices.map((i) => (
              <TableRow selected={picked().includes(i.id)}>
                <TableCell class="flex-none">
                  <Checkbox
                    accessibilityLabel={`Select ${i.id}`}
                    checked={picked().includes(i.id)}
                    onChange={(e) => toggle(i.id, e.checked)}
                  />
                </TableCell>
                <TableCell class={col.invoice}>
                  <Text class="font-medium">{i.id}</Text>
                </TableCell>
                <TableCell>
                  <Text>{i.method}</Text>
                </TableCell>
                <TableCell class={col.amount}>
                  <Text class="tabular-nums">{i.amount}</Text>
                </TableCell>
              </TableRow>
            ))}
          </TableBody>
        </Table>
        <Text class="text-sm text-muted-foreground">{`${picked().length} of ${invoices.length} row(s) selected.`}</Text>
      </View>
    </Demo>
  );
}

function ActionsDemo() {
  return (
    <Demo class="items-stretch">
      <View class="w-full max-w-[560px] self-center">
        <Table accessibilityLabel="Members">
          <TableHeader>
            <TableRow>
              <TableHead>
                <Text>Name</Text>
              </TableHead>
              <TableHead class="w-[96px] flex-none">
                <Text>Role</Text>
              </TableHead>
              <TableHead class="w-[48px] flex-none items-end">
                <Text> </Text>
              </TableHead>
            </TableRow>
          </TableHeader>
          <TableBody>
            {members.map((m) => (
              <TableRow>
                <TableCell>
                  <View class="gap-0.5">
                    <Text class="font-medium">{m.name}</Text>
                    <Text class="text-xs text-muted-foreground">{m.email}</Text>
                  </View>
                </TableCell>
                <TableCell class="w-[96px] flex-none">
                  <Text>{m.role}</Text>
                </TableCell>
                <TableCell class="w-[48px] flex-none items-end">
                  <DropdownMenu
                    trigger={
                      <Button variant="ghost" size="sm" iconOnly icon="ellipsis" accessibilityLabel={`Actions for ${m.name}`} />
                    }
                  >
                    <DropdownMenuItem icon="settings">
                      <Text>Edit role</Text>
                    </DropdownMenuItem>
                    <DropdownMenuItem icon="activity">
                      <Text>View activity</Text>
                    </DropdownMenuItem>
                    <DropdownMenuSeparator />
                    <DropdownMenuItem icon="trash-2" variant="destructive">
                      <Text>Remove</Text>
                    </DropdownMenuItem>
                  </DropdownMenu>
                </TableCell>
              </TableRow>
            ))}
          </TableBody>
        </Table>
      </View>
    </Demo>
  );
}
