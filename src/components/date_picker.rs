//! `DatePicker`: the library's date picker on `nsh/base`'s `DatePicker`
//! (the value, the popup at the trigger or the sheet on touch screens, the
//! calendar's behavior, the presets, the time field, the keys, the
//! combobox role and its value for screen readers). The library draws the
//! trigger (the field look), the popup's surface (`foundation::popup`), the
//! calendar (`Calendar`'s look, shared), the presets, the time row and its
//! segments. API: `docs/components/date-picker.md`.

use nsh_base::{
    DatePickerEvent as BaseDatePickerEvent, OpenChangeReason, date_picker as base_date_picker,
    terminal::is_terminal,
};
use nsh_core::{
    element::{Renderer, Style, StyleRefinement, TextPartStyle, locale_settings, safe_area},
    prelude::*,
};

pub use nsh_base::{
    DatePickerValue, DateRangePreset, DateTime, HourCycle, PickedDate, TimeOfDay, TimePrecision,
};

use crate::{
    components::{
        button::{
            BUTTON_GHOST_FOREGROUND, BUTTON_GHOST_HOVER, BUTTON_GHOST_HOVER_FOREGROUND,
            BUTTON_HEIGHT_SM, BUTTON_OUTLINE, BUTTON_OUTLINE_BORDER, BUTTON_OUTLINE_FOREGROUND,
            BUTTON_OUTLINE_HOVER, BUTTON_OUTLINE_HOVER_FOREGROUND, BUTTON_PADDING_X_SM,
            BUTTON_RADIUS,
        },
        calendar::{
            self, CALENDAR_PADDING, CalendarEntry, LookConfig, Matcher, Parts, YearRange,
            color_class,
        },
        dropdown_menu::sheet_look,
        input::{self, vertical_hit_slop},
        select::SELECT_TRIGGER_HOVER,
    },
    foundation::{
        field::{
            self, INPUT_BACKGROUND, INPUT_BORDER, INPUT_BORDER_FOCUS, INPUT_BORDER_INVALID,
            INPUT_BORDER_WIDTH, INPUT_FOREGROUND, INPUT_PLACEHOLDER_FOREGROUND, INPUT_RADIUS,
            INPUT_RING, INPUT_RING_WIDTH, INPUT_SHADOW, Joined, Kind, field_look,
        },
        field_ref::FieldRef,
        icon::{IconName, icon_size_token, icon_view, provide_icon_size, provide_text_color},
        labels::{self, Label},
        menu::{
            MENU_ITEM_FOCUS, MENU_ITEM_FOCUS_FOREGROUND, MENU_ITEM_PADDING_X, MENU_MIN_WIDTH,
            list_as_sheet,
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
        semantic::{BORDER, BORDER_WIDTH, OPACITY_DISABLED, OVERLAY, RADIUS_XS, TEXT_SM},
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
}

/// The field's format without `dateFormat`: `Oct 28, 2026`.
const DATE_FORMAT: &str = "%b %-d, %Y";

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
    /// closes the popup.
    #[prop(default)]
    pub range: Prop<bool>,
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
    /// days").
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
        // (`w-fit`), while a row's `items-center` still centers it. The
        // app's class goes over it.
        .app_class(Some(SharedString::from("w-fit")));
    if terminal {
        // `nsh/base`'s terminal calendar and marks, the value as text.
        let shown = move || {
            current
                .get()
                .format(&format.get())
                .or_else(|| props.placeholder.get())
                .unwrap_or_else(|| labels::text(Label::PickADate))
        };
        return match props.trigger.with_untracked(Option::is_some) {
            true => root.trigger(props.trigger.place(|trigger| trigger)),
            false => root.trigger(text(shown)),
        };
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
                .font_weight(medium(t)),
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
        let padding = t.length(CALENDAR_PADDING);
        let look = if list_as_sheet() {
            StyleRefinement::default()
                .flex_row()
                .flex_wrap()
                .gap(t.length(SPACING) * 2.0)
                .p(padding)
                .pb(px(0.0))
        } else {
            StyleRefinement::default()
                .flex_col()
                .gap(t.length(SPACING))
                .p(padding)
                .min_w(t.length(MENU_MIN_WIDTH))
                .border_r(t.length(BORDER_WIDTH))
                .border_color(t.color(BORDER))
        };
        Style::from(look).then(&props.presets_style.get())
    });
    parts.preset_style = Prop::from(IntoReactive::<TextPartStyle>::into_reactive(move || {
        let font_size = t.length(font_size_token(Size::Sm));
        let look = StyleRefinement::default()
            .flex_row()
            .items_center()
            .flex_shrink_0()
            .h(t.length(BUTTON_HEIGHT_SM))
            .rounded(t.length(BUTTON_RADIUS))
            .text_size(font_size)
            .line_height(line_height(font_size))
            .font_weight(FontWeight::NORMAL)
            .whitespace_nowrap()
            .cursor(Cursor::Default);
        let look = if list_as_sheet() {
            // Outline buttons.
            look.px(t.length(BUTTON_PADDING_X_SM))
                .border(t.length(BORDER_WIDTH))
                .border_color(t.color(BUTTON_OUTLINE_BORDER))
        } else {
            look.px(t.length(MENU_ITEM_PADDING_X))
        };
        TextPartStyle::from(Style::from(look).then(&Style::from(props.preset_style.get())))
    }));
    parts.time_style = bound(move || {
        let look = StyleRefinement::default()
            .gap(t.length(SPACING) * 2.0)
            .p(t.length(CALENDAR_PADDING))
            .border_t(t.length(BORDER_WIDTH))
            .border_color(t.color(BORDER));
        Style::from(look).then(&props.time_style.get())
    });
    parts.time_field_style = bound(move || {
        let size = size.get();
        let font_size = t.length(input::font_size_token(size));
        // The field look, but its edge's color, which is a class (it
        // changes with `focus-within:`, and an inline color would win).
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
            .then(&props.time_field_style.get())
    });
    parts.segment_style = Prop::from(IntoReactive::<TextPartStyle>::into_reactive(move || {
        let look = StyleRefinement::default()
            .px(px(2.0))
            .rounded(t.length(RADIUS_XS))
            .tabular_nums();
        TextPartStyle::from(Style::from(look).then(&Style::from(props.segment_style.get())))
    }));
    parts.separator_style = Prop::from(IntoReactive::<TextPartStyle>::into_reactive(move || {
        TextPartStyle::from(Style::from(
            StyleRefinement::default().text_color(t.color(INPUT_PLACEHOLDER_FOREGROUND)),
        ))
    }));
    root
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
) -> impl IntoElement {
    let size = move || props.size.get();
    let height = move || t.length(input::height_token(size()));
    let font_size = move || t.length(input::font_size_token(size()));
    let invalid = move || props.invalid.get() || joined.invalid();
    let shown = computed(move || format.with(|f| current.get().format(f)));
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

/// Ends `class` with the app's classes.
fn with_app(mut class: String, app: Option<SharedString>) -> SharedString {
    if let Some(app) = app {
        class.push_str(&app);
    }
    class.truncate(class.trim_end().len());
    class.into()
}

/// The presets' classes: the hover (and the press, for a finger), ghost
/// beside the calendar and outline in the sheet, from the tokens, which an
/// inline style cannot take; then the app's.
fn preset_class(t: Theme, app: Option<SharedString>) -> SharedString {
    let mut class = String::new();
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
            t.color(BUTTON_GHOST_FOREGROUND),
            t.color(BUTTON_GHOST_HOVER),
            t.color(BUTTON_GHOST_HOVER_FOREGROUND),
        )
    };
    if let Some(rest) = rest {
        color_class(&mut class, "", "bg", rest);
    }
    color_class(&mut class, "", "text", rest_text);
    for state in ["hover", "active"] {
        color_class(&mut class, &format!("{state}:"), "bg", fill);
        color_class(&mut class, &format!("{state}:"), "text", text);
    }
    with_app(class, app)
}

/// The time field's classes: the field look's edge color, the focus edge
/// and (with a pointer) ring while the native field inside has the focus
/// (`focus-within:`), the disabled opacity; then the app's.
fn time_field_class(t: Theme, app: Option<SharedString>) -> SharedString {
    let mut class = String::new();
    color_class(&mut class, "", "border", t.color(INPUT_BORDER));
    color_class(
        &mut class,
        "focus-within:",
        "border",
        t.color(INPUT_BORDER_FOCUS),
    );
    if pointer() {
        class.push_str(&format!(
            "focus-within:ring-[{}px] ",
            f32::from(t.length(INPUT_RING_WIDTH))
        ));
        color_class(&mut class, "focus-within:", "ring", t.color(INPUT_RING));
    }
    class.push_str(&format!(
        "disabled:opacity-[{}] ",
        t.number(OPACITY_DISABLED)
    ));
    with_app(class, app)
}

/// The segments' classes: the edited one's highlight (`focus:`, which
/// `nsh/base` projects on it); then the app's.
fn segment_class(t: Theme, app: Option<SharedString>) -> SharedString {
    let mut class = String::new();
    color_class(
        &mut class,
        "focus:",
        "bg",
        t.color(DATE_PICKER_SEGMENT_FOCUS),
    );
    color_class(
        &mut class,
        "focus:",
        "text",
        t.color(DATE_PICKER_SEGMENT_FOCUS_FOREGROUND),
    );
    with_app(class, app)
}
