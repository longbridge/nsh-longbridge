// The Kbd page: keys, groups, in a button and in a tooltip.

import { keymap, Text, View } from "nsh";
import { Button, Kbd, KbdGroup, Tooltip } from "nsh-longbridge";
import { Page, Preview, Section } from "../ui";

const keys = ["⌘", "⇧", "⌥", "⌃", "Esc", "Enter"];
const combos = ["mod+k", "mod+shift+p", "alt+enter"];

export function KbdPage() {
  return (
    <Page
      title="Kbd"
      description="Key caps for shortcuts in buttons, menus and tooltips. It does not parse key names: give it keymap.label(..)."
    >
      <Section title="Keys" description="One character is square; a name widens it.">
        <Preview code={keys.map((k) => `<Kbd>${k}</Kbd>`).join("\n")}>
          {keys.map((k) => (
            <Kbd>
              <Text>{k}</Text>
            </Kbd>
          ))}
        </Preview>
      </Section>

      <Section title="Groups" description="Caps in a row; words between them are the app's own text.">
        <Preview
          code={[
            `<KbdGroup>`,
            `  <Kbd>Ctrl</Kbd>`,
            `  <Text class="text-xs text-muted-foreground">+</Text>`,
            `  <Kbd>B</Kbd>`,
            `</KbdGroup>`,
            ...combos.map((c) => `<Kbd>{keymap.label("${c}")}</Kbd>`),
          ].join("\n")}
        >
          <KbdGroup>
            <Kbd>
              <Text>Ctrl</Text>
            </Kbd>
            <Text class="text-xs text-muted-foreground">+</Text>
            <Kbd>
              <Text>B</Text>
            </Kbd>
          </KbdGroup>
          {combos.map((c) => (
            <Kbd>
              <Text>{keymap.label(c)}</Text>
            </Kbd>
          ))}
        </Preview>
      </Section>

      <Section title="In a button and a tooltip" description="In a tooltip a cap takes the tooltip's inverted colors by itself.">
        <Preview
          code={[
            `<Button variant="outline" size="sm">Search <Kbd>{keymap.label("mod+k")}</Kbd></Button>`,
            `<View tooltip={<Tooltip open>Save <Kbd>{keymap.label("mod+s")}</Kbd></Tooltip>}>`,
            `  <Button>Save</Button>`,
            `</View>`,
          ].join("\n")}
          class="pt-16"
        >
          <Button variant="outline" size="sm">
            <Text>Search</Text>
            <Kbd>
              <Text>{keymap.label("mod+k")}</Text>
            </Kbd>
          </Button>
          <View
            tooltip={
              <Tooltip open>
                <Text>Save</Text>
                <Kbd>
                  <Text>{keymap.label("mod+s")}</Text>
                </Kbd>
              </Tooltip>
            }
          >
            <Button>
              <Text>Save</Text>
            </Button>
          </View>
        </Preview>
      </Section>
    </Page>
  );
}
