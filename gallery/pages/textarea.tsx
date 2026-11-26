// The Textarea page: growing with its text, sizes beside inputs, states,
// a cap, Return that submits.

import { signal, Text, View } from "nsh";
import { Button, Input, Label, Size, Textarea, type TextareaRef } from "nsh-longbridge";
import { Caption, Page, Preview, Section } from "../ui";

const sizes: Size[] = ["xs", "sm", "md", "lg"];

/** The rows of the state matrix: a caption and the field's props. */
const states: { label: string; props: Record<string, unknown> }[] = [
  { label: "Invalid", props: { invalid: true, defaultValue: "Too short" } },
  { label: "Read-only", props: { readonly: true, defaultValue: "Line one\nLine two" } },
  { label: "Disabled", props: { disabled: true, defaultValue: "Locked" } },
];

export function TextareaPage() {
  return (
    <Page
      title="Textarea"
      description="Multi-line text entry. Input's look, growing with its text from a least height of 64."
    >
      <Section title="Default" description="Press Return a few times: the field grows in the same frame as the line.">
        <Feedback />
      </Section>

      <Section
        title="Sizes"
        description="size changes the text, its line height and the horizontal padding, so a Textarea lines up with an Input of the same size. The least height stays 64."
      >
        <Preview
          class="flex-col items-stretch"
          code={sizes.map((s) => `<Textarea size="${s}" placeholder="Notes" />`).join("\n")}
        >
          {sizes.map((size) => (
            <View class="flex-row items-start gap-3">
              <Caption class="w-8 pt-2">{size}</Caption>
              <View class="w-72 gap-2">
                <Input size={size} placeholder="Title" accessibilityLabel={`Title ${size}`} />
                <Textarea size={size} placeholder="Notes" accessibilityLabel={`Notes ${size}`} />
              </View>
            </View>
          ))}
        </Preview>
      </Section>

      <Section title="States" description="The same states as Input: invalid, read-only, disabled; the focus ring outside.">
        <Preview
          class="flex-col items-stretch gap-3"
          code={[
            `<Textarea invalid value="Too short" />`,
            `<Textarea readonly value={log()} />`,
            `<Textarea disabled value="Locked" />`,
          ].join("\n")}
        >
          {states.map((st) => (
            <View class="flex-row items-start gap-3">
              <Caption class="w-28 pt-2">{st.label}</Caption>
              <View class="w-72">
                <Textarea {...st.props} accessibilityLabel={st.label} />
              </View>
            </View>
          ))}
        </Preview>
      </Section>

      <Section title="A cap and a fixed height" description="max-h-* caps growing, past which it scrolls inside; h-* fixes the height.">
        <Preview
          code={[`<Textarea class="max-h-28" defaultValue={"1\\n2\\n3\\n4\\n5\\n6\\n7"} />`, `<Textarea class="h-32" placeholder="Fixed at 128" />`].join(
            "\n",
          )}
        >
          <View class="w-64">
            <Textarea class="max-h-28" defaultValue={"1\n2\n3\n4\n5\n6\n7"} accessibilityLabel="Capped" />
          </View>
          <View class="w-64">
            <Textarea class="h-32" placeholder="Fixed at 128" accessibilityLabel="Fixed" />
          </View>
        </Preview>
      </Section>

      <Section title="Return submits" description={`submitBehavior="submit": Return sends, Shift+Return breaks the line.`}>
        <Composer />
      </Section>
    </Page>
  );
}

function Feedback() {
  const text = signal("");
  const field = signal<TextareaRef>();
  return (
    <Preview
      code={[
        `const text = signal("");`,
        `const field = signal<TextareaRef>();`,
        `<Label htmlFor={field()}>Your message</Label>`,
        `<Textarea`,
        `  ref={field}`,
        `  value={text()}`,
        `  onChange={({ value }) => text.set(value)}`,
        `  placeholder="Type your message here."`,
        `  maxLength={500}`,
        `  accessibilityLabel="Your message"`,
        `/>`,
      ].join("\n")}
    >
      <View class="w-80 gap-2">
        <Label htmlFor={field()}>
          <Text>Your message</Text>
        </Label>
        <Textarea
          ref={field}
          value={text()}
          onChange={({ value }) => text.set(value)}
          placeholder="Type your message here."
          maxLength={500}
          accessibilityLabel="Your message"
        />
        <Caption>{`${text().length} / 500`}</Caption>
      </View>
    </Preview>
  );
}

function Composer() {
  const draft = signal("");
  const sent = signal<string[]>([]);
  const send = (value: string) => {
    if (value.trim() === "") return;
    sent.update((list) => [...list, value]);
    draft.set("");
  };
  return (
    <Preview
      code={[
        `<Textarea`,
        `  submitBehavior="submit"`,
        `  value={draft()}`,
        `  onChange={({ value }) => draft.set(value)}`,
        `  onSubmit={({ value }) => send(value)}`,
        `/>`,
      ].join("\n")}
    >
      <View class="w-80 gap-2">
        <Textarea
          submitBehavior="submit"
          value={draft()}
          onChange={({ value }) => draft.set(value)}
          onSubmit={({ value }) => send(value)}
          placeholder="Write a message"
          accessibilityLabel="Message"
        />
        <View class="flex-row items-center justify-between">
          <Caption>{`${sent().length} sent`}</Caption>
          <Button size="sm" onPress={() => send(draft())}>
            <Text>Send</Text>
          </Button>
        </View>
      </View>
    </Preview>
  );
}
