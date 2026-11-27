//! `InputGroup` in a headless engine: a press on an addon focuses the
//! field, buttons and the password toggle (its pressed state, the masked
//! field it shows, its translatable name, the selection kept), the frame
//! drawing the field's focus and invalid state, a disabled field disabling
//! the group, the sizes on pointer and touch, a block addon making a
//! column, and a scope's token reaching an addon.

mod common;

use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
    time::Duration,
};

use common::{bounds, light, size, start};
use nsh_core::{
    Checked, Command, EventKind, EventPayload, MountItem, NodeId, PayloadValue, ViewStyle,
    prelude::*,
};
use nsh_longbridge::{
    IconName, InputGroupAlign, ScopeTokens, Size,
    foundation::{
        field::{INPUT_BORDER, INPUT_BORDER_FOCUS, INPUT_BORDER_INVALID},
        labels::{Label, set_translator},
    },
    input, input_group, input_group_addon, input_group_button, input_group_password_toggle,
    textarea, theme_scope,
};
use nsh_testing::Headless;

/// A URL with a text addon, a key with a Copy button, a password with its
/// toggle, a disabled field with a button, an invalid field.
#[component]
fn Groups() -> impl IntoElement {
    let focuses = signal(0u32);
    let copies = signal(0u32);
    let clears = signal(0u32);
    let asked = signal(String::from("-"));
    view()
        .gap_4()
        .child(
            input_group()
                .test_id("site")
                .child(
                    input_group_addon()
                        .test_id("scheme")
                        .child(text("https://")),
                )
                .child(
                    input()
                        .on_focus(move || focuses.update(|n| *n += 1))
                        .accessibility_label("Website"),
                ),
        )
        .child(
            input_group()
                .child(
                    input()
                        .readonly(true)
                        .default_value("sk-123")
                        .accessibility_label("Key"),
                )
                .child(
                    input_group_addon().align(InputGroupAlign::InlineEnd).child(
                        input_group_button()
                            .icon(IconName::Copy)
                            .on_press(move || copies.update(|n| *n += 1))
                            .child(text("Copy")),
                    ),
                ),
        )
        .child(
            input_group()
                .test_id("password")
                .child(
                    input()
                        .masked(true)
                        .default_value("hunter22")
                        .accessibility_label("Password"),
                )
                .child(
                    input_group_addon().align(InputGroupAlign::InlineEnd).child(
                        input_group_password_toggle()
                            .on_change(move |shown: &bool| asked.set(shown.to_string())),
                    ),
                ),
        )
        .child(
            input_group()
                .test_id("locked")
                .child(input().disabled(true).accessibility_label("Locked"))
                .child(
                    input_group_addon().align(InputGroupAlign::InlineEnd).child(
                        input_group_button()
                            .test_id("locked-clear")
                            .icon_only(true)
                            .icon(IconName::X)
                            .on_press(move || clears.update(|n| *n += 1))
                            .accessibility_label("Clear locked"),
                    ),
                ),
        )
        .child(
            input_group()
                .test_id("bad")
                .child(input().invalid(true).accessibility_label("Bad")),
        )
        .child(text(move || {
            format!(
                "focus:{} copy:{} clear:{} asked:{}",
                focuses.get(),
                copies.get(),
                clears.get(),
                asked.get()
            )
        }))
}

fn watch(app: &mut Headless) -> Arc<Mutex<HashMap<NodeId, MountItem>>> {
    let seen = Arc::new(Mutex::new(HashMap::new()));
    let into = seen.clone();
    app.observe(move |_, batch| {
        for item in &batch.items {
            match item {
                MountItem::ViewStyle { id, .. } | MountItem::Input { id, .. } => {
                    into.lock().unwrap().insert(*id, item.clone());
                }
                _ => {}
            }
        }
    });
    seen
}

/// `C` on a Mac, with every view style and input configuration kept.
fn start_watched<C: Component<Props = ()>>() -> (Headless, Arc<Mutex<HashMap<NodeId, MountItem>>>) {
    let mut config = nsh_engine::EngineConfig {
        viewport: nsh_core::Size::new(px(800.0), px(900.0)),
        clock: nsh_engine::Clock::manual(std::time::SystemTime::now()),
        ..nsh_engine::EngineConfig::new(nsh_testing::windowed_root::<C>())
    };
    config.environment.os = nsh_core::platform::Os::MacOs;
    let mut app = Headless::start(config).unwrap();
    let seen = watch(&mut app);
    app.first_render().unwrap();
    (app, seen)
}

fn style_of(seen: &Arc<Mutex<HashMap<NodeId, MountItem>>>, id: NodeId) -> ViewStyle {
    match seen.lock().unwrap().get(&id) {
        Some(MountItem::ViewStyle { style, .. }) => (**style).clone(),
        other => panic!("no style for {id:?}: {other:?}"),
    }
}

fn secure(app: &Headless, seen: &Arc<Mutex<HashMap<NodeId, MountItem>>>, selector: &str) -> bool {
    let id = app.screen().id(selector).unwrap();
    let mut found = None;
    // The configuration of the input (its own node or the one it hosts).
    for (node, item) in seen.lock().unwrap().iter() {
        if let MountItem::Input { config, .. } = item
            && (*node == id
                || app
                    .screen()
                    .id(&format!("view:{node}"))
                    .is_ok_and(|n| n == id))
        {
            found = Some(config.secure);
        }
    }
    found.unwrap_or_else(|| panic!("no input config for {selector}"))
}

#[test]
fn a_press_on_an_addon_focuses_the_field() {
    let mut app = start::<Groups>(true);
    app.press("text:https://").unwrap();
    app.wait_for("text:/focus:1 /").unwrap();
    // The addon is no stop and no element of its own.
    assert!(!app.screen().matches("role:button[https://]").unwrap());
}

#[test]
fn a_button_reports_its_press() {
    let mut app = start::<Groups>(true);
    app.press("role:button[Copy]").unwrap();
    app.wait_for("text:/copy:1 /").unwrap();
}

#[test]
fn the_password_toggle_shows_the_field_and_says_pressed() {
    let (mut app, seen) = start_watched::<Groups>();
    let toggle = "role:button[Show password]";
    let state = app.screen().accessibility_state(toggle).unwrap();
    assert_eq!(state.pressed, Some(Checked::False));
    assert!(secure(&app, &seen, "label:Password"));

    app.press(toggle).unwrap();
    app.wait_for("text:/asked:true/").unwrap();
    app.run_for(Duration::from_millis(50)).unwrap();
    let state = app.screen().accessibility_state(toggle).unwrap();
    assert_eq!(state.pressed, Some(Checked::True));
    assert!(!secure(&app, &seen, "label:Password"));
    // Its name stays: the state says what changed.
    assert!(app.screen().matches(toggle).unwrap());

    app.press(toggle).unwrap();
    app.wait_for("text:/asked:false/").unwrap();
    app.run_for(Duration::from_millis(50)).unwrap();
    assert!(secure(&app, &seen, "label:Password"));
}

#[test]
fn showing_the_password_keeps_the_selection() {
    let mut app = start::<Groups>(true);
    app.type_text("label:Password", "!").unwrap();
    app.run_for(Duration::from_millis(50)).unwrap();
    let field = app.screen().id("label:Password").unwrap();
    app.engine().control().dispatch_event(
        field,
        EventKind::SelectionChange,
        EventPayload::Map(vec![
            ("start", PayloadValue::Number(2.0)),
            ("end", PayloadValue::Number(5.0)),
        ]),
    );
    app.run_for(Duration::from_millis(50)).unwrap();
    app.clear_commands();
    app.press("role:button[Show password]").unwrap();
    app.wait_for("text:/asked:true/").unwrap();
    app.run_for(Duration::from_millis(100)).unwrap();
    assert!(
        app.screen()
            .sent(&Command::SetSelection { start: 2, end: 5 }),
        "{:?}",
        app.screen().commands()
    );
}

/// The password toggle under a translator.
#[component]
fn Translated() -> impl IntoElement {
    set_translator(|label| (label == Label::ShowPassword).then(|| "Afficher".into()));
    input_group()
        .child(input().masked(true).accessibility_label("Mot de passe"))
        .child(
            input_group_addon()
                .align(InputGroupAlign::InlineEnd)
                .child(input_group_password_toggle()),
        )
}

#[test]
fn the_toggles_name_is_translatable() {
    let app = start::<Translated>(true);
    assert!(app.screen().matches("role:button[Afficher]").unwrap());
}

#[test]
fn the_frame_shows_the_fields_focus_and_not_a_buttons() {
    let (mut app, seen) = start_watched::<Groups>();
    let frame = app.screen().id("id:site").unwrap();
    let edge = |s: &ViewStyle| Some(s.borders.colors[0].0);
    assert_eq!(edge(&style_of(&seen, frame)), light(INPUT_BORDER));
    // A press on the addon focuses the field.
    app.press("text:https://").unwrap();
    app.run_for(Duration::from_millis(300)).unwrap();
    assert_eq!(edge(&style_of(&seen, frame)), light(INPUT_BORDER_FOCUS));
    // Tab moves on to the next field's group: this frame rests again.
    app.press_key("Tab").unwrap();
    app.run_for(Duration::from_millis(300)).unwrap();
    assert_eq!(edge(&style_of(&seen, frame)), light(INPUT_BORDER));
}

#[test]
fn the_frame_shows_the_fields_invalid_state() {
    let (app, seen) = start_watched::<Groups>();
    let frame = app.screen().id("id:bad").unwrap();
    let style = style_of(&seen, frame);
    assert_eq!(Some(style.borders.colors[0].0), light(INPUT_BORDER_INVALID));
    let _ = app;
}

#[test]
fn a_disabled_field_disables_the_group() {
    let mut app = start::<Groups>(true);
    let screen = app.screen();
    assert_eq!(
        screen.view_style("id:locked").unwrap().unwrap().opacity,
        0.5
    );
    // Dimmed once, by the group.
    assert_eq!(
        screen
            .view_style("id:locked-clear")
            .unwrap()
            .unwrap()
            .opacity,
        1.0
    );
    assert!(
        screen
            .accessibility_state("role:button[Clear locked]")
            .unwrap()
            .disabled
    );
    let _ = app.press("role:button[Clear locked]");
    app.run_for(Duration::from_millis(100)).unwrap();
    app.wait_for("text:/clear:0 /").unwrap();
    // Nor does a press on it focus the field.
    let _ = app.press("text:https://");
}

/// A group of each size, each with a button.
#[component]
fn Sizes() -> impl IntoElement {
    let one = |size: Size| {
        input_group()
            .size(size)
            .test_id(size.as_str())
            .child(input().accessibility_label(size.as_str()))
            .child(
                input_group_addon().align(InputGroupAlign::InlineEnd).child(
                    input_group_button()
                        .test_id(&format!("button-{}", size.as_str()))
                        .icon_only(true)
                        .icon(IconName::X)
                        .accessibility_label("Clear"),
                ),
            )
    };
    view()
        .gap_4()
        .child(one(Size::Xs))
        .child(one(Size::Sm))
        .child(one(Size::Md))
        .child(one(Size::Lg))
}

#[test]
fn sizes_follow_pointer_and_touch() {
    let desktop = start::<Sizes>(true);
    let touch = start::<Sizes>(false);
    for (s, pointer_h, touch_h) in [
        (Size::Xs, 24.0, 32.0),
        (Size::Sm, 28.0, 36.0),
        (Size::Md, 32.0, 40.0),
        (Size::Lg, 36.0, 48.0),
    ] {
        let id = format!("id:{}", s.as_str());
        let button = format!("id:button-{}", s.as_str());
        assert_eq!(size(&desktop, &id).1, pointer_h, "{s:?}");
        assert_eq!(size(&touch, &id).1, touch_h, "{s:?}");
        // A square, the field's height less 4 above and below.
        assert_eq!(
            size(&desktop, &button),
            (pointer_h - 8.0, pointer_h - 8.0),
            "{s:?}"
        );
        assert_eq!(
            size(&touch, &button),
            (touch_h - 8.0, touch_h - 8.0),
            "{s:?}"
        );
        // The field inside: the frame's height less its edges.
        let field = format!("label:{}", s.as_str());
        assert_eq!(size(&desktop, &field).1, pointer_h - 2.0, "{s:?}");
        // The button 4 from the frame's inner edge at the end.
        let frame = bounds(&desktop, &id);
        let b = bounds(&desktop, &button);
        let inset = (frame.origin.x + frame.size.width) - (b.origin.x + b.size.width);
        assert_eq!(inset.0, 4.0 + 1.0, "{s:?}");
    }
}

/// A message box: a textarea and a toolbar under it.
#[component]
fn Message() -> impl IntoElement {
    view().child(
        input_group()
            .test_id("message")
            .child(textarea().accessibility_label("Message"))
            .child(
                input_group_addon()
                    .test_id("toolbar")
                    .align(InputGroupAlign::BlockEnd)
                    .child(text("0 / 280"))
                    .child(
                        input_group_button()
                            .variant(nsh_longbridge::ButtonVariant::Default)
                            .icon_only(true)
                            .icon(IconName::ArrowRight)
                            .accessibility_label("Send"),
                    ),
            ),
    )
}

#[test]
fn a_block_addon_makes_a_column() {
    let app = start::<Message>(true);
    let frame = bounds(&app, "id:message");
    let area = bounds(&app, "label:Message");
    let toolbar = bounds(&app, "id:toolbar");
    // The toolbar under the area, both the frame's width (less its edges).
    assert!(
        toolbar.origin.y >= area.origin.y + area.size.height,
        "{area:?} {toolbar:?}"
    );
    assert_eq!(area.size.width, frame.size.width - px(2.0));
    assert_eq!(toolbar.size.width, frame.size.width - px(2.0));
    assert_eq!(
        frame.size.height,
        area.size.height + toolbar.size.height + px(2.0)
    );
}

/// The same addon outside and inside a scope that changes its color.
#[component]
fn Scoped() -> impl IntoElement {
    let group = |words: &'static str| {
        input_group()
            .child(input_group_addon().child(text(words)))
            .child(input().accessibility_label(words))
    };
    view().child(group("outside")).child(
        theme_scope()
            .tokens(ScopeTokens::new().set("input-group-addon-foreground", "#2563eb"))
            .child(group("inside")),
    )
}

#[test]
fn a_scope_reaches_the_addons_color() {
    let app = start::<Scoped>(true);
    let color = |selector: &str| app.screen().span_style(selector).unwrap().color;
    let blue = nsh_core::Color::parse("#2563eb").unwrap();
    assert_eq!(color("text:inside"), blue);
    assert_ne!(color("text:outside"), blue);
}

/// TSX apps' declarations.
#[test]
fn tsx_gets_the_parts() {
    let registry = nsh_longbridge::registry();
    assert_eq!(registry.undocumented(), Vec::<String>::new());
    let ts = nsh_core::element::declarations(&registry, &[]);
    for wanted in [
        "export interface InputGroupProps",
        "export interface InputGroupAddonProps",
        "export interface InputGroupButtonProps",
        "export interface InputGroupPasswordToggleProps",
        "\"inlineStart\"",
        "\"blockEnd\"",
        "\"show-password\"",
        "\"eye-off\"",
    ] {
        assert!(ts.contains(wanted), "{wanted}");
    }
    let start = ts.find("export interface InputGroupAddonProps").unwrap();
    let props = &ts[start..start + ts[start..].find("\n}").unwrap()];
    assert!(props.contains("align?:"), "{props}");
}
