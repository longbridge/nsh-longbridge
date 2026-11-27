// The Tabs page: account settings in cards, the two looks, sizes, icons and
// a disabled tab, vertical, controlled.

import { signal, Text, View } from "nsh";
import {
  Badge,
  Button,
  Card,
  CardContent,
  CardDescription,
  CardFooter,
  CardHeader,
  CardTitle,
  Input,
  Label,
  Switch,
  type Size,
  Tabs,
  TabsContent,
  TabsList,
  TabsTrigger,
  type TabsVariant,
} from "nsh-longbridge";
import { Demo, Demos, Page, Row, Section } from "../ui/layout";
import { TokensSection } from "../ui/tokens";

const sizes: Size[] = ["xs", "sm", "md", "lg"];
const variants: TabsVariant[] = ["default", "line"];
const ranges = ["Day", "Week", "Month", "Year"];
const steps = [
  { value: "details", title: "Details", body: "Name the project and pick its region." },
  { value: "team", title: "Team", body: "Invite the people who deploy it." },
  { value: "deploy", title: "Deploy", body: "Connect a repository and ship the first build." },
];

export function TabsPage() {
  return (
    <Page
      eyebrow="Components"
      title="Tabs"
      description="Views of one page, one shown at a time: a list of triggers and a panel for each. A pill on a muted track, or a line under the chosen tab; horizontal or vertical."
    >
      <Demo hero>
        <AccountTabs />
      </Demo>

      <Section
        title="Variants"
        description={`The default look puts the chosen tab on a raised pill; variant="line" underlines it, for a page's sections.`}
      >
        <Demos>
          {variants.map((variant) => (
            <Demo label={variant}>
              <Tabs defaultValue="week">
                <TabsList variant={variant} accessibilityLabel={`Range, ${variant}`}>
                  {ranges.map((r) => (
                    <TabsTrigger value={r.toLowerCase()}>
                      <Text>{r}</Text>
                    </TabsTrigger>
                  ))}
                </TabsList>
              </Tabs>
            </Demo>
          ))}
        </Demos>
      </Section>

      <Section title="Sizes" description="The size is set on the list: its height, the triggers' text and padding, as a Button of that size.">
        <Demo class="items-stretch">
          {sizes.map((size) => (
            <Row label={size}>
              <Tabs defaultValue="week">
                <TabsList size={size} accessibilityLabel={`Range, ${size}`}>
                  {ranges.map((r) => (
                    <TabsTrigger value={r.toLowerCase()}>
                      <Text>{r}</Text>
                    </TabsTrigger>
                  ))}
                </TabsList>
              </Tabs>
              <Button size={size} variant="outline">
                <Text>Export</Text>
              </Button>
            </Row>
          ))}
        </Demo>
      </Section>

      <Section
        title="Icons and a disabled tab"
        description="A trigger's icon sits before its text. A disabled trigger is dimmed and the arrow keys pass over it."
      >
        <ProjectTabs />
      </Section>

      <Section
        title="Vertical and controlled"
        description={`With orientation="vertical" the triggers stack beside the panels; the up and down arrows move between them. The app can own value, here with steps of a wizard.`}
      >
        <Demos>
          <SettingsDemo />
          <WizardDemo />
        </Demos>
      </Section>

      <TokensSection prefix="tabs" component="Tabs" />
    </Page>
  );
}

/** Account and password cards, one tab each. */
function AccountTabs() {
  return (
    <Tabs defaultValue="account" class="w-full max-w-[400px] gap-3">
      <TabsList accessibilityLabel="Settings">
        <TabsTrigger value="account">
          <Text>Account</Text>
        </TabsTrigger>
        <TabsTrigger value="password">
          <Text>Password</Text>
        </TabsTrigger>
      </TabsList>
      <TabsContent value="account">
        <Card>
          <CardHeader>
            <CardTitle>
              <Text>Account</Text>
            </CardTitle>
            <CardDescription>
              <Text>Make changes to your account here. Click save when you're done.</Text>
            </CardDescription>
          </CardHeader>
          <CardContent class="gap-4">
            <View class="gap-2">
              <Label>
                <Text>Name</Text>
              </Label>
              <Input defaultValue="Pedro Duarte" accessibilityLabel="Name" />
            </View>
            <View class="gap-2">
              <Label>
                <Text>Username</Text>
              </Label>
              <Input defaultValue="@peduarte" accessibilityLabel="Username" />
            </View>
          </CardContent>
          <CardFooter>
            <Button>
              <Text>Save changes</Text>
            </Button>
          </CardFooter>
        </Card>
      </TabsContent>
      <TabsContent value="password">
        <Card>
          <CardHeader>
            <CardTitle>
              <Text>Password</Text>
            </CardTitle>
            <CardDescription>
              <Text>Change your password here. After saving, you'll be logged out.</Text>
            </CardDescription>
          </CardHeader>
          <CardContent class="gap-4">
            <View class="gap-2">
              <Label>
                <Text>Current password</Text>
              </Label>
              <Input masked accessibilityLabel="Current password" />
            </View>
            <View class="gap-2">
              <Label>
                <Text>New password</Text>
              </Label>
              <Input masked accessibilityLabel="New password" />
            </View>
          </CardContent>
          <CardFooter>
            <Button>
              <Text>Save password</Text>
            </Button>
          </CardFooter>
        </Card>
      </TabsContent>
    </Tabs>
  );
}

/** A project's views, underlined, with icons; billing is not open to this member. */
function ProjectTabs() {
  return (
    <Demo class="items-stretch justify-start">
      <Tabs defaultValue="overview" class="w-full gap-4">
        <TabsList variant="line" accessibilityLabel="Project">
          <TabsTrigger value="overview" icon="layout-dashboard">
            <Text>Overview</Text>
          </TabsTrigger>
          <TabsTrigger value="activity" icon="activity">
            <Text>Activity</Text>
          </TabsTrigger>
          <TabsTrigger value="settings" icon="settings">
            <Text>Settings</Text>
          </TabsTrigger>
          <TabsTrigger value="billing" disabled>
            <Text>Billing</Text>
          </TabsTrigger>
        </TabsList>
        <TabsContent value="overview">
          <View class="gap-1">
            <Text class="text-2xl font-semibold tabular-nums text-foreground">99.98%</Text>
            <Text class="text-sm text-muted-foreground">Uptime over the last 30 days.</Text>
          </View>
        </TabsContent>
        <TabsContent value="activity">
          <View class="flex-row items-center gap-2">
            <Badge variant="secondary">
              <Text>Ready</Text>
            </Badge>
            <Text class="text-sm text-muted-foreground">Olivia deployed main 2 minutes ago.</Text>
          </View>
        </TabsContent>
        <TabsContent value="settings">
          <Text class="text-sm text-muted-foreground">Domains, environment variables and build settings.</Text>
        </TabsContent>
      </Tabs>
    </Demo>
  );
}

/** Settings in a side column. */
function SettingsDemo() {
  return (
    <Demo label={`orientation="vertical"`} class="items-stretch justify-start">
      <Tabs defaultValue="general" orientation="vertical" class="gap-6">
        <TabsList accessibilityLabel="Settings sections">
          <TabsTrigger value="general">
            <Text>General</Text>
          </TabsTrigger>
          <TabsTrigger value="notifications">
            <Text>Notifications</Text>
          </TabsTrigger>
          <TabsTrigger value="security">
            <Text>Security</Text>
          </TabsTrigger>
        </TabsList>
        <TabsContent value="general" class="flex-1">
          <Setting label="Compact sidebar" />
        </TabsContent>
        <TabsContent value="notifications" class="flex-1">
          <Setting label="Email me about mentions" on />
        </TabsContent>
        <TabsContent value="security" class="flex-1">
          <Setting label="Two-factor authentication" on />
        </TabsContent>
      </Tabs>
    </Demo>
  );
}

/** A wizard: the app owns the tab, Back and Next move it. */
function WizardDemo() {
  const step = signal(0);
  return (
    <Demo label={`value: ${steps[step()].value}`} class="items-stretch justify-start">
      <Tabs value={steps[step()].value} onChange={(e) => step.set(steps.findIndex((s) => s.value === e.value))} class="gap-4">
        <TabsList variant="line" accessibilityLabel="Steps">
          {steps.map((s, i) => (
            <TabsTrigger value={s.value}>
              <Text>{`${i + 1}. ${s.title}`}</Text>
            </TabsTrigger>
          ))}
        </TabsList>
        {steps.map((s) => (
          <TabsContent value={s.value}>
            <Text class="text-sm text-muted-foreground">{s.body}</Text>
          </TabsContent>
        ))}
      </Tabs>
      <View class="flex-row justify-end gap-2">
        <Button variant="outline" size="sm" disabled={step() === 0} onPress={() => step.set(step() - 1)}>
          <Text>Back</Text>
        </Button>
        <Button size="sm" disabled={step() === steps.length - 1} onPress={() => step.set(step() + 1)}>
          <Text>Next</Text>
        </Button>
      </View>
    </Demo>
  );
}

/** A switch that keeps its own state. */
function Setting(props: { label: string; on?: boolean }) {
  const on = signal(props.on ?? false);
  return (
    <Switch checked={on()} onChange={(e) => on.set(e.checked)}>
      <Text>{props.label}</Text>
    </Switch>
  );
}
