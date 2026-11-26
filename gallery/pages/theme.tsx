// The Theme page: seed tokens tweaked live, the theme file to export, and
// `ThemeScope`.

import { Clipboard, signal, Text, View } from "nsh";
import { Slider } from "nsh/base";
import { Button, ButtonVariant, ThemeScope } from "nsh-longbridge";
import {
  controlHeight,
  controlHeightDefault,
  primary,
  radius,
  RADIUS_DEFAULT,
  resetTweaks,
  swatches,
  themeJson,
} from "../theme";
import { Caption, Code, Page, Preview, Section } from "../ui";

const variants: ButtonVariant[] = ["default", "secondary", "outline", "ghost", "destructive", "link"];

export function ThemePage() {
  return (
    <Page
      title="Theme"
      description="A few seed tokens move the whole library. The color and density presets are in the top bar; the tweaks here go over them."
    >
      <Section title="Seeds" description="Applied with setTheme as you change them.">
        <View class="rounded-xl border border-border p-6 gap-6">
          <SliderRow
            label="radius"
            min={0}
            max={20}
            step={1}
            value={() => radius() ?? RADIUS_DEFAULT}
            onChange={(v) => radius.set(v)}
          />
          <SliderRow
            label="control-height"
            min={24}
            max={40}
            step={2}
            value={() => controlHeight() ?? controlHeightDefault()}
            onChange={(v) => controlHeight.set(v)}
          />
          <View class="flex-row items-center gap-4">
            <Text class="w-32 font-mono text-sm text-foreground">primary</Text>
            <View class="flex-row flex-wrap gap-2">
              <Button size="sm" variant={primary() === null ? "secondary" : "ghost"} onPress={() => primary.set(null)}>
                <Text>Preset</Text>
              </Button>
              {swatches.map((s) => (
                <Button
                  size="sm"
                  variant={primary() === s.id ? "secondary" : "ghost"}
                  onPress={() => primary.set(s.id)}
                >
                  <View class="size-3 rounded-full" style={{ backgroundColor: s.light }} />
                  <Text>{s.label}</Text>
                </Button>
              ))}
            </View>
          </View>
          <View class="flex-row">
            <Button variant="outline" size="sm" onPress={resetTweaks}>
              <Text>Reset</Text>
            </Button>
          </View>
        </View>
        <Preview>
          {variants.map((v) => (
            <Button variant={v}>
              <Text>{v[0].toUpperCase() + v.slice(1)}</Text>
            </Button>
          ))}
          <Button variant="outline" icon="plus">
            <Text>New file</Text>
          </Button>
          <Button variant="outline" iconOnly icon="plus" accessibilityLabel="Add" />
        </Preview>
      </Section>

      <Section
        title="Export"
        description='The theme shown, as a file: save it next to nsh.json and name it there ("theme": "my-theme.json").'
      >
        <ExportBlock />
      </Section>

      <Section
        title="ThemeScope"
        description="Tokens for one part of the screen. This library's components inside use them; scopes nest, the inner one winning."
      >
        <View class="flex-row flex-wrap gap-4">
          <ScopeDemo
            code={`<ThemeScope appearance="dark" surface>\n  <Button>Save</Button>\n  <Button variant="outline">Cancel</Button>\n</ThemeScope>`}
          >
            <ThemeScope appearance="dark" surface class="rounded-lg p-6 flex-row flex-wrap gap-3">
              <Button>
                <Text>Save</Text>
              </Button>
              <Button variant="outline">
                <Text>Cancel</Text>
              </Button>
              <Button variant="ghost" iconOnly icon="trash-2" accessibilityLabel="Delete" />
            </ThemeScope>
          </ScopeDemo>
          <ScopeDemo
            code={`<ThemeScope tokens={{\n  primary: "oklch(0.627 0.194 149.214)",\n  "button-radius": "0px",\n}}>\n  <Button>Save</Button>\n</ThemeScope>`}
          >
            <ThemeScope
              tokens={{ primary: "oklch(0.627 0.194 149.214)", "primary-foreground": "oklch(0.985 0 0)", "button-radius": "0px" }}
              class="p-6 flex-row flex-wrap gap-3"
            >
              <Button>
                <Text>Save</Text>
              </Button>
              <Button variant="outline">
                <Text>Cancel</Text>
              </Button>
              <Button variant="link">
                <Text>Details</Text>
              </Button>
            </ThemeScope>
          </ScopeDemo>
        </View>
      </Section>
    </Page>
  );
}

function SliderRow(props: {
  label: string;
  min: number;
  max: number;
  step: number;
  value: () => number;
  onChange: (value: number) => void;
}) {
  return (
    <View class="flex-row items-center gap-4">
      <Text class="w-32 font-mono text-sm text-foreground">{props.label}</Text>
      <Slider
        class="flex-1 max-w-80"
        min={props.min}
        max={props.max}
        step={props.step}
        value={props.value()}
        accessibilityLabel={props.label}
        trackClass="h-6 justify-center"
        indicatorClass="h-1.5 rounded-full bg-muted"
        rangeClass="rounded-full bg-primary"
        thumbClass="size-4 rounded-full border border-primary bg-background shadow-sm focus-visible:ring-4 focus-visible:ring-ring/50"
        onChange={(e) => props.onChange(e.value as number)}
      />
      <Text class="w-12 text-right font-mono text-sm text-muted-foreground">{`${props.value()}`}</Text>
    </View>
  );
}

function ExportBlock() {
  const copied = signal(false);
  const copy = async () => {
    await Clipboard.writeText(themeJson());
    copied.set(true);
    setTimeout(() => copied.set(false), 1500);
  };
  return (
    <View class="rounded-xl border border-border overflow-hidden">
      <View class="flex-row items-center justify-between px-5 py-3">
        <Caption>my-theme.json</Caption>
        <Button size="xs" variant="outline" onPress={copy}>
          <Text>{copied() ? "Copied" : "Copy"}</Text>
        </Button>
      </View>
      <Code>{themeJson()}</Code>
    </View>
  );
}

function ScopeDemo(props: { code: string; children?: JSX.Element }) {
  return (
    <View class="flex-1 min-w-80 rounded-xl border border-border overflow-hidden">
      <View class="p-4">{props.children}</View>
      <Code>{props.code}</Code>
    </View>
  );
}
