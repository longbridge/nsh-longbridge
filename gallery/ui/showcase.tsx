// The showcase: screens of a product made only of the library's components,
// which the home page and the theme editor show. Every color, radius and
// height comes from the theme, so all of it follows the theme controls.

import { keymap, signal, Text, View, type JSX } from "nsh";
import {
  Avatar,
  Badge,
  type BadgeVariant,
  Button,
  Card,
  CardAction,
  CardContent,
  CardDescription,
  CardFooter,
  CardHeader,
  CardTitle,
  Checkbox,
  Combobox,
  DatePicker,
  type DatePickerValue,
  Field,
  FieldContent,
  FieldDescription,
  FieldGroup,
  FieldLabel,
  Input,
  type InputRef,
  Kbd,
  KbdGroup,
  Label,
  Pagination,
  Progress,
  RadioGroup,
  RadioGroupItem,
  Separator,
  Skeleton,
  type SelectItem,
  Slider,
  Spinner,
  Switch,
  Table,
  TableBody,
  TableCell,
  TableHead,
  TableHeader,
  TableRow,
  Textarea,
  ToggleGroup,
  ToggleGroupItem,
  Tooltip,
} from "nsh-longbridge";
import { numberOf } from "../theme";

const notifications = [
  { id: "mentions", title: "Mentions", description: "When someone mentions you in a comment." },
  { id: "approvals", title: "Approvals", description: "When a deployment waits for your review." },
  { id: "digest", title: "Weekly digest", description: "A summary of the week, every Monday." },
];

const plans = [
  { id: "hobby", title: "Hobby", price: "Free", note: "For personal projects" },
  { id: "pro", title: "Pro", price: "$20 / month", note: "For small teams", badge: "Popular" },
  { id: "team", title: "Enterprise", price: "Custom", note: "SSO, audit logs, support" },
];

const members: { name: string; email: string; role: string; variant: BadgeVariant }[] = [
  { name: "Sofia Davis", email: "sofia@example.com", role: "Owner", variant: "default" },
  { name: "Jackson Lee", email: "jackson@example.com", role: "Member", variant: "secondary" },
  { name: "Isabella Nguyen", email: "isabella@example.com", role: "Invited", variant: "outline" },
];

const shortcuts = [
  { action: "Search", keys: "mod+k" },
  { action: "New file", keys: "mod+n" },
  { action: "Toggle sidebar", keys: "mod+b" },
  { action: "Command palette", keys: "mod+shift+p" },
  { action: "Save", keys: "mod+s" },
];

const revenue = [42, 58, 49, 66, 72, 61, 80, 74, 91, 86, 98, 104];

const zones: SelectItem[] = [
  { value: "london", title: "London (GMT)" },
  { value: "new-york", title: "New York (EST)" },
  { value: "san-francisco", title: "San Francisco (PST)" },
  { value: "singapore", title: "Singapore (SGT)" },
  { value: "tokyo", title: "Tokyo (JST)" },
];

const invoices: { id: string; status: string; variant: BadgeVariant; amount: string }[][] = [
  [
    { id: "INV-1042", status: "Paid", variant: "secondary", amount: "$1,250.00" },
    { id: "INV-1041", status: "Pending", variant: "outline", amount: "$480.00" },
    { id: "INV-1040", status: "Overdue", variant: "destructive", amount: "$2,100.00" },
    { id: "INV-1039", status: "Paid", variant: "secondary", amount: "$320.00" },
  ],
  [
    { id: "INV-1038", status: "Paid", variant: "secondary", amount: "$990.00" },
    { id: "INV-1037", status: "Paid", variant: "secondary", amount: "$145.00" },
    { id: "INV-1036", status: "Pending", variant: "outline", amount: "$760.00" },
    { id: "INV-1035", status: "Paid", variant: "secondary", amount: "$1,810.00" },
  ],
  [
    { id: "INV-1034", status: "Paid", variant: "secondary", amount: "$275.00" },
    { id: "INV-1033", status: "Paid", variant: "secondary", amount: "$3,400.00" },
    { id: "INV-1032", status: "Paid", variant: "secondary", amount: "$610.00" },
    { id: "INV-1031", status: "Paid", variant: "secondary", amount: "$88.00" },
  ],
];

// Text here takes the card's color, never a Tailwind color class: those read
// the app's theme, while a ThemeScope reaches only the library's components.
// Muted text is a CardDescription, so it follows a scope too.

/** Quieter text, the description's color; `xs` for a smaller size. */
function Muted(props: { children: string; xs?: boolean }) {
  return (
    <CardDescription>
      <Text class={props.xs ? "text-xs" : undefined}>{props.children}</Text>
    </CardDescription>
  );
}

/** How far a small button's box reaches past a card title's line, on each side. */
const titleOverhang = () => Math.max(0, (numberOf("button-height-sm") - numberOf("card-title-font-size")) / 2);

export function SignInCard(props: { class?: string } = {}) {
  const email = signal<InputRef>();
  const password = signal<InputRef>();
  const remember = signal(true);
  return (
    <Card class={props.class}>
      <CardHeader>
        <CardTitle>
          <Text>Sign in</Text>
        </CardTitle>
        <CardDescription>
          <Text>Enter your email below to sign in to your account.</Text>
        </CardDescription>
        <CardAction>
          {/* The link's box is a control's height and the title's line is
              its font size: centered on the title's line, not below it. */}
          <Button variant="link" size="sm" class="px-0" style={{ marginTop: -titleOverhang(), marginBottom: -titleOverhang() }}>
            <Text>Sign up</Text>
          </Button>
        </CardAction>
      </CardHeader>
      <CardContent class="gap-4">
        <View class="gap-2">
          <Label htmlFor={email()}>
            <Text>Email</Text>
          </Label>
          <Input ref={email} placeholder="m@example.com" accessibilityLabel="Email" autoComplete="email" />
        </View>
        <View class="gap-2">
          <View class="flex-row items-center justify-between">
            <Label htmlFor={password()}>
              <Text>Password</Text>
            </Label>
            <Button variant="link" size="xs" class="px-0">
              <Text>Forgot your password?</Text>
            </Button>
          </View>
          <Input ref={password} masked defaultValue="hunter22" accessibilityLabel="Password" />
        </View>
        <Checkbox checked={remember()} onChange={(e) => remember.set(e.checked)}>
          <Text>Remember me for 30 days</Text>
        </Checkbox>
      </CardContent>
      <CardFooter class="flex-col items-stretch gap-2">
        <Button>
          <Text>Sign in</Text>
        </Button>
        <Button variant="outline">
          <Text>Sign in with SSO</Text>
        </Button>
      </CardFooter>
    </Card>
  );
}

export function NotificationsCard(props: { class?: string } = {}) {
  const on = signal<string[]>(["mentions", "approvals"]);
  return (
    <Card class={props.class}>
      <CardHeader>
        <CardTitle>
          <Text>Notifications</Text>
        </CardTitle>
        <CardDescription>
          <Text>Choose what you hear about.</Text>
        </CardDescription>
      </CardHeader>
      <CardContent>
        {notifications.map((n, i) => (
          <View>
            <If when={i > 0}>
              <Separator class="my-4" />
            </If>
            <View class="flex-row items-center gap-4">
              <View class="flex-1 gap-1">
                <Text class="text-sm font-medium">{n.title}</Text>
                <Muted>{n.description}</Muted>
              </View>
              <Switch
                checked={on().includes(n.id)}
                accessibilityLabel={n.title}
                onChange={(e) => on.set(e.checked ? [...on(), n.id] : on().filter((x) => x !== n.id))}
              />
            </View>
          </View>
        ))}
      </CardContent>
    </Card>
  );
}

export function PlanCard(props: { class?: string } = {}) {
  const plan = signal("pro");
  return (
    <Card class={props.class}>
      <CardHeader>
        <CardTitle>
          <Text>Choose a plan</Text>
        </CardTitle>
        <CardDescription>
          <Text>Change it at any time from your settings.</Text>
        </CardDescription>
      </CardHeader>
      <CardContent>
        <RadioGroup accessibilityLabel="Plan" value={plan()} onChange={(e) => plan.set(e.value)} class="gap-4">
          {plans.map((p) => (
            <RadioGroupItem value={p.id}>
              <View class="gap-0.5">
                <View class="flex-row items-center gap-2">
                  <Text class="text-sm font-medium">{p.title}</Text>
                  <If when={p.badge}>
                    <Badge>
                      <Text>{p.badge!}</Text>
                    </Badge>
                  </If>
                </View>
                <Muted xs>{`${p.price} · ${p.note}`}</Muted>
              </View>
            </RadioGroupItem>
          ))}
        </RadioGroup>
      </CardContent>
      <CardFooter>
        <Button class="flex-1" iconEnd="arrow-right">
          <Text>Continue</Text>
        </Button>
      </CardFooter>
    </Card>
  );
}

export function InviteCard(props: { class?: string } = {}) {
  return (
    <Card class={props.class}>
      <CardHeader>
        <CardTitle>
          <Text>Invite your team</Text>
        </CardTitle>
        <CardDescription>
          <Text>Anyone with an invitation can view and deploy.</Text>
        </CardDescription>
      </CardHeader>
      <CardContent class="gap-5">
        <View class="flex-row gap-2">
          <View class="flex-1">
            <Input placeholder="name@example.com" accessibilityLabel="Email to invite" keyboardType="email-address" />
          </View>
          <Button variant="secondary">
            <Text>Invite</Text>
          </Button>
        </View>
        <Separator />
        <View class="gap-4">
          <Text class="text-sm font-medium">People with access</Text>
          {members.map((m) => (
            <View class="flex-row items-center gap-3">
              <Avatar name={m.name} />
              <View class="flex-1 gap-0.5">
                <Text class="truncate text-sm font-medium">{m.name}</Text>
                <Muted xs>{m.email}</Muted>
              </View>
              <Badge variant={m.variant}>
                <Text>{m.role}</Text>
              </Badge>
              <Button
                variant="ghost"
                size="sm"
                iconOnly
                icon="trash-2"
                accessibilityLabel={`Remove ${m.name}`}
                disabled={m.role === "Owner"}
                tooltip={
                  <Tooltip>
                    <Text>{m.role === "Owner" ? "The owner stays" : `Remove ${m.name.split(" ")[0]}`}</Text>
                  </Tooltip>
                }
              />
            </View>
          ))}
        </View>
      </CardContent>
    </Card>
  );
}

export function ActivityCard(props: { class?: string } = {}) {
  return (
    <Card class={props.class}>
      <CardHeader>
        <CardTitle>
          <Text>Activity</Text>
        </CardTitle>
        <CardDescription>
          <Text>Deployments across your projects.</Text>
        </CardDescription>
        <CardAction>
          <View class="flex-row items-center gap-2">
            <Spinner size="sm" />
            <Muted xs>Syncing</Muted>
          </View>
        </CardAction>
      </CardHeader>
      <CardContent class="gap-5">
        <View class="flex-row items-center gap-3">
          <Avatar name="Olivia Martin" />
          <View class="flex-1 gap-0.5">
            <Text class="text-sm">Olivia deployed acme-web</Text>
            <Muted xs>2 minutes ago · main</Muted>
          </View>
          <Badge variant="secondary">
            <Text>Ready</Text>
          </Badge>
        </View>
        <View class="flex-row items-center gap-3">
          <Avatar name="Liam Johnson" />
          <View class="flex-1">
            <Progress value={64} accessibilityLabel="acme-docs deployment">
              <View class="flex-row justify-between gap-2">
                <Text class="truncate text-sm">Liam is deploying acme-docs</Text>
                <Muted xs>64%</Muted>
              </View>
            </Progress>
          </View>
        </View>
        <View class="gap-5" accessibilityLabel="Loading activity">
          {[0.72, 0.56, 0.64].map((w) => (
            <View class="flex-row items-center gap-3">
              <Skeleton class="size-8 rounded-full" />
              <View class="flex-1 gap-2">
                <Skeleton class="h-3.5" style={{ width: `${w * 100}%` }} />
                <Skeleton class="h-3 w-1/3" />
              </View>
            </View>
          ))}
        </View>
      </CardContent>
    </Card>
  );
}

export function ShortcutsCard(props: { class?: string } = {}) {
  return (
    <Card class={props.class}>
      <CardHeader>
        <CardTitle>
          <Text>Keyboard shortcuts</Text>
        </CardTitle>
        <CardDescription>
          <Text>Work without leaving the keyboard.</Text>
        </CardDescription>
      </CardHeader>
      <CardContent class="gap-3">
        {shortcuts.map((s) => (
          <View class="flex-row items-center justify-between">
            <Text class="text-sm">{s.action}</Text>
            <KbdGroup>
              {keymap
                .label(s.keys)
                .split(/(?<=[⌘⇧⌥⌃])|\+/)
                .filter((k) => k)
                .map((k) => (
                  <Kbd>
                    <Text>{k}</Text>
                  </Kbd>
                ))}
            </KbdGroup>
          </View>
        ))}
      </CardContent>
    </Card>
  );
}

export function FeedbackCard(props: { class?: string } = {}) {
  const sent = signal(false);
  const text = signal("");
  return (
    <Card class={props.class}>
      <CardHeader>
        <CardTitle>
          <Text>Send feedback</Text>
        </CardTitle>
        <CardDescription>
          <Text>What would make your work easier?</Text>
        </CardDescription>
      </CardHeader>
      <CardContent>
        <Textarea
          placeholder="Tell us what you think…"
          accessibilityLabel="Feedback"
          value={text()}
          onChange={(e) => text.set(e.value)}
        />
      </CardContent>
      <CardFooter class="justify-between gap-2">
        <Muted xs>{sent() ? "Thank you!" : `${text().length} / 500`}</Muted>
        <View class="flex-row gap-2">
          <Button variant="ghost" size="sm" onPress={() => text.set("")}>
            <Text>Clear</Text>
          </Button>
          <Button size="sm" disabled={text().trim() === ""} onPress={() => sent.set(true)}>
            <Text>Send</Text>
          </Button>
        </View>
      </CardFooter>
    </Card>
  );
}

export function RevenueCard(props: { class?: string } = {}) {
  const max = Math.max(...revenue);
  return (
    <Card class={props.class}>
      <CardHeader>
        <CardDescription>
          <Text>Total revenue</Text>
        </CardDescription>
        <CardTitle class="text-2xl tabular-nums">
          <Text>$15,231.89</Text>
        </CardTitle>
        <CardAction>
          <Badge variant="outline">
            <Text>+20.1%</Text>
          </Badge>
        </CardAction>
      </CardHeader>
      <CardContent>
        <View class="h-24 flex-row items-end gap-1.5" accessibilityLabel="Revenue by month, rising">
          {revenue.map((v, i) => (
            <View
              class={`flex-1 rounded-sm ${i === revenue.length - 1 ? "bg-primary" : "bg-chart-2"}`}
              style={{ height: `${(v / max) * 100}%`, opacity: i === revenue.length - 1 ? 1 : 0.35 + (i / revenue.length) * 0.5 }}
            />
          ))}
        </View>
      </CardContent>
      <CardFooter>
        <Muted xs>Compared with the same months last year.</Muted>
      </CardFooter>
    </Card>
  );
}

const exports = [
  { id: "invoices", label: "Invoices", note: "1,204 rows" },
  { id: "customers", label: "Customers", note: "318 rows" },
  { id: "activity", label: "Activity log", note: "Last 90 days" },
];

export function ExportCard(props: { class?: string } = {}) {
  const picked = signal<string[]>(["invoices", "customers"]);
  const format = signal("csv");
  return (
    <Card class={props.class}>
      <CardHeader>
        <CardTitle>
          <Text>Export data</Text>
        </CardTitle>
        <CardDescription>
          <Text>A file with what you choose, sent by email.</Text>
        </CardDescription>
      </CardHeader>
      <CardContent class="gap-5">
        <View class="gap-3">
          {exports.map((e) => (
            <View class="flex-row items-center justify-between gap-3">
              <Checkbox
                checked={picked().includes(e.id)}
                onChange={(ev) => picked.set(ev.checked ? [...picked(), e.id] : picked().filter((x) => x !== e.id))}
              >
                <Text>{e.label}</Text>
              </Checkbox>
              <Muted xs>{e.note}</Muted>
            </View>
          ))}
        </View>
        <Separator />
        <RadioGroup
          accessibilityLabel="Format"
          value={format()}
          onChange={(e) => format.set(e.value)}
          class="flex-row gap-5"
          size="sm"
        >
          <RadioGroupItem value="csv">
            <Text>CSV</Text>
          </RadioGroupItem>
          <RadioGroupItem value="xlsx">
            <Text>Excel</Text>
          </RadioGroupItem>
          <RadioGroupItem value="json">
            <Text>JSON</Text>
          </RadioGroupItem>
        </RadioGroup>
      </CardContent>
      <CardFooter>
        <Button variant="outline" class="flex-1" disabled={picked().length === 0}>
          <Text>{picked().length === 0 ? "Choose something to export" : `Export ${picked().length} ${picked().length === 1 ? "table" : "tables"}`}</Text>
        </Button>
      </CardFooter>
    </Card>
  );
}

export function MeetingCard(props: { class?: string } = {}) {
  const day = signal<DatePickerValue>("2026-12-03");
  const zone = signal<string[]>(["london"]);
  return (
    <Card class={props.class}>
      <CardHeader>
        <CardTitle>
          <Text>Schedule a meeting</Text>
        </CardTitle>
        <CardDescription>
          <Text>Everyone invited gets it in their own time zone.</Text>
        </CardDescription>
      </CardHeader>
      <CardContent>
        <FieldGroup>
          <Field>
            <FieldLabel>
              <Text>Title</Text>
            </FieldLabel>
            <Input defaultValue="Quarterly planning" accessibilityLabel="Title" />
          </Field>
          <Field>
            <FieldLabel>
              <Text>Date</Text>
            </FieldLabel>
            <DatePicker class="w-full" value={day()} onChange={(e) => day.set(e.value)} accessibilityLabel="Date" />
          </Field>
          <Field>
            <FieldLabel>
              <Text>Time zone</Text>
            </FieldLabel>
            <Combobox items={zones} value={zone()} onChange={(e) => zone.set(e.value)} accessibilityLabel="Time zone" />
          </Field>
        </FieldGroup>
      </CardContent>
      <CardFooter class="justify-end gap-2">
        <Button variant="ghost" size="sm">
          <Text>Cancel</Text>
        </Button>
        <Button size="sm" disabled={day() === null}>
          <Text>Schedule</Text>
        </Button>
      </CardFooter>
    </Card>
  );
}

export function DisplayCard(props: { class?: string } = {}) {
  const size = signal(16);
  const density = signal("default");
  return (
    <Card class={props.class}>
      <CardHeader>
        <CardTitle>
          <Text>Display</Text>
        </CardTitle>
        <CardDescription>
          <Text>How the editor looks on this device.</Text>
        </CardDescription>
      </CardHeader>
      <CardContent>
        <FieldGroup>
          <Field orientation="horizontal">
            <FieldContent>
              <FieldLabel>
                <Text>Text size</Text>
              </FieldLabel>
              <FieldDescription>
                <Text>{`${size()} points`}</Text>
              </FieldDescription>
            </FieldContent>
            <Slider min={12} max={24} value={size()} onChange={(e) => size.set(e.value as number)} accessibilityLabel="Text size" />
          </Field>
          <Field>
            <FieldLabel>
              <Text>Density</Text>
            </FieldLabel>
            <ToggleGroup
              size="sm"
              variant="outline"
              value={[density()]}
              onChange={(e) => e.value[0] && density.set(e.value[0])}
              accessibilityLabel="Density"
            >
              <ToggleGroupItem value="compact">
                <Text>Compact</Text>
              </ToggleGroupItem>
              <ToggleGroupItem value="default">
                <Text>Default</Text>
              </ToggleGroupItem>
              <ToggleGroupItem value="comfortable">
                <Text>Comfortable</Text>
              </ToggleGroupItem>
            </ToggleGroup>
          </Field>
        </FieldGroup>
      </CardContent>
    </Card>
  );
}

export function InvoicesCard(props: { class?: string } = {}) {
  const page = signal(1);
  return (
    <Card class={props.class}>
      <CardHeader>
        <CardTitle>
          <Text>Invoices</Text>
        </CardTitle>
        <CardDescription>
          <Text>The latest first.</Text>
        </CardDescription>
      </CardHeader>
      <CardContent class="gap-3">
        <Table size="sm" accessibilityLabel="Invoices">
          <TableHeader>
            <TableRow>
              <TableHead class="w-20 flex-none">
                <Text>Invoice</Text>
              </TableHead>
              <TableHead>
                <Text>Status</Text>
              </TableHead>
              <TableHead class="w-24 flex-none items-end">
                <Text>Amount</Text>
              </TableHead>
            </TableRow>
          </TableHeader>
          <TableBody>
            {invoices[page() - 1].map((i) => (
              <TableRow>
                <TableCell class="w-20 flex-none">
                  <Text>{i.id}</Text>
                </TableCell>
                <TableCell>
                  <Badge variant={i.variant}>
                    <Text>{i.status}</Text>
                  </Badge>
                </TableCell>
                <TableCell class="w-24 flex-none items-end">
                  <Text class="tabular-nums">{i.amount}</Text>
                </TableCell>
              </TableRow>
            ))}
          </TableBody>
        </Table>
        <Pagination
          size="sm"
          page={page()}
          totalPages={invoices.length}
          compact
          onChange={({ page: next }) => page.set(next)}
          accessibilityLabel="Invoice pages"
        >
          <Muted xs>{`${page()} / ${invoices.length}`}</Muted>
        </Pagination>
      </CardContent>
    </Card>
  );
}

type CardView = (props: { class?: string }) => JSX.Element;

/**
 * The cards in columns: three, two or one, balanced by height. The last card
 * of each column takes what is left, so the columns end on one line.
 */
export function Showcase(props: { columns: number }) {
  const layouts: Record<number, CardView[][]> = {
    3: [
      [SignInCard, MeetingCard, ShortcutsCard, ExportCard],
      [RevenueCard, PlanCard, InvoicesCard, ActivityCard],
      [NotificationsCard, DisplayCard, InviteCard, FeedbackCard],
    ],
    2: [
      [SignInCard, MeetingCard, PlanCard, ActivityCard, ExportCard],
      [RevenueCard, NotificationsCard, DisplayCard, InvoicesCard, InviteCard, ShortcutsCard, FeedbackCard],
    ],
    1: [
      [
        SignInCard,
        RevenueCard,
        NotificationsCard,
        MeetingCard,
        PlanCard,
        DisplayCard,
        InvoicesCard,
        InviteCard,
        ActivityCard,
        ShortcutsCard,
        ExportCard,
        FeedbackCard,
      ],
    ],
  };
  return (
    <View class="flex-row gap-4">
      {layouts[props.columns].map((column) => (
        <View class="flex-1 gap-4">
          {column.map((C, i) => (
            <C class={props.columns > 1 && i === column.length - 1 ? "flex-1" : undefined} />
          ))}
        </View>
      ))}
    </View>
  );
}
