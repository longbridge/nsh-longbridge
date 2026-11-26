//! The library's own words (what screen readers say for a spinner, a
//! loading button): English by default; the host translates them with
//! [`set_translator`] (TSX: `setTranslator`).

use std::{
    collections::HashMap,
    sync::{Arc, RwLock},
};

use nsh_core::{
    element::{Callback, module},
    prelude::*,
};

/// A word the library says itself (TS: its key, `"loading"`).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default, PropEnum)]
#[non_exhaustive]
pub enum Label {
    /// "Loading": a spinner, a loading control without a label.
    #[default]
    Loading,
}

impl Label {
    /// Every word, in a stable order.
    pub const ALL: &'static [Label] = &[Label::Loading];

    /// Its key, which a translator is given (`"loading"`).
    pub fn key(self) -> &'static str {
        match self {
            Label::Loading => "loading",
        }
    }

    /// Its English text.
    pub fn english(self) -> &'static str {
        match self {
            Label::Loading => "Loading",
        }
    }
}

type Translator = Arc<dyn Fn(Label) -> Option<SharedString> + Send + Sync>;

static TRANSLATOR: RwLock<Option<Translator>> = RwLock::new(None);

/// Translates the library's words from now on: `translate(label)` gives
/// the text, `None` for English. Reading the locale inside it (`locale()`)
/// makes the components that show a label follow its changes.
pub fn set_translator(translate: impl Fn(Label) -> Option<SharedString> + Send + Sync + 'static) {
    *TRANSLATOR.write().unwrap_or_else(|e| e.into_inner()) = Some(Arc::new(translate));
}

/// Back to English.
pub fn clear_translator() {
    *TRANSLATOR.write().unwrap_or_else(|e| e.into_inner()) = None;
}

/// The text of `label` (read it in a binding).
pub fn text(label: Label) -> SharedString {
    let translator = TRANSLATOR.read().unwrap_or_else(|e| e.into_inner()).clone();
    translator
        .and_then(|t| t(label))
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
    /// call that throws (logged), keeps the English one. Call it before
    /// the screens that show the words are built, and again when the
    /// language changes: a word already shown keeps its text until its
    /// component is built again. `null` goes back to English.
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
    use super::*;

    /// One test: the translator is the process's.
    #[test]
    fn a_table_answers_once_per_word_and_falls_back_to_english() {
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
}
