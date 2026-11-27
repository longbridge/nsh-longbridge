//! Where a query matches a text, as the lists' filter matches it
//! (`nsh/base`'s `Select` and `Combobox`: the text holds the query,
//! ignoring case): the matched parts of a title, to draw them apart from
//! the rest (`Combobox`'s rows).
//!
//! The filter folds both sides with Rust's lower-casing, which can change a
//! text's length ("İ" is two characters lowered, "ẞ" a shorter one), so a
//! match is found in the folded text and taken back to the original's
//! characters: a match that starts or ends inside a character's fold takes
//! the whole character. The ranges are byte ranges of the original, on its
//! character boundaries, so slicing it with them never panics.

use std::ops::Range;

/// Which matches [`split`] marks.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum Occurrences {
    /// The first match only: the reason a row is in a filtered list.
    #[default]
    First,
    /// Every match, left to right, none overlapping.
    All,
}

/// A run of a text: matched by the query or not.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Part<'a> {
    /// The run's text, a slice of the original.
    pub text: &'a str,
    /// The query matched it.
    pub matched: bool,
}

/// Whether `text` holds `query`, ignoring case, as the lists' filter
/// decides it (every text holds an empty query).
pub fn matches(text: &str, query: &str) -> bool {
    query.is_empty() || text.to_lowercase().contains(&query.to_lowercase())
}

/// The byte ranges of `text` the query matches, ignoring case, left to
/// right and none overlapping (two that overlap once widened to whole
/// characters are joined); none for an empty query.
pub fn match_ranges(text: &str, query: &str) -> Vec<Range<usize>> {
    ranges(text, query, Occurrences::All)
}

/// The byte range of `text` the query first matches, ignoring case.
pub fn first_match(text: &str, query: &str) -> Option<Range<usize>> {
    ranges(text, query, Occurrences::First).into_iter().next()
}

/// `text` as runs, matched and not, in order: the first match or all of
/// them (`which`). The runs join back to `text`; an empty query, or one
/// that matches nothing, gives one unmatched run (none for an empty text).
pub fn split<'a>(text: &'a str, query: &str, which: Occurrences) -> Vec<Part<'a>> {
    runs(text, query, which)
        .into_iter()
        .map(|(range, matched)| Part {
            text: &text[range],
            matched,
        })
        .collect()
}

/// [`split`]'s runs as byte ranges of `text`, each with whether the query
/// matched it: for a caller that keeps them apart from the text.
pub fn runs(text: &str, query: &str, which: Occurrences) -> Vec<(Range<usize>, bool)> {
    let mut runs = Vec::new();
    let mut at = 0;
    for range in ranges(text, query, which) {
        if range.start > at {
            runs.push((at..range.start, false));
        }
        at = range.end;
        runs.push((range, true));
    }
    if at < text.len() {
        runs.push((at..text.len(), false));
    }
    runs
}

fn ranges(text: &str, query: &str, which: Occurrences) -> Vec<Range<usize>> {
    let query = query.to_lowercase();
    if query.is_empty() || text.is_empty() {
        return Vec::new();
    }
    let (folded, starts) = fold(text);
    let mut out: Vec<Range<usize>> = Vec::new();
    for (start, found) in folded.match_indices(query.as_str()) {
        let range = original(&starts, text.len(), start..start + found.len());
        match out.last_mut() {
            // Two matches inside one character's fold: one run.
            Some(last) if last.end > range.start => last.end = last.end.max(range.end),
            _ => out.push(range),
        }
        if which == Occurrences::First {
            break;
        }
    }
    out
}

/// `text` folded as the filter folds it, and where each of its characters
/// starts in the fold and in `text`: `(folded, original)` byte offsets,
/// increasing.
fn fold(text: &str) -> (String, Vec<(usize, usize)>) {
    let folded = text.to_lowercase();
    let mut starts = Vec::with_capacity(text.len());
    let mut at = 0;
    for (i, c) in text.char_indices() {
        starts.push((at, i));
        at += folded_len(c);
    }
    if at == folded.len() {
        return (folded, starts);
    }
    // The fold's lengths disagree with `str::to_lowercase` (a rule this
    // does not know): fold each character alone, which is consistent
    // with the offsets, if not with the filter in that rare case.
    let mut own = String::with_capacity(folded.len());
    starts.clear();
    for (i, c) in text.char_indices() {
        starts.push((own.len(), i));
        own.extend(c.to_lowercase());
    }
    (own, starts)
}

/// The length of `c` lowered within a text. A capital sigma lowers to
/// `σ` or, ending a word, `ς` (`str::to_lowercase`'s one rule with
/// context): both two bytes.
fn folded_len(c: char) -> usize {
    if c == 'Σ' {
        return 'σ'.len_utf8();
    }
    c.to_lowercase().map(char::len_utf8).sum()
}

/// The range of `text` (`len` bytes) whose characters' folds hold the
/// folded range `range`.
fn original(starts: &[(usize, usize)], len: usize, range: Range<usize>) -> Range<usize> {
    // The character whose fold holds the range's first byte…
    let first = starts.partition_point(|&(f, _)| f <= range.start) - 1;
    // …and the one after the character whose fold holds its last byte.
    let after = starts.partition_point(|&(f, _)| f < range.end);
    let start = starts[first].1;
    let end = starts.get(after).map_or(len, |&(_, o)| o);
    start..end
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn folds_agree_with_str_to_lowercase() {
        for text in [
            "İstanbul",
            "ΟΔΟΣ ΣΟΦΟΣ",
            "GROẞ",
            "5 \u{212A}",
            "Straße",
            "東京",
            "ǅemal",
        ] {
            let (folded, _) = fold(text);
            assert_eq!(folded, text.to_lowercase(), "{text}");
        }
    }
}
