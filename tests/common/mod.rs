//! What the component tests share: a headless engine on a desktop or a
//! phone, and reading a view's size and fill.

#![allow(dead_code, unreachable_pub)]

use nsh_core::{Size as Viewport, platform::Os, prelude::*};
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

/// A CSS color as a fill reads.
pub fn css(color: &str) -> Option<u32> {
    Some(nsh_core::Color::parse(color).unwrap().0)
}
