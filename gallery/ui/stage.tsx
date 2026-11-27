// The stage: a block in the preset's primary color, and on it a product
// window made of the library's components (an invoicing app), with the
// preset swatches that recolor every demo on the site. The theme studio
// shows it whole; the home page's scene builds on its window and colors.
// Only the window's frame (its traffic lights) and the layout around the
// components are the gallery's.

import { signal, Text, View, type JSX } from "nsh";
import {
  Avatar,
  Badge,
  type BadgeVariant,
  Button,
  Input,
  Progress,
  Table,
  TableBody,
  TableCell,
  TableHead,
  TableHeader,
  TableRow,
  Tabs,
  TabsList,
  TabsTrigger,
} from "nsh-longbridge";
import { ChartColumn, Check, FileText, LayoutDashboard, Package, Settings, Users } from "nsh-icon-lucide";
import { scheme } from "../theme";
import { presetLabel, PresetSwatches } from "./theme-picker";
import { Glyph } from "./icons";
import { siteWidth, stageColors } from "./site";

const invoices: { id: string; customer: string; status: string; variant: BadgeVariant; due: string; amount: string }[] = [
  { id: "INV-2041", customer: "Lumen Labs", status: "Paid", variant: "default", due: "Nov 28", amount: "$4,800.00" },
  { id: "INV-2040", customer: "Harbor & Pine", status: "Open", variant: "outline", due: "Dec 2", amount: "$1,250.00" },
  { id: "INV-2039", customer: "Orbit Freight", status: "Overdue", variant: "destructive", due: "Nov 18", amount: "$3,120.00" },
  { id: "INV-2038", customer: "Fieldnote", status: "Paid", variant: "default", due: "Nov 15", amount: "$860.00" },
  { id: "INV-2037", customer: "Kestrel Health", status: "Draft", variant: "secondary", due: "—", amount: "$12,400.00" },
  { id: "INV-2036", customer: "Northwind Rail", status: "Paid", variant: "default", due: "Nov 11", amount: "$2,075.40" },
];

const nav = [
  { label: "Overview", icon: LayoutDashboard },
  { label: "Invoices", icon: FileText },
  { label: "Customers", icon: Users },
  { label: "Products", icon: Package },
  { label: "Reports", icon: ChartColumn },
];

const paid = [38, 44, 41, 52, 49, 58, 63, 61, 70, 76, 74, 88];

/** The red, yellow and green of a macOS window, which read the same in both appearances. */
export function TrafficLights() {
  return (
    <View class="flex-row gap-2">
      {["#ff5f57", "#febc2e", "#28c840"].map((c) => (
        <View class="size-3 rounded-full" style={{ backgroundColor: c, boxShadow: "inset 0 0 0 0.5px #00000026" }} />
      ))}
    </View>
  );
}

function Sidebar() {
  return (
    <View class="w-56 border-r border-sidebar-border bg-sidebar">
      <View class="h-12 justify-center px-4">
        <TrafficLights />
      </View>
      <View class="flex-row items-center gap-2.5 px-4 pb-4 pt-1">
        <Avatar name="Northwind Studio" size="sm" />
        <View class="flex-1 gap-0.5">
          <Text class="text-sm font-medium text-sidebar-foreground">Northwind</Text>
          <Text class="text-xs text-muted-foreground">Studio plan</Text>
        </View>
      </View>
      <View class="gap-0.5 px-2">
        {nav.map((n) => (
          <Button
            size="sm"
            variant={n.label === "Invoices" ? "secondary" : "ghost"}
            class="w-full justify-start px-2"
            labelClass={n.label === "Invoices" ? "font-medium" : "font-normal"}
          >
            <Glyph icon={n.icon} size={15} class={n.label === "Invoices" ? "text-foreground" : "text-muted-foreground"} />
            <Text>{n.label}</Text>
          </Button>
        ))}
      </View>
      <View class="flex-1" />
      <View class="gap-0.5 px-2 pb-2">
        <Button size="sm" variant="ghost" class="w-full justify-start px-2" labelClass="font-normal">
          <Glyph icon={Settings} size={15} class="text-muted-foreground" />
          <Text>Settings</Text>
        </Button>
      </View>
      <View class="mx-3 mb-3 gap-2.5 rounded-lg border border-sidebar-border bg-background p-3">
        <View class="flex-row items-baseline justify-between">
          <Text class="text-xs font-medium text-foreground">Invoices this month</Text>
        </View>
        <Progress value={64} accessibilityLabel="Invoices this month" />
        <Text class="text-xs text-muted-foreground tabular-nums">128 of 200 on your plan</Text>
      </View>
    </View>
  );
}

function Stat(props: { label: string; value: string; note: string; chart?: boolean; badge?: string }) {
  const max = Math.max(...paid);
  return (
    <View class="flex-1 min-w-0 flex-row items-end gap-3 rounded-lg border border-border bg-card p-4">
      <View class="flex-1 min-w-0 gap-1.5 self-start">
        <Text class="text-xs font-medium text-muted-foreground">{props.label}</Text>
        <Text class="text-xl font-semibold tracking-tight text-card-foreground tabular-nums">{props.value}</Text>
        <View class="h-5 flex-row items-center gap-1.5">
          <If when={props.badge}>
            <Badge variant="outline">
              <Text>{props.badge!}</Text>
            </Badge>
          </If>
          <Text class="flex-1 min-w-0 truncate text-xs text-muted-foreground">{props.note}</Text>
        </View>
      </View>
      <If when={props.chart}>
        <View class="h-12 w-20 flex-none flex-row items-end gap-[3px]" accessibilityLabel="Paid by week, rising">
          {paid.map((v, i) => (
            <View
              class={`flex-1 rounded-[2px] ${i === paid.length - 1 ? "bg-primary" : "bg-chart-2"}`}
              style={{ height: `${(v / max) * 100}%`, opacity: i === paid.length - 1 ? 1 : 0.3 + (i / paid.length) * 0.45 }}
            />
          ))}
        </View>
      </If>
    </View>
  );
}

function Customer(props: { name: string; compact?: boolean }) {
  return (
    <View class="min-w-0 flex-row items-center gap-2">
      <If when={!props.compact}>
        <Avatar name={props.name} size="xs" />
      </If>
      <Text class="truncate text-sm text-foreground">{props.name}</Text>
    </View>
  );
}

function InvoiceTable(props: { rows: number; compact?: boolean }) {
  const amount = "w-28 flex-none items-end";
  return (
    <Table size="sm" accessibilityLabel="Invoices">
      <TableHeader>
        <TableRow>
          <If when={!props.compact}>
            <TableHead class="w-24 flex-none">
              <Text>Invoice</Text>
            </TableHead>
          </If>
          <TableHead>
            <Text>Customer</Text>
          </TableHead>
          <TableHead class={props.compact ? "w-[84px] flex-none" : "w-24 flex-none"}>
            <Text>Status</Text>
          </TableHead>
          <If when={!props.compact}>
            <TableHead class="w-20 flex-none">
              <Text>Due</Text>
            </TableHead>
          </If>
          <TableHead class={props.compact ? "w-24 flex-none items-end" : amount}>
            <Text>Amount</Text>
          </TableHead>
        </TableRow>
      </TableHeader>
      <TableBody>
        {invoices.slice(0, props.rows).map((r, i) => (
          <TableRow>
            <If when={!props.compact}>
              <TableCell class="w-24 flex-none" labelClass="text-muted-foreground tabular-nums">
                <Text>{r.id}</Text>
              </TableCell>
            </If>
            <TableCell>
              <Customer name={r.customer} compact={props.compact} />
            </TableCell>
            <TableCell class={props.compact ? "w-[84px] flex-none" : "w-24 flex-none"}>
              <Badge variant={r.variant}>
                <Text>{r.status}</Text>
              </Badge>
            </TableCell>
            <If when={!props.compact}>
              <TableCell class="w-20 flex-none" labelClass="text-muted-foreground tabular-nums">
                <Text>{r.due}</Text>
              </TableCell>
            </If>
            <TableCell class={props.compact ? "w-24 flex-none items-end" : amount} labelClass="font-medium tabular-nums">
              <Text>{r.amount}</Text>
            </TableCell>
          </TableRow>
        ))}
      </TableBody>
    </Table>
  );
}

function StatusTabs(props: { size?: "sm" | "md" }) {
  const tab = signal("all");
  return (
    <Tabs value={tab()} onChange={(e) => tab.set(e.value)}>
      <TabsList size={props.size ?? "sm"} accessibilityLabel="Invoice status">
        {["All", "Open", "Paid", "Overdue"].map((t) => (
          <TabsTrigger value={t.toLowerCase()}>
            <Text>{t}</Text>
          </TabsTrigger>
        ))}
      </TabsList>
    </Tabs>
  );
}

/**
 * The product window at a desktop's width: a sidebar, a toolbar, figures and
 * a table. `exportMenu`: a menu shown open under Export, over the content.
 */
export function DesktopApp(props: { width: number; exportMenu?: JSX.Element }) {
  const stats = () => (props.width >= 960 ? 3 : 2);
  return (
    <View class="flex-row">
      <If when={props.width >= 860}>
        <Sidebar />
      </If>
      <View class="flex-1 min-w-0 bg-background">
        <View class="h-12 flex-row items-center gap-2 border-b border-border px-5" style={{ zIndex: 2 }}>
          <If when={props.width < 860}>
            <View class="mr-2">
              <TrafficLights />
            </View>
          </If>
          <Text class="flex-1 text-sm font-semibold text-foreground">Invoices</Text>
          <View class="w-52">
            <Input size="sm" placeholder="Search invoices" accessibilityLabel="Search invoices" />
          </View>
          <View>
            <Button size="sm" variant="outline">
              <Text>Export</Text>
            </Button>
            <If when={props.exportMenu}>
              <View class="absolute" style={{ top: 38, right: 0 }}>
                {props.exportMenu!}
              </View>
            </If>
          </View>
          <Button size="sm" icon="plus">
            <Text>New invoice</Text>
          </Button>
        </View>
        <View class="flex-1 gap-5 p-5">
          <View class="flex-row gap-3">
            <Stat label="Outstanding" value="$24,580.00" note="12 open invoices" />
            <Stat label="Paid in November" value="$61,204.50" badge="+18.2%" note="on Oct" chart />
            <If when={stats() === 3}>
              <Stat label="Overdue" value="$3,120.00" note="1 invoice, 9 days late" />
            </If>
          </View>
          <View class="gap-3">
            <View class="flex-row items-center justify-between">
              <StatusTabs />
              <Text class="text-xs text-muted-foreground tabular-nums">128 invoices</Text>
            </View>
            <InvoiceTable rows={6} />
          </View>
        </View>
      </View>
    </View>
  );
}

/** The product window at a phone's width: its title, one figure and a shorter table. */
export function PhoneApp() {
  return (
    <View class="bg-background">
      <View class="h-10 flex-row items-center px-3.5">
        <TrafficLights />
      </View>
      <View class="gap-4 px-4 pb-4">
        <View class="flex-row items-center justify-between">
          <Text class="text-base font-semibold text-foreground">Invoices</Text>
          <Button size="sm" icon="plus">
            <Text>New</Text>
          </Button>
        </View>
        <Stat label="Paid in November" value="$61,204.50" badge="+18.2%" note="on Oct" chart />
        <StatusTabs />
        <InvoiceTable rows={4} compact />
      </View>
    </View>
  );
}

/** A popup's shadow over the window: a contact shadow, then two soft ones. */
export const floatShadow =
  "0 1px 2px #0000000d, 0 4px 8px -2px #00000014, 0 16px 32px -8px #00000029, 0 32px 64px -16px #00000033";

/** A note over the window's corner, as a product shows one after an action. */
export function PaidNote(props: { width?: number } = {}) {
  return (
    <View
      style={{ width: props.width ?? 300, boxShadow: floatShadow }}
      class="flex-row items-center gap-3 rounded-xl border border-border bg-popover p-3"
    >
      <View class="size-8 items-center justify-center rounded-full bg-primary">
        <Glyph icon={Check} size={15} class="text-primary-foreground" />
      </View>
      <View class="flex-1 min-w-0 gap-0.5">
        <Text class="text-sm font-medium text-popover-foreground">Payment received</Text>
        <Text class="truncate text-xs text-muted-foreground">Lumen Labs paid $4,800.00</Text>
      </View>
      <Button size="xs" variant="outline">
        <Text>View</Text>
      </Button>
    </View>
  );
}

/** The color presets on the stage's glass: the chosen one's name, then the swatches. */
export function StagePresets() {
  return (
    <View
      class="flex-row items-center gap-3 rounded-full py-1 pl-4 pr-1.5"
      style={{ backgroundColor: "#ffffff14", boxShadow: "inset 0 0 0 1px #ffffff1f", backdropFilter: "blur(12px)" }}
    >
      <Text class="text-xs font-medium" style={{ color: "#ffffff", minWidth: 48 }}>
        {presetLabel()}
      </Text>
      <PresetSwatches tone="stage" size={20} gap={8} />
    </View>
  );
}

/** Layered: a hairline edge (light on dark), a contact shadow, then two soft ones. */
export const windowShadow = () =>
  `0 0 0 1px ${scheme() === "dark" ? "#ffffff1a" : "#0000000f"}, 0 1px 1px #0000000a, 0 4px 8px -2px #00000014, 0 18px 36px -10px #00000033, 0 48px 96px -24px #00000059`;

/** The stage's ground: the preset's color, a glow from the top, a darker floor and a faint grid. */
export function stageFill() {
  const c = stageColors();
  return {
    backgroundColor: c.base,
    backgroundImage: [
      `radial-gradient(90% 70% at 50% 0%, ${c.glow} 0%, transparent 70%)`,
      `linear-gradient(180deg, transparent 55%, ${c.floor} 100%)`,
      `repeating-linear-gradient(90deg, ${c.line} 0px, ${c.line} 1px, transparent 1px, transparent 40px)`,
      `repeating-linear-gradient(0deg, ${c.line} 0px, ${c.line} 1px, transparent 1px, transparent 40px)`,
    ].join(", "),
  };
}

/**
 * The stage. `width`: the stage's own (the home page's column by default);
 * `presets`: the preset swatches over the window (default on); `below`: what
 * stands on the stage under the window (the studio's cards).
 */
export function Stage(props: { width?: number; presets?: boolean; below?: JSX.Element } = {}) {
  const width = () => props.width ?? siteWidth();
  const phone = () => width() < 640;
  const pad = () => (phone() ? 14 : width() < 1000 ? 40 : 72);
  const appWidth = () => Math.min(1040, width() - 2 * pad());
  const presets = () => props.presets !== false;
  return (
    <View
      class="overflow-hidden items-center"
      style={{
        borderRadius: phone() ? 20 : 28,
        ...stageFill(),
        paddingTop: presets() ? (phone() ? 16 : 28) : phone() ? 20 : 56,
        paddingBottom: phone() ? 40 : 88,
        paddingHorizontal: pad(),
      }}
    >
      <If when={presets()}>
        <View class="w-full flex-row items-center justify-between" style={{ marginBottom: phone() ? 16 : 44 }}>
          <If when={!phone()}>
            <Text class="text-xs font-medium" style={{ color: "#ffffffb3" }}>
              Live: every control in this window is NSH Longbridge
            </Text>
          </If>
          <View class={phone() ? "flex-1 items-center" : undefined}>
            <StagePresets />
          </View>
        </View>
      </If>
      <View style={{ width: appWidth() }}>
        <View class="overflow-hidden rounded-xl bg-background" style={{ boxShadow: windowShadow() }}>
          {phone() ? <PhoneApp /> : <DesktopApp width={appWidth()} />}
        </View>
        <If when={appWidth() >= 880 && pad() >= 56}>
          <View class="absolute" style={{ right: -36, bottom: -40 }}>
            <PaidNote />
          </View>
        </If>
      </View>
      <If when={props.below}>
        <View style={{ width: appWidth(), marginTop: phone() ? 20 : 56 }}>{props.below!}</View>
      </If>
    </View>
  );
}
