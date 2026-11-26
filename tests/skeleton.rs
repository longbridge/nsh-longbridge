//! `Skeleton` in a headless engine: no size of its own, the app's size and
//! corners winning over its own, its fill, and a scope's tokens reaching it.

mod common;

use common::{css, fill, light, size, start};
use nsh_core::prelude::*;
use nsh_longbridge::{ScopeTokens, components::skeleton::SKELETON, skeleton, theme_scope};

/// A bare skeleton, a sized one, a round one, one in a scope.
#[component]
fn Blocks() -> impl IntoElement {
    view()
        .items_start()
        .gap(px(8.0))
        .child(skeleton().test_id("bare"))
        .child(skeleton().test_id("line").h(px(16.0)).w(px(250.0)))
        .child(skeleton().test_id("avatar").size(px(48.0)).rounded_full())
        .child(
            theme_scope()
                .tokens(ScopeTokens::new().set("skeleton", "#2563eb"))
                .child(skeleton().test_id("scoped").h(px(16.0)).w(px(100.0))),
        )
}

#[test]
fn it_takes_the_apps_size() {
    let app = start::<Blocks>(true);
    // No size of its own: nothing on screen.
    let bare = app.screen().layout("id:bare").unwrap();
    assert!(bare.is_none_or(|l| l.bounds.size.width.0 == 0.0 && l.bounds.size.height.0 == 0.0));
    assert_eq!(size(&app, "id:line"), (250.0, 16.0));
    assert_eq!(size(&app, "id:avatar"), (48.0, 48.0));
}

#[test]
fn its_fill_is_skeleton_and_a_scope_reaches_it() {
    let app = start::<Blocks>(true);
    assert_eq!(fill(&app, "id:line"), light(SKELETON));
    assert_eq!(fill(&app, "id:scoped"), css("#2563eb"));
}

#[test]
fn screen_readers_skip_it() {
    let app = start::<Blocks>(true);
    let node = app.screen().accessibility(Some("id:line")).unwrap();
    assert!(
        node.as_ref()
            .is_none_or(|n| n.role.is_none() && n.name.is_empty()),
        "{node:?}"
    );
}
