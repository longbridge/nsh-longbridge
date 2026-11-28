//! `Toaster` and `toast()` in a headless engine: a toast shows its title,
//! description and buttons, is an `alert` read as it shows, times out,
//! answers with the button that closed it, changes in place by `id`, keeps
//! `visibleToasts` mounted; `dismissToast`; the stack's place and width;
//! it shows above an open dialog, in the `Toaster`'s theme scope; reduced
//! motion skips the entrance; the remaining-time bar runs, stands still
//! while the stack is expanded, starts over by `id` and runs out as the
//! toast goes; the gaps between its icon, message and buttons.

mod common;

use std::{
    sync::{Arc, Mutex},
    time::Duration,
};

use common::{bounds, start};
use nsh_core::{MountItem, NodeId, prelude::*};
use nsh_longbridge::{
    ScopeTokens, ToastKind, ToastPlacement, button, dialog, dialog_content, dialog_title,
    dismiss_toast, theme_scope, toast, toaster,
};
use nsh_testing::Headless;

/// Shows what `make` shows when "Show" is pressed, writing down the
/// answers; "Dismiss" dismisses every toast.
fn app(make: fn(u32) -> nsh_longbridge::ToastBuilder, toaster_el: AnyElement) -> impl IntoElement {
    let log = signal(Vec::<String>::new());
    let count = stored(0u32);
    view()
        .pt(px(40.0))
        .pl(px(40.0))
        .items_start()
        .gap(px(8.0))
        .child(button().child("Show").on_press(move || {
            let n = count.get() + 1;
            count.replace(n);
            let answer = make(n).show();
            spawn(async move {
                let result = answer.await;
                let entry = format!("answer:{}", result.action.as_deref().unwrap_or("none"));
                log.update(|l| l.push(entry));
            });
        }))
        .child(button().child("Dismiss").on_press(|| dismiss_toast(None)))
        .child(text(move || format!("log:{}", log.get().join(","))))
        .child(toaster_el)
}

fn saved(_: u32) -> nsh_longbridge::ToastBuilder {
    toast("Event created")
        .description("Sunday at 9:00")
        .action("undo", "Undo")
        .duration(None)
}

#[component]
fn Basic() -> impl IntoElement {
    app(saved, toaster().into_element())
}

fn show(app: &mut Headless) {
    app.press("role:button[Show]").unwrap();
    app.run_for(Duration::from_millis(500)).unwrap();
}

#[test]
fn a_toast_shows_its_text_and_is_an_alert() {
    let mut app = start::<Basic>(true);
    assert!(!app.shows("Event created"));
    show(&mut app);
    assert!(app.shows("Event created"));
    assert!(app.shows("Sunday at 9:00"));
    assert!(
        app.screen()
            .matches("role:alert[Event created. Sunday at 9:00]")
            .unwrap(),
        "read once as it shows: title, then description"
    );
    assert!(app.screen().matches("role:button[Undo]").unwrap());
    assert!(
        !app.screen().matches("role:button[Close]").unwrap(),
        "no × by default"
    );
}

#[test]
fn its_action_closes_it_and_answers() {
    let mut app = start::<Basic>(true);
    show(&mut app);
    app.press("role:button[Undo]").unwrap();
    app.run_for(Duration::from_millis(600)).unwrap();
    assert!(!app.shows("Event created"));
    app.wait_for("text:log:answer:undo").unwrap();
}

#[test]
fn dismiss_toast_closes_every_toast_without_an_answer() {
    let mut app = start::<Basic>(true);
    show(&mut app);
    show(&mut app);
    app.press("role:button[Dismiss]").unwrap();
    app.run_for(Duration::from_millis(600)).unwrap();
    assert!(!app.shows("Event created"));
    app.wait_for("text:log:answer:none,answer:none").unwrap();
}

fn make_brief(_: u32) -> nsh_longbridge::ToastBuilder {
    toast("Copied").duration(Some(Duration::from_millis(1000)))
}

#[component]
fn Brief() -> impl IntoElement {
    app(make_brief, toaster().into_element())
}

/// It goes by itself after its time.
#[test]
fn a_toast_goes_after_its_time() {
    let mut app = start::<Brief>(true);
    show(&mut app);
    // A headless step may run the clock for up to 2 s while toasts tick.
    app.run_for(Duration::from_millis(3000)).unwrap();
    assert!(!app.shows("Copied"));
    app.wait_for("text:log:answer:none").unwrap();
}

/// The first is loading; the next ones change it in place (same `id`).
fn make_upload(n: u32) -> nsh_longbridge::ToastBuilder {
    if n == 1 {
        toast("Uploading").id("upload").kind(ToastKind::Loading)
    } else {
        toast("Uploaded")
            .id("upload")
            .kind(ToastKind::Success)
            .duration(None)
    }
}

#[component]
fn Upload() -> impl IntoElement {
    app(make_upload, toaster().into_element())
}

#[test]
fn an_id_changes_a_toast_in_place() {
    let mut app = start::<Upload>(true);
    show(&mut app);
    assert!(app.shows("Uploading"));
    // Loading never times out.
    app.run_for(Duration::from_millis(6000)).unwrap();
    assert!(app.shows("Uploading"));
    show(&mut app);
    assert!(app.shows("Uploaded"));
    assert!(!app.shows("Uploading"), "one toast, changed");
}

fn make_numbered(n: u32) -> nsh_longbridge::ToastBuilder {
    toast(format!("Toast {n}")).duration(None)
}

#[component]
fn Numbered() -> impl IntoElement {
    app(make_numbered, toaster().into_element())
}

/// `visibleToasts` (3) are mounted; older ones wait.
#[test]
fn three_toasts_are_mounted_at_once() {
    let mut app = start::<Numbered>(true);
    for _ in 0..5 {
        show(&mut app);
    }
    let shown: Vec<bool> = (1..=5).map(|n| app.shows(&format!("Toast {n}"))).collect();
    assert_eq!(shown, vec![false, false, true, true, true]);
}

/// Bottom right on a desktop: 356 wide, 24 from the window's edges.
#[test]
fn the_stack_sits_at_the_bottom_right() {
    let mut app = start::<Basic>(true);
    show(&mut app);
    let alert = bounds(&app, "role:alert[Event created. Sunday at 9:00]");
    assert_eq!(alert.size.width.0, 356.0, "{alert:?}");
    assert_eq!(alert.origin.x.0 + alert.size.width.0, 800.0 - 24.0);
    assert_eq!(alert.origin.y.0 + alert.size.height.0, 600.0 - 24.0);
}

#[component]
fn TopCenter() -> impl IntoElement {
    app(
        saved,
        toaster()
            .placement(ToastPlacement::TopCenter)
            .close_button(true)
            .into_element(),
    )
}

/// On a phone the stack spans the window less 16, at the placement's edge.
#[test]
fn on_a_phone_the_stack_spans_the_window() {
    let mut app = start::<TopCenter>(false);
    show(&mut app);
    let alert = bounds(&app, "role:alert[Event created. Sunday at 9:00]");
    assert_eq!(alert.origin.x.0, 16.0, "{alert:?}");
    assert_eq!(alert.size.width.0, 800.0 - 32.0);
    assert_eq!(alert.origin.y.0, 16.0);
}

/// `closeButton`: the "×" at the top-end corner, inside the toast, closes
/// it, answering nothing.
#[test]
fn the_close_button_dismisses_it() {
    let mut app = start::<TopCenter>(true);
    show(&mut app);
    let alert = bounds(&app, "role:alert[Event created. Sunday at 9:00]");
    let close = bounds(&app, "role:button[Close]");
    assert_eq!((close.size.width.0, close.size.height.0), (20.0, 20.0));
    // Inside: its end `spacing` (4) into the padding (16) past the border
    // (1); centered on the title's first line (18 high).
    let end = alert.origin.x.0 + alert.size.width.0;
    assert_eq!(
        close.origin.x.0 + close.size.width.0,
        end - 13.0,
        "top-end corner"
    );
    assert_eq!(close.origin.y.0, alert.origin.y.0 + 16.0);
    app.press("role:button[Close]").unwrap();
    app.run_for(Duration::from_millis(600)).unwrap();
    assert!(!app.shows("Event created"));
    app.wait_for("text:log:answer:none").unwrap();
}

/// A dialog open over the page, the toaster in a scope of its own.
#[component]
fn OverDialog() -> impl IntoElement {
    let open = signal(true);
    view()
        .child(app(
            saved,
            theme_scope()
                .tokens(ScopeTokens::new().set("toast", "#2563eb"))
                .child(toaster())
                .into_element(),
        ))
        .child(
            theme_scope()
                .tokens(ScopeTokens::new().set("toast", "#dc2626"))
                .child(
                    dialog()
                        .open(move || Some(open.get()))
                        .on_open_change(move |e: &nsh_longbridge::DialogEvent| {
                            if let nsh_longbridge::DialogEvent::OpenChange(next, _) = e {
                                open.set(*next);
                            }
                        })
                        .child(
                            dialog_content()
                                .child(dialog_title().child("Settings"))
                                .child(button().child("Notify").on_press(|| {
                                    toast("From the dialog").duration(None).show();
                                })),
                        ),
                ),
        )
}

/// Whether the toast `label` is inside the dialog's modal.
fn in_dialog(app: &Headless, label: &str) -> bool {
    let Ok(modal) = app.screen().id("role:dialog[Settings]") else {
        return false;
    };
    let nodes = app.screen().layout_nodes();
    let mut at = app.screen().id(&format!("role:alert[{label}]")).unwrap();
    loop {
        if at == modal {
            return true;
        }
        match nodes.iter().find(|n| n.id == at) {
            Some(n) if n.parent != at => at = n.parent,
            _ => return false,
        }
    }
}

/// The toast box's fill: the alert's child.
fn toast_fill(
    app: &Headless,
    styles: &Arc<Mutex<Vec<(NodeId, nsh_core::ViewStyle)>>>,
    label: &str,
) -> u32 {
    let alert = app.screen().id(&format!("role:alert[{label}]")).unwrap();
    let nodes = app.screen().layout_nodes();
    let boxed = nodes.iter().find(|n| n.parent == alert).unwrap().id;
    styles
        .lock()
        .unwrap()
        .iter()
        .rev()
        .find(|(id, _)| *id == boxed)
        .map(|(_, s)| s.background_color.0)
        .unwrap()
}

fn watch(app: &mut Headless) -> Arc<Mutex<Vec<(NodeId, nsh_core::ViewStyle)>>> {
    let styles = Arc::new(Mutex::new(Vec::new()));
    let seen = styles.clone();
    app.observe(move |_, batch| {
        for item in &batch.items {
            if let MountItem::ViewStyle { id, style } = item {
                seen.lock().unwrap().push((*id, (**style).clone()));
            }
        }
    });
    styles
}

/// A toast shown while a dialog is open is drawn in the dialog's modal,
/// once, in the `Toaster`'s scope; it stays as the dialog closes.
#[test]
fn toasts_show_above_an_open_dialog_in_the_toasters_scope() {
    let mut app = start::<OverDialog>(true);
    let styles = watch(&mut app);
    app.run_for(Duration::from_millis(400)).unwrap();
    assert!(app.shows("Settings"));
    app.press("role:button[Notify]").unwrap();
    app.run_for(Duration::from_millis(500)).unwrap();
    assert!(app.shows("From the dialog"));
    let copies = app
        .screen()
        .texts()
        .iter()
        .filter(|t| t.trim() == "From the dialog")
        .count();
    assert_eq!(copies, 1, "drawn once");
    assert!(in_dialog(&app, "From the dialog"), "drawn in the modal");
    let blue = nsh_core::Color::parse("#2563eb").unwrap().0;
    assert_eq!(toast_fill(&app, &styles, "From the dialog"), blue);
    app.press_key("Escape").unwrap();
    app.run_for(Duration::from_millis(500)).unwrap();
    assert!(!app.shows("Settings"));
    assert!(app.shows("From the dialog"), "it moved back to the page");
    assert!(!in_dialog(&app, "From the dialog"));
    assert_eq!(toast_fill(&app, &styles, "From the dialog"), blue);
}

/// With reduced motion a toast has no entrance: its first style is its
/// look.
#[test]
fn reduced_motion_skips_the_entrance() {
    let mut config = nsh_engine::EngineConfig {
        viewport: nsh_core::Size::new(px(800.0), px(600.0)),
        clock: nsh_engine::Clock::manual(std::time::SystemTime::now()),
        ..nsh_engine::EngineConfig::new(nsh_testing::windowed_root::<Basic>())
    };
    config.environment.os = nsh_core::platform::Os::MacOs;
    config.environment.reduce_motion = true;
    let mut app = Headless::start(config).unwrap();
    app.first_render().unwrap();
    let styles = watch(&mut app);
    show(&mut app);
    let alert = app
        .screen()
        .id("role:alert[Event created. Sunday at 9:00]")
        .unwrap();
    let firsts: Vec<f32> = styles
        .lock()
        .unwrap()
        .iter()
        .filter(|(id, _)| *id == alert)
        .map(|(_, s)| s.opacity)
        .collect();
    assert!(!firsts.is_empty());
    assert!(firsts.iter().all(|o| *o == 1.0), "{firsts:?}");
}

/// Without reduced motion its first style is the starting look.
#[test]
fn a_toast_enters_from_the_edge() {
    let mut app = start::<Basic>(true);
    let styles = watch(&mut app);
    show(&mut app);
    let alert = app
        .screen()
        .id("role:alert[Event created. Sunday at 9:00]")
        .unwrap();
    let first = styles
        .lock()
        .unwrap()
        .iter()
        .find(|(id, _)| *id == alert)
        .map(|(_, s)| s.clone())
        .unwrap();
    assert!(first.opacity < 1.0, "{first:?}");
}

/// A second `Toaster` draws nothing.
#[component]
fn Two() -> impl IntoElement {
    view()
        .child(app(saved, toaster().into_element()))
        .child(toaster())
}

#[test]
fn a_second_toaster_draws_nothing() {
    let mut app = start::<Two>(true);
    show(&mut app);
    let copies = app
        .screen()
        .texts()
        .iter()
        .filter(|t| t.trim() == "Event created")
        .count();
    assert_eq!(copies, 1);
}

/// TSX gets `toast` and `dismissToast` as functions.
#[test]
fn tsx_gets_toast_and_dismiss_toast() {
    let registry = nsh_longbridge::registry();
    assert_eq!(registry.undocumented(), Vec::<String>::new());
    let ts = nsh_core::element::declarations(&registry, &[]);
    for declaration in [
        "export declare function toast(title: string, options?: ToastOptions | null): Promise<ToastResult>;",
        "export declare function dismissToast(id?: string | null): void;",
        "export type ToastKind = \"default\" | \"success\" | \"info\" | \"warning\" | \"error\" | \"loading\";",
        "export declare const Toaster: ((props: ToasterProps) => JSX.Element);",
    ] {
        assert!(ts.contains(declaration), "{declaration}\n{ts}");
    }
}

fn make_layered(n: u32) -> nsh_longbridge::ToastBuilder {
    match n {
        1 => toast("Older").action("undo", "Undo").duration(None),
        _ => toast("Newer").duration(None),
    }
}

#[component]
fn Layered() -> impl IntoElement {
    app(make_layered, toaster().into_element())
}

/// Collapsed, a layer behind the newest hides its content (cards behind
/// the front one); the pointer on the stack spreads it out
/// and shows it.
#[test]
fn a_collapsed_stacks_back_layers_hide_their_content() {
    let mut app = start::<Layered>(true);
    show(&mut app);
    show(&mut app);
    let opacity = |app: &Headless| {
        app.screen()
            .view_style("role:button[Undo]")
            .unwrap()
            .unwrap()
            .opacity
    };
    assert_eq!(opacity(&app), 0.0, "behind the newest");
    app.hover("role:alert[Newer]").unwrap();
    app.run_for(Duration::from_millis(600)).unwrap();
    assert_eq!(opacity(&app), 1.0, "spread out");
}

// The remaining-time bar.

/// The styles applied to each view, with the app's time when they were.
type Timed = Arc<Mutex<Vec<(std::time::Duration, NodeId, nsh_core::ViewStyle)>>>;

fn watch_timed(app: &mut Headless) -> Timed {
    let styles = Timed::default();
    let seen = styles.clone();
    let clock = app.clock().clone();
    let start = clock.now();
    app.observe(move |_, batch| {
        let at = clock.now().saturating_duration_since(start);
        for item in &batch.items {
            if let MountItem::ViewStyle { id, style } = item {
                seen.lock().unwrap().push((at, *id, (**style).clone()));
            }
        }
    });
    styles
}

/// The remaining-time bar of the toast `label`: the 2-high view inside
/// it.
fn timer_bar(app: &Headless, label: &str) -> Option<NodeId> {
    let alert = app.screen().id(&format!("role:alert[{label}]")).unwrap();
    let nodes = app.screen().layout_nodes();
    let inside = |mut at: NodeId| loop {
        if at == alert {
            return true;
        }
        match nodes.iter().find(|n| n.id == at) {
            Some(n) if n.parent != at => at = n.parent,
            _ => return false,
        }
    };
    nodes
        .iter()
        .find(|n| n.id != alert && n.bounds.size.height.0 == 2.0 && inside(n.id))
        .map(|n| n.id)
}

/// What the bar shows: its horizontal scale and the time its transition
/// takes, as last styled.
fn bar_look(styles: &Timed, bar: NodeId) -> (f32, f32) {
    let styles = styles.lock().unwrap();
    let (_, _, style) = styles.iter().rev().find(|(_, id, _)| *id == bar).unwrap();
    let scale = style
        .transform
        .ops()
        .iter()
        .find(|op| op.kind == nsh_core::TransformKind::ScaleX)
        .map_or(1.0, |op| op.value);
    (scale, style.transition.map_or(0.0, |t| t.duration))
}

const LONG: Duration = Duration::from_secs(20);

fn make_archived(_: u32) -> nsh_longbridge::ToastBuilder {
    toast("Archived")
        .id("archived")
        .action("undo", "Undo")
        .duration(Some(LONG))
}

#[component]
fn Archived() -> impl IntoElement {
    app(make_archived, toaster().into_element())
}

/// A toast with a button shows its remaining time: a bar along its bottom
/// edge, inside the border, shrinking to nothing at a constant pace over
/// the time left once the toast is in. Decorative: no accessible element.
#[test]
fn a_toast_with_a_button_shows_its_remaining_time() {
    let mut app = start::<Archived>(true);
    let styles = watch_timed(&mut app);
    show(&mut app);
    let bar = timer_bar(&app, "Archived").expect("a bar");
    let alert = bounds(&app, "role:alert[Archived]");
    let b = app
        .screen()
        .layout_nodes()
        .into_iter()
        .find(|n| n.id == bar)
        .unwrap()
        .bounds;
    assert_eq!(
        b.size.width.0,
        alert.size.width.0 - 2.0,
        "inside the border"
    );
    assert_eq!(
        b.origin.y.0 + 2.0,
        alert.origin.y.0 + alert.size.height.0 - 1.0,
        "on the bottom edge, inside the border"
    );
    let (scale, ms) = bar_look(&styles, bar);
    assert_eq!(scale, 0.0, "running to nothing");
    assert!(ms > 15_000.0 && ms <= 20_000.0, "over the time left: {ms}");
    let style = styles
        .lock()
        .unwrap()
        .iter()
        .rev()
        .find(|(_, id, _)| *id == bar)
        .map(|(_, _, s)| s.clone())
        .unwrap();
    let transition = style.transition.unwrap();
    assert_eq!(
        transition.easing,
        [0.0, 0.0, 1.0, 1.0],
        "at a constant pace"
    );
    assert!(
        app.screen().matches("role:alert[Archived]").unwrap(),
        "read as before: the bar adds nothing"
    );
}

/// Expanding the stack stops the bar where the time is; collapsing it
/// runs it again over exactly what is left; the time stood still.
#[test]
fn the_bar_stands_still_while_the_stack_is_expanded() {
    let mut app = start::<Archived>(true);
    let styles = watch_timed(&mut app);
    show(&mut app);
    app.run_for(Duration::from_millis(3000)).unwrap();
    let bar = timer_bar(&app, "Archived").unwrap();
    app.hover("role:alert[Archived]").unwrap();
    let (paused, ms) = bar_look(&styles, bar);
    assert!(paused > 0.5 && paused < 0.95, "what is left: {paused}");
    assert_eq!(ms, 0.0, "stands at once");
    app.run_for(LONG + Duration::from_secs(5)).unwrap();
    assert!(app.shows("Archived"), "its time stood still");
    assert_eq!(bar_look(&styles, bar), (paused, 0.0), "the bar too");
    app.hover("role:button[Show]").unwrap();
    let (scale, ms) = bar_look(&styles, bar);
    assert_eq!(scale, 0.0);
    let left = paused * LONG.as_secs_f32() * 1000.0;
    assert!((ms - left).abs() < 1.0, "over what is left: {ms} vs {left}");
}

/// Shown again with its `id`, a toast's bar fills again: its time starts
/// over.
#[test]
fn showing_a_toast_again_by_id_starts_its_bar_over() {
    let mut app = start::<Archived>(true);
    let styles = watch_timed(&mut app);
    show(&mut app);
    app.run_for(Duration::from_millis(3000)).unwrap();
    let bar = timer_bar(&app, "Archived").unwrap();
    app.hover("role:alert[Archived]").unwrap();
    let (before, _) = bar_look(&styles, bar);
    assert!(before < 0.95, "{before}");
    // Shown again while the stack is expanded: full, and standing still.
    app.press("role:button[Show]").unwrap();
    app.hover("role:alert[Archived]").unwrap();
    app.run_for(Duration::from_millis(1000)).unwrap();
    let bar = timer_bar(&app, "Archived").unwrap();
    assert_eq!(bar_look(&styles, bar).0, 1.0, "full again");
}

/// The bar runs out as the toast goes: they read the same time.
#[test]
fn the_bar_runs_out_as_the_toast_goes() {
    let mut app = start::<Archived>(true);
    let styles = watch_timed(&mut app);
    show(&mut app);
    let bar = timer_bar(&app, "Archived").unwrap();
    let alert = app.screen().id("role:alert[Archived]").unwrap();
    app.run_for(LONG + Duration::from_secs(3)).unwrap();
    assert!(!app.shows("Archived"));
    let styles = styles.lock().unwrap();
    let (started, _, run) = styles
        .iter()
        .find(|(_, id, s)| *id == bar && s.transition.is_some_and(|t| t.duration > 1000.0))
        .unwrap();
    let ends = *started + Duration::from_secs_f32(run.transition.unwrap().duration / 1000.0);
    let (gone, _, _) = styles
        .iter()
        .find(|(at, id, s)| *id == alert && *at > *started && s.opacity < 1.0)
        .unwrap();
    let off = gone.as_secs_f32() - ends.as_secs_f32();
    assert!(
        off.abs() <= 0.1,
        "the toast went {off} s after the bar ran out"
    );
}

fn make_mixed(n: u32) -> nsh_longbridge::ToastBuilder {
    let long = Some(LONG);
    match n {
        1 => toast("Copied").duration(long),
        2 => toast("Pinned").timer(true).duration(long),
        3 => toast("Moved")
            .action("undo", "Undo")
            .timer(false)
            .duration(long),
        4 => toast("Syncing")
            .kind(ToastKind::Loading)
            .action("stop", "Stop"),
        5 => toast("Kept").action("undo", "Undo").duration(None),
        _ => toast("Sent").cancel("open", "Open").duration(long),
    }
}

#[component]
fn Mixed() -> impl IntoElement {
    app(make_mixed, toaster().visible_toasts(6).into_element())
}

/// No bar on a plain message, a loading toast or one that stays until
/// dismissed; `timer` forces it on or off; a cancel counts as a button.
#[test]
fn which_toasts_show_the_bar() {
    let mut app = start::<Mixed>(true);
    for _ in 0..6 {
        show(&mut app);
    }
    // Spread out, every layer's content shows.
    app.hover("role:alert[Sent]").unwrap();
    let shown: Vec<(&str, bool)> = ["Copied", "Pinned", "Moved", "Syncing", "Kept", "Sent"]
        .into_iter()
        .map(|label| (label, timer_bar(&app, label).is_some()))
        .collect();
    assert_eq!(
        shown,
        vec![
            ("Copied", false),
            ("Pinned", true),
            ("Moved", false),
            ("Syncing", false),
            ("Kept", false),
            ("Sent", true),
        ]
    );
}

#[component]
fn NoTimers() -> impl IntoElement {
    app(make_archived, toaster().timer(Some(false)).into_element())
}

/// The `Toaster`'s `timer: false` turns the bars off.
#[test]
fn the_toaster_turns_the_bars_off() {
    let mut app = start::<NoTimers>(true);
    show(&mut app);
    assert!(app.shows("Archived"));
    assert!(timer_bar(&app, "Archived").is_none());
}

/// With reduced motion there is no bar; the time is unchanged.
#[test]
fn reduced_motion_shows_no_bar() {
    let mut config = nsh_engine::EngineConfig {
        viewport: nsh_core::Size::new(px(800.0), px(600.0)),
        clock: nsh_engine::Clock::manual(std::time::SystemTime::now()),
        ..nsh_engine::EngineConfig::new(nsh_testing::windowed_root::<Archived>())
    };
    config.environment.os = nsh_core::platform::Os::MacOs;
    config.environment.reduce_motion = true;
    let mut app = Headless::start(config).unwrap();
    app.first_render().unwrap();
    show(&mut app);
    assert!(app.shows("Archived"));
    assert!(timer_bar(&app, "Archived").is_none());
}

#[component]
fn TopArchived() -> impl IntoElement {
    app(
        make_archived,
        toaster().placement(ToastPlacement::TopRight).into_element(),
    )
}

/// At the top, the bar is on the toast's top edge.
#[test]
fn at_the_top_the_bar_is_on_the_top_edge() {
    let mut app = start::<TopArchived>(true);
    show(&mut app);
    let bar = timer_bar(&app, "Archived").unwrap();
    let alert = bounds(&app, "role:alert[Archived]");
    let b = app
        .screen()
        .layout_nodes()
        .into_iter()
        .find(|n| n.id == bar)
        .unwrap()
        .bounds;
    assert_eq!(b.origin.y.0, alert.origin.y.0 + 1.0);
}

fn make_failed(_: u32) -> nsh_longbridge::ToastBuilder {
    toast("Upload failed")
        .description("The file is too large")
        .kind(ToastKind::Error)
        .cancel("later", "Later")
        .action("retry", "Retry")
        .duration(None)
}

#[component]
fn Failed() -> impl IntoElement {
    app(make_failed, toaster().into_element())
}

/// The start and end of `node`, across and down.
fn spans(app: &Headless, node: NodeId) -> ((f32, f32), (f32, f32)) {
    let b = bounds(app, &format!("view:{node}"));
    (
        (b.origin.x.0, (b.origin.x + b.size.width).0),
        (b.origin.y.0, (b.origin.y + b.size.height).0),
    )
}

/// The icon, the message and the buttons are three groups `toast-gap`
/// (12) apart; the title and the description, parts of one message, 4; the
/// two buttons, closely related, 8.
#[test]
fn the_icon_message_and_buttons_are_spaced_on_the_scale() {
    let mut app = start::<Failed>(true);
    show(&mut app);
    let screen = app.screen();
    let nodes = screen.layout_nodes();
    let parent = |id: NodeId| nodes.iter().find(|n| n.id == id).unwrap().parent;
    let title = screen.id("text:Upload failed").unwrap();
    let description = screen.id("text:The file is too large").unwrap();
    let message = parent(title);
    let toast = parent(message);
    let parts: Vec<NodeId> = nodes
        .iter()
        .filter(|n| n.parent == toast)
        .map(|n| n.id)
        .collect();
    let at = parts.iter().position(|&p| p == message).unwrap();
    let (icon, buttons) = (parts[at - 1], parts[at + 1]);
    assert_eq!(
        spans(&app, icon).0.1 - spans(&app, icon).0.0,
        16.0,
        "the icon"
    );
    let later = screen.id("role:button[Later]").unwrap();
    let retry = screen.id("role:button[Retry]").unwrap();
    assert_eq!(parent(later), buttons);
    let (icon, message, buttons) = (
        spans(&app, icon),
        spans(&app, message),
        spans(&app, buttons),
    );
    assert_eq!(message.0.0 - icon.0.1, 12.0, "icon to message");
    assert_eq!(buttons.0.0 - message.0.1, 12.0, "message to buttons");
    assert_eq!(
        spans(&app, description).1.0 - spans(&app, title).1.1,
        4.0,
        "title to description"
    );
    assert_eq!(
        spans(&app, retry).0.0 - spans(&app, later).0.1,
        8.0,
        "between the buttons"
    );
}
