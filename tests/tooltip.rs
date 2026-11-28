//! `Tooltip` in a headless engine: taken as the element's hint layer from
//! Rust and from TSX (its delay, placement and look, not a core default
//! around it), its colors and corners, a scope reaching it, `Kbd` inside
//! it, and the events passed through.

mod common;

use std::time::Duration;

use common::{length, radius, watch_styles};

use nsh_core::{Size as Viewport, platform::Os, prelude::*};
use nsh_engine::EngineConfig;
use nsh_longbridge::{
    ScopeTokens, ThemeAppearance, TooltipEvent, button,
    components::kbd::{KBD_RADIUS, KBD_TOOLTIP_RADIUS},
    kbd,
    theme::semantic::RADIUS_SM,
    theme_scope, tooltip,
};
use nsh_testing::Headless;

fn config(desktop: bool, mut config: EngineConfig) -> EngineConfig {
    config.viewport = Viewport::new(px(800.0), px(600.0));
    config.clock = nsh_engine::Clock::manual(std::time::SystemTime::now());
    config.environment.os = if desktop { Os::MacOs } else { Os::Ios };
    config
}

fn start<C: Component<Props = ()>>() -> Headless {
    let mut app = Headless::start(config(
        true,
        EngineConfig::new(nsh_testing::windowed_root::<C>()),
    ))
    .unwrap();
    app.first_render().unwrap();
    app
}

fn start_tsx(entry: &str) -> Headless {
    let sets = vec![nsh_base::registry(), nsh_longbridge::registry()];
    let path = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join(entry);
    let bundle = nsh_testing::compile_tsx(&path, &sets).unwrap();
    let mut app = Headless::start(config(
        true,
        EngineConfig {
            ui_sets: sets,
            ..EngineConfig::new(bundle)
        },
    ))
    .unwrap();
    app.first_render().unwrap();
    app
}

fn bounds(app: &Headless, selector: &str) -> nsh_core::geometry::Bounds<Points> {
    app.screen()
        .layout(selector)
        .unwrap()
        .unwrap_or_else(|| panic!("no {selector}"))
        .bounds
}

fn fill(app: &Headless, selector: &str) -> Option<u32> {
    app.screen()
        .view_style(selector)
        .unwrap()
        .unwrap_or_else(|| panic!("no {selector}"))
        .background_color
}

/// What the library resolves `token` to here, in the light appearance.
fn light(token: nsh_longbridge::theme::Token<nsh_longbridge::theme::kind::Color>) -> u32 {
    let tokens = nsh_base::Theme::default().into_tokens();
    let c = nsh_longbridge::theme::resolve_color_in(&tokens, &[], true, token);
    nsh_core::Color::from(c.light).0
}

/// A trigger with a tooltip whose `delay` is 300, one with the default
/// delay, and a place away from both.
#[component]
fn Triggers() -> impl IntoElement {
    view()
        .pt(px(100.0))
        .pl(px(40.0))
        .child(
            view()
                .test_id("trigger")
                .w(px(80.0))
                .h(px(30.0))
                .tooltip(
                    tooltip()
                        .test_id("bubble")
                        .delay(300.0)
                        .child("Tip")
                        .child(kbd().test_id("kbd").child("K")),
                )
                .child(text("Trigger")),
        )
        .child(
            view()
                .test_id("plain")
                .mt(px(40.0))
                .w(px(80.0))
                .h(px(30.0))
                .tooltip(tooltip().child("Plain"))
                .child(text("Default delay")),
        )
        .child(view().test_id("away").h(px(200.0)).child(text("Away")))
        .child(kbd().test_id("outside").child("Esc"))
}

/// The tooltip shows after its own delay (not the core's 500), above its
/// element, with the library's look.
fn shows_as_the_hint_layer(mut app: Headless) {
    app.hover("id:trigger").unwrap();
    app.run_for(Duration::from_millis(250)).unwrap();
    assert!(!app.shows("Tip"), "not before its 300 ms");
    app.run_for(Duration::from_millis(100)).unwrap();
    assert!(
        app.shows("Tip"),
        "shown after its 300 ms, before the core's 500"
    );
    // Its entrance (150 ms) ran.
    app.run_for(Duration::from_millis(200)).unwrap();
    let bubble = bounds(&app, "id:bubble");
    let trigger = bounds(&app, "id:trigger");
    let gap = trigger.origin.y.0 - (bubble.origin.y.0 + bubble.size.height.0);
    assert!(
        (gap - 4.0).abs() < 0.5,
        "above the element, `tooltip-offset` (4) away: {bubble:?} over {trigger:?}"
    );
    // One line: 16 + 6 × 2.
    assert_eq!(bubble.size.height.0, 28.0);
    assert_eq!(
        fill(&app, "id:bubble"),
        Some(light(nsh_longbridge::components::tooltip::TOOLTIP))
    );
    // A `Kbd` inside takes the tooltip's colors, one outside its own.
    assert_eq!(
        fill(&app, "id:kbd"),
        Some(light(nsh_longbridge::components::kbd::KBD_TOOLTIP))
    );
    assert_eq!(
        fill(&app, "id:outside"),
        Some(light(nsh_longbridge::components::kbd::KBD))
    );
    // Leaving hides it.
    app.hover("id:away").unwrap();
    app.run_for(Duration::from_millis(400)).unwrap();
    assert!(!app.shows("Tip"));
    // Without `delay`, `tooltip-delay` (500).
    app.hover("id:plain").unwrap();
    app.run_for(Duration::from_millis(400)).unwrap();
    assert!(!app.shows("Plain"), "not before 500 ms");
    app.run_for(Duration::from_millis(300)).unwrap();
    assert!(app.shows("Plain"));
}

#[test]
fn from_rust_the_tooltip_is_the_hint_layer() {
    shows_as_the_hint_layer(start::<Triggers>());
}

#[test]
fn from_tsx_the_tooltip_is_the_hint_layer() {
    shows_as_the_hint_layer(start_tsx("tests/tsx/tooltip.tsx"));
}

/// The bubble takes `radius-sm`, under a capsule's; a `Kbd` inside it
/// takes `kbd-tooltip-radius` (2), concentric in the bubble's padding, and
/// one outside `kbd-radius`.
#[test]
fn the_bubble_and_a_kbd_inside_it_take_their_radii() {
    let mut app = Headless::start(config(
        true,
        EngineConfig::new(nsh_testing::windowed_root::<Triggers>()),
    ))
    .unwrap();
    let styles = watch_styles(&mut app);
    app.first_render().unwrap();
    app.hover("id:trigger").unwrap();
    app.run_for(Duration::from_millis(600)).unwrap();
    assert!(app.shows("Tip"));
    let radius = |selector: &str| radius(&app, &styles, selector);
    assert_eq!(radius("id:bubble"), length(RADIUS_SM));
    assert_eq!(radius("id:bubble"), 6.0);
    assert_eq!(radius("id:kbd"), length(KBD_TOOLTIP_RADIUS));
    assert_eq!(radius("id:kbd"), 2.0);
    assert_eq!(radius("id:outside"), length(KBD_RADIUS));
    assert_eq!(radius("id:outside"), 6.0);
}

/// A button with a tooltip in a dark scope that also changes `foreground`,
/// and one outside.
#[component]
fn ScopedTips() -> impl IntoElement {
    view()
        .pt(px(100.0))
        .pl(px(40.0))
        .gap(px(40.0))
        .child(
            view()
                .test_id("outside")
                .self_start()
                .tooltip(tooltip().test_id("outside-tip").delay(0.0).child("Outside"))
                .child(button().child("Out")),
        )
        .child(
            theme_scope()
                .tokens(ScopeTokens::new().set("foreground", "#2563eb"))
                .appearance(ThemeAppearance::Dark)
                .child(
                    view()
                        .test_id("inside")
                        .self_start()
                        .tooltip(tooltip().test_id("inside-tip").delay(0.0).child("Inside"))
                        .child(button().child("In")),
                ),
        )
}

#[test]
fn the_tooltip_reads_its_triggers_scope() {
    let mut app = start::<ScopedTips>();
    app.hover("id:inside").unwrap();
    app.run_for(Duration::from_millis(300)).unwrap();
    assert!(app.shows("Inside"));
    let blue = nsh_core::Color::parse("#2563eb").unwrap().0;
    assert_eq!(fill(&app, "id:inside-tip"), Some(blue));
    app.hover("id:outside").unwrap();
    app.run_for(Duration::from_millis(400)).unwrap();
    assert!(app.shows("Outside"));
    assert_ne!(fill(&app, "id:outside-tip"), Some(blue));
}

/// A disabled button in a view with a tooltip that writes down its open changes.
#[component]
fn Events() -> impl IntoElement {
    let changes = signal(Vec::<String>::new());
    let opened = signal(0u32);
    let closed = signal(0u32);
    view()
        .pt(px(100.0))
        .pl(px(40.0))
        .child(
            // A Rust component takes no `tooltip` (it is not `Interactive`):
            // a view around it does.
            view()
                .test_id("trigger")
                .self_start()
                .tooltip(
                    tooltip()
                        .delay(0.0)
                        .on_open_change(move |e: &TooltipEvent| {
                            if let TooltipEvent::OpenChange(open, reason) = e {
                                let change = format!("{open}:{reason:?}");
                                changes.update(|c| c.push(change));
                            }
                        })
                        .on_open(move || opened.update(|n| *n += 1))
                        .on_close(move || closed.update(|n| *n += 1))
                        .child("Why it is off"),
                )
                .child(button().disabled(true).child("Off")),
        )
        .child(view().test_id("away").h(px(200.0)).child(text("Away")))
        .child(text(move || {
            format!(
                "changes:{} opened:{} closed:{}",
                changes.get().join(","),
                opened.get(),
                closed.get()
            )
        }))
}

#[test]
fn a_disabled_buttons_tooltip_shows_and_reports() {
    let mut app = start::<Events>();
    app.hover("id:trigger").unwrap();
    app.run_for(Duration::from_millis(100)).unwrap();
    assert!(
        app.shows("Why it is off"),
        "a disabled button's tooltip shows"
    );
    app.wait_for("text:changes:true:Hover opened:1 closed:0")
        .unwrap();
    app.hover("id:away").unwrap();
    app.run_for(Duration::from_millis(400)).unwrap();
    assert!(!app.shows("Why it is off"));
    app.wait_for("text:changes:true:Hover,false:Hover opened:1 closed:1")
        .unwrap();
}

/// From TSX `tooltip` is a host prop of every component: the library's
/// `Button`, disabled, takes one directly.
#[test]
fn from_tsx_a_disabled_button_takes_a_tooltip() {
    let mut app = start_tsx("tests/tsx/tooltip.tsx");
    app.hover("id:button").unwrap();
    app.run_for(Duration::from_millis(100)).unwrap();
    assert!(app.shows("On a button"));
}

/// A shown tooltip does not become its trigger's accessibility description
/// in NSH 0.7.0 (the spec's NSH gap 2): triggers carry their own name and
/// hint. When this fails, NSH reads tooltips: update the spec.
#[test]
fn a_tooltip_is_not_its_triggers_description() {
    let mut app = start::<Triggers>();
    app.hover("id:trigger").unwrap();
    app.run_for(Duration::from_millis(500)).unwrap();
    assert!(app.shows("Tip"));
    let screen = app.screen();
    assert_eq!(
        screen.accessibility_described_by("id:trigger").unwrap(),
        None
    );
    assert_eq!(
        screen.accessibility_description("id:trigger").unwrap(),
        None
    );
}

const LONG: &str = "Shows the full text of a label cut short, wrapping at the tooltip's maximum width rather than running out of the window.";

/// A truncated label whose tooltip, shown, holds its full text.
#[component]
fn LongTip() -> impl IntoElement {
    view().pt(px(200.0)).pl(px(200.0)).child(
        view().w(px(192.0)).child(
            text(LONG)
                .test_id("trigger")
                .number_of_lines(1u32)
                .whitespace_nowrap()
                .tooltip(tooltip().test_id("bubble").open(true).child(LONG)),
        ),
    )
}

/// Long text wraps inside `tooltip-max-width` (320) instead of running out
/// of the bubble, though the truncated trigger it belongs to does not wrap.
#[test]
fn long_text_wraps_at_the_maximum_width() {
    let mut app = start::<LongTip>();
    app.run_for(Duration::from_millis(300)).unwrap();
    let bubble = bounds(&app, "id:bubble");
    assert!(bubble.size.width.0 <= 320.0, "{bubble:?}");
    assert!(
        bubble.size.height.0 > 28.0,
        "more than one line: {bubble:?}"
    );
}

/// A view with bold, underlined, uppercase, spaced text (as a button's
/// root sets its label's) and a tooltip, shown.
#[component]
fn StyledTrigger() -> impl IntoElement {
    view().pt(px(200.0)).pl(px(200.0)).child(
        view()
            .self_start()
            .font_weight(FontWeight::BOLD)
            .underline()
            .uppercase()
            .letter_spacing(px(2.0))
            .tooltip(tooltip().open(true).child("Full title"))
            .child(text("Title")),
    )
}

/// The bubble sits in its trigger's tree, yet takes none of the trigger's
/// text styles: a bubble reads the same over any trigger.

#[test]
fn the_tooltip_takes_none_of_its_triggers_text_styles() {
    let mut app = start::<StyledTrigger>();
    app.run_for(Duration::from_millis(300)).unwrap();
    let style = app.screen().span_style("text:Full title").unwrap();
    assert_eq!(style.font_weight, FontWeight::NORMAL.0);
    assert_eq!(style.decoration, Default::default());
    assert_eq!(style.transform, Default::default());
    assert_eq!(style.letter_spacing.0, 0.0);
}
