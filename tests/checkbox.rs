//! `Checkbox` in a headless engine: presses on the box and its label, the
//! keyboard, the disabled state, what screen readers get, pointer and
//! touch sizes, the keyboard focus reaching the box, and a scope's tokens
//! reaching its look.

use nsh_core::{Checked, Color, NodeId, Size as Viewport, platform::Os, prelude::*};
use nsh_engine::EngineConfig;
use nsh_longbridge::{ScopeTokens, Size, checkbox, theme_scope};
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

/// The box of the checkbox `selector` names: the view in `nsh/base`'s
/// indicator, its first child.
fn indicator(app: &Headless, selector: &str) -> NodeId {
    let screen = app.screen();
    let root = screen.id(selector).unwrap();
    let nodes = screen.layout_nodes();
    let child = |of: NodeId| {
        nodes
            .iter()
            .find(|n| n.parent == of)
            .unwrap_or_else(|| panic!("{selector}: no child of {of:?}"))
            .id
    };
    child(child(root))
}

fn side(app: &Headless, id: NodeId) -> (f32, f32) {
    let layout = app
        .screen()
        .layout(&format!("view:{}", id))
        .unwrap()
        .unwrap();
    (layout.bounds.size.width.0, layout.bounds.size.height.0)
}

fn fill(app: &Headless, id: NodeId) -> Option<u32> {
    app.screen()
        .view_style(&format!("view:{}", id))
        .unwrap()
        .unwrap()
        .background_color
}

/// A controlled checkbox, a "select all" one that is indeterminate, a
/// disabled one, an invalid one and a box alone.
#[component]
fn Checkboxes() -> impl IntoElement {
    let terms = signal(false);
    let all = signal(0u32);
    let disabled = signal(0u32);
    view()
        .items_start()
        .child(
            checkbox()
                .checked(move || terms.get())
                .on_change(move |checked: &bool| terms.set(*checked))
                .child("Accept terms"),
        )
        .child(
            checkbox()
                .indeterminate(true)
                .on_change(move |checked: &bool| {
                    if *checked {
                        all.update(|n| *n += 1)
                    }
                })
                .child("All"),
        )
        .child(
            checkbox()
                .disabled(true)
                .checked(true)
                .on_change(move |_: &bool| disabled.update(|n| *n += 1))
                .child("Locked"),
        )
        .child(checkbox().invalid(true).child("Required"))
        .child(checkbox().accessibility_label("Select row"))
        .child(text(move || {
            format!(
                "terms:{} all:{} disabled:{}",
                terms.get(),
                all.get(),
                disabled.get()
            )
        }))
}

#[test]
fn a_press_on_the_label_reports_the_next_value() {
    let mut app = start::<Checkboxes>(true);
    app.press("text:Accept terms").unwrap();
    app.wait_for("text:terms:true all:0 disabled:0").unwrap();
    // The app passed it back: the next press asks for unchecked.
    app.press("text:Accept terms").unwrap();
    app.wait_for("text:terms:false all:0 disabled:0").unwrap();
}

#[test]
fn an_indeterminate_press_asks_for_checked() {
    let mut app = start::<Checkboxes>(true);
    app.press("text:All").unwrap();
    app.wait_for("text:terms:false all:1 disabled:0").unwrap();
}

#[test]
fn space_toggles_the_focused_checkbox() {
    let mut app = start::<Checkboxes>(true);
    app.press_key("Tab").unwrap();
    app.press_key("Space").unwrap();
    app.wait_for("text:terms:true all:0 disabled:0").unwrap();
}

#[test]
fn a_disabled_checkbox_does_not_change() {
    let mut app = start::<Checkboxes>(true);
    assert!(!app.screen().enabled("role:checkbox[Locked]").unwrap());
    let _ = app.press("text:Locked");
    app.press("text:Accept terms").unwrap();
    app.wait_for("text:terms:true all:0 disabled:0").unwrap();
    // Box and label dim together: the root takes `opacity-disabled`.
    let style = app
        .screen()
        .view_style("role:checkbox[Locked]")
        .unwrap()
        .unwrap();
    assert_eq!(style.opacity, 0.5);
}

#[test]
fn screen_readers_get_the_role_and_states() {
    let mut app = start::<Checkboxes>(true);
    let screen = app.screen();
    let state = |s: &str| screen.accessibility_state(s).unwrap();
    assert_eq!(
        state("role:checkbox[Accept terms]").checked,
        Some(Checked::False)
    );
    assert_eq!(state("role:checkbox[All]").checked, Some(Checked::Mixed));
    let locked = state("role:checkbox[Locked]");
    assert!(locked.disabled);
    assert_eq!(locked.checked, Some(Checked::True));
    assert!(state("role:checkbox[Required]").invalid);
    // A box alone is named by its label.
    assert!(screen.matches("role:checkbox[Select row]").unwrap());
    app.press("text:Accept terms").unwrap();
    app.wait_for("text:terms:true all:0 disabled:0").unwrap();
    assert_eq!(
        app.screen()
            .accessibility_state("role:checkbox[Accept terms]")
            .unwrap()
            .checked,
        Some(Checked::True)
    );
}

#[test]
fn a_disabled_checked_box_keeps_its_checked_fill() {
    let app = start::<Checkboxes>(true);
    let locked = indicator(&app, "role:checkbox[Locked]");
    let rest = indicator(&app, "role:checkbox[Accept terms]");
    assert!(fill(&app, locked).is_some());
    // Unchecked, in light, the box is transparent.
    assert_eq!(fill(&app, rest), None);
}

/// A medium checkbox and an extra small one.
#[component]
fn Sizes() -> impl IntoElement {
    view()
        .items_start()
        .child(checkbox().child("Medium"))
        .child(checkbox().size(Size::Xs).child("Small"))
}

#[test]
fn touch_screens_get_larger_boxes() {
    let desktop = start::<Sizes>(true);
    assert_eq!(
        side(&desktop, indicator(&desktop, "role:checkbox[Medium]")),
        (16.0, 16.0)
    );
    assert_eq!(
        side(&desktop, indicator(&desktop, "role:checkbox[Small]")),
        (12.0, 12.0)
    );
    let touch = start::<Sizes>(false);
    assert_eq!(
        side(&touch, indicator(&touch, "role:checkbox[Medium]")),
        (20.0, 20.0)
    );
    assert_eq!(
        side(&touch, indicator(&touch, "role:checkbox[Small]")),
        (16.0, 16.0)
    );
}

fn center_y(app: &Headless, selector: &str) -> f32 {
    let b = app.screen().layout(selector).unwrap().unwrap().bounds;
    b.origin.y.0 + b.size.height.0 / 2.0
}

#[test]
fn the_box_is_centered_on_the_labels_first_line() {
    for desktop in [true, false] {
        let app = start::<Sizes>(desktop);
        for name in ["Medium", "Small"] {
            let id = indicator(&app, &format!("role:checkbox[{name}]"));
            let box_y = center_y(&app, &format!("view:{id}"));
            let text_y = center_y(&app, &format!("text:{name}"));
            assert!(
                (box_y - text_y).abs() < 0.5,
                "{name}: box {box_y}, text {text_y}"
            );
        }
    }
}

/// A checkbox whose box turns blue while the checkbox's keyboard focus
/// shows, through the group the library names `nlb-checkbox`.
#[component]
fn Focus() -> impl IntoElement {
    view().child(
        checkbox()
            .indicator_style(|s| {
                s.group_focus_visible_named("nlb-checkbox", |s| {
                    s.bg(Color::parse("#2563eb").unwrap())
                })
            })
            .child("Focus me"),
    )
}

#[test]
fn the_keyboard_focus_reaches_the_box() {
    let mut app = start::<Focus>(true);
    let id = indicator(&app, "role:checkbox[Focus me]");
    let blue = Some(Color::parse("#2563eb").unwrap().0);
    assert_ne!(fill(&app, id), blue);
    app.press_key("Tab").unwrap();
    assert_eq!(fill(&app, id), blue);
}

/// The same checked checkbox outside and inside a scope that changes
/// `checkbox-checked`.
#[component]
fn Scoped() -> impl IntoElement {
    view()
        .child(checkbox().checked(true).child("Outside"))
        .child(
            theme_scope()
                .tokens(ScopeTokens::new().set("checkbox-checked", "#2563eb"))
                .child(checkbox().checked(true).child("Inside")),
        )
}

#[test]
fn a_scope_token_reaches_the_checked_fill() {
    let app = start::<Scoped>(true);
    let blue = Some(Color::parse("#2563eb").unwrap().0);
    assert_eq!(fill(&app, indicator(&app, "role:checkbox[Inside]")), blue);
    assert_ne!(fill(&app, indicator(&app, "role:checkbox[Outside]")), blue);
}
