//! `Icon` in a headless engine, in the dark appearance: its color outside
//! the library's parts, inside a button and a badge, and from a class.

mod common;

use common::{dark, start_dark, stroke};
use nsh_core::{Color, prelude::*};
use nsh_longbridge::{
    IconName, badge, button, components::badge::BADGE_DEFAULT_FOREGROUND, icon, theme::semantic,
};

/// An icon in a plain view, one in a button, one in a badge, and one given
/// a color by a class.
#[component]
fn Colors() -> impl IntoElement {
    view()
        .items_start()
        .gap(px(8.0))
        .child(view().child(icon().test_id("plain").name(IconName::Check)))
        .child(
            button()
                .child(icon().test_id("in-button").name(IconName::Plus))
                .child("Add"),
        )
        .child(
            badge()
                .child(icon().test_id("in-badge").name(IconName::Check))
                .child("Done"),
        )
        .child(
            view().child(
                icon()
                    .test_id("classed")
                    .name(IconName::Info)
                    .app_class(Some("text-red-500".into())),
            ),
        )
}

#[test]
fn outside_the_librarys_parts_it_is_foreground_not_black() {
    let (app, strokes) = start_dark::<Colors>();
    assert_eq!(
        stroke(&app, &strokes, "id:plain"),
        dark(semantic::FOREGROUND)
    );
}

#[test]
fn inside_a_button_or_a_badge_it_takes_their_text_color() {
    let (app, strokes) = start_dark::<Colors>();
    let in_button = stroke(&app, &strokes, "id:in-button");
    assert_eq!(in_button, dark(semantic::PRIMARY_FOREGROUND));
    assert_ne!(in_button, dark(semantic::FOREGROUND));
    assert_eq!(
        stroke(&app, &strokes, "id:in-badge"),
        dark(BADGE_DEFAULT_FOREGROUND)
    );
}

#[test]
fn a_class_color_goes_over_foreground() {
    let (app, strokes) = start_dark::<Colors>();
    assert_eq!(
        stroke(&app, &strokes, "id:classed"),
        Color::parse("oklch(63.7% 0.237 25.331)").unwrap()
    );
}
