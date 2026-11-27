// The Progress page: uploads that move, an unknown wait, a label under the
// bar, other heights and colors.

import { signal, Text, View } from "nsh";
import { Button, Card, CardContent, CardDescription, CardHeader, CardTitle, Progress } from "nsh-longbridge";
import { Demo, Page, Section } from "../ui/layout";
import { TokensSection } from "../ui/tokens";

export function ProgressPage() {
  return (
    <Page
      eyebrow="Components"
      title="Progress"
      description="How far a task is: a track filled to the value, the fill moving smoothly as it changes; a segment crossing the track while the extent is unknown."
    >
      <HeroDemo />

      <Section
        title="Unknown length"
        description="A segment crosses the track again and again. With reduced motion it stays still across the track, dimmed, so it reads neither as done nor as not started."
      >
        <Demo class="items-stretch">
          <View class="w-full max-w-[420px] self-center gap-2">
            <Progress indeterminate accessibilityLabel="Connecting to the server" />
            <Text class="text-sm text-muted-foreground">Connecting to the server…</Text>
          </View>
        </Demo>
      </Section>

      <Section title="With a label" description="Children go under the track, and screen readers read them as its name.">
        <Demo class="items-stretch">
          <View class="w-full max-w-[420px] self-center gap-6">
            <Progress value={66}>
              <View class="flex-row justify-between">
                <Text class="text-sm text-muted-foreground">2 of 3 files</Text>
                <Text class="text-sm font-medium tabular-nums text-foreground">66%</Text>
              </View>
            </Progress>
            <Progress value={100}>
              <Text class="text-sm text-muted-foreground">Backup complete</Text>
            </Progress>
          </View>
        </Demo>
      </Section>

      <Section
        title="Heights and colors"
        description="One 8-point bar by default; the track's classes set another height, the indicator's another fill."
      >
        <Demo class="items-stretch">
          <View class="w-full max-w-[420px] self-center gap-6">
            <StorageRow label="Photos" value={72} indicator="bg-blue-600" />
            <StorageRow label="Documents" value={38} indicator="bg-green-600" />
            <StorageRow label="Storage used" value={91} indicator="bg-destructive" />
          </View>
        </Demo>
      </Section>

      <TokensSection prefix="progress" component="Progress" />
    </Page>
  );
}

function StorageRow(props: { label: string; value: number; indicator: string }) {
  return (
    <View class="gap-2">
      <View class="flex-row justify-between">
        <Text class="text-sm text-foreground">{props.label}</Text>
        <Text class="text-sm tabular-nums text-muted-foreground">{`${props.value}%`}</Text>
      </View>
      <Progress value={props.value} trackClass="h-1" indicatorClass={props.indicator} accessibilityLabel={props.label} />
    </View>
  );
}

const files = [
  { name: "report-2026.pdf", size: "2.4 MB" },
  { name: "screenshots.zip", size: "18.1 MB" },
  { name: "notes.md", size: "12 KB" },
];

function HeroDemo() {
  const done = signal([13, 48, 100]);
  const step = () => done.set(done().map((v, i) => Math.min(100, v + [21, 17, 0][i])));
  const reset = () => done.set([0, 0, 0]);
  return (
    <Demo hero>
      <Card class="w-full max-w-[420px]">
        <CardHeader>
          <CardTitle>
            <Text>Uploading</Text>
          </CardTitle>
          <CardDescription>
            <Text>{`${done().filter((v) => v === 100).length} of ${files.length} files done`}</Text>
          </CardDescription>
        </CardHeader>
        <CardContent class="gap-5">
          {files.map((f, i) => (
            <View class="gap-2">
              <View class="flex-row justify-between gap-3">
                <Text class="truncate text-sm font-medium">{f.name}</Text>
                <Text class="text-sm tabular-nums text-muted-foreground">{done()[i] === 100 ? f.size : `${done()[i]}%`}</Text>
              </View>
              <Progress value={done()[i]} accessibilityLabel={`Uploading ${f.name}`} />
            </View>
          ))}
          <View class="flex-row gap-2">
            <Button size="sm" onPress={step} disabled={done().every((v) => v === 100)}>
              <Text>Continue</Text>
            </Button>
            <Button size="sm" variant="outline" onPress={reset}>
              <Text>Start over</Text>
            </Button>
          </View>
        </CardContent>
      </Card>
    </Demo>
  );
}
