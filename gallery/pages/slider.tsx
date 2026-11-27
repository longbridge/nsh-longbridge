// The Slider page: a sound and display panel, a price range, a price
// filter with its histogram and fields, marks, value bubbles, steps,
// vertical sliders, states.

import { signal, Text, View } from "nsh";
import { Input, Label, Slider } from "nsh-longbridge";
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

// How many of the stays cost each $25 of a night, from $0 to $1000: most
// between $100 and $300, a long tail of the dear ones.
const nightly = [
  2, 5, 9, 16, 27, 38, 46, 51, 49, 44, 38, 33, 28, 24, 20, 17, 14, 12, 10, 9,
  8, 7, 6, 5, 5, 4, 4, 3, 3, 3, 2, 2, 2, 1, 1, 2, 1, 1, 0, 3,
];
const stays = nightly.reduce((a, b) => a + b, 0);

/** The stays between `lo` and `hi`, a bucket's count taken as spread evenly over it. */
function staysIn(lo: number, hi: number) {
  const width = 1000 / nightly.length;
  return Math.round(
    nightly.reduce((sum, n, i) => {
      const from = i * width;
      const covered = Math.max(0, Math.min(hi, from + width) - Math.max(lo, from));
      return sum + (n * covered) / width;
    }, 0),
  );
}

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

      <Section
        title="A price filter"
        description="histogram draws how many results fall at each price above the track; the bars between the thumbs take the range's color, split exactly where the thumbs are, as they move. The fields below take an exact price: they follow the thumbs, and a typed price moves its thumb, held to the bounds and to the other end on Return or when the field is left."
      >
        <Demo>
          <PriceFilterDemo />
        </Demo>
      </Section>

      <Section
        title="Marks"
        description="Ticks below the track, with labels where given; a step on the marks makes the thumb stop on them. The labels at the ends line up with the ends instead of overflowing."
      >
        <Demos>
          <Demo label="Labelled stops: step 25">
            <VolumeMarksDemo />
          </Demo>
          <Demo label="A tick per step, labels at the ends">
            <TextSizeDemo />
          </Demo>
        </Demos>
      </Section>

      <Section
        title="Value bubble"
        description="With bubble auto, the value shows above the thumb under the pointer, while it is dragged and while the keyboard focus is on it (not after a click); formatValue gives its text. A range's two bubbles become one when they would touch."
      >
        <Demos>
          <Demo label="auto, with a percent">
            <OpacityDemo />
          </Demo>
          <Demo label="always, a range of hours">
            <HoursDemo />
          </Demo>
        </Demos>
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
          <Demo label="Disabled with a histogram and marks: all dimmed">
            <View class="w-full max-w-[280px]">
              <Slider
                disabled
                min={0}
                max={1000}
                step={10}
                value={[150, 600]}
                histogram={nightly}
                marks={[{ value: 0, label: "$0" }, 500, { value: 1000, label: "$1,000" }]}
                startAccessibilityLabel="Minimum price"
                accessibilityLabel="Maximum price"
              />
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
          <Text class="text-sm tabular-nums text-muted-foreground">{`${volume()}%`}</Text>
        </View>
        <Slider value={volume()} onChange={(e) => volume.set(e.value as number)} accessibilityLabel="Volume" />
      </View>
      <View class="gap-3">
        <View class="flex-row items-center gap-2">
          <Glyph icon={Sun} size={16} class="text-muted-foreground" />
          <Text class="flex-1 text-sm font-medium text-card-foreground">Brightness</Text>
          <Text class="text-sm tabular-nums text-muted-foreground">{`${brightness()}%`}</Text>
        </View>
        <Slider value={brightness()} onChange={(e) => brightness.set(e.value as number)} accessibilityLabel="Brightness" />
      </View>
      <View class="gap-3">
        <View class="flex-row items-center justify-between">
          <Text class="text-sm font-medium text-card-foreground">Balance</Text>
          <Text class="text-sm tabular-nums text-muted-foreground">
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
        <Text class="text-sm tabular-nums text-muted-foreground">{`$${price()[0]} – $${price()[1]}`}</Text>
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
        <Text class="text-sm tabular-nums text-muted-foreground">{`${size()} pt, saved ${saved()}`}</Text>
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
      <Text class="text-sm tabular-nums text-muted-foreground">{`${gain() > 0 ? "+" : ""}${gain()}`}</Text>
      <View class="h-48">
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

/** A price filter: the stays' spread above a range, its ends as fields kept in step with the thumbs. */
function PriceFilterDemo() {
  const range = signal<[number, number]>([100, 450]);
  const minText = signal("100");
  const maxText = signal("450");
  const show = (v: [number, number]) => {
    range.set(v);
    minText.set(String(v[0]));
    maxText.set(String(v[1]));
  };
  const parse = (text: string) => {
    const n = Number(text.replace(/[^\d.]/g, ""));
    return text.trim() !== "" && Number.isFinite(n) ? n : null;
  };
  // A typed end moves its thumb while it is a number within reach; on
  // Return or leaving the field it is held to the bounds and the other end,
  // and written back as the slider shows it.
  const typed = (end: 0 | 1, text: string, commit: boolean) => {
    const n = parse(text);
    const [lo, hi] = range();
    if (n === null) {
      if (commit) show([lo, hi]);
      return;
    }
    const at = Math.round(n / 10) * 10;
    const next: [number, number] =
      end === 0 ? [Math.min(Math.max(at, 0), hi), hi] : [lo, Math.max(Math.min(at, 1000), lo)];
    if (commit) show(next);
    else if (end === 0 ? n >= 0 && n <= hi : n >= lo && n <= 1000) range.set(next);
  };
  return (
    <View class="w-full max-w-[400px] gap-5 rounded-xl border border-border bg-card p-6">
      <View class="gap-1">
        <Text class="text-base font-semibold text-card-foreground">Price range</Text>
        <Text class="text-sm text-muted-foreground">
          {`Nightly prices, fees included · ${staysIn(range()[0], range()[1])} of ${stays} stays`}
        </Text>
      </View>
      <Slider
        min={0}
        max={1000}
        step={10}
        histogram={nightly}
        value={range()}
        onChange={(e) => show(e.value as [number, number])}
        startAccessibilityLabel="Minimum price"
        accessibilityLabel="Maximum price"
      />
      <View class="flex-row items-end gap-3">
        <View class="flex-1 gap-1.5">
          <Label>
            <Text>Minimum ($)</Text>
          </Label>
          <Input
            value={minText()}
            keyboardType="number-pad"
            onChange={(e) => {
              minText.set(e.value);
              typed(0, e.value, false);
            }}
            onSubmit={(e) => typed(0, e.value, true)}
            onBlur={() => typed(0, minText(), true)}
            accessibilityLabel="Minimum price"
          />
        </View>
        <Text class="pb-2 text-muted-foreground">–</Text>
        <View class="flex-1 gap-1.5">
          <Label>
            <Text>Maximum ($)</Text>
          </Label>
          <Input
            value={maxText()}
            keyboardType="number-pad"
            onChange={(e) => {
              maxText.set(e.value);
              typed(1, e.value, false);
            }}
            onSubmit={(e) => typed(1, e.value, true)}
            onBlur={() => typed(1, maxText(), true)}
            accessibilityLabel="Maximum price"
          />
        </View>
      </View>
    </View>
  );
}

function VolumeMarksDemo() {
  const volume = signal(50);
  return (
    <View class="w-full max-w-[320px] gap-3">
      <View class="flex-row items-center justify-between">
        <Text class="text-sm font-medium text-foreground">Alert volume</Text>
        <Text class="text-sm tabular-nums text-muted-foreground">{`${volume()}%`}</Text>
      </View>
      <Slider
        step={25}
        value={volume()}
        onChange={(e) => volume.set(e.value as number)}
        marks={[
          { value: 0, label: "0" },
          { value: 25, label: "25" },
          { value: 50, label: "50" },
          { value: 75, label: "75" },
          { value: 100, label: "100" },
        ]}
        accessibilityLabel="Alert volume"
      />
    </View>
  );
}

function TextSizeDemo() {
  const size = signal(16);
  return (
    <View class="w-full max-w-[320px] gap-3">
      <View class="flex-row items-center justify-between">
        <Text class="text-sm font-medium text-foreground">Text size</Text>
        <Text class="text-sm tabular-nums text-muted-foreground">{`${size()} pt`}</Text>
      </View>
      <Slider
        min={12}
        max={24}
        step={2}
        value={size()}
        onChange={(e) => size.set(e.value as number)}
        marks={[{ value: 12, label: "Smaller" }, 14, 16, 18, 20, 22, { value: 24, label: "Larger" }]}
        accessibilityLabel="Text size"
      />
    </View>
  );
}

function OpacityDemo() {
  return (
    <View class="w-full max-w-[320px] gap-3 pt-8">
      <Text class="text-sm font-medium text-foreground">Layer opacity</Text>
      <Slider bubble="auto" formatValue={(v) => `${v}%`} defaultValue={80} accessibilityLabel="Layer opacity" />
    </View>
  );
}

function HoursDemo() {
  const hours = signal<[number, number]>([9, 17]);
  return (
    <View class="w-full max-w-[320px] gap-3 pt-8">
      <Slider
        min={0}
        max={24}
        bubble="always"
        formatValue={(v) => `${String(v).padStart(2, "0")}:00`}
        value={hours()}
        onChange={(e) => hours.set(e.value as [number, number])}
        marks={[{ value: 0, label: "00:00" }, 6, { value: 12, label: "12:00" }, 18, { value: 24, label: "24:00" }]}
        startAccessibilityLabel="From"
        accessibilityLabel="Until"
      />
    </View>
  );
}
