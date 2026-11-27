// The home page's token demo: three seed tokens on sliders, the theme file
// they make, and a small screen of the library's components that follows
// them. The screen is inside a `ThemeScope`, so the sliders restyle it and
// nothing else on the page.

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
  Input,
  Label,
  Slider,
  Switch,
  Tabs,
  TabsList,
  TabsTrigger,
  ThemeScope,
} from "nsh-longbridge";
import { cssColor, followers } from "../theme";
import { Code } from "./code";
import { site } from "./site";
import { cardStyle } from "./site-ui";

export const knobs = [
  { name: "radius", label: "Radius", min: 0, max: 20, step: 1, start: 10 },
  { name: "control-height", label: "Control height", min: 24, max: 40, step: 2, start: 32 },
  { name: "font-size", label: "Font size", min: 12, max: 16, step: 1, start: 14 },
];

export function TokenDemo(props: { width: number }) {
  const values = signal<Record<string, number>>(Object.fromEntries(knobs.map((k) => [k.name, k.start])));
  const set = (name: string, v: number) => values.set({ ...values(), [name]: v });
  const tokens = () => Object.fromEntries(knobs.map((k) => [k.name, `${values()[k.name]}px`]));
  const file = () =>
    [
      "{",
      '  "cssVars": {',
      '    "theme": {',
      ...knobs.map((k, i) => `      "${k.name}": "${values()[k.name]}px"${i < knobs.length - 1 ? "," : ""}`),
      "    }",
      "  }",
      "}",
    ].join("\n");

  const split = () => props.width >= 860;

  const panel = () => (
    <View
      class="gap-6 p-6"
      style={split() ? { width: 340, borderRightWidth: 1, borderColor: site().line } : { borderBottomWidth: 1, borderColor: site().line }}
    >
      <View class="gap-5">
        {knobs.map((k) => (
          <View class="gap-3">
            <View class="flex-row items-center justify-between">
              <Text style={{ color: site().ink, fontSize: 13.5, fontWeight: "500" }}>{k.label}</Text>
              <Text class="font-mono tabular-nums" style={{ color: site().muted, fontSize: 12 }}>
                {`${values()[k.name]}px`}
              </Text>
            </View>
            <Slider
              min={k.min}
              max={k.max}
              step={k.step}
              value={values()[k.name]}
              onChange={(e) => set(k.name, e.value as number)}
              accessibilityLabel={k.label}
            />
          </View>
        ))}
      </View>
      <View class="rounded-xl px-4 py-3.5" style={{ backgroundColor: site().code.bg, boxShadow: `inset 0 0 0 1px ${site().line}` }}>
        <Code text={file()} size={12} />
      </View>
    </View>
  );

  const preview = () => (
    <View
      class="flex-1 min-w-0 items-center justify-center"
      style={{ backgroundColor: cssColor("muted"), paddingVertical: split() ? 40 : 24, paddingHorizontal: split() ? 40 : 14 }}
    >
      <WorkspaceCard tokens={tokens()} />
    </View>
  );

  return (
    <View class={split() ? "flex-row overflow-hidden" : "overflow-hidden"} style={cardStyle()}>
      {panel()}
      {preview()}
    </View>
  );
}

/** A small settings screen of the library's components, in a scope that carries `tokens` (the knobs' values). */
export function WorkspaceCard(props: { tokens: Record<string, string> }) {
  const tab = signal("general");
  const digest = signal(true);
  // A new scope for new values: one given new values in place does not pass
  // them on to the components in it.
  return (
    <Each of={[JSON.stringify(props.tokens)]}>
      {() => (
    <ThemeScope tokens={{ ...props.tokens, ...followers(props.tokens) }} class="w-full items-center">
      <Card class="w-full" style={{ maxWidth: 400 }}>
        <CardHeader>
          <CardTitle>
            <Text>Workspace</Text>
          </CardTitle>
          <CardDescription>
            <Text>How your team sees Northwind Studio.</Text>
          </CardDescription>
        </CardHeader>
        <CardContent class="gap-5">
          <Tabs value={tab()} onChange={(e) => tab.set(e.value)}>
            <TabsList accessibilityLabel="Settings">
              <TabsTrigger value="general">
                <Text>General</Text>
              </TabsTrigger>
              <TabsTrigger value="members">
                <Text>Members</Text>
              </TabsTrigger>
              <TabsTrigger value="billing">
                <Text>Billing</Text>
              </TabsTrigger>
            </TabsList>
          </Tabs>
          <View class="gap-2">
            <Label>
              <Text>Name</Text>
            </Label>
            <Input defaultValue="Northwind Studio" accessibilityLabel="Name" />
          </View>
          <View class="flex-row items-center justify-between gap-4">
            <View class="flex-row items-center gap-2">
              <Label>
                <Text>Weekly digest</Text>
              </Label>
              <Badge variant="secondary">
                <Text>New</Text>
              </Badge>
            </View>
            <Switch checked={digest()} onChange={(e) => digest.set(e.checked)} accessibilityLabel="Weekly digest" />
          </View>
        </CardContent>
        <CardFooter class="justify-end gap-2">
          <Button variant="outline">
            <Text>Cancel</Text>
          </Button>
          <Button>
            <Text>Save</Text>
          </Button>
        </CardFooter>
      </Card>
    </ThemeScope>
      )}
    </Each>
  );
}
