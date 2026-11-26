// The Button page: variants, sizes, icons, states, parts.

import { signal, Text, View } from "nsh";
import { Button, ButtonVariant, Size } from "nsh-longbridge";
import { Caption, Page, Preview, Section } from "../ui";

const variants: ButtonVariant[] = ["default", "secondary", "outline", "ghost", "destructive", "link"];
const sizes: Size[] = ["xs", "sm", "md", "lg"];
const title = (s: string) => s[0].toUpperCase() + s.slice(1);

export function ButtonPage() {
  return (
    <Page
      title="Button"
      description="Runs one action when pressed. shadcn/ui's button on nsh/base's Button: six variants, four sizes, icons, an icon-only square and a loading state."
    >
      <Section title="Variants" description="Default for the main action; outline for the ordinary one beside it; destructive for deleting.">
        <Preview
          code={variants.map((v) => `<Button variant="${v}">${title(v)}</Button>`).join("\n")}
        >
          {variants.map((v) => (
            <Button variant={v}>
              <Text>{title(v)}</Text>
            </Button>
          ))}
        </Preview>
      </Section>

      <Section
        title="Sizes"
        description="Height, type, icon and padding change together. Touch screens get taller buttons and a 44pt press area."
      >
        <Preview
          class="flex-col items-start"
          code={[
            `<Button size="xs">Extra small</Button>`,
            `<Button size="sm" variant="outline" icon="plus">Small</Button>`,
            `<Button size="lg" iconEnd="arrow-right">Large</Button>`,
            `<Button size="md" variant="outline" iconOnly icon="plus" accessibilityLabel="Add" />`,
          ].join("\n")}
        >
          {sizes.map((size) => (
            <View class="flex-row items-center gap-3">
              <Caption class="w-8">{size}</Caption>
              <Button size={size}>
                <Text>Button</Text>
              </Button>
              <Button size={size} variant="outline" icon="plus">
                <Text>New file</Text>
              </Button>
              <Button size={size} variant="secondary" iconEnd="chevron-down">
                <Text>Options</Text>
              </Button>
              <Button size={size} variant="outline" iconOnly icon="plus" accessibilityLabel="Add" />
              <Button size={size} variant="ghost" iconOnly icon="trash-2" accessibilityLabel="Delete" />
            </View>
          ))}
        </Preview>
      </Section>

      <Section title="Icons" description="An icon before or after the label, or alone in a square (give it an accessibilityLabel).">
        <Preview
          code={[
            `<Button variant="outline" icon="plus">New file</Button>`,
            `<Button iconEnd="arrow-right">Continue</Button>`,
            `<Button variant="destructive" icon="trash-2">Delete</Button>`,
            `<Button variant="ghost" iconOnly icon="plus" accessibilityLabel="Add" />`,
          ].join("\n")}
        >
          <Button variant="outline" icon="plus">
            <Text>New file</Text>
          </Button>
          <Button iconEnd="arrow-right">
            <Text>Continue</Text>
          </Button>
          <Button variant="destructive" icon="trash-2">
            <Text>Delete</Text>
          </Button>
          <Button variant="ghost" iconOnly icon="plus" accessibilityLabel="Add" />
        </Preview>
      </Section>

      <Section
        title="States"
        description="Disabled dims and takes no presses. Loading shows a spinner, takes no presses and does not dim. Invalid marks the outline's edge. Press Tab to see the focus ring."
      >
        <StateMatrix />
      </Section>

      <Section title="Loading" description="Press it: the action runs for two seconds.">
        <LoadingDemo />
      </Section>

      <Section title="Parts" description="labelClass and iconClass style the label and the icons; class styles the root.">
        <Preview
          code={[
            `<Button variant="outline" labelClass="font-semibold" iconClass="text-destructive" icon="trash-2">`,
            `  Remove`,
            `</Button>`,
            `<Button variant="secondary" class="rounded-full">Rounded</Button>`,
          ].join("\n")}
        >
          <Button variant="outline" labelClass="font-semibold" iconClass="text-destructive" icon="trash-2">
            <Text>Remove</Text>
          </Button>
          <Button variant="secondary" class="rounded-full">
            <Text>Rounded</Text>
          </Button>
        </Preview>
      </Section>

      <Section title="String children" description="With NSH 0.7.0 a string child of a Rust component does not show yet (fixed on NSH main). Until the next release, write it inside Text.">
        <Preview code={`// NSH 0.7.0\n<Button><Text>Save</Text></Button>\n// later\n<Button>Save</Button>`}>
          <Button>
            <Text>Save</Text>
          </Button>
        </Preview>
      </Section>
    </Page>
  );
}

const columns = ["Rest", "Disabled", "Loading", "Invalid"];

function StateMatrix() {
  return (
    <Preview
      class="flex-col items-stretch gap-2"
      code={[
        `<Button disabled>Disabled</Button>`,
        `<Button loading>Saving</Button>`,
        `<Button variant="outline" invalid>Submit</Button>`,
      ].join("\n")}
    >
      <View class="flex-row items-center gap-3">
        <View class="w-24" />
        {columns.map((c) => (
          <Caption class="w-32">{c}</Caption>
        ))}
      </View>
      {variants.map((v) => (
        <View class="flex-row items-center gap-3">
          <Caption class="w-24">{v}</Caption>
          <View class="w-32 items-start">
            <Button variant={v}>
              <Text>Button</Text>
            </Button>
          </View>
          <View class="w-32 items-start">
            <Button variant={v} disabled>
              <Text>Button</Text>
            </Button>
          </View>
          <View class="w-32 items-start">
            <Button variant={v} loading>
              <Text>Button</Text>
            </Button>
          </View>
          <View class="w-32 items-start">
            <Button variant={v} invalid>
              <Text>Button</Text>
            </Button>
          </View>
        </View>
      ))}
    </Preview>
  );
}

function LoadingDemo() {
  const saving = signal(false);
  const saves = signal(0);
  const save = () => {
    saving.set(true);
    setTimeout(() => {
      saving.set(false);
      saves.update((n) => n + 1);
    }, 2000);
  };
  return (
    <Preview
      code={[
        `const saving = signal(false);`,
        `<Button loading={saving()} icon="plus" onPress={save}>Save</Button>`,
        `<Button variant="outline" iconOnly icon="plus" loading={saving()} accessibilityLabel="Save" />`,
      ].join("\n")}
    >
      <Button loading={saving()} icon="plus" onPress={save}>
        <Text>{saving() ? "Saving…" : "Save"}</Text>
      </Button>
      <Button variant="outline" iconOnly icon="plus" loading={saving()} onPress={save} accessibilityLabel="Save" />
      <Caption>{`Saved ${saves()} times`}</Caption>
    </Preview>
  );
}
