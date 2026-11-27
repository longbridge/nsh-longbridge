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
}

impl Label {
    /// Every word, in a stable order.
    pub const ALL: &'static [Label] = &[Label::Loading, Label::Close, Label::Back];

    /// Its key, which a translator is given (`"loading"`).
    pub fn key(self) -> &'static str {
        match self {
            Label::Loading => "loading",
            Label::Close => "close",
            Label::Back => "back",
        }
    }

    /// Its English text.
    pub fn english(self) -> &'static str {
        match self {
            Label::Loading => "Loading",
            Label::Close => "Close",
            Label::Back => "Back",
        }
    }
}

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
