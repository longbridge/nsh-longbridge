// The Dialog page: an edit form's panel, dialogs to open (a form, a long
// text that scrolls, one without the "×", one the app opens), controlled.

import { Scroll, type Signal, signal, Text, View } from "nsh";
import {
  Button,
  Checkbox,
  Dialog,
  DialogClose,
  DialogContent,
  DialogDescription,
  DialogFooter,
  DialogHeader,
  DialogTitle,
  Input,
  type InputRef,
  Label,
  Textarea,
  toast,
} from "nsh-longbridge";
import { Demo, Demos, Page, Section } from "../ui/layout";
import { TokensSection } from "../ui/tokens";

const terms = [
  "These terms apply to your use of the service and every workspace you join.",
  "You keep the rights to what you upload. You give us the rights we need to store it, show it to the people you share it with, and keep backups.",
  "Payments renew each month until you cancel. Cancelling stops the next renewal; the current month stays paid.",
  "We may change these terms. We tell you by email 30 days before a change that affects you, and you may leave before it applies.",
  "We keep logs for 90 days to find faults and abuse, then delete them. Exports and deletion are in your settings.",
  "Either of us may end this agreement at any time. When it ends, you can export your data for 30 days.",
];

export function DialogPage() {
  return (
    <Page
      eyebrow="Components"
      title="Dialog"
      description="A modal window over the app: a centered panel over a dimmed page. The focus and screen readers stay inside until it closes, then the focus goes back."
    >
      <Demo hero class="bg-muted/40">
        <ProfilePanel />
      </Demo>

      <Section
        title="Open one"
        description="Escape, the overlay, the × and a DialogClose close it. Below 640 points wide the header centers and the footer stacks, the first button at the bottom."
      >
        <Demo row class="gap-3">
          <EditProfileDialog />
          <TermsDialog />
          <ShareDialog />
          <FeedbackDialog />
        </Demo>
      </Section>

      <Section title="Controlled" description="The app owns open. onOpen comes once the dialog is all there: the place to focus its first field.">
        <ControlledDemo />
      </Section>

      <TokensSection prefix="dialog" component="Dialog" note="AlertDialog reads them too." />
    </Page>
  );
}

/** The fields of a profile: a name and a username, each named by its label. */
function ProfileFields(props: { name?: Signal<InputRef | undefined> }) {
  const name = props.name ?? signal<InputRef>();
  const username = signal<InputRef>();
  return (
    <View class="gap-4">
      <View class="gap-3">
        <Label htmlFor={name()}>
          <Text>Name</Text>
        </Label>
        <Input ref={name} defaultValue="Pedro Duarte" />
      </View>
      <View class="gap-3">
        <Label htmlFor={username()}>
          <Text>Username</Text>
        </Label>
        <Input ref={username} defaultValue="@peduarte" />
      </View>
    </View>
  );
}

/** The panel drawn in place: the look of an open dialog. */
function ProfilePanel() {
  return (
    <DialogContent class="max-w-[425px]">
      <DialogHeader>
        <DialogTitle>
          <Text>Edit profile</Text>
        </DialogTitle>
        <DialogDescription>
          <Text>Make changes to your profile here. Click save when you're done.</Text>
        </DialogDescription>
      </DialogHeader>
      <ProfileFields />
      <DialogFooter>
        <DialogClose>
          <Text>Cancel</Text>
        </DialogClose>
        <Button>
          <Text>Save changes</Text>
        </Button>
      </DialogFooter>
    </DialogContent>
  );
}

function EditProfileDialog() {
  return (
    <Dialog trigger={<Button variant="outline"><Text>Edit profile</Text></Button>}>
      <DialogContent class="max-w-[425px]">
        <DialogHeader>
          <DialogTitle>
            <Text>Edit profile</Text>
          </DialogTitle>
          <DialogDescription>
            <Text>Make changes to your profile here. Click save when you're done.</Text>
          </DialogDescription>
        </DialogHeader>
        <ProfileFields />
        <DialogFooter>
          <DialogClose>
            <Text>Cancel</Text>
          </DialogClose>
          <DialogClose variant="default" onPress={() => toast("Profile saved", { kind: "success" })}>
            <Text>Save changes</Text>
          </DialogClose>
        </DialogFooter>
      </DialogContent>
    </Dialog>
  );
}

/** A long text: its body scrolls, the header and footer stay. */
function TermsDialog() {
  const agreed = signal(false);
  return (
    <Dialog trigger={<Button variant="outline"><Text>Terms of service</Text></Button>}>
      <DialogContent>
        <DialogHeader>
          <DialogTitle>
            <Text>Terms of service</Text>
          </DialogTitle>
          <DialogDescription>
            <Text>Last updated March 2025.</Text>
          </DialogDescription>
        </DialogHeader>
        <Scroll class="max-h-[240px]" contentClass="gap-3 pr-3">
          {[...terms, ...terms].map((t, i) => (
            <Text class="text-sm leading-6">{`${i + 1}. ${t}`}</Text>
          ))}
        </Scroll>
        <Checkbox checked={agreed()} onChange={(e) => agreed.set(e.checked)}>
          <Text>I have read and accept the terms</Text>
        </Checkbox>
        <DialogFooter>
          <DialogClose>
            <Text>Decline</Text>
          </DialogClose>
          <DialogClose variant="default" disabled={!agreed()}>
            <Text>Accept</Text>
          </DialogClose>
        </DialogFooter>
      </DialogContent>
    </Dialog>
  );
}

/** No "×": the footer's buttons close it. */
function ShareDialog() {
  return (
    <Dialog trigger={<Button variant="outline"><Text>Share link</Text></Button>}>
      <DialogContent showCloseButton={false} class="max-w-[448px]">
        <DialogHeader>
          <DialogTitle>
            <Text>Share link</Text>
          </DialogTitle>
          <DialogDescription>
            <Text>Anyone who has this link will be able to view this.</Text>
          </DialogDescription>
        </DialogHeader>
        <Input defaultValue="https://ui.shadcn.com/docs/installation" accessibilityLabel="Link" readonly />
        <DialogFooter class="justify-start">
          <DialogClose variant="secondary">
            <Text>Close</Text>
          </DialogClose>
        </DialogFooter>
      </DialogContent>
    </Dialog>
  );
}

function FeedbackDialog() {
  const text = signal("");
  return (
    <Dialog trigger={<Button variant="ghost"><Text>Send feedback</Text></Button>} closeOnBackdropPress={false}>
      <DialogContent>
        <DialogHeader>
          <DialogTitle>
            <Text>Send feedback</Text>
          </DialogTitle>
          <DialogDescription>
            <Text>A press outside does not close this one: what you typed stays.</Text>
          </DialogDescription>
        </DialogHeader>
        <Textarea placeholder="Tell us what you think…" accessibilityLabel="Feedback" value={text()} onChange={(e) => text.set(e.value)} />
        <DialogFooter>
          <DialogClose>
            <Text>Cancel</Text>
          </DialogClose>
          <DialogClose
            variant="default"
            disabled={text().trim() === ""}
            onPress={() => {
              toast("Thanks for the feedback", { description: `${text().length} characters sent.` });
              text.set("");
            }}
          >
            <Text>Send</Text>
          </DialogClose>
        </DialogFooter>
      </DialogContent>
    </Dialog>
  );
}

function ControlledDemo() {
  const open = signal(false);
  const log = signal<string[]>([]);
  const first = signal<InputRef>();
  const note = (entry: string) => log.set([...log().slice(-3), entry]);
  return (
    <Demos>
      <Demo row label={`open: ${open()}`}>
        <Button onPress={() => open.set(true)}>
          <Text>Open from the app</Text>
        </Button>
        <Dialog
          open={open()}
          onOpenChange={(e) => {
            open.set(e.open);
            note(`openChange: ${e.open} (${e.reason})`);
          }}
          onOpen={() => {
            first()?.focus();
            note("open");
          }}
          onClose={() => note("close")}
        >
          <DialogContent class="max-w-[425px]">
            <DialogHeader>
              <DialogTitle>
                <Text>Edit profile</Text>
              </DialogTitle>
              <DialogDescription>
                <Text>The name field has the focus as it opens.</Text>
              </DialogDescription>
            </DialogHeader>
            <ProfileFields name={first} />
            <DialogFooter>
              <DialogClose>
                <Text>Cancel</Text>
              </DialogClose>
              <DialogClose variant="default">
                <Text>Save changes</Text>
              </DialogClose>
            </DialogFooter>
          </DialogContent>
        </Dialog>
      </Demo>
      <Demo label="Its events, newest last" class="items-start justify-start">
        {log().length === 0 ? (
          <Text class="text-sm text-muted-foreground">Nothing yet</Text>
        ) : (
          log().map((l) => <Text class="font-mono text-xs text-foreground">{l}</Text>)
        )}
      </Demo>
    </Demos>
  );
}
