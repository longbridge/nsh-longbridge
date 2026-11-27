// The Label page: naming fields, with other elements, disabled, and the
// line height of 1 with CJK text and descenders.

import { signal, Text, View } from "nsh";
import { Badge, Icon, Input, type InputRef, Label, Switch, Textarea, type TextareaRef } from "nsh-longbridge";
import { Demo, Demos, Page, Section } from "../ui/layout";
import { TokensSection } from "../ui/tokens";

/** Labels whose line height of 1 is checked: descenders, CJK text. */
const lineHeightSamples = ["Typography gjpqy", "显示名称", "表示名 · 표시 이름"];

export function LabelPage() {
  return (
    <Page
      eyebrow="Components"
      title="Label"
      description="The name of a field. Pressing it moves the focus into the field htmlFor names. Medium text, a line height of 1, not selectable."
    >
      <Demo hero>
        <ProjectForm />
      </Demo>

      <Section title="With other elements" description="Children sit in a row, 8 apart: an icon, a hint, a badge.">
        <Demo row class="gap-10">
          <Label>
            <Icon name="plus" />
            <Text>New project</Text>
          </Label>
          <Label>
            <Text>API key</Text>
            <Text class="font-normal text-muted-foreground">(optional)</Text>
          </Label>
          <Label>
            <Text>Webhooks</Text>
            <Badge variant="secondary">
              <Text>Beta</Text>
            </Badge>
          </Label>
        </Demo>
      </Section>

      <Section title="Disabled" description="Give the label its field's disabled: the two dim together, and a press focuses nothing.">
        <LockedDemo />
      </Section>

      <Section title="Line height" description="shadcn's leading-none: the line, shaded here, is exactly as tall as the text, checked with descenders and CJK text.">
        <Demo row class="gap-10">
          {lineHeightSamples.map((t) => (
            <View class="bg-muted">
              <Label>
                <Text>{t}</Text>
              </Label>
            </View>
          ))}
        </Demo>
      </Section>

      <TokensSection prefix="label" component="Label" />
    </Page>
  );
}

/** Press a label: its field takes the focus. */
function ProjectForm() {
  const name = signal<InputRef>();
  const slug = signal<InputRef>();
  const about = signal<TextareaRef>();
  return (
    <View class="w-full max-w-[400px] gap-5">
      <View class="gap-2">
        <Label htmlFor={name()}>
          <Text>Project name</Text>
        </Label>
        <Input ref={name} placeholder="Acme Web" accessibilityLabel="Project name" />
      </View>
      <View class="gap-2">
        <Label htmlFor={slug()}>
          <Text>Address</Text>
          <Text class="font-normal text-muted-foreground">acme.app/</Text>
        </Label>
        <Input ref={slug} placeholder="acme-web" autoCapitalize="none" accessibilityLabel="Address" />
      </View>
      <View class="gap-2">
        <Label htmlFor={about()}>
          <Text>Description</Text>
        </Label>
        <Textarea ref={about} placeholder="What the project is for" accessibilityLabel="Description" />
      </View>
      <Text class="text-xs text-muted-foreground">Press a label: its field takes the focus.</Text>
    </View>
  );
}

function LockedDemo() {
  const locked = signal(true);
  const field = signal<InputRef>();
  return (
    <Demos>
      <Demo label={locked() ? "Locked: the label and the field dim" : "Unlocked"}>
        <View class="w-full max-w-72 gap-2">
          <Label htmlFor={field()} disabled={locked()}>
            <Text>Account ID</Text>
          </Label>
          <Input ref={field} disabled={locked()} defaultValue="ACC-20931" accessibilityLabel="Account ID" />
        </View>
        <Switch checked={!locked()} onChange={(e) => locked.set(!e.checked)} size="sm">
          <Text>Allow editing</Text>
        </Switch>
      </Demo>
    </Demos>
  );
}
