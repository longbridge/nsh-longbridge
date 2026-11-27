// The gallery's page layout: one type scale (title 30, section 20, body 14,
// caption 12), one spacing scale, hairline borders. Only theme tokens in
// classes: the gallery looks like the library it shows.

import { effect, Text, View, type JSX, type ViewRef, windowSize } from "nsh";
import { narrow, pageScroll } from "./screen";

/** A page: its title and one line under it, then its sections. */
export function Page(props: { title: string; description: string; eyebrow?: string; children?: JSX.Element }) {
  return (
    <View class="gap-14">
      <View class="gap-3">
        <If when={props.eyebrow}>
          <Text class="text-sm font-medium text-muted-foreground">{props.eyebrow!}</Text>
        </If>
        <Text class="text-3xl font-semibold tracking-tight text-foreground">{props.title}</Text>
        <Text class="max-w-[640px] text-base leading-7 text-muted-foreground">{props.description}</Text>
      </View>
      {props.children}
    </View>
  );
}

/** A section: a heading, a sentence, and what it shows. */
export function Section(props: { title: string; description?: string; children?: JSX.Element; class?: string }) {
  return (
    <View class={`gap-5 ${props.class ?? ""}`}>
      <View class="gap-1.5">
        <Text class="text-xl font-semibold tracking-tight text-foreground">{props.title}</Text>
        <If when={props.description}>
          <Text class="max-w-[640px] text-sm leading-6 text-muted-foreground">{props.description!}</Text>
        </If>
      </View>
      {props.children}
    </View>
  );
}

/** The header's height (`h-14`). */
const HEADER = 56;

/**
 * A scene: the components on the page's surface inside a hairline frame.
 * `hero` is the large one at the top of a component page.
 */
export function Demo(props: {
  children?: JSX.Element;
  class?: string;
  hero?: boolean;
  label?: string;
  row?: boolean;
  /**
   * Told whether the scene's top is in the window (below the header), as the page scrolls: a scene holding a layer
   * open shows it only then, so the layer never stays at the window's edge or over the header.
   */
  inView?: (shown: boolean) => void;
}) {
  let scene: ViewRef | undefined;
  if (props.inView) {
    const report = props.inView;
    effect(() => {
      pageScroll();
      const height = windowSize().height;
      scene?.measure().then((m) => {
        if (m) report(m.pagePosition.y > HEADER - 16 && m.pagePosition.y < height - 64);
      });
    });
  }
  return (
    <View class="flex-1 min-w-[240px] gap-2.5" ref={(r: ViewRef) => (scene = r)}>
      <View
        class={`flex-1 rounded-xl border border-border bg-background items-center justify-center ${
          props.hero ? "min-h-[320px] p-12" : "min-h-[176px] p-8"
        } ${props.row ? "flex-row flex-wrap content-center gap-3" : "gap-4"} ${props.class ?? ""}`}
      >
        {props.children}
      </View>
      <If when={props.label}>
        <Text class="px-1 text-xs text-muted-foreground">{props.label!}</Text>
      </If>
    </View>
  );
}

/** Scenes side by side, wrapping to a column when narrow. */
export function Demos(props: { children?: JSX.Element }) {
  return <View class="flex-row flex-wrap gap-6">{props.children}</View>;
}

/** A small muted caption. */
export function Caption(props: { children: string; class?: string }) {
  return <Text class={`text-xs text-muted-foreground ${props.class ?? ""}`}>{props.children}</Text>;
}

/** A row of a scene: a muted name at the start (above, at phone widths), the components after it. */
export function Row(props: { label: string; children?: JSX.Element; class?: string }) {
  return (
    <View class={`${narrow() ? "gap-2" : "flex-row items-center gap-4"} ${props.class ?? ""}`}>
      <Text class={`${narrow() ? "" : "w-20"} text-xs text-muted-foreground`}>{props.label}</Text>
      <View class="flex-1 flex-row flex-wrap items-center gap-3">{props.children}</View>
    </View>
  );
}
