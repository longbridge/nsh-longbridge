//! `Toaster` and `toast()` in a headless engine: a toast shows its title,
//! description and buttons, is an `alert` read as it shows, times out,
//! answers with the button that closed it, changes in place by `id`, keeps
//! `visibleToasts` mounted; `dismissToast`; the stack's place and width;
//! it shows above an open dialog, in the `Toaster`'s theme scope; reduced
//! motion skips the entrance.

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

/// `closeButton`: the "×" on the top-start corner closes it, answering
/// nothing.
#[test]
fn the_close_button_dismisses_it() {
    let mut app = start::<TopCenter>(true);
    show(&mut app);
    let alert = bounds(&app, "role:alert[Event created. Sunday at 9:00]");
    let close = bounds(&app, "role:button[Close]");
    assert_eq!((close.size.width.0, close.size.height.0), (20.0, 20.0));
    // Out by 35% (7) from inside the edge (1).
    assert_eq!(close.origin.x.0, alert.origin.x.0 - 6.0, "out by 35%");
    assert_eq!(close.origin.y.0, alert.origin.y.0 - 6.0);
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
