//! The library's own words (what screen readers say for a spinner, a
//! loading button): English by default; the app translates them with
//! [`set_translator`] (TSX: `setTranslator`). The translator is the
//! surface's (a [`global`]): apps sharing a process (plugins in one host)
//! keep their own, and the words shown follow a new one at once.

use std::{collections::HashMap, rc::Rc};

use nsh_core::{
    element::{Callback, global, module},
    prelude::*,
};

/// A word the library says itself (TS: its key, `"loading"`).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default, PropEnum)]
#[non_exhaustive]
pub enum Label {
    /// "Loading": a spinner, a loading control without a label.
    #[default]
    Loading,
    /// "Close": the button that closes a dialog or a toast.
    Close,
    /// "Back": the row that goes back up from a submenu in a sheet.
    Back,
    /// "Su": Sunday over a calendar's days.
    #[prop(rename = "weekday-0")]
    Weekday0,
    /// "Mo": Monday over a calendar's days.
    #[prop(rename = "weekday-1")]
    Weekday1,
    /// "Tu": Tuesday over a calendar's days.
    #[prop(rename = "weekday-2")]
    Weekday2,
    /// "We": Wednesday over a calendar's days.
    #[prop(rename = "weekday-3")]
    Weekday3,
    /// "Th": Thursday over a calendar's days.
    #[prop(rename = "weekday-4")]
    Weekday4,
    /// "Fr": Friday over a calendar's days.
    #[prop(rename = "weekday-5")]
    Weekday5,
    /// "Sa": Saturday over a calendar's days.
    #[prop(rename = "weekday-6")]
    Weekday6,
    /// "January": a calendar's month toggle and titles.
    #[prop(rename = "month-1")]
    Month1,
    /// "February": a calendar's month toggle and titles.
    #[prop(rename = "month-2")]
    Month2,
    /// "March": a calendar's month toggle and titles.
    #[prop(rename = "month-3")]
    Month3,
    /// "April": a calendar's month toggle and titles.
    #[prop(rename = "month-4")]
    Month4,
    /// "May": a calendar's month toggle and titles.
    #[prop(rename = "month-5")]
    Month5,
    /// "June": a calendar's month toggle and titles.
    #[prop(rename = "month-6")]
    Month6,
    /// "July": a calendar's month toggle and titles.
    #[prop(rename = "month-7")]
    Month7,
    /// "August": a calendar's month toggle and titles.
    #[prop(rename = "month-8")]
    Month8,
    /// "September": a calendar's month toggle and titles.
    #[prop(rename = "month-9")]
    Month9,
    /// "October": a calendar's month toggle and titles.
    #[prop(rename = "month-10")]
    Month10,
    /// "November": a calendar's month toggle and titles.
    #[prop(rename = "month-11")]
    Month11,
    /// "December": a calendar's month toggle and titles.
    #[prop(rename = "month-12")]
    Month12,
    /// "Jan": January in a calendar's month view.
    #[prop(rename = "month-short-1")]
    MonthShort1,
    /// "Feb": February in a calendar's month view.
    #[prop(rename = "month-short-2")]
    MonthShort2,
    /// "Mar": March in a calendar's month view.
    #[prop(rename = "month-short-3")]
    MonthShort3,
    /// "Apr": April in a calendar's month view.
    #[prop(rename = "month-short-4")]
    MonthShort4,
    /// "May": May in a calendar's month view.
    #[prop(rename = "month-short-5")]
    MonthShort5,
    /// "Jun": June in a calendar's month view.
    #[prop(rename = "month-short-6")]
    MonthShort6,
    /// "Jul": July in a calendar's month view.
    #[prop(rename = "month-short-7")]
    MonthShort7,
    /// "Aug": August in a calendar's month view.
    #[prop(rename = "month-short-8")]
    MonthShort8,
    /// "Sep": September in a calendar's month view.
    #[prop(rename = "month-short-9")]
    MonthShort9,
    /// "Oct": October in a calendar's month view.
    #[prop(rename = "month-short-10")]
    MonthShort10,
    /// "Nov": November in a calendar's month view.
    #[prop(rename = "month-short-11")]
    MonthShort11,
    /// "Dec": December in a calendar's month view.
    #[prop(rename = "month-short-12")]
    MonthShort12,
    /// "Go to the previous month": a calendar's Previous.
    #[prop(rename = "previous-month")]
    PreviousMonth,
    /// "Go to the next month": a calendar's Next.
    #[prop(rename = "next-month")]
    NextMonth,
    /// "Time": the label of a date picker's time field.
    #[prop(rename = "time")]
    Time,
    /// "Pick a date": a date picker's placeholder.
    #[prop(rename = "pick-a-date")]
    PickADate,
    /// "Previous": the word of a pagination's step back.
    #[prop(rename = "previous")]
    Previous,
    /// "Next": the word of a pagination's step forward.
    #[prop(rename = "next")]
    Next,
    /// "Go to previous page": what screen readers call a pagination's
    /// step back.
    #[prop(rename = "go-to-previous-page")]
    GoToPreviousPage,
    /// "Go to next page": what screen readers call a pagination's step
    /// forward.
    #[prop(rename = "go-to-next-page")]
    GoToNextPage,
    /// "Pagination": what screen readers call a pagination.
    #[prop(rename = "pagination")]
    Pagination,
    /// "Select row": a data table's box of a row.
    #[prop(rename = "select-row")]
    SelectRow,
    /// "Select all": a data table's box over every row.
    #[prop(rename = "select-all")]
    SelectAll,
    /// "No items found.": a list a search left empty (a combobox's).
    #[prop(rename = "no-items")]
    NoItems,
    /// "Clear": the button that clears a value (a combobox's).
    Clear,
    /// "Remove": the button that removes a chosen value, said before its
    /// title (a combobox's chips).
    Remove,
    /// "{start} – {end}": two values as a range (a slider's bubble once its
    /// thumbs' two come together); `{start}` and `{end}` are replaced by
    /// the values' text.
    #[prop(rename = "value-range")]
    ValueRange,
    /// "Start date": a date range picker's typed start.
    #[prop(rename = "start-date")]
    StartDate,
    /// "End date": a date range picker's typed end.
    #[prop(rename = "end-date")]
    EndDate,
    /// "Date": a date picker's typed date.
    Date,
    /// "Cancel": the button that drops a pending choice (a date picker's
    /// `confirm`).
    Cancel,
    /// "Apply": the button that commits a pending choice (a date picker's
    /// `confirm`).
    Apply,
    /// "Search": a list's search field, its placeholder and name (a
    /// searchable select's).
    Search,
    /// "Create “{query}”": the row that makes what was typed a new value
    /// (a creatable combobox's); `{query}` stands for it (left out, the
    /// query follows the words).
    Create,
    /// "Show password": an input group's password toggle (its pressed state
    /// says whether it shows).
    #[prop(rename = "show-password")]
    ShowPassword,
    /// "{count} selected": a selection bar's count, `{count}` replaced by
    /// the number of selected items.
    #[prop(rename = "count-selected")]
    CountSelected,
    /// "Clear selection": a selection bar's "×".
    #[prop(rename = "clear-selection")]
    ClearSelection,
}

impl Label {
    /// Every word, in a stable order.
    pub const ALL: &'static [Label] = &[
        Label::Loading,
        Label::Close,
        Label::Back,
        Label::Weekday0,
        Label::Weekday1,
        Label::Weekday2,
        Label::Weekday3,
        Label::Weekday4,
        Label::Weekday5,
        Label::Weekday6,
        Label::Month1,
        Label::Month2,
        Label::Month3,
        Label::Month4,
        Label::Month5,
        Label::Month6,
        Label::Month7,
        Label::Month8,
        Label::Month9,
        Label::Month10,
        Label::Month11,
        Label::Month12,
        Label::MonthShort1,
        Label::MonthShort2,
        Label::MonthShort3,
        Label::MonthShort4,
        Label::MonthShort5,
        Label::MonthShort6,
        Label::MonthShort7,
        Label::MonthShort8,
        Label::MonthShort9,
        Label::MonthShort10,
        Label::MonthShort11,
        Label::MonthShort12,
        Label::PreviousMonth,
        Label::NextMonth,
        Label::Time,
        Label::PickADate,
        Label::Previous,
        Label::Next,
        Label::GoToPreviousPage,
        Label::GoToNextPage,
        Label::Pagination,
        Label::SelectRow,
        Label::SelectAll,
        Label::NoItems,
        Label::Clear,
        Label::Remove,
        Label::ValueRange,
        Label::StartDate,
        Label::EndDate,
        Label::Date,
        Label::Cancel,
        Label::Apply,
        Label::Search,
        Label::Create,
        Label::ShowPassword,
        Label::CountSelected,
        Label::ClearSelection,
    ];

    /// Its key, which a translator is given (`"loading"`).
    pub fn key(self) -> &'static str {
        match self {
            Label::Loading => "loading",
            Label::Close => "close",
            Label::Back => "back",
            Label::Weekday0 => "weekday-0",
            Label::Weekday1 => "weekday-1",
            Label::Weekday2 => "weekday-2",
            Label::Weekday3 => "weekday-3",
            Label::Weekday4 => "weekday-4",
            Label::Weekday5 => "weekday-5",
            Label::Weekday6 => "weekday-6",
            Label::Month1 => "month-1",
            Label::Month2 => "month-2",
            Label::Month3 => "month-3",
            Label::Month4 => "month-4",
            Label::Month5 => "month-5",
            Label::Month6 => "month-6",
            Label::Month7 => "month-7",
            Label::Month8 => "month-8",
            Label::Month9 => "month-9",
            Label::Month10 => "month-10",
            Label::Month11 => "month-11",
            Label::Month12 => "month-12",
            Label::MonthShort1 => "month-short-1",
            Label::MonthShort2 => "month-short-2",
            Label::MonthShort3 => "month-short-3",
            Label::MonthShort4 => "month-short-4",
            Label::MonthShort5 => "month-short-5",
            Label::MonthShort6 => "month-short-6",
            Label::MonthShort7 => "month-short-7",
            Label::MonthShort8 => "month-short-8",
            Label::MonthShort9 => "month-short-9",
            Label::MonthShort10 => "month-short-10",
            Label::MonthShort11 => "month-short-11",
            Label::MonthShort12 => "month-short-12",
            Label::PreviousMonth => "previous-month",
            Label::NextMonth => "next-month",
            Label::Time => "time",
            Label::PickADate => "pick-a-date",
            Label::Previous => "previous",
            Label::Next => "next",
            Label::GoToPreviousPage => "go-to-previous-page",
            Label::GoToNextPage => "go-to-next-page",
            Label::Pagination => "pagination",
            Label::SelectRow => "select-row",
            Label::SelectAll => "select-all",
            Label::NoItems => "no-items",
            Label::Clear => "clear",
            Label::Remove => "remove",
            Label::ValueRange => "value-range",
            Label::StartDate => "start-date",
            Label::EndDate => "end-date",
            Label::Date => "date",
            Label::Cancel => "cancel",
            Label::Apply => "apply",
            Label::Search => "search",
            Label::Create => "create",
            Label::ShowPassword => "show-password",
            Label::CountSelected => "count-selected",
            Label::ClearSelection => "clear-selection",
        }
    }

    /// Its English text.
    pub fn english(self) -> &'static str {
        match self {
            Label::Loading => "Loading",
            Label::Close => "Close",
            Label::Back => "Back",
            Label::Weekday0 => "Su",
            Label::Weekday1 => "Mo",
            Label::Weekday2 => "Tu",
            Label::Weekday3 => "We",
            Label::Weekday4 => "Th",
            Label::Weekday5 => "Fr",
            Label::Weekday6 => "Sa",
            Label::Month1 => "January",
            Label::Month2 => "February",
            Label::Month3 => "March",
            Label::Month4 => "April",
            Label::Month5 => "May",
            Label::Month6 => "June",
            Label::Month7 => "July",
            Label::Month8 => "August",
            Label::Month9 => "September",
            Label::Month10 => "October",
            Label::Month11 => "November",
            Label::Month12 => "December",
            Label::MonthShort1 => "Jan",
            Label::MonthShort2 => "Feb",
            Label::MonthShort3 => "Mar",
            Label::MonthShort4 => "Apr",
            Label::MonthShort5 => "May",
            Label::MonthShort6 => "Jun",
            Label::MonthShort7 => "Jul",
            Label::MonthShort8 => "Aug",
            Label::MonthShort9 => "Sep",
            Label::MonthShort10 => "Oct",
            Label::MonthShort11 => "Nov",
            Label::MonthShort12 => "Dec",
            Label::PreviousMonth => "Go to the previous month",
            Label::NextMonth => "Go to the next month",
            Label::Time => "Time",
            Label::PickADate => "Pick a date",
            Label::Previous => "Previous",
            Label::Next => "Next",
            Label::GoToPreviousPage => "Go to previous page",
            Label::GoToNextPage => "Go to next page",
            Label::Pagination => "Pagination",
            Label::SelectRow => "Select row",
            Label::SelectAll => "Select all",
            Label::NoItems => "No items found.",
            Label::Clear => "Clear",
            Label::Remove => "Remove",
            Label::ValueRange => "{start} – {end}",
            Label::StartDate => "Start date",
            Label::EndDate => "End date",
            Label::Date => "Date",
            Label::Cancel => "Cancel",
            Label::Apply => "Apply",
            Label::Search => "Search",
            Label::Create => "Create “{query}”",
            Label::ShowPassword => "Show password",
            Label::CountSelected => "{count} selected",
            Label::ClearSelection => "Clear selection",
        }
    }

    /// The short name of weekday `day` (`0` Sunday to `6`; taken modulo
    /// 7): `Su` … `Sa`.
    pub fn weekday(day: u32) -> Label {
        [
            Label::Weekday0,
            Label::Weekday1,
            Label::Weekday2,
            Label::Weekday3,
            Label::Weekday4,
            Label::Weekday5,
            Label::Weekday6,
        ][(day % 7) as usize]
    }

    /// The name of month `month` (`1` January to `12`; clamped): `January`
    /// … `December`.
    pub fn month(month: u32) -> Label {
        MONTHS[month.clamp(1, 12) as usize - 1]
    }

    /// The short name of month `month` (`1` to `12`; clamped): `Jan` …
    /// `Dec`.
    pub fn month_short(month: u32) -> Label {
        MONTHS_SHORT[month.clamp(1, 12) as usize - 1]
    }
}

const MONTHS: [Label; 12] = [
    Label::Month1,
    Label::Month2,
    Label::Month3,
    Label::Month4,
    Label::Month5,
    Label::Month6,
    Label::Month7,
    Label::Month8,
    Label::Month9,
    Label::Month10,
    Label::Month11,
    Label::Month12,
];

const MONTHS_SHORT: [Label; 12] = [
    Label::MonthShort1,
    Label::MonthShort2,
    Label::MonthShort3,
    Label::MonthShort4,
    Label::MonthShort5,
    Label::MonthShort6,
    Label::MonthShort7,
    Label::MonthShort8,
    Label::MonthShort9,
    Label::MonthShort10,
    Label::MonthShort11,
    Label::MonthShort12,
];

type Translator = Rc<dyn Fn(Label) -> Option<SharedString>>;

/// The surface's translator (`None`: English).
#[derive(Clone, Default)]
struct Translations(Option<Translator>);

/// Translates the library's words on this surface from now on:
/// `translate(label)` gives the text, `None` for English. The words shown
/// change at once, and so they do when what `translate` reads changes
/// (`locale()`). Call it inside the app's runtime (a component, a
/// listener, a task), as the surface's globals require: outside it, it
/// panics.
pub fn set_translator(translate: impl Fn(Label) -> Option<SharedString> + 'static) {
    global::<Translations>().set(Translations(Some(Rc::new(translate))));
}

/// Back to English on this surface (inside the app's runtime, as
/// [`set_translator`]).
pub fn clear_translator() {
    global::<Translations>().set(Translations(None));
}

/// The text of `label` on this surface. Read in a binding, it follows the
/// translator; outside a runtime, it is English.
pub fn text(label: Label) -> SharedString {
    try_global::<Translations>()
        .and_then(|translations| translations.get().0)
        .and_then(|translate| translate(label))
        .unwrap_or_else(|| label.english().into())
}

/// The library's words for TSX: `setTranslator`.
#[module]
pub(crate) struct Labels;

#[module(flat)]
impl Labels {
    /// Translates this library's own words (what screen readers say for a
    /// spinner or a loading button without a label), English by default.
    /// `translate(key)` is called for every key while `setTranslator`
    /// runs, and its answers are kept: a string is the text; `null`, or a
    /// call that throws (logged), keeps the English one. The words shown
    /// change at once; call it again when the language changes. The
    /// translation is this app's own: other apps in the same process keep
    /// theirs. `null` goes back to English.
    pub(crate) fn set_translator(#[args(key)] translate: Option<Callback<Label, Option<String>>>) {
        match translate {
            // A JS function runs only inside a call from JS, never in the
            // bindings that read the words: ask it for every word now.
            Some(translate) => set_table(|label| translate.call(label)),
            None => clear_translator(),
        }
    }
}

/// Translates the words with what `ask` answers for each, asked once
/// each now: `Ok(None)` and errors (logged) keep the English text.
fn set_table(mut ask: impl FnMut(Label) -> Result<Option<String>, String>) {
    let table: HashMap<Label, SharedString> = Label::ALL
        .iter()
        .filter_map(|&label| match ask(label) {
            Ok(text) => text.map(|text| (label, SharedString::from(text))),
            Err(error) => {
                log::warn!(
                    "nsh-longbridge: the translator failed for `{}`: {error}",
                    label.key()
                );
                None
            }
        })
        .collect();
    set_translator(move |label| table.get(&label).cloned());
}

#[cfg(test)]
mod tests {
    use nsh_core::element::Runtime;

    use super::*;

    #[test]
    fn a_table_answers_once_per_word_and_falls_back_to_english() {
        let runtime = Runtime::default();
        let _entered = runtime.enter();
        let mut asked = Vec::new();
        set_table(|label| {
            asked.push(label);
            Ok(Some(format!("[{}]", label.key())))
        });
        assert_eq!(asked, Label::ALL);
        assert_eq!(text(Label::Loading), "[loading]");

        set_table(|_| Ok(None));
        assert_eq!(text(Label::Loading), "Loading");

        set_table(|_| Err("thrown".into()));
        assert_eq!(text(Label::Loading), "Loading");

        set_table(|_| Ok(Some("Chargement".into())));
        clear_translator();
        assert_eq!(text(Label::Loading), "Loading");
    }

    #[test]
    fn outside_a_runtime_the_words_are_english() {
        assert_eq!(text(Label::Loading), "Loading");
    }
}
