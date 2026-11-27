//! `DatePicker` in a headless engine: the field opens the calendar and a
//! day picks and closes it, controlled and uncontrolled, a range, the
//! presets, the time row, Delete clearing; what screen readers get; the
//! field's sizes on pointer and touch screens (a sheet on touch screens);
//! joining a `Field`; a token changed while the popup is open reaching the
//! popup, a day and a segment; the time field's focus edge and ring; the
//! time label's color.

mod common;

use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
    time::Duration,
};

use common::{bounds, css, dark, light, size, start, start_dark};
use nsh_core::{Command, MountItem, NodeId, prelude::*};
use nsh_longbridge::{
    DatePicker, DatePickerEvent, DateRangePreset, DateTime, HourCycle, PickedDate, ScopeTokens,
    Size, TimePrecision, button,
    chrono::NaiveDate,
    components::{calendar::CALENDAR_SELECTED, date_picker::DATE_PICKER_SEGMENT_FOCUS},
    date_picker, field, field_label,
    foundation::field::{INPUT_BORDER, INPUT_BORDER_FOCUS, INPUT_PLACEHOLDER_FOREGROUND},
    label,
    theme::semantic::{POPOVER, POPOVER_FOREGROUND},
    theme_scope,
};
use nsh_testing::Headless;

fn d(y: i32, m: u32, day: u32) -> NaiveDate {
    NaiveDate::from_ymd_opt(y, m, day).unwrap()
}

fn day(iso: &str) -> String {
    format!("role:button[{iso}]")
}

fn combobox(name: &str) -> String {
    format!("role:combobox[{name}]")
}

fn open(app: &mut Headless, name: &str) {
    app.press(&combobox(name)).unwrap();
    app.run_for(Duration::from_millis(300)).unwrap();
}

/// A controlled picker writing down what it reports, an uncontrolled range
/// one with presets, and a date and time.
#[component]
fn Pickers() -> impl IntoElement {
    let due = signal(DateTime::from(d(2026, 10, 5)));
    let log = signal(Vec::<String>::new());
    let note = move |s: String| log.update(|l| l.push(s));
    view()
        .pt(px(40.0))
        .pl(px(40.0))
        .items_start()
        .gap(px(8.0))
        .child(
            date_picker()
                .first_day_of_week(Some(0))
                .value(move || due.get())
                .accessibility_label("Due")
                .on_change(move |picked: &PickedDate| {
                    note(format!(
                        "change:{:?}",
                        picked.value.start().map(|v| v.date())
                    ));
                    due.set(picked.value);
                })
                .on_open_change(move |e: &DatePickerEvent| {
                    if let DatePickerEvent::OpenChange(open, reason) = e {
                        note(format!("open:{open}:{reason:?}"));
                    }
                }),
        )
        .child(
            date_picker()
                .range(true)
                .default_value(DateTime::from((d(2026, 12, 20), d(2027, 1, 3))))
                .presets(vec![
                    DateRangePreset::single("Launch", d(2026, 12, 1)),
                    DateRangePreset::range("Holidays", d(2026, 12, 24), d(2026, 12, 26)),
                ])
                .accessibility_label("Stay"),
        )
        .child(
            date_picker()
                .time_precision(Some(TimePrecision::Minute))
                .hour_cycle(Some(HourCycle::H23))
                .date_format(Some("%Y-%m-%d".into()))
                .default_value(DateTime::from(
                    d(2026, 11, 26).and_hms_opt(9, 30, 0).unwrap(),
                ))
                .accessibility_label("Meeting"),
        )
        .child(date_picker().accessibility_label("Empty"))
        .child(view().test_id("away").w(px(200.0)).h(px(40.0)))
        .child(text(move || format!("log:{}", log.get().join(","))))
}

#[test]
fn a_day_picks_and_closes_the_popup() {
    let mut app = start::<Pickers>(true);
    assert!(app.shows("Oct 5, 2026"));
    open(&mut app, "Due");
    app.press(&day("2026-10-14")).unwrap();
    app.wait_for("text:log:open:true:TriggerPress,change:Some(2026-10-14),open:false:Select")
        .unwrap();
    app.run_for(Duration::from_millis(400)).unwrap();
    assert!(!app.screen().matches(&day("2026-10-14")).unwrap());
    assert!(app.shows("Oct 14, 2026"));
}

#[test]
fn an_uncontrolled_range_keeps_its_own_value() {
    let mut app = start::<Pickers>(true);
    assert!(app.shows("Dec 20, 2026 – Jan 3, 2027"));
    open(&mut app, "Stay");
    app.press(&day("2026-12-07")).unwrap();
    app.press(&day("2026-12-09")).unwrap();
    app.run_for(Duration::from_millis(400)).unwrap();
    assert!(app.shows("Dec 7, 2026 – Dec 9, 2026"));
}

/// What screen readers hear as the value of the combobox `name`.
fn value_of(app: &Headless, name: &str) -> Option<String> {
    app.screen()
        .accessibility(Some(&combobox(name)))
        .unwrap()
        .unwrap()
        .value
}

#[test]
fn a_range_reads_with_the_value_range_dash() {
    let app = start::<Pickers>(true);
    // The field's ends joined by `value-range` (`{start} – {end}`), as a
    // slider's two values; a single date unchanged.
    assert!(app.shows("Dec 20, 2026 – Jan 3, 2027"));
    assert!(!app.shows("Dec 20, 2026 - Jan 3, 2027"));
    assert!(app.shows("Oct 5, 2026"));
    // `nsh/base` writes its combobox's value itself, with ` - ` (NSH gaps
    // 15): this fails once it takes the library's words, and the gap goes.
    assert_eq!(
        value_of(&app, "Stay").as_deref(),
        Some("Dec 20, 2026 - Jan 3, 2027")
    );
}

/// The ranges of a default and a `confirm` picker, under a translator of
/// `value-range`.
#[component]
fn TranslatedRanges() -> impl IntoElement {
    nsh_longbridge::set_translator(|label| {
        (label == nsh_longbridge::labels::Label::ValueRange).then(|| "{start} to {end}".into())
    });
    view()
        .pt(px(40.0))
        .pl(px(40.0))
        .items_start()
        .gap(px(8.0))
        .child(
            date_picker()
                .range(true)
                .default_value(DateTime::from((d(2026, 12, 20), d(2027, 1, 3))))
                .accessibility_label("Stay"),
        )
        .child(
            date_picker()
                .range(true)
                .confirm(true)
                .default_value(DateTime::from((d(2026, 10, 5), d(2026, 10, 9))))
                .presets(vec![DateRangePreset::range(
                    "Holidays",
                    d(2026, 12, 24),
                    d(2026, 12, 26),
                )])
                .accessibility_label("Period"),
        )
}

#[test]
fn a_ranges_dash_is_translatable() {
    let mut app = start::<TranslatedRanges>(true);
    assert!(app.shows("Dec 20, 2026 to Jan 3, 2027"));
    assert!(app.shows("Oct 5, 2026 to Oct 9, 2026"));
    // With `confirm` the library makes the combobox: its value too.
    assert_eq!(
        value_of(&app, "Period").as_deref(),
        Some("Oct 5, 2026 to Oct 9, 2026")
    );
    // A preset's dates under its label.
    open(&mut app, "Period");
    assert!(app.shows("Dec 24, 2026 to Dec 26, 2026"));
}

#[test]
fn a_preset_sets_the_value_and_closes_it() {
    let mut app = start::<Pickers>(true);
    open(&mut app, "Stay");
    assert!(app.screen().matches("role:button[Launch]").unwrap());
    app.press("role:button[Holidays]").unwrap();
    app.run_for(Duration::from_millis(400)).unwrap();
    assert!(!app.screen().matches("role:button[Holidays]").unwrap());
    assert!(app.shows("Dec 24, 2026 – Dec 26, 2026"));
}

#[test]
fn the_time_row_edits_the_time_and_the_field_shows_it() {
    let mut app = start::<Pickers>(true);
    assert!(app.shows("2026-11-26 09:30"));
    open(&mut app, "Meeting");
    assert!(app.shows("Time"));
    assert!(app.screen().matches("label:Time").unwrap());
    // On a desktop the popup stays open for the time.
    app.press(&day("2026-11-27")).unwrap();
    app.run_for(Duration::from_millis(100)).unwrap();
    assert!(app.screen().matches(&day("2026-11-27")).unwrap());
    app.type_text("label:Time", "1").unwrap();
    app.type_text("label:Time", "4").unwrap();
    app.run_for(Duration::from_millis(100)).unwrap();
    assert!(app.shows("2026-11-27 14:30"), "{:?}", app.screen().texts());
}

/// The time row's label takes the popup's text color, in the dark
/// appearance too (by default a text is black).
#[test]
fn the_time_label_takes_the_popup_foreground() {
    let (mut app, _) = start_dark::<Pickers>();
    open(&mut app, "Meeting");
    let label = app.screen().span_style("text:Time").unwrap();
    assert_eq!(label.color, dark(POPOVER_FOREGROUND));
}

#[test]
fn screen_readers_get_a_combobox_with_the_value() {
    let mut app = start::<Pickers>(true);
    let state = app.screen().accessibility_state(&combobox("Due")).unwrap();
    assert_eq!(state.expanded, Some(false));
    assert!(state.has_popup);
    let node = app
        .screen()
        .accessibility(Some(&combobox("Due")))
        .unwrap()
        .unwrap();
    assert_eq!(node.value.as_deref(), Some("Oct 5, 2026"));
    open(&mut app, "Due");
    assert_eq!(
        app.screen()
            .accessibility_state(&combobox("Due"))
            .unwrap()
            .expanded,
        Some(true)
    );
    assert!(app.screen().matches("role:columnheader").unwrap());
}

#[test]
fn delete_on_the_closed_field_clears_it() {
    let mut app = start::<Pickers>(true);
    // The first stop of Tab.
    app.press_key("tab").unwrap();
    app.press_key("delete").unwrap();
    app.wait_for("text:/change:None/").unwrap();
    assert!(app.shows("Pick a date"));
}

#[test]
fn without_a_value_the_field_shows_the_placeholder_muted() {
    let app = start::<Pickers>(true);
    assert!(app.shows("Pick a date"));
    let color = app
        .screen()
        .span_style(&format!("{} >> text:Pick a date", combobox("Empty")))
        .unwrap()
        .color;
    assert_eq!(Some(color.0), light(INPUT_PLACEHOLDER_FOREGROUND));
}

/// Pickers of each size, open from the start (the medium one).
#[component]
fn Sizes() -> impl IntoElement {
    view()
        .items_start()
        .child(date_picker().size(Size::Xs).accessibility_label("xs"))
        .child(date_picker().size(Size::Sm).accessibility_label("sm"))
        .child(
            date_picker()
                .default_value(DateTime::from(d(2026, 10, 5)))
                .default_open(true)
                .accessibility_label("md"),
        )
        .child(date_picker().size(Size::Lg).accessibility_label("lg"))
}

#[test]
fn the_field_takes_inputs_heights_and_the_cells_the_calendars() {
    let mut desktop = start::<Sizes>(true);
    desktop.run_for(Duration::from_millis(300)).unwrap();
    for (name, h) in [("xs", 24.0), ("sm", 28.0), ("md", 32.0), ("lg", 36.0)] {
        assert_eq!(size(&desktop, &combobox(name)).1, h, "{name}");
    }
    assert_eq!(size(&desktop, &day("2026-10-14")), (32.0, 32.0));
    let mut touch = start::<Sizes>(false);
    touch.run_for(Duration::from_millis(600)).unwrap();
    for (name, h) in [("xs", 32.0), ("sm", 36.0), ("md", 40.0), ("lg", 48.0)] {
        assert_eq!(size(&touch, &combobox(name)).1, h, "{name}");
    }
    assert_eq!(size(&touch, &day("2026-10-14")).1, 44.0);
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

/// The popup (or sheet) of `selector`: the nearest ancestor with a fill
/// and an edge.
fn popup_of(app: &Headless, selector: &str) -> NodeId {
    let nodes = app.screen().layout_nodes();
    let mut at = app.screen().id(selector).unwrap();
    loop {
        let parent = nodes.iter().find(|n| n.id == at).unwrap().parent;
        let style = app.screen().view_style(&format!("view:{parent}")).unwrap();
        if style.is_some_and(|s| s.background_color == light(POPOVER)) {
            return parent;
        }
        at = parent;
    }
}

#[test]
fn the_popup_is_the_popover_surface_at_the_field() {
    let mut app = start::<Pickers>(true);
    let styles = watch_styles(&mut app);
    open(&mut app, "Due");
    let popup = popup_of(&app, &day("2026-10-14"));
    let style = styles.lock().unwrap().get(&popup).cloned().unwrap();
    assert_eq!(Some(style.background_color.0), light(POPOVER));
    let b = bounds(&app, &format!("view:{popup}"));
    let field = bounds(&app, &combobox("Due"));
    // Below the field, from its start, at least as wide.
    assert_eq!(b.origin.x, field.origin.x);
    assert_eq!((b.origin.y - field.origin.y - field.size.height).0, 4.0);
    assert!(b.size.width >= field.size.width);
    // The calendar inside `calendar-padding` (12) and the edge.
    let first = bounds(&app, "role:button[Go to the previous month]");
    assert_eq!((first.origin.x - b.origin.x).0, 13.0);
}

#[test]
fn the_open_field_keeps_the_focus_edge() {
    let mut app = start::<Pickers>(true);
    let styles = watch_styles(&mut app);
    open(&mut app, "Due");
    app.hover("id:away").unwrap();
    app.run_for(Duration::from_millis(300)).unwrap();
    let trigger = app.screen().id(&combobox("Due")).unwrap();
    let style = styles.lock().unwrap().get(&trigger).cloned().unwrap();
    assert_eq!(Some(style.borders.colors[0].0), light(INPUT_BORDER_FOCUS));
}

#[test]
fn touch_screens_get_a_sheet_with_one_month() {
    let mut app = start::<Sizes>(false);
    app.run_for(Duration::from_millis(600)).unwrap();
    let sheet = popup_of(&app, &day("2026-10-14"));
    assert_eq!(size(&app, &format!("view:{sheet}")).0, 800.0);
    // The days share the width.
    assert!(size(&app, &day("2026-10-14")).0 > 44.0);
}

/// A picker open in a scope whose tokens a button changes, its time row
/// editable.
#[component]
fn Live() -> impl IntoElement {
    let blue = signal(false);
    view()
        .items_start()
        .child(button().on_press(move || blue.set(true)).child("Blue"))
        .child(
            theme_scope()
                .tokens(move || {
                    if blue.get() {
                        ScopeTokens::new()
                            .set("popover", "#dbeafe")
                            .set("calendar-selected", "#2563eb")
                            .set("date-picker-segment-focus", "#93c5fd")
                    } else {
                        ScopeTokens::new()
                    }
                })
                .child(
                    view().pt(px(40.0)).child(
                        date_picker()
                            .open(Some(true))
                            .time_precision(Some(TimePrecision::Minute))
                            .hour_cycle(Some(HourCycle::H23))
                            .default_value(DateTime::from(
                                d(2026, 10, 5).and_hms_opt(9, 30, 0).unwrap(),
                            ))
                            .accessibility_label("Live"),
                    ),
                ),
        )
}

/// The view of the segment of `selector` (its text).
fn segment(app: &Headless, selector: &str) -> NodeId {
    let nodes = app.screen().layout_nodes();
    let id = app.screen().id(selector).unwrap();
    nodes.iter().find(|n| n.id == id).unwrap().parent
}

/// The selected day's square: under the item, its third drawing.
fn selected_square(app: &Headless, iso: &str) -> NodeId {
    let nodes = app.screen().layout_nodes();
    let children = |of: NodeId| -> Vec<NodeId> {
        nodes
            .iter()
            .filter(|n| n.parent == of)
            .map(|n| n.id)
            .collect()
    };
    let mut at = app.screen().id(&day(iso)).unwrap();
    let mut content = children(at);
    while content.len() == 1 {
        at = content[0];
        content = children(at);
    }
    // The square's layers: today, hover, selected.
    children(content[2])[2]
}

#[test]
fn a_token_changed_while_open_reaches_the_popup_a_day_and_a_segment() {
    let mut app = start::<Live>(true);
    app.run_for(Duration::from_millis(300)).unwrap();
    let square = selected_square(&app, "2026-10-05");
    assert_eq!(
        common::fill(&app, &format!("view:{square}")),
        light(CALENDAR_SELECTED)
    );
    let popup = popup_of(&app, &day("2026-10-14"));
    app.press("text:Blue").unwrap();
    app.run_for(Duration::from_millis(300)).unwrap();
    // Still open, the new values in.
    assert!(app.screen().matches(&day("2026-10-14")).unwrap());
    assert_eq!(common::fill(&app, &format!("view:{popup}")), css("#dbeafe"));
    assert_eq!(
        common::fill(&app, &format!("view:{square}")),
        css("#2563eb")
    );
    app.press("text:09").unwrap();
    app.run_for(Duration::from_millis(100)).unwrap();
    assert_eq!(
        common::fill(&app, &format!("view:{}", segment(&app, "text:09"))),
        css("#93c5fd")
    );
    assert_ne!(light(DATE_PICKER_SEGMENT_FOCUS), css("#93c5fd"));
}

#[test]
fn the_time_field_takes_the_focus_edge_and_ring_while_its_field_is_focused() {
    let mut app = start::<Live>(true);
    let styles = watch_styles(&mut app);
    app.run_for(Duration::from_millis(300)).unwrap();
    // The time field: the segments' parent.
    let nodes = app.screen().layout_nodes();
    let hours = segment(&app, "text:09");
    let field = nodes.iter().find(|n| n.id == hours).unwrap().parent;
    let minutes = format!("view:{field} >> text:30");
    app.press(&minutes).unwrap();
    app.run_for(Duration::from_millis(300)).unwrap();
    let focused = styles.lock().unwrap().get(&field).cloned();
    let focused = focused.expect("the time field's style changed with the focus");
    assert_eq!(Some(focused.borders.colors[0].0), light(INPUT_BORDER_FOCUS));
    let rings = focused
        .effects
        .as_ref()
        .map(|e| e.box_shadows.len())
        .unwrap_or(0);
    // A ring over the field look's shadow (tier 1).
    assert!(rings > 1, "a ring: {focused:?}");
    // The edited segment is lit, the other one not.
    assert_eq!(
        common::fill(&app, &format!("view:{}", segment(&app, &minutes))),
        light(DATE_PICKER_SEGMENT_FOCUS)
    );
    assert_eq!(common::fill(&app, &format!("view:{hours}")), None);
    // Tab from the last segment leaves it: the rest edge again.
    app.press_key("tab").unwrap();
    app.run_for(Duration::from_millis(300)).unwrap();
    let rest = styles.lock().unwrap().get(&field).cloned().unwrap();
    assert_eq!(Some(rest.borders.colors[0].0), light(INPUT_BORDER));
}

/// A label naming a picker through its ref, an invalid picker and a
/// disabled one.
#[component]
fn Labelled() -> impl IntoElement {
    let due = component_ref::<DatePicker>();
    view()
        .child(label().html_for(due).child("Due date"))
        .child(
            date_picker()
                .component_ref(&due)
                .invalid(true)
                .accessibility_label("Due"),
        )
        .child(date_picker().disabled(true).accessibility_label("Locked"))
}

#[test]
fn a_labels_press_focuses_the_field_without_opening_it() {
    let mut app = start::<Labelled>(true);
    let screen = app.screen();
    assert!(
        screen
            .accessibility_state(&combobox("Due"))
            .unwrap()
            .invalid
    );
    assert!(!screen.enabled(&combobox("Locked")).unwrap());
    let trigger = screen.id(&combobox("Due")).unwrap();
    app.press("text:Due date").unwrap();
    app.run_for(Duration::from_millis(50)).unwrap();
    assert!(
        app.screen()
            .commands()
            .iter()
            .any(|(id, c)| *id == trigger && matches!(c, Command::Focus))
    );
    assert!(!app.screen().matches(&day("2026-10-14")).unwrap());
}

/// TSX apps' declarations: the value type, the props and the ref.
#[test]
fn tsx_gets_the_value_type_and_the_ref() {
    let registry = nsh_longbridge::registry();
    assert_eq!(registry.undocumented(), Vec::<String>::new());
    let ts = nsh_core::element::declarations(&registry, &[]);
    for wanted in [
        "export type DatePickerValue =",
        "export interface DatePickerRef {",
        "timePrecision?:",
        "presets?: (DateRangePreset)[];",
    ] {
        assert!(ts.contains(wanted), "{wanted} in:\n{ts}");
    }
}

/// A date picker beside a long label in a narrow horizontal field.
#[component]
fn Beside() -> impl IntoElement {
    view().p(px(20.0)).w(px(220.0)).child(
        field()
            .orientation(nsh_longbridge::FieldOrientation::Horizontal)
            .child(field_label().child(text("Due date of the quarterly report")))
            .child(date_picker().accessibility_label("Due")),
    )
}

#[test]
fn in_a_horizontal_field_it_shrinks_to_share_the_row() {
    let app = start::<Beside>(true);
    let picker = bounds(&app, &combobox("Due"));
    let label = bounds(&app, "text:Due date of the quarterly report");
    // Inside the 180-wide field, after the label and the gap.
    assert!(
        picker.origin.x + picker.size.width <= px(200.5),
        "{picker:?}"
    );
    assert!(
        label.origin.x + label.size.width + px(8.0) <= picker.origin.x + px(0.5),
        "{label:?} {picker:?}"
    );
}

/// Confirmed pickers (`confirm`): a controlled range with a preset, an
/// empty single date and a date and time, writing down what they report.
#[component]
fn Confirmed() -> impl IntoElement {
    let period = signal(DateTime::from((d(2026, 10, 5), d(2026, 10, 9))));
    let log = signal(Vec::<String>::new());
    let note = move |s: String| log.update(|l| l.push(s));
    view()
        .pt(px(40.0))
        .pl(px(40.0))
        .items_start()
        .gap(px(8.0))
        .child(
            date_picker()
                .range(true)
                .confirm(true)
                .first_day_of_week(Some(0))
                .value(move || period.get())
                .presets(vec![DateRangePreset::range(
                    "Holidays",
                    d(2026, 12, 24),
                    d(2026, 12, 26),
                )])
                .accessibility_label("Period")
                .on_change(move |picked: &PickedDate| {
                    let v = picked.value;
                    note(format!(
                        "change:{:?}..{:?}",
                        v.start().map(|v| v.date()),
                        v.end().map(|v| v.date())
                    ));
                    period.set(v);
                })
                .on_open_change(move |e: &DatePickerEvent| {
                    if let DatePickerEvent::OpenChange(open, reason) = e {
                        note(format!("open:{open}:{reason:?}"));
                    }
                })
                .on_cancel(move || note("cancel".into())),
        )
        .child(
            date_picker()
                .confirm(true)
                .first_day_of_week(Some(0))
                .accessibility_label("Due"),
        )
        .child(
            date_picker()
                .confirm(true)
                .time_precision(Some(TimePrecision::Minute))
                .hour_cycle(Some(HourCycle::H23))
                .date_format(Some("%Y-%m-%d".into()))
                .default_value(DateTime::from(
                    d(2026, 11, 26).and_hms_opt(9, 30, 0).unwrap(),
                ))
                .accessibility_label("Meeting"),
        )
        .child(view().test_id("away").w(px(200.0)).h(px(40.0)))
        .child(text(move || format!("log:{}", log.get().join(","))))
}

fn field_text(app: &Headless, name: &str) -> String {
    app.screen()
        .input_text(&format!("label:{name}"))
        .unwrap()
        .unwrap_or_default()
        .to_owned()
}

fn apply_enabled(app: &Headless) -> bool {
    app.screen().enabled("role:button[Apply]").unwrap()
}

#[test]
fn confirm_keeps_a_complete_range_pending_until_apply() {
    let mut app = start::<Confirmed>(true);
    open(&mut app, "Period");
    assert_eq!(field_text(&app, "Start date"), "Oct 5, 2026");
    assert_eq!(field_text(&app, "End date"), "Oct 9, 2026");
    app.press(&day("2026-10-12")).unwrap();
    app.press(&day("2026-10-14")).unwrap();
    app.run_for(Duration::from_millis(400)).unwrap();
    // Still open, nothing reported, the field unchanged.
    assert!(app.screen().matches(&day("2026-10-14")).unwrap());
    assert!(
        app.shows("log:open:true:TriggerPress"),
        "{:?}",
        app.screen().texts()
    );
    assert!(app.shows("Oct 5, 2026 – Oct 9, 2026"));
    assert_eq!(field_text(&app, "Start date"), "Oct 12, 2026");
    assert_eq!(field_text(&app, "End date"), "Oct 14, 2026");
    app.press("role:button[Apply]").unwrap();
    app.wait_for(
        "text:log:open:true:TriggerPress,change:Some(2026-10-12)..Some(2026-10-14),open:false:Select",
    )
    .unwrap();
    app.run_for(Duration::from_millis(400)).unwrap();
    assert!(!app.screen().matches(&day("2026-10-14")).unwrap());
    assert!(app.shows("Oct 12, 2026 – Oct 14, 2026"));
}

#[test]
fn confirm_presets_show_their_dates_fill_the_calendar_and_cancel_drops_them() {
    let mut app = start::<Confirmed>(true);
    open(&mut app, "Period");
    // The dates under the label, in the field's format.
    assert!(app.shows("Dec 24, 2026 – Dec 26, 2026"));
    let preset = app
        .screen()
        .accessibility(Some("role:button[Holidays]"))
        .unwrap()
        .unwrap();
    assert!(!preset.selected);
    app.press("role:button[Holidays]").unwrap();
    app.run_for(Duration::from_millis(300)).unwrap();
    // Open, the fields and the calendar on it, the preset lit.
    assert_eq!(field_text(&app, "Start date"), "Dec 24, 2026");
    assert_eq!(field_text(&app, "End date"), "Dec 26, 2026");
    assert!(app.screen().matches(&day("2026-12-24")).unwrap());
    let preset = app
        .screen()
        .accessibility(Some("role:button[Holidays]"))
        .unwrap()
        .unwrap();
    assert!(preset.selected);
    app.press("role:button[Cancel]").unwrap();
    app.wait_for("text:log:open:true:TriggerPress,cancel,open:false:Cancel")
        .unwrap();
    app.run_for(Duration::from_millis(400)).unwrap();
    assert!(app.shows("Oct 5, 2026 – Oct 9, 2026"));
    // Opening again starts from the committed value.
    open(&mut app, "Period");
    assert_eq!(field_text(&app, "Start date"), "Oct 5, 2026");
}

#[test]
fn confirm_escape_drops_the_pending_value() {
    let mut app = start::<Confirmed>(true);
    open(&mut app, "Period");
    app.press("role:button[Holidays]").unwrap();
    app.press_key("escape").unwrap();
    app.run_for(Duration::from_millis(400)).unwrap();
    assert!(!app.screen().matches("role:button[Apply]").unwrap());
    assert!(app.shows("Oct 5, 2026 – Oct 9, 2026"));
    assert!(!app.screen().matches("text:/change:/").unwrap());
}

#[test]
fn confirm_typed_fields_set_the_ends_and_return_applies() {
    let mut app = start::<Confirmed>(true);
    open(&mut app, "Period");
    // An ISO date reads; a start after the end drops the end.
    app.type_text("label:Start date", "2026-10-20").unwrap();
    app.run_for(Duration::from_millis(100)).unwrap();
    assert_eq!(field_text(&app, "Start date"), "2026-10-20");
    assert_eq!(field_text(&app, "End date"), "");
    assert!(!apply_enabled(&app));
    // The field's format reads, in any case.
    app.type_text("label:End date", "oct 25, 2026").unwrap();
    app.run_for(Duration::from_millis(100)).unwrap();
    assert!(apply_enabled(&app));
    app.submit("label:End date").unwrap();
    app.wait_for("text:/change:Some\\(2026-10-20\\)..Some\\(2026-10-25\\)/")
        .unwrap();
    app.run_for(Duration::from_millis(400)).unwrap();
    assert!(app.shows("Oct 20, 2026 – Oct 25, 2026"));
}

#[test]
fn confirm_refused_text_is_kept_and_shown_invalid_on_leaving() {
    let mut app = start::<Confirmed>(true);
    open(&mut app, "Period");
    app.type_text("label:End date", "someday").unwrap();
    app.run_for(Duration::from_millis(100)).unwrap();
    // Not while typing.
    let state = app.screen().accessibility_state("label:End date").unwrap();
    assert!(!state.invalid);
    app.press_key("tab").unwrap();
    app.run_for(Duration::from_millis(100)).unwrap();
    let state = app.screen().accessibility_state("label:End date").unwrap();
    assert!(state.invalid);
    assert_eq!(field_text(&app, "End date"), "someday");
    assert!(!apply_enabled(&app));
    // An end before the start is refused too.
    app.type_text("label:End date", "Oct 1, 2026").unwrap();
    app.submit("label:End date").unwrap();
    app.run_for(Duration::from_millis(100)).unwrap();
    assert!(
        app.screen()
            .accessibility_state("label:End date")
            .unwrap()
            .invalid
    );
    // A date that reads clears it, and leaving writes it in the format.
    app.type_text("label:End date", "2026-10-07").unwrap();
    app.run_for(Duration::from_millis(100)).unwrap();
    assert!(
        !app.screen()
            .accessibility_state("label:End date")
            .unwrap()
            .invalid
    );
    app.press_key("tab").unwrap();
    app.run_for(Duration::from_millis(100)).unwrap();
    assert_eq!(field_text(&app, "End date"), "Oct 7, 2026");
    assert!(apply_enabled(&app));
}

#[test]
fn confirm_a_single_date_and_time_waits_for_apply() {
    let mut app = start::<Confirmed>(true);
    open(&mut app, "Due");
    assert_eq!(field_text(&app, "Date"), "");
    assert!(!apply_enabled(&app));
    app.press(&day("2026-11-27")).unwrap();
    app.run_for(Duration::from_millis(100)).unwrap();
    assert_eq!(field_text(&app, "Date"), "Nov 27, 2026");
    assert!(apply_enabled(&app));
    app.press("role:button[Apply]").unwrap();
    app.run_for(Duration::from_millis(400)).unwrap();
    assert!(app.shows("Nov 27, 2026"));

    open(&mut app, "Meeting");
    assert!(app.screen().matches("label:Time").unwrap());
    app.press(&day("2026-11-27")).unwrap();
    app.type_text("label:Time", "1").unwrap();
    app.type_text("label:Time", "4").unwrap();
    app.run_for(Duration::from_millis(100)).unwrap();
    // Pending: the field still shows the committed value.
    assert!(app.shows("2026-11-26 09:30"));
    app.press("role:button[Apply]").unwrap();
    app.run_for(Duration::from_millis(400)).unwrap();
    assert!(app.shows("2026-11-27 14:30"), "{:?}", app.screen().texts());
}

#[test]
fn confirm_keeps_the_fields_combobox_and_its_keys() {
    let mut app = start::<Confirmed>(true);
    let node = app
        .screen()
        .accessibility(Some(&combobox("Period")))
        .unwrap()
        .unwrap();
    assert_eq!(node.value.as_deref(), Some("Oct 5, 2026 – Oct 9, 2026"));
    assert_eq!(node.expanded, Some(false));
    // Return on the focused field opens it, the focus going to the first
    // typed field.
    app.press_key("tab").unwrap();
    app.press_key("enter").unwrap();
    app.run_for(Duration::from_millis(300)).unwrap();
    assert_eq!(
        app.screen()
            .accessibility_state(&combobox("Period"))
            .unwrap()
            .expanded,
        Some(true)
    );
    assert!(app.screen().matches("role:button[Apply]").unwrap());
    // Below the field, from its start, at least as wide.
    let popup = popup_of(&app, "role:button[Apply]");
    let b = bounds(&app, &format!("view:{popup}"));
    let field = bounds(&app, &combobox("Period"));
    assert_eq!(b.origin.x, field.origin.x);
    assert_eq!((b.origin.y - field.origin.y - field.size.height).0, 4.0);
    assert!(b.size.width >= field.size.width);
    // A press outside (on another picker) drops the pending value and
    // closes it.
    app.press("role:button[Holidays]").unwrap();
    app.press(&combobox("Due")).unwrap();
    app.run_for(Duration::from_millis(400)).unwrap();
    assert_eq!(
        app.screen()
            .accessibility_state(&combobox("Period"))
            .unwrap()
            .expanded,
        Some(false)
    );
    assert!(
        app.screen()
            .matches("text:/open:false:PressOutside/")
            .unwrap()
    );
    assert!(app.shows("Oct 5, 2026 – Oct 9, 2026"));
}

#[test]
fn confirm_escape_gives_the_focus_back_and_delete_clears() {
    let mut app = start::<Confirmed>(true);
    app.press_key("tab").unwrap();
    app.press_key("enter").unwrap();
    app.run_for(Duration::from_millis(300)).unwrap();
    app.press_key("escape").unwrap();
    app.run_for(Duration::from_millis(400)).unwrap();
    assert!(!app.screen().matches("role:button[Apply]").unwrap());
    // The focus is back on the field: Delete on the closed field clears it.
    app.press_key("delete").unwrap();
    app.wait_for("text:/change:None..None/").unwrap();
}

#[test]
fn confirm_on_touch_screens_is_a_sheet_with_the_footer() {
    let mut app = start::<Confirmed>(false);
    open(&mut app, "Period");
    app.run_for(Duration::from_millis(400)).unwrap();
    // No typed fields; the pending value above Cancel and Apply.
    assert!(!app.screen().matches("label:Start date").unwrap());
    assert!(app.screen().matches("role:button[Apply]").unwrap());
    assert!(app.screen().matches("role:button[Holidays]").unwrap());
    // The presets show their labels only; the footer what is pending.
    assert!(!app.shows("Dec 24, 2026 – Dec 26, 2026"));
    app.press("role:button[Holidays]").unwrap();
    app.run_for(Duration::from_millis(100)).unwrap();
    assert!(app.shows("Dec 24, 2026 – Dec 26, 2026"));
    let sheet = popup_of(&app, &day("2026-12-24"));
    assert_eq!(size(&app, &format!("view:{sheet}")).0, 800.0);
    app.press("role:button[Apply]").unwrap();
    app.wait_for("text:/change:Some\\(2026-12-24\\)..Some\\(2026-12-26\\)/")
        .unwrap();
}

#[test]
fn tsx_gets_confirm_and_its_parts() {
    let registry = nsh_longbridge::registry();
    let ts = nsh_core::element::declarations(&registry, &[]);
    for wanted in [
        "confirm?:",
        "presetDescriptionClass?:",
        "fieldsClass?:",
        "footerStyle?:",
    ] {
        assert!(ts.contains(wanted), "{wanted}");
    }
}
