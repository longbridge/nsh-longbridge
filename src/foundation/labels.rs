//! The library's own words (what screen readers say for a spinner, a
//! loading button): English by default; the host translates them with
//! [`set_translator`].

use std::sync::{Arc, RwLock};

use nsh_core::prelude::SharedString;

/// A word the library says itself.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum Label {
    /// "Loading": a spinner, a loading control without a label.
    Loading,
}

impl Label {
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
