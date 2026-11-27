//! The field integration (`foundation::field`, `foundation::field_ref`) in a
//! headless engine: a field's `invalid` and `disabled` reach the control
//! inside it (or'ed with its own), its description or error describes it,
//! one control joins a field, a set's `disabled` reaches every control, and
//! a label's press focuses a text control and presses a checkbox-like one,
//! through a field or a ref. Outside a field nothing changes (each
//! control's own tests).

mod common;

use std::time::Duration;

use common::start;
use nsh_core::{Checked, Command, prelude::*};
use nsh_longbridge::{
    Checkbox, FieldRef, SelectItem, Switch, checkbox,
    foundation::field::{self, FieldState},
    input, label, radio_group, radio_group_item, select, switch, textarea,
};
use nsh_testing::Headless;

/// The props of [`TestField`].
#[derive(Debug, Clone, PartialEq, Props)]
struct TestFieldProps {
    #[prop(default)]
    invalid: Prop<bool>,
    #[prop(default)]
    disabled: Prop<bool>,
    /// The label's words.
    #[prop(default)]
    name: Prop<SharedString>,
    /// A description under the control.
    description: Prop<Option<SharedString>>,
    /// An error under the control, shown while set.
    error: Prop<Option<SharedString>>,
    children: Prop<Children>,
}

/// A field as `Field` makes one: a label naming the field's control, the
/// control, and its description and error registered with the field.
#[component]
fn TestField(props: TestFieldProps, _cx: Scope<NoEvent>) -> impl IntoElement {
    let state = field::provide(move || props.invalid.get(), move || props.disabled.get());
    view()
        .child(
            label()
                .html_for(move || state.control().map(FieldRef::node).unwrap_or_default())
                .child(text(props.name)),
        )
        .child(props.children)
        .child(dynamic(move || {
            props.description.get().map(|words| {
                let r = element_ref();
                state.register_description(r);
                text(words).element_ref(&r)
            })
        }))
        .child(dynamic(move || {
            props.error.get().map(|words| {
                let r = element_ref();
                state.register_error(r);
                text(words).element_ref(&r)
            })
        }))
}

/// The props of [`TestSet`].
#[derive(Debug, Clone, PartialEq, Props)]
struct TestSetProps {
    #[prop(default)]
    disabled: Prop<bool>,
    children: Prop<Children>,
}

/// A set as `FieldSet` makes one.
#[component]
fn TestSet(props: TestSetProps, _cx: Scope<NoEvent>) -> impl IntoElement {
    field::provide_set(move || props.disabled.get());
    view().child(props.children)
}

fn note(log: Signal<Vec<String>>, s: impl Into<String>) {
    let s = s.into();
    log.update(|l| l.push(s));
}

/// Text fields in fields: an invalid one with an error, a disabled one, one
/// with a description, a field holding two inputs, and a field whose own
/// `invalid` is off while the input's is on.
#[component]
fn TextFields() -> impl IntoElement {
    let error = signal(Some(SharedString::from("Enter your name.")));
    let log = signal(Vec::<String>::new());
    view()
        .child(
            test_field()
                .name("Name")
                .invalid(move || error.get().is_some())
                .error(move || error.get())
                .description("Your full name.")
                .child(
                    input()
                        .accessibility_label("Name box")
                        .on_focus(move || note(log, "name")),
                ),
        )
        .child(
            test_field().name("Locked").disabled(true).child(
                input()
                    .accessibility_label("Locked box")
                    .on_focus(move || note(log, "locked")),
            ),
        )
        .child(
            test_field()
                .name("Notes")
                .description("Anything else.")
                .child(textarea().accessibility_label("Notes box")),
        )
        .child(
            test_field()
                .name("Two")
                .invalid(true)
                .child(input().accessibility_label("First box"))
                .child(input().accessibility_label("Second box")),
        )
        .child(
            test_field()
                .name("Own")
                .child(input().invalid(true).accessibility_label("Own box")),
        )
        .child(view().on_press(move || error.set(None)).child(text("Fix")))
        .child(text(move || format!("log:{}", log.get().join(","))))
}

#[test]
fn a_fields_invalid_and_disabled_reach_its_control() {
    let app = start::<TextFields>(true);
    let screen = app.screen();
    assert!(
        screen
            .accessibility_state("label:Name box")
            .unwrap()
            .invalid
    );
    assert!(!screen.enabled("label:Locked box").unwrap());
    assert!(screen.enabled("label:Notes box").unwrap());
    // The control's own `invalid` stays on in a valid field.
    assert!(screen.accessibility_state("label:Own box").unwrap().invalid);
}

#[test]
fn one_control_joins_a_field() {
    let app = start::<TextFields>(true);
    let screen = app.screen();
    assert!(
        screen
            .accessibility_state("label:First box")
            .unwrap()
            .invalid
    );
    assert!(
        !screen
            .accessibility_state("label:Second box")
            .unwrap()
            .invalid
    );
}

#[test]
fn the_error_while_it_shows_else_the_description_describes_the_control() {
    let mut app = start::<TextFields>(true);
    assert_eq!(
        app.screen()
            .accessibility_description("label:Name box")
            .unwrap()
            .as_deref(),
        Some("Enter your name.")
    );
    assert_eq!(
        app.screen()
            .accessibility_description("label:Notes box")
            .unwrap()
            .as_deref(),
        Some("Anything else.")
    );
    app.press("text:Fix").unwrap();
    app.run_for(Duration::from_millis(50)).unwrap();
    let screen = app.screen();
    assert!(
        !screen
            .accessibility_state("label:Name box")
            .unwrap()
            .invalid
    );
    assert_eq!(
        screen
            .accessibility_description("label:Name box")
            .unwrap()
            .as_deref(),
        Some("Your full name.")
    );
}

#[test]
fn a_fields_label_focuses_its_text_control_unless_disabled() {
    let mut app = start::<TextFields>(true);
    let _ = app.press("text:Locked");
    app.press("text:Name").unwrap();
    app.wait_for("text:log:name").unwrap();
}

/// Choice controls: a checkbox and a switch in fields, a disabled
/// checkbox field, a checkbox and a switch named by a plain label's ref,
/// and a radio group in an invalid, then disabled, field.
#[component]
fn Choices() -> impl IntoElement {
    let terms = signal(false);
    let wifi = signal(false);
    let off = signal(false);
    let news = signal(false);
    let dark = signal(false);
    let plan = signal(None::<SharedString>);
    let locked = signal(false);
    let news_ref = component_ref::<Checkbox>();
    let dark_ref = component_ref::<Switch>();
    view()
        .child(
            test_field().name("Accept terms").child(
                checkbox()
                    .checked(terms)
                    .on_change(move |c: &bool| terms.set(*c))
                    .accessibility_label("Terms"),
            ),
        )
        .child(
            test_field().name("Wi-Fi").child(
                switch()
                    .checked(wifi)
                    .on_change(move |c: &bool| wifi.set(*c))
                    .accessibility_label("Wifi"),
            ),
        )
        .child(
            test_field().name("Disabled box").disabled(true).child(
                checkbox()
                    .checked(off)
                    .on_change(move |c: &bool| off.set(*c))
                    .accessibility_label("Off"),
            ),
        )
        .child(label().html_for(news_ref).child("Newsletter"))
        .child(
            checkbox()
                .component_ref(&news_ref)
                .checked(news)
                .on_change(move |c: &bool| news.set(*c))
                .accessibility_label("News"),
        )
        .child(label().html_for(dark_ref).child("Dark mode"))
        .child(
            switch()
                .component_ref(&dark_ref)
                .checked(dark)
                .on_change(move |c: &bool| dark.set(*c))
                .accessibility_label("Dark"),
        )
        .child(
            test_field()
                .name("Plan")
                .invalid(true)
                .disabled(locked)
                .child(
                    radio_group()
                        .value(plan)
                        .on_change(move |v: &SharedString| plan.set(Some(v.clone())))
                        .accessibility_label("Plan")
                        .child(radio_group_item().value("free").accessibility_label("Free"))
                        .child(radio_group_item().value("pro").accessibility_label("Pro")),
                ),
        )
        .child(
            view()
                .on_press(move || locked.set(true))
                .child(text("Lock")),
        )
        .child(text(move || {
            format!(
                "state:{}{}{}{}{}:{}",
                terms.get() as u8,
                wifi.get() as u8,
                off.get() as u8,
                news.get() as u8,
                dark.get() as u8,
                plan.get().unwrap_or_default()
            )
        }))
}

fn checked(app: &Headless, selector: &str) -> Option<Checked> {
    app.screen().accessibility_state(selector).unwrap().checked
}

#[test]
fn a_label_presses_a_checkbox_like_control() {
    let mut app = start::<Choices>(true);
    app.press("text:Accept terms").unwrap();
    app.wait_for("text:state:10000:").unwrap();
    assert_eq!(checked(&app, "role:checkbox[Terms]"), Some(Checked::True));
    app.press("text:Wi-Fi").unwrap();
    app.wait_for("text:state:11000:").unwrap();
    app.press("text:Accept terms").unwrap();
    app.wait_for("text:state:01000:").unwrap();
}

#[test]
fn a_plain_labels_ref_presses_a_checkbox_or_a_switch() {
    let mut app = start::<Choices>(true);
    app.press("text:Newsletter").unwrap();
    app.wait_for("text:state:00010:").unwrap();
    app.press("text:Dark mode").unwrap();
    app.wait_for("text:state:00011:").unwrap();
}

#[test]
fn a_disabled_fields_label_presses_nothing() {
    let mut app = start::<Choices>(true);
    assert!(!app.screen().enabled("role:checkbox[Off]").unwrap());
    let _ = app.press("text:Disabled box");
    app.run_for(Duration::from_millis(50)).unwrap();
    assert!(app.shows("state:00000:"));
}

#[test]
fn a_radio_group_takes_its_fields_state() {
    let mut app = start::<Choices>(true);
    assert!(app.screen().enabled("role:radio[Free]").unwrap());
    // The label reaches no item.
    let _ = app.press("text:Plan");
    app.run_for(Duration::from_millis(50)).unwrap();
    assert!(app.shows("state:00000:"));
    app.press("text:Lock").unwrap();
    app.run_for(Duration::from_millis(50)).unwrap();
    assert!(!app.screen().enabled("role:radio[Free]").unwrap());
    assert!(!app.screen().enabled("role:radio[Pro]").unwrap());
}

/// A choice card: a radio item in a field (as a `FieldLabel` around a
/// `Field` makes one), the field's label pressing it.
#[component]
fn Card() -> impl IntoElement {
    let plan = signal(None::<SharedString>);
    view()
        .child(
            radio_group()
                .value(plan)
                .on_change(move |v: &SharedString| plan.set(Some(v.clone())))
                .accessibility_label("Plan")
                .child(
                    test_field()
                        .name("Kubernetes")
                        .child(radio_group_item().value("k8s").accessibility_label("K8s")),
                )
                .child(
                    test_field()
                        .name("Virtual machine")
                        .child(radio_group_item().value("vm").accessibility_label("VM")),
                ),
        )
        .child(text(move || {
            format!("plan:{}", plan.get().unwrap_or_default())
        }))
}

#[test]
fn a_choice_cards_label_checks_its_radio() {
    let mut app = start::<Card>(true);
    app.press("text:Virtual machine").unwrap();
    app.wait_for("text:plan:vm").unwrap();
    app.press("text:Kubernetes").unwrap();
    app.wait_for("text:plan:k8s").unwrap();
}

/// A select in a field with a description, and a disabled one.
#[component]
fn Selects() -> impl IntoElement {
    let items = || {
        vec![
            SelectItem::new("nl", "Netherlands"),
            SelectItem::new("jp", "Japan"),
        ]
    };
    view()
        .child(
            test_field()
                .name("Country")
                .description("Where you live.")
                .invalid(true)
                .child(select().items(items()).accessibility_label("Country")),
        )
        .child(
            test_field()
                .name("Region")
                .disabled(true)
                .child(select().items(items()).accessibility_label("Region")),
        )
}

#[test]
fn a_select_takes_its_fields_state_and_its_label_focuses_the_trigger() {
    let mut app = start::<Selects>(true);
    let screen = app.screen();
    assert!(
        screen
            .accessibility_state("role:combobox[Country]")
            .unwrap()
            .invalid
    );
    assert_eq!(
        screen
            .accessibility_description("role:combobox[Country]")
            .unwrap()
            .as_deref(),
        Some("Where you live.")
    );
    assert!(!screen.enabled("role:combobox[Region]").unwrap());
    let trigger = screen.id("role:combobox[Country]").unwrap();
    app.press("text:Country").unwrap();
    app.run_for(Duration::from_millis(50)).unwrap();
    assert!(
        app.screen()
            .commands()
            .iter()
            .any(|(id, c)| *id == trigger && matches!(c, Command::Focus))
    );
    // Focused, not opened.
    assert!(!app.screen().matches("role:listbox").unwrap());
}

/// Controls in a disabled set, without fields, and one in a field in it.
#[component]
fn Set() -> impl IntoElement {
    test_set()
        .disabled(true)
        .child(input().accessibility_label("Street box"))
        .child(checkbox().accessibility_label("Gift"))
        .child(
            test_field()
                .name("City")
                .child(input().accessibility_label("City box")),
        )
}

#[test]
fn a_sets_disabled_reaches_every_control() {
    let app = start::<Set>(true);
    let screen = app.screen();
    assert!(!screen.enabled("label:Street box").unwrap());
    assert!(!screen.enabled("role:checkbox[Gift]").unwrap());
    assert!(!screen.enabled("label:City box").unwrap());
}

/// Says whether it is made in a field, as the field's parts read it.
#[component]
fn Probe() -> impl IntoElement {
    let state: Option<FieldState> = field::current();
    text(format!("in-field:{}", state.is_some()))
}

/// A probe in a field and one outside.
#[component]
fn Current() -> impl IntoElement {
    view()
        .child(test_field().name("Inside").child(probe()))
        .child(probe())
}

#[test]
fn current_finds_the_enclosing_field() {
    let app = start::<Current>(true);
    assert_eq!(app.screen().count("text:in-field:true").unwrap(), 1);
    assert_eq!(app.screen().count("text:in-field:false").unwrap(), 1);
}

/// TSX apps' declarations: the refs a label takes, each with `focus()`.
#[test]
fn tsx_labels_take_every_controls_ref() {
    let ts = nsh_core::element::declarations(&nsh_longbridge::registry(), &[]);
    assert!(
        ts.contains("htmlFor?: InputRef | TextareaRef | SelectRef | CheckboxRef | SwitchRef | DatePickerRef;"),
        "{ts}"
    );
    for name in ["SelectRef", "CheckboxRef", "SwitchRef", "DatePickerRef"] {
        let start = ts
            .find(&format!("export interface {name} {{"))
            .unwrap_or_else(|| panic!("no {name}"));
        let body = &ts[start..start + ts[start..].find('}').unwrap()];
        assert!(body.contains("focus(): void;"), "{body}");
    }
}
