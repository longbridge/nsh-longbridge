// The Card page: the full structure, without an action, with dividers, in
// a scope.

import { Text, View } from "nsh";
import {
  Button,
  Card,
  CardAction,
  CardContent,
  CardDescription,
  CardFooter,
  CardHeader,
  CardTitle,
  ThemeScope,
} from "nsh-longbridge";
import { Page, Preview, Section } from "../ui";

const fields = [
  { label: "Email", value: "m@example.com" },
  { label: "Password", value: "••••••••" },
];
const stats = [
  { title: "Total revenue", value: "$15,231.89", note: "+20.1% from last month" },
  { title: "Subscriptions", value: "+2,350", note: "+180.1% from last month" },
];

export function CardPage() {
  return (
    <Page
      title="Card"
      description="A bordered surface with a light shadow, laid out as a header (title, description, action), content and footer. Not interactive."
    >
      <Section title="Structure" description="CardAction sits at the top right, beside the title and the description.">
        <Preview
          code={[
            `<Card class="w-[360px]">`,
            `  <CardHeader>`,
            `    <CardTitle>Login to your account</CardTitle>`,
            `    <CardDescription>Enter your email below to login to your account</CardDescription>`,
            `    <CardAction>`,
            `      <Button variant="link">Sign Up</Button>`,
            `    </CardAction>`,
            `  </CardHeader>`,
            `  <CardContent>{/* fields */}</CardContent>`,
            `  <CardFooter class="gap-2">`,
            `    <Button class="flex-1">Login</Button>`,
            `  </CardFooter>`,
            `</Card>`,
          ].join("\n")}
        >
          <Card class="w-[360px]">
            <CardHeader>
              <CardTitle>
                <Text>Login to your account</Text>
              </CardTitle>
              <CardDescription>
                <Text>Enter your email below to login to your account</Text>
              </CardDescription>
              <CardAction>
                <Button variant="link">
                  <Text>Sign Up</Text>
                </Button>
              </CardAction>
            </CardHeader>
            <CardContent>
              <View class="gap-4">
                {fields.map((f) => (
                  <View class="gap-1">
                    <Text class="text-sm font-medium">{f.label}</Text>
                    <Text class="text-sm text-muted-foreground">{f.value}</Text>
                  </View>
                ))}
              </View>
            </CardContent>
            <CardFooter class="gap-2">
              <Button class="flex-1">
                <Text>Login</Text>
              </Button>
            </CardFooter>
          </Card>
        </Preview>
      </Section>

      <Section title="Without an action" description="The title spans the header. Text inside takes card-foreground.">
        <Preview
          code={[
            `<Card class="w-[260px]">`,
            `  <CardHeader>`,
            `    <CardDescription>Total revenue</CardDescription>`,
            `    <CardTitle class="text-2xl">$15,231.89</CardTitle>`,
            `  </CardHeader>`,
            `  <CardFooter>`,
            `    <Text class="text-xs text-muted-foreground">+20.1% from last month</Text>`,
            `  </CardFooter>`,
            `</Card>`,
          ].join("\n")}
        >
          {stats.map((s) => (
            <Card class="w-[260px]">
              <CardHeader>
                <CardDescription>
                  <Text>{s.title}</Text>
                </CardDescription>
                <CardTitle class="text-2xl">
                  <Text>{s.value}</Text>
                </CardTitle>
              </CardHeader>
              <CardFooter>
                <Text class="text-xs text-muted-foreground">{s.note}</Text>
              </CardFooter>
            </Card>
          ))}
        </Preview>
      </Section>

      <Section title="Dividers" description="Give the divider its padding along with its edge: border-b pb-6 on the header, border-t pt-6 on the footer.">
        <Preview
          code={[
            `<Card class="w-[360px]">`,
            `  <CardHeader class="border-b pb-6">…</CardHeader>`,
            `  <CardContent>…</CardContent>`,
            `  <CardFooter class="border-t pt-6 justify-end gap-2">…</CardFooter>`,
            `</Card>`,
          ].join("\n")}
        >
          <Card class="w-[360px]">
            <CardHeader class="border-b border-border pb-6">
              <CardTitle>
                <Text>Notifications</Text>
              </CardTitle>
              <CardDescription>
                <Text>You have 3 unread messages.</Text>
              </CardDescription>
            </CardHeader>
            <CardContent>
              <Text class="text-sm">Your call has been confirmed.</Text>
            </CardContent>
            <CardFooter class="border-t border-border pt-6 justify-end gap-2">
              <Button variant="outline">
                <Text>Later</Text>
              </Button>
              <Button>
                <Text>Mark all as read</Text>
              </Button>
            </CardFooter>
          </Card>
        </Preview>
      </Section>

      <Section title="Scoped tokens" description="No variants: a flat, dense card is two tokens in a scope.">
        <Preview
          code={[
            `<ThemeScope tokens={{ "card-shadow": 0, "card-padding": "16px", "card-gap": "16px" }}>`,
            `  <Card class="w-[260px]">…</Card>`,
            `</ThemeScope>`,
          ].join("\n")}
        >
          <ThemeScope tokens={{ "card-shadow": 0, "card-padding": "16px", "card-gap": "16px" }}>
            <Card class="w-[260px]">
              <CardHeader>
                <CardTitle>
                  <Text>Compact</Text>
                </CardTitle>
                <CardDescription>
                  <Text>No shadow, 16 of padding.</Text>
                </CardDescription>
              </CardHeader>
              <CardContent>
                <Text class="text-sm">Body text.</Text>
              </CardContent>
            </Card>
          </ThemeScope>
        </Preview>
      </Section>
    </Page>
  );
}
