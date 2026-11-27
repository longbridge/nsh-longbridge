// The site's own controls and type: the solid primary button and the raised
// secondary one, the segmented control (a recessed track, the chosen item a
// lit pill), the kicker (a citrus dot and a short label) and the Figtree
// headings. They are the site's chrome, so they take the site's
// colors (`site()`), never the theme's: a preset recolors the demos only.

import { Text, View, type JSX } from "nsh";
import { Glyph } from "./icons";
import { DISPLAY, site } from "./site";

/** A text class in color `hex` (an arbitrary value: the site's colors are no tokens). */
export const ink = (hex: string) => `text-[${hex}]`;

/** A button of the site: `primary` solid in the page's ink (one per view), `secondary` raised on the page. */
export function SiteButton(props: {
  variant?: "primary" | "secondary";
  size?: "md" | "lg";
  label: string;
  icon?: string;
  iconEnd?: string;
  onPress?: () => void;
  role?: "button" | "link";
}) {
  const primary = () => (props.variant ?? "primary") === "primary";
  const s = () => site();
  const h = (props.size ?? "lg") === "lg" ? 40 : 34;
  return (
    <View
      class="flex-row items-center justify-center gap-2 active:opacity-90"
      style={{
        height: h,
        paddingHorizontal: h === 40 ? 18 : 14,
        borderRadius: 10,
        borderWidth: 1,
        borderColor: primary() ? s().solid : s().border,
        backgroundColor: primary() ? s().solid : s().raised,
        boxShadow: primary() ? "inset 0 1px 0 #ffffff1f, 0 1px 2px #0000001f" : s().raisedShadow,
        cursor: "default",
      }}
      hoverStyle={primary() ? { backgroundColor: s().solidHover, borderColor: s().solidHover } : { backgroundColor: s().card }}
      role={props.role ?? "button"}
      accessibilityLabel={props.label}
      onPress={props.onPress}
    >
      <If when={props.icon}>
        <Glyph icon={props.icon!} size={15} class={ink(primary() ? s().onSolid : s().ink)} />
      </If>
      <Text
        style={{
          color: primary() ? s().onSolid : s().ink,
          fontSize: h === 40 ? 14.5 : 13.5,
          fontWeight: "550",
          letterSpacing: -0.1,
        }}
      >
        {props.label}
      </Text>
      <If when={props.iconEnd}>
        <Glyph icon={props.iconEnd!} size={15} class={ink(primary() ? s().onSolid : s().muted)} />
      </If>
    </View>
  );
}

/** One choice of a segmented control: its value, its icon or label, what screen readers say. */
export interface Segment {
  value: string;
  label: string;
  icon?: string;
}

/** A segmented control: a recessed track, the chosen segment a lit pill. Icons only when the segments have them. */
export function Segmented(props: {
  segments: Segment[];
  value: string;
  onChange: (value: string) => void;
  accessibilityLabel: string;
  /** A segment's side (icon segments) or height (labels). */
  size?: number;
}) {
  const s = () => site();
  const side = props.size ?? 26;
  return (
    <View
      class="flex-row"
      style={{
        padding: 2,
        gap: 2,
        borderRadius: 9,
        backgroundColor: s().inset,
        boxShadow: `${s().insetShadow}, inset 0 0 0 1px ${s().line}`,
      }}
      role="radiogroup"
      accessibilityLabel={props.accessibilityLabel}
    >
      {props.segments.map((seg) => {
        const on = () => props.value === seg.value;
        return (
          <View
            class="items-center justify-center"
            style={{
              height: side,
              minWidth: seg.icon ? side + 2 : undefined,
              paddingHorizontal: seg.icon ? 0 : 10,
              borderRadius: 7,
              backgroundImage: on() ? s().thumb : undefined,
              boxShadow: on() ? s().thumbShadow : undefined,
              cursor: "default",
            }}
            hoverStyle={on() ? undefined : { backgroundColor: s().hover }}
            role="radio"
            accessibilityState={{ checked: on() }}
            accessibilityLabel={seg.label}
            onPress={() => props.onChange(seg.value)}
          >
            {seg.icon ? (
              <Glyph icon={seg.icon} size={14} class={ink(on() ? s().ink : s().muted)} />
            ) : (
              <Text style={{ color: on() ? s().ink : s().muted, fontSize: 12.5, fontWeight: on() ? "550" : "450" }}>
                {seg.label}
              </Text>
            )}
          </View>
        );
      })}
    </View>
  );
}

/** A section's kicker: a citrus dot and a short label in the quiet ink. `quiet`: no dot, the fainter ink. */
export function Kicker(props: { children: string; quiet?: boolean }) {
  return (
    <View class="flex-row items-center gap-2">
      <If when={!props.quiet}>
        <View style={{ width: 6, height: 6, borderRadius: 3, backgroundColor: site().citrusFill }} />
      </If>
      <Text
        style={{
          color: props.quiet ? site().faint : site().muted,
          fontSize: props.quiet ? 12 : 13,
          fontWeight: "550",
          letterSpacing: props.quiet ? 0.2 : -0.05,
        }}
      >
        {props.children}
      </Text>
    </View>
  );
}

/** A section's heading: the kicker, the Figtree title, a lead; `end` beside them on wide screens. */
export function SectionHeading(props: {
  kicker: string;
  title: string;
  lead?: string;
  narrow?: boolean;
  end?: JSX.Element;
}) {
  const size = () => (props.narrow ? 30 : 42);
  return (
    <View class={props.narrow ? "gap-6" : "flex-row items-end justify-between gap-10"}>
      <View class="gap-4" style={{ maxWidth: 680, flexShrink: 1 }}>
        <Kicker>{props.kicker}</Kicker>
        <Text
          style={{
            fontFamily: DISPLAY,
            color: site().ink,
            fontSize: size(),
            lineHeight: Math.round(size() * 1.1),
            fontWeight: "600",
            letterSpacing: -size() * 0.03,
          }}
        >
          {props.title}
        </Text>
        <If when={props.lead}>
          <Text style={{ color: site().muted, fontSize: 16, lineHeight: 26 }}>{props.lead!}</Text>
        </If>
      </View>
      <If when={props.end}>
        <View class="items-start">{props.end}</View>
      </If>
    </View>
  );
}

/** The site's card surface: its fill, a hairline, the top light and a soft shadow (radius 16). */
export const cardStyle = () => ({
  backgroundColor: site().card,
  borderWidth: 1,
  borderColor: site().border,
  borderRadius: 16,
  boxShadow: site().cardShadow,
});
