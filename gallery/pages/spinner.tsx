// The Spinner page: sizes, colors, in a row of text and in a button.

import { Text, View } from "nsh";
import { Button, Size, Spinner } from "nsh-longbridge";
import { Caption, Page, Preview, Section } from "../ui";

const sizes: Size[] = ["xs", "sm", "md", "lg"];
const colors = ["text-foreground", "text-muted-foreground", "text-primary", "text-destructive"];

export function SpinnerPage() {
  return (
    <Page
      title="Spinner"
      description="Something is going on and how long it takes is unknown. It keeps turning with reduced motion: its turning is what it says."
    >
      <Section title="Sizes" description="12 / 14 / 16 / 16, the icon sizes. Larger with a class.">
        <Preview code={[...sizes.map((s) => `<Spinner size="${s}" />`), `<Spinner class="size-8" />`].join("\n")}>
          {sizes.map((s) => (
            <View class="items-center gap-2">
              <Spinner size={s} />
              <Caption>{s}</Caption>
            </View>
          ))}
          <View class="items-center gap-2">
            <Spinner class="size-8" />
            <Caption>size-8</Caption>
          </View>
        </Preview>
      </Section>

      <Section title="Color" description="The color of the text around it.">
        <Preview code={colors.map((c) => `<Spinner class="${c}" />`).join("\n")}>
          {colors.map((c) => (
            <Spinner class={c} />
          ))}
        </Preview>
      </Section>

      <Section title="In a row" description="Beside a status; in a button, it takes the button's size and color.">
        <Preview
          code={[
            `<View class="flex-row items-center gap-2">`,
            `  <Spinner size="sm" />`,
            `  <Text class="text-sm text-muted-foreground">Syncing…</Text>`,
            `</View>`,
            `<Button variant="outline" size="sm" disabled><Spinner />Please wait</Button>`,
          ].join("\n")}
        >
          <View class="flex-row items-center gap-2 text-muted-foreground">
            <Spinner size="sm" />
            <Text class="text-sm text-muted-foreground">Syncing…</Text>
          </View>
          <Button variant="outline" size="sm" disabled>
            <Spinner />
            <Text>Please wait</Text>
          </Button>
        </Preview>
      </Section>
    </Page>
  );
}
