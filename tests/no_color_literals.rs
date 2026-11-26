//! Components hold no color literals: every color is a token.

use std::{fs, path::Path};

/// What a color literal starts with, in CSS or in NSH's Rust API.
const LITERALS: &[&str] = &[
    "rgb(",
    "rgba(",
    "hsl(",
    "hsla(",
    "oklch(",
    "oklab(",
    "lab(",
    "lch(",
    "Hsla {",
    "Rgba {",
    "Color::parse",
    "Color(0x",
];

fn hex_literal(line: &str) -> bool {
    line.match_indices('#').any(|(i, _)| {
        let digits = line[i + 1..]
            .chars()
            .take_while(char::is_ascii_hexdigit)
            .count();
        matches!(digits, 3 | 4 | 6 | 8)
            && !line[i + 1 + digits..]
                .chars()
                .next()
                .is_some_and(|c| c.is_alphanumeric() || c == '_')
    })
}

fn check(dir: &Path, found: &mut Vec<String>) {
    for entry in fs::read_dir(dir).unwrap() {
        let path = entry.unwrap().path();
        if path.is_dir() {
            check(&path, found);
            continue;
        }
        let source = fs::read_to_string(&path).unwrap();
        for (n, line) in source.lines().enumerate() {
            let code = line.split("//").next().unwrap_or("");
            if LITERALS.iter().any(|l| code.contains(l)) || hex_literal(code) {
                found.push(format!("{}:{}: {}", path.display(), n + 1, line.trim()));
            }
        }
    }
}

#[test]
fn components_have_no_color_literals() {
    let mut found = Vec::new();
    check(
        &Path::new(env!("CARGO_MANIFEST_DIR")).join("src/components"),
        &mut found,
    );
    assert!(found.is_empty(), "color literals:\n{}", found.join("\n"));
}

#[test]
fn the_check_sees_literals() {
    assert!(hex_literal(r##"bg("#2563eb")"##));
    assert!(hex_literal("#fff"));
    assert!(!hex_literal("#[component]"));
    assert!(!hex_literal("issue #12345"));
}
