// A component page's Tokens section: the component's tokens from the
// registry, each with its derived default, its value now (a swatch for a
// color) and a way to change it. A change goes into the app's theme as that
// component's token, so only that kind of component follows.

import { Text, View } from "nsh";
import { Badge, Button } from "nsh-longbridge";
import {
  componentTokens,
  derived,
  formatDerived,
  formatValue,
  isEdited,
  kindOf,
  numberOf,
  resetToken,
  resetTokens,
  scheme,
  sectionOf,
  setToken,
} from "../theme";
import { ColorEdit } from "./color-edit";
import { Stepper } from "./controls";
import { Glyph, RotateCcw } from "./icons";
import { Section } from "./layout";
import { compact } from "./screen";

/** The range and step a number token's stepper takes. */
function rangeOf(name: string): { min: number; max: number; step: number; format?: (v: number) => string } {
  const kind = kindOf(name);
  if (kind === "tier") return { min: 0, max: 4, step: 1, format: (v) => ["none", "xs", "sm", "md", "lg"][v] ?? String(v) };
  if (kind === "ms" && name === "toast-duration") return { min: 0, max: 10000, step: 500 };
  if (kind === "ms") return { min: 0, max: 2000, step: 50 };
  if (/opacity/.test(name)) return { min: 0, max: 1, step: 0.1 };
  if (/animate/.test(name)) return { min: 0, max: 1, step: 1, format: (v) => (v ? "on" : "off") };
  if (/weight/.test(name)) return { min: 100, max: 900, step: 100 };
  if (/max-width/.test(name)) return { min: 120, max: 640, step: 20 };
  if (/thickness|border-width|ring-width/.test(name)) return { min: 0, max: 6, step: 0.5 };
  // A popup's or the toast stack's width.
  if (/^(popover|toast)-width$/.test(name)) return { min: 160, max: 480, step: 4 };
  if (/radius/.test(name)) return { min: 0, max: 9999, step: 1, format: (v) => (v >= 999 ? "full" : String(v)) };
  return { min: 0, max: 96, step: 1 };
}

function NumberEdit(props: { name: string }) {
  const range = rangeOf(props.name);
  const kind = kindOf(props.name);
  const write = (v: number) => {
    // A full radius steps down to 24 and back up past 32.
    const current = numberOf(props.name);
    let next = v;
    if (/radius/.test(props.name)) {
      if (current >= 999 && v < current) next = 24;
      else if (v > 32) next = 9999;
    }
    setToken("theme", props.name, kind === "length" ? `${next}px` : String(next));
  };
  return (
    <Stepper
      name={props.name}
      value={() => Number(numberOf(props.name).toFixed(2))}
      min={range.min}
      max={range.max}
      step={range.step}
      format={range.format}
      onChange={write}
    />
  );
}

function TokenRow(props: { name: string; last: boolean }) {
  const kind = kindOf(props.name);
  return (
    <View class={`min-h-12 flex-row items-center gap-4 px-4 py-2 ${props.last ? "" : "border-b border-border"}`}>
      <View class="flex-1 gap-0.5">
        <View class="flex-row items-center gap-2">
          <Text class="font-mono text-xs text-foreground">
            {props.name}
          </Text>
          <If when={isEdited(props.name)}>
            <Badge variant="secondary">
              <Text>Edited</Text>
            </Badge>
          </If>
        </View>
        <If when={compact()}>
          <Text class="font-mono text-xs text-muted-foreground">{formatDerived(derived(props.name, scheme()))}</Text>
        </If>
      </View>
      <If when={!compact()}>
        <Text class="w-56 font-mono text-xs text-muted-foreground">{formatDerived(derived(props.name, scheme()))}</Text>
      </If>
      <View class="w-44 flex-row items-center justify-end gap-2">
        {kind === "color" ? (
          <>
            <Text class="font-mono text-xs tabular-nums text-muted-foreground">{formatValue(props.name)}</Text>
            <ColorEdit name={props.name} onChange={(v) => setToken(sectionOf(props.name), props.name, v)} />
          </>
        ) : (
          <NumberEdit name={props.name} />
        )}
      </View>
      <View class="w-7 items-center">
        <If when={isEdited(props.name)}>
          <Button
            variant="ghost"
            size="xs"
            class="w-6 px-0"
            accessibilityLabel={`Reset ${props.name}`}
            onPress={() => resetToken(props.name)}
          >
            <Glyph icon={RotateCcw} size={12} />
          </Button>
        </If>
      </View>
    </View>
  );
}

function TokenTable(props: { title: string; names: string[] }) {
  return (
    <View class="gap-2.5">
      <Text class="text-sm font-medium text-foreground">{props.title}</Text>
      <View class="rounded-xl border border-border overflow-hidden">
        {props.names.map((name, i) => (
          <TokenRow name={name} last={i === props.names.length - 1} />
        ))}
      </View>
    </View>
  );
}

/**
 * The Tokens section of component `prefix` (`button`, `radio-group`).
 * `component` names it in the text; `note` says what it shares.
 */
export function TokensSection(props: { prefix: string; component: string; note?: string; extra?: string[] }) {
  const names = [...componentTokens(props.prefix), ...(props.extra ?? [])];
  const colors = names.filter((n) => kindOf(n) === "color");
  const others = names.filter((n) => kindOf(n) !== "color");
  const edited = () => names.filter(isEdited);
  return (
    <Section
      title="Tokens"
      description={names.length === 0 ? undefined : `Each ${props.component} token, derived from the semantic tokens, with its value in this theme and appearance. Change one: it goes into the theme as this component's token, so only ${props.component} follows while everything else keeps the token it derives from.${
        props.note ? ` ${props.note}` : ""
      }`}
    >
      <If when={names.length === 0}>
        <View class="rounded-xl border border-border px-5 py-4">
          <Text class="text-sm text-muted-foreground">{`${props.component} ${props.note ?? "has no tokens of its own."}`}</Text>
        </View>
      </If>
      <If when={edited().length > 0}>
        <View class="flex-row items-center justify-between rounded-lg bg-muted px-4 py-2.5">
          <Text class="text-sm text-foreground">
            {`${edited().length} ${edited().length === 1 ? "token" : "tokens"} changed. The scenes above follow.`}
          </Text>
          <Button size="xs" variant="outline" onPress={() => resetTokens(names)}>
            <Text>Reset</Text>
          </Button>
        </View>
      </If>
      <If when={colors.length > 0}>
        <TokenTable title="Colors" names={colors} />
      </If>
      <If when={others.length > 0}>
        <TokenTable title="Sizes and numbers" names={others} />
      </If>
    </Section>
  );
}
