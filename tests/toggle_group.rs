//! `ToggleGroup` in a headless engine: one value or several, controlled or
//! its own, a pressed single item letting go, disabled, what screen readers
//! get, the arrows moving without pressing, the items' sizes and slop, the
//! joined shape (corners, one line per boundary, the row's shadow) and a
//! group apart.

mod common;

use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
    time::Duration,
};

use common::{fill, light, size, start};
use nsh_core::{Checked, MountItem, NodeId, Size as Viewport, ViewStyle, platform::Os, prelude::*};
use nsh_engine::EngineConfig;
use nsh_longbridge::{
    Size, ToggleVariant, components::toggle::TOGGLE_PRESSED, toggle_group, toggle_group_item,
};
use nsh_testing::Headless;

fn shown(value: &[SharedString]) -> String {
    value
        .iter()
        .map(|v| v.to_string())
        .collect::<Vec<_>>()
        .join("+")
}

/// A controlled single group (alignment), an uncontrolled multiple one
/// (formatting), a disabled group and one with a disabled item.
#[component]
fn Groups() -> impl IntoElement {
    let align = signal(vec![SharedString::from("left")]);
    let format = signal(String::new());
    let changes = signal(0);
    view()
        .flex_col()
        .items_start()
        .gap(px(8.0))
        .p(px(20.0))
        .child(
            toggle_group()
                .value(move || Some(align.get()))
                .on_change(move |v: &Vec<SharedString>| {
                    changes.update(|c| *c += 1);
                    align.set(v.clone());
                })
                .accessibility_label("Text alignment")
                .child(
                    toggle_group_item()
                        .value("left")
                        .accessibility_label("Left")
                        .child(text("L")),
                )
                .child(
                    toggle_group_item()
                        .value("center")
                        .accessibility_label("Center")
                        .child(text("C")),
                )
                .child(
                    toggle_group_item()
                        .value("right")
                        .accessibility_label("Right")
                        .child(text("R")),
                ),
        )
        .child(
            toggle_group()
                .multiple(true)
                .default_value(vec![SharedString::from("bold")])
                .on_change(move |v: &Vec<SharedString>| format.set(shown(v)))
                .accessibility_label("Formatting")
                .child(
                    toggle_group_item()
                        .value("bold")
                        .accessibility_label("Bold")
                        .child(text("B")),
                )
                .child(
                    toggle_group_item()
                        .value("italic")
                        .accessibility_label("Italic")
                        .child(text("I")),
                ),
        )
        .child(
            toggle_group()
                .disabled(true)
                .accessibility_label("Locked")
                .child(
                    toggle_group_item()
                        .value("a")
                        .accessibility_label("Locked A")
                        .child(text("A")),
                ),
        )
        .child(
            toggle_group()
                .accessibility_label("Partly")
                .child(
                    toggle_group_item()
                        .value("x")
                        .accessibility_label("Open X")
                        .child(text("X")),
                )
                .child(
                    toggle_group_item()
                        .value("y")
                        .disabled(true)
                        .accessibility_label("Shut Y")
                        .child(text("Y")),
                ),
        )
        .child(text(move || {
            format!(
                "align:{} format:{} changes:{}",
                shown(&align.get()),
                format.get(),
                changes.get()
            )
        }))
}

fn pressed(app: &Headless, selector: &str) -> Option<Checked> {
    app.screen().accessibility_state(selector).unwrap().pressed
}

#[test]
fn a_single_group_holds_one_value_and_lets_the_pressed_one_go() {
    let mut app = start::<Groups>(true);
    assert_eq!(pressed(&app, "role:button[Left]"), Some(Checked::True));
    assert_eq!(pressed(&app, "role:button[Center]"), Some(Checked::False));
    app.press("role:button[Center]").unwrap();
    app.wait_for("text:align:center format: changes:1").unwrap();
    assert_eq!(pressed(&app, "role:button[Left]"), Some(Checked::False));
    assert_eq!(pressed(&app, "role:button[Center]"), Some(Checked::True));
    app.press("role:button[Center]").unwrap();
    app.wait_for("text:align: format: changes:2").unwrap();
    assert_eq!(pressed(&app, "role:button[Center]"), Some(Checked::False));
}

#[test]
fn a_multiple_group_keeps_its_own_value_and_adds_or_removes() {
    let mut app = start::<Groups>(true);
    assert_eq!(pressed(&app, "role:button[Bold]"), Some(Checked::True));
    app.press("role:button[Italic]").unwrap();
    app.wait_for("text:align:left format:bold+italic changes:0")
        .unwrap();
    app.press("role:button[Bold]").unwrap();
    app.wait_for("text:align:left format:italic changes:0")
        .unwrap();
    assert_eq!(pressed(&app, "role:button[Bold]"), Some(Checked::False));
    assert_eq!(pressed(&app, "role:button[Italic]"), Some(Checked::True));
}

#[test]
fn a_disabled_group_or_item_reports_nothing() {
    let mut app = start::<Groups>(true);
    let screen = app.screen();
    assert!(!screen.enabled("role:button[Locked A]").unwrap());
    assert!(screen.enabled("role:button[Open X]").unwrap());
    assert!(!screen.enabled("role:button[Shut Y]").unwrap());
    let _ = app.press("role:button[Shut Y]");
    app.run_for(Duration::from_millis(50)).unwrap();
    assert_eq!(pressed(&app, "role:button[Shut Y]"), Some(Checked::False));
}

#[test]
fn screen_readers_get_a_named_set_of_toggle_buttons() {
    let app = start::<Groups>(true);
    let screen = app.screen();
    assert!(screen.matches("role:group[Text alignment]").unwrap());
    assert!(screen.matches("role:group[Formatting]").unwrap());
    assert_eq!(screen.count("role:toolbar").unwrap(), 4);
}

#[test]
fn the_arrows_move_the_focus_without_pressing() {
    let mut app = start::<Groups>(true);
    app.press_key("Tab").unwrap();
    app.press_key("ArrowRight").unwrap();
    app.run_for(Duration::from_millis(50)).unwrap();
    assert!(app.shows("align:left format: changes:0"));
    app.press_key("Space").unwrap();
    app.wait_for("text:align:center format: changes:1").unwrap();
}

#[test]
fn the_pressed_item_takes_the_toggle_fill() {
    let app = start::<Groups>(true);
    assert_eq!(fill(&app, "role:button[Left]"), light(TOGGLE_PRESSED));
    assert_eq!(fill(&app, "role:button[Center]"), None);
}

/// A joined outline row, an outline row apart, and sizes.
#[component]
fn Shapes() -> impl IntoElement {
    let items = |prefix: &'static str| {
        ["a", "b", "c"].map(move |v| {
            toggle_group_item()
                .value(v)
                .accessibility_label(format!("{prefix} {v}"))
                .child(text(v))
        })
    };
    let [a, b, c] = items("Joined");
    let [d, e, f] = items("Apart");
    let [g, h, i] = items("Small");
    view()
        .flex_col()
        .items_start()
        .gap(px(8.0))
        .p(px(20.0))
        .child(
            toggle_group()
                .variant(ToggleVariant::Outline)
                .accessibility_label("Joined")
                .child(a)
                .child(b)
                .child(c),
        )
        .child(
            toggle_group()
                .variant(ToggleVariant::Outline)
                .spacing(2.0)
                .accessibility_label("Apart")
                .child(d)
                .child(e)
                .child(f),
        )
        .child(
            toggle_group()
                .size(Size::Sm)
                .accessibility_label("Small")
                .child(g)
                .child(h)
                .child(i),
        )
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

fn style_of(
    app: &Headless,
    styles: &Arc<Mutex<HashMap<NodeId, ViewStyle>>>,
    selector: &str,
) -> ViewStyle {
    let id = app.screen().id(selector).unwrap();
    styles
        .lock()
        .unwrap()
        .get(&id)
        .cloned()
        .unwrap_or_else(|| panic!("no style for {selector}"))
}

fn shadows(style: &ViewStyle) -> usize {
    style
        .effects
        .as_ref()
        .map(|e| e.box_shadows.len())
        .unwrap_or(0)
}

/// `Shapes` as `common::start` makes it, with every style it sends kept
/// from the first frame.
fn start_shapes(desktop: bool) -> (Headless, Arc<Mutex<HashMap<NodeId, ViewStyle>>>) {
    let mut config = EngineConfig {
        viewport: Viewport::new(px(800.0), px(600.0)),
        clock: nsh_engine::Clock::manual(std::time::SystemTime::now()),
        ..EngineConfig::new(nsh_testing::windowed_root::<Shapes>())
    };
    config.environment.os = if desktop { Os::MacOs } else { Os::Ios };
    let mut app = Headless::start(config).unwrap();
    let styles = watch_styles(&mut app);
    app.first_render().unwrap();
    (app, styles)
}

/// The group (the root) around the toolbar holding `item`.
fn row_of(app: &Headless, item: &str) -> NodeId {
    let nodes = app.screen().layout_nodes();
    let parent = |at: NodeId| nodes.iter().find(|n| n.id == at).unwrap().parent;
    parent(parent(app.screen().id(item).unwrap()))
}

#[test]
fn a_joined_row_is_one_shape_with_one_line_per_boundary() {
    let (app, styles) = start_shapes(true);
    let (a, b, c) = (
        style_of(&app, &styles, "role:button[Joined a]"),
        style_of(&app, &styles, "role:button[Joined b]"),
        style_of(&app, &styles, "role:button[Joined c]"),
    );
    // The first item's start corners and the last's end corners.
    let corners = |s: &ViewStyle| {
        let r = s.radii();
        // Start (top and bottom left), end (top and bottom right).
        (
            r.top_left.0,
            r.bottom_left.0,
            r.top_right.0,
            r.bottom_right.0,
        )
    };
    assert_eq!(corners(&a), (8.0, 8.0, 0.0, 0.0), "{a:?}");
    assert_eq!(corners(&b), (0.0, 0.0, 0.0, 0.0), "{b:?}");
    assert_eq!(corners(&c), (0.0, 0.0, 8.0, 8.0), "{c:?}");
    // A start line on the first item only.
    assert_eq!(a.borders.widths.left.0, 1.0);
    assert_eq!(b.borders.widths.left.0, 0.0);
    assert_eq!(c.borders.widths.left.0, 0.0);
    assert_eq!(b.borders.widths.right.0, 1.0);
    // The row carries the shadow, its items none.
    assert_eq!(shadows(&a) + shadows(&b) + shadows(&c), 0);
    let row = row_of(&app, "role:button[Joined a]");
    let row = styles.lock().unwrap().get(&row).cloned().unwrap();
    assert_eq!(shadows(&row), 1, "{row:?}");
    assert_eq!(row.border_radius.0, 8.0);
    // The items touch.
    let ab = common::bounds(&app, "role:button[Joined a]");
    let bb = common::bounds(&app, "role:button[Joined b]");
    assert_eq!(ab.origin.x + ab.size.width, bb.origin.x);
}

#[test]
fn a_row_apart_has_whole_toggles_and_the_gap() {
    let (app, styles) = start_shapes(true);
    for v in ["a", "b", "c"] {
        let style = style_of(&app, &styles, &format!("role:button[Apart {v}]"));
        assert_eq!(style.border_radius.0, 8.0, "{style:?}");
        assert_eq!(style.borders.widths.left.0, 1.0);
        assert_eq!(shadows(&style), 1);
    }
    // The row draws nothing of its own (no style sent, or one without a
    // shadow).
    let row = row_of(&app, "role:button[Apart a]");
    let row = styles.lock().unwrap().get(&row).cloned();
    assert_eq!(row.map_or(0, |r| shadows(&r)), 0);
    let a = common::bounds(&app, "role:button[Apart a]");
    let b = common::bounds(&app, "role:button[Apart b]");
    assert_eq!(b.origin.x - (a.origin.x + a.size.width), px(8.0));
}

#[test]
fn items_take_the_groups_size_and_padding_without_a_least_width() {
    let app = start::<Shapes>(true);
    assert_eq!(size(&app, "role:button[Small a]").1, 28.0);
    // 12 each side around one letter: wider than tall here, never a square
    // of the height.
    assert_eq!(size(&app, "role:button[Joined a]").1, 32.0);
    let touch = start::<Shapes>(false);
    assert_eq!(size(&touch, "role:button[Small a]").1, 36.0);
    assert_eq!(size(&touch, "role:button[Joined a]").1, 40.0);
}
