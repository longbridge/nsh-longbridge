//! `RadioGroup` and `RadioGroupItem` in a headless engine: the group's
//! value checking its items, presses and the arrow keys reporting values,
//! the group's disabled state, what screen readers get, pointer and touch
//! sizes, the dot on whole points at every size, the keyboard focus
//! reaching the circle, and a scope's tokens reaching its look.

use nsh_core::{Checked, Color, NodeId, Size as Viewport, platform::Os, prelude::*};
use nsh_engine::EngineConfig;
use nsh_longbridge::{ScopeTokens, Size, radio_group, radio_group_item, theme_scope};
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

/// The circle of the item `selector` names: its first view.
fn circle(app: &Headless, selector: &str) -> NodeId {
    let root = app.screen().id(selector).unwrap();
    children(app, root)[0]
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

/// A plan picker (the last item disabled), a disabled group, and an item
/// outside any group.
#[component]
fn Groups() -> impl IntoElement {
    let plan = signal(Some(SharedString::from("monthly")));
    let changes = signal(0u32);
    view()
        .items_start()
        .child(
            radio_group()
                .accessibility_label("Plan")
                .value(move || plan.get())
                .on_change(move |value: &SharedString| {
                    changes.update(|n| *n += 1);
                    plan.set(Some(value.clone()));
                })
                .child(radio_group_item().value("monthly").child("Monthly"))
                .child(radio_group_item().value("yearly").child("Yearly"))
                .child(
                    radio_group_item()
                        .value("lifetime")
                        .disabled(true)
                        .child("Lifetime"),
                ),
        )
        .child(
            radio_group()
                .accessibility_label("Sort")
                .disabled(true)
                .value(Some(SharedString::from("name")))
                .child(radio_group_item().value("name").child("Name"))
                .child(radio_group_item().value("date").child("Date")),
        )
        .child(radio_group_item().value("alone").child("Alone"))
        .child(text(move || {
            format!(
                "plan:{} changes:{}",
                plan.get().as_deref().unwrap_or("none"),
                changes.get()
            )
        }))
}

#[test]
fn a_press_reports_the_items_value() {
    let mut app = start::<Groups>(true);
    app.press("text:Yearly").unwrap();
    app.wait_for("text:plan:yearly changes:1").unwrap();
    // The checked item reports nothing.
    app.press("text:Yearly").unwrap();
    app.press("text:Monthly").unwrap();
    app.wait_for("text:plan:monthly changes:2").unwrap();
}

#[test]
fn the_arrows_move_and_check_skipping_disabled_items() {
    let mut app = start::<Groups>(true);
    app.press_key("Tab").unwrap();
    app.press_key("ArrowDown").unwrap();
    app.wait_for("text:plan:yearly changes:1").unwrap();
    // Past the disabled item, around the end.
    app.press_key("ArrowDown").unwrap();
    app.wait_for("text:plan:monthly changes:2").unwrap();
}

#[test]
fn the_groups_disabled_reaches_its_items() {
    let mut app = start::<Groups>(true);
    assert!(!app.screen().enabled("role:radio[Date]").unwrap());
    assert!(!app.screen().enabled("role:radio[Lifetime]").unwrap());
    let _ = app.press("text:Date");
    let _ = app.press("text:Lifetime");
    app.press("text:Yearly").unwrap();
    app.wait_for("text:plan:yearly changes:1").unwrap();
    let style = app
        .screen()
        .view_style("role:radio[Date]")
        .unwrap()
        .unwrap();
    assert_eq!(style.opacity, 0.5);
}

#[test]
fn screen_readers_get_the_roles_and_states() {
    let mut app = start::<Groups>(true);
    let screen = app.screen();
    assert!(screen.matches("role:radiogroup[Plan]").unwrap());
    let monthly = screen.accessibility_state("role:radio[Monthly]").unwrap();
    assert_eq!(monthly.checked, Some(Checked::True));
    assert!(monthly.selected);
    let yearly = screen.accessibility_state("role:radio[Yearly]").unwrap();
    assert_eq!(yearly.checked, Some(Checked::False));
    assert!(
        screen
            .accessibility_state("role:radio[Date]")
            .unwrap()
            .disabled
    );
    // Outside a group an item is never checked.
    assert_eq!(
        screen
            .accessibility_state("role:radio[Alone]")
            .unwrap()
            .checked,
        Some(Checked::False)
    );
    app.press("text:Yearly").unwrap();
    app.wait_for("text:plan:yearly changes:1").unwrap();
    let screen = app.screen();
    assert_eq!(
        screen
            .accessibility_state("role:radio[Yearly]")
            .unwrap()
            .checked,
        Some(Checked::True)
    );
    assert_eq!(
        screen
            .accessibility_state("role:radio[Monthly]")
            .unwrap()
            .checked,
        Some(Checked::False)
    );
}

#[test]
fn the_checked_item_shows_a_dot() {
    let mut app = start::<Groups>(true);
    let monthly = circle(&app, "role:radio[Monthly]");
    let yearly = circle(&app, "role:radio[Yearly]");
    assert_eq!(children(&app, monthly).len(), 1);
    assert!(children(&app, yearly).is_empty());
    app.press("text:Yearly").unwrap();
    app.wait_for("text:plan:yearly changes:1").unwrap();
    assert!(children(&app, monthly).is_empty());
    let dot = children(&app, yearly);
    assert_eq!(bounds(&app, dot[0]), (8.0, 8.0));
}

/// A medium group and an extra small one.
#[component]
fn Sizes() -> impl IntoElement {
    view()
        .items_start()
        .child(radio_group().child(radio_group_item().value("m").child("Medium")))
        .child(
            radio_group()
                .size(Size::Xs)
                .child(radio_group_item().value("s").child("Small")),
        )
}

#[test]
fn touch_screens_get_larger_circles() {
    let desktop = start::<Sizes>(true);
    assert_eq!(
        bounds(&desktop, circle(&desktop, "role:radio[Medium]")),
        (16.0, 16.0)
    );
    assert_eq!(
        bounds(&desktop, circle(&desktop, "role:radio[Small]")),
        (12.0, 12.0)
    );
    let touch = start::<Sizes>(false);
    assert_eq!(
        bounds(&touch, circle(&touch, "role:radio[Medium]")),
        (20.0, 20.0)
    );
    assert_eq!(
        bounds(&touch, circle(&touch, "role:radio[Small]")),
        (16.0, 16.0)
    );
}

/// A checked item of each size.
#[component]
fn Dots() -> impl IntoElement {
    let one = |size: Size, name: &'static str| {
        radio_group()
            .size(size)
            .value(Some(SharedString::from(name)))
            .child(radio_group_item().value(name).child(name))
    };
    view()
        .items_start()
        .child(one(Size::Xs, "xs"))
        .child(one(Size::Sm, "sm"))
        .child(one(Size::Md, "md"))
        .child(one(Size::Lg, "lg"))
}

/// The dot is centered in the circle on whole points, at every size (a
/// half circle's side would put the odd ones on half points), with a
/// pointer and on touch screens.
#[test]
fn the_dot_sits_on_whole_points_at_every_size() {
    for desktop in [true, false] {
        let app = start::<Dots>(desktop);
        for name in ["xs", "sm", "md", "lg"] {
            let circle = circle(&app, &format!("role:radio[{name}]"));
            let dot = children(&app, circle)[0];
            let at = |id: NodeId| {
                app.screen()
                    .layout(&format!("view:{id}"))
                    .unwrap()
                    .unwrap()
                    .bounds
            };
            let (circle, dot) = (at(circle), at(dot));
            let start = (dot.origin.x - circle.origin.x).0;
            let top = (dot.origin.y - circle.origin.y).0;
            let end = (circle.origin.x + circle.size.width - dot.origin.x - dot.size.width).0;
            let what = format!("{name}, desktop {desktop}: {dot:?} in {circle:?}");
            assert_eq!(start.fract(), 0.0, "{what}");
            assert_eq!(top.fract(), 0.0, "{what}");
            assert_eq!(start, end, "centered: {what}");
            assert_eq!(start, top, "centered: {what}");
        }
    }
}

/// An item whose circle turns blue while the item's keyboard focus shows,
/// through the group the library names `nlb-radio`.
#[component]
fn Focus() -> impl IntoElement {
    radio_group().child(
        radio_group_item()
            .value("a")
            .indicator_style(|s| {
                s.group_focus_visible_named("nlb-radio", |s| s.bg(Color::parse("#2563eb").unwrap()))
            })
            .child("Focus me"),
    )
}

#[test]
fn the_keyboard_focus_reaches_the_circle() {
    let mut app = start::<Focus>(true);
    let id = circle(&app, "role:radio[Focus me]");
    let blue = Some(Color::parse("#2563eb").unwrap().0);
    assert_ne!(fill(&app, id), blue);
    app.press_key("Tab").unwrap();
    assert_eq!(fill(&app, id), blue);
}

/// The same checked item outside and inside a scope that changes
/// `radio-group-item-dot`.
#[component]
fn Scoped() -> impl IntoElement {
    let group = |name: &'static str| {
        radio_group()
            .value(Some(SharedString::from(name)))
            .child(radio_group_item().value(name).child(name))
    };
    view().child(group("Outside")).child(
        theme_scope()
            .tokens(ScopeTokens::new().set("radio-group-item-dot", "#2563eb"))
            .child(group("Inside")),
    )
}

#[test]
fn a_scope_token_reaches_the_dot() {
    let app = start::<Scoped>(true);
    let blue = Some(Color::parse("#2563eb").unwrap().0);
    let dot = |name: &str| children(&app, circle(&app, &format!("role:radio[{name}]")))[0];
    assert_eq!(fill(&app, dot("Inside")), blue);
    assert_ne!(fill(&app, dot("Outside")), blue);
}
