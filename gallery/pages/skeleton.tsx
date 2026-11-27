// The Skeleton page: a feed loading, a profile, a card, a table.

import { signal, Text, View } from "nsh";
import { Button, Card, CardContent, CardHeader, Skeleton, Table, TableBody, TableCell, TableHead, TableHeader, TableRow } from "nsh-longbridge";
import { Demo, Demos, Page, Section } from "../ui/layout";
import { TokensSection } from "../ui/tokens";

const lines = ["w-[250px]", "w-[200px]"];
const posts = [
  { initials: "OM", name: "Olivia Martin", text: "Shipped the new billing page. Thanks for the reviews, everyone!" },
  { initials: "JL", name: "Jackson Lee", text: "The staging database moves to the new cluster on Friday." },
];
const columns = ["w-24", "flex-1", "w-28"];

export function SkeletonPage() {
  return (
    <Page
      eyebrow="Components"
      title="Skeleton"
      description="A pulsing block in the shape of content still loading. It has no size of its own and does not pulse with reduced motion."
    >
      <Demo hero>
        <FeedDemo />
      </Demo>

      <Section title="Shapes" description="Round, a line, a card: the shape of what comes, so nothing jumps when it arrives. Say “loading” on the container; screen readers skip each block.">
        <Demos>
          <Demo label="A profile">
            <View class="flex-row items-center gap-4" accessibilityLabel="Loading profile" accessibilityState={{ busy: true }}>
              <Skeleton class="size-12 rounded-full" />
              <View class="gap-2">
                {lines.map((w) => (
                  <Skeleton class={`h-4 ${w}`} />
                ))}
              </View>
            </View>
          </Demo>
          <Demo label="A card">
            <View class="gap-3" accessibilityLabel="Loading card" accessibilityState={{ busy: true }}>
              <Skeleton class="h-[125px] w-[250px] rounded-xl" />
              <View class="gap-2">
                {lines.map((w) => (
                  <Skeleton class={`h-4 ${w}`} />
                ))}
              </View>
            </View>
          </Demo>
        </Demos>
      </Section>

      <Section title="A table" description="Rows of the table's own height, columns of its own widths.">
        <Demo class="items-stretch">
          <View class="w-full max-w-[560px] self-center overflow-hidden rounded-lg border border-border" accessibilityState={{ busy: true }}>
            <Table size="lg" accessibilityLabel="Loading invoices">
              <TableHeader>
                <TableRow>
                  <TableHead class={`${columns[0]} flex-none pl-4`}>
                    <Text>Invoice</Text>
                  </TableHead>
                  <TableHead class={columns[1]}>
                    <Text>Customer</Text>
                  </TableHead>
                  <TableHead class={`${columns[2]} flex-none items-end pr-4`}>
                    <Text>Amount</Text>
                  </TableHead>
                </TableRow>
              </TableHeader>
              <TableBody>
                {[0, 1, 2, 3].map((i) => (
                  <TableRow>
                    <TableCell class={`${columns[0]} flex-none pl-4`}>
                      <Skeleton class="h-3.5 w-full" />
                    </TableCell>
                    <TableCell class={columns[1]}>
                      <Skeleton class={`h-3.5 ${i % 2 ? "w-1/2" : "w-2/3"}`} />
                    </TableCell>
                    <TableCell class={`${columns[2]} flex-none items-end pr-4`}>
                      <Skeleton class="h-3.5 w-full" />
                    </TableCell>
                  </TableRow>
                ))}
              </TableBody>
            </Table>
          </View>
        </Demo>
      </Section>

      <TokensSection prefix="skeleton" component="Skeleton" />
    </Page>
  );
}

/** A feed that loads: skeletons until the posts arrive, again on Reload. */
function FeedDemo() {
  const loading = signal(true);
  const reload = () => {
    loading.set(true);
    setTimeout(() => loading.set(false), 1800);
  };
  return (
    <View class="w-full max-w-[400px] gap-4">
      <Card class="w-full">
        <CardHeader>
          <View class="flex-row items-center justify-between">
            <Text class="text-sm font-semibold text-card-foreground">Team updates</Text>
            <Button size="xs" variant="outline" onPress={() => (loading() ? loading.set(false) : reload())}>
              <Text>{loading() ? "Show posts" : "Reload"}</Text>
            </Button>
          </View>
        </CardHeader>
        <CardContent class="gap-5">
          {posts.map((p) => (
            <View class="flex-row items-start gap-3">
              {loading() ? (
                <>
                  <Skeleton class="size-9 rounded-full" />
                  <View class="flex-1 gap-2 pt-1" accessibilityLabel="Loading post" accessibilityState={{ busy: true }}>
                    <Skeleton class="h-3.5 w-28" />
                    <Skeleton class="h-3 w-full" />
                    <Skeleton class="h-3 w-3/5" />
                  </View>
                </>
              ) : (
                <>
                  <View class="size-9 items-center justify-center rounded-full bg-muted">
                    <Text class="text-xs font-medium text-muted-foreground">{p.initials}</Text>
                  </View>
                  <View class="flex-1 gap-1">
                    <Text class="text-sm font-medium text-card-foreground">{p.name}</Text>
                    <Text class="text-sm text-muted-foreground">{p.text}</Text>
                  </View>
                </>
              )}
            </View>
          ))}
        </CardContent>
      </Card>
    </View>
  );
}
