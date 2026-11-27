//! `foundation::text_match`: where a query matches a title, as the lists'
//! filter matches it (ignoring case), as byte ranges of the title on its
//! character boundaries, across case folds that change a text's length.

use nsh_longbridge::foundation::text_match::{
    Occurrences, Part, first_match, match_ranges, matches, runs, split,
};

fn parts<'a>(text: &'a str, query: &str, which: Occurrences) -> Vec<(&'a str, bool)> {
    split(text, query, which)
        .into_iter()
        .map(|Part { text, matched }| (text, matched))
        .collect()
}

#[test]
fn the_first_match_ignoring_case() {
    assert_eq!(
        parts("Singapore", "sing", Occurrences::First),
        [("Sing", true), ("apore", false)]
    );
    assert_eq!(
        parts("Banana", "AN", Occurrences::First),
        [("B", false), ("an", true), ("ana", false)]
    );
    assert_eq!(first_match("Next.js", "t.j"), Some(3..6));
}

#[test]
fn every_match_none_overlapping() {
    assert_eq!(
        parts("Banana", "an", Occurrences::All),
        [("B", false), ("an", true), ("an", true), ("a", false)]
    );
    assert_eq!(match_ranges("aaaa", "aa"), [0..2, 2..4]);
}

#[test]
fn an_empty_query_or_none_matching_leaves_the_text_as_it_is() {
    assert_eq!(parts("Remix", "", Occurrences::First), [("Remix", false)]);
    assert_eq!(parts("Remix", "zz", Occurrences::All), [("Remix", false)]);
    assert!(parts("", "a", Occurrences::All).is_empty());
    assert!(match_ranges("Remix", "").is_empty());
    // A query longer than the text.
    assert!(match_ranges("Re", "Remix").is_empty());
}

#[test]
fn the_runs_join_back_to_the_text() {
    for (text, query) in [
        ("São Paulo (gru1)", "o"),
        ("İstanbul İzmir", "i"),
        ("東京 Tokyo", "京"),
        ("ΟΔΟΣ ΣΟΦΟΣ", "ος"),
        ("👩‍💻 Dev", "dev"),
    ] {
        let joined: String = split(text, query, Occurrences::All)
            .iter()
            .map(|p| p.text)
            .collect();
        assert_eq!(joined, text);
        let ranges: Vec<_> = runs(text, query, Occurrences::All);
        assert!(
            ranges
                .iter()
                .all(|(r, _)| text.is_char_boundary(r.start) && text.is_char_boundary(r.end)),
            "{text}"
        );
    }
}

#[test]
fn characters_of_more_than_one_byte() {
    // "Ã" is two bytes: the match ends after it.
    assert_eq!(first_match("SÃO PAULO", "são"), Some(0..4));
    assert_eq!(first_match("東京タワー", "京"), Some(3..6));
    assert_eq!(first_match("Zürich", "ÜR"), Some(1..4));
}

#[test]
fn folds_that_change_the_length() {
    // "İ" (2 bytes) lowers to "i̇" (3): a query of its first letter takes the
    // whole character, and the rest of the title stays in place.
    assert_eq!(first_match("İstanbul", "i"), Some(0..2));
    assert_eq!(first_match("İstanbul", "stan"), Some(2..6));
    assert_eq!(first_match("İstanbul", "İst"), Some(0..4));
    // "ẞ" (3 bytes) lowers to "ß" (2); the Kelvin sign (3) to "k" (1).
    assert_eq!(first_match("GROẞ", "ß"), Some(3..6));
    assert_eq!(first_match("5 \u{212A}", "k"), Some(2..5));
    // A final sigma lowers to "ς", as the filter folds it.
    assert_eq!(first_match("ΟΔΟΣ", "ος"), Some(4..8));
    assert_eq!(first_match("ΟΔΟΣ", "οσ"), None);
}

#[test]
fn it_matches_what_the_filter_matches() {
    for (text, query) in [
        ("İstanbul", "ist"),
        ("İstanbul", "i̇st"),
        ("Straße", "STRASSE"),
        ("ΟΔΟΣ", "οσ"),
        ("Next.js", "next"),
        ("5 \u{212A}", "k"),
    ] {
        let filtered = text.to_lowercase().contains(&query.to_lowercase());
        assert_eq!(matches(text, query), filtered, "{text} {query}");
        assert_eq!(
            first_match(text, query).is_some(),
            filtered,
            "{text} {query}"
        );
    }
}
