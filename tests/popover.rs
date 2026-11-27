//! `Popover` in a headless engine: its trigger opens and closes it, the
//! popup surface (fill, edge, shadow) reaches the popup unclipped, a scope
//! around the trigger reaches the content, its entrance slides from the
//! side it went on, and the events pass through.

mod common;

use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
    time::Duration,
};

use common::{bounds, light, start};
use nsh_core::{MountItem, NodeId, prelude::*};
use nsh_longbridge::{
    ButtonVariant, PopoverEvent, ScopeTokens, ThemeAppearance, button, popover,
    popover_description, popover_header, popover_title, theme::semantic::POPOVER, theme_scope,
    tooltip,
};
use nsh_testing::Headless;

/// A popover opened by a button, writing down its events.
#[component]
fn Basic() -> impl IntoElement {
    let log = signal(Vec::<String>::new());
    view()
        .pt(px(100.0))
        .pl(px(100.0))
        .child(
            view().self_start().child(
                popover()
                    .trigger(button().variant(ButtonVariant::Outline).child("Open"))
                    .on_open_change(move |e: &PopoverEvent| {
                        if let PopoverEvent::OpenChange(open, reason) = e {
                            let entry = format!("change:{open}:{reason:?}");
                            log.update(|l| l.push(entry));
                        }
                    })
                    .on_open(move || log.update(|l| l.push("open".into())))
                    .on_close(move || log.update(|l| l.push("close".into())))
                    .on_cancel(move || log.update(|l| l.push("cancel".into())))
                    .child(
                        popover_header()
                            .test_id("content")
                            .child(popover_title().child("Dimensions"))
                            .child(popover_description().child("Set the dimensions.")),
                    ),
            ),
        )
        .child(
            view()
                .test_id("away")
                .h(px(200.0))
                .on_press(|| {})
                .child(text("Away")),
        )
        .child(text(move || format!("log:{}", log.get().join(","))))
}

#[test]
fn the_trigger_opens_it_and_escape_closes_it() {
    let mut app = start::<Basic>(true);
    assert!(!app.shows("Dimensions"));
    app.press("role:button[Open]").unwrap();
    app.run_for(Duration::from_millis(300)).unwrap();
    assert!(app.shows("Dimensions"));
    let state = app
        .screen()
        .accessibility_state("role:button[Open]")
        .unwrap();
    assert_eq!(state.expanded, Some(true), "the trigger is expanded");
    app.wait_for("text:log:change:true:TriggerPress,open")
        .unwrap();
    app.press_key("Escape").unwrap();
    app.run_for(Duration::from_millis(400)).unwrap();
    assert!(!app.shows("Dimensions"));
    app.wait_for("text:log:change:true:TriggerPress,open,cancel,change:false:Cancel,close")
        .unwrap();
    let state = app
        .screen()
        .accessibility_state("role:button[Open]")
        .unwrap();
    assert_eq!(state.expanded, Some(false));
}

#[test]
fn a_press_outside_closes_it() {
    let mut app = start::<Basic>(true);
    app.press("role:button[Open]").unwrap();
    app.run_for(Duration::from_millis(300)).unwrap();
    assert!(app.shows("Dimensions"));
    app.press("id:away").unwrap();
    app.run_for(Duration::from_millis(400)).unwrap();
    assert!(!app.shows("Dimensions"));
    // Opened again, its content is made again.
    app.press("role:button[Open]").unwrap();
    app.run_for(Duration::from_millis(300)).unwrap();
    assert!(app.shows("Dimensions"));
}

/// The view the popup's panel is: the content's parent.
fn panel(app: &Headless) -> NodeId {
    let content = app.screen().id("id:content").unwrap();
    app.screen()
        .layout_nodes()
        .iter()
        .find(|n| n.id == content)
        .unwrap()
        .parent
}

/// Watches the views' styles: the latest of each.
fn watch_styles(app: &mut Headless) -> Arc<Mutex<HashMap<NodeId, nsh_core::ViewStyle>>> {
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

/// The popup surface: the `popover` fill, `popover-width` wide, and a
/// shadow (tier 3) that is drawn: on the panel, which nothing clips.
#[test]
fn the_surface_and_its_shadow_reach_the_popup() {
    let mut app = start::<Basic>(true);
    let styles = watch_styles(&mut app);
    app.press("role:button[Open]").unwrap();
    app.run_for(Duration::from_millis(300)).unwrap();
    let panel = panel(&app);
    let node = app
        .screen()
        .layout_nodes()
        .into_iter()
        .find(|n| n.id == panel)
        .unwrap();
    // `w-72`, its edge inside it.
    assert_eq!(node.bounds.size.width.0, 288.0);
    let style = styles.lock().unwrap().get(&panel).cloned().unwrap();
    assert_eq!(Some(style.background_color.0), light(POPOVER));
    let shadows = style
        .effects
        .as_ref()
        .map(|e| e.box_shadows.len())
        .unwrap_or(0);
    assert_eq!(shadows, 2, "tier 3 has two layers: {style:?}");
    // Nothing around it clips it (its shadow is drawn past its frame).
    let nodes = app.screen().layout_nodes();
    let mut at = Some(panel);
    while let Some(id) = at {
        let Some(n) = nodes.iter().find(|n| n.id == id) else {
            break;
        };
        assert!(!n.clips || id == panel, "{id:?} clips the popup");
        at = (n.parent != nsh_core::ROOT_ID).then_some(n.parent);
    }
}

/// A trigger near the bottom of the window: the popup flips above it.
#[component]
fn Flipped() -> impl IntoElement {
    view().pt(px(540.0)).pl(px(100.0)).child(
        view().self_start().child(
            popover()
                .trigger(button().child("Open"))
                .child(view().test_id("content").h(px(120.0)).child(text("Tall"))),
        ),
    )
}

/// The popup's entrance starts 8 points toward its trigger from the side it
/// went on: after a flip to the top, from below (`translate-y-2`).
#[test]
fn a_flipped_popup_slides_from_the_side_it_went_on() {
    let mut app = start::<Flipped>(true);
    let firsts = Arc::new(Mutex::new(Vec::<(NodeId, nsh_core::ViewStyle)>::new()));
    let seen = firsts.clone();
    app.observe(move |_, batch| {
        for item in &batch.items {
            if let MountItem::ViewStyle { id, style } = item {
                seen.lock().unwrap().push((*id, (**style).clone()));
            }
        }
    });
    app.press("role:button[Open]").unwrap();
    app.run_for(Duration::from_millis(300)).unwrap();
    assert!(app.shows("Tall"));
    let content = bounds(&app, "id:content");
    let trigger = bounds(&app, "role:button[Open]");
    assert!(
        content.origin.y < trigger.origin.y,
        "flipped above: {content:?} {trigger:?}"
    );
    // The popup is the panel's parent; its first style is its starting look.
    let panel = panel(&app);
    let popup = app
        .screen()
        .layout_nodes()
        .into_iter()
        .find(|n| n.id == panel)
        .unwrap()
        .parent;
    let styles = firsts.lock().unwrap();
    let first = styles
        .iter()
        .find(|(id, _)| *id == popup)
        .map(|(_, s)| s.clone())
        .expect("the popup's style");
    assert!(first.opacity < 1.0, "it starts faded: {first:?}");
    let translate_y: f32 = first
        .transform
        .ops()
        .iter()
        .filter(|op| op.kind == nsh_core::TransformKind::TranslateY)
        .map(|op| op.value)
        .sum();
    assert!(
        translate_y > 0.0,
        "it starts below its place, toward the trigger: {:?}",
        first.transform
    );
}

/// A popover in a dark scope that changes `popover`.
#[component]
fn Scoped() -> impl IntoElement {
    view().pt(px(100.0)).pl(px(100.0)).child(
        theme_scope()
            .tokens(ScopeTokens::new().set("popover", "#2563eb"))
            .appearance(ThemeAppearance::Dark)
            .child(
                view().self_start().child(
                    popover()
                        .trigger(button().child("Open"))
                        .child(view().test_id("content").child(text("Inside"))),
                ),
            ),
    )
}

#[test]
fn the_scope_around_the_trigger_reaches_the_popup() {
    let mut app = start::<Scoped>(true);
    let styles = watch_styles(&mut app);
    app.press("role:button[Open]").unwrap();
    app.run_for(Duration::from_millis(300)).unwrap();
    let panel = panel(&app);
    let style = styles.lock().unwrap().get(&panel).cloned().unwrap();
    let blue = nsh_core::Color::parse("#2563eb").unwrap().0;
    assert_eq!(style.background_color.0, blue);
}

/// A trigger with a tooltip, opened from the keyboard.
#[component]
fn TippedTrigger() -> impl IntoElement {
    view().pt(px(100.0)).pl(px(100.0)).child(
        view()
            .self_start()
            .tooltip(tooltip().delay(0.0).child("Hint"))
            .child(
                popover()
                    .trigger(button().child("Open"))
                    .child(view().test_id("content").child(text("Panel"))),
            ),
    )
}

/// The trigger's tooltip neither covers its open popover nor takes its
/// Escape (the spec's NSH gap 1).
#[test]
fn a_triggers_tooltip_stays_out_of_its_open_popover() {
    let mut app = start::<TippedTrigger>(true);
    app.press_key("Tab").unwrap();
    app.run_for(Duration::from_millis(300)).unwrap();
    let tip_on_focus = app.shows("Hint");
    app.press_key("Enter").unwrap();
    app.run_for(Duration::from_millis(300)).unwrap();
    assert!(app.shows("Panel"), "opened from the keyboard");
    let tip_while_open = app.shows("Hint");
    app.press_key("Escape").unwrap();
    app.run_for(Duration::from_millis(400)).unwrap();
    let panel_after_escape = app.shows("Panel");
    eprintln!(
        "tip on focus: {tip_on_focus}, tip while open: {tip_while_open}, panel after one Escape: {panel_after_escape}"
    );
    assert!(!tip_while_open, "the tooltip shows over the open popover");
    assert!(!panel_after_escape, "the first Escape closes the popover");
}

/// A `Button` trigger keeps its hover fill while its popover is open.
#[test]
fn an_open_popovers_button_trigger_is_lit() {
    let mut app = start::<Basic>(true);
    let rest = common::fill(&app, "role:button[Open]");
    app.press("role:button[Open]").unwrap();
    app.run_for(Duration::from_millis(300)).unwrap();
    // Away from it, so no hover lights it.
    app.hover("id:away").unwrap();
    app.run_for(Duration::from_millis(300)).unwrap();
    let open = common::fill(&app, "role:button[Open]");
    assert_eq!(
        open,
        light(nsh_longbridge::components::button::BUTTON_OUTLINE_HOVER)
    );
    assert_ne!(open, rest);
}

/// From TSX, a library `Button` as the trigger (it takes the press before
/// the trigger slot's view) opens and closes it, lit while open; a button
/// in it is not its trigger; a plain view as the trigger still works.
#[test]
fn from_tsx_a_button_trigger_opens_and_closes_it() {
    let mut app = common::start_tsx("tests/tsx/popover.tsx", true);
    assert!(app.shows("Open at first"));
    app.press("role:button[Held]").unwrap();
    app.run_for(Duration::from_millis(400)).unwrap();
    assert!(!app.shows("Open at first"), "it closes one open at first");

    let rest = common::fill(&app, "role:button[Share]");
    app.press("role:button[Share]").unwrap();
    app.run_for(Duration::from_millis(300)).unwrap();
    assert!(app.shows("Anyone with the link"));
    app.hover("id:away").unwrap();
    app.run_for(Duration::from_millis(300)).unwrap();
    let open = common::fill(&app, "role:button[Share]");
    assert_eq!(
        open,
        light(nsh_longbridge::components::button::BUTTON_OUTLINE_HOVER)
    );
    assert_ne!(open, rest);

    app.press("id:copy").unwrap();
    app.run_for(Duration::from_millis(400)).unwrap();
    assert!(
        app.shows("Anyone with the link"),
        "a button in it keeps it open"
    );
    app.press("role:button[Share]").unwrap();
    app.run_for(Duration::from_millis(400)).unwrap();
    assert!(
        !app.shows("Anyone with the link"),
        "a second press closes it"
    );

    app.press("id:plain").unwrap();
    app.run_for(Duration::from_millis(300)).unwrap();
    assert!(app.shows("From a view"));
}
