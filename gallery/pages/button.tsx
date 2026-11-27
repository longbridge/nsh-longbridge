// The Button page: variants, sizes, icons, states and buttons at work.

import { signal, Text, View } from "nsh";
import {
  Button,
  type ButtonVariant,
  Card,
  CardContent,
  CardDescription,
  CardFooter,
  CardHeader,
  CardTitle,
  Separator,
  type Size,
  Tooltip,
} from "nsh-longbridge";
import { Demo, Demos, Page, Row, Section } from "../ui/layout";
import { TokensSection } from "../ui/tokens";

const variants: { variant: ButtonVariant; label: string }[] = [
  { variant: "default", label: "Default" },
  { variant: "secondary", label: "Secondary" },
  { variant: "outline", label: "Outline" },
  { variant: "ghost", label: "Ghost" },
  { variant: "destructive", label: "Destructive" },
  { variant: "link", label: "Link" },
];
const sizes: Size[] = ["xs", "sm", "md", "lg"];
const tools = [
  { icon: "plus", label: "Add" },
  { icon: "trash-2", label: "Delete" },
] as const;

export function ButtonPage() {
  return (
    <Page
      eyebrow="Components"
      title="Button"
      description="Runs one action when pressed: six variants, four sizes, icons, an icon-only square and a loading state."
    >
      <Demo hero row>
        {variants.map((v) => (
          <Button variant={v.variant}>
            <Text>{v.label}</Text>
          </Button>
        ))}
      </Demo>

      <Section
        title="Sizes"
        description="Height, type, icon and padding change together. Touch screens get taller buttons and a 44 point press area."
      >
        <Demo class="items-stretch">
          {sizes.map((size) => (
            <Row label={size}>
              <Button size={size}>
                <Text>Continue</Text>
              </Button>
              <Button size={size} variant="outline" icon="plus">
                <Text>New file</Text>
              </Button>
              <Button size={size} variant="secondary" iconEnd="chevron-down">
                <Text>Options</Text>
              </Button>
              <Button size={size} variant="outline" iconOnly icon="plus" accessibilityLabel="Add" />
            </Row>
          ))}
        </Demo>
      </Section>

      <Section title="Icons" description="An icon before or after the label, or alone in a square that names itself to screen readers.">
        <Demos>
          <Demo label="A toolbar of icon buttons, with their names on hover">
            <View class="flex-row items-center gap-1 rounded-lg border border-border p-1">
              {tools.map((t) => (
                <Button
                  variant="ghost"
                  size="sm"
                  iconOnly
                  icon={t.icon}
                  accessibilityLabel={t.label}
                  tooltip={
                    <Tooltip>
                      <Text>{t.label}</Text>
                    </Tooltip>
                  }
                />
              ))}
              <Separator orientation="vertical" class="mx-1 h-5" />
              <Button variant="ghost" size="sm" iconEnd="chevron-down">
                <Text>Sort</Text>
              </Button>
            </View>
          </Demo>
          <Demo row label="Leading and trailing icons">
            <Button variant="outline" icon="plus">
              <Text>New file</Text>
            </Button>
            <Button iconEnd="arrow-right">
              <Text>Continue</Text>
            </Button>
          </Demo>
        </Demos>
      </Section>

      <Section
        title="States"
        description="Disabled dims and takes no presses. Loading shows a spinner, takes no presses and does not dim. Invalid marks the edge. Tab shows the focus ring."
      >
        <Demos>
          <Demo row label="Disabled until something changes">
            <Button variant="outline">
              <Text>Discard</Text>
            </Button>
            <Button disabled>
              <Text>Save changes</Text>
            </Button>
          </Demo>
          <SaveDemo />
          <Demo row label="Invalid, beside the field it belongs to">
            <Button variant="outline" invalid>
              <Text>Choose a file</Text>
            </Button>
            <Text class="text-sm text-destructive">A file is required.</Text>
          </Demo>
        </Demos>
      </Section>

      <Section title="At work" description="A destructive action and its way out, at the end of a card.">
        <Demo>
          <Card class="w-full max-w-[420px]">
            <CardHeader>
              <CardTitle>
                <Text>Delete project</Text>
              </CardTitle>
              <CardDescription>
                <Text>The project, its deployments and its domains are removed. This cannot be undone.</Text>
              </CardDescription>
            </CardHeader>
            <CardContent>
              <View class="rounded-md bg-muted px-3 py-2">
                <Text class="text-sm text-muted-foreground">acme-web · 14 deployments · 2 domains</Text>
              </View>
            </CardContent>
            <CardFooter class="justify-end gap-2">
              <Button variant="outline">
                <Text>Cancel</Text>
              </Button>
              <Button variant="destructive" icon="trash-2">
                <Text>Delete</Text>
              </Button>
            </CardFooter>
          </Card>
        </Demo>
      </Section>

      <TokensSection prefix="button" component="Button" />
    </Page>
  );
}

function SaveDemo() {
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
    <Demo row label={saves() === 0 ? "Loading: press it" : `Saved ${saves()} ${saves() === 1 ? "time" : "times"}`}>
      <Button loading={saving()} onPress={save}>
        <Text>{saving() ? "Saving…" : "Save"}</Text>
      </Button>
      <Button variant="outline" iconOnly icon="plus" loading={saving()} onPress={save} accessibilityLabel="Save" />
    </Demo>
  );
}
