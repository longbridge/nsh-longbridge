// Code as the page shows it: the gallery's tokenizer (`gallery/ui/code-tokens.ts`),
// written as HTML spans colored by the site's code tokens (`--s-code-*`).

import { type State, tokenize } from '../../../gallery/ui/code-tokens';

const escape = (s: string) => s.replace(/&/g, '&amp;').replace(/</g, '&lt;').replace(/>/g, '&gt;');

/** `text` as numbered lines of colored spans. */
export function codeHtml(text: string): string {
  const lines = text.replace(/^\n+/, '').replace(/\n$/, '').split('\n');
  const state: State = { inTag: false };
  return lines
    .map((line) => {
      const body = line === '' ? ' ' : tokenize(line, state).map(([k, t]) => `<span class="c-${k}">${escape(t)}</span>`).join('');
      return `<span class="code__line">${body}</span>`;
    })
    .join('');
}
