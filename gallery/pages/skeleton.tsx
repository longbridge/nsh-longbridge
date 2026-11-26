// The Skeleton page: a profile, a card, round and square blocks.

import { View } from "nsh";
import { Skeleton } from "nsh-longbridge";
import { Page, Preview, Section } from "../ui";

const lines = ["w-[250px]", "w-[200px]"];

export function SkeletonPage() {
  return (
    <Page
      title="Skeleton"
      description="A pulsing block holding the place of content still loading. No size of its own; no pulse with reduced motion."
    >
      <Section title="A profile" description="Say “loading” on the container; screen readers skip each block.">
        <Preview
          code={[
            `<View class="flex-row items-center gap-4" accessibilityLabel="Loading profile" accessibilityState={{ busy: true }}>`,
            `  <Skeleton class="size-12 rounded-full" />`,
            `  <View class="gap-2">`,
            ...lines.map((w) => `    <Skeleton class="h-4 ${w}" />`),
            `  </View>`,
            `</View>`,
          ].join("\n")}
        >
          <View class="flex-row items-center gap-4" accessibilityLabel="Loading profile" accessibilityState={{ busy: true }}>
            <Skeleton class="size-12 rounded-full" />
            <View class="gap-2">
              {lines.map((w) => (
                <Skeleton class={`h-4 ${w}`} />
              ))}
            </View>
          </View>
        </Preview>
      </Section>

      <Section title="A card" description="The shape of what comes, so nothing jumps when it arrives.">
        <Preview
          code={[
            `<View class="gap-3">`,
            `  <Skeleton class="h-[125px] w-[250px] rounded-xl" />`,
            `  <View class="gap-2">`,
            ...lines.map((w) => `    <Skeleton class="h-4 ${w}" />`),
            `  </View>`,
            `</View>`,
          ].join("\n")}
        >
          <View class="gap-3">
            <Skeleton class="h-[125px] w-[250px] rounded-xl" />
            <View class="gap-2">
              {lines.map((w) => (
                <Skeleton class={`h-4 ${w}`} />
              ))}
            </View>
          </View>
        </Preview>
      </Section>
    </Page>
  );
}
