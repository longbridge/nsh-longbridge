//! `setTranslator`: the TSX face of `labels::set_translator`, as apps'
//! declarations show it.

use nsh_core::element::declarations;
use nsh_longbridge::labels::Label;

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
