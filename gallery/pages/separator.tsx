// The Separator page: horizontal and vertical, a shorter one, in a scope.

import { Text, View } from "nsh";
import { Separator } from "nsh-longbridge";
import { Page, Preview, Section } from "../ui";

const links = ["Blog", "Docs", "Source"];

export function SeparatorPage() {
  return (
    <Page title="Separator" description="A thin line between sections or the items of a row. Decorative: screen readers skip it.">
      <Section title="Horizontal and vertical" description="It stretches across its parent: the column's width, the row's height.">
        <Preview
          class="flex-col items-stretch"
          code={[
            `<View class="gap-1">`,
            `  <Text class="text-sm font-medium">Radix Primitives</Text>`,
            `  <Text class="text-sm text-muted-foreground">An open-source UI component library.</Text>`,
            `</View>`,
            `<Separator />`,
            `<View class="flex-row h-5 items-center gap-4">`,
            ...links.flatMap((l, i) => (i ? [`  <Separator orientation="vertical" />`] : []).concat(`  <Text>${l}</Text>`)),
            `</View>`,
          ].join("\n")}
        >
          <View class="gap-4">
            <View class="gap-1">
              <Text class="text-sm font-medium text-foreground">Radix Primitives</Text>
              <Text class="text-sm text-muted-foreground">An open-source UI component library.</Text>
            </View>
            <Separator />
            <View class="flex-row h-5 items-center gap-4">
              {links.flatMap((l, i) => [
                ...(i > 0 ? [<Separator orientation="vertical" />] : []),
                <Text class="text-sm text-foreground">{l}</Text>,
              ])}
            </View>
          </View>
        </Preview>
      </Section>

      <Section title="Shorter" description="An explicit length stops the stretch: class wins.">
        <Preview code={`<View class="flex-row h-8 items-center gap-3">\n  <Text>Bold</Text>\n  <Separator orientation="vertical" class="h-4" />\n  <Text>Italic</Text>\n</View>`}>
          <View class="flex-row h-8 items-center gap-3">
            <Text class="text-sm text-foreground">Bold</Text>
            <Separator orientation="vertical" class="h-4" />
            <Text class="text-sm text-foreground">Italic</Text>
          </View>
        </Preview>
      </Section>
    </Page>
  );
}
