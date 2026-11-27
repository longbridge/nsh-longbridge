// The Textarea page: a message composer, sizes beside inputs, states, a
// cap and a fixed height, Return that submits.

import { signal, Text, View } from "nsh";
import { Button, Input, Kbd, KbdGroup, Label, type Size, Textarea, type TextareaRef } from "nsh-longbridge";
import { Demo, Demos, Page, Section } from "../ui/layout";
import { TokensSection } from "../ui/tokens";

const sizes: Size[] = ["xs", "sm", "md", "lg"];

const states: { label: string; note: string; value: string; invalid?: boolean; readonly?: boolean; disabled?: boolean }[] = [
  { label: "Summary", note: "At least 20 characters.", value: "Too short", invalid: true },
  { label: "Build log", note: "Read-only: focusable, selectable", value: "✓ compiled 214 files\n✓ 38 tests passed", readonly: true },
  { label: "Notes", note: "Disabled", value: "Archived with the project.", disabled: true },
];

export function TextareaPage() {
  return (
    <Page
      eyebrow="Components"
      title="Textarea"
      description="Several lines of text: notes, a description, a message. Input's look, growing with its text from a least height of 64."
    >
      <Demo hero>
        <FeedbackForm />
      </Demo>

      <Section
        title="Sizes"
        description="Size changes the text, its line height and the padding, so a Textarea lines up with an Input of the same size. The least height stays 64."
      >
        <Demo class="items-stretch">
          <View class="w-full flex-row flex-wrap gap-6">
            {sizes.map((size) => (
              <View class="flex-1 min-w-40 gap-2">
                <Text class="text-xs text-muted-foreground">{size}</Text>
                <Input size={size} placeholder="Title" accessibilityLabel={`Title, ${size}`} />
                <Textarea size={size} placeholder="Notes" accessibilityLabel={`Notes, ${size}`} />
              </View>
            ))}
          </View>
        </Demo>
      </Section>

      <Section title="States" description="Input's states: invalid, read-only, disabled; the focus ring outside.">
        <Demo class="items-stretch">
          <View class="w-full flex-row flex-wrap gap-6">
            {states.map((s) => (
              <View class="flex-1 min-w-48 gap-2">
                <Label disabled={s.disabled}>
                  <Text>{s.label}</Text>
                </Label>
                <Textarea
                  defaultValue={s.value}
                  invalid={s.invalid}
                  readonly={s.readonly}
                  disabled={s.disabled}
                  accessibilityLabel={s.label}
                />
                <Text class={`text-xs ${s.invalid ? "text-destructive" : "text-muted-foreground"}`}>{s.note}</Text>
              </View>
            ))}
          </View>
        </Demo>
      </Section>

      <Section title="Height" description="A max-h class caps the growth, past which the text scrolls inside; an h class fixes the height.">
        <Demos>
          <Demo label="Capped at 112: it grows, then scrolls">
            <View class="w-full max-w-64">
              <Textarea
                class="max-h-28"
                defaultValue={"Monday: plan\nTuesday: build\nWednesday: review\nThursday: fix\nFriday: ship\nSaturday: rest\nSunday: rest"}
                accessibilityLabel="Capped"
              />
            </View>
          </Demo>
          <Demo label="Fixed at 128">
            <View class="w-full max-w-64">
              <Textarea class="h-32" placeholder="Fixed at 128" accessibilityLabel="Fixed" />
            </View>
          </Demo>
        </Demos>
      </Section>

      <Section title="Return sends" description={`With submitBehavior="submit", Return sends the message, Shift+Return breaks the line.`}>
        <Composer />
      </Section>

      <TokensSection
        prefix="textarea"
        component="Textarea"
        note="Its edge, fill, text, radius and focus are the fields' input-* tokens, shared with Input: change those on the Input page."
      />
    </Page>
  );
}

function FeedbackForm() {
  const text = signal("");
  const field = signal<TextareaRef>();
  const topic = signal("Bug");
  return (
    <View class="w-full max-w-[420px] gap-4">
      <View class="gap-2">
        <Label htmlFor={field()}>
          <Text>Feedback</Text>
        </Label>
        <View class="flex-row flex-wrap gap-1.5">
          {["Bug", "Idea", "Question"].map((t) => (
            <Button size="xs" variant={topic() === t ? "secondary" : "ghost"} onPress={() => topic.set(t)}>
              <Text>{t}</Text>
            </Button>
          ))}
        </View>
        <Textarea
          ref={field}
          value={text()}
          onChange={({ value }) => text.set(value)}
          placeholder="Tell us what happened, and what you expected. Press Return for new lines: the field grows with them."
          maxLength={500}
          accessibilityLabel="Feedback"
        />
      </View>
      <View class="flex-row items-center justify-between">
        <Text class="text-xs tabular-nums text-muted-foreground">{`${text().length} / 500`}</Text>
        <Button size="sm" disabled={text().trim() === ""}>
          <Text>Send feedback</Text>
        </Button>
      </View>
    </View>
  );
}

function Composer() {
  const draft = signal("");
  const sent = signal<string[]>(["Is the release still on for Friday?"]);
  const send = (value: string) => {
    if (value.trim() === "") return;
    sent.update((list) => [...list, value.trim()]);
    draft.set("");
  };
  return (
    <Demo class="items-stretch">
      <View class="w-full max-w-[440px] self-center gap-3">
        <View class="gap-2">
          {sent().map((m, i) => (
            <View class={`max-w-[80%] rounded-lg px-3 py-2 ${i === 0 ? "self-start bg-muted" : "self-end bg-primary"}`}>
              <Text class={`text-sm ${i === 0 ? "text-foreground" : "text-primary-foreground"}`}>{m}</Text>
            </View>
          ))}
        </View>
        <View class="flex-row items-end gap-2">
          <View class="flex-1">
            <Textarea
              submitBehavior="submit"
              value={draft()}
              onChange={({ value }) => draft.set(value)}
              onSubmit={({ value }) => send(value)}
              placeholder="Write a reply"
              accessibilityLabel="Message"
            />
          </View>
          <Button onPress={() => send(draft())} disabled={draft().trim() === ""}>
            <Text>Send</Text>
          </Button>
        </View>
        <View class="flex-row items-center gap-2">
          <Kbd>
            <Text>Return</Text>
          </Kbd>
          <Text class="text-xs text-muted-foreground">sends ·</Text>
          <KbdGroup>
            <Kbd>
              <Text>Shift</Text>
            </Kbd>
            <Kbd>
              <Text>Return</Text>
            </Kbd>
          </KbdGroup>
          <Text class="text-xs text-muted-foreground">breaks the line</Text>
        </View>
      </View>
    </Demo>
  );
}
