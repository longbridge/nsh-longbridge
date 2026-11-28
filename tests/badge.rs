//! `Badge` in a headless engine: its size on both kinds of screens, a
//! one-character badge's circle, its variants' fills, an icon's size inside it, what screen readers get, and
//! a scope's tokens reaching its look.

mod common;

use common::{bounds, css, fill, light, size, start};
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
        .child(
            badge()
                .test_id("count")
                .app_class(Some("px-1".into()))
                .child("3"),
        )
}

/// Badges in a column that stretches its children, beside a two-line name
/// in a row that centers its children, and given a width by a class.
#[component]
fn Layouts() -> impl IntoElement {
    view()
        .w(px(400.0))
        .child(badge().test_id("in-column").child("Badge"))
        .child(
            view()
                .test_id("row")
                .flex_row()
                .items_center()
                .child(view().w(px(120.0)).h(px(40.0)))
                .child(badge().test_id("in-row").child("Owner")),
        )
        .child(
            badge()
                .test_id("classed")
                .app_class(Some("w-16".into()))
                .child("B"),
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

/// A count (one character, a tight padding as counts take) makes a
/// circle, not a pill narrower than it is tall.
#[test]
fn a_one_character_badge_is_as_wide_as_it_is_tall() {
    for desktop in [true, false] {
        let app = start::<Badges>(desktop);
        assert_eq!(size(&app, "id:count"), (22.0, 22.0), "desktop {desktop}");
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

#[test]
fn a_stretching_column_does_not_widen_it() {
    let app = start::<Layouts>(true);
    let (w, _) = size(&app, "id:in-column");
    assert!(w < 80.0, "as wide as its text, not the column: {w}");
}

#[test]
fn a_row_centers_it_beside_taller_content() {
    let app = start::<Layouts>(true);
    let row = bounds(&app, "id:row");
    let badge = bounds(&app, "id:in-row");
    let above = badge.origin.y.0 - row.origin.y.0;
    let below = row.size.height.0 - above - badge.size.height.0;
    assert_eq!(row.size.height.0, 40.0);
    assert_eq!(above, below, "centered in the row, not at its top");
}

#[test]
fn a_width_class_goes_over_its_own_width() {
    let app = start::<Layouts>(true);
    assert_eq!(size(&app, "id:classed").0, 64.0);
}
