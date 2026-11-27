// The Slider page: a sound and display panel, a price range, steps,
// vertical sliders, states.

import { signal, Text, View } from "nsh";
import { Slider } from "nsh-longbridge";
import { Glyph, Sun, Volume2 } from "../ui/icons";
import { Demo, Demos, Page, Section } from "../ui/layout";
import { TokensSection } from "../ui/tokens";

const bands = [
  { label: "60", value: 4 },
  { label: "250", value: 2 },
  { label: "1k", value: 0 },
  { label: "4k", value: -2 },
  { label: "16k", value: 3 },
];

const qualities = ["Draft", "Good", "Better", "Best", "Lossless"];

export function SliderPage() {
  return (
    <Page
      eyebrow="Components"
      title="Slider"
      description="A number from a range, picked by dragging: a volume, a size, a price range with two thumbs. Arrow keys step it; on touch screens the thumb grows and the whole height takes presses."
    >
      <Demo hero>
        <PanelDemo />
      </Demo>

      <Section title="A range" description="A pair as the value makes two thumbs; each has its own name for screen readers, and they cannot cross.">
        <Demo>
          <PriceDemo />
        </Demo>
      </Section>

      <Section title="Steps" description="Values snap to the step; onChangeEnd reports once when the drag or the key ends, the time to save.">
        <Demos>
          <Demo>
            <QualityDemo />
          </Demo>
          <Demo>
            <SavedDemo />
          </Demo>
        </Demos>
      </Section>

      <Section title="Vertical" description="As tall as its parent, at least slider-vertical-min-height; the rail runs up from the bottom.">
        <Demo>
          <EqualizerDemo />
        </Demo>
      </Section>

      <Section title="States" description="Disabled dims it and takes no drags or keys. Hover or Tab rings the thumb.">
        <Demos>
          <Demo label="Disabled">
            <View class="w-full max-w-[280px]">
              <Slider disabled value={40} accessibilityLabel="Locked" />
            </View>
          </Demo>
          <Demo label="A quieter range: the app's classes on the parts">
            <View class="w-full max-w-[280px]">
              <Slider
                defaultValue={65}
                rangeClass="bg-muted-foreground"
                thumbClass="border-muted-foreground"
                accessibilityLabel="Battery limit"
              />
            </View>
          </Demo>
        </Demos>
      </Section>

      <TokensSection prefix="slider" component="Slider" />
    </Page>
  );
}

function PanelDemo() {
  const volume = signal(62);
  const brightness = signal(80);
  const balance = signal(0);
  return (
    <View class="w-full max-w-[400px] gap-6 rounded-xl border border-border bg-card p-6">
      <View class="gap-1">
        <Text class="text-base font-semibold text-card-foreground">Sound and display</Text>
        <Text class="text-sm text-muted-foreground">Changes apply as you drag.</Text>
      </View>
      <View class="gap-3">
        <View class="flex-row items-center gap-2">
          <Glyph icon={Volume2} size={16} class="text-muted-foreground" />
          <Text class="flex-1 text-sm font-medium text-card-foreground">Volume</Text>
          <Text class="font-mono text-xs tabular-nums text-muted-foreground">{`${volume()}%`}</Text>
        </View>
        <Slider value={volume()} onChange={(e) => volume.set(e.value as number)} accessibilityLabel="Volume" />
      </View>
      <View class="gap-3">
        <View class="flex-row items-center gap-2">
          <Glyph icon={Sun} size={16} class="text-muted-foreground" />
          <Text class="flex-1 text-sm font-medium text-card-foreground">Brightness</Text>
          <Text class="font-mono text-xs tabular-nums text-muted-foreground">{`${brightness()}%`}</Text>
        </View>
        <Slider value={brightness()} onChange={(e) => brightness.set(e.value as number)} accessibilityLabel="Brightness" />
      </View>
      <View class="gap-3">
        <View class="flex-row items-center justify-between">
          <Text class="text-sm font-medium text-card-foreground">Balance</Text>
          <Text class="font-mono text-xs tabular-nums text-muted-foreground">
            {balance() === 0 ? "Center" : balance() < 0 ? `L ${-balance()}` : `R ${balance()}`}
          </Text>
        </View>
        <Slider
          min={-10}
          max={10}
          value={balance()}
          onChange={(e) => balance.set(e.value as number)}
          accessibilityLabel="Balance"
        />
      </View>
    </View>
  );
}

function PriceDemo() {
  const price = signal<[number, number]>([200, 750]);
  return (
    <View class="w-full max-w-[360px] gap-3">
      <View class="flex-row items-center justify-between">
        <Text class="text-sm font-medium text-foreground">Price</Text>
        <Text class="font-mono text-sm tabular-nums text-muted-foreground">{`$${price()[0]} – $${price()[1]}`}</Text>
      </View>
      <Slider
        min={0}
        max={1000}
        step={50}
        value={price()}
        onChange={(e) => price.set(e.value as [number, number])}
        startAccessibilityLabel="Minimum price"
        accessibilityLabel="Maximum price"
      />
      <View class="flex-row justify-between">
        <Text class="text-xs text-muted-foreground">$0</Text>
        <Text class="text-xs text-muted-foreground">$1000</Text>
      </View>
    </View>
  );
}

function QualityDemo() {
  const quality = signal(2);
  return (
    <View class="w-full max-w-[280px] gap-3">
      <View class="flex-row items-center justify-between">
        <Text class="text-sm font-medium text-foreground">Export quality</Text>
        <Text class="text-sm text-muted-foreground">{qualities[quality()]}</Text>
      </View>
      <Slider
        max={qualities.length - 1}
        step={1}
        value={quality()}
        onChange={(e) => quality.set(e.value as number)}
        accessibilityLabel="Export quality"
      />
    </View>
  );
}

function SavedDemo() {
  const size = signal(14);
  const saved = signal(14);
  return (
    <View class="w-full max-w-[280px] gap-3">
      <View class="flex-row items-center justify-between">
        <Text class="text-sm font-medium text-foreground">Font size</Text>
        <Text class="font-mono text-xs tabular-nums text-muted-foreground">{`${size()} pt, saved ${saved()}`}</Text>
      </View>
      <Slider
        min={10}
        max={24}
        step={2}
        value={size()}
        onChange={(e) => size.set(e.value as number)}
        onChangeEnd={(e) => saved.set(e.value as number)}
        accessibilityLabel="Font size"
      />
    </View>
  );
}

function EqualizerDemo() {
  return (
    <View class="flex-row items-end gap-8">
      {bands.map((b) => (
        <Band label={b.label} value={b.value} />
      ))}
    </View>
  );
}

function Band(props: { label: string; value: number }) {
  const gain = signal(props.value);
  return (
    <View class="items-center gap-2">
      <Text class="font-mono text-xs tabular-nums text-muted-foreground">{`${gain() > 0 ? "+" : ""}${gain()}`}</Text>
      <View class="h-40">
        <Slider
          axis="vertical"
          min={-6}
          max={6}
          value={gain()}
          onChange={(e) => gain.set(e.value as number)}
          accessibilityLabel={`${props.label} Hz`}
        />
      </View>
      <Text class="text-xs text-muted-foreground">{props.label}</Text>
    </View>
  );
}
