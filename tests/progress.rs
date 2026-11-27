//! `Progress` in a headless engine: what screen readers get, the track's
//! size, the indicator's width (moved, not resized), the moving segment
//! and its still form with reduced motion, the app's classes and a scope's
//! tokens reaching the look.

mod common;

use std::time::Duration;

use common::{bounds, light, start};
use nsh_core::{NodeId, Size as Viewport, platform::Os, prelude::*};
use nsh_engine::EngineConfig;
use nsh_longbridge::{
    ScopeTokens, components::progress::PROGRESS_INDICATOR, progress, theme_scope,
};
use nsh_testing::Headless;

/// The bar's track and indicator: its first child and that one's.
fn parts(app: &Headless, label: &str) -> (NodeId, NodeId) {
    let screen = app.screen();
    let root = screen.id(&format!("role:progressbar[{label}]")).unwrap();
    let nodes = screen.layout_nodes();
    let child = |of: NodeId| {
        nodes
            .iter()
            .find(|n| n.parent == of)
            .unwrap_or_else(|| panic!("nothing in {of:?}"))
            .id
    };
    let track = child(root);
    (track, child(track))
}

fn width_height(app: &Headless, node: NodeId) -> (f32, f32) {
    let b = bounds(app, &format!("view:{node}"));
    (b.size.width.0, b.size.height.0)
}

/// Bars: one at 30 (60 after a press), one indeterminate, a thin one in
/// the app's color, one in a scope.
#[component]
fn Bars() -> impl IntoElement {
    let value = signal(30.0_f32);
    view()
        .w(px(400.0))
        .child(progress().value(value).accessibility_label("Upload"))
        .child(
            progress()
                .indeterminate(true)
                .accessibility_label("Connecting"),
        )
        .child(
            progress()
                .value(90.0)
                .track_class(Some("h-1".into()))
                .indicator_class(Some("bg-[#16a34a]".into()))
                .accessibility_label("Storage"),
        )
        .child(
            theme_scope()
                .tokens(ScopeTokens::new().set("progress-indicator", "#2563eb"))
                .child(progress().value(50.0).accessibility_label("Scoped")),
        )
        .child(
            view()
                .test_id("more")
                .w(px(40.0))
                .h(px(40.0))
                .on_press(move || value.set(60.0)),
        )
}

#[test]
fn screen_readers_get_one_named_progress_bar() {
    let app = start::<Bars>(true);
    let screen = app.screen();
    // The range and value are `nsh/base`'s (its own tests): the bar is one
    // element, named.
    let upload = screen
        .accessibility(Some("role:progressbar[Upload]"))
        .unwrap()
        .unwrap();
    assert!(upload.children.is_empty());
    assert!(screen.matches("role:progressbar[Connecting]").unwrap());
}

/// ★ NSH gaps 2: the indicator spans the track whatever the value (it is
/// moved back by what it leaves, which transitions; a width would change at
/// once), and a new value leaves its width as it is.
#[test]
fn the_indicator_spans_the_track_and_is_moved() {
    let mut app = start::<Bars>(true);
    let (track, indicator) = parts(&app, "Upload");
    assert_eq!(width_height(&app, track), (400.0, 8.0));
    assert_eq!(width_height(&app, indicator), (400.0, 8.0));
    app.press("id:more").unwrap();
    app.run_for(Duration::from_millis(300)).unwrap();
    assert_eq!(width_height(&app, indicator), (400.0, 8.0));
}

#[test]
fn the_apps_classes_and_a_scope_reach_the_look() {
    let app = start::<Bars>(true);
    let (track, indicator) = parts(&app, "Storage");
    assert_eq!(width_height(&app, track).1, 4.0);
    let fill = |node: NodeId| {
        app.screen()
            .view_style(&format!("view:{node}"))
            .unwrap()
            .unwrap()
            .background_color
    };
    assert_eq!(fill(indicator), common::css("#16a34a"));
    assert_eq!(fill(parts(&app, "Scoped").1), common::css("#2563eb"));
    assert_eq!(fill(parts(&app, "Upload").1), light(PROGRESS_INDICATOR));
}

/// ★ NSH gaps 1: the indeterminate segment is 40 % of the track and runs
/// on a timer while shown.
#[test]
fn the_indeterminate_segment_is_a_share_of_the_track() {
    let mut app = start::<Bars>(true);
    let (_, segment) = parts(&app, "Connecting");
    assert_eq!(width_height(&app, segment), (160.0, 8.0));
    app.run_for(Duration::from_millis(3200)).unwrap();
    assert_eq!(width_height(&app, segment), (160.0, 8.0));
}

/// With reduced motion the indeterminate bar is still: the whole track at
/// half opacity.
#[test]
fn with_reduced_motion_the_indeterminate_bar_is_still() {
    let mut config = EngineConfig {
        viewport: Viewport::new(px(800.0), px(600.0)),
        clock: nsh_engine::Clock::manual(std::time::SystemTime::now()),
        ..EngineConfig::new(nsh_testing::windowed_root::<Bars>())
    };
    config.environment.os = Os::MacOs;
    config.environment.reduce_motion = true;
    let mut app = Headless::start(config).unwrap();
    app.first_render().unwrap();
    let (_, segment) = parts(&app, "Connecting");
    assert_eq!(width_height(&app, segment), (400.0, 8.0));
    let opacity = app
        .screen()
        .view_style(&format!("view:{segment}"))
        .unwrap()
        .unwrap()
        .opacity;
    assert_eq!(opacity, 0.5);
}
