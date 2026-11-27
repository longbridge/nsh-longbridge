// The Toast page: an event created with Undo, the kinds, a loading toast
// replaced by its result, the answer of each button, where the stack is.

import { signal, Text, View } from "nsh";
import {
  Badge,
  Button,
  Card,
  CardContent,
  CardDescription,
  CardFooter,
  CardHeader,
  CardTitle,
  dismissToast,
  RadioGroup,
  RadioGroupItem,
  Switch,
  toast,
  type ToastKind,
  type ToastPlacement,
} from "nsh-longbridge";
import { Demo, Demos, Page, Section } from "../ui/layout";
import { TokensSection } from "../ui/tokens";
import { toasterCloseButton, toasterPlacement } from "../ui/toaster";

const kinds: { kind: ToastKind; label: string; title: string; description?: string }[] = [
  { kind: "default", label: "Default", title: "Event has been created", description: "Sunday, December 03, 2023 at 9:00 AM" },
  { kind: "success", label: "Success", title: "Changes saved" },
  { kind: "info", label: "Info", title: "Be at the area 10 minutes before the event time" },
  { kind: "warning", label: "Warning", title: "Event start time cannot be earlier than 8am" },
  { kind: "error", label: "Error", title: "Event has not been created", description: "Check your connection and try again." },
  { kind: "loading", label: "Loading", title: "Syncing your calendar…" },
];

const placements: { id: ToastPlacement; label: string }[] = [
  { id: "topLeft", label: "Top left" },
  { id: "topCenter", label: "Top center" },
  { id: "topRight", label: "Top right" },
  { id: "bottomLeft", label: "Bottom left" },
  { id: "bottomCenter", label: "Bottom center" },
  { id: "bottomRight", label: "Bottom right" },
];

export function ToastPage() {
  return (
    <Page
      eyebrow="Components"
      title="Toast"
      description="Brief messages at a corner of the window that stack and go by themselves. Place one Toaster at the app's root; call toast() from anywhere."
    >
      <Demo hero label="Add it to the calendar: the toast shows at the corner of the window, with Undo.">
        <EventCard />
      </Demo>

      <Section title="Kinds" description="The kind sets the icon before the text, nothing else. A loading toast spins and never times out. The pointer on the stack spreads it out and pauses the time.">
        <Demo row>
          {kinds.map((k) => (
            <Button
              variant="outline"
              onPress={() => toast(k.title, { kind: k.kind, description: k.description, id: k.kind === "loading" ? "sync" : undefined })}
            >
              <Text>{k.label}</Text>
            </Button>
          ))}
          <Button variant="ghost" onPress={() => dismissToast()}>
            <Text>Dismiss all</Text>
          </Button>
        </Demo>
      </Section>

      <Section title="Answers and changes" description="A toast() call resolves once the toast is gone, with the button that closed it. A toast shown again with its id changes in place: a loading toast becomes its result.">
        <Demos>
          <AnswerDemo />
          <UploadDemo />
        </Demos>
      </Section>

      <Section title="Where the stack is" description="The Toaster's placement; on touch screens and windows narrower than 600 the stack spans the window at that edge. Toasts show above an open dialog.">
        <PlacementDemo />
      </Section>

      <TokensSection prefix="toast" component="Toast" />
    </Page>
  );
}

/** A calendar event, created with an Undo. */
function EventCard() {
  const created = signal(false);
  const create = async () => {
    created.set(true);
    const { action } = await toast("Event has been created", {
      description: "Sunday, December 03, 2023 at 9:00 AM",
      action: { id: "undo", label: "Undo" },
    });
    if (action === "undo") created.set(false);
  };
  return (
    <Card class="w-full max-w-[360px]">
      <CardHeader>
        <CardTitle>
          <Text>Team sync</Text>
        </CardTitle>
        <CardDescription>
          <Text>Sunday, December 03 · 9:00 – 9:30 AM</Text>
        </CardDescription>
      </CardHeader>
      <CardContent class="gap-3">
        <View class="flex-row items-center gap-2">
          <Badge variant={created() ? "default" : "outline"}>
            <Text>{created() ? "On your calendar" : "Not added"}</Text>
          </Badge>
        </View>
        <Text class="text-sm text-muted-foreground">Weekly planning with the design and platform teams.</Text>
      </CardContent>
      <CardFooter>
        <Button class="flex-1" disabled={created()} onPress={create}>
          <Text>Add to calendar</Text>
        </Button>
      </CardFooter>
    </Card>
  );
}

function AnswerDemo() {
  const answer = signal("—");
  const ask = async () => {
    answer.set("waiting…");
    const { action } = await toast("Message archived", {
      description: "It is out of your inbox.",
      cancel: { id: "open", label: "Open" },
      action: { id: "undo", label: "Undo" },
      duration: 8000,
    });
    answer.set(action ?? "null (it went by itself, a swipe or ×)");
  };
  return (
    <Demo row label={`answer: ${answer()}`}>
      <Button variant="outline" onPress={ask}>
        <Text>Archive</Text>
      </Button>
    </Demo>
  );
}

function UploadDemo() {
  const busy = signal(false);
  const upload = (fails: boolean) => {
    busy.set(true);
    toast("Uploading report.pdf…", { id: "upload", kind: "loading" });
    setTimeout(() => {
      busy.set(false);
      if (fails) {
        toast("Upload failed", { id: "upload", kind: "error", description: "Check your connection.", duration: Infinity, closeButton: true });
      } else {
        toast("Uploaded report.pdf", { id: "upload", kind: "success" });
      }
    }, 2000);
  };
  return (
    <Demo row label="One toast, changed in place">
      <Button variant="outline" disabled={busy()} onPress={() => upload(false)}>
        <Text>Upload</Text>
      </Button>
      <Button variant="outline" disabled={busy()} onPress={() => upload(true)}>
        <Text>Upload, failing</Text>
      </Button>
    </Demo>
  );
}

function PlacementDemo() {
  return (
    <Demo class="items-stretch">
      <View class="flex-row flex-wrap items-start gap-x-12 gap-y-6 self-center">
        <RadioGroup
          accessibilityLabel="Placement"
          value={toasterPlacement()}
          onChange={(e) => {
            toasterPlacement.set(e.value as ToastPlacement);
            toast(`Shown ${placements.find((p) => p.id === e.value)?.label.toLowerCase()}`);
          }}
          class="gap-3"
        >
          {placements.map((p) => (
            <RadioGroupItem value={p.id}>
              <Text>{p.label}</Text>
            </RadioGroupItem>
          ))}
        </RadioGroup>
        <View class="gap-4">
          <Switch checked={toasterCloseButton()} onChange={(e) => toasterCloseButton.set(e.checked)}>
            <Text>Every toast shows ×</Text>
          </Switch>
          <Button variant="outline" onPress={() => toast("Here", { description: "With the Toaster's current options." })}>
            <Text>Show a toast</Text>
          </Button>
        </View>
      </View>
    </Demo>
  );
}
