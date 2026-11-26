// The gallery's layout pieces. Only theme tokens in classes: the gallery
// looks like the library it shows.

import { Text, View } from "nsh";
import { Button } from "nsh-longbridge";

export function Page(props: { title: string; description: string; children?: JSX.Element }) {
  return (
    <View class="gap-10">
      <View class="gap-2">
        <Text class="text-3xl font-semibold text-foreground">{props.title}</Text>
        <Text class="text-base text-muted-foreground">{props.description}</Text>
      </View>
      {props.children}
    </View>
  );
}

export function Section(props: { title: string; description?: string; children?: JSX.Element }) {
  return (
    <View class="gap-4">
      <View class="gap-1">
        <Text class="text-xl font-semibold text-foreground">{props.title}</Text>
        <If when={props.description}>
          <Text class="text-sm text-muted-foreground">{props.description}</Text>
        </If>
      </View>
      {props.children}
    </View>
  );
}

/** A demo over its code. */
export function Preview(props: { code?: string; class?: string; children?: JSX.Element }) {
  return (
    <View class="rounded-xl border border-border overflow-hidden">
      <View class={`p-8 flex-row flex-wrap items-center justify-center gap-3 bg-background ${props.class ?? ""}`}>
        {props.children}
      </View>
      <If when={props.code}>
        <Code>{props.code!}</Code>
      </If>
    </View>
  );
}

export function Code(props: { children: string; class?: string }) {
  return (
    <View selectable class={`border-t border-border bg-muted/50 px-5 py-4 ${props.class ?? ""}`}>
      <Text class="font-mono text-xs leading-5 text-foreground">{props.children}</Text>
    </View>
  );
}

/** One of a few options, as a row of small buttons. */
export function Segmented(props: {
  label: string;
  options: { id: string; label: string }[];
  value: () => string;
  onChange: (id: string) => void;
}) {
  return (
    <View class="flex-row items-center gap-2">
      <Text class="text-xs text-muted-foreground">{props.label}</Text>
      <View class="flex-row gap-0.5 rounded-lg bg-muted p-0.5">
        <Each of={props.options}>
          {(option) => (
            <Button
              size="xs"
              variant={props.value() === option().id ? "outline" : "ghost"}
              onPress={() => props.onChange(option().id)}
            >
              <Text>{option().label}</Text>
            </Button>
          )}
        </Each>
      </View>
    </View>
  );
}

/** A small muted caption. */
export function Caption(props: { children: string; class?: string }) {
  return <Text class={`text-xs text-muted-foreground ${props.class ?? ""}`}>{props.children}</Text>;
}
