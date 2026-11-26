//! `Separator` in a headless engine: it spans its parent across, takes
//! `separator-thickness`, stops stretching at an explicit length, is out of
//! the accessibility tree, and a scope's tokens reach it.

mod common;

use common::{css, fill, light, size, start};
use nsh_core::{geometry::Axis, prelude::*};
use nsh_longbridge::{ScopeTokens, components::separator::SEPARATOR, separator, theme_scope};

/// A horizontal separator in a column, vertical ones in a row (one shorter
/// by `h`), and one in a scope.
#[component]
fn Lines() -> impl IntoElement {
    view()
        .w(px(300.0))
        .gap(px(16.0))
        .child(text("Above"))
        .child(separator().test_id("horizontal"))
        .child(
            view()
                .flex_row()
                .items_center()
                .h(px(20.0))
                .gap(px(16.0))
                .child(text("Blog"))
                .child(separator().test_id("vertical").orientation(Axis::Vertical))
                .child(text("Docs"))
                .child(
                    separator()
                        .test_id("short")
                        .orientation(Axis::Vertical)
                        .h(px(12.0)),
                ),
        )
        .child(
            theme_scope()
                .tokens(
                    ScopeTokens::new()
                        .set("separator", "#2563eb")
                        .set("separator-thickness", 2),
                )
                .child(separator().test_id("scoped")),
        )
}

#[test]
fn a_separator_spans_its_parent() {
    let app = start::<Lines>(true);
    assert_eq!(size(&app, "id:horizontal"), (300.0, 1.0));
    assert_eq!(size(&app, "id:vertical"), (1.0, 20.0));
}

#[test]
fn an_explicit_length_stops_the_stretch() {
    let app = start::<Lines>(true);
    assert_eq!(size(&app, "id:short"), (1.0, 12.0));
}

#[test]
fn its_color_is_separator_and_a_scope_reaches_it() {
    let app = start::<Lines>(true);
    assert_eq!(fill(&app, "id:horizontal"), light(SEPARATOR));
    assert_eq!(fill(&app, "id:scoped"), css("#2563eb"));
    assert_eq!(size(&app, "id:scoped").1, 2.0);
}

#[test]
fn screen_readers_skip_it() {
    let app = start::<Lines>(true);
    let screen = app.screen();
    for id in ["id:horizontal", "id:vertical"] {
        let node = screen.accessibility(Some(id)).unwrap();
        assert!(
            node.as_ref()
                .is_none_or(|n| n.role.is_none() && n.name.is_empty()),
            "{id}: {node:?}"
        );
    }
}
