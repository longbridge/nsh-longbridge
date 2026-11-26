//! `Input` in a headless engine: controlled and uncontrolled values, the
//! events, the disabled, read-only and invalid states, what screen readers
//! get, pointer and touch sizes, and a scope's tokens reaching its look.

use nsh_core::{Color, Size as Viewport, platform::Os, prelude::*};
use nsh_engine::EngineConfig;
use nsh_longbridge::{
    InputApi, ScopeTokens, Size, components::input::font_size_token, input,
    theme::resolve_number_in, theme_scope,
};
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

/// A controlled input that reports its value, its submits and its focus,
/// an uncontrolled one, a disabled, a read-only and an invalid one.
#[component]
fn Fields() -> impl IntoElement {
    let email = signal(String::from("ada@"));
    let submitted = signal(String::new());
    let focuses = signal(0u32);
    let blurs = signal(0u32);
    let disabled_edits = signal(0u32);
    view()
        .child(
            input()
                .value(move || Some(email.get().into()))
                .on_change(move |value: &String| email.set(value.clone()))
                .on_submit(move |value: &String| submitted.set(value.clone()))
                .on_focus(move || focuses.update(|n| *n += 1))
                .on_blur(move || blurs.update(|n| *n += 1))
                .accessibility_label("Email"),
        )
        .child(
            input()
                .default_value("draft")
                .placeholder("Notes")
                .accessibility_label("Notes"),
        )
        .child(
            input()
                .disabled(true)
                .default_value("locked")
                .on_change(move |_: &String| disabled_edits.update(|n| *n += 1))
                .accessibility_label("Disabled"),
        )
        .child(
            input()
                .readonly(true)
                .default_value("ACC-1")
                .accessibility_label("Account"),
        )
        .child(
            input()
                .invalid(true)
                .default_value("x")
                .accessibility_label("Invalid"),
        )
        .child(text(move || {
            format!(
                "email:{} submitted:{} focus:{} blur:{} disabled:{}",
                email.get(),
                submitted.get(),
                focuses.get(),
                blurs.get(),
                disabled_edits.get()
            )
        }))
}

#[test]
fn a_controlled_input_reports_edits_and_submits() {
    let mut app = start::<Fields>(true);
    app.type_text("label:Email", "lovelace.dev").unwrap();
    app.wait_for("text:/email:ada@lovelace.dev submitted: focus:1/")
        .unwrap();
    app.submit("label:Email").unwrap();
    app.wait_for("text:/submitted:ada@lovelace.dev focus:1 blur:1/")
        .unwrap();
}

#[test]
fn an_uncontrolled_input_keeps_its_own_text() {
    let mut app = start::<Fields>(true);
    assert_eq!(
        app.screen().input_text("label:Notes").unwrap(),
        Some("draft")
    );
    app.type_text("label:Notes", "!").unwrap();
    assert_eq!(
        app.screen().input_text("label:Notes").unwrap(),
        Some("draft!")
    );
}

#[test]
fn a_disabled_input_takes_no_edits() {
    let mut app = start::<Fields>(true);
    let _ = app.type_text("label:Disabled", "x");
    assert_eq!(
        app.screen().input_text("label:Disabled").unwrap(),
        Some("locked")
    );
    app.wait_for("text:/disabled:0/").unwrap();
}

#[test]
fn screen_readers_get_the_states() {
    let app = start::<Fields>(true);
    let screen = app.screen();
    assert!(
        screen
            .accessibility_state("label:Disabled")
            .unwrap()
            .disabled
    );
    let invalid = screen.accessibility_state("label:Invalid").unwrap();
    assert!(invalid.invalid && !invalid.disabled);
    let account = screen.accessibility_state("label:Account").unwrap();
    assert!(!account.disabled && !account.invalid);
    assert!(!screen.accessibility_state("label:Email").unwrap().invalid);
}

/// The four sizes.
#[component]
fn Sizes() -> impl IntoElement {
    view()
        .child(input().size(Size::Xs).accessibility_label("xs"))
        .child(input().size(Size::Sm).accessibility_label("sm"))
        .child(input().accessibility_label("md"))
        .child(input().size(Size::Lg).accessibility_label("lg"))
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
fn sizes_follow_pointer_and_touch() {
    let desktop = start::<Sizes>(true);
    let touch = start::<Sizes>(false);
    let app = nsh_base::Theme::default().into_tokens();
    for (size, pointer_h, touch_h, pointer_text, touch_text) in [
        (Size::Xs, 24.0, 32.0, 12.0, 14.0),
        (Size::Sm, 28.0, 36.0, 13.0, 14.0),
        (Size::Md, 32.0, 40.0, 14.0, 16.0),
        (Size::Lg, 36.0, 48.0, 14.0, 16.0),
    ] {
        let selector = format!("label:{}", size.as_str());
        assert_eq!(height(&desktop, &selector), pointer_h, "{size:?}");
        assert_eq!(height(&touch, &selector), touch_h, "{size:?}");
        // The headless screen reads no input's text style: its text size
        // token, as the field reads it.
        let token = font_size_token(size);
        assert_eq!(resolve_number_in(&app, &[], true, token), pointer_text);
        assert_eq!(resolve_number_in(&app, &[], false, token), touch_text);
    }
}

/// An input whose ref focuses it.
#[component]
fn WithRef() -> impl IntoElement {
    let field = component_ref::<nsh_longbridge::Input>();
    let focuses = signal(0u32);
    view()
        .child(
            input()
                .component_ref(&field)
                .on_focus(move || focuses.update(|n| *n += 1))
                .accessibility_label("Name"),
        )
        .child(
            view()
                .on_press(move || {
                    if let Some(api) = field.api::<InputApi>() {
                        api.focus();
                    }
                })
                .child(text("Focus it")),
        )
        .child(text(move || format!("focus:{}", focuses.get())))
}

#[test]
fn the_ref_focuses_the_field() {
    let mut app = start::<WithRef>(true);
    app.press("text:Focus it").unwrap();
    app.wait_for("text:focus:1").unwrap();
}

/// The same input outside and inside a scope that changes
/// `input-background`.
#[component]
fn Scoped() -> impl IntoElement {
    view().child(input().accessibility_label("Outside")).child(
        theme_scope()
            .tokens(ScopeTokens::new().set("input-background", "#2563eb"))
            .child(input().accessibility_label("Inside")),
    )
}

fn fill(app: &Headless, selector: &str) -> Option<u32> {
    app.screen()
        .view_style(selector)
        .unwrap()
        .unwrap_or_else(|| panic!("no {selector}"))
        .background_color
}

#[test]
fn a_scope_input_background_reaches_the_field() {
    let app = start::<Scoped>(true);
    let blue = Some(Color::parse("#2563eb").unwrap().0);
    assert_eq!(fill(&app, "label:Inside"), blue);
    assert_ne!(fill(&app, "label:Outside"), blue);
}
