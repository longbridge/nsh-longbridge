// The Toggle page: an editor's formatting row, variants, sizes, states,
// toggles with words.

import { signal, Text, View, type JSX } from "nsh";
import { type Size, Toggle, type ToggleVariant } from "nsh-longbridge";
import { Bold, Bookmark, Glyph, Heart, Italic, Star, Strikethrough, Underline } from "../ui/icons";
import { Demo, Demos, Page, Row, Section } from "../ui/layout";
import { TokensSection } from "../ui/tokens";

const sizes: Size[] = ["xs", "sm", "md", "lg"];
const iconSizes: Record<Size, number> = { xs: 12, sm: 14, md: 16, lg: 18 };

const states: { label: string; pressed?: boolean; disabled?: boolean }[] = [
  { label: "Off" },
  { label: "On", pressed: true },
  { label: "Disabled", disabled: true },
  { label: "Disabled, on", disabled: true, pressed: true },
];

export function TogglePage() {
  return (
    <Page
      eyebrow="Components"
      title="Toggle"
      description="A button that stays on or off: a format, a filter, a view option. Two variants, four sizes; on, it takes the accent fill."
    >
      <Demo hero>
        <EditorDemo />
      </Demo>

      <Section title="Variants" description="Default is transparent until hovered or on; outline adds the field's edge and shadow, for a toggle standing alone.">
        <Demos>
          <Demo row label="default">
            <Live on accessibilityLabel="Bold">
              <Glyph icon={Bold} />
            </Live>
            <Live accessibilityLabel="Italic">
              <Glyph icon={Italic} />
            </Live>
          </Demo>
          <Demo row label="outline">
            <Live variant="outline" on accessibilityLabel="Bold">
              <Glyph icon={Bold} />
            </Live>
            <Live variant="outline" accessibilityLabel="Italic">
              <Glyph icon={Italic} />
            </Live>
          </Demo>
        </Demos>
      </Section>

      <Section title="Sizes" description="A toggle is a square of the control height of its size with an icon alone, and as wide as its words with them: a button of the same size beside it lines up.">
        <Demo class="items-stretch">
          {sizes.map((size) => (
            <Row label={size}>
              <Live size={size} on accessibilityLabel={`Bold, ${size}`}>
                <Glyph icon={Bold} size={iconSizes[size]} />
              </Live>
              <Live size={size} variant="outline" accessibilityLabel={`Italic, ${size}`}>
                <Glyph icon={Italic} size={iconSizes[size]} />
              </Live>
              <Live size={size} variant="outline">
                <Glyph icon={Star} size={iconSizes[size]} />
                <Text>Starred</Text>
              </Live>
            </Row>
          ))}
        </Demo>
      </Section>

      <Section title="States" description="Disabled dims it and takes no presses, on or off. Tab shows the ring.">
        <Demo row class="gap-x-6 gap-y-4">
          {states.map((s) => (
            <Live variant="outline" on={s.pressed} disabled={s.disabled}>
              <Glyph icon={Bookmark} size={16} />
              <Text>{s.label}</Text>
            </Live>
          ))}
        </Demo>
      </Section>

      <Section title="With words" description="An icon and a word, or a word alone: the toggle names itself, so no accessibilityLabel is needed.">
        <FiltersDemo />
      </Section>

      <TokensSection prefix="toggle" component="Toggle" exclude={["toggle-group"]} />
    </Page>
  );
}

function EditorDemo() {
  const bold = signal(true);
  const italic = signal(false);
  const underline = signal(false);
  const strike = signal(false);
  const style = () =>
    [
      bold() ? "font-semibold" : "",
      italic() ? "italic" : "",
      underline() ? "underline" : strike() ? "line-through" : "",
    ].join(" ");
  return (
    <View class="w-full max-w-[460px] rounded-xl border border-border bg-card">
      <View class="flex-row items-center gap-1 border-b border-border p-2">
        <Toggle size="sm" pressed={bold()} onChange={(e) => bold.set(e.pressed)} accessibilityLabel="Bold">
          <Glyph icon={Bold} size={14} />
        </Toggle>
        <Toggle size="sm" pressed={italic()} onChange={(e) => italic.set(e.pressed)} accessibilityLabel="Italic">
          <Glyph icon={Italic} size={14} />
        </Toggle>
        <Toggle size="sm" pressed={underline()} onChange={(e) => underline.set(e.pressed)} accessibilityLabel="Underline">
          <Glyph icon={Underline} size={14} />
        </Toggle>
        <Toggle size="sm" pressed={strike()} onChange={(e) => strike.set(e.pressed)} accessibilityLabel="Strikethrough">
          <Glyph icon={Strikethrough} size={14} />
        </Toggle>
      </View>
      <View class="gap-2 p-5">
        <Text class="text-xs font-medium text-muted-foreground">Release notes</Text>
        <Text class={`text-base leading-7 text-card-foreground ${style()}`}>
          Charts now load twice as fast, and every table remembers its sort order between visits.
        </Text>
      </View>
    </View>
  );
}

function FiltersDemo() {
  const starred = signal(true);
  const liked = signal(false);
  const unread = signal(false);
  return (
    <Demo row>
      <Toggle variant="outline" size="sm" pressed={starred()} onChange={(e) => starred.set(e.pressed)}>
        <Glyph icon={Star} size={14} />
        <Text>Starred</Text>
      </Toggle>
      <Toggle variant="outline" size="sm" pressed={liked()} onChange={(e) => liked.set(e.pressed)}>
        <Glyph icon={Heart} size={14} />
        <Text>Liked</Text>
      </Toggle>
      <Toggle variant="outline" size="sm" pressed={unread()} onChange={(e) => unread.set(e.pressed)}>
        <Text>Unread only</Text>
      </Toggle>
    </Demo>
  );
}

/** A toggle that keeps its own state (Toggle is controlled only). */
function Live(props: {
  on?: boolean;
  variant?: ToggleVariant;
  size?: Size;
  disabled?: boolean;
  accessibilityLabel?: string;
  children?: JSX.Element;
}) {
  const on = signal(props.on ?? false);
  return (
    <Toggle
      variant={props.variant}
      size={props.size}
      disabled={props.disabled}
      accessibilityLabel={props.accessibilityLabel}
      pressed={on()}
      onChange={(e) => on.set(e.pressed)}
    >
      {props.children}
    </Toggle>
  );
}
