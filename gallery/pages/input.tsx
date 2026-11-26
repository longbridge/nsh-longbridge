// The Input page: sizes, states, a controlled field with its label, and
// the ring against a container's edge.

import { signal, Text, View } from "nsh";
import { Button, Input, type InputRef, Label, Size } from "nsh-longbridge";
import { Caption, Page, Preview, Section } from "../ui";

const sizes: Size[] = ["xs", "sm", "md", "lg"];

/** The rows of the state matrix: a caption and the field's props. */
const states: { label: string; props: Record<string, unknown> }[] = [
  { label: "Placeholder", props: { placeholder: "Placeholder" } },
  { label: "Value", props: { defaultValue: "Ada Lovelace" } },
  { label: "Invalid", props: { invalid: true, defaultValue: "ada@" } },
  { label: "Read-only", props: { readonly: true, defaultValue: "ACC-20931" } },
  { label: "Disabled", props: { disabled: true, defaultValue: "Read from server" } },
];

export function InputPage() {
  return (
    <Page
      title="Input"
      description="Single-line text entry. shadcn/ui's input on NSH's native text field: four sizes, invalid, read-only and disabled."
    >
      <Section title="Default" description="Give every field an accessibilityLabel, even beside a visible Label.">
        <Preview code={`<Input placeholder="Email" accessibilityLabel="Email" />`}>
          <View class="w-72">
            <Input placeholder="Email" accessibilityLabel="Email" />
          </View>
        </Preview>
      </Section>

      <Section title="With a label" description="Pressing the label focuses the field. A label above its field takes the ref through a signal.">
        <EmailField />
      </Section>

      <Section
        title="Sizes"
        description="Height, text and padding change together; an Input and a Button of the same size line up. Touch screens get taller fields with 16pt text at md and lg."
      >
        <Preview
          class="flex-col items-stretch"
          code={sizes.map((s) => `<Input size="${s}" placeholder="Search" />`).join("\n")}
        >
          {sizes.map((size) => (
            <View class="flex-row items-center gap-3">
              <Caption class="w-8">{size}</Caption>
              <View class="w-72">
                <Input size={size} placeholder="Search" accessibilityLabel={`Search ${size}`} />
              </View>
              <Button size={size} variant="outline">
                <Text>Search</Text>
              </Button>
            </View>
          ))}
        </Preview>
      </Section>

      <Section
        title="States"
        description="No hover state. Click a field: focus draws a ring outside the field (on touch screens the edge alone). Invalid turns the edge and the ring destructive; read-only looks as at rest; disabled dims."
      >
        <StateMatrix />
      </Section>

      <Section title="Password" description="masked hides what is typed; autoComplete tells autofill what the field holds.">
        <Preview code={`<Input masked autoComplete="password" placeholder="Password" accessibilityLabel="Password" />`}>
          <View class="w-72">
            <Input masked autoComplete="password" placeholder="Password" accessibilityLabel="Password" />
          </View>
        </Preview>
      </Section>

      <Section
        title="Against an edge"
        description="The ring is drawn outside the field and takes no layout: a clipping container leaves ring-width (3) of padding, or the ring is cut."
      >
        <Preview
          code={[`<View class="overflow-hidden p-[3px]">`, `  <Input placeholder="Ring fits" />`, `</View>`].join("\n")}
        >
          <View class="w-72 gap-2">
            <Caption>p-[3px]</Caption>
            <View class="overflow-hidden p-[3px]">
              <Input placeholder="Ring fits" accessibilityLabel="Ring fits" />
            </View>
          </View>
          <View class="w-72 gap-2">
            <Caption>No padding</Caption>
            <View class="overflow-hidden">
              <Input placeholder="Ring clipped" accessibilityLabel="Ring clipped" />
            </View>
          </View>
        </Preview>
      </Section>
    </Page>
  );
}

function EmailField() {
  const email = signal("");
  const invalid = () => email().length > 0 && !email().includes("@");
  const field = signal<InputRef>();
  return (
    <Preview
      code={[
        `const email = signal("");`,
        `const field = signal<InputRef>();`,
        `<Label htmlFor={field()}>Email</Label>`,
        `<Input`,
        `  ref={field}`,
        `  value={email()}`,
        `  onChange={({ value }) => email.set(value)}`,
        `  placeholder="name@example.com"`,
        `  keyboardType="email-address"`,
        `  autoComplete="email"`,
        `  autoCapitalize="none"`,
        `  invalid={email().length > 0 && !email().includes("@")}`,
        `  accessibilityLabel="Email"`,
        `/>`,
      ].join("\n")}
    >
      <View class="w-72 gap-2">
        <Label htmlFor={field()}>
          <Text>Email</Text>
        </Label>
        <Input
          ref={field}
          value={email()}
          onChange={({ value }) => email.set(value)}
          placeholder="name@example.com"
          keyboardType="email-address"
          autoComplete="email"
          autoCapitalize="none"
          invalid={invalid()}
          accessibilityLabel="Email"
        />
        <Caption>{invalid() ? "Enter an address with an @." : "Type an address without an @ to see invalid."}</Caption>
      </View>
    </Preview>
  );
}

function StateMatrix() {
  return (
    <Preview
      class="flex-col items-stretch gap-3"
      code={states.map((st) => `<Input ${props(st.props)} />`).join("\n")}
    >
      {states.map((st) => (
        <View class="flex-row items-center gap-3">
          <Caption class="w-28">{st.label}</Caption>
          <View class="w-72">
            <Input {...st.props} accessibilityLabel={st.label} />
          </View>
        </View>
      ))}
    </Preview>
  );
}

/** Props as JSX attributes, for the code under a demo. */
function props(p: Record<string, unknown>) {
  return Object.entries(p)
    .map(([k, v]) => (v === true ? k : typeof v === "string" ? `${k}="${v}"` : `${k}={${JSON.stringify(v)}}`))
    .join(" ");
}
