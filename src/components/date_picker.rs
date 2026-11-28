//! `DatePicker`: the library's date picker on `nsh/base`'s `DatePicker`
//! (the value, the popup at the trigger or the sheet on touch screens, the
//! calendar's behavior, the presets, the time field, the keys, the
//! combobox role and its value for screen readers). The library draws the
//! trigger (the field look), the popup's surface (`foundation::popup`), the
//! calendar (`Calendar`'s look, shared), the presets, the time row and its
//! segments. With `confirm` (nothing committed until Apply) the popup is
//! composed from `nsh/base`'s `Popover` or `Sheet`, `Calendar` and
//! `TimeField`, as `nsh/base`'s `DatePicker` has no place for typed fields
//! or a footer: [`confirmed`]. API: `docs/components/date-picker.md`.

use chrono::{NaiveDate, NaiveDateTime, NaiveTime};
use nsh_base::{
    CalendarValue, Date, DatePickerEvent as BaseDatePickerEvent, OpenChangeReason,
    PopoverEvent as BasePopoverEvent, SheetEvent as BaseSheetEvent, calendar as base_calendar,
    date_picker as base_date_picker, popover as base_popover, sheet as base_sheet,
    terminal::is_terminal, time_field as base_time_field,
};
use nsh_core::{
    element::{Renderer, Style, StyleRefinement, TextPartStyle, locale_settings, safe_area},
    keymap::When,
    prelude::*,
};

pub use nsh_base::{
    DatePickerValue, DateRangePreset, DateTime, HourCycle, PickedDate, TimeOfDay, TimePrecision,
};

use crate::{
    components::{
        button::{
            BUTTON_HEIGHT_SM, BUTTON_OUTLINE, BUTTON_OUTLINE_BORDER, BUTTON_OUTLINE_FOREGROUND,
            BUTTON_OUTLINE_HOVER, BUTTON_OUTLINE_HOVER_FOREGROUND, BUTTON_PADDING_X_SM,
            BUTTON_RADIUS, ButtonVariant, button,
        },
        calendar::{self, CALENDAR_PADDING, CalendarEntry, LookConfig, Matcher, Parts, YearRange},
        dropdown_menu::sheet_look,
        input::{self, input, vertical_hit_slop},
        select::SELECT_TRIGGER_HOVER,
    },
    foundation::{
        class::Classes,
        field::{
            self, INPUT_BACKGROUND, INPUT_BORDER, INPUT_BORDER_FOCUS, INPUT_BORDER_INVALID,
            INPUT_BORDER_WIDTH, INPUT_FOREGROUND, INPUT_PLACEHOLDER_FOREGROUND, INPUT_RADIUS,
            INPUT_RING, INPUT_RING_WIDTH, INPUT_SHADOW, Joined, Kind, field_look,
        },
        field_ref::FieldRef,
        icon::{IconName, icon_size_token, icon_view, provide_icon_size, provide_text_color},
        labels::{self, Label},
        menu::{
            MENU_ITEM_FOCUS, MENU_ITEM_FOCUS_FOREGROUND, MENU_ITEM_PADDING_X, MENU_ITEM_RADIUS,
            MENU_MIN_WIDTH, MENU_PADDING, list_as_sheet,
        },
        popup::{self, POPOVER_MARGIN, POPOVER_OFFSET, set_popup_style},
        shadow,
        size::{Size, pointer},
        trigger,
        typography::{font_size_token, line_height, medium},
    },
    theme::{
        Theme,
        seeds::SPACING,
        semantic::{
            BORDER, BORDER_WIDTH, FONT_SIZE_XS, MUTED_FOREGROUND, OPACITY_DISABLED, OVERLAY,
            POPOVER_FOREGROUND, RADIUS_XS, TEXT_SM,
        },
        theme,
    },
    tokens,
};

tokens! {
    /// The field's fill under the pointer: Select's trigger's, so two
    /// fields that open a list look the same under it.
    DATE_PICKER_TRIGGER_HOVER: Color = "date-picker-trigger-hover" => |t| t.color(SELECT_TRIGGER_HOVER),
    /// The fill of the time segment the keys edit: the menu's row
    /// highlight.
    DATE_PICKER_SEGMENT_FOCUS: Color = "date-picker-segment-focus" => |t| t.color(MENU_ITEM_FOCUS),
    /// The text of the time segment the keys edit.
    DATE_PICKER_SEGMENT_FOCUS_FOREGROUND: Color = "date-picker-segment-focus-foreground" => |t| t.color(MENU_ITEM_FOCUS_FOREGROUND),
    /// The dates under a preset's label (`confirm`): the muted text of
    /// descriptions.
    DATE_PICKER_PRESET_DESCRIPTION: Color = "date-picker-preset-description" => |t| t.color(MUTED_FOREGROUND),
}

/// The field's format without `dateFormat`: `Oct 28, 2026`.
const DATE_FORMAT: &str = "%b %-d, %Y";

/// `value` as text in `format` (`chrono`'s), `None` until complete: a
/// single date as `nsh/base` writes it, a range's ends joined by the
/// library's `value-range` words (`Oct 5, 2026 – Oct 9, 2026`), as a
/// slider's two values, so translators control the dash. Every place the
/// library shows the value as text goes through it.
fn shown_text(value: DateTime, format: &str) -> Option<SharedString> {
    match value {
        DateTime::Range(Some(start), Some(end)) => Some(labels::range(
            &start.format(format).to_string(),
            &end.format(format).to_string(),
        )),
        value => value.format(format),
    }
}

/// The props of [`DatePicker`].
#[derive(Debug, Clone, PartialEq, Props)]
pub struct DatePickerProps {
    /// The value (controlled): an ISO date, `[start, end]` with `range`, a
    /// date and time with `timePrecision` (`"2026-10-28T09:30"`), `null`
    /// for none. The app sets it in `onChange`; left out, the picker keeps
    /// its own, from `defaultValue`.
    #[prop(default)]
    pub value: Prop<DatePickerValue>,
    /// The first value while `value` is left out.
    #[prop(default)]
    pub default_value: Prop<DateTime>,
    /// Picks a range: the first press starts it, the second ends it and
    /// closes the popup (with `confirm`, keeps it open).
    #[prop(default)]
    pub range: Prop<bool>,
    /// Nothing is committed until Apply: a pick or a preset fills the
    /// calendar (and the typed fields) and the popup stays open, Cancel and
    /// Apply at its foot; Escape, a press outside and Cancel drop the
    /// pending value. On desktops the popup has typed Start / End fields
    /// (Date for a single date) and each preset shows its dates. Read once,
    /// when the picker is made; not in a terminal.
    #[prop(default)]
    pub confirm: Prop<bool>,
    /// What the field shows without a value: "Pick a date" (the library's
    /// words) when left out.
    pub placeholder: Prop<Option<SharedString>>,
    /// How the field shows the value and screen readers hear it, in
    /// `chrono`'s format (`"%b %-d, %Y"`, `Oct 28, 2026`, when left out); a
    /// range's ends joined by ` - `; with `timePrecision`, the time after a
    /// space.
    pub date_format: Prop<Option<SharedString>>,
    /// Its size (`xs`, `sm`, `md`, `lg`): the field's height, text,
    /// padding and icon, as an `Input` of that size, and the calendar's
    /// cells.
    #[prop(default)]
    pub size: Prop<Size>,
    /// It does not open, takes no focus, and is dimmed. Or'ed with an
    /// enclosing `Field`'s or `FieldSet`'s.
    #[prop(default)]
    pub disabled: Prop<bool>,
    /// The app's checks reject the value: a red edge (and ring while
    /// focused); screen readers say it is invalid. Or'ed with an enclosing
    /// `Field`'s.
    #[prop(default)]
    pub invalid: Prop<bool>,
    /// Open (controlled): the app sets it in `onOpenChange`. Left out, the
    /// picker keeps its own state.
    pub open: Prop<Option<bool>>,
    /// Open at first, while `open` is left out.
    #[prop(default)]
    pub default_open: Prop<bool>,
    /// Edits the time of day too (a single date only): a time row under
    /// the calendar.
    pub time_precision: Prop<Option<TimePrecision>>,
    /// How the time field counts the hours; left out, the user's (12-hour
    /// where the user's is).
    pub hour_cycle: Prop<Option<HourCycle>>,
    /// The time a date gets before the user sets one (`00:00` when left
    /// out).
    pub default_time: Prop<Option<TimeOfDay>>,
    /// Buttons beside the calendar that set the value ("Today", "Last 7
    /// days"); with `confirm` they fill the calendar instead, and on
    /// desktops show the dates they stand for under the label.
    #[prop(default)]
    pub presets: Prop<Vec<DateRangePreset>>,
    /// How many months show side by side (one in a window narrower than
    /// 768, and in the sheet).
    #[prop(default = 1)]
    pub number_of_months: Prop<u32>,
    /// The weeks' first day, `0` (Sunday) to `6`; left out, the user's.
    pub first_day_of_week: Prop<Option<u32>>,
    /// The days that cannot be picked.
    pub disabled_matcher: Prop<Option<Matcher>>,
    /// The years the year view offers (50 years either side when left
    /// out).
    pub year_range: Prop<Option<YearRange>>,
    /// Desktops: the side of the field the popup goes on (`"bottom"` by
    /// default); the other one when only it has room.
    #[prop(default)]
    pub placement: Prop<Placement>,
    /// Desktops: how the popup lines up with the field (`"start"` by
    /// default).
    #[prop(default = Align::Start)]
    pub align: Prop<Align>,
    /// The app's own trigger in place of the field (an icon `Button` in a
    /// toolbar): it is placed against and announced as the combobox.
    pub trigger: Prop<Option<Slot>>,
    /// What screen readers call it (what the date is about).
    pub accessibility_label: Prop<Option<SharedString>>,
    /// What screen readers call the time field: "Time" (the library's
    /// words) when left out.
    pub time_accessibility_label: Prop<Option<SharedString>>,
    /// A day's label inside the library's cell, in place of its number
    /// (the library still draws the cell's states). Days only.
    pub render_item: Prop<Option<Renderer<CalendarEntry>>>,
    /// Tailwind classes of the field, over the field look.
    pub trigger_class: Prop<Option<SharedString>>,
    /// The inline style of the field, over the field look.
    #[prop(default)]
    pub trigger_style: Prop<Style>,
    /// Tailwind classes of the field's text (the value or the
    /// placeholder).
    pub value_class: Prop<Option<SharedString>>,
    /// The inline style of the field's text.
    #[prop(default)]
    pub value_style: Prop<TextPartStyle>,
    /// Tailwind classes of the field's `calendar` icon.
    pub icon_class: Prop<Option<SharedString>>,
    /// The inline style of the field's `calendar` icon.
    #[prop(default)]
    pub icon_style: Prop<Style>,
    /// Tailwind classes of the popup (or sheet): they add to the library's
    /// look but cannot override what it sets inline (fill, edge, corners,
    /// shadow).
    pub content_class: Prop<Option<SharedString>>,
    /// The inline style of the popup (or sheet), over the library's look.
    #[prop(default)]
    pub content_style: Prop<Style>,
    /// Tailwind classes of the calendar's box in the popup.
    pub calendar_class: Prop<Option<SharedString>>,
    /// The inline style of the calendar's box, over its padding.
    #[prop(default)]
    pub calendar_style: Prop<Style>,
    /// Tailwind classes of the calendar's items, as `Calendar`'s.
    pub item_class: Prop<Option<SharedString>>,
    /// The inline style of the calendar's items, as `Calendar`'s.
    #[prop(default)]
    pub item_style: Prop<TextPartStyle>,
    /// Tailwind classes of the presets' column (row in the sheet).
    pub presets_class: Prop<Option<SharedString>>,
    /// The inline style of the presets' column, over the library's look.
    #[prop(default)]
    pub presets_style: Prop<Style>,
    /// Tailwind classes of each preset: they add to the library's look
    /// (its hover is a class too, which the app's follow).
    pub preset_class: Prop<Option<SharedString>>,
    /// The inline style of each preset, over the library's look.
    #[prop(default)]
    pub preset_style: Prop<TextPartStyle>,
    /// `confirm`, desktops: Tailwind classes of each preset's dates line
    /// (`hidden` hides it).
    pub preset_description_class: Prop<Option<SharedString>>,
    /// `confirm`, desktops: the inline style of each preset's dates line.
    #[prop(default)]
    pub preset_description_style: Prop<TextPartStyle>,
    /// `confirm`, desktops: Tailwind classes of the typed fields' row.
    pub fields_class: Prop<Option<SharedString>>,
    /// `confirm`, desktops: the inline style of the typed fields' row.
    #[prop(default)]
    pub fields_style: Prop<Style>,
    /// `confirm`: Tailwind classes of the footer (Cancel and Apply).
    pub footer_class: Prop<Option<SharedString>>,
    /// `confirm`: the inline style of the footer, over the library's look.
    #[prop(default)]
    pub footer_style: Prop<Style>,
    /// Tailwind classes of the time row (its label and the time field).
    pub time_class: Prop<Option<SharedString>>,
    /// The inline style of the time row, over the library's look.
    #[prop(default)]
    pub time_style: Prop<Style>,
    /// Tailwind classes of the time field (the field look around the
    /// segments).
    pub time_field_class: Prop<Option<SharedString>>,
    /// The inline style of the time field, over the field look.
    #[prop(default)]
    pub time_field_style: Prop<Style>,
    /// Tailwind classes of each segment of the time (hours, minutes,
    /// seconds, AM / PM).
    pub segment_class: Prop<Option<SharedString>>,
    /// The inline style of each segment, over the library's look.
    #[prop(default)]
    pub segment_style: Prop<TextPartStyle>,
}

/// The events of a [`DatePicker`].
#[derive(Debug, Clone, PartialEq, Event)]
pub enum DatePickerEvent {
    /// The user changed the value (a day, a range's end, a preset, the
    /// time, Delete / Backspace on the field): what the app makes `value`.
    #[args(value)]
    Change(PickedDate),
    /// The user asks to open or close the popup, with why: set `open` to
    /// follow it.
    #[args(open, reason)]
    OpenChange(bool, OpenChangeReason),
    /// The popup is shown (a sheet once it came in).
    Open,
    /// The popup is gone, after its exit (whoever closed it).
    Close,
    /// Closed without a choice: Escape, back, a press outside or on the
    /// backdrop.
    Cancel,
}

/// The methods of a [`DatePicker`]'s ref.
pub struct DatePickerApi {
    trigger: ElementRef,
    disabled: Prop<bool>,
    joined: Joined,
}

#[methods]
impl DatePickerApi {
    /// Moves the keyboard focus to the field (the popup stays closed);
    /// nothing while disabled.
    pub fn focus(&self) {
        if !self.disabled.get_untracked() && !untracked(|| self.joined.disabled()) {
            self.trigger.focus();
        }
    }
}

/// A date picker: a date, a range of dates or a date and time, picked from
/// a calendar that a field opens: a popup at the field on desktops, a sheet
/// at the bottom on touch screens. Presets beside the calendar set the
/// value; with `timePrecision` a time row under it edits the time.
#[component(methods = DatePickerApi)]
pub fn DatePicker(props: DatePickerProps, cx: Scope<DatePickerEvent>) -> impl IntoElement {
    provide_text_color();
    let t = theme();
    let size = computed(move || props.size.get());
    provide_icon_size(move || size.get());
    // A label's press focuses the field, as a browser's label does.
    let trigger_ref = element_ref();
    let joined = field::join(cx.__host(), Kind::Focus(trigger_ref), move || {
        props.disabled.get()
    });
    // The field's one line in its middle; an app's trigger says nothing.
    if props.trigger.with_untracked(Option::is_none) {
        joined.report_first_line(move || t.length(input::height_token(props.size.get())) / 2.0);
    }
    cx.methods(DatePickerApi {
        trigger: trigger_ref,
        disabled: props.disabled,
        joined,
    });
    let disabled = move || props.disabled.get() || joined.disabled();
    // The value the field shows: the app's, else its own.
    let own = signal(props.default_value.get_untracked());
    let current = computed(move || match props.value.get() {
        DatePickerValue::Uncontrolled => own.get(),
        DatePickerValue::Controlled(value) => value,
    });
    let hour_cycle = computed(move || {
        props.hour_cycle.get().unwrap_or_else(|| {
            if locale_settings().hour_cycle.twelve_hour() {
                HourCycle::H12
            } else {
                HourCycle::H23
            }
        })
    });
    // One format for the field and for screen readers: the date's, then
    // the time's while the popup edits it.
    let format = computed(move || {
        let date = props
            .date_format
            .get()
            .map_or_else(|| DATE_FORMAT.to_owned(), |f| f.to_string());
        match props.time_precision.get().filter(|_| !props.range.get()) {
            Some(precision) => format!("{date} {}", precision.format(hour_cycle.get())),
            None => date,
        }
    });

    // The open state, held here so that a library `Button` as the app's
    // `trigger` (which takes the press itself) can open it too.
    let own_open = signal(props.default_open.get_untracked());
    let is_open = move || props.open.get().unwrap_or_else(|| own_open.get());
    let one = trigger::OnePress::new();
    let request = move |open: bool, reason: OpenChangeReason| {
        if reason == OpenChangeReason::TriggerPress && !one.first() {
            return;
        }
        if open && untracked(disabled) {
            return;
        }
        if props.open.get_untracked().is_none() {
            own_open.set(open);
        }
        cx.emit(DatePickerEvent::OpenChange(open, reason));
    };
    trigger::provide(
        move || request(!untracked(is_open), OpenChangeReason::TriggerPress),
        is_open,
    );
    // Open, from the request to the end of the exit, the popup counts
    // among the open popups, so the trigger's tooltip stays hidden over it.
    let held = signal(false);
    effect(move || {
        if is_open() {
            held.set(true);
        }
    });
    effect(move || {
        if held.get() {
            untracked(popup::hold_open);
        }
    });

    let first_day = computed(move || calendar::first_day_of_week(props.first_day_of_week.get()));
    // One month in the sheet, as below `md`.
    let months = computed(move || {
        if list_as_sheet() {
            1
        } else {
            calendar::shown_months(props.number_of_months.get())
        }
    });
    let terminal = is_terminal();
    if props.confirm.get_untracked() && !terminal {
        return confirmed(
            props.clone(),
            cx,
            Shared {
                t,
                size,
                current,
                own,
                format,
                hour_cycle,
                first_day,
                months,
                held,
                trigger_ref,
                joined,
            },
            is_open,
            request,
        );
    }
    let mut root = base_date_picker()
        .value(move || DatePickerValue::Controlled(current.get()))
        .open(move || Some(is_open()))
        .range(props.range)
        .disabled(disabled)
        .accessibility_label(props.accessibility_label)
        .date_format(move || Some(SharedString::from(format.get())))
        .time_precision(props.time_precision)
        .hour_cycle(move || hour_cycle.get())
        .default_time(props.default_time)
        .presets(props.presets)
        .number_of_months(move || months.get())
        .first_day_of_week(move || first_day.get())
        .disabled_matcher(props.disabled_matcher)
        .year_range(props.year_range)
        .placement(props.placement)
        .align(props.align)
        .offset(move || t.length(POPOVER_OFFSET))
        .margin(move || t.length(POPOVER_MARGIN))
        .previous_accessibility_label(move || Some(labels::text(Label::PreviousMonth)))
        .next_accessibility_label(move || Some(labels::text(Label::NextMonth)))
        .time_accessibility_label(move || {
            Some(
                props
                    .time_accessibility_label
                    .get()
                    .unwrap_or_else(|| labels::text(Label::Time)),
            )
        })
        .on_change(move |picked: &PickedDate| {
            if props.value.get_untracked() == DatePickerValue::Uncontrolled {
                own.set(picked.value);
            }
            cx.emit(DatePickerEvent::Change(*picked));
        })
        .on_open_change(move |e: &BaseDatePickerEvent| {
            if let BaseDatePickerEvent::OpenChange(open, reason) = e {
                request(*open, *reason);
            }
        })
        .on_open(move || cx.emit(DatePickerEvent::Open))
        .on_close(move || {
            if !untracked(is_open) {
                held.set(false);
            }
            cx.emit(DatePickerEvent::Close)
        })
        .on_cancel(move || cx.emit(DatePickerEvent::Cancel))
        // As wide as its content where a column stretches its children
        // (`w-fit`), while a row's `items-center` still centers it; beside
        // a label in a horizontal field, it shrinks to share the row. The
        // app's class goes over it.
        .app_class(move || {
            Some(SharedString::from(if joined.in_row() {
                "w-fit min-w-0 shrink"
            } else {
                "w-fit"
            }))
        });
    if terminal {
        // `nsh/base`'s terminal calendar and marks, the value as text.
        let shown = move || {
            shown_text(current.get(), &format.get())
                .or_else(|| props.placeholder.get())
                .unwrap_or_else(|| labels::text(Label::PickADate))
        };
        return match props.trigger.with_untracked(Option::is_some) {
            true => root.trigger(props.trigger.place(|trigger| trigger)),
            false => root.trigger(text(shown)),
        }
        .into_element();
    }

    root = if props.trigger.with_untracked(Option::is_some) {
        // The app's trigger: what a label's press focuses and the field's
        // description describes.
        root.trigger(props.trigger.place(move |trigger| {
            trigger
                .element_ref(&trigger_ref)
                .accessibility_described_by_target(move || joined.described_by())
        }))
    } else {
        root.trigger(field_trigger(
            props.clone(),
            t,
            current,
            format,
            trigger_ref,
            joined,
        ))
    };
    root = root
        .popup_class(props.content_class)
        // No `onPosition` on a date picker: no slide, zooming from the
        // middle.
        .popup_starting_class(move || popup::ending_class(popup::animate(t)))
        .popup_ending_class(move || popup::ending_class(popup::animate(t)))
        .backdrop_style(move |s| s.bg(move || t.color(OVERLAY)))
        .calendar_class(props.calendar_class)
        .presets_class(props.presets_class)
        .preset_class(move || Some(preset_class(t, props.preset_class.get())))
        .time_class(props.time_class)
        .time_field_class(move || Some(time_field_class(t, props.time_field_class.get())))
        .segment_class(move || Some(segment_class(t, props.segment_class.get())))
        .time_label(
            text(move || labels::text(Label::Time))
                .text_size(move || t.length(TEXT_SM))
                .line_height(move || line_height(t.length(TEXT_SM)))
                .font_weight(medium(t))
                // Its own color: nsh/base's time row does not pass on the
                // popup's.
                .text_color(move || t.color(POPOVER_FOREGROUND)),
        );
    let content_style = props.content_style;
    set_popup_style(&mut root, move || {
        let look = if list_as_sheet() {
            sheet_look(t).p(px(0.0)).pb(safe_area().bottom)
        } else {
            popup::surface(StyleRefinement::default(), t)
        };
        Style::from(look).then(&content_style.get())
    });

    let look = calendar::look(
        t,
        LookConfig {
            size,
            first_day,
            months,
            parts: Parts {
                item_class: props.item_class,
                item_style: props.item_style,
                ..Parts::default()
            },
            render_item: props.render_item,
        },
    );
    let parts = root.__element().__props_mut();
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
    parts.content_style = bound(move || {
        // Presets beside the calendar; above it in the sheet.
        let look = if list_as_sheet() {
            StyleRefinement::default().flex_col()
        } else {
            StyleRefinement::default().flex_row()
        };
        Style::from(look)
    });
    parts.calendar_style = bound(move || {
        let look = StyleRefinement::default().p(t.length(CALENDAR_PADDING));
        Style::from(look).then(&props.calendar_style.get())
    });
    parts.presets_style = bound(move || {
        Style::from(presets_look(t, list_as_sheet())).then(&props.presets_style.get())
    });
    parts.preset_style = Prop::from(IntoReactive::<TextPartStyle>::into_reactive(move || {
        let look = preset_look(t, list_as_sheet()).h(t.length(BUTTON_HEIGHT_SM));
        TextPartStyle::from(Style::from(look).then(&Style::from(props.preset_style.get())))
    }));
    parts.time_style = bound(move || Style::from(time_row_look(t)).then(&props.time_style.get()));
    parts.time_field_style =
        bound(move || time_field_style(t, size.get(), props.time_field_style.get()));
    parts.segment_style = Prop::from(IntoReactive::<TextPartStyle>::into_reactive(move || {
        segment_style(t, props.segment_style.get())
    }));
    parts.separator_style = Prop::from(IntoReactive::<TextPartStyle>::into_reactive(move || {
        separator_style(t)
    }));
    root.into_element()
}

/// The presets' column beside the calendar, or (`sheet`) their wrapping
/// row above it.
fn presets_look(t: Theme, sheet: bool) -> StyleRefinement {
    let padding = t.length(CALENDAR_PADDING);
    if sheet {
        StyleRefinement::default()
            .flex_row()
            .flex_wrap()
            .gap(t.length(SPACING) * 2.0)
            .p(padding)
            .pb(px(0.0))
    } else {
        // A menu's list: its padding, its rows touching, so a row in the
        // popup's corner is concentric with it (`menu-item-radius`).
        StyleRefinement::default()
            .flex_col()
            .p(t.length(MENU_PADDING))
            .min_w(t.length(MENU_MIN_WIDTH))
            .border_r(t.length(BORDER_WIDTH))
            .border_color(t.color(BORDER))
    }
}

/// A preset button's look but its height: a menu row beside the calendar,
/// outline (`sheet`) in the sheet's row.
fn preset_look(t: Theme, sheet: bool) -> StyleRefinement {
    let font_size = t.length(font_size_token(Size::Sm));
    let look = StyleRefinement::default()
        .flex_row()
        .items_center()
        .flex_shrink_0()
        // A button's corners in the sheet's row, a menu row's beside the
        // calendar.
        .rounded(if sheet {
            t.length(BUTTON_RADIUS)
        } else {
            t.length(MENU_ITEM_RADIUS)
        })
        .text_size(font_size)
        .line_height(line_height(font_size))
        .font_weight(FontWeight::NORMAL)
        .whitespace_nowrap()
        .cursor(Cursor::Default);
    if sheet {
        look.px(t.length(BUTTON_PADDING_X_SM))
            .border(t.length(BORDER_WIDTH))
            .border_color(t.color(BUTTON_OUTLINE_BORDER))
    } else {
        look.px(t.length(MENU_ITEM_PADDING_X))
    }
}

/// The time row under the calendar: an edge above, `calendar-padding`.
fn time_row_look(t: Theme) -> StyleRefinement {
    StyleRefinement::default()
        .gap(t.length(SPACING) * 2.0)
        .p(t.length(CALENDAR_PADDING))
        .border_t(t.length(BORDER_WIDTH))
        .border_color(t.color(BORDER))
}

/// The time field's look at `size` (the field look, but its edge's color,
/// which is a class: it changes with `focus-within:`, and an inline color
/// would win), then the app's style.
fn time_field_style(t: Theme, size: Size, app: Style) -> Style {
    let font_size = t.length(input::font_size_token(size));
    let look = StyleRefinement::default()
        .relative()
        .items_center()
        .h(t.length(input::height_token(size)))
        .px(t.length(input::padding_token(size)))
        .rounded(t.length(INPUT_RADIUS))
        .border(t.length(INPUT_BORDER_WIDTH))
        .bg(t.color(INPUT_BACKGROUND))
        .text_color(t.color(INPUT_FOREGROUND))
        .text_size(font_size)
        .line_height(line_height(font_size))
        .whitespace_nowrap()
        .cursor(Cursor::Default);
    Style::from(look)
        .then(&shadow::shadow(t, shadow::level(t.number(INPUT_SHADOW))))
        .then(&app)
}

/// A time segment's look, then the app's style.
fn segment_style(t: Theme, app: TextPartStyle) -> TextPartStyle {
    let look = StyleRefinement::default()
        .px(px(2.0))
        .rounded(t.length(RADIUS_XS))
        .tabular_nums();
    TextPartStyle::from(Style::from(look).then(&Style::from(app)))
}

/// The `:` between the time's segments.
fn separator_style(t: Theme) -> TextPartStyle {
    TextPartStyle::from(Style::from(
        StyleRefinement::default().text_color(t.color(INPUT_PLACEHOLDER_FOREGROUND)),
    ))
}

/// A style prop bound to `f`: run again when what it reads changes.
fn bound(f: impl Fn() -> Style + 'static) -> Prop<Style> {
    Prop::from(IntoReactive::<Style>::into_reactive(f))
}

/// The field: the `calendar` icon, then the value in the format or the
/// placeholder, both muted without a value. `nsh/base` makes it the
/// combobox (its role, value, states, keys and press) and places the popup
/// against it. Its view carries `trigger_ref` and the field's described-by
/// target: `nsh/base`'s slot binds them to the combobox element it makes of
/// it.
fn field_trigger(
    props: DatePickerProps,
    t: Theme,
    current: Memo<DateTime>,
    format: Memo<String>,
    trigger_ref: ElementRef,
    joined: Joined,
) -> View {
    let size = move || props.size.get();
    let height = move || t.length(input::height_token(size()));
    let font_size = move || t.length(input::font_size_token(size()));
    let invalid = move || props.invalid.get() || joined.invalid();
    let shown = computed(move || format.with(|f| shown_text(current.get(), f)));
    let color = move || {
        if shown.with(Option::is_some) {
            t.color(INPUT_FOREGROUND)
        } else {
            t.color(INPUT_PLACEHOLDER_FOREGROUND)
        }
    };
    let el = view()
        .element_ref(&trigger_ref)
        .accessibility_described_by_target(move || joined.described_by())
        .flex_row()
        .items_center()
        .gap(move || t.length(SPACING) * 2.0)
        .h(height)
        .px(move || t.length(input::padding_token(size())))
        .text_size(font_size)
        .line_height(move || line_height(font_size()))
        .font_weight(FontWeight::NORMAL)
        .whitespace_nowrap()
        .cursor(Cursor::Default)
        .hit_slop(move || vertical_hit_slop(t, height()))
        .accessibility_state(move || AccessibilityState {
            invalid: invalid(),
            ..AccessibilityState::default()
        })
        .hover(move |s| s.bg(move || t.color(DATE_PICKER_TRIGGER_HOVER)))
        // Open, it keeps the focus edge (no ring).
        .expanded(move |s| {
            s.border_color(move || {
                if invalid() {
                    t.color(INPUT_BORDER_INVALID)
                } else {
                    t.color(INPUT_BORDER_FOCUS)
                }
            })
        });
    let icon = icon_view(
        t,
        || IconName::Calendar,
        move || t.length(icon_size_token(size())),
    )
    .text_color(color)
    .app_class(props.icon_class)
    .inline_style(move || props.icon_style.get());
    let value = text(move || {
        shown
            .get()
            .or_else(|| props.placeholder.get())
            .unwrap_or_else(|| labels::text(Label::PickADate))
    })
    .flex_1()
    .min_w(px(0.0))
    .truncate()
    .text_color(color)
    .app_class(props.value_class)
    .inline_style(move || Style::from(props.value_style.get()));
    field_look(el, t, invalid)
        .app_class(props.trigger_class)
        .inline_style(move || props.trigger_style.get())
        .child(icon)
        .child(value)
}

/// Rust: `label().html_for(due)` with `due` this date picker's ref.
impl From<ComponentRef<DatePicker>> for FieldRef {
    fn from(r: ComponentRef<DatePicker>) -> Self {
        FieldRef::lazy(move || r.node())
    }
}

nsh_core::static_reactive!(FieldRef: ComponentRef<DatePicker>);

/// The presets' classes: the hover (and the press, for a finger), a menu
/// row's highlight beside the calendar (the column is a menu's list) and
/// outline in the sheet, from the tokens, which an inline style cannot
/// take; then the app's.
fn preset_class(t: Theme, app: Option<SharedString>) -> SharedString {
    let mut class = Classes::new();
    let (rest, rest_text, fill, text) = if list_as_sheet() {
        (
            Some(t.color(BUTTON_OUTLINE)),
            t.color(BUTTON_OUTLINE_FOREGROUND),
            t.color(BUTTON_OUTLINE_HOVER),
            t.color(BUTTON_OUTLINE_HOVER_FOREGROUND),
        )
    } else {
        (
            None,
            t.color(POPOVER_FOREGROUND),
            t.color(MENU_ITEM_FOCUS),
            t.color(MENU_ITEM_FOCUS_FOREGROUND),
        )
    };
    if let Some(rest) = rest {
        class = class.color("", "bg", rest);
    }
    class = class.color("", "text", rest_text);
    // `selected`: the confirmed picker's preset whose value is pending.
    for state in ["hover", "active", "selected"] {
        class = class.color(state, "bg", fill);
        class = class.color(state, "text", text);
    }
    class.then(app)
}

/// The time field's classes: the field look's edge color, the focus edge
/// and (with a pointer) ring while the native field inside has the focus
/// (`focus-within:`), the disabled opacity; then the app's.
fn time_field_class(t: Theme, app: Option<SharedString>) -> SharedString {
    let mut class = Classes::new();
    class = class.color("", "border", t.color(INPUT_BORDER));
    class = class.color("focus-within", "border", t.color(INPUT_BORDER_FOCUS));
    if pointer() {
        class = class.push(&format!(
            "focus-within:ring-[{}px]",
            f32::from(t.length(INPUT_RING_WIDTH))
        ));
        class = class.color("focus-within", "ring", t.color(INPUT_RING));
    }
    class = class.push(&format!(
        "disabled:opacity-[{}]",
        t.number(OPACITY_DISABLED)
    ));
    class.then(app)
}

/// The segments' classes: the edited one's highlight (`focus:`, which
/// `nsh/base` projects on it); then the app's.
fn segment_class(t: Theme, app: Option<SharedString>) -> SharedString {
    let mut class = Classes::new();
    class = class.color("focus", "bg", t.color(DATE_PICKER_SEGMENT_FOCUS));
    class = class.color(
        "focus",
        "text",
        t.color(DATE_PICKER_SEGMENT_FOCUS_FOREGROUND),
    );
    class.then(app)
}

/// What the picker's body computes that the confirmed path shares.
#[derive(Clone, Copy)]
struct Shared {
    t: Theme,
    size: Memo<Size>,
    /// The committed value: the app's, else the picker's own.
    current: Memo<DateTime>,
    own: Signal<DateTime>,
    /// The field's format (with the time's, as the field shows it).
    format: Memo<String>,
    hour_cycle: Memo<HourCycle>,
    first_day: Memo<u32>,
    months: Memo<u32>,
    held: Signal<bool>,
    trigger_ref: ElementRef,
    joined: Joined,
}

/// `value` in the shape `range` asks for: a single date as a range's start,
/// a range's start as the single date.
fn shaped(value: DateTime, range: bool) -> DateTime {
    match (value, range) {
        (DateTime::Single(start), true) => DateTime::Range(start, None),
        (DateTime::Range(start, _), false) => DateTime::Single(start),
        (value, _) => value,
    }
}

/// A value Apply can commit: a date, or a range with both ends.
fn complete(value: &DateTime) -> bool {
    matches!(
        value,
        DateTime::Single(Some(_)) | DateTime::Range(Some(_), Some(_))
    )
}

/// The end of `value` a typed field edits: `0` the start (or the single
/// date), `1` a range's end.
fn end_at(value: &DateTime, at: usize) -> Option<NaiveDateTime> {
    if at == 0 { value.start() } else { value.end() }
}

/// What a typed field's text reads as, `format` (`chrono`'s) first, then
/// an ISO date: `Ok(None)` when empty, `Err` when it reads as neither.
pub(crate) fn read_typed(text: &str, format: &str) -> Result<Option<NaiveDate>, ()> {
    let text = text.trim();
    if text.is_empty() {
        return Ok(None);
    }
    NaiveDate::parse_from_str(text, format)
        .or_else(|_| NaiveDate::parse_from_str(text, "%Y-%m-%d"))
        .map(Some)
        .map_err(|_| ())
}

/// `value` with its end `at` typed as `day`, at the time that end had,
/// else `time`; `None` when refused (an end before the start). A start
/// after the end starts the range again there, as a pick there does.
fn typed(value: DateTime, at: usize, day: Option<NaiveDate>, time: NaiveTime) -> Option<DateTime> {
    let keep = |was: Option<NaiveDateTime>| day.map(|d| d.and_time(was.map_or(time, |v| v.time())));
    match value {
        DateTime::Single(was) => Some(DateTime::Single(keep(was))),
        DateTime::Range(start, end) if at == 0 => {
            let start = keep(start);
            let end = end.filter(|e| start.is_none_or(|s| s.date() <= e.date()));
            Some(DateTime::Range(start, end))
        }
        DateTime::Range(start, end) => {
            if let (Some(s), Some(d)) = (start, day)
                && d < s.date()
            {
                return None;
            }
            Some(DateTime::Range(start, keep(end)))
        }
    }
}

/// Today in UTC: the typed fields' example (a placeholder, where a day off
/// around midnight does not matter).
fn example_day() -> NaiveDate {
    let ms = nsh_core::time::unix_ms();
    chrono::DateTime::from_timestamp_millis(ms as i64)
        .unwrap_or_default()
        .date_naive()
}

/// The confirmed picker (`confirm`): nothing is committed until Apply.
///
/// `nsh/base`'s `DatePicker` commits a complete pick and has no place for
/// typed fields or a footer, so the popup is composed from `nsh/base`'s
/// public parts: on desktops a `Popover` at the field (its anchoring,
/// flipping, Escape, press outside and focus back), on touch screens a
/// `Sheet` (its `footer` holds Cancel and Apply); `nsh/base`'s `Calendar`
/// (controlled with the pending value) and `TimeField`, in the library's
/// looks. The library adds the pending value, the typed fields' parsing,
/// the presets with their dates and the footer, and binds the field's keys
/// as `nsh/base`'s `DatePicker` does.
fn confirmed(
    props: DatePickerProps,
    cx: Scope<DatePickerEvent>,
    shared: Shared,
    is_open: impl Fn() -> bool + Copy + 'static,
    request: impl Fn(bool, OpenChangeReason) + Copy + 'static,
) -> AnyElement {
    let Shared {
        t,
        size,
        current,
        own,
        format,
        hour_cycle,
        first_day,
        months,
        held,
        trigger_ref,
        joined,
    } = shared;
    let disabled = move || props.disabled.get() || joined.disabled();
    let sheet = list_as_sheet();
    let default_time = move || props.default_time.get().map_or(NaiveTime::MIN, |t| t.0);
    // The typed fields' format: the date's alone (the time has its row).
    let date_format = computed(move || {
        props
            .date_format
            .get()
            .map_or_else(|| DATE_FORMAT.to_owned(), |f| f.to_string())
    });
    let show = move |v: Option<NaiveDateTime>| {
        v.map(|v| v.format(&date_format.get_untracked()).to_string())
            .unwrap_or_default()
    };

    // The pending value; what the calendar is given (set by all but the
    // calendar's own picks, so a pick does not turn its months); the time
    // of a single date; the typed fields' texts and whether each shows
    // refused.
    let pending = signal(DateTime::default());
    let shown = signal(Date::default());
    let time = signal(NaiveTime::MIN);
    let texts = [signal(String::new()), signal(String::new())];
    let refused = [signal(false), signal(false)];
    let fill = move |value: DateTime, from_calendar: bool| {
        batch(|| {
            pending.set(value);
            if !from_calendar {
                shown.set(value.date());
            }
            for at in 0..2 {
                texts[at].set(show(end_at(&value, at)));
                refused[at].set(false);
            }
        })
    };
    // Each opening starts from the committed value.
    effect(move || {
        if is_open() {
            untracked(|| {
                let value = shaped(current.get(), props.range.get());
                time.set(value.start().map_or_else(default_time, |v| v.time()));
                fill(value, false);
            });
        }
    });
    let can_apply = move || complete(&pending.get()) && !refused[0].get() && !refused[1].get();
    let apply = move || {
        if !untracked(can_apply) {
            return;
        }
        let value = pending.get_untracked();
        if props.value.get_untracked() == DatePickerValue::Uncontrolled {
            own.set(value);
        }
        let mut picked = PickedDate::default();
        picked.value = value;
        picked.time_precision = props.time_precision.get_untracked();
        cx.emit(DatePickerEvent::Change(picked));
        request(false, OpenChangeReason::Select);
    };
    let cancel = move || {
        cx.emit(DatePickerEvent::Cancel);
        request(false, OpenChangeReason::Cancel);
    };
    // A typed field's text read: a date that reads becomes that end of the
    // pending value; `false` when refused.
    let take = move |at: usize, text: &str| -> bool {
        let Ok(day) = read_typed(text, &date_format.get_untracked()) else {
            return false;
        };
        let off = day.is_some_and(|d| {
            props
                .disabled_matcher
                .with_untracked(|m| m.as_ref().and_then(|m| m.matched(&d)) == Some(true))
        });
        if off {
            return false;
        }
        let fallback = if props.range.get_untracked() {
            untracked(default_time)
        } else {
            time.get_untracked()
        };
        let Some(next) = typed(pending.get_untracked(), at, day, fallback) else {
            return false;
        };
        batch(|| {
            pending.set(next);
            shown.set(next.date());
            // A start after the end dropped the end.
            if at == 0 && next.end().is_none() && props.range.get_untracked() {
                texts[1].set(String::new());
                refused[1].set(false);
            }
        });
        true
    };
    let changed = move |at: usize, text: String| {
        let read = take(at, &text);
        texts[at].set(text);
        if read {
            refused[at].set(false);
        }
    };
    // Leaving a field: a date that reads is written in the format again;
    // what does not stays as typed, shown refused.
    let left = move |at: usize| {
        let text = texts[at].get_untracked();
        if take(at, &text) {
            texts[at].set(show(end_at(&pending.get_untracked(), at)));
            refused[at].set(false);
        } else {
            refused[at].set(true);
        }
    };
    let choose = move |value: DateTime| {
        let value = shaped(value, props.range.get_untracked());
        if !props.range.get_untracked()
            && let Some(start) = value.start()
        {
            time.set(start.time());
        }
        fill(value, false);
    };

    // The parts, made on each opening.
    let calendar = move || {
        let look = calendar::look(
            t,
            LookConfig {
                size,
                first_day,
                months,
                parts: Parts {
                    item_class: props.item_class,
                    item_style: props.item_style,
                    ..Parts::default()
                },
                render_item: props.render_item,
            },
        );
        let mut grid = base_calendar()
            .value(move || CalendarValue::Controlled(shown.get()))
            .range(props.range)
            .number_of_months(move || months.get())
            .first_day_of_week(move || first_day.get())
            .disabled_matcher(props.disabled_matcher)
            .year_range(props.year_range)
            .previous_accessibility_label(move || Some(labels::text(Label::PreviousMonth)))
            .next_accessibility_label(move || Some(labels::text(Label::NextMonth)))
            .on_change(move |picked: &Date| {
                let was = pending.get_untracked();
                let fallback = untracked(default_time);
                let at = |day: Option<NaiveDate>, was: Option<NaiveDateTime>| {
                    day.map(|d| d.and_time(was.map_or(fallback, |v| v.time())))
                };
                let next = match *picked {
                    Date::Single(day) => {
                        DateTime::Single(day.map(|d| d.and_time(time.get_untracked())))
                    }
                    Date::Range(start, end) => {
                        DateTime::Range(at(start, was.start()), at(end, was.end()))
                    }
                };
                fill(next, true);
            });
        let parts = grid.__element().__props_mut();
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
            .app_class(props.calendar_class)
            .inline_style(move || {
                Style::from(StyleRefinement::default().p(t.length(CALENDAR_PADDING)))
                    .then(&props.calendar_style.get())
            })
            .child(grid)
    };
    let time_row = move || {
        when(
            move || props.time_precision.get().is_some() && !props.range.get(),
            move || {
                let mut field = base_time_field()
                    .value(move || Some(TimeOfDay(time.get())))
                    .precision(move || props.time_precision.get().unwrap_or_default())
                    .hour_cycle(move || hour_cycle.get())
                    .disabled(disabled)
                    .accessibility_label(move || {
                        Some(
                            props
                                .time_accessibility_label
                                .get()
                                .unwrap_or_else(|| labels::text(Label::Time)),
                        )
                    })
                    .segment_class(move || Some(segment_class(t, props.segment_class.get())))
                    .on_change(move |v: &TimeOfDay| {
                        time.set(v.0);
                        if let DateTime::Single(Some(at)) = pending.get_untracked() {
                            pending.set(DateTime::Single(Some(at.date().and_time(v.0))));
                        }
                    });
                let parts = field.__element().__props_mut();
                parts.segment_style =
                    Prop::from(IntoReactive::<TextPartStyle>::into_reactive(move || {
                        segment_style(t, props.segment_style.get())
                    }));
                parts.separator_style =
                    Prop::from(IntoReactive::<TextPartStyle>::into_reactive(move || {
                        separator_style(t)
                    }));
                view()
                    .flex_row()
                    .items_center()
                    .justify_between()
                    .app_class(props.time_class)
                    .inline_style(move || {
                        Style::from(time_row_look(t)).then(&props.time_style.get())
                    })
                    .child(
                        text(move || labels::text(Label::Time))
                            .text_size(move || t.length(TEXT_SM))
                            .line_height(move || line_height(t.length(TEXT_SM)))
                            .font_weight(medium(t)),
                    )
                    .child(
                        view()
                            .flex_row()
                            .disabled(disabled)
                            .app_class(move || {
                                Some(time_field_class(t, props.time_field_class.get()))
                            })
                            .inline_style(move || {
                                time_field_style(t, size.get(), props.time_field_style.get())
                            })
                            .child(field),
                    )
            },
        )
    };
    // The typed fields (desktops): Start and End, or Date.
    let fields = move || {
        let field = move |at: usize, name: Label| {
            view()
                .flex_col()
                .flex_1()
                .min_w(px(0.0))
                .gap(move || t.length(SPACING) * 1.5)
                .child(
                    text(move || labels::text(name))
                        .text_size(move || t.length(TEXT_SM))
                        .line_height(move || line_height(t.length(TEXT_SM)))
                        .font_weight(medium(t))
                        .truncate(),
                )
                .child(
                    input()
                        .size(move || size.get())
                        .value(move || Some(SharedString::from(texts[at].get())))
                        .invalid(move || refused[at].get())
                        .disabled(disabled)
                        .placeholder(move || {
                            Some(SharedString::from(
                                example_day().format(&date_format.get()).to_string(),
                            ))
                        })
                        .accessibility_label(move || Some(labels::text(name)))
                        .auto_focus(at == 0)
                        .select_text_on_focus(true)
                        .on_change(move |text: &String| changed(at, text.clone()))
                        .on_submit(move |_: &String| {
                            left(at);
                            apply();
                        })
                        .on_blur(move || left(at)),
                )
        };
        let range = props.range.get_untracked();
        view()
            .flex_row()
            .gap(move || t.length(SPACING) * 2.0)
            .app_class(props.fields_class)
            .inline_style(move || {
                let padding = t.length(CALENDAR_PADDING);
                Style::from(StyleRefinement::default().p(padding).pb(px(0.0)))
                    .then(&props.fields_style.get())
            })
            .child(field(0, if range { Label::StartDate } else { Label::Date }))
            .when(range, |row| row.child(field(1, Label::EndDate)))
    };
    let looks = PresetLooks {
        range: props.range,
        class: props.preset_class,
        style: props.preset_style,
        description_class: props.preset_description_class,
        description_style: props.preset_description_style,
    };
    let presets = move || {
        dynamic(move || {
            let presets = props.presets.get();
            if presets.is_empty() {
                return None;
            }
            let buttons = presets
                .into_iter()
                .map(|p| confirm_preset(looks, t, format, pending, p, sheet, choose))
                .collect::<Vec<_>>();
            Some(
                view()
                    .app_class(props.presets_class)
                    .inline_style(move || {
                        Style::from(presets_look(t, sheet)).then(&props.presets_style.get())
                    })
                    .children(buttons)
                    .into_element(),
            )
        })
    };
    // Cancel and Apply; in the sheet the pending value above them.
    let footer = move || {
        let button_size = if sheet { Size::Md } else { Size::Sm };
        let cancel_button = button()
            .variant(ButtonVariant::Outline)
            .size(button_size)
            .on_press(cancel)
            .child(text(move || labels::text(Label::Cancel)));
        let apply_button = button()
            .size(button_size)
            .disabled(move || !can_apply())
            .on_press(apply)
            .child(text(move || labels::text(Label::Apply)));
        let padding = move || t.length(CALENDAR_PADDING);
        let gap = move || t.length(SPACING) * 2.0;
        let look = move || {
            Style::from(
                StyleRefinement::default()
                    .p(padding())
                    .border_t(t.length(BORDER_WIDTH))
                    .border_color(t.color(BORDER)),
            )
            .then(&props.footer_style.get())
        };
        if !sheet {
            return view()
                .flex_row()
                .justify_end()
                .items_center()
                .gap(gap)
                .app_class(props.footer_class)
                .inline_style(look)
                .child(cancel_button)
                .child(apply_button)
                .into_element();
        }
        let summary = computed(move || shown_text(pending.get(), &format.get()));
        view()
            .flex_col()
            .gap(gap)
            .app_class(props.footer_class)
            .inline_style(look)
            .child(
                text(move || {
                    summary
                        .get()
                        .or_else(|| props.placeholder.get())
                        .unwrap_or_else(|| labels::text(Label::PickADate))
                })
                .text_size(move || t.length(TEXT_SM))
                .line_height(move || line_height(t.length(TEXT_SM)))
                .text_color(move || {
                    if summary.with(Option::is_some) {
                        t.color(INPUT_FOREGROUND)
                    } else {
                        t.color(INPUT_PLACEHOLDER_FOREGROUND)
                    }
                })
                .truncate(),
            )
            .child(
                view()
                    .flex_row()
                    .gap(gap)
                    .child(view().flex_1().child(cancel_button.w_full()))
                    .child(view().flex_1().child(apply_button.w_full())),
            )
            .into_element()
    };

    // The field's keys while it has the focus, as `nsh/base`'s `DatePicker`
    // binds them: Return opens and closes, Delete and Backspace clear the
    // closed picker.
    let focused = signal(false);
    effect(move || {
        if !focused.get() || disabled() {
            return;
        }
        on_key_with(
            "enter",
            KeyOptions {
                when: When::NotEditing,
                repeat: false,
                for_focus: true,
                ..Default::default()
            },
            move |_| request(!untracked(is_open), OpenChangeReason::TriggerPress),
        );
        for key in ["delete", "backspace"] {
            on_key_with(
                key,
                KeyOptions {
                    when: When::NotEditing,
                    repeat: false,
                    ..Default::default()
                },
                move |_| {
                    if untracked(is_open) {
                        return;
                    }
                    let none = shaped(DateTime::Single(None), props.range.get_untracked());
                    if props.value.get_untracked() == DatePickerValue::Uncontrolled {
                        own.set(none);
                    }
                    let mut picked = PickedDate::default();
                    picked.value = none;
                    picked.time_precision = props.time_precision.get_untracked();
                    cx.emit(DatePickerEvent::Change(picked));
                },
            );
        }
    });

    // The field (or the app's trigger) as `nsh/base`'s `DatePicker` makes
    // it: the combobox with the value, focusable unless disabled; on touch
    // screens it also takes the press and says it is expanded (`Popover`
    // does on desktops).
    let width = signal(None::<Points>);
    macro_rules! combobox {
        ($el:expr) => {
            $el.role(AccessibilityRole::Combobox)
                .optional_label(props.accessibility_label)
                .disabled(disabled)
                .focusable(move || !disabled())
                .accessibility_value(move || {
                    shown_text(current.get(), &format.get()).map(|text| {
                        nsh_core::AccessibilityValue {
                            text: Some(text.as_str().into()),
                            ..nsh_core::AccessibilityValue::default()
                        }
                    })
                })
                .on_layout(move |frame: &Bounds<Points>| width.set(Some(frame.size.width)))
                .on_focus(move || focused.set(true))
                .on_blur(move || focused.set(false))
                .when(sheet, |el| {
                    el.accessibility_state(move || AccessibilityState {
                        expanded: Some(is_open()),
                        has_popup: true,
                        disabled: disabled(),
                        invalid: props.invalid.get() || joined.invalid(),
                        ..AccessibilityState::default()
                    })
                    .on_press(move || request(!untracked(is_open), OpenChangeReason::TriggerPress))
                })
        };
    }
    let trigger = if props.trigger.with_untracked(Option::is_some) {
        props.trigger.place(move |trigger| {
            combobox!(
                trigger
                    .element_ref(&trigger_ref)
                    .accessibility_described_by_target(move || joined.described_by())
            )
        })
    } else {
        combobox!(field_trigger(
            props.clone(),
            t,
            current,
            format,
            trigger_ref,
            joined
        ))
        .into_element()
    };
    let on_close = move || {
        if !untracked(is_open) {
            held.set(false);
        }
        cx.emit(DatePickerEvent::Close)
    };
    let root_class = move || {
        Some(SharedString::from(if joined.in_row() {
            "w-fit min-w-0 shrink"
        } else {
            "w-fit"
        }))
    };

    if sheet {
        // The sheet: the presets' row (labels), one month, the time row;
        // the footer the sheet's own, at its bottom.
        let panel = move || {
            untracked(|| {
                confirm_panel().child(
                    view()
                        .flex_col()
                        .child(presets())
                        .child(calendar())
                        .child(time_row()),
                )
            })
        };
        let mut layer = base_sheet()
            .open(move || Some(is_open()))
            .accessibility_label(props.accessibility_label)
            .popup_class(props.content_class)
            .backdrop_style(move |s| s.bg(move || t.color(OVERLAY)))
            .footer(dynamic(move || {
                untracked(|| confirm_panel().child(footer()))
            }))
            .on_open_change(move |e: &BaseSheetEvent| {
                if let BaseSheetEvent::OpenChange(open, reason) = e {
                    request(*open, *reason);
                }
            })
            .on_open(move || cx.emit(DatePickerEvent::Open))
            .on_close(on_close)
            .on_cancel(move || cx.emit(DatePickerEvent::Cancel))
            .child(dynamic(panel));
        layer.__element().__props_mut().popup_style = bound(move || {
            Style::from(sheet_look(t).p(px(0.0)).pb(safe_area().bottom))
                .then(&props.content_style.get())
        });
        return view()
            .app_class(root_class)
            .child(trigger)
            .child(layer)
            .into_element();
    }

    // The popup: presets beside the fields, the calendar and the time
    // row; the footer under them all.
    let panel = move || {
        untracked(|| {
            confirm_panel().child(
                view()
                    .flex_col()
                    .child(
                        view().flex_row().child(presets()).child(
                            view()
                                .flex_col()
                                .child(fields())
                                .child(calendar())
                                .child(time_row()),
                        ),
                    )
                    .child(footer()),
            )
        })
    };
    let mut layer = base_popover()
        .trigger(trigger)
        .open(move || Some(is_open()))
        .placement(props.placement)
        .align(props.align)
        .offset(move || t.length(POPOVER_OFFSET))
        .margin(move || t.length(POPOVER_MARGIN))
        .popup_class(props.content_class)
        // As the default path's: no slide, zooming from the middle.
        .popup_starting_class(move || popup::ending_class(popup::animate(t)))
        .popup_ending_class(move || popup::ending_class(popup::animate(t)))
        .on_open_change(move |e: &BasePopoverEvent| {
            if let BasePopoverEvent::OpenChange(open, reason) = e {
                request(*open, *reason);
            }
        })
        .on_open(move || cx.emit(DatePickerEvent::Open))
        .on_close(on_close)
        .on_cancel(move || cx.emit(DatePickerEvent::Cancel))
        .child(dynamic(panel));
    layer.__element().__props_mut().popup_style = bound(move || {
        // At least the field's width, as the default path's popup.
        let look =
            popup::surface(StyleRefinement::default(), t).min_w(width.get().unwrap_or_default());
        Style::from(look).then(&props.content_style.get())
    });
    view().app_class(root_class).child(layer).into_element()
}

/// The props of [`ConfirmPanel`].
#[derive(Debug, Clone, PartialEq, Props)]
struct ConfirmPanelProps {
    /// The popup's (or the sheet's footer's) content.
    children: Prop<Children>,
}

/// The confirmed picker's popup content: a component of its own so that
/// the library `Button`s in it (Cancel, Apply) are not the picker's
/// trigger (`trigger::clear`): the picker's trigger context is above them.
#[component]
fn ConfirmPanel(props: ConfirmPanelProps, _cx: Scope<NoEvent>) -> impl IntoElement {
    trigger::clear();
    view().flex_col().child(props.children)
}

/// The props a confirmed picker's presets read.
#[derive(Clone, Copy)]
struct PresetLooks {
    range: Prop<bool>,
    class: Prop<Option<SharedString>>,
    style: Prop<TextPartStyle>,
    description_class: Prop<Option<SharedString>>,
    description_style: Prop<TextPartStyle>,
}

/// A preset of the confirmed picker: a button that fills the pending
/// value; on desktops its dates under its label (the field's format,
/// muted), lit while its value is the pending one.
fn confirm_preset(
    props: PresetLooks,
    t: Theme,
    format: Memo<String>,
    pending: Signal<DateTime>,
    preset: DateRangePreset,
    sheet: bool,
    choose: impl Fn(DateTime) + Copy + 'static,
) -> AnyElement {
    let value = preset.value;
    let range = props.range;
    // A range of one day shows as that day.
    let dates = move || {
        if sheet {
            return None;
        }
        let shown = match shaped(value, range.get()) {
            DateTime::Range(Some(start), Some(end)) if start.date() == end.date() => {
                DateTime::Single(Some(start))
            }
            shown => shown,
        };
        shown_text(shown, &format.get())
    };
    let lit = move || pending.get().date() == shaped(value, range.get()).date();
    view()
        .role(AccessibilityRole::Button)
        .label(preset.label.clone())
        .optional_hint(dates)
        // Not a stop of Tab, as `nsh/base`'s presets.
        .focusable(false)
        .accessibility_state(move || AccessibilityState {
            selected: lit(),
            ..AccessibilityState::default()
        })
        .app_class(move || Some(preset_class(t, props.class.get())))
        .inline_style(move || {
            let look = preset_look(t, sheet);
            let height = t.length(BUTTON_HEIGHT_SM);
            let look = if sheet {
                look.h(height)
            } else {
                // Two lines: the label, its dates.
                look.flex_col()
                    .items_start()
                    .justify_center()
                    .min_h(height)
                    .py(t.length(SPACING) * 1.5)
            };
            Style::from(look).then(&Style::from(props.style.get()))
        })
        .on_press(move || choose(value))
        .child(
            text(preset.label)
                .when(!sheet, |label| label.w_full())
                .truncate(),
        )
        .child(when(
            move || dates().is_some(),
            move || {
                let font_size = t.length(FONT_SIZE_XS);
                text(move || dates().unwrap_or_default())
                    .w_full()
                    .truncate()
                    .text_size(font_size)
                    .line_height(line_height(font_size))
                    .text_color(move || t.color(DATE_PICKER_PRESET_DESCRIPTION))
                    .app_class(props.description_class)
                    .inline_style(move || Style::from(props.description_style.get()))
            },
        ))
        .into_element()
}
