// The Badge page: variants, icons, counts and statuses in their places.

import { Text, View } from "nsh";
import { Badge, type BadgeVariant, Icon } from "nsh-longbridge";
import { Demo, Demos, Page, Section } from "../ui/layout";
import { TokensSection } from "../ui/tokens";

const variants: { variant: BadgeVariant; label: string }[] = [
  { variant: "default", label: "Default" },
  { variant: "secondary", label: "Secondary" },
  { variant: "destructive", label: "Destructive" },
  { variant: "outline", label: "Outline" },
];
const counts = ["3", "28", "99+"];
const inbox = [
  { label: "Inbox", count: "28" },
  { label: "Mentions", count: "3" },
  { label: "Archive", count: "" },
];
const releases: { name: string; date: string; status: string; variant: BadgeVariant }[] = [
  { name: "v2.4.0", date: "Today", status: "Published", variant: "default" },
  { name: "v2.4.0-rc.2", date: "Yesterday", status: "Draft", variant: "secondary" },
  { name: "v2.3.9", date: "Mar 12", status: "Failed", variant: "destructive" },
  { name: "v2.3.8", date: "Mar 4", status: "Archived", variant: "outline" },
];

export function BadgePage() {
  return (
    <Page
      eyebrow="Components"
      title="Badge"
      description="A small mark: a status, a category, a version or a count. Four variants, one size, not interactive."
    >
      <Demo hero>
        <View class="w-full max-w-[480px] gap-8">
          <View class="gap-3 rounded-xl border border-border p-5">
            <View class="flex-row items-center gap-2">
              <Badge>
                <Icon name="check" />
                <Text>Open</Text>
              </Badge>
              <Text class="text-xs text-muted-foreground">#1284 · opened 2 days ago</Text>
            </View>
            <Text class="text-lg font-semibold tracking-tight text-foreground">The sidebar flickers while the window resizes</Text>
            <View class="flex-row flex-wrap items-center gap-1.5">
              <Badge variant="destructive">
                <Text>bug</Text>
              </Badge>
              <Badge variant="secondary">
                <Text>desktop</Text>
              </Badge>
              <Badge variant="outline">
                <Text>v2.4.0</Text>
              </Badge>
              <Badge variant="outline">
                <Text>needs review</Text>
              </Badge>
            </View>
          </View>
          <View class="flex-row flex-wrap items-center justify-center gap-2">
            {variants.map((v) => (
              <Badge variant={v.variant}>
                <Text>{v.label}</Text>
              </Badge>
            ))}
          </View>
        </View>
      </Demo>

      <Section title="Statuses" description="Beside a row's title. Domain colors (a market's up and down) are scoped tokens, not variants.">
        <Demo class="items-stretch">
          <View class="w-full max-w-[460px] self-center rounded-lg border border-border">
            {releases.map((r, i) => (
              <View class={`flex-row items-center gap-3 px-4 py-3 ${i > 0 ? "border-t border-border" : ""}`}>
                <Text class="flex-1 font-mono text-sm text-foreground">{r.name}</Text>
                <Text class="text-sm text-muted-foreground">{r.date}</Text>
                <View class="w-24 items-end">
                  <Badge variant={r.variant}>
                    <Text>{r.status}</Text>
                  </Badge>
                </View>
              </View>
            ))}
          </View>
        </Demo>
      </Section>

      <Section title="Icons and counts" description="The library's icons are 12 inside a badge, colored as its text. A count is a class: a least width, tighter padding, tabular digits.">
        <Demos>
          <Demo row label="With an icon">
            <Badge variant="secondary">
              <Icon name="check" />
              <Text>Verified</Text>
            </Badge>
            <Badge variant="outline">
              <Text>Next</Text>
              <Icon name="arrow-right" />
            </Badge>
            <Badge variant="destructive">
              <Icon name="trash-2" />
              <Text>Deleted</Text>
            </Badge>
          </Demo>
          <Demo label="Counts beside a list">
            <View class="w-48 gap-1">
              {inbox.map((item) => (
                <View class="h-8 flex-row items-center justify-between rounded-md px-2">
                  <Text class="text-sm text-foreground">{item.label}</Text>
                  <View class="justify-center">
                    <If when={item.count}>
                      <Badge variant={item.label === "Mentions" ? "default" : "secondary"} class="min-w-5 px-1 tabular-nums">
                        <Text>{item.count}</Text>
                      </Badge>
                    </If>
                  </View>
                </View>
              ))}
            </View>
          </Demo>
        </Demos>
      </Section>

      <Section title="Long text" description="One line, cut with an ellipsis at its parent's width; and restyled per instance.">
        <Demos>
          <Demo label="Cut at 160 points">
            <View class="w-40">
              <Badge variant="secondary">
                <Text>A label much too long for its place</Text>
              </Badge>
            </View>
          </Demo>
          <Demo row label="Uppercase and square, with labelClass and class">
            <Badge variant="outline" labelClass="uppercase tracking-wide">
              <Text>Beta</Text>
            </Badge>
            <Badge class="rounded-sm">
              <Text>Square</Text>
            </Badge>
            {counts.map((n) => (
              <Badge variant="destructive" class="min-w-5 px-1 tabular-nums">
                <Text>{n}</Text>
              </Badge>
            ))}
          </Demo>
        </Demos>
      </Section>

      <TokensSection prefix="badge" component="Badge" />
    </Page>
  );
}
