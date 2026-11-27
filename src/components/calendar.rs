//! `Calendar`: the library's calendar on `nsh/base`'s `Calendar` (the value,
//! the range, the month and year views, disabled days, the keys, the
//! states of each item, the roles for screen readers). The library draws
//! every item through its own `renderItem` (a day's band and square and
//! number, the names, the chevrons) and writes the parts' look into
//! `nsh/base`'s part props. `DatePicker` draws its calendar with the same
//! look ([`look`]). API: `docs/components/calendar.md`.

use std::rc::Rc;

use chrono::{Datelike, NaiveDate};
use nsh_base::calendar as base_calendar;
use nsh_core::{
    element::{Renderer, Style, StyleRefinement, TextPartStyle, locale_settings, window},
    prelude::*,
};

pub use nsh_base::{CalendarEntry, CalendarItemKind, CalendarValue, Date, Matcher, YearRange};

use crate::{
    components::{
        button::{
            BUTTON_GHOST_FOREGROUND, BUTTON_GHOST_HOVER, BUTTON_GHOST_HOVER_FOREGROUND,
            BUTTON_RADIUS, BUTTON_RING,
        },
        card::in_card,
    },
    foundation::{
        class::Classes,
        focus::focus_ring,
        icon::{IconName, icon_size_token, icon_view, provide_icon_size, provide_text_color},
        labels::{self, Label},
        popup,
        size::Size,
        typography::{font_size_token, line_height, medium},
    },
    theme::{
        Theme, Token, kind,
        seeds::SPACING,
        semantic::{
            ACCENT, ACCENT_FOREGROUND, BACKGROUND, CONTROL_HEIGHT_LG, CONTROL_HEIGHT_MD,
            CONTROL_HEIGHT_SM, CONTROL_HEIGHT_TOUCH_LG, CONTROL_HEIGHT_TOUCH_MD,
            CONTROL_HEIGHT_TOUCH_SM, CONTROL_HEIGHT_TOUCH_XS, CONTROL_HEIGHT_XS, FONT_SIZE_XS,
            MUTED_FOREGROUND, OPACITY_DISABLED, PRIMARY, PRIMARY_FOREGROUND, TEXT_SM, TOUCH_TARGET,
        },
        theme,
    },
    tokens,
};

tokens! {
    /// The calendar's fill: transparent inside a library `Popover` or
    /// `Card`.
    CALENDAR_BACKGROUND: Color = "calendar-background" => |t| t.color(BACKGROUND),
    /// The space around the header and the months (`p-3`).
    CALENDAR_PADDING: Length = "calendar-padding" => |t| t.length(SPACING) * 3.0,
    /// The space between the header and the months.
    CALENDAR_GAP: Length = "calendar-gap" => |t| t.length(SPACING) * 4.0,
    /// The space between months side by side.
    CALENDAR_MONTH_GAP: Length = "calendar-month-gap" => |t| t.length(SPACING) * 4.0,
    /// The space above each week (under the weekdays too), and between the
    /// rows of the month and year views.
    CALENDAR_ROW_GAP: Length = "calendar-row-gap" => |t| t.length(SPACING) * 2.0,
    /// An extra small calendar's cell: at least `touch-target` on touch
    /// screens.
    CALENDAR_CELL_SIZE_XS: Length = "calendar-cell-size-xs" => |t| t.pointer_touch(t.length(CONTROL_HEIGHT_XS), t.max(t.length(CONTROL_HEIGHT_TOUCH_XS), t.length(TOUCH_TARGET))),
    /// A small calendar's cell.
    CALENDAR_CELL_SIZE_SM: Length = "calendar-cell-size-sm" => |t| t.pointer_touch(t.length(CONTROL_HEIGHT_SM), t.max(t.length(CONTROL_HEIGHT_TOUCH_SM), t.length(TOUCH_TARGET))),
    /// A medium calendar's cell: a medium control's height.
    CALENDAR_CELL_SIZE_MD: Length = "calendar-cell-size-md" => |t| t.pointer_touch(t.length(CONTROL_HEIGHT_MD), t.max(t.length(CONTROL_HEIGHT_TOUCH_MD), t.length(TOUCH_TARGET))),
    /// A large calendar's cell.
    CALENDAR_CELL_SIZE_LG: Length = "calendar-cell-size-lg" => |t| t.pointer_touch(t.length(CONTROL_HEIGHT_LG), t.max(t.length(CONTROL_HEIGHT_TOUCH_LG), t.length(TOUCH_TARGET))),
    /// The month and year toggles' and the months' titles' text size:
    /// Tailwind's `text-sm`.
    CALENDAR_CAPTION_FONT_SIZE: Length = "calendar-caption-font-size" => |t| t.length(TEXT_SM),
    /// The weekdays' text size.
    CALENDAR_WEEKDAY_FONT_SIZE: Length = "calendar-weekday-font-size" => |t| t.length(FONT_SIZE_XS),
    /// The weekdays' text, and the toggles' chevrons.
    CALENDAR_WEEKDAY_FOREGROUND: Color = "calendar-weekday-foreground" => |t| t.color(MUTED_FOREGROUND),
    /// A day's number.
    CALENDAR_DAY_FOREGROUND: Color = "calendar-day-foreground" => |t| t.color(BUTTON_GHOST_FOREGROUND),
    /// The number of a day of the months around, and of a disabled day.
    CALENDAR_OUTSIDE_FOREGROUND: Color = "calendar-outside-foreground" => |t| t.color(MUTED_FOREGROUND),
    /// Today's square.
    CALENDAR_TODAY: Color = "calendar-today" => |t| t.color(ACCENT),
    /// Today's number.
    CALENDAR_TODAY_FOREGROUND: Color = "calendar-today-foreground" => |t| t.color(ACCENT_FOREGROUND),
    /// The selected day's square (a range's ends, a month or a year of
    /// those views).
    CALENDAR_SELECTED: Color = "calendar-selected" => |t| t.color(PRIMARY),
    /// The selected day's number.
    CALENDAR_SELECTED_FOREGROUND: Color = "calendar-selected-foreground" => |t| t.color(PRIMARY_FOREGROUND),
    /// The band behind a range's days.
    CALENDAR_RANGE: Color = "calendar-range" => |t| t.color(ACCENT),
    /// The number of a day inside a range.
    CALENDAR_RANGE_FOREGROUND: Color = "calendar-range-foreground" => |t| t.color(ACCENT_FOREGROUND),
}

/// The group each item is, which its content's looks follow
/// (`group-selected/nlb-calendar-item:`, ...).
pub(crate) const ITEM_GROUP: &str = "nlb-calendar-item";

/// The props of [`Calendar`].
#[derive(Debug, Clone, PartialEq, Props)]
pub struct CalendarProps {
    /// The value (controlled): an ISO date (`"2026-10-28"`), `[start,
    /// end]` with `range`, `null` for none. Set, the app owns it and sets
    /// it in `onChange`; left out, the calendar keeps its own.
    #[prop(default)]
    pub value: Prop<CalendarValue>,
    /// The value at first, while `value` is left out.
    #[prop(default)]
    pub default_value: Prop<Date>,
    /// Pick a range: the first press starts it, the second ends it.
    #[prop(default)]
    pub range: Prop<bool>,
    /// How many months show side by side, at least 1; one in a window
    /// narrower than 768 (Tailwind's `md`).
    #[prop(default = 1)]
    pub number_of_months: Prop<u32>,
    /// The day the weeks start on, `0` Sunday to `6` Saturday. Left out,
    /// the user's (`localeSettings().firstDayOfWeek`).
    pub first_day_of_week: Prop<Option<u32>>,
    /// The days that cannot be picked: days of the week (`[0, 6]`, Sunday
    /// 0), `{ before, after }`, `{ from, to }`, or a function of the ISO
    /// date (asked once per shown day).
    pub disabled_matcher: Prop<Option<Matcher>>,
    /// The years the year view pages through, `[start, end)` (default: 50
    /// years before this one to 50 after).
    pub year_range: Prop<Option<YearRange>>,
    /// Its size (`xs`, `sm`, `md`, `lg`): the cells (a `Button`'s height
    /// of that size, at least 44 on touch screens), the text and the icons.
    #[prop(default)]
    pub size: Prop<Size>,
    /// What screen readers say for Previous ("Go to the previous month").
    pub previous_accessibility_label: Prop<Option<SharedString>>,
    /// What screen readers say for Next ("Go to the next month").
    pub next_accessibility_label: Prop<Option<SharedString>>,
    /// A day's label in place of its number (a price under it, an event
    /// dot): the library still draws the cell, the band, the square and
    /// every state. It gets what the day is, never its state; it is called
    /// for the days only.
    pub render_item: Prop<Option<Renderer<CalendarEntry>>>,
    /// Tailwind classes of the header: Previous, the toggles (or the
    /// months' titles), Next.
    pub header_class: Prop<Option<SharedString>>,
    /// The inline style of the header, over the library's look.
    #[prop(default)]
    pub header_style: Prop<Style>,
    /// Tailwind classes of the body: the months, or the month or year view.
    pub body_class: Prop<Option<SharedString>>,
    /// The inline style of the body, over the library's look.
    #[prop(default)]
    pub body_style: Prop<Style>,
    /// Tailwind classes of each month: its weekdays and weeks.
    pub month_class: Prop<Option<SharedString>>,
    /// The inline style of each month, over the library's look.
    #[prop(default)]
    pub month_style: Prop<Style>,
    /// Tailwind classes of each row: the weekdays, a week, a row of months
    /// or years.
    pub row_class: Prop<Option<SharedString>>,
    /// The inline style of each row, over the library's look.
    #[prop(default)]
    pub row_style: Prop<Style>,
    /// Tailwind classes of each item (a day cell, a weekday, a toggle,
    /// Previous, Next, a month, a year), after the library's: `selected:`,
    /// `disabled:`, `focus-visible:` and on the days `data-range-start:`,
    /// `data-range-end:`, `data-range-middle:`, `data-range-preview-*:`,
    /// `data-today:`, `data-outside:`.
    pub item_class: Prop<Option<SharedString>>,
    /// The inline style of each item, over the library's look.
    #[prop(default)]
    pub item_style: Prop<TextPartStyle>,
}

/// The events of a [`Calendar`].
#[derive(Debug, Clone, PartialEq, Event)]
pub enum CalendarEvent {
    /// The user picked a complete value: a day, or a range's end. A
    /// `value` the app passes is never reported.
    #[args(value)]
    Change(Date),
}

/// A calendar: a month's days to pick a date or a range of
/// dates from, with Previous and Next, the month and year toggles, several
/// months side by side, disabled days and the week starting on the user's
/// day. The days are ghost buttons; today, the selected days and a range's
/// band take the `calendar-*` tokens.
#[component]
pub fn Calendar(props: CalendarProps, cx: Scope<CalendarEvent>) -> impl IntoElement {
    provide_text_color();
    let t = theme();
    let size = computed(move || props.size.get());
    provide_icon_size(move || size.get());
    let first_day = computed(move || first_day_of_week(props.first_day_of_week.get()));
    let months = computed(move || shown_months(props.number_of_months.get()));
    // Transparent inside a library `Popover` or `Card`: their surface
    // shows through.
    let transparent = popup::in_popup() || in_card();
    let look = look(
        t,
        LookConfig {
            size,
            first_day,
            months,
            parts: Parts {
                header_class: props.header_class,
                header_style: props.header_style,
                body_class: props.body_class,
                body_style: props.body_style,
                month_class: props.month_class,
                month_style: props.month_style,
                row_class: props.row_class,
                row_style: props.row_style,
                item_class: props.item_class,
                item_style: props.item_style,
            },
            render_item: props.render_item,
        },
    );
    let mut calendar = base_calendar()
        .value(props.value)
        .default_value(props.default_value)
        .range(props.range)
        .number_of_months(move || months.get())
        .first_day_of_week(move || first_day.get())
        .disabled_matcher(props.disabled_matcher)
        .year_range(props.year_range)
        .previous_accessibility_label(move || {
            Some(
                props
                    .previous_accessibility_label
                    .get()
                    .unwrap_or_else(|| labels::text(Label::PreviousMonth)),
            )
        })
        .next_accessibility_label(move || {
            Some(
                props
                    .next_accessibility_label
                    .get()
                    .unwrap_or_else(|| labels::text(Label::NextMonth)),
            )
        })
        .on_change(move |value: &Date| cx.emit(CalendarEvent::Change(*value)));
    let parts = calendar.__element().__props_mut();
    parts.header_class = look.header_class;
    parts.header_style = look.header_style;
    parts.body_class = look.body_class;
    parts.body_style = look.body_style;
    parts.month_class = look.month_class;
    parts.month_style = look.month_style;
    parts.row_class = look.row_class;
    parts.row_style = look.row_style;
    parts.item_class = look.item_class;
    parts.item_style = look.item_style;
    parts.render_item = look.render_item;
    view()
        .p(move || t.length(CALENDAR_PADDING))
        .bg(move || {
            if transparent {
                AdaptiveColor::TRANSPARENT
            } else {
                t.color(CALENDAR_BACKGROUND)
            }
        })
        .text_color(move || t.color(CALENDAR_DAY_FOREGROUND))
        // As wide as its content (`w-fit`); the app's class goes
        // over it.
        .app_class(Some(SharedString::from("w-fit")))
        .child(calendar)
}

/// The weeks' first day: the app's, else the user's (`0` Sunday). Read it
/// in a binding.
pub(crate) fn first_day_of_week(app: Option<u32>) -> u32 {
    app.map(|day| day % 7)
        .unwrap_or_else(|| u32::from(locale_settings().first_day_of_week.number()) % 7)
}

/// How many months show for `asked`: at least one, and one in a window
/// narrower than Tailwind's `md` (768), where one header row over stacked
/// months would not line up. Read it in a binding.
pub(crate) fn shown_months(asked: u32) -> u32 {
    if f32::from(window().width) < 768.0 {
        1
    } else {
        asked.max(1)
    }
}

/// The cell size token of `size`.
pub(crate) fn cell_size_token(size: Size) -> Token<kind::Length> {
    size.pick(
        CALENDAR_CELL_SIZE_XS,
        CALENDAR_CELL_SIZE_SM,
        CALENDAR_CELL_SIZE_MD,
        CALENDAR_CELL_SIZE_LG,
    )
}

/// The app's classes and styles of a calendar's parts.
#[derive(Clone, Copy, Default)]
pub(crate) struct Parts {
    pub(crate) header_class: Prop<Option<SharedString>>,
    pub(crate) header_style: Prop<Style>,
    pub(crate) body_class: Prop<Option<SharedString>>,
    pub(crate) body_style: Prop<Style>,
    pub(crate) month_class: Prop<Option<SharedString>>,
    pub(crate) month_style: Prop<Style>,
    pub(crate) row_class: Prop<Option<SharedString>>,
    pub(crate) row_style: Prop<Style>,
    pub(crate) item_class: Prop<Option<SharedString>>,
    pub(crate) item_style: Prop<TextPartStyle>,
}

/// What the calendar look is drawn from.
pub(crate) struct LookConfig {
    /// The cells', the text's and the icons' size.
    pub(crate) size: Memo<Size>,
    /// The weeks' first day (`0` Sunday), as given to `nsh/base`.
    pub(crate) first_day: Memo<u32>,
    /// How many months show, as given to `nsh/base`.
    pub(crate) months: Memo<u32>,
    /// The app's classes and styles of the parts.
    pub(crate) parts: Parts,
    /// The app's drawing of a day's label.
    pub(crate) render_item: Prop<Option<Renderer<CalendarEntry>>>,
}

/// The calendar look, as `nsh/base`'s calendar part props (a `Calendar`'s
/// or a `DatePicker`'s): the parts' styles (the library's, then the
/// app's), the items' classes (the group the content follows, then the
/// app's) and the library's `renderItem`.
pub(crate) struct Look {
    pub(crate) header_class: Prop<Option<SharedString>>,
    pub(crate) header_style: Prop<Style>,
    pub(crate) body_class: Prop<Option<SharedString>>,
    pub(crate) body_style: Prop<Style>,
    pub(crate) month_class: Prop<Option<SharedString>>,
    pub(crate) month_style: Prop<Style>,
    pub(crate) row_class: Prop<Option<SharedString>>,
    pub(crate) row_style: Prop<Style>,
    pub(crate) item_class: Prop<Option<SharedString>>,
    pub(crate) item_style: Prop<TextPartStyle>,
    pub(crate) render_item: Prop<Option<Renderer<CalendarEntry>>>,
}

fn bound_style(f: impl Fn() -> Style + 'static) -> Prop<Style> {
    Prop::from(IntoReactive::<Style>::into_reactive(f))
}

/// The calendar look on `nsh/base`'s parts. Each part's style is bound, so
/// a token change or a `ThemeScope` reaches it; the app's style goes over
/// it.
pub(crate) fn look(t: Theme, config: LookConfig) -> Look {
    let LookConfig {
        size,
        first_day,
        months,
        parts,
        render_item,
    } = config;
    let cell = move || t.length(cell_size_token(size.get()));
    let header_style = bound_style(move || {
        // Previous and Next at the ends, as wide as a cell; the toggles (or
        // one title per month) share the rest, so the two toggles meet in
        // the middle and each title is over its month.
        let n = months.get().max(1);
        let columns = if n == 1 {
            "auto 1fr 1fr auto".to_string()
        } else {
            format!("auto repeat({n}, 1fr) auto")
        };
        let look = StyleRefinement::default()
            .grid()
            .grid_template_columns(SharedString::from(columns))
            .items_center()
            .gap_x(t.length(SPACING))
            .h(cell())
            .mb(t.length(CALENDAR_GAP));
        Style::from(look).then(&parts.header_style.get())
    });
    let body_style = bound_style(move || {
        // The day view's width, also the month and year views': the
        // calendar keeps its width as the view changes.
        let n = months.get().max(1) as f32;
        let width = cell() * 7.0 * n + t.length(CALENDAR_MONTH_GAP) * (n - 1.0);
        let look = StyleRefinement::default()
            .gap_x(t.length(CALENDAR_MONTH_GAP))
            .gap_y(t.length(CALENDAR_ROW_GAP))
            .min_w(width);
        Style::from(look).then(&parts.body_style.get())
    });
    let month_style = bound_style(move || {
        let look = StyleRefinement::default().gap(t.length(CALENDAR_ROW_GAP));
        Style::from(look).then(&parts.month_style.get())
    });
    let row_style =
        bound_style(move || Style::from(StyleRefinement::default()).then(&parts.row_style.get()));
    let item_style = Prop::from(IntoReactive::<TextPartStyle>::into_reactive(move || {
        let font_size = t.length(font_size_token(size.get()));
        let look = StyleRefinement::default()
            .relative()
            .p(px(0.0))
            .text_size(font_size)
            .line_height(line_height(font_size))
            .font_weight(FontWeight::NORMAL)
            .whitespace_nowrap()
            .cursor(Cursor::Default);
        TextPartStyle::from(Style::from(look).then(&Style::from(parts.item_style.get())))
    }));
    let item_class = Prop::from(IntoReactive::<Option<SharedString>>::into_reactive(
        move || Some(item_class(t, parts.item_class.get())),
    ));
    let draw = Rc::new(move |entry: &CalendarEntry| {
        content(t, entry, size, first_day, months, render_item).into_element()
    });
    let render_item = Prop::from(Some(Renderer::new(move |entry: &CalendarEntry| {
        draw(entry)
    })));
    Look {
        header_class: parts.header_class,
        header_style,
        body_class: parts.body_class,
        body_style,
        month_class: parts.month_class,
        month_style,
        row_class: parts.row_class,
        row_style,
        item_class,
        item_style,
        render_item,
    }
}

/// `group-<state>/nlb-calendar-item:` for each of `states`, as one
/// variant chain (ending in `:`, to prefix a utility).
fn in_item(states: &[&str]) -> String {
    states
        .iter()
        .map(|s| format!("group-{s}/{ITEM_GROUP}:"))
        .collect()
}

/// The variant chain of [`in_item`] without its last `:`, for
/// [`Classes::color`].
fn on_item(states: &[&str]) -> String {
    let mut chain = in_item(states);
    chain.pop();
    chain
}

/// The items' classes: the group their content follows, the keyboard's
/// day over its neighbours (its ring), dimmed while disabled, then the
/// app's.
fn item_class(t: Theme, app: Option<SharedString>) -> SharedString {
    Classes::new()
        .push(&format!("group/{ITEM_GROUP} focus-visible:z-10"))
        .number("disabled", "opacity", t.number(OPACITY_DISABLED))
        .then(app)
}

/// The text colors of a day's square by its item's states, as classes
/// from the tokens (an inline style cannot take a state). NSH orders a
/// rule by its variants: `group-selected` before `group-hover` before
/// `group-disabled` before every `group-data-*`, the `group-data-*` ones
/// among themselves in the order written, and a chain of variants after
/// its last one alone. So the rules are written lowest first, and a state
/// that must win over a later-ranked one is chained with it.
fn day_text_class(t: Theme) -> SharedString {
    let day = t.color(CALENDAR_DAY_FOREGROUND);
    let outside = t.color(CALENDAR_OUTSIDE_FOREGROUND);
    let today = t.color(CALENDAR_TODAY_FOREGROUND);
    let hover = t.color(BUTTON_GHOST_HOVER_FOREGROUND);
    let selected = t.color(CALENDAR_SELECTED_FOREGROUND);
    let range = t.color(CALENDAR_RANGE_FOREGROUND);
    let mut c = Classes::new();
    c = c.color("", "text", day);
    c = c.color(&on_item(&["selected"]), "text", selected);
    c = c.color(&on_item(&["hover"]), "text", hover);
    c = c.color(&on_item(&["selected", "hover"]), "text", selected);
    c = c.color(&on_item(&["disabled"]), "text", outside);
    // The `data-*` states, lowest first.
    c = c.color(&on_item(&["data-outside"]), "text", outside);
    c = c.color(&on_item(&["data-today"]), "text", today);
    for state in ["data-range-middle", "data-range-preview-middle"] {
        c = c.color(&on_item(&[state]), "text", range);
        c = c.color(&on_item(&[state, "data-outside"]), "text", outside);
    }
    for state in ["data-range-start", "data-range-end"] {
        c = c.color(&on_item(&[state]), "text", selected);
    }
    // A hovered day of the months around takes the hover's text, unless
    // selected.
    c = c.color(&on_item(&["hover", "data-outside"]), "text", hover);
    // Selected (a date, a half range's start) wins over today and the
    // months around; a range's middle keeps its own.
    for state in ["data-today", "data-outside"] {
        c = c.color(&on_item(&["selected", state]), "text", selected);
    }
    c = c.color(
        &on_item(&["selected", "hover", "data-outside"]),
        "text",
        selected,
    );
    c = c.color(&on_item(&["selected", "data-range-middle"]), "text", range);
    c = c.color(
        &on_item(&["selected", "data-range-middle", "data-outside"]),
        "text",
        outside,
    );
    c = c.color(
        &on_item(&["selected", "data-range-middle", "data-today"]),
        "text",
        range,
    );
    c = c.color(&on_item(&["disabled", "data-today"]), "text", outside);
    c.build()
}

/// The text of a month or a year of those views: the ghost's, selected
/// `calendar-selected-foreground` (also under the pointer).
fn pick_text_class(t: Theme) -> SharedString {
    let selected = t.color(CALENDAR_SELECTED_FOREGROUND);
    let mut c = Classes::new();
    c = c.color(&on_item(&["selected"]), "text", selected);
    c = c.color(&on_item(&["selected", "hover"]), "text", selected);
    c.build()
}

/// A toggle whose view is open keeps the hover's fill and text.
fn toggle_class(t: Theme) -> SharedString {
    let mut c = Classes::new();
    c = c.color(&on_item(&["selected"]), "bg", t.color(BUTTON_GHOST_HOVER));
    c = c.color(
        &on_item(&["selected"]),
        "text",
        t.color(BUTTON_GHOST_HOVER_FOREGROUND),
    );
    c.build()
}

/// A layer filling its parent, shown while `shown` (classes of the
/// item's states) holds: `opacity-0` until then.
fn layer(
    fill: impl Fn() -> AdaptiveColor + 'static,
    radius: impl Fn() -> Points + 'static,
    shown: &str,
) -> View {
    view()
        .absolute()
        .inset(px(0.0))
        .rounded(radius)
        .bg(fill)
        .app_class(Some(SharedString::from(format!("opacity-0 {shown}"))))
}

/// An item's content: what the library draws inside each of `nsh/base`'s
/// items, from what it is. Hidden from screen readers, which hear the
/// item.
fn content(
    t: Theme,
    entry: &CalendarEntry,
    size: Memo<Size>,
    first_day: Memo<u32>,
    months: Memo<u32>,
    app: Prop<Option<Renderer<CalendarEntry>>>,
) -> View {
    let cell = move || t.length(cell_size_token(size.get()));
    let icon_side = move || t.length(icon_size_token(size.get()));
    let radius = move || t.length(BUTTON_RADIUS);
    let el = match entry.kind {
        CalendarItemKind::Day => day(t, entry, size, first_day, app),
        CalendarItemKind::Weekday => {
            let font_size = move || t.length(CALENDAR_WEEKDAY_FONT_SIZE);
            let name = entry.value.max(0) as u32;
            view().min_w(cell).items_center().child(
                text(move || labels::text(Label::weekday(name)))
                    .text_size(font_size)
                    .line_height(move || line_height(font_size()))
                    .text_color(move || t.color(CALENDAR_WEEKDAY_FOREGROUND)),
            )
        }
        CalendarItemKind::Previous | CalendarItemKind::Next => {
            let name = if entry.kind == CalendarItemKind::Previous {
                IconName::ChevronLeft
            } else {
                IconName::ChevronRight
            };
            view()
                .size(cell)
                .items_center()
                .justify_center()
                .rounded(radius)
                .group_hover_named(ITEM_GROUP, move |s| {
                    s.bg(move || t.color(BUTTON_GHOST_HOVER))
                        .text_color(move || t.color(BUTTON_GHOST_HOVER_FOREGROUND))
                })
                .group_focus_visible_named(ITEM_GROUP, move |s| {
                    focus_ring(s, t, move || t.color(BUTTON_RING), None)
                })
                .app_class(Some(SharedString::from(format!(
                    "{}bg-transparent",
                    in_item(&["disabled"])
                ))))
                .child(icon_view(t, move || name, icon_side))
        }
        CalendarItemKind::YearToggle => toggle(t, entry, size),
        // With several months, `nsh/base` gives each month's title the
        // month toggle's kind (and no press).
        CalendarItemKind::MonthToggle if untracked(|| months.get()) == 1 => toggle(t, entry, size),
        CalendarItemKind::MonthToggle => title(t, entry),
        CalendarItemKind::Month | CalendarItemKind::Year => {
            let month = entry.kind == CalendarItemKind::Month;
            let value = entry.value;
            let label = move || -> SharedString {
                if month {
                    labels::text(Label::month_short(value.max(1) as u32))
                } else {
                    value.to_string().into()
                }
            };
            view()
                .flex_1()
                .h(cell)
                .mx(px(2.0))
                .items_center()
                .justify_center()
                .child(layer(
                    move || t.color(BUTTON_GHOST_HOVER),
                    radius,
                    &format!("{}opacity-100", in_item(&["hover"])),
                ))
                .child(layer(
                    move || t.color(CALENDAR_SELECTED),
                    radius,
                    &format!("{}opacity-100", in_item(&["selected"])),
                ))
                .child(
                    text(label)
                        .relative()
                        .app_class(move || Some(pick_text_class(t)))
                        .group_hover_named(ITEM_GROUP, move |s| {
                            s.text_color(move || t.color(BUTTON_GHOST_HOVER_FOREGROUND))
                        }),
                )
                .group_focus_visible_named(ITEM_GROUP, move |s| {
                    focus_ring(s, t, move || t.color(BUTTON_RING), None)
                })
                .rounded(radius)
        }
    };
    el.accessibility_elements_hidden(true)
}

/// The month or year toggle of a one-month calendar: the month's short
/// name (`Oct`) or the year and a chevron, a
/// ghost button a cell tall; the month's toggle sits at the end of its
/// column, the year's at the start, so they meet in the middle. A short
/// name keeps the header within seven cells whatever the month, so the
/// calendar keeps its width as it turns.
fn toggle(t: Theme, entry: &CalendarEntry, size: Memo<Size>) -> View {
    let month = entry.kind == CalendarItemKind::MonthToggle;
    let value = entry.value;
    let cell = move || t.length(cell_size_token(size.get()));
    let font_size = move || t.length(CALENDAR_CAPTION_FONT_SIZE);
    let label = move || -> SharedString {
        if month {
            labels::text(Label::month_short(value.max(1) as u32))
        } else {
            value.to_string().into()
        }
    };
    let el = view()
        .flex_row()
        .items_center()
        .gap(move || t.length(SPACING))
        .h(cell)
        .pl(move || t.length(SPACING) * 2.0)
        .pr(move || t.length(SPACING))
        .rounded(move || t.length(BUTTON_RADIUS))
        .text_size(font_size)
        .line_height(move || line_height(font_size()))
        .font_weight(medium(t))
        .group_hover_named(ITEM_GROUP, move |s| {
            s.bg(move || t.color(BUTTON_GHOST_HOVER))
                .text_color(move || t.color(BUTTON_GHOST_HOVER_FOREGROUND))
        })
        .group_focus_visible_named(ITEM_GROUP, move |s| {
            focus_ring(s, t, move || t.color(BUTTON_RING), None)
        })
        .app_class(move || Some(toggle_class(t)))
        .child(text(label))
        .child(
            icon_view(t, || IconName::ChevronDown, || px(14.0))
                .text_color(move || t.color(CALENDAR_WEEKDAY_FOREGROUND)),
        );
    if month {
        el.self_end()
    } else {
        el.self_start()
    }
}

/// A month's title over it, with several months: "October 2026",
/// centered in its column of the header.
fn title(t: Theme, entry: &CalendarEntry) -> View {
    let (month, year) = (entry.month.max(1), entry.year);
    let font_size = move || t.length(CALENDAR_CAPTION_FONT_SIZE);
    view().items_center().child(
        text(move || format!("{} {year}", labels::text(Label::month(month))))
            .text_size(font_size)
            .line_height(move || line_height(font_size()))
            .font_weight(medium(t)),
    )
}

/// A day: the band behind it while in a range, its square (today, under
/// the pointer, selected) and its number, or the app's label.
fn day(
    t: Theme,
    entry: &CalendarEntry,
    size: Memo<Size>,
    first_day: Memo<u32>,
    app: Prop<Option<Renderer<CalendarEntry>>>,
) -> View {
    let cell = move || t.length(cell_size_token(size.get()));
    let radius = move || t.length(BUTTON_RADIUS);
    let date = entry
        .date
        .as_deref()
        .and_then(|d| NaiveDate::parse_from_str(d, "%Y-%m-%d").ok());
    // The day's column in its week: the band's corners round at the
    // week's first and last.
    let column = move || {
        date.map(|d| (d.weekday().num_days_from_sunday() + 7 - first_day.get() % 7) % 7)
            .unwrap_or(0)
    };
    let band = |start_half: bool| {
        let shown = if start_half {
            [
                "data-range-end",
                "data-range-middle",
                "data-range-preview-end",
                "data-range-preview-middle",
            ]
        } else {
            [
                "data-range-start",
                "data-range-middle",
                "data-range-preview-start",
                "data-range-preview-middle",
            ]
        };
        let mut class = String::from("opacity-0 ");
        for state in shown {
            class.push_str(&format!("{}opacity-100 ", in_item(&[state])));
        }
        // A range of one day (start and end) has no band.
        class.push_str(&format!(
            "{}opacity-0 {}opacity-0",
            in_item(&["data-range-start", "data-range-end"]),
            in_item(&["data-range-preview-start", "data-range-preview-end"]),
        ));
        let el = view()
            .absolute()
            .top(px(0.0))
            .bottom(px(0.0))
            .bg(move || t.color(CALENDAR_RANGE))
            .app_class(Some(SharedString::from(class)));
        if start_half {
            el.left(px(0.0)).right(relative(0.5)).style_with(move |s| {
                if column() == 0 {
                    s.rounded_l(radius())
                } else {
                    s
                }
            })
        } else {
            el.left(relative(0.5)).right(px(0.0)).style_with(move |s| {
                if column() == 6 {
                    s.rounded_r(radius())
                } else {
                    s
                }
            })
        }
    };
    let label = match app.get() {
        Some(render) => {
            let key = format!(
                "day-{}-{:02}-{}",
                entry.year,
                entry.month,
                entry.date.clone().unwrap_or_default()
            );
            render
                .render(key, entry.clone())
                .relative()
                .items_center()
                .justify_center()
                .gap(move || t.length(SPACING))
                .into_element()
        }
        None => text(entry.value.to_string()).relative().into_element(),
    };
    let hover_shown = format!(
        "{}opacity-100 {}opacity-100 {}opacity-0",
        in_item(&["hover"]),
        in_item(&["data-range-preview-end"]),
        in_item(&["disabled"]),
    );
    let selected_shown = format!(
        "{}opacity-100 {}opacity-0",
        in_item(&["selected"]),
        in_item(&["data-range-middle"]),
    );
    let square = view()
        .relative()
        .size(cell)
        .items_center()
        .justify_center()
        .rounded(radius)
        .child(layer(
            move || t.color(CALENDAR_TODAY),
            radius,
            &format!("{}opacity-100", in_item(&["data-today"])),
        ))
        .child(layer(
            move || t.color(BUTTON_GHOST_HOVER),
            radius,
            &hover_shown,
        ))
        .child(layer(
            move || t.color(CALENDAR_SELECTED),
            radius,
            &selected_shown,
        ))
        .group_focus_visible_named(ITEM_GROUP, move |s| {
            focus_ring(s, t, move || t.color(BUTTON_RING), None)
        })
        .app_class(move || Some(day_text_class(t)))
        .child(label);
    view()
        .relative()
        .flex_row()
        .min_w(cell)
        .h(cell)
        .items_center()
        .justify_center()
        .child(band(true))
        .child(band(false))
        .child(square)
}
