// The Field page: a payment form, horizontal settings, a responsive group,
// validation, choice cards, a disabled set.

import { signal, Text, View } from "nsh";
import {
  Button,
  Checkbox,
  Field,
  FieldContent,
  FieldDescription,
  FieldError,
  FieldGroup,
  FieldLabel,
  FieldLegend,
  FieldSeparator,
  FieldSet,
  FieldTitle,
  Input,
  RadioGroup,
  RadioGroupItem,
  Select,
  type SelectItem,
  Slider,
  Switch,
  Textarea,
  ToggleGroup,
  ToggleGroupItem,
} from "nsh-longbridge";
import { Demo, Demos, Page, Section } from "../ui/layout";
import { TokensSection } from "../ui/tokens";

const months: SelectItem[] = Array.from({ length: 12 }, (_, i) => {
  const m = String(i + 1).padStart(2, "0");
  return { value: m, title: m };
});

const years: SelectItem[] = Array.from({ length: 8 }, (_, i) => {
  const y = String(2026 + i);
  return { value: y, title: y };
});

const plans = [
  { value: "kubernetes", title: "Kubernetes", description: "Run GPU workloads on a K8s cluster." },
  { value: "vm", title: "Virtual machine", description: "Access a VM configured cluster to run workloads." },
];

const rules = [
  { test: (p: string) => p.length >= 8, message: "At least 8 characters." },
  { test: (p: string) => /[0-9]/.test(p), message: "At least one digit." },
  { test: (p: string) => /[A-Z]/.test(p), message: "At least one capital letter." },
];

export function FieldPage() {
  return (
    <Page
      eyebrow="Components"
      title="Field"
      description="The layout of a form: a control with its label, its help and its error, in groups and titled sets. A Field hands its invalid and disabled state, and its help or error for screen readers, to the control inside it."
    >
      <Demo hero>
        <PaymentDemo />
      </Demo>

      <Section
        title="Horizontal"
        description="The label beside the control. With a FieldContent the row aligns to the top and a switch or a checkbox centers on its first line; a press on the label presses the control."
      >
        <Demo>
          <SettingsDemo />
        </Demo>
      </Section>

      <Section
        title="Responsive"
        description="Vertical while its FieldGroup is narrower than field-responsive-width (448), horizontal from it. Change the group's width."
      >
        <ResponsiveDemo />
      </Section>

      <Section
        title="Validation"
        description="Invalid turns the label and the control destructive. FieldError shows its children, or a list of messages from a validation library (repeats dropped), and nothing without either; it describes the control while it shows."
      >
        <Demos>
          <Demo>
            <EmailDemo />
          </Demo>
          <Demo>
            <PasswordDemo />
          </Demo>
        </Demos>
      </Section>

      <Section
        title="Choice cards"
        description="A FieldLabel around a Field is a card the whole of which is the option: a press anywhere checks the control, and the card takes the checked edge and fill."
      >
        <Demos>
          <Demo>
            <PlanDemo />
          </Demo>
          <Demo>
            <BackupDemo />
          </Demo>
        </Demos>
      </Section>

      <Section title="Disabled" description="A disabled FieldSet disables every field in it: the labels dim and the controls take no input.">
        <Demo>
          <DisabledDemo />
        </Demo>
      </Section>

      <TokensSection prefix="field" component="Field" note="Labels and titles read Label's tokens; the controls keep their own." />
    </Page>
  );
}

function PaymentDemo() {
  const name = signal("");
  const number = signal("");
  const same = signal(true);
  const tried = signal(false);
  const nameError = () => (tried() && name().trim() === "" ? "Enter the name on the card." : null);
  const numberError = () =>
    tried() && number().replace(/\s/g, "").length !== 16 ? "Enter the 16 digits of the card number." : null;
  return (
    <View class="w-full max-w-[420px] rounded-xl border border-border bg-card p-6">
      <FieldGroup>
        <FieldSet accessibilityLabel="Payment method">
          <FieldLegend>
            <Text>Payment method</Text>
          </FieldLegend>
          <FieldDescription>
            <Text>All transactions are secure and encrypted.</Text>
          </FieldDescription>
          <FieldGroup>
            <Field invalid={nameError() !== null}>
              <FieldLabel>
                <Text>Name on card</Text>
              </FieldLabel>
              <Input value={name()} onChange={(e) => name.set(e.value)} placeholder="Ada Lovelace" accessibilityLabel="Name on card" />
              <FieldError>{nameError() === null ? null : <Text>{nameError()!}</Text>}</FieldError>
            </Field>
            <Field invalid={numberError() !== null}>
              <FieldLabel>
                <Text>Card number</Text>
              </FieldLabel>
              <Input
                value={number()}
                onChange={(e) => number.set(e.value)}
                placeholder="1234 5678 9012 3456"
                accessibilityLabel="Card number"
              />
              <FieldDescription>
                <Text>Enter your 16-digit card number.</Text>
              </FieldDescription>
              <FieldError>{numberError() === null ? null : <Text>{numberError()!}</Text>}</FieldError>
            </Field>
            <View class="flex-row gap-4">
              <Field class="flex-1">
                <FieldLabel>
                  <Text>Month</Text>
                </FieldLabel>
                <Select class="w-full" items={months} placeholder="MM" accessibilityLabel="Expiry month" />
              </Field>
              <Field class="flex-1">
                <FieldLabel>
                  <Text>Year</Text>
                </FieldLabel>
                <Select class="w-full" items={years} placeholder="YYYY" accessibilityLabel="Expiry year" />
              </Field>
              <Field class="flex-1">
                <FieldLabel>
                  <Text>CVV</Text>
                </FieldLabel>
                <Input placeholder="123" accessibilityLabel="CVV" />
              </Field>
            </View>
          </FieldGroup>
        </FieldSet>
        <FieldSeparator />
        <FieldSet accessibilityLabel="Billing address">
          <FieldLegend>
            <Text>Billing address</Text>
          </FieldLegend>
          <FieldDescription>
            <Text>The billing address associated with your payment method.</Text>
          </FieldDescription>
          <Field orientation="horizontal">
            <Checkbox checked={same()} onChange={(e) => same.set(e.checked)} accessibilityLabel="Same as shipping address" />
            <FieldLabel>
              <Text>Same as shipping address</Text>
            </FieldLabel>
          </Field>
        </FieldSet>
        <Field>
          <FieldLabel>
            <Text>Comments</Text>
          </FieldLabel>
          <Textarea placeholder="Add any additional comments" accessibilityLabel="Comments" />
        </Field>
        <Field orientation="horizontal">
          <Button onPress={() => tried.set(true)}>
            <Text>Submit</Text>
          </Button>
          <Button
            variant="outline"
            onPress={() => {
              tried.set(false);
              name.set("");
              number.set("");
            }}
          >
            <Text>Cancel</Text>
          </Button>
        </Field>
      </FieldGroup>
    </View>
  );
}

function SettingsDemo() {
  const share = signal(true);
  const updates = signal(false);
  const volume = signal(60);
  return (
    <View class="w-full max-w-[480px]">
      <FieldGroup>
        <Field orientation="horizontal">
          <FieldContent>
            <FieldLabel>
              <Text>Share across devices</Text>
            </FieldLabel>
            <FieldDescription>
              <Text>Focus is shared across devices, and turns off when you leave the app.</Text>
            </FieldDescription>
          </FieldContent>
          <Switch checked={share()} onChange={(e) => share.set(e.checked)} accessibilityLabel="Share across devices" />
        </Field>
        <Field orientation="horizontal">
          <Checkbox checked={updates()} onChange={(e) => updates.set(e.checked)} accessibilityLabel="Email me about updates" />
          <FieldContent>
            <FieldLabel>
              <Text>Email me about updates</Text>
            </FieldLabel>
            <FieldDescription>
              <Text>A short note when something you follow changes. No more than one a day.</Text>
            </FieldDescription>
          </FieldContent>
        </Field>
        <Field orientation="horizontal">
          <FieldContent>
            <FieldLabel>
              <Text>Alert volume</Text>
            </FieldLabel>
            <FieldDescription>
              <Text>How loud a new message plays.</Text>
            </FieldDescription>
          </FieldContent>
          <Slider value={volume()} onChange={(e) => volume.set(e.value as number)} accessibilityLabel="Alert volume" />
        </Field>
      </FieldGroup>
    </View>
  );
}

function ResponsiveDemo() {
  const width = signal("wide");
  return (
    <Demo class="items-stretch gap-6">
      <ToggleGroup
        variant="outline"
        size="sm"
        class="self-center"
        value={[width()]}
        onChange={(e) => e.value[0] && width.set(e.value[0])}
        accessibilityLabel="Group width"
      >
        <ToggleGroupItem value="narrow">
          <Text>320 wide</Text>
        </ToggleGroupItem>
        <ToggleGroupItem value="wide">
          <Text>560 wide</Text>
        </ToggleGroupItem>
      </ToggleGroup>
      <View class={`w-full self-center ${width() === "wide" ? "max-w-[560px]" : "max-w-[320px]"}`}>
        <FieldGroup>
          <Field orientation="responsive">
            <FieldContent>
              <FieldLabel>
                <Text>Name</Text>
              </FieldLabel>
              <FieldDescription>
                <Text>Provide your full name for identification.</Text>
              </FieldDescription>
            </FieldContent>
            <Input placeholder="Ada Lovelace" accessibilityLabel="Name" />
          </Field>
          <Field orientation="responsive">
            <FieldContent>
              <FieldLabel>
                <Text>Message</Text>
              </FieldLabel>
              <FieldDescription>
                <Text>You can write your message here. Keep it short.</Text>
              </FieldDescription>
            </FieldContent>
            <Textarea placeholder="Hello, world!" accessibilityLabel="Message" />
          </Field>
        </FieldGroup>
      </View>
    </Demo>
  );
}

function EmailDemo() {
  const email = signal("ada@");
  const valid = () => /^[^@\s]+@[^@\s]+\.[^@\s]+$/.test(email());
  return (
    <View class="w-full max-w-[320px]">
      <Field invalid={!valid()}>
        <FieldLabel>
          <Text>Email</Text>
        </FieldLabel>
        <Input value={email()} onChange={(e) => email.set(e.value)} accessibilityLabel="Email" />
        <FieldDescription>
          <Text>Where receipts are sent.</Text>
        </FieldDescription>
        <FieldError>{valid() ? null : <Text>Enter an email address like name@example.com.</Text>}</FieldError>
      </Field>
    </View>
  );
}

function PasswordDemo() {
  const password = signal("secret");
  const issues = () => rules.filter((r) => !r.test(password())).map((r) => ({ message: r.message }));
  return (
    <View class="w-full max-w-[320px]">
      <Field invalid={issues().length > 0}>
        <FieldLabel>
          <Text>Password</Text>
        </FieldLabel>
        <Input value={password()} onChange={(e) => password.set(e.value)} accessibilityLabel="Password" />
        <FieldError errors={issues()} />
        <If when={issues().length === 0}>
          <FieldDescription>
            <Text>Strong enough.</Text>
          </FieldDescription>
        </If>
      </Field>
    </View>
  );
}

function PlanDemo() {
  const plan = signal("kubernetes");
  return (
    <View class="w-full max-w-[380px]">
      <FieldSet accessibilityLabel="Compute environment">
        <FieldLegend variant="label">
          <Text>Compute environment</Text>
        </FieldLegend>
        <FieldDescription>
          <Text>Select the compute environment for your cluster.</Text>
        </FieldDescription>
        <RadioGroup value={plan()} onChange={(e) => plan.set(e.value)} accessibilityLabel="Compute environment" class="gap-3">
          {plans.map((p) => (
            <FieldLabel>
              <Field orientation="horizontal">
                <FieldContent>
                  <FieldTitle>
                    <Text>{p.title}</Text>
                  </FieldTitle>
                  <FieldDescription>
                    <Text>{p.description}</Text>
                  </FieldDescription>
                </FieldContent>
                <RadioGroupItem value={p.value} accessibilityLabel={p.title} />
              </Field>
            </FieldLabel>
          ))}
        </RadioGroup>
      </FieldSet>
    </View>
  );
}

function BackupDemo() {
  const backup = signal(true);
  const alerts = signal(false);
  return (
    <View class="w-full max-w-[380px] gap-3">
      <FieldLabel>
        <Field orientation="horizontal">
          <Checkbox checked={backup()} onChange={(e) => backup.set(e.checked)} accessibilityLabel="Daily backups" />
          <FieldContent>
            <FieldTitle>
              <Text>Daily backups</Text>
            </FieldTitle>
            <FieldDescription>
              <Text>Kept for 30 days, restored in one click.</Text>
            </FieldDescription>
          </FieldContent>
        </Field>
      </FieldLabel>
      <FieldLabel>
        <Field orientation="horizontal">
          <FieldContent>
            <FieldTitle>
              <Text>Usage alerts</Text>
            </FieldTitle>
            <FieldDescription>
              <Text>A notice when a cluster passes 80% of its budget.</Text>
            </FieldDescription>
          </FieldContent>
          <Switch checked={alerts()} onChange={(e) => alerts.set(e.checked)} accessibilityLabel="Usage alerts" />
        </Field>
      </FieldLabel>
    </View>
  );
}

function DisabledDemo() {
  return (
    <View class="w-full max-w-[420px]">
      <FieldSet disabled accessibilityLabel="Organization">
        <FieldLegend>
          <Text>Organization</Text>
        </FieldLegend>
        <FieldDescription>
          <Text>Only an owner can change these.</Text>
        </FieldDescription>
        <FieldGroup>
          <Field>
            <FieldLabel>
              <Text>Display name</Text>
            </FieldLabel>
            <Input value="Acme Inc." accessibilityLabel="Display name" />
          </Field>
          <Field orientation="horizontal">
            <FieldContent>
              <FieldLabel>
                <Text>Require two-factor sign-in</Text>
              </FieldLabel>
              <FieldDescription>
                <Text>Every member signs in with a second factor.</Text>
              </FieldDescription>
            </FieldContent>
            <Switch checked accessibilityLabel="Require two-factor sign-in" />
          </Field>
        </FieldGroup>
      </FieldSet>
    </View>
  );
}
