//! `Dialog` in a headless engine: its trigger, the overlay, Escape, the
//! "×" and `DialogClose` close it with their reasons; it is a `dialog` named
//! by its title and described by its description; a scope around it
//! reaches its panel; Escape closes a popover inside it before it; the
//! panel's width and its layout on narrow and wide windows; a controlled
//! dialog waits for its app; `dialog-animate: 0`.

mod common;

use std::time::Duration;

use common::{bounds, light, start};
use nsh_core::prelude::*;
use nsh_longbridge::{
    ButtonVariant, DialogEvent, ScopeTokens, ThemeAppearance, button, components::dialog::DIALOG,
    dialog, dialog_close, dialog_content, dialog_description, dialog_footer, dialog_header,
    dialog_title, popover, theme_scope,
};
use nsh_testing::Headless;

/// A dialog opened by its trigger, writing down its events.
#[component]
fn Basic() -> impl IntoElement {
    let log = signal(Vec::<String>::new());
    view()
        .pt(px(40.0))
        .pl(px(40.0))
        .items_start()
        .child(
            dialog()
                .trigger(
                    button()
                        .variant(ButtonVariant::Outline)
                        .child("Edit profile"),
                )
                .on_open_change(move |e: &DialogEvent| {
                    if let DialogEvent::OpenChange(open, reason) = e {
                        let entry = format!("change:{open}:{reason:?}");
                        log.update(|l| l.push(entry));
                    }
                })
                .on_open(move || log.update(|l| l.push("open".into())))
                .on_close(move || log.update(|l| l.push("close".into())))
                .on_cancel(move || log.update(|l| l.push("cancel".into())))
                .child(
                    dialog_content()
                        .child(
                            dialog_header()
                                .test_id("header")
                                .child(dialog_title().test_id("title").child("Edit profile"))
                                .child(
                                    dialog_description()
                                        .child("Make changes to your profile here."),
                                ),
                        )
                        .child(
                            dialog_footer()
                                .child(dialog_close().child("Cancel"))
                                .child(button().child("Save changes")),
                        ),
                ),
        )
        .child(text(move || format!("log:{}", log.get().join(","))))
}

/// Presses the overlay near the window's top left corner: the view as
/// large as the window there that hears presses.
fn press_overlay(app: &mut Headless) {
    let candidates: Vec<String> = app
        .screen()
        .layout_nodes()
        .iter()
        .filter(|n| n.bounds.size.width.0 == 800.0 && n.bounds.size.height.0 == 600.0)
        .map(|n| format!("view:{}", n.id))
        .collect();
    for candidate in candidates.iter().rev() {
        if app.press_at(candidate, point(px(10.0), px(10.0))).is_ok() {
            return;
        }
    }
    panic!("no overlay to press among {candidates:?}");
}

/// The panel: the view around its header (`id:header`).
fn panel(app: &Headless) -> nsh_core::geometry::Bounds<Points> {
    let header = app.screen().id("id:header").unwrap();
    let nodes = app.screen().layout_nodes();
    let parent = nodes.iter().find(|n| n.id == header).unwrap().parent;
    nodes.iter().find(|n| n.id == parent).unwrap().bounds
}

/// The panel's view, as a selector.
fn panel_selector(app: &Headless) -> String {
    let header = app.screen().id("id:header").unwrap();
    let nodes = app.screen().layout_nodes();
    format!(
        "view:{}",
        nodes.iter().find(|n| n.id == header).unwrap().parent
    )
}

fn open(app: &mut Headless) {
    app.press("role:button[Edit profile]").unwrap();
    app.run_for(Duration::from_millis(400)).unwrap();
    assert!(app.shows("Make changes to your profile here."));
}

fn gone(app: &mut Headless) {
    app.run_for(Duration::from_millis(400)).unwrap();
    assert!(!app.shows("Make changes to your profile here."));
}

#[test]
fn the_trigger_opens_it_and_escape_closes_it() {
    let mut app = start::<Basic>(true);
    assert!(!app.shows("Make changes to your profile here."));
    open(&mut app);
    app.wait_for("text:log:change:true:TriggerPress,open")
        .unwrap();
    app.press_key("Escape").unwrap();
    gone(&mut app);
    app.wait_for("text:log:change:true:TriggerPress,open,cancel,change:false:Cancel,close")
        .unwrap();
}

#[test]
fn dialog_close_closes_it_as_a_choice() {
    let mut app = start::<Basic>(true);
    open(&mut app);
    app.press("role:button[Cancel]").unwrap();
    gone(&mut app);
    app.wait_for("text:log:change:true:TriggerPress,open,change:false:Select,close")
        .unwrap();
}

#[test]
fn the_close_button_cancels() {
    let mut app = start::<Basic>(true);
    open(&mut app);
    app.press("role:button[Close]").unwrap();
    gone(&mut app);
    app.wait_for("text:log:change:true:TriggerPress,open,cancel,change:false:Cancel,close")
        .unwrap();
}

#[test]
fn a_press_on_the_overlay_closes_it() {
    let mut app = start::<Basic>(true);
    open(&mut app);
    press_overlay(&mut app);
    gone(&mut app);
    app.wait_for("text:log:change:true:TriggerPress,open,cancel,change:false:BackdropPress,close")
        .unwrap();
}

#[test]
fn screen_readers_get_a_named_described_dialog() {
    let mut app = start::<Basic>(true);
    open(&mut app);
    let screen = app.screen();
    assert!(
        screen.matches("role:dialog[Edit profile]").unwrap(),
        "named by its title"
    );
    assert!(screen.matches("role:header[Edit profile]").unwrap());
    assert!(
        screen
            .accessibility_described_by("role:dialog[Edit profile]")
            .unwrap()
            .is_some(),
        "described by its description"
    );
    let trigger = screen
        .accessibility_state("role:button[Edit profile]")
        .unwrap();
    assert_eq!(trigger.expanded, Some(true));
}

/// The panel is centered, `dialog-max-width` (512) wide on an 800 wide
/// window, with the `dialog` fill; the "×" is 16 square.
#[test]
fn the_panel_is_centered_and_512_wide() {
    let mut app = start::<Basic>(true);
    open(&mut app);
    let panel = panel(&app);
    assert_eq!(panel.size.width.0, 512.0, "{panel:?}");
    assert!(
        (panel.origin.x.0 - 144.0).abs() < 0.5,
        "centered: {panel:?}"
    );
    let middle = panel.origin.y.0 + panel.size.height.0 / 2.0;
    assert!((middle - 300.0).abs() < 1.0, "centered: {panel:?}");
    assert_eq!(common::fill(&app, &panel_selector(&app)), light(DIALOG));
    let close = bounds(&app, "role:button[Close]");
    assert_eq!((close.size.width.0, close.size.height.0), (16.0, 16.0));
    // 16 inside the panel's edge (1), as CSS's `top-4 right-4`.
    assert_eq!(close.origin.y.0 - panel.origin.y.0, 17.0);
    assert_eq!(
        panel.origin.x.0 + panel.size.width.0 - (close.origin.x.0 + 16.0),
        17.0
    );
}

/// A dialog in a dark scope that changes `dialog`: the scope reaches the
/// panel drawn in the modal.
#[component]
fn Scoped() -> impl IntoElement {
    view().items_start().child(
        theme_scope()
            .tokens(ScopeTokens::new().set("dialog", "#2563eb"))
            .appearance(ThemeAppearance::Dark)
            .child(
                dialog().default_open(true).child(
                    dialog_content().child(
                        dialog_header()
                            .test_id("header")
                            .child(dialog_title().child("Scoped")),
                    ),
                ),
            ),
    )
}

#[test]
fn a_scope_around_the_dialog_reaches_its_panel() {
    let mut app = start::<Scoped>(true);
    app.run_for(Duration::from_millis(400)).unwrap();
    assert!(app.shows("Scoped"));
    let blue = nsh_core::Color::parse("#2563eb").unwrap().0;
    assert_eq!(common::fill(&app, &panel_selector(&app)), Some(blue));
}

/// A popover inside an open dialog.
#[component]
fn Nested() -> impl IntoElement {
    view().child(
        dialog().default_open(true).child(
            dialog_content().child(dialog_title().child("Outer")).child(
                view().self_start().child(
                    popover()
                        .trigger(button().child("More"))
                        .child(text("Inner panel")),
                ),
            ),
        ),
    )
}

/// Escape goes to the topmost layer: the popover first, then the dialog.
#[test]
fn escape_closes_a_popover_in_the_dialog_first() {
    let mut app = start::<Nested>(true);
    app.run_for(Duration::from_millis(400)).unwrap();
    app.press("role:button[More]").unwrap();
    app.run_for(Duration::from_millis(300)).unwrap();
    assert!(app.shows("Inner panel"));
    app.press_key("Escape").unwrap();
    app.run_for(Duration::from_millis(400)).unwrap();
    assert!(!app.shows("Inner panel"), "the popover closed");
    assert!(app.shows("Outer"), "the dialog stays");
    app.press_key("Escape").unwrap();
    app.run_for(Duration::from_millis(400)).unwrap();
    assert!(!app.shows("Outer"), "the second Escape closes the dialog");
}

/// `C` on a window `width` × 700 of a Mac or an iPhone.
fn start_at<C: Component<Props = ()>>(width: f32, desktop: bool) -> Headless {
    let mut config = nsh_engine::EngineConfig {
        viewport: nsh_core::Size::new(px(width), px(700.0)),
        clock: nsh_engine::Clock::manual(std::time::SystemTime::now()),
        ..nsh_engine::EngineConfig::new(nsh_testing::windowed_root::<C>())
    };
    config.environment.os = if desktop {
        nsh_core::platform::Os::MacOs
    } else {
        nsh_core::platform::Os::Ios
    };
    let mut app = Headless::start(config).unwrap();
    app.first_render().unwrap();
    app
}

/// Narrower than 640 (a phone): the panel keeps 16 from the window's sides,
/// the header is centered and the footer a column, its first button at the
/// bottom and each as wide as the panel.
#[test]
fn a_narrow_window_stacks_the_footer() {
    let mut app = start_at::<Basic>(390.0, false);
    open(&mut app);
    let panel = panel(&app);
    assert_eq!(panel.origin.x.0, 16.0, "{panel:?}");
    assert_eq!(panel.size.width.0, 390.0 - 32.0);
    let cancel = bounds(&app, "role:button[Cancel]");
    let save = bounds(&app, "role:button[Save changes]");
    assert!(cancel.origin.y > save.origin.y, "Cancel below Save");
    // Full width inside the padding (24) and the edge (1).
    assert_eq!(cancel.size.width.0, panel.size.width.0 - 50.0);
    // Its header is centered.
    let title = bounds(&app, "id:title");
    let middle = title.origin.x.0 + title.size.width.0 / 2.0;
    assert!((middle - 195.0).abs() < 0.5, "centered: {title:?}");
}

#[test]
fn a_wide_window_puts_the_footer_in_a_row_at_the_end() {
    let mut app = start_at::<Basic>(800.0, true);
    open(&mut app);
    let panel = panel(&app);
    let cancel = bounds(&app, "role:button[Cancel]");
    let save = bounds(&app, "role:button[Save changes]");
    assert_eq!(cancel.origin.y, save.origin.y, "one row");
    assert!(cancel.origin.x < save.origin.x, "in order");
    let end = panel.origin.x.0 + panel.size.width.0 - 25.0;
    assert!(
        (save.origin.x.0 + save.size.width.0 - end).abs() < 0.5,
        "at the end"
    );
}

/// A controlled dialog whose app keeps it open on the first request.
#[component]
fn Controlled() -> impl IntoElement {
    let open = signal(true);
    let declined = signal(false);
    view()
        .child(text(move || format!("open:{}", open.get())))
        .child(
            dialog()
                .open(move || Some(open.get()))
                .on_open_change(move |e: &DialogEvent| {
                    if let DialogEvent::OpenChange(next, _) = e {
                        if !*next && !declined.get_untracked() {
                            declined.set(true);
                        } else {
                            open.set(*next);
                        }
                    }
                })
                .child(dialog_content().child(dialog_title().child("Kept"))),
        )
}

/// The app owns `open`: a request it declines leaves the dialog open; the
/// next, followed, closes it.
#[test]
fn a_controlled_dialog_waits_for_the_app() {
    let mut app = start::<Controlled>(true);
    app.run_for(Duration::from_millis(400)).unwrap();
    assert!(app.shows("Kept"));
    app.press_key("Escape").unwrap();
    app.run_for(Duration::from_millis(400)).unwrap();
    assert!(app.shows("Kept"), "declined: still open");
    // A headless step settles the app, its exit included.
    app.press_key("Escape").unwrap();
    assert!(!app.shows("Kept"));
    app.wait_for("text:open:false").unwrap();
}

/// `dialog-animate: 0` shows and removes it in one frame.
#[component]
fn Still() -> impl IntoElement {
    let open = signal(true);
    theme_scope()
        .tokens(ScopeTokens::new().set("dialog-animate", 0))
        .child(
            text("page")
                .test_id("page")
                .on_press(move || open.set(false)),
        )
        .child(
            dialog()
                .open(move || Some(open.get()))
                .on_open_change(move |e: &DialogEvent| {
                    if let DialogEvent::OpenChange(next, _) = e {
                        open.set(*next);
                    }
                })
                .child(dialog_content().child(dialog_title().child("Still"))),
        )
}

#[test]
fn without_motion_it_goes_at_once() {
    let mut app = start::<Still>(true);
    app.run_for(Duration::from_millis(16)).unwrap();
    assert!(app.shows("Still"));
    app.press_key("Escape").unwrap();
    app.run_for(Duration::from_millis(16)).unwrap();
    assert!(!app.shows("Still"));
}
