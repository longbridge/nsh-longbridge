//! `Switch` in a headless engine: presses on the track and its label, the
//! keyboard, the disabled state, what screen readers get, pointer and
//! touch sizes, the keyboard focus reaching the track, and a scope's tokens
//! reaching its look.

use nsh_core::{Checked, Color, NodeId, Size as Viewport, platform::Os, prelude::*};
use nsh_engine::EngineConfig;
use nsh_longbridge::{ScopeTokens, Size, switch, theme_scope};
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

/// The views directly in `of`, in order.
fn children(app: &Headless, of: NodeId) -> Vec<NodeId> {
    app.screen()
        .layout_nodes()
        .iter()
        .filter(|n| n.parent == of)
        .map(|n| n.id)
        .collect()
}

/// The track of the switch `selector` names: the root's second view, after
/// `nsh/base`'s own track, which it hides.
fn track(app: &Headless, selector: &str) -> NodeId {
    let root = app.screen().id(selector).unwrap();
    children(app, root)[1]
}

fn bounds(app: &Headless, id: NodeId) -> (f32, f32) {
    let layout = app.screen().layout(&format!("view:{id}")).unwrap().unwrap();
    (layout.bounds.size.width.0, layout.bounds.size.height.0)
}

fn fill(app: &Headless, id: NodeId) -> Option<u32> {
    app.screen()
        .view_style(&format!("view:{id}"))
        .unwrap()
        .unwrap()
        .background_color
}

/// A controlled switch, a disabled one that is on, and a track alone.
#[component]
fn Switches() -> impl IntoElement {
    let wifi = signal(false);
    let disabled = signal(0u32);
    view()
        .items_start()
        .child(
            switch()
                .checked(move || wifi.get())
                .on_change(move |checked: &bool| wifi.set(*checked))
                .child("Wi-Fi"),
        )
        .child(
            switch()
                .disabled(true)
                .checked(true)
                .on_change(move |_: &bool| disabled.update(|n| *n += 1))
                .child("Managed"),
        )
        .child(switch().accessibility_label("Airplane mode"))
        .child(text(move || {
            format!("wifi:{} disabled:{}", wifi.get(), disabled.get())
        }))
}

#[test]
fn a_press_on_the_label_reports_the_next_value() {
    let mut app = start::<Switches>(true);
    app.press("text:Wi-Fi").unwrap();
    app.wait_for("text:wifi:true disabled:0").unwrap();
    app.press("text:Wi-Fi").unwrap();
    app.wait_for("text:wifi:false disabled:0").unwrap();
}

#[test]
fn space_toggles_the_focused_switch() {
    let mut app = start::<Switches>(true);
    app.press_key("Tab").unwrap();
    app.press_key("Space").unwrap();
    app.wait_for("text:wifi:true disabled:0").unwrap();
}

#[test]
fn a_disabled_switch_does_not_change() {
    let mut app = start::<Switches>(true);
    assert!(!app.screen().enabled("role:switch[Managed]").unwrap());
    let _ = app.press("text:Managed");
    app.press("text:Wi-Fi").unwrap();
    app.wait_for("text:wifi:true disabled:0").unwrap();
    let style = app
        .screen()
        .view_style("role:switch[Managed]")
        .unwrap()
        .unwrap();
    assert_eq!(style.opacity, 0.5);
}

#[test]
fn screen_readers_get_the_role_and_states() {
    let mut app = start::<Switches>(true);
    let screen = app.screen();
    assert_eq!(
        screen
            .accessibility_state("role:switch[Wi-Fi]")
            .unwrap()
            .checked,
        Some(Checked::False)
    );
    let managed = screen.accessibility_state("role:switch[Managed]").unwrap();
    assert!(managed.disabled);
    assert_eq!(managed.checked, Some(Checked::True));
    assert!(screen.matches("role:switch[Airplane mode]").unwrap());
    app.press("text:Wi-Fi").unwrap();
    app.wait_for("text:wifi:true disabled:0").unwrap();
    assert_eq!(
        app.screen()
            .accessibility_state("role:switch[Wi-Fi]")
            .unwrap()
            .checked,
        Some(Checked::True)
    );
}

#[test]
fn the_track_follows_the_value_and_disabled_keeps_it() {
    let mut app = start::<Switches>(true);
    let off = fill(&app, track(&app, "role:switch[Wi-Fi]"));
    let on = fill(&app, track(&app, "role:switch[Managed]"));
    assert_ne!(off, on);
    app.press("text:Wi-Fi").unwrap();
    app.wait_for("text:wifi:true disabled:0").unwrap();
    assert_eq!(fill(&app, track(&app, "role:switch[Wi-Fi]")), on);
}

/// A medium switch and an extra small one.
#[component]
fn Sizes() -> impl IntoElement {
    view()
        .items_start()
        .child(switch().child("Medium"))
        .child(switch().size(Size::Xs).child("Small"))
}

#[test]
fn touch_screens_get_larger_tracks() {
    let desktop = start::<Sizes>(true);
    assert_eq!(
        bounds(&desktop, track(&desktop, "role:switch[Medium]")),
        (32.0, 18.0)
    );
    assert_eq!(
        bounds(&desktop, track(&desktop, "role:switch[Small]")),
        (24.0, 14.0)
    );
    let medium = track(&desktop, "role:switch[Medium]");
    assert_eq!(
        bounds(&desktop, children(&desktop, medium)[0]),
        (16.0, 16.0)
    );
    let touch = start::<Sizes>(false);
    assert_eq!(
        bounds(&touch, track(&touch, "role:switch[Medium]")),
        (40.0, 22.0)
    );
    assert_eq!(
        bounds(&touch, track(&touch, "role:switch[Small]")),
        (32.0, 18.0)
    );
}

/// A switch whose track turns blue while the switch's keyboard focus
/// shows, through the group the library names `nlb-switch`.
#[component]
fn Focus() -> impl IntoElement {
    view().child(
        switch()
            .track_style(|s| {
                s.group_focus_visible_named("nlb-switch", |s| {
                    s.bg(Color::parse("#2563eb").unwrap())
                })
            })
            .child("Focus me"),
    )
}

#[test]
fn the_keyboard_focus_reaches_the_track() {
    let mut app = start::<Focus>(true);
    let id = track(&app, "role:switch[Focus me]");
    let blue = Some(Color::parse("#2563eb").unwrap().0);
    assert_ne!(fill(&app, id), blue);
    app.press_key("Tab").unwrap();
    assert_eq!(fill(&app, id), blue);
}

/// The same switch, on, outside and inside a scope that changes
/// `switch-checked`.
#[component]
fn Scoped() -> impl IntoElement {
    view().child(switch().checked(true).child("Outside")).child(
        theme_scope()
            .tokens(ScopeTokens::new().set("switch-checked", "#2563eb"))
            .child(switch().checked(true).child("Inside")),
    )
}

#[test]
fn a_scope_token_reaches_the_track() {
    let app = start::<Scoped>(true);
    let blue = Some(Color::parse("#2563eb").unwrap().0);
    assert_eq!(fill(&app, track(&app, "role:switch[Inside]")), blue);
    assert_ne!(fill(&app, track(&app, "role:switch[Outside]")), blue);
}
