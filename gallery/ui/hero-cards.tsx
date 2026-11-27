// The cards around the hero's sheet (`hero-demo.tsx`), each the library's
// components: who has the invoice open (an avatar group), where its amount
// falls among the quarter's (a slider over a histogram, its value in a
// bubble) and the month's collections (a progress bar). They float over the
// sheet's edges, lean away from the pointer a little (`pointer`, from the
// page), and come and go with the loop. The poster's HTML of them is in
// `website/src/components/Posters.astro`, from the same data (`hero-data.ts`).

import { type JSX, Text, View } from "nsh";
import { Avatar, AvatarGroup, Card, CardDescription, CardTitle, Progress, Slider } from "nsh-longbridge";
import { AMOUNT_MAX, AMOUNT_STEP, amounts, DUE, money, percent, presence, viewers } from "./hero-data";
import { floatShadow } from "./stage";

export type Pointer = { x: number; y: number } | null;

/**
 * A card floating at `place` (beside the sheet); hidden, it is `from` away
 * and clear. `depth`: how far it leans from the pointer, in points.
 */
export function Floating(props: {
  place: Record<string, number>;
  shown: boolean;
  from: [number, number];
  depth: number;
  pointer: Pointer;
  children?: JSX.Element;
}) {
  const lean = () => (props.pointer ? [-props.pointer.x * props.depth, -props.pointer.y * props.depth] : [0, 0]);
  return (
    <View
      class="absolute"
      accessibilityElementsHidden={!props.shown}
      style={{
        ...props.place,
        opacity: props.shown ? 1 : 0,
        pointerEvents: props.shown ? "auto" : "none",
        translate: [lean()[0] + (props.shown ? 0 : props.from[0]), lean()[1] + (props.shown ? 0 : props.from[1])],
        transitionProperty: "opacity, translate",
        transitionDuration: 480,
        transitionTimingFunction: "ease-out",
      }}
    >
      {props.children}
    </View>
  );
}

/** Who has the invoice open: the first `count` viewers. */
export function PresenceCard(props: { count: number }) {
  const text = () => presence(props.count);
  return (
    <Card class="flex-row items-center gap-3 py-2.5 pl-2.5 pr-4" style={{ boxShadow: floatShadow }}>
      <AvatarGroup size="sm" accessibilityLabel="Viewing this invoice">
        {viewers.slice(0, props.count).map((v) => (
          <Avatar name={v.name} src={v.photo} />
        ))}
      </AvatarGroup>
      <View class="gap-0.5">
        <CardTitle class="text-sm">
          <Text>{text().title}</Text>
        </CardTitle>
        <CardDescription class="text-xs">
          <Text>{text().line}</Text>
        </CardDescription>
      </View>
    </Card>
  );
}

/** The amount among the quarter's invoices: a slider over their histogram, kept with the sheet's. */
export function AmountsCard(props: { width: number; amount: number; onAmount: (v: number) => void }) {
  return (
    <Card class="gap-4 p-4" style={{ width: props.width, boxShadow: floatShadow }}>
      <View class="gap-0.5">
        <CardTitle class="text-sm">
          <Text>Against this quarter</Text>
        </CardTitle>
        <CardDescription class="text-xs">
          <Text>186 invoices, most near $2,000</Text>
        </CardDescription>
      </View>
      <View>
        <Slider
          min={0}
          max={AMOUNT_MAX}
          step={AMOUNT_STEP}
          histogram={amounts}
          value={props.amount}
          bubble="always"
          formatValue={(v) => money(v)}
          onChange={(e) => props.onAmount(e.value as number)}
          accessibilityLabel="Amount against this quarter"
        />
      </View>
    </Card>
  );
}

/** The month's collections: `collected` of what is due. */
export function CollectedCard(props: { width: number; collected: number }) {
  return (
    <Card class="gap-3 p-4" style={{ width: props.width, boxShadow: floatShadow }}>
      <View class="flex-row items-center justify-between">
        <CardTitle class="text-sm">
          <Text>Collected in December</Text>
        </CardTitle>
        <CardTitle class="text-sm tabular-nums">
          <Text>{percent(props.collected)}</Text>
        </CardTitle>
      </View>
      <Progress value={(props.collected / DUE) * 100} accessibilityLabel="Collected in December" />
      <CardDescription class="text-xs tabular-nums">
        <Text>{`${money(props.collected)} of ${money(DUE)}`}</Text>
      </CardDescription>
    </Card>
  );
}
