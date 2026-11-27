// The site's small TSX tokenizer (no NSH), so the gallery's `Code` and the
// website color snippets alike: keywords, strings, tags, attributes, calls,
// numbers, comments.

export type Kind = "keyword" | "string" | "comment" | "fn" | "type" | "number" | "property" | "punct" | "plain";

const KEYWORDS = new Set([
  "import", "from", "export", "function", "const", "let", "return", "if", "else", "true", "false", "null",
  "undefined", "new", "type", "interface", "as", "async", "await",
]);

/** Whether a line starts inside a tag (its attributes run over several lines). */
export interface State {
  inTag: boolean;
}

/** The tokens of one line (a snippet has no comment or string over several lines). */
export function tokenize(line: string, state: State): [Kind, string][] {
  const out: [Kind, string][] = [];
  let i = 0;
  let inTag = state.inTag;
  // Between a tag's ">" and the next "<" or "{": the element's text.
  let inText = false;
  // Spaces go at the start of the token after them: a text's trailing
  // spaces are dropped, so a span of spaces alone would vanish.
  let space = "";
  const push = (k: Kind, t: string) => {
    const last = out[out.length - 1];
    if (last && last[0] === k && space === "") last[1] += t;
    else if (last && last[0] === k) last[1] += space + t;
    else out.push([k, space + t]);
    space = "";
  };
  while (i < line.length) {
    const rest = line.slice(i);
    let m: RegExpExecArray | null;
    if (inText && (m = /^[^<{]+/.exec(rest))) {
      push("plain", m[0]);
    } else if ((m = /^\s+/.exec(rest))) {
      space += m[0];
    } else if ((m = /^\/\/.*/.exec(rest))) {
      push("comment", m[0]);
    } else if ((m = /^("[^"]*"|'[^']*'|`[^`]*`)/.exec(rest))) {
      push("string", m[0]);
    } else if ((m = /^<\/?([A-Za-z][\w.]*)/.exec(rest))) {
      // A tag's opening: the bracket as punctuation, the name as a type.
      const bracket = m[0].slice(0, m[0].length - m[1].length);
      inText = false;
      push("punct", bracket);
      push("type", m[1]);
      inTag = true;
    } else if ((m = /^\/?>/.exec(rest)) && inTag) {
      push("punct", m[0]);
      inTag = false;
      inText = m[0] === ">";
    } else if ((m = /^[A-Za-z_$][\w$]*/.exec(rest))) {
      const word = m[0];
      const after = line.slice(i + word.length);
      if (KEYWORDS.has(word)) push("keyword", word);
      else if (inTag && /^=/.test(after)) push("property", word);
      else if (/^\s*\(/.test(after)) push("fn", word);
      else if (/^[A-Z]/.test(word)) push("type", word);
      else push("plain", word);
    } else if ((m = /^\d+(\.\d+)?/.exec(rest))) {
      push("number", m[0]);
    } else if ((m = /^[{}()[\];,.=<>!&|?:+\-*/]+/.exec(rest))) {
      push("punct", m[0]);
      inText = false;
    } else {
      push("plain", rest[0]);
      m = { 0: rest[0] } as unknown as RegExpExecArray;
    }
    i += m[0].length;
  }
  state.inTag = inTag;
  return out;
}

