//! The library's words: `setTranslator` as TSX apps' declarations show
//! it, and the words a surface shows following its translator.

use nsh_core::{Size as Viewport, element::declarations, platform::Os, prelude::*};
use nsh_engine::EngineConfig;
use nsh_longbridge::{
    IconName, button,
    labels::{self, Label},
};
use nsh_testing::Headless;

#[test]
fn tsx_gets_set_translator_over_the_label_keys() {
    let registry = nsh_longbridge::registry();
    assert_eq!(registry.undocumented(), Vec::<String>::new());
    let ts = declarations(&registry, &[]);
    assert!(
        ts.contains("export declare function setTranslator(translate?: ((key: Label) => string | null) | null): void;"),
        "{ts}"
    );
    // The keys TS passes are the ones `Label::key` names.
    let keys: Vec<String> = Label::ALL
        .iter()
        .map(|l| format!("\"{}\"", l.key()))
        .collect();
    assert!(
        ts.contains(&format!("export type Label = {};", keys.join(" | "))),
        "{ts}"
    );
}

fn start<C: Component<Props = ()>>() -> Headless {
    let mut config = EngineConfig {
        viewport: Viewport::new(px(800.0), px(600.0)),
        ..EngineConfig::new(nsh_testing::windowed_root::<C>())
    };
    config.environment.os = Os::MacOs;
    let mut app = Headless::start(config).unwrap();
    app.first_render().unwrap();
    app
}

/// An icon-only button loading without a label (named by `labels`), and
/// buttons that translate the words and go back to English.
#[component]
fn Shown() -> impl IntoElement {
    view()
        .child(button().icon_only(true).icon(IconName::Plus).loading(true))
        .child(
            button()
                .on_press(|| {
                    labels::set_translator(|label| {
                        (label == Label::Loading).then(|| "Chargement".into())
                    })
                })
                .child("French"),
        )
        .child(button().on_press(labels::clear_translator).child("English"))
}

#[test]
fn a_shown_word_follows_the_translator() {
    let mut app = start::<Shown>();
    assert!(app.screen().matches("role:button[Loading]").unwrap());
    app.press("text:French").unwrap();
    app.wait_for("role:button[Chargement]").unwrap();
    app.press("text:English").unwrap();
    app.wait_for("role:button[Loading]").unwrap();
}

#[test]
fn each_surface_keeps_its_own_translator() {
    let mut french = start::<Shown>();
    let english = start::<Shown>();
    french.press("text:French").unwrap();
    french.wait_for("role:button[Chargement]").unwrap();
    // The other app in the process still says it in English.
    assert!(english.screen().matches("role:button[Loading]").unwrap());
    let english = start::<Shown>();
    assert!(english.screen().matches("role:button[Loading]").unwrap());
}
