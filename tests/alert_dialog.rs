//! `AlertDialog` in a headless engine: an `alertdialog` named by its title,
//! opened with the focus on Cancel (else its action); the overlay does not
//! close it; Cancel, the action and Escape close it with their reasons;
//! size `sm` is 320 wide with two equal buttons; the media sits above the
//! title or beside it, its icon 32 square.

mod common;

use std::time::Duration;

use common::{bounds, light, start};
use nsh_core::prelude::*;
use nsh_longbridge::{
    AlertDialogEvent, ButtonVariant, IconName, Size, alert_dialog, alert_dialog_action,
    alert_dialog_cancel, alert_dialog_content, alert_dialog_description, alert_dialog_footer,
    alert_dialog_header, alert_dialog_media, alert_dialog_title, button,
    components::alert_dialog::ALERT_DIALOG_MEDIA, icon,
};
use nsh_testing::Headless;

const QUESTION: &str = "Are you absolutely sure?";

/// An alert opened by its trigger, writing down its events and its
/// action's presses.
fn logged(size: Size, cancel: bool, media: bool) -> impl IntoElement {
    let log = signal(Vec::<String>::new());
    let header = alert_dialog_header().test_id("header");
    let header = if media {
        header.child(
            alert_dialog_media()
                .test_id("media")
                .child(icon().test_id("icon").name(IconName::Trash2)),
        )
    } else {
        header
    };
    let footer = alert_dialog_footer();
    let footer = if cancel {
        footer.child(alert_dialog_cancel().child("Cancel"))
    } else {
        footer
    };
    view()
        .pt(px(40.0))
        .pl(px(40.0))
        .items_start()
        .child(
            alert_dialog()
                .trigger(button().variant(ButtonVariant::Outline).child("Delete"))
                .on_open_change(move |e: &AlertDialogEvent| {
                    if let AlertDialogEvent::OpenChange(open, reason) = e {
                        let entry = format!("change:{open}:{reason:?}");
                        log.update(|l| l.push(entry));
                    }
                })
                .on_open(move || log.update(|l| l.push("open".into())))
                .on_close(move || log.update(|l| l.push("close".into())))
                .on_cancel(move || log.update(|l| l.push("cancel".into())))
                .child(
                    alert_dialog_content()
                        .size(size)
                        .child(
                            header
                                .child(alert_dialog_title().test_id("title").child(QUESTION))
                                .child(
                                    alert_dialog_description()
                                        .test_id("description")
                                        .child("This action cannot be undone."),
                                ),
                        )
                        .child(
                            footer.child(
                                alert_dialog_action()
                                    .variant(ButtonVariant::Destructive)
                                    .on_press(move || log.update(|l| l.push("press".into())))
                                    .child("Continue"),
                            ),
                        ),
                ),
        )
        .child(text(move || format!("log:{}", log.get().join(","))))
}

#[component]
fn Basic() -> impl IntoElement {
    logged(Size::Md, true, false)
}

#[component]
fn Small() -> impl IntoElement {
    logged(Size::Sm, true, true)
}

#[component]
fn WithMedia() -> impl IntoElement {
    logged(Size::Md, true, true)
}

#[component]
fn ActionOnly() -> impl IntoElement {
    logged(Size::Md, false, false)
}

fn open(app: &mut Headless) {
    app.press("role:button[Delete]").unwrap();
    app.run_for(Duration::from_millis(400)).unwrap();
    assert!(app.shows(QUESTION));
}

fn gone(app: &mut Headless) {
    app.run_for(Duration::from_millis(400)).unwrap();
    assert!(!app.shows(QUESTION));
}

/// The panel: the view around its header (`id:header`).
fn panel(app: &Headless) -> nsh_core::geometry::Bounds<Points> {
    let header = app.screen().id("id:header").unwrap();
    let nodes = app.screen().layout_nodes();
    let parent = nodes.iter().find(|n| n.id == header).unwrap().parent;
    nodes.iter().find(|n| n.id == parent).unwrap().bounds
}

#[test]
fn it_is_an_alertdialog_named_by_its_title() {
    let mut app = start::<Basic>(true);
    open(&mut app);
    let screen = app.screen();
    assert!(
        screen
            .matches(&format!("role:alertdialog[{QUESTION}]"))
            .unwrap()
    );
    assert!(
        screen
            .accessibility_described_by(&format!("role:alertdialog[{QUESTION}]"))
            .unwrap()
            .is_some()
    );
    assert!(
        !screen.matches("role:button[Close]").unwrap(),
        "no close button"
    );
}

/// The focus starts on Cancel: Return right away cancels.
#[test]
fn it_opens_with_the_focus_on_cancel() {
    let mut app = start::<Basic>(true);
    open(&mut app);
    app.press_key("Enter").unwrap();
    gone(&mut app);
    app.wait_for("text:log:change:true:TriggerPress,open,change:false:Cancel,close")
        .unwrap();
}

/// Without a Cancel, the focus starts on the action.
#[test]
fn without_cancel_the_focus_is_on_the_action() {
    let mut app = start::<ActionOnly>(true);
    open(&mut app);
    app.press_key("Enter").unwrap();
    gone(&mut app);
    app.wait_for("text:log:change:true:TriggerPress,open,change:false:Select,press,close")
        .unwrap();
}

/// A press on the overlay is heard (`onCancel`) but does not close it.
#[test]
fn the_overlay_does_not_close_it() {
    let mut app = start::<Basic>(true);
    open(&mut app);
    let candidates: Vec<String> = app
        .screen()
        .layout_nodes()
        .iter()
        .filter(|n| n.bounds.size.width.0 == 800.0 && n.bounds.size.height.0 == 600.0)
        .map(|n| format!("view:{}", n.id))
        .collect();
    let pressed = candidates
        .iter()
        .rev()
        .any(|c| app.press_at(c, point(px(10.0), px(10.0))).is_ok());
    assert!(pressed);
    app.run_for(Duration::from_millis(400)).unwrap();
    assert!(app.shows(QUESTION), "still open");
    app.wait_for("text:log:change:true:TriggerPress,open,cancel")
        .unwrap();
}

#[test]
fn cancel_and_the_action_close_it_with_their_reasons() {
    let mut app = start::<Basic>(true);
    open(&mut app);
    app.press("role:button[Cancel]").unwrap();
    gone(&mut app);
    app.wait_for("text:log:change:true:TriggerPress,open,change:false:Cancel,close")
        .unwrap();
    open(&mut app);
    app.press("role:button[Continue]").unwrap();
    gone(&mut app);
    app.wait_for(
        "text:log:change:true:TriggerPress,open,change:false:Cancel,close,\
         change:true:TriggerPress,open,change:false:Select,press,close",
    )
    .unwrap();
}

#[test]
fn escape_closes_it() {
    let mut app = start::<Basic>(true);
    open(&mut app);
    app.press_key("Escape").unwrap();
    gone(&mut app);
    app.wait_for("text:log:change:true:TriggerPress,open,cancel,change:false:Cancel,close")
        .unwrap();
}

/// `md` on a wide window: as wide as a dialog, the header start-aligned,
/// the buttons in a row at the end.
#[test]
fn a_default_alert_is_a_dialogs_width() {
    let mut app = start::<Basic>(true);
    open(&mut app);
    let panel = panel(&app);
    assert_eq!(panel.size.width.0, 512.0);
    let title = bounds(&app, "id:title");
    assert_eq!(title.origin.x.0, panel.origin.x.0 + 25.0, "start-aligned");
    let cancel = bounds(&app, "role:button[Cancel]");
    let action = bounds(&app, "role:button[Continue]");
    assert_eq!(cancel.origin.y, action.origin.y);
    assert!(cancel.size.width.0 < 200.0, "content-sized: {cancel:?}");
}

/// `sm`: 320 wide, the header centered, two equal buttons side by side,
/// Cancel on the left; the media centered above the title.
#[test]
fn a_small_alert_is_narrow_with_two_equal_buttons() {
    let mut app = start::<Small>(true);
    open(&mut app);
    let panel = panel(&app);
    assert_eq!(panel.size.width.0, 320.0);
    let cancel = bounds(&app, "role:button[Cancel]");
    let action = bounds(&app, "role:button[Continue]");
    assert_eq!(cancel.origin.y, action.origin.y, "side by side");
    assert!(cancel.origin.x < action.origin.x, "Cancel first");
    assert_eq!(cancel.size.width, action.size.width, "equal");
    // 320 less the padding (24) and the edge (1) on each side, less the gap.
    assert_eq!(cancel.size.width.0, (320.0 - 50.0 - 8.0) / 2.0);
    let media = bounds(&app, "id:media");
    let title = bounds(&app, "id:title");
    let middle = panel.origin.x.0 + panel.size.width.0 / 2.0;
    assert!(
        (media.origin.x.0 + 32.0 - middle).abs() < 0.5,
        "centered: {media:?}"
    );
    assert!(
        media.origin.y.0 + 64.0 <= title.origin.y.0,
        "above the title"
    );
}

/// `md` with media on a wide window: the media on the left, across the
/// title's and the description's rows; its icon 32 square on `muted`.
#[test]
fn the_media_sits_beside_a_start_aligned_header() {
    let mut app = start::<WithMedia>(true);
    open(&mut app);
    let media = bounds(&app, "id:media");
    let title = bounds(&app, "id:title");
    let description = bounds(&app, "id:description");
    assert_eq!((media.size.width.0, media.size.height.0), (64.0, 64.0));
    assert_eq!(
        title.origin.x.0,
        media.origin.x.0 + 64.0 + 24.0,
        "beside it"
    );
    assert_eq!(description.origin.x, title.origin.x);
    assert_eq!(title.origin.y, media.origin.y, "from its top");
    assert!(description.origin.y > title.origin.y);
    let icon = bounds(&app, "id:icon");
    assert_eq!((icon.size.width.0, icon.size.height.0), (32.0, 32.0));
    assert_eq!(common::fill(&app, "id:media"), light(ALERT_DIALOG_MEDIA));
}

/// `md` on a phone: the header centered, the buttons stacked, the action
/// on top.
#[test]
fn on_a_narrow_window_the_buttons_stack() {
    let mut config = nsh_engine::EngineConfig {
        viewport: nsh_core::Size::new(px(390.0), px(700.0)),
        clock: nsh_engine::Clock::manual(std::time::SystemTime::now()),
        ..nsh_engine::EngineConfig::new(nsh_testing::windowed_root::<WithMedia>())
    };
    config.environment.os = nsh_core::platform::Os::Ios;
    let mut app = Headless::start(config).unwrap();
    app.first_render().unwrap();
    open(&mut app);
    let cancel = bounds(&app, "role:button[Cancel]");
    let action = bounds(&app, "role:button[Continue]");
    assert!(action.origin.y < cancel.origin.y, "the action on top");
    assert_eq!(cancel.size.width, action.size.width);
    let panel = panel(&app);
    let media = bounds(&app, "id:media");
    let middle = panel.origin.x.0 + panel.size.width.0 / 2.0;
    assert!(
        (media.origin.x.0 + 32.0 - middle).abs() < 0.5,
        "centered: {media:?}"
    );
}

/// `Icon` keeps its own size inside the media when it has one.
#[component]
fn SizedIcon() -> impl IntoElement {
    alert_dialog().default_open(true).child(
        alert_dialog_content().child(
            alert_dialog_header()
                .child(
                    alert_dialog_media()
                        .child(icon().test_id("icon").name(IconName::Info).size(Size::Sm)),
                )
                .child(alert_dialog_title().child("Sized")),
        ),
    )
}

#[test]
fn an_icon_with_its_own_size_keeps_it_in_the_media() {
    let mut app = start::<SizedIcon>(true);
    app.run_for(Duration::from_millis(400)).unwrap();
    let icon = bounds(&app, "id:icon");
    assert_eq!(icon.size.width.0, 14.0);
}
