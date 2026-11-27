// The Separator page: a profile header, horizontal and vertical, shorter,
// in a list.

import { Text, View } from "nsh";
import { Separator } from "nsh-longbridge";
import { Demo, Demos, Page, Section } from "../ui/layout";
import { TokensSection } from "../ui/tokens";

const links = ["Blog", "Docs", "Source"];
const formats = ["Bold", "Italic", "Underline"];
const settings = [
  { title: "Language", value: "English" },
  { title: "Time zone", value: "Europe/London" },
  { title: "Date format", value: "26 Nov 2026" },
];

export function SeparatorPage() {
  return (
    <Page
      eyebrow="Components"
      title="Separator"
      description="A thin line between sections, or between the items of a row. Decorative: screen readers skip it."
    >
      <Demo hero>
        <View class="w-full max-w-[360px] gap-4">
          <View class="gap-1">
            <Text class="text-sm font-medium leading-none text-foreground">NSH Primitives</Text>
            <Text class="text-sm text-muted-foreground">Unstyled components for native apps.</Text>
          </View>
          <Separator />
          <View class="h-5 flex-row items-center gap-4">
            {links.flatMap((l, i) => [
              ...(i > 0 ? [<Separator orientation="vertical" />] : []),
              <Text class="text-sm text-foreground">{l}</Text>,
            ])}
          </View>
        </View>
      </Demo>

      <Section title="Lengths" description="It stretches across its parent: the column's width, the row's height. A length in class stops the stretch.">
        <Demos>
          <Demo row label="Shorter than its row, with h-4">
            <View class="h-8 flex-row items-center gap-3 rounded-md border border-border px-3">
              {formats.flatMap((f, i) => [
                ...(i > 0 ? [<Separator orientation="vertical" class="h-4" />] : []),
                <Text class="text-sm text-foreground">{f}</Text>,
              ])}
            </View>
          </Demo>
          <Demo label="Between the rows of a list">
            <View class="w-full max-w-64">
              {settings.map((s, i) => (
                <View>
                  <If when={i > 0}>
                    <Separator />
                  </If>
                  <View class="flex-row items-center justify-between py-3">
                    <Text class="text-sm text-foreground">{s.title}</Text>
                    <Text class="text-sm text-muted-foreground">{s.value}</Text>
                  </View>
                </View>
              ))}
            </View>
          </Demo>
        </Demos>
      </Section>

      <TokensSection prefix="separator" component="Separator" />
    </Page>
  );
}
