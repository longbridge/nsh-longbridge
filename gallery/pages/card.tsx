// The Card page: the full structure, a stat, dividers, a scoped dense card.

import { signal, Text, View } from "nsh";
import {
  Badge,
  Button,
  Card,
  CardAction,
  CardContent,
  CardDescription,
  CardFooter,
  CardHeader,
  CardTitle,
  Switch,
  ThemeScope,
} from "nsh-longbridge";
import { Demo, Demos, Page, Section } from "../ui/layout";
import { TokensSection } from "../ui/tokens";
import { SignInCard } from "../ui/showcase";

const stats = [
  { title: "Total revenue", value: "$15,231.89", note: "+20.1% from last month", trend: "+20.1%" },
  { title: "Subscriptions", value: "+2,350", note: "+180.1% from last month", trend: "+180%" },
];
const messages = [
  { title: "Your call has been confirmed.", time: "1 hour ago" },
  { title: "You have a new message.", time: "1 hour ago" },
  { title: "Your subscription is expiring soon.", time: "2 hours ago" },
];
const projects = [
  { name: "acme-web", note: "Deployed 2 minutes ago" },
  { name: "acme-api", note: "Deployed yesterday" },
];

export function CardPage() {
  const push = signal(false);
  return (
    <Page
      eyebrow="Components"
      title="Card"
      description="A bordered surface with a light shadow, laid out as a header (title, description, action), content and footer. Not interactive."
    >
      <Demo hero>
        <View class="w-full max-w-[380px]">
          <SignInCard />
        </View>
      </Demo>

      <Section title="Figures" description="A figure as the title, its trend as the action, a footer and no content. The description may come before the title.">
        <Demo row class="gap-4">
          {stats.map((s) => (
            <Card class="w-[260px]">
              <CardHeader>
                <CardDescription>
                  <Text>{s.title}</Text>
                </CardDescription>
                <CardTitle class="text-2xl tabular-nums">
                  <Text>{s.value}</Text>
                </CardTitle>
                <CardAction>
                  <Badge variant="outline">
                    <Text>{s.trend}</Text>
                  </Badge>
                </CardAction>
              </CardHeader>
              <CardFooter>
                <Text class="text-xs text-muted-foreground">{s.note}</Text>
              </CardFooter>
            </Card>
          ))}
        </Demo>
      </Section>

      <Section title="Dividers" description="A divider takes its padding along with its edge: border-b and pb-6 on the header, border-t and pt-6 on the footer.">
        <Demo>
          <Card class="w-full max-w-[380px]">
            <CardHeader class="border-b border-border pb-6">
              <CardTitle>
                <Text>Notifications</Text>
              </CardTitle>
              <CardDescription>
                <Text>You have 3 unread messages.</Text>
              </CardDescription>
            </CardHeader>
            <CardContent class="gap-4">
              <View class="flex-row items-center gap-4 rounded-md border border-border p-4">
                <View class="flex-1 gap-1">
                  <Text class="text-sm font-medium text-card-foreground">Push notifications</Text>
                  <Text class="text-sm text-muted-foreground">Send notifications to this device.</Text>
                </View>
                <Switch checked={push()} onChange={(e) => push.set(e.checked)} accessibilityLabel="Push notifications" />
              </View>
              {messages.map((m) => (
                <View class="flex-row items-start gap-3">
                  <View class="mt-1.5 size-2 rounded-full bg-primary" />
                  <View class="flex-1 gap-1">
                    <Text class="text-sm font-medium text-card-foreground">{m.title}</Text>
                    <Text class="text-sm text-muted-foreground">{m.time}</Text>
                  </View>
                </View>
              ))}
            </CardContent>
            <CardFooter class="border-t border-border pt-6">
              <Button class="flex-1" icon="check">
                <Text>Mark all as read</Text>
              </Button>
            </CardFooter>
          </Card>
        </Demo>
      </Section>

      <Section title="Scoped tokens" description="No variants: a flat, dense card is three tokens in a ThemeScope around it.">
        <Demos>
          <Demo label="Default">
            <ProjectsCard />
          </Demo>
          <Demo label="card-shadow 0, card-padding 16, card-gap 12">
            <ThemeScope tokens={{ "card-shadow": 0, "card-padding": "16px", "card-gap": "12px" }}>
              <ProjectsCard />
            </ThemeScope>
          </Demo>
        </Demos>
      </Section>

      <TokensSection prefix="card" component="Card" note="Its fill and text are the semantic card and card-foreground." />
    </Page>
  );
}

function ProjectsCard() {
  return (
    <Card class="w-[260px]">
      <CardHeader>
        <CardTitle>
          <Text>Projects</Text>
        </CardTitle>
        <CardDescription>
          <Text>Two active.</Text>
        </CardDescription>
      </CardHeader>
      <CardContent class="gap-3">
        {projects.map((p) => (
          <View class="gap-0.5">
            <Text class="font-mono text-sm text-card-foreground">{p.name}</Text>
            <Text class="text-xs text-muted-foreground">{p.note}</Text>
          </View>
        ))}
      </CardContent>
    </Card>
  );
}
