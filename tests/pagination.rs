//! `Pagination` in a headless engine: the landmark and the items' names
//! and states, a press reporting the page, the current page's outline and
//! the others' ghost look (in a forced appearance too), the ends dimmed,
//! item heights on a pointer and on a touch screen.

mod common;

use std::time::Duration;

use common::{bounds, fill, light, start};
use nsh_core::prelude::*;
use nsh_longbridge::{
    ScopeTokens, ThemeAppearance, components::button::BUTTON_OUTLINE, pagination, theme_scope,
};
use nsh_testing::Headless;

/// Ten pages, the app keeping the page; a second one forced to dark.
#[component]
fn Pages() -> impl IntoElement {
    let page = signal(1u32);
    view()
        .child(
            pagination()
                .page(page)
                .total_pages(10)
                .on_change(move |next: &u32| page.set(*next)),
        )
        .child(text(move || format!("page:{}", page.get())))
        .child(
            theme_scope()
                .appearance(ThemeAppearance::Dark)
                .tokens(ScopeTokens::new())
                .child(
                    pagination()
                        .page(2)
                        .total_pages(3)
                        .accessibility_label("Dark pages"),
                ),
        )
}

fn opacity(app: &Headless, selector: &str) -> f32 {
    app.screen().view_style(selector).unwrap().unwrap().opacity
}

#[test]
fn screen_readers_get_the_landmark_and_the_items() {
    let app = start::<Pages>(true);
    let screen = app.screen();
    assert!(screen.matches("role:navigation[Pagination]").unwrap());
    assert!(
        screen
            .matches("role:navigation[Pagination] >> role:button[Go to previous page]")
            .unwrap()
    );
    assert!(
        screen
            .matches("role:navigation[Pagination] >> role:button[Go to next page]")
            .unwrap()
    );
    let one = screen
        .accessibility_state("role:navigation[Pagination] >> role:button[1]")
        .unwrap();
    assert!(one.selected);
    assert!(
        screen
            .accessibility_state("role:navigation[Pagination] >> role:button[Go to previous page]")
            .unwrap()
            .disabled
    );
    assert!(screen.shows("Previous"), "the word shows in a wide window");
}

#[test]
fn a_press_reports_the_page() {
    let mut app = start::<Pages>(true);
    app.press("role:navigation[Pagination] >> role:button[3]")
        .unwrap();
    app.wait_for("text:page:3").unwrap();
    app.press("role:navigation[Pagination] >> role:button[Go to next page]")
        .unwrap();
    app.wait_for("text:page:4").unwrap();
}

/// ★ NSH gaps 1: the current page draws the outline look and the others
/// the ghost look, in a forced appearance too; Previous at page 1 is
/// dimmed.
#[test]
fn the_current_page_is_outlined_and_the_ends_dimmed() {
    let mut app = start::<Pages>(true);
    app.run_for(Duration::from_millis(300)).unwrap();
    let current = "role:navigation[Pagination] >> role:button[1]";
    let other = "role:navigation[Pagination] >> role:button[2]";
    assert_eq!(fill(&app, current), light(BUTTON_OUTLINE));
    assert_eq!(fill(&app, other), None);
    let previous = "role:navigation[Pagination] >> role:button[Go to previous page]";
    assert_eq!(opacity(&app, previous), 0.5);
    assert_eq!(
        opacity(
            &app,
            "role:navigation[Pagination] >> role:button[Go to next page]"
        ),
        1.0
    );
    let dark = fill(&app, "role:navigation[Dark pages] >> role:button[2]");
    assert_eq!(dark, Some(common::dark(BUTTON_OUTLINE).0));
}

#[test]
fn items_are_the_button_height() {
    let desktop = start::<Pages>(true);
    let page = "role:navigation[Pagination] >> role:button[2]";
    assert_eq!(bounds(&desktop, page).size.height.0, 32.0);
    assert_eq!(bounds(&desktop, page).size.width.0, 32.0);
    let touch = start::<Pages>(false);
    assert_eq!(bounds(&touch, page).size.height.0, 40.0);
}
