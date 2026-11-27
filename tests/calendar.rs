//! `Calendar` in a headless engine: a press picks a day or a range,
//! controlled and uncontrolled; what screen readers get; pointer and touch
//! cells; each state of a day reaching the library's drawing inside it
//! (through the item's named group and its `data-*` attributes); the
//! names from the library's words; the user's first day of the week; one
//! month in a narrow window; no fill inside a `Card`; a scope's and the
//! dark appearance's tokens reaching the days.

mod common;

use common::{css, dark, light, start};
use nsh_core::{
    Color, NodeId, Size as Viewport,
    platform::{ColorScheme, Os, Weekday},
    prelude::*,
};
use nsh_engine::EngineConfig;
use nsh_longbridge::{
    CalendarValue, Date, Matcher, ScopeTokens, Size, calendar, card, card_content,
    chrono::NaiveDate,
    components::{
        button::BUTTON_GHOST_HOVER,
        calendar::{
            CALENDAR_BACKGROUND, CALENDAR_OUTSIDE_FOREGROUND, CALENDAR_RANGE,
            CALENDAR_RANGE_FOREGROUND, CALENDAR_SELECTED, CALENDAR_SELECTED_FOREGROUND,
            CALENDAR_TODAY, CALENDAR_TODAY_FOREGROUND,
        },
    },
    labels::{self, Label},
    theme_scope,
};
use nsh_testing::Headless;

fn d(y: i32, m: u32, day: u32) -> NaiveDate {
    NaiveDate::from_ymd_opt(y, m, day).unwrap()
}

fn day(iso: &str) -> String {
    format!("role:button[{iso}]")
}

/// A controlled single calendar writing down what it reports, an
/// uncontrolled range one, and one with weekends disabled.
#[component]
fn Picks() -> impl IntoElement {
    let value = signal(Date::Single(Some(d(2026, 10, 5))));
    let log = signal(Vec::<String>::new());
    view()
        .child(
            calendar()
                .first_day_of_week(Some(0))
                .value(move || CalendarValue::Controlled(value.get()))
                .on_change(move |v: &Date| {
                    log.update(|l| l.push(format!("{:?}", v.start())));
                    value.set(*v);
                }),
        )
        .child(
            calendar()
                .range(true)
                .first_day_of_week(Some(0))
                .default_value(Date::Range(None, None))
                .value(CalendarValue::Uncontrolled)
                .on_change(move |v: &Date| {
                    log.update(|l| l.push(format!("range:{:?}-{:?}", v.start(), v.end())))
                }),
        )
        .child(text(move || format!("log:{}", log.get().join(","))))
}

#[test]
fn a_press_picks_a_day() {
    let mut app = start::<Picks>(true);
    app.press(&day("2026-10-14")).unwrap();
    app.wait_for("text:log:Some(2026-10-14)").unwrap();
    assert!(
        app.screen()
            .accessibility_state(&day("2026-10-14"))
            .unwrap()
            .selected
    );
    assert!(
        !app.screen()
            .accessibility_state(&day("2026-10-05"))
            .unwrap()
            .selected
    );
}

#[test]
fn a_range_takes_two_presses_and_reports_once() {
    let mut app = start::<Picks>(true);
    app.press(&day("2026-11-10")).unwrap();
    app.press(&day("2026-11-12")).unwrap();
    app.wait_for("text:log:range:Some(2026-11-10)-Some(2026-11-12)")
        .unwrap();
}

#[test]
fn screen_readers_get_buttons_headers_and_the_arrows_names() {
    let app = start::<Picks>(true);
    let screen = app.screen();
    assert!(screen.matches(&day("2026-10-28")).unwrap());
    assert!(
        screen
            .matches("role:button[Go to the previous month]")
            .unwrap()
    );
    assert!(screen.matches("role:button[Go to the next month]").unwrap());
    assert!(screen.matches("role:columnheader").unwrap());
}

/// Calendars of each size on the desktop, and a medium one.
#[component]
fn Sizes() -> impl IntoElement {
    view()
        .flex_row()
        .items_start()
        .child(
            calendar()
                .size(Size::Xs)
                .default_value(Date::Single(Some(d(2026, 1, 15)))),
        )
        .child(calendar().default_value(Date::Single(Some(d(2026, 2, 15)))))
        .child(
            calendar()
                .size(Size::Lg)
                .default_value(Date::Single(Some(d(2026, 3, 15)))),
        )
}

fn height(app: &Headless, selector: &str) -> f32 {
    common::size(app, selector).1
}

#[test]
fn cells_take_the_control_heights_and_44_on_touch_screens() {
    let desktop = start::<Sizes>(true);
    assert_eq!(height(&desktop, &day("2026-01-15")), 24.0);
    assert_eq!(height(&desktop, &day("2026-02-15")), 32.0);
    assert_eq!(height(&desktop, &day("2026-03-15")), 36.0);
    // As wide as tall: seven cells.
    assert_eq!(common::size(&desktop, &day("2026-02-15")).0, 32.0);
    let touch = start::<Sizes>(false);
    assert_eq!(height(&touch, &day("2026-01-15")), 44.0);
    assert_eq!(height(&touch, &day("2026-02-15")), 44.0);
    assert_eq!(height(&touch, &day("2026-03-15")), 48.0);
}

/// The library's drawing in a day: its band's two halves (toward the
/// week's start, toward its end), then its square's layers (today, under
/// the pointer, selected).
struct Drawing {
    band_start: NodeId,
    band_end: NodeId,
    today: NodeId,
    hover: NodeId,
    selected: NodeId,
}

fn drawing(app: &Headless, iso: &str) -> Drawing {
    let screen = app.screen();
    let nodes = screen.layout_nodes();
    let children = |of: NodeId| -> Vec<NodeId> {
        nodes
            .iter()
            .filter(|n| n.parent == of)
            .map(|n| n.id)
            .collect()
    };
    // Under the item (and the view the renderer places), the drawing: the
    // first view with more than one child.
    let mut at = screen.id(&day(iso)).unwrap();
    let mut content = children(at);
    while content.len() == 1 {
        at = content[0];
        content = children(at);
    }
    let [band_start, band_end, square] = content[..] else {
        panic!("{iso}: {content:?}")
    };
    let layers = children(square);
    Drawing {
        band_start,
        band_end,
        today: layers[0],
        hover: layers[1],
        selected: layers[2],
    }
}

fn opacity(app: &Headless, node: NodeId) -> f32 {
    app.screen()
        .view_style(&format!("view:{node}"))
        .unwrap()
        .unwrap()
        .opacity
}

fn fill_of(app: &Headless, node: NodeId) -> Option<u32> {
    app.screen()
        .view_style(&format!("view:{node}"))
        .unwrap()
        .unwrap()
        .background_color
}

fn shown(app: &Headless, node: NodeId) -> bool {
    opacity(app, node) == 1.0
}

/// The color of a day's number.
fn number(app: &Headless, iso: &str) -> Option<u32> {
    let n: u32 = iso[8..].parse().unwrap();
    let color = app
        .screen()
        .span_style(&format!("{} >> text:{n}", day(iso)))
        .unwrap()
        .color;
    Some(color.0)
}

/// A complete range in October 2026 (weeks from Sunday), weekends
/// disabled in November; a one-day range; a half range to preview.
#[component]
fn States() -> impl IntoElement {
    view()
        .flex_row()
        .flex_wrap()
        .child(
            calendar()
                .range(true)
                .first_day_of_week(Some(0))
                .default_value(Date::Range(Some(d(2026, 10, 20)), Some(d(2026, 10, 23)))),
        )
        .child(
            calendar()
                .first_day_of_week(Some(0))
                .default_value(Date::Single(Some(d(2026, 12, 9))))
                .disabled_matcher(Some(Matcher::from(vec![0, 6]))),
        )
        .child(
            calendar()
                .range(true)
                .default_value(Date::Range(Some(d(2027, 1, 7)), Some(d(2027, 1, 7)))),
        )
        .child(
            calendar()
                .range(true)
                .default_value(Date::Range(Some(d(2027, 4, 1)), None)),
        )
}

#[test]
fn a_ranges_states_reach_the_band_and_the_square() {
    let app = start::<States>(true);
    let selected = light(CALENDAR_SELECTED_FOREGROUND);
    // The start: the band toward the range, the selected square.
    let start = drawing(&app, "2026-10-20");
    assert!(!shown(&app, start.band_start) && shown(&app, start.band_end));
    assert!(shown(&app, start.selected));
    assert_eq!(number(&app, "2026-10-20"), selected);
    // The middle: the band across, no square, the range's text.
    let middle = drawing(&app, "2026-10-21");
    assert!(shown(&app, middle.band_start) && shown(&app, middle.band_end));
    assert!(!shown(&app, middle.selected));
    assert_eq!(fill_of(&app, middle.band_start), light(CALENDAR_RANGE));
    assert_eq!(number(&app, "2026-10-21"), light(CALENDAR_RANGE_FOREGROUND));
    // The end: the band toward the start.
    let end = drawing(&app, "2026-10-23");
    assert!(shown(&app, end.band_start) && !shown(&app, end.band_end));
    assert!(shown(&app, end.selected));
    assert_eq!(number(&app, "2026-10-23"), selected);
    // Outside the range: nothing.
    let out = drawing(&app, "2026-10-26");
    assert!(!shown(&app, out.band_start) && !shown(&app, out.band_end));
    assert!(!shown(&app, out.selected) && !shown(&app, out.hover));
    // A range of one day: the square, no band.
    let one = drawing(&app, "2027-01-07");
    assert!(!shown(&app, one.band_start) && !shown(&app, one.band_end));
    assert!(shown(&app, one.selected));
}

#[test]
fn a_selected_day_a_day_of_another_month_and_a_disabled_day() {
    let app = start::<States>(true);
    let single = drawing(&app, "2026-12-09");
    assert!(shown(&app, single.selected));
    assert_eq!(fill_of(&app, single.selected), light(CALENDAR_SELECTED));
    assert_eq!(
        number(&app, "2026-12-09"),
        light(CALENDAR_SELECTED_FOREGROUND)
    );
    // September 27 shows in October's first week, muted.
    assert_eq!(
        number(&app, "2026-09-27"),
        light(CALENDAR_OUTSIDE_FOREGROUND)
    );
    // A Saturday of December: dimmed, muted, not pressable.
    let screen = app.screen();
    assert!(
        screen
            .accessibility_state(&day("2026-12-12"))
            .unwrap()
            .disabled
    );
    assert_eq!(
        screen
            .view_style(&day("2026-12-12"))
            .unwrap()
            .unwrap()
            .opacity,
        0.5
    );
    assert_eq!(
        number(&app, "2026-12-12"),
        light(CALENDAR_OUTSIDE_FOREGROUND)
    );
}

#[test]
fn the_pointer_lights_a_day_and_previews_a_half_range() {
    let mut app = start::<States>(true);
    app.hover(&day("2026-10-27")).unwrap();
    let hovered = drawing(&app, "2026-10-27");
    assert!(shown(&app, hovered.hover));
    assert_eq!(fill_of(&app, hovered.hover), light(BUTTON_GHOST_HOVER));
    // No hover on a disabled day.
    app.hover(&day("2026-12-13")).unwrap();
    assert!(!shown(&app, drawing(&app, "2026-12-13").hover));
    // From April 1 to the day under the pointer.
    app.hover(&day("2027-04-03")).unwrap();
    app.run_for(std::time::Duration::from_millis(50)).unwrap();
    let first = drawing(&app, "2027-04-01");
    assert!(shown(&app, first.selected) && shown(&app, first.band_end));
    let between = drawing(&app, "2027-04-02");
    assert!(shown(&app, between.band_start) && shown(&app, between.band_end));
    assert_eq!(number(&app, "2027-04-02"), light(CALENDAR_RANGE_FOREGROUND));
    let reach = drawing(&app, "2027-04-03");
    assert!(shown(&app, reach.band_start) && !shown(&app, reach.band_end));
    assert!(shown(&app, reach.hover));
}

/// A calendar on today's month: today's mark.
#[component]
fn Today() -> impl IntoElement {
    view().child(calendar())
}

#[test]
fn today_has_its_square() {
    let app = start::<Today>(true);
    let today = nsh_longbridge::chrono::Local::now()
        .date_naive()
        .format("%Y-%m-%d")
        .to_string();
    let mark = drawing(&app, &today);
    assert!(shown(&app, mark.today));
    assert_eq!(fill_of(&app, mark.today), light(CALENDAR_TODAY));
    assert_eq!(number(&app, &today), light(CALENDAR_TODAY_FOREGROUND));
}

/// One selected day outside and inside a scope that changes its fill.
#[component]
fn Scoped() -> impl IntoElement {
    view()
        .child(calendar().default_value(Date::Single(Some(d(2026, 6, 10)))))
        .child(
            theme_scope()
                .tokens(ScopeTokens::new().set("calendar-selected", "#2563eb"))
                .child(calendar().default_value(Date::Single(Some(d(2026, 8, 10))))),
        )
}

#[test]
fn a_scope_reaches_the_selected_square() {
    let app = start::<Scoped>(true);
    let inside = drawing(&app, "2026-08-10");
    assert_eq!(fill_of(&app, inside.selected), css("#2563eb"));
    let outside = drawing(&app, "2026-06-10");
    assert_eq!(fill_of(&app, outside.selected), light(CALENDAR_SELECTED));
}

#[test]
fn in_the_dark_the_days_take_the_dark_values() {
    let mut config = EngineConfig {
        viewport: Viewport::new(px(800.0), px(600.0)),
        ..EngineConfig::new(nsh_testing::windowed_root::<States>())
    };
    config.environment.os = Os::MacOs;
    config.environment.color_scheme = ColorScheme::Dark;
    let mut app = Headless::start(config).unwrap();
    app.first_render().unwrap();
    let as_fill = |c: Color| Some(c.0);
    assert_eq!(
        number(&app, "2026-10-20"),
        as_fill(dark(CALENDAR_SELECTED_FOREGROUND))
    );
    assert_eq!(
        number(&app, "2026-10-21"),
        as_fill(dark(CALENDAR_RANGE_FOREGROUND))
    );
    let start = drawing(&app, "2026-10-20");
    assert_eq!(
        fill_of(&app, start.selected),
        as_fill(dark(CALENDAR_SELECTED))
    );
}

/// A calendar alone and one in a card.
#[component]
fn InCard() -> impl IntoElement {
    view()
        .items_start()
        .child(
            calendar()
                .test_id("alone")
                .default_value(Date::Single(Some(d(2026, 5, 1)))),
        )
        .child(
            card().child(
                card_content().child(
                    calendar()
                        .test_id("carded")
                        .default_value(Date::Single(Some(d(2026, 7, 1)))),
                ),
            ),
        )
}

#[test]
fn it_has_no_fill_inside_a_card() {
    let app = start::<InCard>(true);
    assert_eq!(common::fill(&app, "id:alone"), light(CALENDAR_BACKGROUND));
    assert_eq!(common::fill(&app, "id:carded"), None);
}

/// Two months, Monday first, and a button that translates the names.
#[component]
fn Named() -> impl IntoElement {
    view()
        .child(
            calendar()
                .number_of_months(2)
                .default_value(Date::Single(Some(d(2026, 10, 14)))),
        )
        .child(
            nsh_longbridge::button()
                .on_press(|| {
                    labels::set_translator(|label| match label {
                        Label::Month10 => Some("Oktober".into()),
                        Label::Weekday1 => Some("Mo.".into()),
                        _ => None,
                    })
                })
                .child("German"),
        )
}

fn start_named(width: f32, first: Weekday) -> Headless {
    let mut config = EngineConfig {
        viewport: Viewport::new(px(width), px(700.0)),
        ..EngineConfig::new(nsh_testing::windowed_root::<Named>())
    };
    config.environment.os = Os::MacOs;
    config.environment.locale_settings.first_day_of_week = first;
    let mut app = Headless::start(config).unwrap();
    app.first_render().unwrap();
    app
}

#[test]
fn the_names_are_the_librarys_words() {
    let mut app = start_named(900.0, Weekday::Monday);
    assert!(app.shows("October 2026"));
    assert!(app.shows("November 2026"));
    assert!(app.shows("Mo"));
    app.press("text:German").unwrap();
    app.wait_for("text:Oktober 2026").unwrap();
    assert!(app.shows("Mo."));
}

#[test]
fn the_week_starts_on_the_users_day() {
    let monday = start_named(900.0, Weekday::Monday);
    // October 2026 starts on a Thursday: from Monday, September 28 first.
    assert!(monday.screen().matches(&day("2026-09-28")).unwrap());
    assert!(!monday.screen().matches(&day("2026-09-27")).unwrap());
    let sunday = start_named(900.0, Weekday::Sunday);
    assert!(sunday.screen().matches(&day("2026-09-27")).unwrap());
}

#[test]
fn a_narrow_window_shows_one_month() {
    let wide = start_named(900.0, Weekday::Monday);
    assert!(wide.screen().matches(&day("2026-11-18")).unwrap());
    let narrow = start_named(600.0, Weekday::Monday);
    assert!(!narrow.screen().matches(&day("2026-11-18")).unwrap());
    // Previous and Next with the month and year toggles.
    assert!(narrow.shows("Oct"));
    assert!(narrow.shows("2026"));
}
