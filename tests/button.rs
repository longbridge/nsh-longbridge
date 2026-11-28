//! `Button` in a headless engine: presses, the disabled and loading states,
//! what screen readers get, pointer and touch sizes, the gap between an
//! icon and its label at each size, and a scope's tokens reaching its look.

use nsh_core::{Color, NodeId, Size as Viewport, platform::Os, prelude::*};
use nsh_engine::EngineConfig;
use nsh_longbridge::{ButtonVariant, IconName, ScopeTokens, Size, button, theme_scope};
use nsh_testing::Headless;

fn start<C: Component<Props = ()>>(desktop: bool) -> Headless {
    let mut config = EngineConfig {
        viewport: Viewport::new(px(800.0), px(600.0)),
        ..EngineConfig::new(nsh_testing::windowed_root::<C>())
    };
    if desktop {
        config.environment.os = Os::MacOs;
    } else {
        config.environment.os = Os::Ios;
    }
    let mut app = Headless::start(config).unwrap();
    app.first_render().unwrap();
    app
}

/// A plain button, a disabled one, a loading one and an icon-only one
/// loading without a label, each counting its presses.
#[component]
fn Buttons() -> impl IntoElement {
    let presses = signal(0u32);
    let disabled = signal(0u32);
    let loading = signal(0u32);
    view()
        .child(
            button()
                .on_press(move || presses.update(|n| *n += 1))
                .child("Save"),
        )
        .child(
            button()
                .disabled(true)
                .on_press(move || disabled.update(|n| *n += 1))
                .child("Disabled"),
        )
        .child(
            button()
                .loading(true)
                .on_press(move || loading.update(|n| *n += 1))
                .child("Saving"),
        )
        .child(
            button()
                .variant(ButtonVariant::Ghost)
                .icon_only(true)
                .icon(IconName::Plus)
                .loading(true),
        )
        .child(text(move || {
            format!(
                "presses:{} disabled:{} loading:{}",
                presses.get(),
                disabled.get(),
                loading.get()
            )
        }))
}

#[test]
fn a_press_reaches_on_press() {
    let mut app = start::<Buttons>(true);
    app.press("text:Save").unwrap();
    app.wait_for("text:presses:1 disabled:0 loading:0").unwrap();
}

#[test]
fn disabled_and_loading_buttons_do_not_press() {
    let mut app = start::<Buttons>(true);
    assert!(!app.screen().enabled("role:button[Disabled]").unwrap());
    let _ = app.press("text:Disabled");
    let _ = app.press("text:Saving");
    app.press("text:Save").unwrap();
    app.wait_for("text:presses:1 disabled:0 loading:0").unwrap();
}

#[test]
fn screen_readers_get_the_role_and_states() {
    let app = start::<Buttons>(true);
    let screen = app.screen();
    assert!(screen.matches("role:button[Save]").unwrap());
    assert!(
        screen
            .accessibility_state("role:button[Disabled]")
            .unwrap()
            .disabled
    );
    let saving = screen.accessibility_state("role:button[Saving]").unwrap();
    assert!(saving.busy && !saving.disabled);
    // An icon-only button loading without a label is named by `labels`.
    assert!(
        screen
            .accessibility_state("role:button[Loading]")
            .unwrap()
            .busy
    );
}

/// A medium button and an extra small one.
#[component]
fn Sizes() -> impl IntoElement {
    view()
        .items_start()
        .child(button().child("Medium"))
        .child(button().size(Size::Xs).child("Small"))
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
fn touch_screens_get_taller_buttons() {
    let desktop = start::<Sizes>(true);
    assert_eq!(height(&desktop, "role:button[Medium]"), 32.0);
    assert_eq!(height(&desktop, "role:button[Small]"), 24.0);
    let touch = start::<Sizes>(false);
    assert_eq!(height(&touch, "role:button[Medium]"), 40.0);
    assert_eq!(height(&touch, "role:button[Small]"), 32.0);
}

/// A button with an icon and a label at each size.
#[component]
fn IconSizes() -> impl IntoElement {
    let one = |size: Size, name: &'static str| {
        button()
            .size(size)
            .icon(IconName::Plus)
            .accessibility_label(name)
            .child("New file")
    };
    view()
        .items_start()
        .child(one(Size::Xs, "xs"))
        .child(one(Size::Sm, "sm"))
        .child(one(Size::Md, "md"))
        .child(one(Size::Lg, "lg"))
}

/// The views directly in the one `selector` names, in order.
fn parts(app: &Headless, selector: &str) -> Vec<NodeId> {
    let screen = app.screen();
    let of = screen.id(selector).unwrap();
    screen
        .layout_nodes()
        .iter()
        .filter(|n| n.parent == of)
        .map(|n| n.id)
        .collect()
}

#[test]
fn the_icon_label_gap_is_4_at_xs_and_sm_and_8_at_md_and_lg() {
    let app = start::<IconSizes>(true);
    for (size, gap) in [("xs", 4.0), ("sm", 4.0), ("md", 8.0), ("lg", 8.0)] {
        let parts = parts(&app, &format!("role:button[{size}]"));
        assert_eq!(parts.len(), 2, "{size}: the icon and the label");
        let edge = |node: NodeId| {
            let b = app
                .screen()
                .layout(&format!("view:{node}"))
                .unwrap()
                .unwrap()
                .bounds;
            (b.origin.x.0, b.origin.x.0 + b.size.width.0)
        };
        let (icon, label) = (edge(parts[0]), edge(parts[1]));
        assert_eq!(label.0 - icon.1, gap, "{size}");
    }
}

/// The same button outside and inside a scope that changes `primary`.
#[component]
fn Scoped() -> impl IntoElement {
    view().child(button().child("Outside")).child(
        theme_scope()
            .tokens(ScopeTokens::new().set("primary", "#2563eb"))
            .child(button().child("Inside")),
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
fn a_scope_primary_reaches_the_default_fill() {
    let app = start::<Scoped>(true);
    let blue = Some(Color::parse("#2563eb").unwrap().0);
    assert_eq!(fill(&app, "role:button[Inside]"), blue);
    assert_ne!(fill(&app, "role:button[Outside]"), blue);
}
