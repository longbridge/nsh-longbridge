// Code as the site shows it: TSX colored with nsh.dev's code palette
// (keywords, strings, tags, attributes, calls, numbers, comments), line
// numbers in the gutter. A small tokenizer of the gallery's own, enough for
// the snippets the site shows, so their colors are the site's tokens.

import { Text, View } from "nsh";
import { type Kind, type State, tokenize } from "./code-tokens";
import { site } from "./site";

/** `text` as colored lines with their numbers. `size`: the type's size (line height 1.6 times it). */
export function Code(props: { text: string; size?: number; numbers?: boolean }) {
  const c = () => site().code;
  const size = props.size ?? 12.5;
  const lh = Math.round(size * 1.65);
  const color = (k: Kind) => (k === "plain" ? c().fg : c()[k]);
  const lines = props.text.replace(/\n$/, "").split("\n");
  const state: State = { inTag: false };
  const tokens = lines.map((line) => tokenize(line, state));
  const width = String(lines.length).length * size * 0.62 + 4;
  return (
    <View>
      {lines.map((line, n) => (
        <View class="flex-row">
          <If when={props.numbers !== false}>
            <Text
              class="font-mono"
              style={{ width, marginRight: 18, textAlign: "right", color: site().faint, fontSize: size, lineHeight: lh, opacity: 0.7 }}
            >
              {String(n + 1)}
            </Text>
          </If>
          <Text class="font-mono" style={{ fontSize: size, lineHeight: lh, color: c().fg }}>
            {line === "" ? " " : tokens[n].map(([k, t]) => (
              <Text style={{ color: color(k), fontStyle: k === "comment" ? "italic" : undefined }}>{t}</Text>
            ))}
          </Text>
        </View>
      ))}
    </View>
  );
}
