//! `Toggle` in a headless engine: a press reports the next value and the
//! toggle shows the one passed back, disabled, what screen readers get,
//! pointer and touch sizes, the pressed and hover fills from the tokens,
//! the outline edge and shadow, and a scoped token reaching the look.

mod common;

use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
    time::Duration,
};

use common::{fill, light, size, start};
use nsh_core::{Checked, MountItem, NodeId, ViewStyle, prelude::*};
use nsh_longbridge::{
    ScopeTokens, Size, ToggleVariant,
    components::toggle::{TOGGLE_HOVER, TOGGLE_PRESSED},
    theme_scope, toggle,
};
use nsh_testing::Headless;

/// A controlled toggle, an outline one, a disabled pressed one and one
/// with a word, each size.
#[component]
fn Toggles() -> impl IntoElement {
    let bold = signal(false);
    let log = signal(Vec::<String>::new());
    view()
        .flex_col()
        .items_start()
        .gap(px(8.0))
        .p(px(20.0))
        .child(
            toggle()
                .pressed(bold)
                .on_change(move |p: &bool| {
                    log.update(|l| l.push(format!("change:{p}")));
                    bold.set(*p);
                })
                .accessibility_label("Bold")
                .child(text("B")),
        )
        .child(
            toggle()
                .variant(ToggleVariant::Outline)
                .accessibility_label("Italic")
                .child(text("I")),
        )
        .child(
            toggle()
                .pressed(true)
                .disabled(true)
                .on_change(move |p: &bool| log.update(|l| l.push(format!("locked:{p}"))))
                .accessibility_label("Locked")
                .child(text("L")),
        )
        .child(toggle().child(text("Bookmark")))
        .child(
            toggle()
                .size(Size::Xs)
                .accessibility_label("Xs")
                .child(text("x")),
        )
        .child(
            toggle()
                .size(Size::Lg)
                .accessibility_label("Lg")
                .child(text("x")),
        )
        .child(text(move || format!("log:{}", log.get().join(","))))
}

fn watch_styles(app: &mut Headless) -> Arc<Mutex<HashMap<NodeId, ViewStyle>>> {
    let styles = Arc::new(Mutex::new(HashMap::new()));
    let seen = styles.clone();
    app.observe(move |_, batch| {
        for item in &batch.items {
            if let MountItem::ViewStyle { id, style } = item {
                seen.lock().unwrap().insert(*id, (**style).clone());
            }
        }
    });
    styles
}

#[test]
fn a_press_reports_the_next_value_and_shows_the_one_passed_back() {
    let mut app = start::<Toggles>(true);
    app.press("role:button[Bold]").unwrap();
    app.wait_for("text:log:change:true").unwrap();
    assert_eq!(
        app.screen()
            .accessibility_state("role:button[Bold]")
            .unwrap()
            .pressed,
        Some(Checked::True)
    );
    app.press("role:button[Bold]").unwrap();
    app.wait_for("text:log:change:true,change:false").unwrap();
}

#[test]
fn space_presses_the_focused_toggle() {
    let mut app = start::<Toggles>(true);
    app.press_key("Tab").unwrap();
    app.press_key("Space").unwrap();
    app.wait_for("text:log:change:true").unwrap();
}

#[test]
fn a_disabled_toggle_reports_nothing_and_keeps_its_on_look() {
    let mut app = start::<Toggles>(true);
    assert!(!app.screen().enabled("role:button[Locked]").unwrap());
    let _ = app.press("role:button[Locked]");
    app.run_for(Duration::from_millis(50)).unwrap();
    assert!(app.shows("log:"));
    assert!(!app.shows("log:locked:false"));
    assert_eq!(fill(&app, "role:button[Locked]"), light(TOGGLE_PRESSED));
}

#[test]
fn screen_readers_get_a_toggle_button() {
    let app = start::<Toggles>(true);
    let screen = app.screen();
    let state = screen.accessibility_state("role:button[Italic]").unwrap();
    assert_eq!(state.pressed, Some(Checked::False));
    // A word names it without a label.
    assert!(screen.matches("role:button[Bookmark]").unwrap());
}

#[test]
fn on_it_takes_the_pressed_fill_and_off_none() {
    let mut app = start::<Toggles>(true);
    assert_eq!(fill(&app, "role:button[Bold]"), None);
    app.press("role:button[Bold]").unwrap();
    app.run_for(Duration::from_millis(300)).unwrap();
    assert_eq!(fill(&app, "role:button[Bold]"), light(TOGGLE_PRESSED));
}

#[test]
fn the_pointer_over_an_off_toggle_shows_the_hover_fill() {
    let mut app = start::<Toggles>(true);
    app.hover("role:button[Bold]").unwrap();
    app.run_for(Duration::from_millis(300)).unwrap();
    assert_eq!(fill(&app, "role:button[Bold]"), light(TOGGLE_HOVER));
}

#[test]
fn sizes_with_a_pointer_and_on_touch_screens() {
    let app = start::<Toggles>(true);
    // A square at least as wide as tall.
    assert_eq!(size(&app, "role:button[Bold]"), (32.0, 32.0));
    assert_eq!(size(&app, "role:button[Xs]").1, 24.0);
    assert_eq!(size(&app, "role:button[Lg]").1, 36.0);
    assert!(size(&app, "role:button[Bookmark]").0 > 32.0);
    let touch = start::<Toggles>(false);
    assert_eq!(size(&touch, "role:button[Bold]"), (40.0, 40.0));
    assert_eq!(size(&touch, "role:button[Xs]").1, 32.0);
    assert_eq!(size(&touch, "role:button[Lg]").1, 48.0);
}

#[test]
fn the_outline_variant_has_an_edge_and_a_shadow() {
    let mut app = start::<Toggles>(true);
    let styles = watch_styles(&mut app);
    // Make its style go out again.
    app.hover("role:button[Italic]").unwrap();
    app.run_for(Duration::from_millis(300)).unwrap();
    let id = app.screen().id("role:button[Italic]").unwrap();
    let style = styles.lock().unwrap().get(&id).cloned().unwrap();
    assert_eq!(style.border_radius.0, 8.0);
    assert_eq!(style.borders.widths.top.0, 1.0);
    let shadows = style
        .effects
        .as_ref()
        .map(|e| e.box_shadows.len())
        .unwrap_or(0);
    assert_eq!(shadows, 1, "{style:?}");
}

/// A pressed toggle outside and inside a scope that changes
/// `toggle-pressed`.
#[component]
fn Scoped() -> impl IntoElement {
    view()
        .child(
            toggle()
                .pressed(true)
                .accessibility_label("Outside")
                .child(text("O")),
        )
        .child(
            theme_scope()
                .tokens(ScopeTokens::new().set("toggle-pressed", "#2563eb"))
                .child(
                    toggle()
                        .pressed(true)
                        .accessibility_label("Inside")
                        .child(text("I")),
                ),
        )
}

#[test]
fn a_scope_toggle_pressed_reaches_the_fill() {
    let app = start::<Scoped>(true);
    assert_eq!(fill(&app, "role:button[Inside]"), common::css("#2563eb"));
    assert_eq!(fill(&app, "role:button[Outside]"), light(TOGGLE_PRESSED));
}
