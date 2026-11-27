// The AlertDialog page: the question's panel, the small size with media,
// alerts to open (a destructive one, one whose action takes time).

import { signal, Text } from "nsh";
import {
  AlertDialog,
  AlertDialogAction,
  AlertDialogCancel,
  AlertDialogContent,
  AlertDialogDescription,
  AlertDialogFooter,
  AlertDialogHeader,
  AlertDialogMedia,
  AlertDialogTitle,
  Button,
  Icon,
  toast,
} from "nsh-longbridge";
import { Demo, Demos, Page, Section } from "../ui/layout";
import { TokensSection } from "../ui/tokens";

export function AlertDialogPage() {
  return (
    <Page
      eyebrow="Components"
      title="Alert Dialog"
      description="A modal question that must be answered: a title, what it does, Cancel and an action. A press outside does not close it, and it opens with the focus on Cancel."
    >
      <Demo hero class="bg-muted/40">
        <AlertDialogContent>
          <AlertDialogHeader>
            <AlertDialogTitle>
              <Text>Are you absolutely sure?</Text>
            </AlertDialogTitle>
            <AlertDialogDescription>
              <Text>This action cannot be undone. This will permanently delete your account and remove your data from our servers.</Text>
            </AlertDialogDescription>
          </AlertDialogHeader>
          <AlertDialogFooter>
            <AlertDialogCancel>
              <Text>Cancel</Text>
            </AlertDialogCancel>
            <AlertDialogAction>
              <Text>Continue</Text>
            </AlertDialogAction>
          </AlertDialogFooter>
        </AlertDialogContent>
      </Demo>

      <Section
        title="Sizes and media"
        description="size sm (and xs) is 320 wide, centered, its two buttons side by side at every width. A media tile sits above the title, or beside it in a wide md alert."
      >
        <Demos>
          <Demo label="sm, with media" class="bg-muted/40">
            <DeleteFilesPanel />
          </Demo>
          <Demo label="md, with media" class="bg-muted/40">
            <AlertDialogContent>
              <AlertDialogHeader>
                <AlertDialogMedia>
                  <Icon name="triangle-alert" />
                </AlertDialogMedia>
                <AlertDialogTitle>
                  <Text>Leave without saving?</Text>
                </AlertDialogTitle>
                <AlertDialogDescription>
                  <Text>Your changes to this page will be lost.</Text>
                </AlertDialogDescription>
              </AlertDialogHeader>
              <AlertDialogFooter>
                <AlertDialogCancel>
                  <Text>Keep editing</Text>
                </AlertDialogCancel>
                <AlertDialogAction>
                  <Text>Leave</Text>
                </AlertDialogAction>
              </AlertDialogFooter>
            </AlertDialogContent>
          </Demo>
        </Demos>
      </Section>

      <Section title="Open one" description="Return right after it opens presses Cancel: the safe answer. Escape cancels too.">
        <Demo row>
          <ShowDialog />
          <DeleteDialog />
          <SlowActionDialog />
        </Demo>
      </Section>

      <TokensSection
        prefix="alert-dialog"
        component="AlertDialog"
        note="Its panel, overlay, title and description read the dialog-* tokens (on the Dialog page)."
      />
    </Page>
  );
}

function DeleteFilesPanel() {
  return (
    <AlertDialogContent size="sm">
      <AlertDialogHeader>
        <AlertDialogMedia>
          <Icon name="trash-2" />
        </AlertDialogMedia>
        <AlertDialogTitle>
          <Text>Delete 3 files?</Text>
        </AlertDialogTitle>
        <AlertDialogDescription>
          <Text>They will be removed for everyone.</Text>
        </AlertDialogDescription>
      </AlertDialogHeader>
      <AlertDialogFooter>
        <AlertDialogCancel>
          <Text>Cancel</Text>
        </AlertDialogCancel>
        <AlertDialogAction variant="destructive">
          <Text>Delete</Text>
        </AlertDialogAction>
      </AlertDialogFooter>
    </AlertDialogContent>
  );
}

function ShowDialog() {
  return (
    <AlertDialog trigger={<Button variant="outline"><Text>Show dialog</Text></Button>}>
      <AlertDialogContent>
        <AlertDialogHeader>
          <AlertDialogTitle>
            <Text>Are you absolutely sure?</Text>
          </AlertDialogTitle>
          <AlertDialogDescription>
            <Text>This action cannot be undone. This will permanently delete your account and remove your data from our servers.</Text>
          </AlertDialogDescription>
        </AlertDialogHeader>
        <AlertDialogFooter>
          <AlertDialogCancel>
            <Text>Cancel</Text>
          </AlertDialogCancel>
          <AlertDialogAction onPress={() => toast("Account deleted", { kind: "success" })}>
            <Text>Continue</Text>
          </AlertDialogAction>
        </AlertDialogFooter>
      </AlertDialogContent>
    </AlertDialog>
  );
}

function DeleteDialog() {
  return (
    <AlertDialog trigger={<Button variant="destructive" icon="trash-2"><Text>Delete files</Text></Button>}>
      <AlertDialogContent size="sm">
        <AlertDialogHeader>
          <AlertDialogMedia>
            <Icon name="trash-2" />
          </AlertDialogMedia>
          <AlertDialogTitle>
            <Text>Delete 3 files?</Text>
          </AlertDialogTitle>
          <AlertDialogDescription>
            <Text>They will be removed for everyone.</Text>
          </AlertDialogDescription>
        </AlertDialogHeader>
        <AlertDialogFooter>
          <AlertDialogCancel>
            <Text>Cancel</Text>
          </AlertDialogCancel>
          <AlertDialogAction
            variant="destructive"
            onPress={() => toast("3 files deleted", { action: { id: "undo", label: "Undo" } })}
          >
            <Text>Delete</Text>
          </AlertDialogAction>
        </AlertDialogFooter>
      </AlertDialogContent>
    </AlertDialog>
  );
}

/** The action waits for its work: the app keeps open until it ends. */
function SlowActionDialog() {
  const open = signal(false);
  const busy = signal(false);
  return (
    <AlertDialog
      open={open()}
      onOpenChange={(e) => {
        if (e.reason === "select") {
          busy.set(true);
          setTimeout(() => {
            busy.set(false);
            open.set(false);
            toast("Signed out of every device", { kind: "success" });
          }, 1500);
        } else if (!busy()) {
          open.set(e.open);
        }
      }}
      trigger={<Button variant="outline"><Text>Sign out everywhere</Text></Button>}
    >
      <AlertDialogContent>
        <AlertDialogHeader>
          <AlertDialogTitle>
            <Text>Sign out of every device?</Text>
          </AlertDialogTitle>
          <AlertDialogDescription>
            <Text>You will need to sign in again on your phone and your other computers.</Text>
          </AlertDialogDescription>
        </AlertDialogHeader>
        <AlertDialogFooter>
          <AlertDialogCancel disabled={busy()}>
            <Text>Cancel</Text>
          </AlertDialogCancel>
          <AlertDialogAction loading={busy()}>
            <Text>{busy() ? "Signing out…" : "Sign out"}</Text>
          </AlertDialogAction>
        </AlertDialogFooter>
      </AlertDialogContent>
    </AlertDialog>
  );
}
