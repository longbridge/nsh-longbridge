// The Spinner page: an upload list, sizes, colors, beside a status and in
// a button.

import { Text, View } from "nsh";
import { Badge, Button, Icon, type Size, Spinner } from "nsh-longbridge";
import { Demo, Demos, Page, Section } from "../ui/layout";
import { FileText, Glyph } from "../ui/icons";
import { TokensSection } from "../ui/tokens";

const sizes: Size[] = ["xs", "sm", "md", "lg"];
const colors = [
  { class: "text-foreground", label: "foreground" },
  { class: "text-muted-foreground", label: "muted" },
  { class: "text-primary", label: "primary" },
  { class: "text-destructive", label: "destructive" },
];
const uploads = [
  { name: "quarterly-report.pdf", size: "2.4 MB", state: "done" },
  { name: "design-review.fig", size: "18.1 MB", state: "uploading" },
  { name: "team-photo.jpg", size: "4.7 MB", state: "waiting" },
];

export function SpinnerPage() {
  return (
    <Page
      eyebrow="Components"
      title="Spinner"
      description="Something is going on and how long it takes is unknown. It keeps turning with reduced motion: its turning is what it says."
    >
      <Demo hero>
        <View class="w-full max-w-[400px] rounded-lg border border-border">
          {uploads.map((u, i) => (
            <View class={`h-14 flex-row items-center gap-3 px-4 ${i > 0 ? "border-t border-border" : ""}`}>
              <View class="size-8 items-center justify-center rounded-md bg-muted">
                <Glyph icon={FileText} size={16} class="text-muted-foreground" />
              </View>
              <View class="flex-1 gap-0.5">
                <Text class="truncate text-sm font-medium text-foreground">{u.name}</Text>
                <Text class="text-xs text-muted-foreground">{u.size}</Text>
              </View>
              {u.state === "done" ? (
                <Badge variant="secondary" class="self-center">
                  <Icon name="check" />
                  <Text>Uploaded</Text>
                </Badge>
              ) : u.state === "uploading" ? (
                <View class="flex-row items-center gap-2">
                  <Spinner size="sm" class="text-muted-foreground" />
                  <Text class="text-xs text-muted-foreground">Uploading</Text>
                </View>
              ) : (
                <Text class="text-xs text-muted-foreground">Waiting</Text>
              )}
            </View>
          ))}
        </View>
      </Demo>

      <Section title="Sizes and colors" description="The icon sizes, 12 / 14 / 16 / 16, and larger with a class. It takes the color of the text around it.">
        <Demos>
          <Demo row class="gap-8" label="xs, sm, md, lg and size-8">
            {sizes.map((s) => (
              <Spinner size={s} class="text-foreground" />
            ))}
            <Spinner class="size-8 text-foreground" />
          </Demo>
          <Demo row class="gap-8" label="foreground, muted, primary, destructive">
            {colors.map((c) => (
              <Spinner class={c.class} />
            ))}
          </Demo>
        </Demos>
      </Section>

      <Section title="In place" description="Beside a status, and in a button, where it takes the button's size and color. A button's own wait is its loading prop.">
        <Demos>
          <Demo row label="A status line">
            <View class="flex-row items-center gap-2">
              <Spinner size="sm" class="text-muted-foreground" />
              <Text class="text-sm text-muted-foreground">Syncing 3 of 12 files…</Text>
            </View>
          </Demo>
          <Demo row label="Buttons that wait">
            <Button variant="outline" size="sm" disabled>
              <Spinner />
              <Text>Please wait</Text>
            </Button>
            <Button size="sm" loading>
              <Text>Deploying</Text>
            </Button>
          </Demo>
        </Demos>
      </Section>

      <TokensSection prefix="spinner" component="Spinner" note="has no tokens of its own: its size is the icon size of its size, and its color is its class's text color." />
    </Page>
  );
}
