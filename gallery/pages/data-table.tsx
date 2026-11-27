// The DataTable page: payments with boxes and a sort, ten thousand rows,
// fixed columns with stripes and row numbers, the empty and loading states.

import { signal, Text, View } from "nsh";
import {
  Badge,
  Button,
  DataTable,
  type DataTableColumn,
  type DataTableSort,
  Input,
  type Size,
  Spinner,
} from "nsh-longbridge";
import { Demo, Page, Row, Section } from "../ui/layout";
import { TokensSection } from "../ui/tokens";

const sizes: Size[] = ["xs", "sm", "md", "lg"];

type Payment = { id: string; status: string; email: string; amount: number };

const allPayments: Payment[] = [
  { id: "m5gr84i9", status: "Success", email: "ken99@example.com", amount: 316 },
  { id: "3u1reuv4", status: "Success", email: "abe45@example.com", amount: 242 },
  { id: "derv1ws0", status: "Processing", email: "monserrat44@example.com", amount: 837 },
  { id: "5kma53ae", status: "Success", email: "silas22@example.com", amount: 874 },
  { id: "bhqecj4p", status: "Failed", email: "carmella@example.com", amount: 721 },
  { id: "p0r8k2sd", status: "Pending", email: "olivia.martin@example.com", amount: 129 },
  { id: "x7c1v9qa", status: "Success", email: "jackson.lee@example.com", amount: 560 },
];

const paymentColumns: DataTableColumn[] = [
  { key: "status", title: "Status", width: 120 },
  { key: "email", title: "Email", sortable: true, grow: 1, minWidth: 180 },
  { key: "amount", title: "Amount", align: "end", sortable: true, width: 110 },
];

const money = (n: number) => `$${n.toFixed(2)}`;

const statusVariant = (s: string) =>
  s === "Success" ? "secondary" : s === "Failed" ? "destructive" : "outline";

const symbols = ["AAPL", "MSFT", "NVDA", "TSLA", "AMZN", "GOOG", "META", "NFLX", "AMD", "INTC"];

// Ten thousand quotes, made once.
const quotes = Array.from({ length: 10_000 }, (_, i) => {
  const base = 50 + ((i * 37) % 450);
  const change = (((i * 53) % 200) - 100) / 25;
  return {
    symbol: `${symbols[i % symbols.length]}.${Math.floor(i / symbols.length)}`,
    last: base.toFixed(2),
    change: Math.round(change * 100) / 100,
    volume: String(((i * 7919) % 900_000) + 1000),
  };
});

const quoteColumns: DataTableColumn[] = [
  { key: "symbol", title: "Symbol", width: 120, fixed: "left" },
  { key: "last", title: "Last", align: "end", width: 100 },
  { key: "change", title: "Change", align: "end", width: 100, positiveClass: "text-green-600", negativeClass: "text-red-600" },
  { key: "volume", title: "Volume", align: "end", width: 120 },
  { key: "open", title: "Open", align: "end", width: 100 },
  { key: "high", title: "High", align: "end", width: 100 },
  { key: "low", title: "Low", align: "end", width: 100 },
  { key: "exchange", title: "Exchange", width: 120 },
];

const wideRows = quotes.slice(0, 60).map((q, i) => ({
  ...q,
  open: (Number(q.last) - 1.2).toFixed(2),
  high: (Number(q.last) + 2.4).toFixed(2),
  low: (Number(q.last) - 3.1).toFixed(2),
  exchange: i % 2 === 0 ? "NASDAQ" : "NYSE",
}));

export function DataTablePage() {
  return (
    <Page
      eyebrow="Components"
      title="Data Table"
      description="Rows and columns from data: only those in view are made, so any number scrolls smoothly. Sorting, selection, resizing, moving and fixed columns, a keyboard cursor, and a column of boxes."
    >
      <HeroDemo />

      <Section
        title="Ten thousand rows"
        description="Only the rows in view are made and reused while scrolling. The arrows, Home, End and Page Up / Down move the selection once the table has the keyboard."
      >
        <Demo class="items-stretch">
          <View class="w-full max-w-[560px] self-center">
            <DataTable
              class="h-[320px]"
              accessibilityLabel="Quotes"
              columns={quoteColumns.slice(0, 4).map((c) => ({ ...c, fixed: undefined, grow: c.key === "symbol" ? 1 : undefined }))}
              data={quotes}
            />
          </View>
        </Demo>
      </Section>

      <Section
        title="Fixed columns"
        description="The symbol stays at the start while the others scroll; odd rows striped, rows numbered, titles dragged to move or resize their columns."
      >
        <Demo class="items-stretch">
          <View class="w-full max-w-[640px] self-center">
            <DataTable class="h-[300px]" accessibilityLabel="Quotes, wide" columns={quoteColumns} data={wideRows} stripe rowHeader />
          </View>
        </Demo>
      </Section>

      <Section
        title="Sizes"
        description="Row heights and type follow Table's sizes: one height for every row, as scrolling only the rows in view needs."
      >
        <Demo class="items-stretch">
          {sizes.map((size) => (
            <Row label={size}>
              <View class="flex-1">
                <DataTable
                  size={size}
                  class={{ xs: "h-[80px]", sm: "h-[92px]", md: "h-[104px]", lg: "h-[116px]" }[size]}
                  accessibilityLabel={`Payments, ${size}`}
                  columns={paymentColumns}
                  data={allPayments.slice(0, 2).map((p) => ({ ...p, amount: money(p.amount) }))}
                  scrollbar={false}
                />
              </View>
            </Row>
          ))}
        </Demo>
      </Section>

      <Section title="Empty and loading" description="The app's own content where there are no rows, and over the rows while they load.">
        <EmptyDemo />
      </Section>

      <TokensSection prefix="data-table" component="DataTable" />
    </Page>
  );
}

function HeroDemo() {
  const filter = signal("");
  const sort = signal<DataTableSort | null>(null);
  const picked = signal<string[]>([]);
  const rows = () => {
    const f = filter().toLowerCase();
    const list = allPayments.filter((p) => p.email.includes(f));
    const s = sort();
    if (!s) return list;
    const dir = s.direction === "descending" ? -1 : 1;
    return [...list].sort((a, b) =>
      s.key === "amount" ? (a.amount - b.amount) * dir : a.email.localeCompare(b.email) * dir,
    );
  };
  return (
    <Demo hero class="items-stretch">
      <View class="w-full max-w-[640px] self-center gap-4">
        <View class="flex-row items-center gap-2">
          <View class="w-64">
            <Input placeholder="Filter emails…" accessibilityLabel="Filter emails" value={filter()} onChange={(e) => filter.set(e.value)} />
          </View>
          <View class="flex-1" />
          <Button variant="outline" disabled={picked().length === 0} onPress={() => picked.set([])}>
            <Text>Clear selection</Text>
          </Button>
        </View>
        <DataTable
          class="h-[300px]"
          accessibilityLabel="Payments"
          columns={paymentColumns}
          data={rows().map((p) => ({ ...p, amount: money(p.amount) }))}
          sort={sort()}
          onSortChange={({ sort: next }) => sort.set(next)}
          rowSelectable={false}
          checkboxColumn
          checkedRows={rows().map((p) => picked().includes(p.id))}
          onCheckedChange={({ row, checked }) => {
            const id = rows()[row].id;
            picked.set(checked ? [...picked(), id] : picked().filter((x) => x !== id));
          }}
          onCheckedAllChange={({ checked }) => picked.set(checked ? rows().map((p) => p.id) : [])}
          renderCell={(cell) =>
            cell.key === "status" ? (
              <View class="flex-row">
                <Badge variant={statusVariant(cell.text)}>
                  <Text>{cell.text}</Text>
                </Badge>
              </View>
            ) : (
              <Text class={`truncate ${cell.key === "amount" ? "text-right tabular-nums" : ""}`}>{cell.text}</Text>
            )
          }
          empty={<Text>No results.</Text>}
        />
        <Text class="text-sm text-muted-foreground">{`${picked().length} of ${rows().length} row(s) selected.`}</Text>
      </View>
    </Demo>
  );
}

function EmptyDemo() {
  const loading = signal(true);
  return (
    <Demo class="items-stretch">
      <View class="w-full max-w-[640px] self-center flex-row flex-wrap gap-6">
        <View class="min-w-[260px] flex-1">
          <DataTable
            class="h-[180px]"
            accessibilityLabel="Payments, none"
            columns={paymentColumns}
            data={[]}
            empty={<Text>No results.</Text>}
          />
        </View>
        <View class="min-w-[260px] flex-1 gap-3">
          <DataTable
            class="h-[180px]"
            accessibilityLabel="Payments, loading"
            columns={paymentColumns}
            data={loading() ? [] : allPayments.slice(0, 3).map((p) => ({ ...p, amount: money(p.amount) }))}
            loading={loading()}
            loadingView={
              <View class="flex-1 items-center justify-center">
                <Spinner />
              </View>
            }
            empty={<Text>No results.</Text>}
          />
          <Button variant="outline" size="sm" class="self-start" onPress={() => loading.set(!loading())}>
            <Text>{loading() ? "Finish loading" : "Load again"}</Text>
          </Button>
        </View>
      </View>
    </Demo>
  );
}
