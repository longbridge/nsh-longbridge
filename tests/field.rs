//! `Field` and its parts in a headless engine: the vertical, horizontal and
//! responsive layouts (a responsive field never shown vertical first in a
//! wide group), a choice control centered on the text beside it, the
//! field's state reaching its label and control, the description and the
//! error describing the control, `FieldError`'s messages, choice cards,
//! sets, legends and separators, and what TSX apps get.

mod common;

use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
    time::Duration,
};

use common::{bounds, fill, light, size, start, start_tsx};
use nsh_core::{Checked, Command, MountItem, NodeId, Size as Viewport, platform::Os, prelude::*};
use nsh_engine::EngineConfig;
use nsh_longbridge::{
    FieldErrors, FieldLegendVariant, FieldOrientation, checkbox,
    components::field::{FIELD_CHOICE_CARD_CHECKED, FIELD_LABEL_FOREGROUND_INVALID},
    field, field_content, field_description, field_error, field_group, field_label, field_legend,
    field_separator, field_set, field_title, input, radio_group, radio_group_item, switch,
};
use nsh_testing::Headless;

/// A vertical field, a horizontal one with content and a switch, a
/// horizontal checkbox field, an invalid one and a disabled one.
#[component]
fn Layouts() -> impl IntoElement {
    let shared = signal(false);
    let same = signal(false);
    let log = signal(Vec::<String>::new());
    view()
        .p(px(20.0))
        .w(px(400.0))
        .child(
            field_group()
                .child(
                    field()
                        .child(field_label().child(text("Name")))
                        .child(
                            input()
                                .accessibility_label("Name box")
                                .on_focus(move || log.update(|l| l.push("name".into()))),
                        )
                        .child(field_description().child(text("Your full name."))),
                )
                .child(
                    field()
                        .orientation(FieldOrientation::Horizontal)
                        .child(
                            field_content()
                                .child(field_label().child(text("Share")))
                                .child(field_description().child(text("Across devices."))),
                        )
                        .child(
                            switch()
                                .checked(shared)
                                .on_change(move |c: &bool| shared.set(*c))
                                .accessibility_label("Share switch"),
                        ),
                )
                .child(
                    field()
                        .orientation(FieldOrientation::Horizontal)
                        .child(
                            checkbox()
                                .checked(same)
                                .on_change(move |c: &bool| same.set(*c))
                                .accessibility_label("Same box"),
                        )
                        .child(field_label().child(text("Same as shipping"))),
                )
                .child(
                    field()
                        .invalid(true)
                        .child(field_label().child(text("Email")))
                        .child(input().accessibility_label("Email box"))
                        .child(field_error().child(text("Enter an email."))),
                )
                .child(
                    field()
                        .disabled(true)
                        .child(field_label().child(text("Locked")))
                        .child(
                            input()
                                .accessibility_label("Locked box")
                                .on_focus(move || log.update(|l| l.push("locked".into()))),
                        ),
                ),
        )
        .child(text(move || {
            format!(
                "shared:{} same:{} log:{}",
                shared.get() as u8,
                same.get() as u8,
                log.get().join(",")
            )
        }))
}

#[test]
fn a_vertical_field_stacks_its_parts_12_apart() {
    let app = start::<Layouts>(true);
    let label = bounds(&app, "text:Name");
    let field = bounds(&app, "label:Name box");
    assert_eq!(label.origin.x, field.origin.x);
    // The label's line (20), then the gap.
    assert_eq!(field.origin.y - label.origin.y, px(20.0 + 12.0));
    // The control as wide as the field.
    assert_eq!(size(&app, "label:Name box").0, 360.0);
    // The description closer to the control: it ends the field (no pull).
    let description = bounds(&app, "text:Your full name.");
    assert_eq!(
        description.origin.y,
        field.origin.y + field.size.height + px(12.0)
    );
}

#[test]
fn a_horizontal_field_puts_its_control_beside_the_text() {
    let app = start::<Layouts>(true);
    let content = bounds(&app, "text:Share");
    let switch = bounds(&app, "role:switch[Share switch]");
    assert!(switch.origin.x > content.origin.x + px(100.0));
    // A label after a checkbox: beside it.
    let check = bounds(&app, "role:checkbox[Same box]");
    let label = bounds(&app, "text:Same as shipping");
    assert!(label.origin.x > check.origin.x);
}

#[test]
fn a_label_focuses_or_presses_its_fields_control() {
    let mut app = start::<Layouts>(true);
    let name = app.screen().id("label:Name box").unwrap();
    app.press("text:Name").unwrap();
    app.run_for(Duration::from_millis(50)).unwrap();
    assert!(
        app.screen()
            .commands()
            .iter()
            .any(|(id, c)| *id == name && matches!(c, Command::Focus))
    );
    app.press("text:Share").unwrap();
    app.wait_for("text:shared:1 same:0 log:name").unwrap();
    app.press("text:Same as shipping").unwrap();
    app.wait_for("text:shared:1 same:1 log:name").unwrap();
}

#[test]
fn an_invalid_fields_label_turns_red_and_its_control_invalid() {
    let app = start::<Layouts>(true);
    assert!(
        app.screen()
            .accessibility_state("label:Email box")
            .unwrap()
            .invalid
    );
    assert_eq!(
        app.screen()
            .accessibility_description("label:Email box")
            .unwrap()
            .as_deref(),
        Some("Enter an email.")
    );
    assert_eq!(
        app.screen()
            .accessibility_description("label:Name box")
            .unwrap()
            .as_deref(),
        Some("Your full name.")
    );
    assert!(app.screen().matches("role:alert").unwrap());
    let red = app.screen().span_style("text:Email").unwrap().color;
    assert_eq!(Some(red.0), light(FIELD_LABEL_FOREGROUND_INVALID));
}

#[test]
fn a_disabled_field_dims_its_label_and_disables_its_control() {
    let mut app = start::<Layouts>(true);
    assert!(!app.screen().enabled("label:Locked box").unwrap());
    let _ = app.press("text:Locked");
    app.run_for(Duration::from_millis(50)).unwrap();
    let locked = app.screen().id("label:Locked box").unwrap();
    assert!(
        !app.screen()
            .commands()
            .iter()
            .any(|(id, c)| *id == locked && matches!(c, Command::Focus))
    );
}

/// A label beside a full-width control, in a narrow row.
#[component]
fn Beside() -> impl IntoElement {
    view().p(px(20.0)).w(px(260.0)).child(
        field()
            .orientation(FieldOrientation::Horizontal)
            .child(field_label().child(text("Alert volume level")))
            .child(input().accessibility_label("Level box")),
    )
}

#[test]
fn a_label_beside_a_full_width_control_shares_the_row_and_wraps() {
    let app = start::<Beside>(true);
    let label = bounds(&app, "text:Alert volume level");
    let input = bounds(&app, "label:Level box");
    // The control shrank to share the row; the label wraps rather than
    // running under it.
    assert!(
        label.origin.x + label.size.width + px(12.0) <= input.origin.x + px(0.5),
        "{label:?} {input:?}"
    );
    assert!(label.size.height > px(20.0), "{label:?}");
    assert!(input.size.width > px(60.0), "{input:?}");
}

/// `FieldError`s: none, one message, several (one repeated, one empty),
/// and children over messages.
#[component]
fn Errors() -> impl IntoElement {
    let many = signal(FieldErrors::from_iter([
        "Too short.",
        "",
        "No digits.",
        "Too short.",
    ]));
    view()
        .p(px(20.0))
        .w(px(400.0))
        .child(field_error().errors(FieldErrors::default()))
        .child(field_error().errors(FieldErrors::from_iter(["Only one."])))
        .child(field_error().errors(move || many.get()))
        .child(
            field_error()
                .errors(FieldErrors::from_iter(["Hidden."]))
                .child(text("Children win.")),
        )
        .child(
            view()
                .on_press(move || many.set(FieldErrors::default()))
                .child(text("Fix")),
        )
}

#[test]
fn field_error_shows_its_messages_once_each_and_nothing_without() {
    let mut app = start::<Errors>(true);
    let screen = app.screen();
    assert_eq!(screen.count("role:alert").unwrap(), 3);
    assert!(app.shows("Only one."));
    assert_eq!(app.screen().count("text:Too short.").unwrap(), 1);
    assert!(app.shows("No digits."));
    assert_eq!(app.screen().count("text:•").unwrap(), 2);
    assert!(app.shows("Children win."));
    assert!(!app.shows("Hidden."));
    app.press("text:Fix").unwrap();
    app.run_for(Duration::from_millis(50)).unwrap();
    assert_eq!(app.screen().count("role:alert").unwrap(), 2);
}

/// Choice cards: radios and a checkbox, each card a whole option.
#[component]
fn Cards() -> impl IntoElement {
    let plan = signal(None::<SharedString>);
    let backup = signal(false);
    let card = |title: &'static str, value: &'static str| {
        field_label().child(
            field()
                .orientation(FieldOrientation::Horizontal)
                .child(
                    field_content()
                        .child(field_title().child(text(title)))
                        .child(field_description().child(text("A plan."))),
                )
                .child(radio_group_item().value(value).accessibility_label(title)),
        )
    };
    view()
        .p(px(20.0))
        .w(px(400.0))
        .gap(px(8.0))
        .child(
            radio_group()
                .value(plan)
                .on_change(move |v: &SharedString| plan.set(Some(v.clone())))
                .accessibility_label("Plan")
                .child(card("Kubernetes", "k8s"))
                .child(card("Virtual machine", "vm")),
        )
        .child(
            field_label().child(
                field()
                    .orientation(FieldOrientation::Horizontal)
                    .child(
                        checkbox()
                            .checked(backup)
                            .on_change(move |c: &bool| backup.set(*c))
                            .accessibility_label("Backup"),
                    )
                    .child(field_content().child(field_title().child(text("Daily backup")))),
            ),
        )
        .child(text(move || {
            format!(
                "plan:{} backup:{}",
                plan.get().unwrap_or_default(),
                backup.get() as u8
            )
        }))
}

/// The card around the text `selector` names: its first ancestor that
/// starts past the padding (16) left of it (the field inside the card
/// starts at the edge's inside, the parts inside it with the text).
fn card_of(app: &Headless, selector: &str) -> String {
    let nodes = app.screen().layout_nodes();
    let x = bounds(app, selector).origin.x;
    let mut at = app.screen().id(selector).unwrap();
    loop {
        let parent = nodes.iter().find(|n| n.id == at).unwrap().parent;
        if bounds(app, &format!("view:{parent}")).origin.x < x - px(16.5) {
            return format!("view:{parent}");
        }
        at = parent;
    }
}

#[test]
fn a_choice_card_presses_its_control_and_shows_it_checked() {
    let mut app = start::<Cards>(true);
    // The edge (1), then the padding (16).
    let card = bounds(&app, &card_of(&app, "text:Kubernetes"));
    let title = bounds(&app, "text:Kubernetes");
    assert_eq!(title.origin.x - card.origin.x, px(17.0));
    assert_eq!(fill(&app, &card_of(&app, "text:Kubernetes")), None);
    app.press("text:A plan. >> nth=0").unwrap();
    app.wait_for("text:plan:k8s backup:0").unwrap();
    app.run_for(Duration::from_millis(50)).unwrap();
    assert_eq!(
        fill(&app, &card_of(&app, "text:Kubernetes")),
        light(FIELD_CHOICE_CARD_CHECKED)
    );
    // The control's own press: once.
    app.press("role:checkbox[Backup]").unwrap();
    app.wait_for("text:plan:k8s backup:1").unwrap();
    app.press("text:Daily backup").unwrap();
    app.wait_for("text:plan:k8s backup:0").unwrap();
    assert_eq!(
        app.screen()
            .accessibility_state("role:checkbox[Backup]")
            .unwrap()
            .checked,
        Some(Checked::False)
    );
}

/// A disabled set with a legend, a description and a group, and a
/// separator with a word.
#[component]
fn Sets() -> impl IntoElement {
    view()
        .p(px(20.0))
        .w(px(400.0))
        .child(
            field_set()
                .disabled(true)
                .accessibility_label("Payment")
                .child(field_legend().child(text("Payment")))
                .child(field_description().child(text("Secure.")))
                .child(
                    field_group().child(
                        field()
                            .child(field_label().child(text("Card")))
                            .child(input().accessibility_label("Card box")),
                    ),
                ),
        )
        .child(
            field_legend()
                .variant(FieldLegendVariant::Label)
                .child(text("Small legend")),
        )
        .child(field_separator().child(text("Or")))
}

#[test]
fn a_set_disables_its_fields_and_spaces_its_parts() {
    let app = start::<Sets>(true);
    assert!(app.screen().matches("role:group[Payment]").unwrap());
    assert!(!app.screen().enabled("label:Card box").unwrap());
    let legend = bounds(&app, "text:Payment");
    let description = bounds(&app, "text:Secure.");
    // The legend's line (24), its margin (12), the set's gap (24), pulled
    // up 6.
    assert_eq!(legend.size.height, px(24.0));
    assert_eq!(
        description.origin.y - legend.origin.y,
        px(24.0 + 12.0 + 24.0 - 6.0)
    );
    assert_eq!(size(&app, "text:Small legend").1, 20.0);
}

#[test]
fn a_separator_is_20_tall_with_its_word_on_the_line() {
    let app = start::<Sets>(true);
    let word = bounds(&app, "text:Or");
    assert_eq!(word.size.height, px(20.0));
    // Centered on the 360-wide line.
    assert!((word.origin.x + word.size.width / 2.0 - px(200.0)).0.abs() < 1.0);
}

/// A responsive field in a wide group and in a narrow one.
#[component]
fn Responsive() -> impl IntoElement {
    let one = |name: &'static str| {
        field()
            .orientation(FieldOrientation::Responsive)
            .child(
                field_content()
                    .child(field_label().child(text(name)))
                    .child(field_description().child(text("Help."))),
            )
            .child(input().accessibility_label(name))
    };
    view()
        .p(px(20.0))
        .child(view().w(px(600.0)).child(field_group().child(one("Wide"))))
        .child(
            view()
                .w(px(300.0))
                .child(field_group().child(one("Narrow"))),
        )
}

#[test]
fn a_responsive_field_is_horizontal_from_448_and_never_shown_vertical_first() {
    let mut config = EngineConfig {
        viewport: Viewport::new(px(800.0), px(600.0)),
        clock: nsh_engine::Clock::manual(std::time::SystemTime::now()),
        ..EngineConfig::new(nsh_testing::windowed_root::<Responsive>())
    };
    config.environment.os = Os::MacOs;
    let mut app = Headless::start(config).unwrap();
    // Every frame each view is given, in order.
    let frames: Arc<Mutex<HashMap<NodeId, Vec<Bounds<Points>>>>> = Arc::default();
    let seen = frames.clone();
    let visible: Arc<Mutex<HashMap<NodeId, Vec<f32>>>> = Arc::default();
    let seen_visible = visible.clone();
    app.observe(move |_, batch| {
        for item in &batch.items {
            match item {
                MountItem::Frame { id, frame } => {
                    seen.lock().unwrap().entry(*id).or_default().push(*frame);
                }
                MountItem::ViewStyle { id, style } => {
                    seen_visible
                        .lock()
                        .unwrap()
                        .entry(*id)
                        .or_default()
                        .push(style.opacity);
                }
                _ => {}
            }
        }
    });
    app.first_render().unwrap();
    app.run_for(Duration::from_millis(50)).unwrap();
    let wide = bounds(&app, "label:Wide");
    let narrow = bounds(&app, "label:Narrow");
    let wide_label = bounds(&app, "text:Wide");
    let narrow_label = bounds(&app, "text:Narrow");
    // Wide: beside its text; narrow: under it.
    assert_eq!(wide.origin.y, wide_label.origin.y);
    assert!(narrow.origin.y > narrow_label.origin.y);
    // Every frame the wide field's input was given puts it beside its
    // text (at the field's top, past the text), from the first: never
    // under it.
    let input = app.screen().id("label:Wide").unwrap();
    let frames = frames.lock().unwrap();
    let placed = frames.get(&input).cloned().unwrap_or_default();
    assert!(!placed.is_empty());
    for frame in &placed {
        assert!(
            frame.origin.y == px(0.0) && frame.origin.x > px(0.0),
            "{placed:?}"
        );
    }
}

#[test]
fn a_tsx_field_error_with_a_null_child_makes_nothing() {
    let mut app = start_tsx("tests/tsx/field.tsx", true);
    app.run_for(Duration::from_millis(50)).unwrap();
    assert!(!app.screen().matches("role:alert").unwrap());
    assert_eq!(
        app.screen()
            .accessibility_description("label:Name box")
            .unwrap()
            .as_deref(),
        Some("Your full name.")
    );
    app.press("role:button[Clear]").unwrap();
    app.wait_for("text:Enter your name.").unwrap();
    assert!(app.screen().matches("role:alert").unwrap());
    assert!(
        app.screen()
            .accessibility_state("label:Name box")
            .unwrap()
            .invalid
    );
    assert_eq!(
        app.screen()
            .accessibility_description("label:Name box")
            .unwrap()
            .as_deref(),
        Some("Enter your name.")
    );
    // The label focuses the input without a ref.
    app.press("text:Name").unwrap();
    app.run_for(Duration::from_millis(50)).unwrap();
    let input = app.screen().id("label:Name box").unwrap();
    assert!(
        app.screen()
            .commands()
            .iter()
            .any(|(id, c)| *id == input && matches!(c, Command::Focus))
    );
}

/// TSX apps' declarations: every part, the orientation, and the errors'
/// shape (what validation libraries give).
#[test]
fn tsx_gets_the_field_parts() {
    let registry = nsh_longbridge::registry();
    assert_eq!(registry.undocumented(), Vec::<String>::new());
    let ts = nsh_core::element::declarations(&registry, &[]);
    let props = |name: &str| {
        let start = ts
            .find(&format!("export interface {name}Props"))
            .unwrap_or_else(|| panic!("{name}Props"));
        ts[start..start + ts[start..].find("\n}").unwrap()].to_owned()
    };
    for part in [
        "Field",
        "FieldLabel",
        "FieldDescription",
        "FieldError",
        "FieldContent",
        "FieldTitle",
        "FieldGroup",
        "FieldSet",
        "FieldLegend",
        "FieldSeparator",
    ] {
        props(part);
    }
    let field = props("Field");
    assert!(field.contains("orientation?:"), "{field}");
    let error = props("FieldError");
    assert!(
        error.contains("errors?: ({ message?: string | null } | null | undefined)[]"),
        "{error}"
    );
    assert!(props("FieldLabel").contains("htmlFor?:"));
}
