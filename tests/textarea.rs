//! `Textarea` in a headless engine: it grows with its text in the same
//! layout as the edit, from its least height, up to an app's cap; its
//! sizes change the text, not the height; Return breaks the line or
//! submits; the states screen readers get.

use nsh_core::{Size as Viewport, platform::Os, prelude::*};
use nsh_engine::EngineConfig;
use nsh_longbridge::{Size, textarea};
use nsh_testing::Headless;

fn start<C: Component<Props = ()>>(desktop: bool) -> Headless {
    let mut config = EngineConfig {
        viewport: Viewport::new(px(800.0), px(600.0)),
        ..EngineConfig::new(nsh_testing::windowed_root::<C>())
    };
    config.environment.os = if desktop { Os::MacOs } else { Os::Ios };
    let mut app = Headless::start(config).unwrap();
    app.first_render().unwrap();
    app
}

/// A controlled textarea, one capped at 120 points, one that submits on
/// Return, a disabled and an invalid one.
#[component]
fn Areas() -> impl IntoElement {
    let text_value = signal(String::new());
    let sent = signal(String::new());
    view()
        .items_start()
        .child(
            textarea()
                .value(move || Some(text_value.get().into()))
                .on_change(move |value: &String| text_value.set(value.clone()))
                .placeholder("Type your message here.")
                .accessibility_label("Message"),
        )
        .child(textarea().max_h(px(120.0)).accessibility_label("Capped"))
        .child(
            textarea()
                .submit_behavior(SubmitBehavior::Submit)
                .on_submit(move |value: &String| sent.set(value.clone()))
                .accessibility_label("Chat"),
        )
        .child(textarea().disabled(true).accessibility_label("Disabled"))
        .child(textarea().invalid(true).accessibility_label("Invalid"))
        .child(text(move || format!("sent:{}", sent.get())))
}

fn height(app: &Headless, selector: &str) -> f32 {
    app.screen()
        .layout(selector)
        .unwrap()
        .unwrap_or_else(|| panic!("no {selector}"))
        .bounds
        .size
        .height
        .0
}

#[test]
fn it_starts_at_its_least_height() {
    let app = start::<Areas>(true);
    assert_eq!(height(&app, "label:Message"), 64.0);
}

#[test]
fn it_grows_with_each_line_in_the_same_layout() {
    let mut app = start::<Areas>(true);
    // Five lines of 20 points, 8 above and below, a 1-point edge.
    let tall = 5.0 * 20.0 + 2.0 * 8.0 + 2.0;
    app.type_text("label:Message", "one").unwrap();
    for _ in 0..4 {
        app.submit("label:Message").unwrap();
    }
    assert_eq!(
        app.screen().input_text("label:Message").unwrap(),
        Some("one\n\n\n\n")
    );
    // No frame between the edit and the height: the host measures the
    // text in the layout the edit makes.
    assert_eq!(height(&app, "label:Message"), tall);
    app.type_text("label:Message", "x").unwrap();
    assert_eq!(height(&app, "label:Message"), tall);
}

#[test]
fn an_apps_cap_stops_it_growing() {
    let mut app = start::<Areas>(true);
    app.type_text("label:Capped", "a").unwrap();
    for _ in 0..10 {
        app.submit("label:Capped").unwrap();
    }
    assert_eq!(height(&app, "label:Capped"), 120.0);
}

#[test]
fn return_submits_when_asked() {
    let mut app = start::<Areas>(true);
    app.type_text("label:Chat", "hello").unwrap();
    app.submit("label:Chat").unwrap();
    app.wait_for("text:sent:hello").unwrap();
    assert_eq!(
        app.screen().input_text("label:Chat").unwrap(),
        Some("hello")
    );
}

#[test]
fn screen_readers_get_the_states() {
    let app = start::<Areas>(true);
    let screen = app.screen();
    assert!(
        screen
            .accessibility_state("label:Disabled")
            .unwrap()
            .disabled
    );
    assert!(screen.accessibility_state("label:Invalid").unwrap().invalid);
}

/// The four sizes, empty.
#[component]
fn Sizes() -> impl IntoElement {
    view()
        .child(textarea().size(Size::Xs).accessibility_label("xs"))
        .child(textarea().size(Size::Sm).accessibility_label("sm"))
        .child(textarea().accessibility_label("md"))
        .child(textarea().size(Size::Lg).accessibility_label("lg"))
}

/// Three lines of `size`'s text: the line height follows the text size.
fn three_lines(desktop: bool, size: &str) -> f32 {
    let mut app = start::<Sizes>(desktop);
    let selector = format!("label:{size}");
    assert_eq!(height(&app, &selector), 64.0, "{size}");
    app.type_text(&selector, "a").unwrap();
    app.submit(&selector).unwrap();
    app.submit(&selector).unwrap();
    height(&app, &selector)
}

#[test]
fn sizes_change_the_text_not_the_least_height() {
    // 8 above and below, a 1-point edge; lines of 16 / 18 / 20 with a
    // pointer, 20 / 20 / 24 on touch screens (12 / 13 / 14 and 14 / 16
    // points of text).
    let frame: f32 = 18.0;
    for (size, pointer_line, touch_line) in [
        ("xs", 16.0_f32, 20.0),
        ("sm", 18.0, 20.0),
        ("md", 20.0, 24.0),
        ("lg", 20.0, 24.0),
    ] {
        let pointer = (3.0 * pointer_line + frame).max(64.0);
        assert_eq!(three_lines(true, size), pointer, "{size}");
        assert_eq!(three_lines(false, size), 3.0 * touch_line + frame, "{size}");
    }
}
