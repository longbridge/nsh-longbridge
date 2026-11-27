//! `Spinner` in a headless engine: its size by `size` and from the control
//! it is in, its color in and out of the library's parts, and what screen
//! readers get.

mod common;

use common::{dark, size, start, start_dark, stroke};
use nsh_core::{Color, prelude::*};
use nsh_longbridge::{Size, button, card, spinner, theme::semantic};

/// Spinners of each size, one without, one with a label, and one inside a
/// small button.
#[component]
fn Spinners() -> impl IntoElement {
    view()
        .items_start()
        .gap(px(8.0))
        .child(spinner().test_id("xs").size(Size::Xs))
        .child(spinner().test_id("sm").size(Size::Sm))
        .child(spinner().test_id("md"))
        .child(spinner().test_id("lg").size(Size::Lg))
        .child(
            spinner()
                .test_id("orders")
                .accessibility_label("Loading orders"),
        )
        .child(
            button()
                .size(Size::Sm)
                .child(spinner().test_id("in-button"))
                .child("Syncing"),
        )
}

#[test]
fn its_side_follows_size_and_the_control_around_it() {
    for desktop in [true, false] {
        let app = start::<Spinners>(desktop);
        assert_eq!(size(&app, "id:xs"), (12.0, 12.0));
        assert_eq!(size(&app, "id:sm"), (14.0, 14.0));
        assert_eq!(size(&app, "id:md"), (16.0, 16.0));
        assert_eq!(size(&app, "id:lg"), (16.0, 16.0));
        assert_eq!(size(&app, "id:in-button"), (14.0, 14.0));
    }
}

#[test]
fn screen_readers_hear_a_busy_progress_indicator() {
    let app = start::<Spinners>(true);
    let screen = app.screen();
    assert!(screen.matches("role:progressbar[Loading]").unwrap());
    let orders = screen
        .accessibility_state("role:progressbar[Loading orders]")
        .unwrap();
    assert!(orders.busy);
}

/// A spinner in a plain view, one in a button, one in a card, and one
/// given a color by a class.
#[component]
fn Colors() -> impl IntoElement {
    view()
        .items_start()
        .gap(px(8.0))
        .child(view().child(spinner().test_id("plain")))
        .child(
            button()
                .child(spinner().test_id("in-button"))
                .child("Saving"),
        )
        .child(card().child(spinner().test_id("in-card")))
        .child(
            view().child(
                spinner()
                    .test_id("classed")
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
fn inside_a_button_or_a_card_it_takes_their_text_color() {
    let (app, strokes) = start_dark::<Colors>();
    let in_button = stroke(&app, &strokes, "id:in-button");
    assert_eq!(in_button, dark(semantic::PRIMARY_FOREGROUND));
    assert_ne!(in_button, dark(semantic::FOREGROUND));
    assert_eq!(
        stroke(&app, &strokes, "id:in-card"),
        dark(semantic::CARD_FOREGROUND)
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
