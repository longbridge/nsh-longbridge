//! `Slider` in a headless engine: its value reported through its events
//! (a drag, a key), a range's two thumbs, disabled, the press area's,
//! rail's and thumb's sizes with a pointer and on touch screens, a
//! vertical slider, the fills, edge and shadow from the tokens, the ring
//! under the pointer, and a scoped token reaching the look.

mod common;

use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
    time::Duration,
};

use common::{bounds, fill, light, size, start};
use nsh_core::{
    MountItem, NodeId, Size as Viewport, ViewStyle, geometry::Axis, platform::Os, prelude::*,
};
use nsh_engine::EngineConfig;
use nsh_longbridge::{
    ScopeTokens, SliderValue,
    components::slider::{SLIDER_RANGE, SLIDER_THUMB, SLIDER_TRACK},
    slider, theme_scope,
};
use nsh_testing::{DragOptions, Headless};

fn shown(value: SliderValue) -> String {
    match value {
        SliderValue::Single(v) => format!("{v}"),
        SliderValue::Range(a, b) => format!("{a}-{b}"),
    }
}

/// A controlled volume 200 wide, a range, a disabled slider and a
/// vertical one.
#[component]
fn Sliders() -> impl IntoElement {
    let volume = signal(40.0);
    let ends = signal(Vec::<String>::new());
    let price = signal((20.0, 80.0));
    view()
        .flex_col()
        .items_start()
        .gap(px(16.0))
        .p(px(20.0))
        .child(
            view().w(px(200.0)).child(
                slider()
                    .value(move || Some(SliderValue::Single(volume.get())))
                    .on_change(move |v: &SliderValue| volume.set(v.end()))
                    .on_change_end(move |v: &SliderValue| ends.update(|e| e.push(shown(*v))))
                    .step(5.0)
                    .accessibility_label("Volume"),
            ),
        )
        .child(
            view().w(px(200.0)).child(
                slider()
                    .value(move || {
                        let (a, b) = price.get();
                        Some(SliderValue::Range(a, b))
                    })
                    .on_change(move |v: &SliderValue| price.set((v.start(), v.end())))
                    .start_accessibility_label("Minimum price")
                    .accessibility_label("Maximum price"),
            ),
        )
        .child(
            view().w(px(200.0)).child(
                slider()
                    .default_value(SliderValue::Single(30.0))
                    .disabled(true)
                    .accessibility_label("Locked"),
            ),
        )
        .child(
            view().h(px(120.0)).child(
                slider()
                    .axis(Axis::Vertical)
                    .default_value(SliderValue::Single(50.0))
                    .accessibility_label("Brightness"),
            ),
        )
        .child(text(move || {
            let (a, b) = price.get();
            format!(
                "volume:{} price:{a}-{b} ends:{}",
                volume.get(),
                ends.get().join(",")
            )
        }))
}

/// The parent of `id`.
fn parent(app: &Headless, id: NodeId) -> NodeId {
    let nodes = app.screen().layout_nodes();
    nodes.iter().find(|n| n.id == id).unwrap().parent
}

/// The first child of `id`.
fn first_child(app: &Headless, id: NodeId) -> NodeId {
    let nodes = app.screen().layout_nodes();
    nodes.iter().find(|n| n.parent == id).unwrap().id
}

/// The rail (`nsh/base`'s indicator) of the slider whose thumb `thumb`
/// names, as a `view:` selector.
fn rail(app: &Headless, thumb: &str) -> String {
    let id = parent(app, app.screen().id(thumb).unwrap());
    format!("view:{id}")
}

/// The press area (`nsh/base`'s track) around the rail.
fn press_area(app: &Headless, thumb: &str) -> String {
    let rail = parent(app, app.screen().id(thumb).unwrap());
    format!("view:{}", parent(app, rail))
}

/// The filled stretch (the rail's first child).
fn range(app: &Headless, thumb: &str) -> String {
    let rail = parent(app, app.screen().id(thumb).unwrap());
    format!("view:{}", first_child(app, rail))
}

#[test]
fn a_drag_reports_each_value_and_its_end_once() {
    let mut app = start::<Sliders>(true);
    app.wait_for("text:volume:40 price:20-80 ends:").unwrap();
    // 40 points of 200: a fifth of the range.
    app.drag(
        "label:Volume",
        Point::new(px(40.0), px(0.0)),
        DragOptions::default(),
    )
    .unwrap();
    app.wait_for("text:volume:60 price:20-80 ends:60").unwrap();
}

#[test]
fn the_keys_move_the_focused_thumb_by_the_step() {
    let mut app = start::<Sliders>(true);
    app.press_key("Tab").unwrap();
    app.press_key("ArrowRight").unwrap();
    app.wait_for("text:volume:45 price:20-80 ends:45").unwrap();
}

#[test]
fn a_range_has_two_named_thumbs_that_move_apart() {
    let mut app = start::<Sliders>(true);
    let screen = app.screen();
    assert!(screen.matches("role:adjustable[Minimum price]").unwrap());
    assert!(screen.matches("role:adjustable[Maximum price]").unwrap());
    app.drag(
        "label:Minimum price",
        Point::new(px(20.0), px(0.0)),
        DragOptions::default(),
    )
    .unwrap();
    app.wait_for("text:volume:40 price:30-80 ends:").unwrap();
}

/// The slider's root (around the press area).
fn root(app: &Headless, thumb: &str) -> String {
    let rail = parent(app, app.screen().id(thumb).unwrap());
    format!("view:{}", parent(app, parent(app, rail)))
}

#[test]
fn a_disabled_slider_is_dimmed_and_takes_no_drag() {
    let mut app = start::<Sliders>(true);
    assert!(!app.screen().enabled("role:adjustable[Locked]").unwrap());
    let style = app
        .screen()
        .view_style(&root(&app, "label:Locked"))
        .unwrap()
        .unwrap();
    assert_eq!(style.opacity, 0.5);
    let _ = app.drag(
        "label:Locked",
        Point::new(px(40.0), px(0.0)),
        DragOptions::default(),
    );
    app.run_for(Duration::from_millis(50)).unwrap();
    assert!(app.shows("volume:40 price:20-80 ends:"));
}

#[test]
fn sizes_with_a_pointer_and_on_touch_screens() {
    let app = start::<Sliders>(true);
    // As wide as its parent, as tall as the thumb.
    assert_eq!(size(&app, &root(&app, "label:Volume")), (200.0, 16.0));
    assert_eq!(size(&app, &press_area(&app, "label:Volume")), (200.0, 16.0));
    assert_eq!(size(&app, &rail(&app, "label:Volume")), (200.0, 6.0));
    assert_eq!(size(&app, "label:Volume"), (16.0, 16.0));
    // The rail is centered in the slider's box, so the thumb (as tall as
    // the box, centered on the rail) sits inside it.
    let slider = bounds(&app, &root(&app, "label:Volume"));
    let line = bounds(&app, &rail(&app, "label:Volume"));
    assert_eq!(line.origin.y, slider.origin.y + px(5.0));
    let touch = start::<Sliders>(false);
    assert_eq!(size(&touch, &root(&touch, "label:Volume")), (200.0, 44.0));
    assert_eq!(size(&touch, &rail(&touch, "label:Volume")).1, 6.0);
    assert_eq!(size(&touch, "label:Volume"), (20.0, 20.0));
}

#[test]
fn a_vertical_slider_is_as_tall_as_its_parent_and_a_thumb_wide() {
    let app = start::<Sliders>(true);
    let (w, h) = size(&app, &root(&app, "label:Brightness"));
    assert_eq!(w, 16.0);
    // At least `slider-vertical-min-height`, past its 120-tall parent.
    assert_eq!(h, 176.0);
    assert_eq!(size(&app, &rail(&app, "label:Brightness")), (6.0, 176.0));
}

#[test]
fn the_rail_range_and_thumb_take_the_tokens_fills() {
    let app = start::<Sliders>(true);
    assert_eq!(fill(&app, &rail(&app, "label:Volume")), light(SLIDER_TRACK));
    assert_eq!(
        fill(&app, &range(&app, "label:Volume")),
        light(SLIDER_RANGE)
    );
    assert_eq!(fill(&app, "label:Volume"), light(SLIDER_THUMB));
    // The range runs from `min` to the thumb.
    assert_eq!(size(&app, &range(&app, "label:Volume")).0, 80.0);
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

/// `Sliders` as `common::start` makes it, with every style it sends kept
/// from the first frame.
fn start_watched() -> (Headless, Arc<Mutex<HashMap<NodeId, ViewStyle>>>) {
    let mut config = EngineConfig {
        viewport: Viewport::new(px(800.0), px(600.0)),
        clock: nsh_engine::Clock::manual(std::time::SystemTime::now()),
        ..EngineConfig::new(nsh_testing::windowed_root::<Sliders>())
    };
    config.environment.os = Os::MacOs;
    let mut app = Headless::start(config).unwrap();
    let styles = watch_styles(&mut app);
    app.first_render().unwrap();
    (app, styles)
}

fn thumb_style(app: &Headless, styles: &Arc<Mutex<HashMap<NodeId, ViewStyle>>>) -> ViewStyle {
    let id = app.screen().id("label:Volume").unwrap();
    styles.lock().unwrap().get(&id).cloned().unwrap()
}

#[test]
fn the_thumb_has_an_edge_a_shadow_and_a_ring_under_the_pointer() {
    let (mut app, styles) = start_watched();
    let rest = thumb_style(&app, &styles);
    assert_eq!(rest.borders.widths.top.0, 1.0);
    assert_eq!(rest.borders.colors[0].0, light(SLIDER_RANGE).unwrap());
    assert!(rest.radii().top_left.0 >= 8.0);
    let shadows = |s: &ViewStyle| s.effects.as_ref().map_or(0, |e| e.box_shadows.len());
    // `sm`: two layers.
    assert_eq!(shadows(&rest), 2, "{rest:?}");
    app.hover("label:Volume").unwrap();
    app.run_for(Duration::from_millis(300)).unwrap();
    let hovered = thumb_style(&app, &styles);
    // The ring joins the shadow.
    assert_eq!(shadows(&hovered), 3, "{hovered:?}");
}

/// A slider outside and inside a scope that changes `slider-range`.
#[component]
fn Scoped() -> impl IntoElement {
    view()
        .w(px(200.0))
        .child(
            slider()
                .default_value(SliderValue::Single(50.0))
                .accessibility_label("Outside"),
        )
        .child(
            theme_scope()
                .tokens(ScopeTokens::new().set("slider-range", "#2563eb"))
                .child(
                    slider()
                        .default_value(SliderValue::Single(50.0))
                        .accessibility_label("Inside"),
                ),
        )
}

#[test]
fn a_scope_slider_range_reaches_the_range() {
    let app = start::<Scoped>(true);
    assert_eq!(
        fill(&app, &range(&app, "label:Inside")),
        common::css("#2563eb")
    );
    assert_eq!(
        fill(&app, &range(&app, "label:Outside")),
        light(SLIDER_RANGE)
    );
}

/// TSX apps' declarations name the value type (`nsh/base`'s, declared by
/// this library too) and the slider's props.
#[test]
fn tsx_gets_the_value_type() {
    let registry = nsh_longbridge::registry();
    assert_eq!(registry.undocumented(), Vec::<String>::new());
    let ts = nsh_core::element::declarations(&registry, &[]);
    let start = ts
        .find("export interface SliderProps")
        .expect("SliderProps");
    let props = &ts[start..start + ts[start..].find("\n}").unwrap()];
    for wanted in [
        "value?: SliderValue | null;",
        "axis?:",
        "onChangeEnd?:",
        "thumbClass?:",
    ] {
        assert!(props.contains(wanted), "{wanted} in:\n{props}");
    }
    assert!(ts.contains("SliderValue ="), "{ts}");
}
