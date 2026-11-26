//! `Card` in a headless engine: its parts' padding and gaps, the header's
//! column for an action (only with one), the title's heading role, text
//! color reaching the app's own text, and a scope's tokens reaching it.

mod common;

use common::{bounds, css, fill, light, start};
use nsh_core::prelude::*;
use nsh_longbridge::{
    ScopeTokens, button, card, card_action, card_content, card_description, card_footer,
    card_header, card_title, theme::semantic::CARD, theme_scope,
};

/// A card with an action, one without, and one in a scope.
#[component]
fn Cards() -> impl IntoElement {
    view()
        .items_start()
        .gap(px(16.0))
        .child(
            card()
                .test_id("card")
                .w(px(360.0))
                .child(
                    card_header()
                        .test_id("header")
                        .child(card_title().test_id("title").child("Login"))
                        .child(
                            card_description()
                                .test_id("description")
                                .child("Enter your email"),
                        )
                        .child(
                            card_action()
                                .test_id("action")
                                .child(button().child("Sign Up")),
                        ),
                )
                .child(card_content().test_id("content").child(view().h(px(40.0))))
                .child(
                    card_footer()
                        .test_id("footer")
                        .child(button().child("Login")),
                ),
        )
        .child(
            card().w(px(360.0)).child(
                card_header()
                    .child(card_title().test_id("bare-title").child("No action"))
                    .child(card_description().child("Just text")),
            ),
        )
        .child(
            theme_scope()
                .tokens(
                    ScopeTokens::new()
                        .set("card", "#2563eb")
                        .set("card-foreground", "#e11d48")
                        .set("card-padding", 16),
                )
                .child(
                    card()
                        .test_id("scoped")
                        .w(px(360.0))
                        .child(card_content().test_id("scoped-content").child(text("Body"))),
                ),
        )
}

#[test]
fn parts_are_padded_and_spaced_by_tokens() {
    let app = start::<Cards>(true);
    let card = bounds(&app, "id:card");
    let header = bounds(&app, "id:header");
    let content = bounds(&app, "id:content");
    let footer = bounds(&app, "id:footer");
    // `card-padding` (24) above the first part (inside the 1pt border).
    assert_eq!(header.origin.y.0 - card.origin.y.0, 25.0);
    // Each part 24 in from the sides.
    assert_eq!(content.origin.x.0 - card.origin.x.0, 1.0);
    assert_eq!(content.size.width.0, 358.0);
    let title = bounds(&app, "id:title");
    assert_eq!(title.origin.x.0 - card.origin.x.0, 25.0);
    // `card-gap` (24) between parts.
    assert_eq!(
        content.origin.y.0 - (header.origin.y.0 + header.size.height.0),
        24.0
    );
    assert_eq!(
        footer.origin.y.0 - (content.origin.y.0 + content.size.height.0),
        24.0
    );
    // `card-padding` below the last part.
    let bottom = card.origin.y.0 + card.size.height.0;
    assert_eq!(bottom - (footer.origin.y.0 + footer.size.height.0), 25.0);
}

#[test]
fn the_header_makes_a_column_for_an_action_only() {
    let app = start::<Cards>(true);
    let card = bounds(&app, "id:card");
    let action = bounds(&app, "id:action");
    let title = bounds(&app, "id:title");
    // The action at the right edge, beside the title, at the top.
    let right = card.origin.x.0 + card.size.width.0 - 25.0;
    assert_eq!(action.origin.x.0 + action.size.width.0, right);
    assert_eq!(action.origin.y.0, title.origin.y.0);
    assert!(title.origin.x.0 + title.size.width.0 <= action.origin.x.0 - 8.0);
    // Without an action the title spans the header: no empty column, no gap.
    let bare = bounds(&app, "id:bare-title");
    assert_eq!(bare.size.width.0, 360.0 - 2.0 - 48.0);
}

#[test]
fn the_title_is_a_heading() {
    let app = start::<Cards>(true);
    assert!(app.screen().matches("role:header[Login]").unwrap());
    // 16pt, `leading-none`: one line 16 high.
    assert_eq!(bounds(&app, "id:title").size.height.0, 16.0);
    assert_eq!(bounds(&app, "id:description").size.height.0, 20.0);
}

#[test]
fn the_fill_is_card_and_a_scope_reaches_it() {
    let app = start::<Cards>(true);
    assert_eq!(fill(&app, "id:card"), light(CARD));
    assert_eq!(fill(&app, "id:scoped"), css("#2563eb"));
    let scoped = bounds(&app, "id:scoped");
    let content = bounds(&app, "id:scoped-content");
    assert_eq!(content.origin.y.0 - scoped.origin.y.0, 17.0);
}

/// The app's own text inside a card takes `card-foreground` (text styles
/// inherit from the card's root).
#[test]
fn the_apps_text_inherits_card_foreground() {
    let app = start::<Cards>(true);
    let color = app.screen().span_style("text:Body").unwrap().color;
    assert_eq!(Some(color.0), css("#e11d48"));
}
