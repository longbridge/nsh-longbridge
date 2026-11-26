//! `Kbd` and `KbdGroup` in a headless engine: a cap's size (square for one
//! character), the group's gap, its fill, and a scope's tokens reaching it.
//! Its colors inside a `Tooltip` are checked in `tests/tooltip.rs`.

mod common;

use common::{bounds, css, fill, light, size, start};
use nsh_core::prelude::*;
use nsh_longbridge::{ScopeTokens, components::kbd::KBD, kbd, kbd_group, theme_scope};

/// One-character and longer caps, a group, and a cap in a scope.
#[component]
fn Caps() -> impl IntoElement {
    view()
        .items_start()
        .gap(px(8.0))
        .child(kbd().test_id("k").child("K"))
        .child(kbd().test_id("ctrl").child("Ctrl"))
        .child(
            kbd_group()
                .test_id("group")
                .child(kbd().test_id("first").child("⌘"))
                .child(kbd().test_id("second").child("K")),
        )
        .child(
            theme_scope()
                .tokens(
                    ScopeTokens::new()
                        .set("kbd", "#2563eb")
                        .set("kbd-height", 24),
                )
                .child(kbd().test_id("scoped").child("S")),
        )
}

#[test]
fn a_cap_is_20_high_and_square_for_one_character() {
    for desktop in [true, false] {
        let app = start::<Caps>(desktop);
        assert_eq!(size(&app, "id:k"), (20.0, 20.0), "desktop {desktop}");
        let (w, h) = size(&app, "id:ctrl");
        assert_eq!(h, 20.0);
        assert!(w > 20.0, "a longer name widens it: {w}");
    }
}

#[test]
fn a_group_spaces_its_caps() {
    let app = start::<Caps>(true);
    let first = bounds(&app, "id:first");
    let second = bounds(&app, "id:second");
    assert_eq!(
        second.origin.x.0 - (first.origin.x.0 + first.size.width.0),
        4.0
    );
    assert_eq!(second.origin.y.0, first.origin.y.0);
}

#[test]
fn the_fill_is_kbd_and_a_scope_reaches_it() {
    let app = start::<Caps>(true);
    assert_eq!(fill(&app, "id:k"), light(KBD));
    assert_eq!(fill(&app, "id:scoped"), css("#2563eb"));
    assert_eq!(size(&app, "id:scoped").1, 24.0);
}

#[test]
fn a_cap_is_read_as_its_text() {
    let app = start::<Caps>(true);
    assert!(app.shows("Ctrl"));
    assert!(!app.screen().matches("role:keyboardkey[Ctrl]").unwrap());
}
