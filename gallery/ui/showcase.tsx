// The showcase: screens of a product made only of the library's components,
// which the home page and the theme editor show. Every color, radius and
// height comes from the theme, so all of it follows the theme controls.

import { keymap, signal, Text, View, type JSX } from "nsh";
import {
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
  Input,
  type InputRef,
  Kbd,
  KbdGroup,
  Label,
  RadioGroup,
  RadioGroupItem,
  Separator,
  Skeleton,
  Spinner,
  Switch,
  Textarea,
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

const members: { initials: string; name: string; email: string; role: string; variant: BadgeVariant }[] = [
  { initials: "SD", name: "Sofia Davis", email: "sofia@example.com", role: "Owner", variant: "default" },
  { initials: "JL", name: "Jackson Lee", email: "jackson@example.com", role: "Member", variant: "secondary" },
  { initials: "IN", name: "Isabella Nguyen", email: "isabella@example.com", role: "Invited", variant: "outline" },
];

const shortcuts = [
  { action: "Search", keys: "mod+k" },
  { action: "New file", keys: "mod+n" },
  { action: "Toggle sidebar", keys: "mod+b" },
  { action: "Command palette", keys: "mod+shift+p" },
  { action: "Save", keys: "mod+s" },
];

const revenue = [42, 58, 49, 66, 72, 61, 80, 74, 91, 86, 98, 104];

// Text here takes the card's color, never a Tailwind color class: those read
// the app's theme, while a ThemeScope reaches only the library's components.
// Muted text is a CardDescription, so it follows a scope too.

/** Initials in a circle: a secondary badge, so a scope colors it. */
function Avatar(props: { initials: string }) {
  return (
    <Badge variant="secondary" class="size-8 rounded-full px-0 py-0">
      <Text>{props.initials}</Text>
    </Badge>
  );
}

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
              <Avatar initials={m.initials} />
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
          <Avatar initials="OM" />
          <View class="flex-1 gap-0.5">
            <Text class="text-sm">Olivia deployed acme-web</Text>
            <Muted xs>2 minutes ago · main</Muted>
          </View>
          <Badge variant="secondary">
            <Text>Ready</Text>
          </Badge>
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

type CardView = (props: { class?: string }) => JSX.Element;

/**
 * The cards in columns: three, two or one, balanced by height. The last card
 * of each column takes what is left, so the columns end on one line.
 */
export function Showcase(props: { columns: number }) {
  const layouts: Record<number, CardView[][]> = {
    3: [
      [SignInCard, ShortcutsCard, ExportCard],
      [RevenueCard, PlanCard, ActivityCard],
      [NotificationsCard, InviteCard, FeedbackCard],
    ],
    2: [
      [SignInCard, PlanCard, ActivityCard, ExportCard],
      [RevenueCard, NotificationsCard, InviteCard, ShortcutsCard, FeedbackCard],
    ],
    1: [[SignInCard, RevenueCard, NotificationsCard, PlanCard, InviteCard, ActivityCard, ShortcutsCard, ExportCard, FeedbackCard]],
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
