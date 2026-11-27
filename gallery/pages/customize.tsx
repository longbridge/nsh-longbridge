// The theme editor: presets, the seeds, every semantic color, a live
// preview of the showcase, a Copy theme button, and ThemeScope.

import { Clipboard, signal, Text, View, type JSX } from "nsh";
import { Badge, Button, Separator, ThemeScope } from "nsh-longbridge";
import {
  colorGroups,
  colorPreset,
  colorPresets,
  cssColor,
  densityPreset,
  densityPresets,
  editCount,
  formatValue,
  isEdited,
  numberOf,
  presetPrimary,
  resetAll,
  resetToken,
  scheme,
  sectionOf,
  seeds,
  setSeed,
  setToken,
  themeJson,
} from "../theme";
import { ColorEdit } from "../ui/color-edit";
import { Segmented, Slider } from "../ui/controls";
import { Check, Copy, Glyph, RotateCcw } from "../ui/icons";
import { Demo, Demos, Section } from "../ui/layout";
import { compact, contentWidth } from "../ui/screen";
import { NotificationsCard, PlanCard, Showcase } from "../ui/showcase";

/** The colors a scope demo sets: a green brand and square corners. */
const brandScope = {
  primary: "oklch(0.596 0.145 163.225)",
  "primary-foreground": "oklch(0.985 0 0)",
  ring: "oklch(0.696 0.17 162.48)",
  radius: "2px",
};

export function CustomizePage() {
  const editorWidth = 340;
  const previewColumns = () => {
    const width = compact() ? contentWidth(true) : contentWidth(true) - editorWidth - 40;
    return width >= 620 ? 2 : 1;
  };
  return (
    <View class="gap-16">
      <View class="gap-3">
        <Text class="text-sm font-medium text-muted-foreground">Overview</Text>
        <Text class="text-3xl font-semibold tracking-tight text-foreground">Customize</Text>
        <Text class="max-w-[640px] text-base leading-7 text-muted-foreground">
          Start from a preset, move the seeds, pick any color. Everything in the gallery follows at once; copy the theme
          when it looks right and name the file in your app's nsh.json.
        </Text>
      </View>

      <View class={compact() ? "gap-10" : "flex-row items-start gap-10"}>
        <View class="gap-8" style={compact() ? undefined : { width: editorWidth }}>
          <Toolbar />
          <EditorGroup title="Preset">
            <Presets />
          </EditorGroup>
          <EditorGroup title="Seeds" note="Each one moves every component in proportion.">
            <Seeds />
          </EditorGroup>
          <EditorGroup title="Colors" note={`The ${scheme()} appearance; switch it in the header to edit the other.`}>
            <Colors />
          </EditorGroup>
        </View>
        <View class="flex-1 gap-3">
          <Text class="text-xs font-medium text-muted-foreground">Preview</Text>
          <Showcase columns={previewColumns()} />
        </View>
      </View>

      <Section
        title="ThemeScope"
        description="Tokens for one part of a screen. The library's components inside take the scope's values, and what derives from them follows; scopes nest, the inner one winning."
      >
        <Demos>
          <Demo label="The dark appearance, with its own surface" class="p-6">
            <ThemeScope appearance="dark" surface class="w-full max-w-[360px] rounded-xl p-4">
              <NotificationsCard />
            </ThemeScope>
          </Demo>
          <Demo label="A green primary and a radius of 2, nothing else" class="p-6">
            <ThemeScope tokens={brandScope} class="w-full max-w-[360px]">
              <PlanCard />
            </ThemeScope>
          </Demo>
        </Demos>
      </Section>
    </View>
  );
}

function EditorGroup(props: { title: string; note?: string; children?: JSX.Element }) {
  return (
    <View class="gap-4">
      <View class="gap-1">
        <Text class="text-sm font-semibold text-foreground">{props.title}</Text>
        <If when={props.note}>
          <Text class="text-xs leading-5 text-muted-foreground">{props.note!}</Text>
        </If>
      </View>
      {props.children}
    </View>
  );
}

/** Copy theme and Reset, with the count of changes. */
function Toolbar() {
  const copied = signal(false);
  const copy = async () => {
    await Clipboard.writeText(themeJson());
    copied.set(true);
    setTimeout(() => copied.set(false), 1600);
  };
  return (
    <View class="gap-3 rounded-xl border border-border bg-card p-4">
      <View class="flex-row items-center justify-between">
        <Text class="text-sm font-medium text-card-foreground">my-theme.json</Text>
        <Badge variant={editCount() > 0 ? "default" : "secondary"}>
          <Text>{editCount() === 0 ? "No changes" : `${editCount()} ${editCount() === 1 ? "change" : "changes"}`}</Text>
        </Badge>
      </View>
      <Text class="text-xs leading-5 text-muted-foreground">
        The presets and your changes, as a theme file. It goes to the clipboard.
      </Text>
      <View class="flex-row gap-2">
        <Button class="flex-1" onPress={copy}>
          <Glyph icon={copied() ? Check : Copy} size={16} />
          <Text>{copied() ? "Copied" : "Copy theme"}</Text>
        </Button>
        <Button variant="outline" disabled={editCount() === 0} onPress={resetAll}>
          <Glyph icon={RotateCcw} size={14} />
          <Text>Reset</Text>
        </Button>
      </View>
    </View>
  );
}

function Presets() {
  return (
    <View class="gap-3">
      <View class="flex-row gap-2">
        {colorPresets.map((p) => (
          <Button
            variant="outline"
            class={`flex-1 ${colorPreset() === p.id ? "border-ring" : ""}`}
            onPress={() => colorPreset.set(p.id)}
            accessibilityLabel={`${p.label} preset`}
          >
            <View
              class="size-3.5 rounded-full border border-border"
              style={{ backgroundColor: presetPrimary(p.id) }}
            />
            <Text>{p.label}</Text>
          </Button>
        ))}
      </View>
      <Segmented
        accessibilityLabel="Density"
        class="self-start"
        options={densityPresets.map((p) => ({ id: p.id, label: p.label }))}
        value={densityPreset}
        onChange={(id) => densityPreset.set(id)}
      />
    </View>
  );
}

function Seeds() {
  return (
    <View class="gap-5">
      {seeds.map((s) => (
        <View class="gap-2.5">
          <View class="flex-row items-center justify-between">
            <View class="flex-row items-center gap-2">
              <Text class="text-sm text-foreground">{s.label}</Text>
              <If when={isEdited(s.name)}>
                <Button
                  variant="ghost"
                  size="xs"
                  class="h-5 w-5 px-0"
                  accessibilityLabel={`Reset ${s.label}`}
                  onPress={() => resetToken(s.name)}
                >
                  <Glyph icon={RotateCcw} size={12} />
                </Button>
              </If>
            </View>
            <Text class="font-mono text-xs tabular-nums text-muted-foreground">{formatValue(s.name)}</Text>
          </View>
          <Slider
            accessibilityLabel={s.label}
            min={s.min}
            max={s.max}
            step={s.step}
            value={() => numberOf(s.name)}
            onChange={(v) => setSeed(s.name, v)}
          />
        </View>
      ))}
    </View>
  );
}

function Colors() {
  return (
    <View class="gap-6">
      {colorGroups.map((g) => (
        <View class="gap-2">
          <Text class="text-xs font-medium text-muted-foreground">{g.title}</Text>
          <View class="rounded-lg border border-border">
            {g.pairs.map(([fill, text], i) => (
              <View>
                <If when={i > 0}>
                  <Separator />
                </If>
                <ColorRow fill={fill} text={text} />
              </View>
            ))}
          </View>
        </View>
      ))}
    </View>
  );
}

/** A color and the text that goes on it: a swatch each, the color's name and value. */
function ColorRow(props: { fill: string; text: string | null }) {
  const names = props.text ? [props.fill, props.text] : [props.fill];
  const edited = () => names.some(isEdited);
  return (
    <View class="h-11 flex-row items-center gap-3 px-3">
      <View class="flex-row gap-1.5">
        {names.map((name) => (
          <ColorEdit name={name} onChange={(v) => setToken(sectionOf(name), name, v)} />
        ))}
        <If when={!props.text}>
          <View class="size-6" />
        </If>
      </View>
      <View class="flex-1 flex-row items-baseline gap-1.5">
        <Text class="truncate font-mono text-xs text-foreground">{props.fill}</Text>
        <If when={props.text}>
          <Text class="font-mono text-xs text-muted-foreground">+ text</Text>
        </If>
      </View>
      <If when={edited()}>
        <Button
          variant="ghost"
          size="xs"
          class="h-5 w-5 px-0"
          accessibilityLabel={`Reset ${props.fill}`}
          onPress={() => names.forEach(resetToken)}
        >
          <Glyph icon={RotateCcw} size={12} />
        </Button>
      </If>
      <Text class="font-mono text-xs tabular-nums text-muted-foreground">{formatValue(props.fill)}</Text>
    </View>
  );
}
