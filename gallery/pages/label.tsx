// The Label page: naming a field, with other elements, disabled, and the
// line height of 1 with CJK text and descenders.

import { signal, Text, View } from "nsh";
import { Icon, Input, type InputRef, Label, Textarea, type TextareaRef } from "nsh-longbridge";
import { Caption, Page, Preview, Section } from "../ui";

/** Labels whose line height of 1 is checked: descenders, CJK text. */
const lineHeightSamples = ["Typography gjpqy", "显示名称", "表示名 · 표시 이름"];

export function LabelPage() {
  const name = signal<InputRef>();
  const bio = signal<TextareaRef>();
  const locked = signal(true);
  const lockedField = signal<InputRef>();
  return (
    <Page
      title="Label"
      description="The text naming a field. Pressing it moves the focus into the field htmlFor names. Medium 14pt text, a line height of 1, not selectable."
    >
      <Section
        title="With a field"
        description="Press a label: its field takes the focus. Screen readers do not get the label as the field's name, so the field keeps its accessibilityLabel."
      >
        <Preview
          code={[
            `const name = signal<InputRef>();`,
            `<Label htmlFor={name()}>Display name</Label>`,
            `<Input ref={name} placeholder="Ada" accessibilityLabel="Display name" />`,
          ].join("\n")}
          class="items-start"
        >
          <View class="w-72 gap-2">
            <Label htmlFor={name()}>
              <Text>Display name</Text>
            </Label>
            <Input ref={name} placeholder="Ada" accessibilityLabel="Display name" />
          </View>
          <View class="w-72 gap-2">
            <Label htmlFor={bio()}>
              <Text>Bio</Text>
            </Label>
            <Textarea ref={bio} placeholder="A few words" accessibilityLabel="Bio" />
          </View>
        </Preview>
      </Section>

      <Section title="With other elements" description="Children sit in a row, 8 apart: an icon, a count.">
        <Preview code={[`<Label>`, `  <Icon name="plus" />`, `  New project`, `</Label>`].join("\n")}>
          <Label>
            <Icon name="plus" />
            <Text>New project</Text>
          </Label>
          <Label>
            <Text>API key</Text>
            <Caption>(optional)</Caption>
          </Label>
        </Preview>
      </Section>

      <Section title="Disabled" description="Pass the field's disabled: the label dims and a press focuses nothing.">
        <Preview
          code={[
            `<Label htmlFor={field()} disabled={locked()}>Account ID</Label>`,
            `<Input ref={field} disabled={locked()} value="ACC-20931" />`,
          ].join("\n")}
        >
          <View class="w-72 gap-2">
            <Label htmlFor={lockedField()} disabled={locked()}>
              <Text>Account ID</Text>
            </Label>
            <Input ref={lockedField} disabled={locked()} defaultValue="ACC-20931" accessibilityLabel="Account ID" />
            <Caption>{locked() ? "Locked" : "Unlocked"}</Caption>
          </View>
        </Preview>
      </Section>

      <Section
        title="Line height"
        description="shadcn's leading-none: the line is as tall as the text. Checked with descenders and CJK text."
      >
        <Preview code={lineHeightSamples.map((t) => `<Label>${t}</Label>`).join("\n")}>
          {lineHeightSamples.map((t) => (
            <Label>
              <Text>{t}</Text>
            </Label>
          ))}
        </Preview>
      </Section>
    </Page>
  );
}
