// The Pagination page: search results paged, a compact pager, sizes, a
// disabled pager.

import { signal, Text, View } from "nsh";
import { Pagination, type Size } from "nsh-longbridge";
import { Demo, Page, Row, Section } from "../ui/layout";
import { TokensSection } from "../ui/tokens";

const sizes: Size[] = ["xs", "sm", "md", "lg"];

const results = Array.from({ length: 96 }, (_, i) => ({
  title: `Quarterly report ${String(i + 1).padStart(2, "0")}`,
  detail: `Finance · updated ${(i % 28) + 1} days ago`,
}));
const perPage = 4;
const pages = Math.ceil(results.length / perPage);

export function PaginationPage() {
  return (
    <Page
      eyebrow="Components"
      title="Pagination"
      description="Previous, the pages around the current one with ellipses for the runs left out, and Next. The current page is outlined; the others are ghost buttons."
    >
      <HeroDemo />

      <Section
        title="Compact"
        description="Previous, the app's words and Next only: a pager for a narrow place. In a narrow window the words of Previous and Next hide anyway."
      >
        <CompactDemo />
      </Section>

      <Section title="Sizes" description="Button's sizes: 24, 28, 32 and 36 points tall with a pointer, taller on touch screens.">
        <Demo class="items-stretch">
          {sizes.map((size) => (
            <Row label={size}>
              <SizedPager size={size} />
            </Row>
          ))}
        </Demo>
      </Section>

      <Section title="Disabled" description="No item takes a press or the focus while the results load.">
        <Demo class="items-stretch">
          <Pagination page={3} totalPages={12} disabled accessibilityLabel="Results pages, loading" />
        </Demo>
      </Section>

      <TokensSection prefix="pagination" component="Pagination" />
    </Page>
  );
}

function HeroDemo() {
  const page = signal(5);
  const shown = () => results.slice((page() - 1) * perPage, page() * perPage);
  return (
    <Demo hero class="items-stretch">
      <View class="w-full max-w-[560px] self-center gap-6">
        <Text class="-mb-3 text-sm text-muted-foreground">{`${results.length} results · page ${page()} of ${pages}`}</Text>
        <View class="rounded-lg border border-border">
          {shown().map((r) => (
            <View class="gap-0.5 border-b border-border px-4 py-3 last:border-b-0">
              <Text class="text-sm font-medium text-foreground">{r.title}</Text>
              <Text class="text-xs text-muted-foreground">{r.detail}</Text>
            </View>
          ))}
        </View>
        <Pagination page={page()} totalPages={pages} visiblePages={7} onChange={({ page: next }) => page.set(next)} accessibilityLabel="Results pages" />
      </View>
    </Demo>
  );
}

function CompactDemo() {
  const page = signal(3);
  return (
    <Demo class="items-stretch">
      <View class="w-full max-w-[320px] self-center">
        <Pagination page={page()} totalPages={12} compact onChange={({ page: next }) => page.set(next)} accessibilityLabel="Photos">
          <Text class="px-2 text-sm tabular-nums text-muted-foreground">{`${page()} / 12`}</Text>
        </Pagination>
      </View>
    </Demo>
  );
}

function SizedPager(props: { size: Size }) {
  const page = signal(2);
  return (
    <Pagination
      size={props.size}
      page={page()}
      totalPages={10}
      class="justify-start"
      onChange={({ page: next }) => page.set(next)}
      accessibilityLabel={`Pages, ${props.size}`}
    />
  );
}
