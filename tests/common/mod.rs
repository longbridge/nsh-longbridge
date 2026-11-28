//! What the component tests share: a headless engine on a desktop or a
//! phone, reading a view's size, fill and corner radius, a token's value,
//! and the color a drawing strokes in the dark appearance.

#![allow(dead_code, unreachable_pub)]

use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};

use nsh_core::{
    Color, MountItem, NodeId, Size as Viewport, ViewStyle,
    canvas::{CanvasOp, Paint},
    platform::{ColorScheme, Os},
    prelude::*,
};
use nsh_engine::EngineConfig;
use nsh_longbridge::theme::{Token, kind};
use nsh_testing::Headless;

/// `C` on an 800 × 600 window of a Mac (`desktop`) or an iPhone, with a
/// clock the test moves.
pub fn start<C: Component<Props = ()>>(desktop: bool) -> Headless {
    let mut config = EngineConfig {
        viewport: Viewport::new(px(800.0), px(600.0)),
        clock: nsh_engine::Clock::manual(std::time::SystemTime::now()),
        ..EngineConfig::new(nsh_testing::windowed_root::<C>())
    };
    config.environment.os = if desktop { Os::MacOs } else { Os::Ios };
    let mut app = Headless::start(config).unwrap();
    app.first_render().unwrap();
    app
}

/// The TSX app at `entry` (from the crate's root), with `nsh/base` and
/// the library as its UI sets, as [`start`] makes it.
pub fn start_tsx(entry: &str, desktop: bool) -> Headless {
    let sets = vec![nsh_base::registry(), nsh_longbridge::registry()];
    let path = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join(entry);
    let bundle = nsh_testing::compile_tsx(&path, &sets).unwrap();
    let mut config = EngineConfig {
        viewport: Viewport::new(px(800.0), px(600.0)),
        clock: nsh_engine::Clock::manual(std::time::SystemTime::now()),
        ui_sets: sets,
        ..EngineConfig::new(bundle)
    };
    config.environment.os = if desktop { Os::MacOs } else { Os::Ios };
    let mut app = Headless::start(config).unwrap();
    app.first_render().unwrap();
    app
}

/// Where `selector` is.
pub fn bounds(app: &Headless, selector: &str) -> nsh_core::geometry::Bounds<Points> {
    app.screen()
        .layout(selector)
        .unwrap()
        .unwrap_or_else(|| panic!("no {selector}"))
        .bounds
}

/// The width and height of `selector`.
pub fn size(app: &Headless, selector: &str) -> (f32, f32) {
    let b = bounds(app, selector);
    (b.size.width.0, b.size.height.0)
}

/// The fill of `selector` (`None`: transparent).
pub fn fill(app: &Headless, selector: &str) -> Option<u32> {
    app.screen()
        .view_style(selector)
        .unwrap()
        .unwrap_or_else(|| panic!("no {selector}"))
        .background_color
}

/// What `token` resolves to with `nsh/base`'s default theme, in the light
/// appearance, as a fill reads.
pub fn light(token: Token<kind::Color>) -> Option<u32> {
    let tokens = nsh_base::Theme::default().into_tokens();
    let c = nsh_longbridge::theme::resolve_color_in(&tokens, &[], true, token);
    Some(nsh_core::Color::from(c.light).0).filter(|c| c & 0xff != 0)
}

/// What `token` resolves to with `nsh/base`'s default theme, with a
/// pointer.
pub fn length(token: Token<kind::Length>) -> f32 {
    let tokens = nsh_base::Theme::default().into_tokens();
    nsh_longbridge::theme::resolve_number_in(&tokens, &[], true, token)
}

/// Every view's style as the host last got it, by node.
pub type Styles = Arc<Mutex<HashMap<NodeId, ViewStyle>>>;

/// Keeps every style `app` sends from now on (call it before the first
/// render to have them all).
pub fn watch_styles(app: &mut Headless) -> Styles {
    let styles = Styles::default();
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

/// `C` as [`start`] makes it, with every style it sends kept from the
/// first frame.
pub fn start_styled<C: Component<Props = ()>>(desktop: bool) -> (Headless, Styles) {
    let mut config = EngineConfig {
        viewport: Viewport::new(px(800.0), px(600.0)),
        clock: nsh_engine::Clock::manual(std::time::SystemTime::now()),
        ..EngineConfig::new(nsh_testing::windowed_root::<C>())
    };
    config.environment.os = if desktop { Os::MacOs } else { Os::Ios };
    let mut app = Headless::start(config).unwrap();
    let styles = watch_styles(&mut app);
    app.first_render().unwrap();
    (app, styles)
}

/// The style the host last got for `selector`.
pub fn style(app: &Headless, styles: &Styles, selector: &str) -> ViewStyle {
    let id = app.screen().id(selector).unwrap();
    styles
        .lock()
        .unwrap()
        .get(&id)
        .cloned()
        .unwrap_or_else(|| panic!("no style for {selector}"))
}

/// The corner radius the host last got for `selector`.
pub fn radius(app: &Headless, styles: &Styles, selector: &str) -> f32 {
    style(app, styles, selector).border_radius.0
}

/// A CSS color as a fill reads.
pub fn css(color: &str) -> Option<u32> {
    Some(nsh_core::Color::parse(color).unwrap().0)
}

/// The color each drawing strokes in, by node.
pub type Strokes = Arc<Mutex<HashMap<NodeId, Option<Color>>>>;

/// `C` as [`start`] makes it on a desktop, in the dark appearance, with the
/// color each drawing strokes in (an svg's `currentColor`), as the host
/// gets it.
pub fn start_dark<C: Component<Props = ()>>() -> (Headless, Strokes) {
    let mut config = EngineConfig {
        viewport: Viewport::new(px(800.0), px(600.0)),
        clock: nsh_engine::Clock::manual(std::time::SystemTime::now()),
        ..EngineConfig::new(nsh_testing::windowed_root::<C>())
    };
    config.environment.os = Os::MacOs;
    config.environment.color_scheme = ColorScheme::Dark;
    let mut app = Headless::start(config).unwrap();
    let strokes = Strokes::default();
    let seen = strokes.clone();
    app.observe(move |_, batch| {
        for item in &batch.items {
            if let MountItem::Canvas { id, drawing } = item {
                let stroke = drawing.ops.iter().find_map(|op| match op {
                    CanvasOp::StrokeStyle(Paint::Color(c)) => Some(*c),
                    _ => None,
                });
                seen.lock().unwrap().insert(*id, stroke);
            }
        }
    });
    app.first_render().unwrap();
    (app, strokes)
}

/// The color the drawing at `selector` strokes in.
pub fn stroke(app: &Headless, strokes: &Strokes, selector: &str) -> Color {
    let node = app.screen().id(selector).unwrap();
    strokes.lock().unwrap()[&node].unwrap_or_else(|| panic!("{selector} strokes nothing"))
}

/// What `token` resolves to with `nsh/base`'s default theme, in the dark
/// appearance.
pub fn dark(token: Token<kind::Color>) -> Color {
    let tokens = nsh_base::Theme::default().into_tokens();
    let c = nsh_longbridge::theme::resolve_color_in(&tokens, &[], true, token);
    Color::from(c.dark)
}
