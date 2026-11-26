//! `Badge` in a headless engine: its size on both kinds of screens, its
//! variants' fills, an icon's size inside it, what screen readers get, and
//! a scope's tokens reaching its look.

mod common;

use common::{css, fill, light, size, start};
use nsh_core::prelude::*;
use nsh_longbridge::{
    BadgeVariant, IconName, ScopeTokens, badge,
    components::badge::{BADGE_DEFAULT, BADGE_DESTRUCTIVE, BADGE_SECONDARY},
    icon, theme_scope,
};

/// Each variant, one with an icon, and one in a scope.
#[component]
fn Badges() -> impl IntoElement {
    view()
        .items_start()
        .gap(px(8.0))
        .child(badge().test_id("default").child("Badge"))
        .child(
            badge()
                .test_id("secondary")
                .variant(BadgeVariant::Secondary)
                .child("Secondary"),
        )
        .child(
            badge()
                .test_id("destructive")
                .variant(BadgeVariant::Destructive)
                .child("Destructive"),
        )
        .child(
            badge()
                .test_id("outline")
                .variant(BadgeVariant::Outline)
                .child("Outline"),
        )
        .child(
            badge()
                .test_id("icon")
                .variant(BadgeVariant::Secondary)
                .child(icon().name(IconName::Plus).test_id("glyph"))
                .child("Verified"),
        )
        .child(
            theme_scope()
                .tokens(ScopeTokens::new().set("primary", "#2563eb"))
                .child(badge().test_id("scoped").child("Scoped")),
        )
        .child(
            view()
                .w(px(60.0))
                .child(badge().test_id("long").child("A very long badge")),
        )
}

#[test]
fn a_badge_is_22_high_on_every_screen() {
    for desktop in [true, false] {
        let app = start::<Badges>(desktop);
        for id in ["default", "outline", "icon"] {
            assert_eq!(
                size(&app, &format!("id:{id}")).1,
                22.0,
                "{id}, desktop {desktop}"
            );
        }
    }
}

#[test]
fn variants_take_their_fills() {
    let app = start::<Badges>(true);
    assert_eq!(fill(&app, "id:default"), light(BADGE_DEFAULT));
    assert_eq!(fill(&app, "id:secondary"), light(BADGE_SECONDARY));
    assert_eq!(fill(&app, "id:destructive"), light(BADGE_DESTRUCTIVE));
    assert_eq!(fill(&app, "id:outline"), None);
}

#[test]
fn an_icon_in_a_badge_is_12() {
    let app = start::<Badges>(true);
    assert_eq!(size(&app, "id:glyph"), (12.0, 12.0));
}

#[test]
fn a_badge_is_as_wide_as_its_content_and_no_wider_than_its_parent() {
    let app = start::<Badges>(true);
    let (w, _) = size(&app, "id:default");
    assert!(w < 80.0, "as wide as its text, not its parent: {w}");
    assert!(size(&app, "id:long").0 <= 60.0);
}

#[test]
fn screen_readers_read_its_text_and_cannot_focus_it() {
    let app = start::<Badges>(true);
    let screen = app.screen();
    assert!(screen.shows("Badge"));
    assert!(!screen.matches("role:button[Badge]").unwrap());
}

#[test]
fn a_scope_primary_reaches_the_default_fill() {
    let app = start::<Badges>(true);
    assert_eq!(fill(&app, "id:scoped"), css("#2563eb"));
}
