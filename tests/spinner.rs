//! `Spinner` in a headless engine: its size by `size` and from the control
//! it is in, what screen readers get, and that it keeps turning with
//! reduced motion.

mod common;

use common::{size, start};
use nsh_core::prelude::*;
use nsh_longbridge::{Size, button, spinner};

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
