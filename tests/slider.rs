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
    ScopeTokens, SliderBubble, SliderFormat, SliderMark, SliderScale, SliderValue,
    components::slider::{
        SLIDER_HISTOGRAM_BAR, SLIDER_HISTOGRAM_BAR_ACTIVE, SLIDER_MARK, SLIDER_MARK_ACTIVE,
        SLIDER_RANGE, SLIDER_THUMB, SLIDER_TRACK,
    },
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

/// The root of a slider without a histogram, marks or bubble: `nsh/base`'s
/// slider (around the press area), as it always was.
fn root(app: &Headless, thumb: &str) -> String {
    format!("view:{}", plain_root_id(app, thumb))
}

fn plain_root_id(app: &Headless, thumb: &str) -> NodeId {
    let rail = parent(app, app.screen().id(thumb).unwrap());
    parent(app, parent(app, rail))
}

/// The root of a slider with a histogram, marks or a bubble: the library's
/// column around the row that holds `nsh/base`'s slider.
fn root_id(app: &Headless, thumb: &str) -> NodeId {
    parent(app, parent(app, plain_root_id(app, thumb)))
}

#[test]
fn without_its_parts_a_slider_is_nsh_bases_slider_alone() {
    let app = start::<Sliders>(true);
    for thumb in ["label:Volume", "label:Locked", "label:Brightness"] {
        let root = plain_root_id(&app, thumb);
        // Straight in the fixture's box, with nothing around it.
        let host = parent(&app, root);
        assert_eq!(children(&app, host), [root], "{thumb}");
        assert_eq!(children(&app, parent(&app, host)).len(), 5, "{thumb}");
    }
    // A vertical slider never takes the parts, so never the column.
    let app = start::<Upright>(true);
    let root = plain_root_id(&app, "label:Level");
    assert_eq!(children(&app, parent(&app, root)), [root]);
    assert_eq!(size(&app, &at(root)).1, 200.0);
}

#[test]
fn a_slider_with_a_part_is_wrapped_in_a_column() {
    let app = start::<Parts>(true);
    // A bubble alone: the row under the column, no histogram or marks.
    let root = root_id(&app, "label:Zoom");
    assert_eq!(children(&app, root).len(), 1);
    assert_eq!(size(&app, &at(root)).0, 200.0);
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
        "histogram?: (number)[];",
        "marks?: (SliderMark)[];",
        "bubble?:",
        "formatValue?: ((value: number) => string) | null;",
        "bubbleClass?:",
    ] {
        assert!(props.contains(wanted), "{wanted} in:\n{props}");
    }
    assert!(ts.contains("SliderValue ="), "{ts}");
    assert!(ts.contains("export type SliderMark = number | {"), "{ts}");
}

/// The views directly in `id`, in order.
fn children(app: &Headless, id: NodeId) -> Vec<NodeId> {
    app.screen()
        .layout_nodes()
        .iter()
        .filter(|n| n.parent == id)
        .map(|n| n.id)
        .collect()
}

fn at(id: NodeId) -> String {
    format!("view:{id}")
}

/// A price filter (a range with a histogram and marks), a bubble always
/// shown with the app's format, one shown on demand, a range's thumbs
/// close together, and marks on a logarithmic scale; each 200 wide.
#[component]
fn Parts() -> impl IntoElement {
    let price = signal((20.0, 60.0));
    let wide = |child: nsh_longbridge::SliderElement| view().w(px(200.0)).child(child);
    view()
        .flex_col()
        .items_start()
        .gap(px(40.0))
        .p(px(40.0))
        .child(wide(
            slider()
                .value(move || {
                    let (a, b) = price.get();
                    Some(SliderValue::Range(a, b))
                })
                .on_change(move |v: &SliderValue| price.set((v.start(), v.end())))
                // Five buckets, 40 wide each.
                .histogram(vec![1.0, 2.0, 4.0, 2.0, 0.0])
                .marks(vec![
                    SliderMark::new(0.0).label("0"),
                    50.0.into(),
                    SliderMark::new(100.0).label("100"),
                ])
                .start_accessibility_label("Minimum price")
                .accessibility_label("Maximum price"),
        ))
        .child(wide(
            slider()
                .default_value(SliderValue::Single(50.0))
                .bubble(SliderBubble::Always)
                .format_value(Some(SliderFormat::new(|v| format!("{v}%").into())))
                .accessibility_label("Opacity"),
        ))
        .child(wide(
            slider()
                .default_value(SliderValue::Single(30.0))
                .bubble(SliderBubble::Auto)
                .accessibility_label("Zoom"),
        ))
        .child(wide(
            slider()
                .default_value(SliderValue::Range(40.0, 45.0))
                .bubble(SliderBubble::Always)
                .start_accessibility_label("From")
                .accessibility_label("To"),
        ))
        .child(wide(
            slider()
                .min(1.0)
                .max(1000.0)
                .scale(SliderScale::Logarithmic)
                .default_value(SliderValue::Single(10.0))
                .marks(vec![10.0.into()])
                .accessibility_label("Gain"),
        ))
        .child(text(move || {
            let (a, b) = price.get();
            format!("price:{a}-{b}")
        }))
}

#[test]
fn the_histogram_sits_on_the_thumbs_and_its_bars_split_at_them() {
    let app = start::<Parts>(true);
    let root = root_id(&app, "label:Maximum price");
    let [histogram, row, marks] = children(&app, root)[..] else {
        panic!("histogram, row, marks: {:?}", children(&app, root));
    };
    let h = bounds(&app, &at(histogram));
    let r = bounds(&app, &at(row));
    let m = bounds(&app, &at(marks));
    assert_eq!((h.size.width.0, h.size.height.0), (200.0, 40.0));
    // With a pointer the thumb fills the press area: the bars end at its
    // top, the ticks start at its bottom.
    assert_eq!(h.origin.y + h.size.height, r.origin.y);
    assert_eq!(m.origin.y, r.origin.y + r.size.height);
    let [muted, window] = children(&app, histogram)[..] else {
        panic!("two copies of the bars");
    };
    // The active copy shows from the start thumb (20: 40 points) to the
    // end thumb (60: 120 points).
    let w = bounds(&app, &at(window));
    assert_eq!(w.origin.x - h.origin.x, px(40.0));
    assert_eq!(w.size.width, px(80.0));
    let slots = children(&app, muted);
    assert_eq!(slots.len(), 5);
    let bar = |slot: NodeId| children(&app, slot)[0];
    // Heights scale to the largest count (4); a 0 draws no bar.
    assert_eq!(size(&app, &at(bar(slots[2]))), (38.0, 40.0));
    assert_eq!(size(&app, &at(bar(slots[0]))).1, 10.0);
    let empty = bar(slots[4]);
    let nodes = app.screen().layout_nodes();
    let empty = nodes.iter().find(|n| n.id == empty).unwrap();
    assert_eq!(empty.bounds.size.height, px(0.0));
    // Each bucket a fifth of the track, inset by half the gap.
    assert_eq!(
        bounds(&app, &at(bar(slots[1]))).origin.x - h.origin.x,
        px(41.0)
    );
    assert_eq!(fill(&app, &at(bar(slots[0]))), light(SLIDER_HISTOGRAM_BAR));
    let active = children(&app, window)[0];
    // The active copy's third bar (40 to 80 along the window): the first
    // two are clipped away.
    let active_bar = children(&app, children(&app, active)[2])[0];
    assert_eq!(
        fill(&app, &at(active_bar)),
        light(SLIDER_HISTOGRAM_BAR_ACTIVE)
    );
    assert_eq!(light(SLIDER_HISTOGRAM_BAR_ACTIVE), light(SLIDER_RANGE));
    // Decorative: nothing of it reaches screen readers.
    assert!(!app.screen().matches("role:image").unwrap_or(false));
}

#[test]
fn a_drag_moves_the_split_with_the_thumb() {
    let mut app = start::<Parts>(true);
    app.drag(
        "label:Minimum price",
        Point::new(px(20.0), px(0.0)),
        DragOptions::default(),
    )
    .unwrap();
    app.wait_for("text:price:30-60").unwrap();
    let root = root_id(&app, "label:Maximum price");
    let histogram = children(&app, root)[0];
    let window = children(&app, histogram)[1];
    let h = bounds(&app, &at(histogram));
    let w = bounds(&app, &at(window));
    assert_eq!(w.origin.x - h.origin.x, px(60.0));
    assert_eq!(w.size.width, px(60.0));
}

#[test]
fn marks_tick_at_their_values_and_their_labels_stay_inside_the_ends() {
    let app = start::<Parts>(true);
    let root = root_id(&app, "label:Maximum price");
    let slider = bounds(&app, &at(root));
    let marks = children(&app, root)[2];
    let ticks: Vec<NodeId> = children(&app, marks)
        .into_iter()
        .map(|mark| children(&app, mark)[0])
        .collect();
    assert_eq!(ticks.len(), 3);
    // Centered on its value, as layout rounds a 1-wide tick to whole
    // points.
    let center = |id: NodeId| {
        let b = bounds(&app, &at(id));
        (b.origin.x + b.size.width / 2.0 - slider.origin.x).0
    };
    for (tick, x) in ticks.iter().zip([0.0, 100.0, 200.0]) {
        assert!((center(*tick) - x).abs() <= 0.5, "{} at {x}", center(*tick));
    }
    // In range (20 to 60): the middle tick.
    assert_eq!(fill(&app, &at(ticks[1])), light(SLIDER_MARK_ACTIVE));
    assert_eq!(fill(&app, &at(ticks[0])), light(SLIDER_MARK));
    // "0" starts at the start, "100" ends at the end: neither overflows.
    let zero = bounds(&app, "text:0");
    assert_eq!(zero.origin.x, slider.origin.x);
    let hundred = bounds(&app, "text:100");
    assert_eq!(
        hundred.origin.x + hundred.size.width,
        slider.origin.x + slider.size.width
    );
}

#[test]
fn marks_follow_a_logarithmic_scale_as_the_thumb_does() {
    let app = start::<Parts>(true);
    let root = root_id(&app, "label:Gain");
    let marks = children(&app, root)[1];
    let tick = children(&app, children(&app, marks)[0])[0];
    let t = bounds(&app, &at(tick));
    // 10 is a third of the way from 1 to 1000. A thumb is centered on its
    // value by a translation, which its layout box leaves out: its box
    // starts at its value.
    let tick_x = t.origin.x + t.size.width / 2.0;
    let thumb_x = bounds(&app, "label:Gain").origin.x;
    assert!((tick_x - thumb_x).0.abs() <= 0.5, "{tick_x:?} {thumb_x:?}");
}

/// The bubble (the text's box) of the text `label`.
fn bubble_style(app: &Headless, label: &str) -> f32 {
    let text = app.screen().id(&format!("text:{label}")).unwrap();
    let pill = parent(app, text);
    app.screen().view_style(&at(pill)).unwrap().unwrap().opacity
}

#[test]
fn an_always_shown_bubble_shows_the_apps_text_above_its_thumb() {
    let app = start::<Parts>(true);
    assert_eq!(bubble_style(&app, "50%"), 1.0);
    let text = app.screen().id("text:50%").unwrap();
    let pill = bounds(&app, &at(parent(&app, text)));
    let thumb = bounds(&app, "label:Opacity");
    // The thumb's layout box starts at its value (it is centered by a
    // translation).
    assert!(
        (pill.origin.x + pill.size.width / 2.0 - thumb.origin.x)
            .0
            .abs()
            < 0.01
    );
    // Above the thumb (its layout box starts at the rail's middle, 8 below
    // its top), `slider-bubble-offset` (8) away.
    assert_eq!(
        pill.origin.y + pill.size.height + px(8.0),
        thumb.origin.y - px(8.0)
    );
}

#[test]
fn an_auto_bubble_shows_while_the_pointer_is_on_the_thumb() {
    let mut app = start::<Parts>(true);
    assert_eq!(bubble_style(&app, "30"), 0.0);
    app.hover("label:Zoom").unwrap();
    app.run_for(Duration::from_millis(300)).unwrap();
    assert_eq!(bubble_style(&app, "30"), 1.0);
}

#[test]
fn a_ranges_close_thumbs_share_one_bubble() {
    let app = start::<Parts>(true);
    assert_eq!(bubble_style(&app, "40 – 45"), 1.0);
    assert_eq!(bubble_style(&app, "40"), 0.0);
    assert_eq!(bubble_style(&app, "45"), 0.0);
}

/// A vertical slider given a histogram and marks.
#[component]
fn Upright() -> impl IntoElement {
    view().h(px(200.0)).child(
        slider()
            .axis(Axis::Vertical)
            .histogram(vec![1.0, 2.0])
            .marks(vec![SliderMark::new(50.0).label("50")])
            .bubble(SliderBubble::Always)
            .accessibility_label("Level"),
    )
}

#[test]
fn a_vertical_slider_draws_no_histogram_marks_or_bubble() {
    let app = start::<Upright>(true);
    let root = plain_root_id(&app, "label:Level");
    assert_eq!(children(&app, root).len(), 1);
    assert!(!app.shows("50"));
}

#[test]
fn a_tsx_format_shows_in_the_bubble() {
    let mut app = common::start_tsx("tests/tsx/slider.tsx", true);
    app.wait_for("text:$250").unwrap();
    app.press_key("Tab").unwrap();
    app.press_key("ArrowRight").unwrap();
    app.wait_for("text:$260").unwrap();
}

/// One slider whose bubble shows on demand, and a place away from it.
#[component]
fn OnDemand() -> impl IntoElement {
    view()
        .flex_col()
        .items_start()
        .gap(px(40.0))
        .p(px(40.0))
        .child(
            view().w(px(200.0)).child(
                slider()
                    .default_value(SliderValue::Single(50.0))
                    .bubble(SliderBubble::Auto)
                    .accessibility_label("Zoom"),
            ),
        )
        .child(text("Away"))
}

#[test]
fn an_auto_bubble_shows_with_the_keyboard_focus() {
    let mut app = start::<OnDemand>(true);
    assert_eq!(bubble_style(&app, "50"), 0.0);
    app.press_key("Tab").unwrap();
    app.run_for(Duration::from_millis(300)).unwrap();
    assert_eq!(bubble_style(&app, "50"), 1.0);
    app.press_key("ArrowRight").unwrap();
    app.run_for(Duration::from_millis(300)).unwrap();
    assert_eq!(bubble_style(&app, "51"), 1.0);
}

#[test]
fn an_auto_bubble_goes_once_a_drag_is_released_away() {
    let mut app = start::<OnDemand>(true);
    app.drag(
        "label:Zoom",
        Point::new(px(20.0), px(0.0)),
        DragOptions::default(),
    )
    .unwrap();
    app.hover("text:Away").unwrap();
    app.run_for(Duration::from_millis(300)).unwrap();
    assert_eq!(bubble_style(&app, "60"), 0.0);
}

#[test]
fn the_histogram_and_marks_meet_the_thumbs_on_touch_screens() {
    let app = start::<Parts>(false);
    let root = root_id(&app, "label:Maximum price");
    let [histogram, row, marks] = children(&app, root)[..] else {
        panic!("histogram, row, marks");
    };
    let h = bounds(&app, &at(histogram));
    let m = bounds(&app, &at(marks));
    let thumb = bounds(&app, "label:Maximum price");
    // The press area is 44 tall, the thumb 20: the bars end at the thumb's
    // top, the ticks start at its bottom (its layout box starts at its
    // middle).
    assert_eq!(bounds(&app, &at(row)).size.height, px(44.0));
    assert_eq!(h.origin.y + h.size.height, thumb.origin.y - px(10.0));
    assert_eq!(m.origin.y, thumb.origin.y + px(10.0));
}

/// A range whose bubbles show on demand, one whose two values share a
/// bubble in the app's words, and marks too close for all their labels;
/// each 200 wide.
#[component]
fn Crowded() -> impl IntoElement {
    nsh_longbridge::set_translator(|label| {
        (label == nsh_longbridge::labels::Label::ValueRange).then(|| "{start} to {end}".into())
    });
    let wide = |child: nsh_longbridge::SliderElement| view().w(px(200.0)).child(child);
    view()
        .flex_col()
        .items_start()
        .gap(px(40.0))
        .p(px(40.0))
        .child(wide(
            slider()
                .default_value(SliderValue::Range(20.0, 80.0))
                .bubble(SliderBubble::Auto)
                .start_accessibility_label("From")
                .accessibility_label("Until"),
        ))
        .child(wide(
            slider()
                .default_value(SliderValue::Range(40.0, 45.0))
                .bubble(SliderBubble::Always)
                .start_accessibility_label("Low")
                .accessibility_label("High"),
        ))
        .child(wide(
            slider()
                .default_value(SliderValue::Single(50.0))
                .marks(
                    (0..=20)
                        .map(|i| {
                            let v = f64::from(i) * 5.0;
                            SliderMark::new(v).label(format!("{v}%"))
                        })
                        .collect::<Vec<_>>(),
                )
                .accessibility_label("Mix"),
        ))
}

#[test]
fn a_ranges_keyboard_bubble_is_the_focused_thumbs() {
    let mut app = start::<Crowded>(true);
    let settle = |app: &mut Headless| app.run_for(Duration::from_millis(300)).unwrap();
    app.press_key("Tab").unwrap();
    settle(&mut app);
    // Which thumb has the focus is known once a key moves it.
    assert_eq!(bubble_style(&app, "20"), 0.0);
    assert_eq!(bubble_style(&app, "80"), 0.0);
    app.press_key("ArrowRight").unwrap();
    settle(&mut app);
    assert_eq!(bubble_style(&app, "21"), 1.0);
    assert_eq!(bubble_style(&app, "80"), 0.0);
    app.press_key("Tab").unwrap();
    app.press_key("ArrowRight").unwrap();
    settle(&mut app);
    assert_eq!(bubble_style(&app, "81"), 1.0);
    assert_eq!(bubble_style(&app, "21"), 0.0);
}

#[test]
fn a_joined_bubble_says_the_range_in_the_apps_words() {
    let app = start::<Crowded>(true);
    assert_eq!(bubble_style(&app, "40 to 45"), 1.0);
}

#[test]
fn crowded_mark_labels_keep_the_ends_and_drop_the_ones_between_that_touch() {
    let app = start::<Crowded>(true);
    let shown = |label: &str| {
        let id = app.screen().id(&format!("text:{label}")).unwrap();
        app.screen().view_style(&at(id)).unwrap().unwrap().opacity == 1.0
    };
    let labels: Vec<String> = (0..=20).map(|i| format!("{}%", i * 5)).collect();
    assert!(shown("0%") && shown("100%"));
    let drawn: Vec<_> = labels
        .iter()
        .filter(|l| shown(l))
        .map(|l| bounds(&app, &format!("text:{l}")))
        .collect();
    assert!(
        drawn.len() > 2 && drawn.len() < labels.len(),
        "{}",
        drawn.len()
    );
    // Left to right, each at least the gap (4) past the one before.
    for pair in drawn.windows(2) {
        assert!(
            pair[1].origin.x >= pair[0].origin.x + pair[0].size.width + px(4.0),
            "{pair:?}"
        );
    }
}
