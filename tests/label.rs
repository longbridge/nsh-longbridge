//! `Label` in a headless engine: a press focuses the field `htmlFor` names
//! (from a ref made before the field), not while disabled; it is no button
//! and no Tab stop; a scope's tokens reach its text.

use nsh_core::{Color, Size as Viewport, platform::Os, prelude::*};
use nsh_engine::EngineConfig;
use nsh_longbridge::{FieldRef, Input, ScopeTokens, Textarea, input, label, textarea, theme_scope};
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

/// Labels before their fields: an input's, a textarea's, a disabled
/// label's and the label of a disabled field, each field counting its
/// focus.
#[component]
fn Form() -> impl IntoElement {
    let email = component_ref::<Input>();
    let notes = component_ref::<Textarea>();
    let locked = component_ref::<Input>();
    let off = component_ref::<Input>();
    let focus = signal(Vec::<&'static str>::new());
    let heard = move |name: &'static str| move || focus.update(|f| f.push(name));
    view()
        .child(label().html_for(email).child("Email address"))
        .child(
            input()
                .component_ref(&email)
                .on_focus(heard("email"))
                .accessibility_label("Email"),
        )
        .child(label().html_for(notes).child("Your notes"))
        .child(
            textarea()
                .component_ref(&notes)
                .on_focus(heard("notes"))
                .accessibility_label("Notes"),
        )
        .child(
            label()
                .html_for(locked)
                .disabled(true)
                .child("Locked label"),
        )
        .child(
            input()
                .component_ref(&locked)
                .on_focus(heard("locked"))
                .accessibility_label("Locked"),
        )
        .child(label().html_for(off).child("Off field"))
        .child(
            input()
                .component_ref(&off)
                .disabled(true)
                .on_focus(heard("off"))
                .accessibility_label("Off"),
        )
        .child(text(move || format!("focus:{}", focus.get().join(","))))
}

#[test]
fn a_press_focuses_the_field_it_names() {
    let mut app = start::<Form>(true);
    app.press("text:Email address").unwrap();
    app.wait_for("text:focus:email").unwrap();
    app.press("text:Your notes").unwrap();
    app.wait_for("text:/focus:email,notes/").unwrap();
}

#[test]
fn a_press_on_touch_screens_focuses_too() {
    let mut app = start::<Form>(false);
    app.press("text:Email address").unwrap();
    app.wait_for("text:focus:email").unwrap();
}

#[test]
fn a_disabled_label_or_field_takes_no_focus() {
    let mut app = start::<Form>(true);
    let _ = app.press("text:Locked label");
    let _ = app.press("text:Off field");
    app.press("text:Email address").unwrap();
    app.wait_for("text:focus:email").unwrap();
}

#[test]
fn a_label_is_text_not_a_button() {
    let app = start::<Form>(true);
    let screen = app.screen();
    assert!(!screen.matches("role:button[Email address]").unwrap());
    assert!(screen.shows("Email address"));
}

/// A field named by its host node, as a TSX ref's `node` gives it.
#[component]
fn ByNode() -> impl IntoElement {
    let field = component_ref::<Input>();
    let target = signal(FieldRef::NONE);
    let focuses = signal(0u32);
    view()
        .child(label().html_for(move || target.get()).child("Name"))
        .child(
            input()
                .component_ref(&field)
                .on_focus(move || focuses.update(|n| *n += 1))
                .accessibility_label("Full name"),
        )
        .child(
            view()
                .on_press(move || {
                    if let Some(node) = field.node() {
                        target.set(FieldRef::node(node));
                    }
                })
                .child(text("Link")),
        )
        .child(text(move || format!("focus:{}", focuses.get())))
}

#[test]
fn a_node_target_set_later_is_followed() {
    let mut app = start::<ByNode>(true);
    app.press("text:Name").unwrap();
    app.press("text:Link").unwrap();
    app.press("text:Name").unwrap();
    app.wait_for("text:focus:1").unwrap();
}

/// A label's text outside and inside a scope that changes
/// `label-foreground`, and its default type.
#[component]
fn Scoped() -> impl IntoElement {
    view().child(label().child("Outside")).child(
        theme_scope()
            .tokens(ScopeTokens::new().set("label-foreground", "#2563eb"))
            .child(label().child("Inside")),
    )
}

#[test]
fn a_scope_label_foreground_reaches_the_text() {
    let app = start::<Scoped>(true);
    let screen = app.screen();
    let inside = screen.span_style("text:Inside").unwrap();
    let outside = screen.span_style("text:Outside").unwrap();
    let blue = Color::parse("#2563eb").unwrap();
    assert_eq!(inside.color, blue);
    assert_ne!(outside.color, blue);
    assert_eq!(outside.font_size.0, 14.0);
    assert_eq!(outside.line_height.0, 14.0);
    assert_eq!(outside.font_weight, 500);
}
