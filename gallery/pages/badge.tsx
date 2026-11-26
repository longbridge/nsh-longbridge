// The Badge page: variants, icons, counts, truncation, parts.

import { Text, View } from "nsh";
import { Badge, BadgeVariant, Icon } from "nsh-longbridge";
import { Page, Preview, Section } from "../ui";

const variants: BadgeVariant[] = ["default", "secondary", "destructive", "outline"];
const title = (s: string) => s[0].toUpperCase() + s.slice(1);
const counts = ["8", "99", "20+"];
const statuses: { label: string; variant: BadgeVariant }[] = [
  { label: "Published", variant: "default" },
  { label: "Draft", variant: "secondary" },
  { label: "Failed", variant: "destructive" },
  { label: "Archived", variant: "outline" },
];

export function BadgePage() {
  return (
    <Page
      title="Badge"
      description="A small mark: a status, a category, a version or a count. Four variants, one size, not interactive."
    >
      <Section title="Variants" description="Default for the main mark; secondary for a quieter one; destructive for a failure; outline for an edge alone.">
        <Preview code={variants.map((v) => `<Badge variant="${v}">${title(v)}</Badge>`).join("\n")}>
          {variants.map((v) => (
            <Badge variant={v}>
              <Text>{title(v)}</Text>
            </Badge>
          ))}
        </Preview>
      </Section>

      <Section title="With an icon" description="The library's Icon is 12 inside a badge, colored as its text.">
        <Preview
          code={[
            `<Badge variant="secondary"><Icon name="plus" />New</Badge>`,
            `<Badge variant="outline">Next<Icon name="arrow-right" /></Badge>`,
            `<Badge variant="destructive"><Icon name="trash-2" />Deleted</Badge>`,
          ].join("\n")}
        >
          <Badge variant="secondary">
            <Icon name="plus" />
            <Text>New</Text>
          </Badge>
          <Badge variant="outline">
            <Text>Next</Text>
            <Icon name="arrow-right" />
          </Badge>
          <Badge variant="destructive">
            <Icon name="trash-2" />
            <Text>Deleted</Text>
          </Badge>
        </Preview>
      </Section>

      <Section title="Counts" description="A count is a class, not a variant: a minimum width, less padding, tabular digits.">
        <Preview code={counts.map((n) => `<Badge class="min-w-5 px-1 tabular-nums">${n}</Badge>`).join("\n")}>
          {counts.map((n) => (
            <Badge class="min-w-5 px-1 tabular-nums">
              <Text>{n}</Text>
            </Badge>
          ))}
        </Preview>
      </Section>

      <Section title="Statuses" description="Beside a row's title. Market colors and other domain states are scoped tokens or classes, not variants.">
        <Preview
          class="flex-col items-stretch"
          code={statuses.map((s) => `<Badge variant="${s.variant}">${s.label}</Badge>`).join("\n")}
        >
          {statuses.map((s) => (
            <View class="flex-row items-center justify-between gap-3">
              <Text class="text-sm text-foreground">Quarterly report</Text>
              <Badge variant={s.variant}>
                <Text>{s.label}</Text>
              </Badge>
            </View>
          ))}
        </Preview>
      </Section>

      <Section title="Long text" description="One line, cut with an ellipsis at its parent's width.">
        <Preview code={`<View class="w-40">\n  <Badge variant="secondary">A label much too long for its place</Badge>\n</View>`}>
          <View class="w-40">
            <Badge variant="secondary">
              <Text>A label much too long for its place</Text>
            </Badge>
          </View>
        </Preview>
      </Section>

      <Section title="Parts" description="labelClass styles the text each child sits in; class styles the root.">
        <Preview
          code={[
            `<Badge variant="outline" labelClass="uppercase tracking-wide">Beta</Badge>`,
            `<Badge class="rounded-sm">Square</Badge>`,
          ].join("\n")}
        >
          <Badge variant="outline" labelClass="uppercase tracking-wide">
            <Text>Beta</Text>
          </Badge>
          <Badge class="rounded-sm">
            <Text>Square</Text>
          </Badge>
        </Preview>
      </Section>
    </Page>
  );
}
