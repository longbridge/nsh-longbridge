// The Input page: a form, sizes beside buttons, states, kinds of field and
// the ring against a container's edge.

import { signal, Text, View } from "nsh";
import { Button, Input, type InputRef, Label, type Size } from "nsh-longbridge";
import { Demo, Demos, Page, Row, Section } from "../ui/layout";
import { TokensSection } from "../ui/tokens";

const sizes: Size[] = ["xs", "sm", "md", "lg"];

const states: { label: string; note: string; value?: string; placeholder?: string; invalid?: boolean; readonly?: boolean; disabled?: boolean }[] = [
  { label: "Display name", note: "Placeholder", placeholder: "Ada Lovelace" },
  { label: "Company", note: "With a value", value: "Analytical Engines Ltd" },
  { label: "Email", note: "Invalid", value: "ada@", invalid: true },
  { label: "Account ID", note: "Read-only: focusable, selectable", value: "ACC-20931", readonly: true },
  { label: "Region", note: "Disabled", value: "Europe (Frankfurt)", disabled: true },
];

export function InputPage() {
  return (
    <Page
      eyebrow="Components"
      title="Input"
      description="One line of text: a name, an email, a password, a search term. Four sizes that line up with buttons; invalid, read-only and disabled."
    >
      <Demo hero>
        <ProfileForm />
      </Demo>

      <Section
        title="Sizes"
        description="Height, text and padding change together, so a field and a button of the same size line up. Touch screens get taller fields with 16 point text at md and lg."
      >
        <Demo class="items-stretch">
          {sizes.map((size) => (
            <Row label={size}>
              <View class="flex-1 min-w-40 max-w-80">
                <Input size={size} placeholder="Search projects" accessibilityLabel={`Search, ${size}`} />
              </View>
              <Button size={size} variant="outline">
                <Text>Search</Text>
              </Button>
            </Row>
          ))}
        </Demo>
      </Section>

      <Section
        title="States"
        description="No hover. Focus draws a ring outside the field (on touch screens the edge alone). Invalid turns the edge and the ring destructive; read-only looks as at rest; disabled dims."
      >
        <Demo class="items-stretch">
          <View class="w-full flex-row flex-wrap gap-x-8 gap-y-6">
            {states.map((s) => (
              <StateField {...s} />
            ))}
          </View>
        </Demo>
      </Section>

      <Section title="Kinds of field" description="The masked prop hides what is typed; keyboardType and autoComplete tell the keyboard and autofill what the field holds.">
        <Demos>
          <Demo label="A password, masked">
            <View class="w-full max-w-72 gap-2">
              <Label>
                <Text>Password</Text>
              </Label>
              <Input masked autoComplete="password" defaultValue="correct horse" accessibilityLabel="Password" />
            </View>
          </Demo>
          <Demo label="A search with its button">
            <View class="w-full max-w-80 flex-row gap-2">
              <View class="flex-1">
                <Input placeholder="Search the docs" accessibilityLabel="Search the docs" />
              </View>
              <Button>
                <Text>Search</Text>
              </Button>
            </View>
          </Demo>
        </Demos>
      </Section>

      <Section
        title="Against an edge"
        description="The ring is drawn outside the field and takes no room: a container that clips leaves ring-width (3) of padding, or the ring is cut. Click each field."
      >
        <Demos>
          <Demo label="3 points of padding: the ring fits">
            <View class="w-full max-w-64 overflow-hidden rounded-lg border border-dashed border-border p-[3px]">
              <Input placeholder="Ring fits" accessibilityLabel="Ring fits" />
            </View>
          </Demo>
          <Demo label="No padding: the ring is cut">
            <View class="w-full max-w-64 overflow-hidden rounded-lg border border-dashed border-border">
              <Input placeholder="Ring cut" accessibilityLabel="Ring cut" />
            </View>
          </Demo>
        </Demos>
      </Section>

      <TokensSection
        prefix="input"
        component="Input"
        note="Textarea, Input Group and every later field share these tokens: a change here reaches them too."
        exclude={["input-group"]}
      />
    </Page>
  );
}

function StateField(props: (typeof states)[number]) {
  const field = signal<InputRef>();
  return (
    <View class="flex-1 min-w-56 gap-2">
      <Label htmlFor={field()} disabled={props.disabled}>
        <Text>{props.label}</Text>
      </Label>
      <Input
        ref={field}
        placeholder={props.placeholder}
        defaultValue={props.value}
        invalid={props.invalid}
        readonly={props.readonly}
        disabled={props.disabled}
        accessibilityLabel={props.label}
      />
      <Text class={`text-xs ${props.invalid ? "text-destructive" : "text-muted-foreground"}`}>
        {props.invalid ? "Enter a whole address, with a domain." : props.note}
      </Text>
    </View>
  );
}

/** A profile form: labels pressed focus their fields; the email checks itself as it is typed. */
function ProfileForm() {
  const name = signal<InputRef>();
  const email = signal<InputRef>();
  const site = signal<InputRef>();
  const address = signal("ada@");
  const invalid = () => address().length > 0 && !/^[^@\s]+@[^@\s]+\.[^@\s]+$/.test(address());
  return (
    <View class="w-full max-w-[400px] gap-5">
      <View class="gap-2">
        <Label htmlFor={name()}>
          <Text>Name</Text>
        </Label>
        <Input ref={name} defaultValue="Ada Lovelace" accessibilityLabel="Name" autoComplete="name" />
      </View>
      <View class="gap-2">
        <Label htmlFor={email()}>
          <Text>Email</Text>
        </Label>
        <Input
          ref={email}
          value={address()}
          onChange={(e) => address.set(e.value)}
          invalid={invalid()}
          keyboardType="email-address"
          autoComplete="email"
          autoCapitalize="none"
          accessibilityLabel="Email"
        />
        <Text class={`text-xs ${invalid() ? "text-destructive" : "text-muted-foreground"}`}>
          {invalid() ? "Finish the address: it turns valid as you type." : "We send receipts here."}
        </Text>
      </View>
      <View class="gap-2">
        <Label htmlFor={site()}>
          <Text>Website</Text>
        </Label>
        <Input ref={site} placeholder="https://" keyboardType="url" autoCapitalize="none" accessibilityLabel="Website" />
      </View>
      <View class="flex-row justify-end gap-2 pt-1">
        <Button variant="ghost">
          <Text>Cancel</Text>
        </Button>
        <Button disabled={invalid()}>
          <Text>Save profile</Text>
        </Button>
      </View>
    </View>
  );
}
