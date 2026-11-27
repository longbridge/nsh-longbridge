// The InputGroup page: a sign-up form with a password's eye, search fields,
// words before and after a field, an action and a check on its edge, a
// message box with a toolbar, sizes and states.

import { Clipboard, keymap, signal, Text, View } from "nsh";
import {
  Button,
  Field,
  FieldDescription,
  FieldError,
  FieldLabel,
  Icon,
  Input,
  InputGroup,
  InputGroupAddon,
  InputGroupButton,
  InputGroupPasswordToggle,
  Kbd,
  type Size,
  Spinner,
  Textarea,
} from "nsh-longbridge";
import { Demo, Demos, Page, Row, Section } from "../ui/layout";
import { TokensSection } from "../ui/tokens";

const sizes: Size[] = ["xs", "sm", "md", "lg"];

export function InputGroupPage() {
  return (
    <Page
      eyebrow="Components"
      title="Input Group"
      description="Icons, words, key hints, spinners and buttons around a field, in one frame. The frame shows the field's focus, invalid and disabled states; a press on an addon edits the field."
    >
      <Demo hero>
        <SignUp />
      </Demo>

      <Section
        title="Password"
        description="The eye shows the password and hides it again; screen readers hear one name, Show password, and whether it is pressed. Clicking it leaves the caret in the field."
      >
        <Demos>
          <Demo label="Hidden, the eye at the end">
            <View class="w-full max-w-72">
              <InputGroup>
                <Input masked defaultValue="correct horse" autoComplete="password" accessibilityLabel="Password" />
                <InputGroupAddon align="inlineEnd">
                  <InputGroupPasswordToggle />
                </InputGroupAddon>
              </InputGroup>
            </View>
          </Demo>
          <Demo label="Shown (controlled)">
            <ShownPassword />
          </Demo>
        </Demos>
      </Section>

      <Section
        title="Search"
        description="A leading icon, a clear button while there is text, the shortcut that focuses the field."
      >
        <Demos>
          <Demo label="Icon, clear, key hint">
            <SearchField />
          </Demo>
          <Demo label="With a search button">
            <View class="w-full max-w-80">
              <InputGroup>
                <InputGroupAddon>
                  <Icon name="search" />
                </InputGroupAddon>
                <Input placeholder="Search the docs" accessibilityLabel="Search the docs" />
                <InputGroupAddon align="inlineEnd">
                  <InputGroupButton variant="secondary">
                    <Text>Search</Text>
                  </InputGroupButton>
                </InputGroupAddon>
              </InputGroup>
            </View>
          </Demo>
        </Demos>
      </Section>

      <Section
        title="Words before and after"
        description="Fixed words the value does not include: a scheme, a domain, a currency and its unit. The field's accessibility label says them too."
      >
        <Demos>
          <Demo label="A URL">
            <View class="w-full max-w-80">
              <InputGroup>
                <InputGroupAddon>
                  <Text>https://</Text>
                </InputGroupAddon>
                <Input placeholder="example.com" keyboardType="url" autoCapitalize="none" accessibilityLabel="Website address" />
              </InputGroup>
            </View>
          </Demo>
          <Demo label="An email at a fixed domain">
            <View class="w-full max-w-80">
              <InputGroup>
                <Input placeholder="ada" autoCapitalize="none" accessibilityLabel="Email name, at example.com" />
                <InputGroupAddon align="inlineEnd">
                  <Text>@example.com</Text>
                </InputGroupAddon>
              </InputGroup>
            </View>
          </Demo>
          <Demo label="An amount with its currency">
            <View class="w-full max-w-64">
              <InputGroup>
                <InputGroupAddon>
                  <Text>$</Text>
                </InputGroupAddon>
                <Input placeholder="0.00" keyboardType="decimal-pad" accessibilityLabel="Amount in US dollars" />
                <InputGroupAddon align="inlineEnd">
                  <Text>USD</Text>
                </InputGroupAddon>
              </InputGroup>
            </View>
          </Demo>
          <Demo label="A price range">
            <View class="w-full max-w-80 flex-row items-center gap-2">
              <View class="flex-1">
                <InputGroup>
                  <InputGroupAddon>
                    <Text>$</Text>
                  </InputGroupAddon>
                  <Input placeholder="Min" keyboardType="decimal-pad" accessibilityLabel="Lowest price in dollars" />
                </InputGroup>
              </View>
              <Text class="text-sm text-muted-foreground">to</Text>
              <View class="flex-1">
                <InputGroup>
                  <InputGroupAddon>
                    <Text>$</Text>
                  </InputGroupAddon>
                  <Input placeholder="Max" keyboardType="decimal-pad" accessibilityLabel="Highest price in dollars" />
                </InputGroup>
              </View>
            </View>
          </Demo>
        </Demos>
      </Section>

      <Section
        title="Actions and checks"
        description="A button on the field's edge, sized to the field and concentric with it; a spinner while the field checks itself, a mark when it is done."
      >
        <Demos>
          <Demo label="Copy">
            <CopyField />
          </Demo>
          <Demo label="Checking a user name">
            <UserName />
          </Demo>
        </Demos>
      </Section>

      <Section
        title="Message box"
        description="A block addon makes a column: a toolbar under a Textarea that grows with its text, or a header over it."
      >
        <Demos>
          <Demo label="A toolbar under it">
            <MessageBox />
          </Demo>
          <Demo label="A header over it">
            <View class="w-full max-w-96">
              <InputGroup>
                <InputGroupAddon align="blockStart">
                  <Icon name="info" />
                  <Text class="flex-1">Release notes</Text>
                  <Text>Markdown</Text>
                </InputGroupAddon>
                <Textarea placeholder="What changed in this version" accessibilityLabel="Release notes" />
              </InputGroup>
            </View>
          </Demo>
        </Demos>
      </Section>

      <Section
        title="Sizes"
        description="The group's size sets the field, its words, icons and buttons together; a group lines up with a button of its size. Buttons are the field's height less 8."
      >
        <Demo class="items-stretch">
          {sizes.map((size) => (
            <Row label={size}>
              <View class="flex-1 min-w-40 max-w-80">
                <InputGroup size={size}>
                  <InputGroupAddon>
                    <Icon name="search" />
                  </InputGroupAddon>
                  <Input placeholder="Search projects" accessibilityLabel={`Search projects, ${size}`} />
                  <InputGroupAddon align="inlineEnd">
                    <InputGroupButton iconOnly icon="x" accessibilityLabel="Clear" />
                  </InputGroupAddon>
                </InputGroup>
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
        description="The field's state draws the frame: focus rings it (a focused button rings itself instead), invalid turns it destructive, read-only looks as at rest and keeps its buttons, disabled dims the group and its buttons once."
      >
        <Demo class="items-stretch">
          <View class="w-full flex-row flex-wrap gap-x-8 gap-y-6">
            <StateGroup label="Invalid" note="Enter a whole address." invalid value="ada@" />
            <StateGroup label="Read-only" note="Copy still works." readonly value="ACC-20931" />
            <StateGroup label="Disabled" note="Nothing takes presses." disabled value="Europe (Frankfurt)" />
          </View>
        </Demo>
      </Section>

      <TokensSection
        prefix="input-group"
        component="Input Group"
        note="The frame reads Input's tokens (input-*), the buttons Button's: a change there reaches the group too."
      />
    </Page>
  );
}

/** A sign-up form: an email with its icon, a password with its eye and a hint, in Fields. */
function SignUp() {
  const password = signal("");
  const short = () => password().length > 0 && password().length < 8;
  return (
    <View class="w-full max-w-[380px] gap-5">
      <Field>
        <FieldLabel>
          <Text>Email</Text>
        </FieldLabel>
        <InputGroup>
          <InputGroupAddon>
            <Text>@</Text>
          </InputGroupAddon>
          <Input
            placeholder="name@example.com"
            keyboardType="email-address"
            autoComplete="email"
            autoCapitalize="none"
            accessibilityLabel="Email"
          />
        </InputGroup>
      </Field>
      <Field invalid={short()}>
        <FieldLabel>
          <Text>Password</Text>
        </FieldLabel>
        <InputGroup>
          <Input
            masked
            value={password()}
            onChange={({ value }) => password.set(value)}
            autoComplete="new-password"
            accessibilityLabel="Password"
          />
          <InputGroupAddon align="inlineEnd">
            <InputGroupPasswordToggle />
          </InputGroupAddon>
        </InputGroup>
        {short() ? (
          <FieldError>
            <Text>Use 8 characters or more.</Text>
          </FieldError>
        ) : (
          <FieldDescription>
            <Text>At least 8 characters.</Text>
          </FieldDescription>
        )}
      </Field>
      <Button disabled={password().length < 8}>
        <Text>Create account</Text>
      </Button>
    </View>
  );
}

function ShownPassword() {
  const shown = signal(true);
  return (
    <View class="w-full max-w-72 gap-2">
      <InputGroup>
        <Input masked defaultValue="correct horse" accessibilityLabel="Password, shown" />
        <InputGroupAddon align="inlineEnd">
          <InputGroupPasswordToggle shown={shown()} onChange={({ shown: next }) => shown.set(next)} />
        </InputGroupAddon>
      </InputGroup>
      <Text class="text-xs text-muted-foreground">{shown() ? "Shown: the app keeps the state." : "Hidden."}</Text>
    </View>
  );
}

function SearchField() {
  const query = signal("");
  return (
    <View class="w-full max-w-80">
      <InputGroup>
        <InputGroupAddon>
          <Icon name="search" />
        </InputGroupAddon>
        <Input value={query()} onChange={({ value }) => query.set(value)} placeholder="Search" accessibilityLabel="Search" />
        <InputGroupAddon align="inlineEnd">
          {query() ? (
            <InputGroupButton iconOnly icon="x" accessibilityLabel="Clear" onPress={() => query.set("")} />
          ) : (
            <Kbd>
              <Text>{keymap.label("mod+k")}</Text>
            </Kbd>
          )}
        </InputGroupAddon>
      </InputGroup>
    </View>
  );
}

function CopyField() {
  const copied = signal(false);
  const key = "sk-live-8f2c41d9a7";
  return (
    <View class="w-full max-w-80">
      <InputGroup>
        <Input readonly value={key} accessibilityLabel="API key" />
        <InputGroupAddon align="inlineEnd">
          <InputGroupButton
            icon={copied() ? "check" : "copy"}
            onPress={async () => {
              await Clipboard.writeText(key);
              copied.set(true);
              setTimeout(() => copied.set(false), 1600);
            }}
          >
            <Text>{copied() ? "Copied" : "Copy"}</Text>
          </InputGroupButton>
        </InputGroupAddon>
      </InputGroup>
    </View>
  );
}

function UserName() {
  const name = signal("ada");
  const checking = signal(false);
  const taken = () => name() === "admin";
  let timer: ReturnType<typeof setTimeout> | undefined;
  const check = (value: string) => {
    name.set(value);
    checking.set(true);
    if (timer) clearTimeout(timer);
    timer = setTimeout(() => checking.set(false), 900);
  };
  return (
    <View class="w-full max-w-72 gap-2">
      <InputGroup>
        <InputGroupAddon>
          <Text>@</Text>
        </InputGroupAddon>
        <Input
          value={name()}
          onChange={({ value }) => check(value)}
          invalid={!checking() && taken()}
          autoCapitalize="none"
          accessibilityLabel="User name"
        />
        <InputGroupAddon align="inlineEnd">
          {checking() ? <Spinner /> : taken() ? <Icon name="x" class="text-destructive" /> : <Icon name="check" />}
        </InputGroupAddon>
      </InputGroup>
      <Text class={`text-xs ${!checking() && taken() ? "text-destructive" : "text-muted-foreground"}`}>
        {checking() ? "Checking…" : taken() ? "That name is taken." : "Available. Try admin."}
      </Text>
    </View>
  );
}

function MessageBox() {
  const message = signal("");
  return (
    <View class="w-full max-w-96">
      <InputGroup>
        <Textarea
          value={message()}
          onChange={({ value }) => message.set(value)}
          placeholder="Write a message"
          maxLength={280}
          accessibilityLabel="Message"
        />
        <InputGroupAddon align="blockEnd">
          <InputGroupButton iconOnly icon="plus" accessibilityLabel="Attach a file" />
          <Text class="flex-1">{`${message().length} / 280`}</Text>
          <InputGroupButton
            variant="default"
            iconOnly
            icon="arrow-right"
            disabled={message().length === 0}
            accessibilityLabel="Send"
            onPress={() => message.set("")}
          />
        </InputGroupAddon>
      </InputGroup>
    </View>
  );
}

function StateGroup(props: { label: string; note: string; value: string; invalid?: boolean; readonly?: boolean; disabled?: boolean }) {
  return (
    <View class="flex-1 min-w-56 gap-2">
      <Text class="text-sm font-medium text-foreground">{props.label}</Text>
      <InputGroup>
        <InputGroupAddon>
          <Icon name="search" />
        </InputGroupAddon>
        <Input
          defaultValue={props.value}
          invalid={props.invalid}
          readonly={props.readonly}
          disabled={props.disabled}
          accessibilityLabel={props.label}
        />
        <InputGroupAddon align="inlineEnd">
          <InputGroupButton icon="copy" accessibilityLabel={`Copy, ${props.label}`}>
            <Text>Copy</Text>
          </InputGroupButton>
        </InputGroupAddon>
      </InputGroup>
      <Text class={`text-xs ${props.invalid ? "text-destructive" : "text-muted-foreground"}`}>{props.note}</Text>
    </View>
  );
}
