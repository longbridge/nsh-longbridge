// Gallery-local controls the library does not have yet, each the smallest
// version built on its `nsh/base` counterpart and drawn with theme tokens
// only: a segmented control (`ToggleGroup`), a select (`Select`), a slider
// (`Slider`) and a stepper. Candidates for the library.

import { Text, View, type JSX } from "nsh";
import { Select as BaseSelect, Slider as BaseSlider, Toggle, ToggleGroup } from "nsh/base";
import { Button } from "nsh-longbridge";
import { numberOf } from "../theme";
import { Glyph } from "./icons";
import { Check as CheckIcon, ChevronDown } from "nsh-icon-lucide";

export interface SegmentOption {
  id: string;
  label?: string;
  icon?: string;
  /** What screen readers say when the option shows an icon alone. */
  name?: string;
}

/** One of a few options, as a segmented tabs list: a muted track, the chosen one raised. */
export function Segmented(props: {
  options: SegmentOption[];
  value: () => string;
  onChange: (id: string) => void;
  accessibilityLabel: string;
  class?: string;
}) {
  const height = () => numberOf("control-height-sm");
  return (
    <ToggleGroup
      class={`flex-row items-center rounded-lg bg-muted p-[3px] ${props.class ?? ""}`}
      style={{ height: height() }}
    >
      <Each of={props.options}>
        {(option) => (
          <Toggle
            pressed={props.value() === option().id}
            onChange={() => props.onChange(option().id)}
            accessibilityLabel={option().name ?? option().label}
            class={`group h-full flex-row items-center justify-center gap-1.5 rounded-md border border-transparent ${
              option().label ? "px-2.5" : "aspect-square"
            } text-muted-foreground hover:text-foreground aria-pressed:bg-background aria-pressed:text-foreground aria-pressed:shadow-sm dark:aria-pressed:border-input dark:aria-pressed:bg-input/30 focus-visible:ring-[3px] focus-visible:ring-ring/50 transition-colors`}
          >
            <If when={option().icon}>
              <Glyph icon={option().icon!} size={14} />
            </If>
            <If when={option().label}>
              <Text class="text-sm font-medium">{option().label!}</Text>
            </If>
          </Toggle>
        )}
      </Each>
    </ToggleGroup>
  );
}

/** A choice from a short list, as a select: an outline trigger and a popover list. */
export function Select(props: {
  items: { value: string; title: string }[];
  value: () => string;
  onChange: (value: string) => void;
  accessibilityLabel: string;
  leading?: (value: string) => JSX.Element;
  class?: string;
}) {
  const title = () => props.items.find((i) => i.value === props.value())?.title ?? "";
  return (
    <BaseSelect
      items={props.items}
      value={props.value()}
      onChange={(e) => e.value && props.onChange(e.value)}
      accessibilityLabel={props.accessibilityLabel}
      cover={false}
      offset={4}
      popupClass="min-w-36 rounded-md border border-border bg-popover p-1 shadow-md"
      itemClass="group h-8 flex-row items-center gap-2 rounded-sm px-2 focus:bg-accent"
      itemStyle={{ fontSize: 14 }}
      renderItem={(entry) => (
        <View class="flex-1 flex-row items-center gap-2">
          {props.leading?.(entry.value)}
          <Text class="flex-1 text-sm text-popover-foreground group-focus:text-accent-foreground">{entry.title}</Text>
          <If when={entry.value === props.value()}>
            <Glyph icon={CheckIcon} size={14} class="text-popover-foreground" />
          </If>
        </View>
      )}
      trigger={
        <View
          class={`flex-row items-center gap-2 rounded-md border border-input bg-background px-3 shadow-xs dark:bg-input/30 focus-visible:border-ring focus-visible:ring-[3px] focus-visible:ring-ring/50 ${
            props.class ?? ""
          }`}
          style={{ height: numberOf("control-height-sm") }}
        >
          {props.leading?.(props.value())}
          <Text class="text-sm text-foreground">{title()}</Text>
          <Glyph icon={ChevronDown} size={14} class="text-muted-foreground opacity-70" />
        </View>
      }
    />
  );
}

/** A number on a track, as a slider. */

export function Slider(props: {
  value: () => number;
  min: number;
  max: number;
  step: number;
  onChange: (value: number) => void;
  accessibilityLabel: string;
  class?: string;
}) {
  return (
    <BaseSlider
      class={props.class}
      min={props.min}
      max={props.max}
      step={props.step}
      value={props.value()}
      accessibilityLabel={props.accessibilityLabel}
      trackClass="h-5 justify-center"
      indicatorClass="h-1.5 rounded-full bg-muted"
      rangeClass="rounded-full bg-primary"
      thumbClass="size-4 rounded-full border border-primary bg-background shadow-sm hover:ring-4 hover:ring-ring/50 focus-visible:ring-4 focus-visible:ring-ring/50 transition-shadow"
      onChange={(e) => props.onChange(e.value as number)}
    />
  );
}

/** A number changed a step at a time, with the library's buttons. */
export function Stepper(props: {
  value: () => number;
  min: number;
  max: number;
  step: number;
  onChange: (value: number) => void;
  name: string;
  format?: (value: number) => string;
}) {
  const set = (v: number) => props.onChange(Math.min(props.max, Math.max(props.min, Number(v.toFixed(2)))));
  return (
    <View class="flex-row items-center gap-1">
      <Button
        size="xs"
        variant="outline"
        iconOnly
        icon="minus"
        accessibilityLabel={`Decrease ${props.name}`}
        disabled={props.value() <= props.min}
        onPress={() => set(props.value() - props.step)}
      />
      <Text class="min-w-12 text-center font-mono text-xs tabular-nums text-foreground">
        {props.format ? props.format(props.value()) : String(props.value())}
      </Text>
      <Button
        size="xs"
        variant="outline"
        iconOnly
        icon="plus"
        accessibilityLabel={`Increase ${props.name}`}
        disabled={props.value() >= props.max}
        onPress={() => set(props.value() + props.step)}
      />
    </View>
  );
}
