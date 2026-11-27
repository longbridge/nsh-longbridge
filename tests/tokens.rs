//! The token registry, `themes/default.json` and the docs' token tables
//! agree; reads resolve scope → app → derived default.
//!
//! `NSH_LONGBRIDGE_WRITE=1 cargo test --test tokens` rewrites the generated
//! files instead of checking them.

use std::path::{Path, PathBuf};

use nsh_core::{Color, element::AdaptiveColor, prelude::*};
use nsh_longbridge::{
    ScopeTokens, ThemeAppearance,
    components::{
        button::{
            BUTTON_DEFAULT, BUTTON_DEFAULT_HOVER, BUTTON_HEIGHT_MD, BUTTON_OUTLINE_BORDER,
            BUTTON_RADIUS, TOKENS as BUTTON_TOKENS,
        },
        checkbox::TOKENS as CHECKBOX_TOKENS,
        radio_group::TOKENS as RADIO_GROUP_TOKENS,
        switch::TOKENS as SWITCH_TOKENS,
    },
    theme::{
        default_theme_json, registry, resolve_color_in, resolve_number_in,
        semantic::{BORDER, INPUT, PRIMARY, RADIUS_MD, RADIUS_XS},
        token_table,
    },
};

fn root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).to_path_buf()
}

fn writing() -> bool {
    std::env::var_os("NSH_LONGBRIDGE_WRITE").is_some()
}

/// `path` holds `content`, or is rewritten to when writing.
fn check_file(path: &Path, content: &str) {
    if writing() {
        std::fs::write(path, content).unwrap();
        return;
    }
    let on_disk = std::fs::read_to_string(path).unwrap_or_default();
    assert!(
        on_disk == content,
        "{} is out of date: run `NSH_LONGBRIDGE_WRITE=1 cargo test --test tokens`",
        path.display()
    );
}

/// The generated block between `<!-- tokens:{name} -->` and
/// `<!-- /tokens -->` in `path`.
fn check_block(path: &Path, name: &str, content: &str) {
    let text = std::fs::read_to_string(path).unwrap();
    let open = format!("<!-- tokens:{name} -->\n");
    let close = "<!-- /tokens -->";
    let start = text
        .find(&open)
        .unwrap_or_else(|| panic!("{} has no {open}", path.display()));
    let end = start
        + open.len()
        + text[start + open.len()..]
            .find(close)
            .unwrap_or_else(|| panic!("{} has no {close}", path.display()));
    let block = &text[start + open.len()..end];
    if writing() {
        let new = format!("{}{content}{}", &text[..start + open.len()], &text[end..]);
        std::fs::write(path, new).unwrap();
        return;
    }
    assert!(
        block == content,
        "the token table in {} is out of date: run `NSH_LONGBRIDGE_WRITE=1 cargo test --test tokens`",
        path.display()
    );
}

fn default_tokens() -> nsh_core::element::Tokens {
    nsh_base::Theme::default().into_tokens()
}

fn rgba(c: Hsla) -> u32 {
    Color::from(c).0
}

fn close(a: AdaptiveColor, b: AdaptiveColor) -> bool {
    let near = |x: Hsla, y: Hsla| {
        let (x, y) = (rgba(x).to_be_bytes(), rgba(y).to_be_bytes());
        x.iter().zip(y).all(|(a, b)| a.abs_diff(b) <= 2)
    };
    near(a.light, b.light) && near(a.dark, b.dark)
}

#[test]
fn the_default_theme_file_is_current() {
    check_file(&root().join("themes/default.json"), &default_theme_json());
}

#[test]
fn the_button_token_table_is_current() {
    check_block(
        &root().join("docs/components/button.md"),
        "button",
        &token_table(BUTTON_TOKENS),
    );
}

#[test]
fn the_input_token_table_is_current() {
    check_block(
        &root().join("docs/components/input.md"),
        "input",
        &token_table(nsh_longbridge::foundation::field::TOKENS),
    );
}

#[test]
fn the_badge_token_table_is_current() {
    check_block(
        &root().join("docs/components/badge.md"),
        "badge",
        &token_table(nsh_longbridge::components::badge::TOKENS),
    );
}

#[test]
fn the_checkbox_token_table_is_current() {
    check_block(
        &root().join("docs/components/checkbox.md"),
        "checkbox",
        &token_table(CHECKBOX_TOKENS),
    );
}

#[test]
fn the_card_token_table_is_current() {
    check_block(
        &root().join("docs/components/card.md"),
        "card",
        &token_table(nsh_longbridge::components::card::TOKENS),
    );
}

#[test]
fn the_label_token_table_is_current() {
    check_block(
        &root().join("docs/components/label.md"),
        "label",
        &token_table(nsh_longbridge::components::label::TOKENS),
    );
}

#[test]
fn the_kbd_token_table_is_current() {
    check_block(
        &root().join("docs/components/kbd.md"),
        "kbd",
        &token_table(nsh_longbridge::components::kbd::TOKENS),
    );
}

#[test]
fn the_switch_token_table_is_current() {
    check_block(
        &root().join("docs/components/switch.md"),
        "switch",
        &token_table(SWITCH_TOKENS),
    );
}

#[test]
fn the_separator_token_table_is_current() {
    check_block(
        &root().join("docs/components/separator.md"),
        "separator",
        &token_table(nsh_longbridge::components::separator::TOKENS),
    );
}

#[test]
fn the_textarea_token_table_is_current() {
    check_block(
        &root().join("docs/components/textarea.md"),
        "textarea",
        &token_table(nsh_longbridge::components::textarea::TOKENS),
    );
}

#[test]
fn the_skeleton_token_table_is_current() {
    check_block(
        &root().join("docs/components/skeleton.md"),
        "skeleton",
        &token_table(nsh_longbridge::components::skeleton::TOKENS),
    );
}

#[test]
fn the_radio_group_token_table_is_current() {
    check_block(
        &root().join("docs/components/radio-group.md"),
        "radio-group",
        &token_table(RADIO_GROUP_TOKENS),
    );
}

#[test]
fn the_tooltip_token_table_is_current() {
    check_block(
        &root().join("docs/components/tooltip.md"),
        "tooltip",
        &token_table(nsh_longbridge::components::tooltip::TOKENS),
    );
}

#[test]
fn the_popover_token_table_is_current() {
    check_block(
        &root().join("docs/components/popover.md"),
        "popover",
        &token_table(
            &[
                nsh_longbridge::foundation::popup::TOKENS,
                nsh_longbridge::components::popover::TOKENS,
            ]
            .concat(),
        ),
    );
}

#[test]
fn the_dialog_token_table_is_current() {
    check_block(
        &root().join("docs/components/dialog.md"),
        "dialog",
        &token_table(nsh_longbridge::components::dialog::TOKENS),
    );
}

#[test]
fn the_alert_dialog_token_table_is_current() {
    check_block(
        &root().join("docs/components/alert-dialog.md"),
        "alert-dialog",
        &token_table(nsh_longbridge::components::alert_dialog::TOKENS),
    );
}

#[test]
fn the_toast_token_table_is_current() {
    check_block(
        &root().join("docs/components/toast.md"),
        "toast",
        &token_table(nsh_longbridge::components::toast::TOKENS),
    );
}

#[test]
fn the_select_token_table_is_current() {
    check_block(
        &root().join("docs/components/select.md"),
        "select",
        &token_table(nsh_longbridge::components::select::TOKENS),
    );
}

#[test]
fn the_tabs_token_table_is_current() {
    check_block(
        &root().join("docs/components/tabs.md"),
        "tabs",
        &token_table(nsh_longbridge::components::tabs::TOKENS),
    );
}

#[test]
fn the_menu_token_table_is_current() {
    check_block(
        &root().join("docs/components/dropdown-menu.md"),
        "menu",
        &token_table(nsh_longbridge::foundation::menu::TOKENS),
    );
}

#[test]
fn names_are_unique_and_kebab_case() {
    let decls = registry();
    let mut seen = std::collections::HashSet::new();
    for d in &decls {
        assert!(seen.insert(d.name), "`{}` declared twice", d.name);
        assert!(
            d.name
                .bytes()
                .all(|b| b.is_ascii_lowercase() || b.is_ascii_digit() || b == b'-'),
            "`{}` is not kebab-case",
            d.name
        );
        assert!(!d.doc.trim().is_empty(), "`{}` has no doc", d.name);
    }
}

/// NSH reads the file whole (as `setTheme` must), and each color it reads
/// is what the library derives.
#[test]
fn nsh_reads_the_default_theme_file_as_the_library_resolves_it() {
    let json = default_theme_json();
    let (theme, ignored) = nsh_base::Theme::from_json_lenient(&json).unwrap();
    assert!(ignored.is_empty(), "NSH left out: {ignored:?}");
    let tokens = theme.into_tokens();
    for decl in registry() {
        if let nsh_longbridge::theme::DeclDefault::Color(default) = decl.default {
            let token = nsh_longbridge::theme::Token::new(decl.name, default);
            let ours = resolve_color_in(&default_tokens(), &[], true, token);
            let read = tokens
                .color(decl.name)
                .unwrap_or_else(|| panic!("NSH has no `{}`", decl.name));
            assert!(
                close(ours, read),
                "`{}`: the library {ours:?}, NSH {read:?}",
                decl.name
            );
        }
    }
}

/// A duration or a delay is whole milliseconds on the default theme, also
/// a derived one (`dialog-duration`: 4/3 of 150 is 200, not 199.99).
#[test]
fn durations_are_whole_milliseconds() {
    let app = default_tokens();
    let mut seen = 0;
    for decl in registry() {
        let nsh_longbridge::theme::DeclDefault::Number(default) = decl.default else {
            continue;
        };
        if !(decl.name.contains("duration") || decl.name.contains("delay")) {
            continue;
        }
        seen += 1;
        let token = nsh_longbridge::theme::Token::<nsh_longbridge::theme::kind::Number>::new(
            decl.name, default,
        );
        for pointer in [true, false] {
            let ms = resolve_number_in(&app, &[], pointer, token);
            assert_eq!(ms, ms.round(), "`{}` is {ms} ms", decl.name);
        }
    }
    assert!(seen > 0, "no durations in the registry");
    // The theme file writes it so CSS arithmetic gets 200 too (a product by
    // 1.3333 gave 199.99).
    let json: serde_json::Value = serde_json::from_str(&default_theme_json()).unwrap();
    assert_eq!(
        json["cssVars"]["theme"]["dialog-duration"],
        "calc(var(--duration) * 4 / 3)"
    );
    assert_eq!(
        resolve_number_in(
            &app,
            &[],
            true,
            nsh_longbridge::components::dialog::DIALOG_DURATION
        ),
        200.0
    );
}

#[test]
fn derived_defaults_follow_their_sources() {
    let app = default_tokens();
    assert_eq!(resolve_number_in(&app, &[], true, RADIUS_MD), 8.0);
    assert_eq!(resolve_number_in(&app, &[], true, BUTTON_RADIUS), 8.0);
    assert_eq!(resolve_number_in(&app, &[], true, BUTTON_HEIGHT_MD), 32.0);
    assert_eq!(resolve_number_in(&app, &[], false, BUTTON_HEIGHT_MD), 40.0);
    let primary = resolve_color_in(&app, &[], true, PRIMARY);
    assert!(close(
        resolve_color_in(&app, &[], true, BUTTON_DEFAULT),
        primary
    ));
    assert!(close(
        resolve_color_in(&app, &[], true, BUTTON_DEFAULT_HOVER),
        primary.opacity(0.9)
    ));
    // Light and dark take different sources.
    let border = resolve_color_in(&app, &[], true, BORDER);
    let input = resolve_color_in(&app, &[], true, INPUT);
    let outline = resolve_color_in(&app, &[], true, BUTTON_OUTLINE_BORDER);
    assert_eq!(rgba(outline.light), rgba(border.light));
    assert_eq!(rgba(outline.dark), rgba(input.dark));
}

/// `radius-xs` is `max(radius − 8, 0)`: 2 by default, never below 0, and
/// written into the theme file with CSS's `max()`.
#[test]
fn radius_xs_never_goes_below_zero() {
    let mut app = default_tokens();
    assert_eq!(resolve_number_in(&app, &[], true, RADIUS_XS), 2.0);
    app.set_number("radius", 4.0);
    assert_eq!(resolve_number_in(&app, &[], true, RADIUS_XS), 0.0);
    app.set_number("radius", 20.0);
    assert_eq!(resolve_number_in(&app, &[], true, RADIUS_XS), 12.0);
    assert!(default_theme_json().contains(r#""radius-xs": "calc(max(var(--radius) - 8px, 0px))""#));
}

#[test]
fn the_app_overrides_defaults_and_seeds() {
    let mut app = default_tokens();
    app.set_number("radius", 0.0);
    app.set_number("control-height", 28.0);
    assert_eq!(
        resolve_number_in(&app, &[], true, BUTTON_RADIUS),
        -2.0_f32.max(-2.0)
    );
    assert_eq!(resolve_number_in(&app, &[], true, BUTTON_HEIGHT_MD), 28.0);
    app.set_number("button-radius", 4.0);
    assert_eq!(resolve_number_in(&app, &[], true, BUTTON_RADIUS), 4.0);
    // `<name>-touch` wins on touch screens only.
    app.set_number("button-height-md-touch", 44.0);
    assert_eq!(resolve_number_in(&app, &[], false, BUTTON_HEIGHT_MD), 44.0);
    assert_eq!(resolve_number_in(&app, &[], true, BUTTON_HEIGHT_MD), 28.0);
}

#[test]
fn a_scope_wins_and_its_primary_reaches_derived_tokens() {
    let app = default_tokens();
    let scope = ScopeTokens::new()
        .set("primary", "#2563eb")
        .set("button-radius", "0px");
    let blue = AdaptiveColor::from(Hsla::from(Color::parse("#2563eb").unwrap()));
    let scopes = [(scope, None)];
    assert!(close(resolve_color_in(&app, &scopes, true, PRIMARY), blue));
    assert!(close(
        resolve_color_in(&app, &scopes, true, BUTTON_DEFAULT),
        blue
    ));
    assert!(close(
        resolve_color_in(&app, &scopes, true, BUTTON_DEFAULT_HOVER),
        blue.opacity(0.9)
    ));
    assert_eq!(resolve_number_in(&app, &scopes, true, BUTTON_RADIUS), 0.0);
    // Inner scopes win over outer ones; the outer one's other values stay.
    let inner = ScopeTokens::new().set("primary", "#e11d48");
    let rose = AdaptiveColor::from(Hsla::from(Color::parse("#e11d48").unwrap()));
    let nested = [scopes[0].clone(), (inner, None)];
    assert!(close(
        resolve_color_in(&app, &nested, true, BUTTON_DEFAULT),
        rose
    ));
    assert_eq!(resolve_number_in(&app, &nested, true, BUTTON_RADIUS), 0.0);
}

/// An app theme that copied `themes/default.json` (its `var()`s resolved
/// when NSH read it) does not stop a scope's `primary` reaching
/// `button-default`.
#[test]
fn a_copied_default_theme_does_not_pin_derived_tokens() {
    let json = default_theme_json();
    let app = nsh_base::Theme::from_json(&json).unwrap().into_tokens();
    let scopes = [(ScopeTokens::new().set("primary", "#2563eb"), None)];
    let blue = AdaptiveColor::from(Hsla::from(Color::parse("#2563eb").unwrap()));
    assert!(close(
        resolve_color_in(&app, &scopes, true, BUTTON_DEFAULT),
        blue
    ));
    // A real override stays the app's.
    let mut app = app;
    app.set_color("button-default", AdaptiveColor::rgb(0x00ff00, 0x00ff00));
    assert!(close(
        resolve_color_in(&app, &scopes, true, BUTTON_DEFAULT),
        AdaptiveColor::rgb(0x00ff00, 0x00ff00)
    ));
}

#[test]
fn a_forced_appearance_takes_that_appearances_values() {
    let app = default_tokens();
    let primary = resolve_color_in(&app, &[], true, PRIMARY);
    let dark = resolve_color_in(
        &app,
        &[(ScopeTokens::new(), Some(ThemeAppearance::Dark))],
        true,
        BUTTON_DEFAULT,
    );
    assert_eq!(rgba(dark.light), rgba(primary.dark));
    assert_eq!(rgba(dark.dark), rgba(primary.dark));
    let light = resolve_color_in(
        &app,
        &[(ScopeTokens::new(), Some(ThemeAppearance::Light))],
        true,
        BUTTON_DEFAULT,
    );
    assert_eq!(rgba(light.dark), rgba(primary.light));
    // A nested scope keeps the outer one's forced appearance unless it sets its own.
    let nested = [
        (ScopeTokens::new(), Some(ThemeAppearance::Dark)),
        (ScopeTokens::new().set("button-radius", 0), None),
    ];
    assert_eq!(
        rgba(resolve_color_in(&app, &nested, true, BUTTON_DEFAULT).light),
        rgba(primary.dark)
    );
}

#[test]
fn scope_values_take_css_forms() {
    let app = default_tokens();
    for (value, expected) in [("12px", 12.0), ("0.5rem", 8.0), ("3", 3.0)] {
        let scopes = [(ScopeTokens::new().set("button-radius", value), None)];
        assert_eq!(
            resolve_number_in(&app, &scopes, true, BUTTON_RADIUS),
            expected,
            "{value}"
        );
    }
    let scopes = [(ScopeTokens::new().set("button-radius", 6), None)];
    assert_eq!(resolve_number_in(&app, &scopes, true, BUTTON_RADIUS), 6.0);
    let scopes = [(
        ScopeTokens::new().set_light_dark("primary", "#ffffff", "#000000"),
        None,
    )];
    let c = resolve_color_in(&app, &scopes, true, BUTTON_DEFAULT);
    assert_eq!(rgba(c.light), 0xffffffff);
    assert_eq!(rgba(c.dark), 0x000000ff);
    // What is no value is ignored.
    let scopes = [(ScopeTokens::new().set("primary", "not a color"), None)];
    assert!(close(
        resolve_color_in(&app, &scopes, true, BUTTON_DEFAULT),
        resolve_color_in(&app, &[], true, PRIMARY)
    ));
}

/// The presets in `themes/` are theme files NSH reads whole (`setTheme`
/// refuses a file with a value it leaves out).
#[test]
fn nsh_reads_every_preset_whole() {
    for entry in std::fs::read_dir(root().join("themes")).unwrap() {
        let path = entry.unwrap().path();
        let json = std::fs::read_to_string(&path).unwrap();
        let (_, ignored) = nsh_base::Theme::from_json_lenient(&json)
            .unwrap_or_else(|e| panic!("{}: {e}", path.display()));
        assert!(
            ignored.is_empty(),
            "{}: NSH left out {ignored:?}",
            path.display()
        );
    }
}
