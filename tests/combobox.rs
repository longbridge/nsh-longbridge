//! `Combobox` in a headless engine: typing filters the list, a choice
//! sets the value and closes it, several values show as chips whose
//! buttons remove them, the clear button, the empty list's words, the
//! list's height capped (the popup shrinking with it), the field look's
//! focus around the input, sizes, disabled, the matched part of each
//! title, the create row, the query's resets, and what TSX apps get.

mod common;

use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
    time::Duration,
};

use common::{light, size, start};
use nsh_core::{MountItem, NodeId, Size as Viewport, ViewStyle, platform::Os, prelude::*};
use nsh_engine::EngineConfig;
use nsh_longbridge::{
    SelectItem, combobox,
    components::combobox::{COMBOBOX_CHIP, COMBOBOX_EMPTY_FOREGROUND},
    foundation::field::INPUT_BORDER_FOCUS,
};
use nsh_testing::Headless;

fn frameworks() -> Vec<SelectItem> {
    vec![
        SelectItem::new("next", "Next.js"),
        SelectItem::new("sveltekit", "SvelteKit"),
        SelectItem::new("nuxt", "Nuxt.js"),
        SelectItem::new("remix", "Remix"),
        SelectItem::new("astro", "Astro"),
    ]
}

/// One value, cleanable; several values; a disabled one; one with a long
/// list; a small one.
#[component]
fn Boxes() -> impl IntoElement {
    let one = signal(Vec::<SharedString>::new());
    let many = signal(vec![
        SharedString::from("next"),
        SharedString::from("astro"),
    ]);
    let long: Vec<SelectItem> = (0..100)
        .map(|i| SelectItem::new(format!("v{i}"), format!("Item {i}")))
        .collect();
    view()
        .flex_col()
        .gap(px(12.0))
        .p(px(20.0))
        .w(px(320.0))
        .child(
            combobox()
                .items(frameworks())
                .value(move || Some(one.get()))
                .on_change(move |v: &Vec<SharedString>| one.set(v.clone()))
                .placeholder("Select a framework")
                .cleanable(true)
                .accessibility_label("Framework"),
        )
        .child(
            combobox()
                .multiple(true)
                .items(frameworks())
                .value(move || Some(many.get()))
                .on_change(move |v: &Vec<SharedString>| many.set(v.clone()))
                .placeholder("Add")
                .accessibility_label("Frameworks"),
        )
        .child(
            combobox()
                .items(frameworks())
                .disabled(true)
                .accessibility_label("Locked"),
        )
        .child(combobox().items(long).accessibility_label("Long"))
        .child(
            combobox()
                .items(frameworks())
                .size(nsh_longbridge::Size::Sm)
                .accessibility_label("Small"),
        )
        .child(text(move || {
            format!("one:{} many:{}", one.get().join("+"), many.get().join("+"))
        }))
}

fn open(app: &mut Headless, name: &str) {
    app.press(&format!("role:combobox[{name}]")).unwrap();
    app.run_for(Duration::from_millis(300)).unwrap();
}

/// The field (the combobox's root) around the input `name` names.
fn field_id(app: &Headless, name: &str) -> NodeId {
    let nodes = app.screen().layout_nodes();
    let input = app.screen().id(&format!("role:combobox[{name}]")).unwrap();
    nodes.iter().find(|n| n.id == input).unwrap().parent
}

/// [`field_id`] as a selector.
fn field_of(app: &Headless, name: &str) -> String {
    format!("view:{}", field_id(app, name))
}

#[test]
fn typing_filters_the_list_and_a_row_chooses_and_closes_it() {
    let mut app = start::<Boxes>(true);
    app.type_text("role:combobox[Framework]", "nu").unwrap();
    app.run_for(Duration::from_millis(300)).unwrap();
    assert_eq!(app.screen().count("role:option").unwrap(), 1);
    app.press("role:option[Nuxt.js]").unwrap();
    app.wait_for("text:one:nuxt many:next+astro").unwrap();
    app.run_for(Duration::from_millis(300)).unwrap();
    assert!(!app.screen().matches("role:listbox").unwrap());
}

#[test]
fn the_clear_button_clears_the_value() {
    let mut app = start::<Boxes>(true);
    open(&mut app, "Framework");
    app.press("role:option[Remix]").unwrap();
    app.wait_for("text:one:remix many:next+astro").unwrap();
    app.run_for(Duration::from_millis(300)).unwrap();
    app.press("role:button[Clear]").unwrap();
    app.wait_for("text:one: many:next+astro").unwrap();
}

#[test]
fn several_values_show_as_chips_whose_buttons_remove_them() {
    let mut app = start::<Boxes>(true);
    assert!(app.shows("Next.js"));
    assert!(app.shows("Astro"));
    app.press("role:button[Remove Next.js]").unwrap();
    app.wait_for("text:one: many:astro").unwrap();
    // A choice in the list adds one, and the list stays open.
    open(&mut app, "Frameworks");
    app.press("role:option[Remix]").unwrap();
    app.wait_for("text:one: many:astro+remix").unwrap();
    assert!(app.screen().matches("role:listbox").unwrap());
}

#[test]
fn a_chip_takes_the_chip_fill_and_height() {
    let app = start::<Boxes>(true);
    let nodes = app.screen().layout_nodes();
    let title = app.screen().id("text:Astro").unwrap();
    let chip = nodes.iter().find(|n| n.id == title).unwrap().parent;
    let chip = format!("view:{chip}");
    assert_eq!(common::fill(&app, &chip), light(COMBOBOX_CHIP));
    assert_eq!(size(&app, &chip).1, 22.0);
    assert_eq!(size(&app, "role:button[Remove Astro]"), (22.0, 22.0));
}

#[test]
fn an_empty_list_says_so() {
    let mut app = start::<Boxes>(true);
    app.type_text("role:combobox[Framework]", "zzz").unwrap();
    app.run_for(Duration::from_millis(300)).unwrap();
    assert!(app.shows("No items found."));
    let words = app.screen().span_style("text:No items found.").unwrap();
    assert_eq!(Some(words.color.0), light(COMBOBOX_EMPTY_FOREGROUND));
    assert_eq!(words.font_size.0, 14.0);
}

/// A combobox with an empty state of the app's.
#[component]
fn OwnEmpty() -> impl IntoElement {
    view().p(px(20.0)).w(px(320.0)).child(
        combobox()
            .items(frameworks())
            .empty(text("No framework matches."))
            .accessibility_label("Framework"),
    )
}

#[test]
fn the_apps_empty_state_inherits_the_words_look() {
    let mut app = start::<OwnEmpty>(true);
    app.type_text("role:combobox[Framework]", "zzz").unwrap();
    app.run_for(Duration::from_millis(300)).unwrap();
    let words = app
        .screen()
        .span_style("text:No framework matches.")
        .unwrap();
    assert_eq!(Some(words.color.0), light(COMBOBOX_EMPTY_FOREGROUND));
    assert_eq!(words.font_size.0, 14.0);
}

#[test]
fn sizes_with_a_pointer_and_on_touch_screens() {
    let app = start::<Boxes>(true);
    let field = field_of(&app, "Framework");
    assert_eq!(size(&app, &field), (280.0, 32.0));
    assert_eq!(size(&app, &field_of(&app, "Small")).1, 28.0);
    let touch = start::<Boxes>(false);
    assert_eq!(size(&touch, &field_of(&touch, "Framework")).1, 40.0);
    // Several values: as tall as the field while the chips fit.
    assert_eq!(size(&app, &field_of(&app, "Frameworks")).1, 32.0);
}

#[test]
fn a_disabled_combobox_is_dimmed_and_does_not_open() {
    let mut app = start::<Boxes>(true);
    let style = app
        .screen()
        .view_style(&field_of(&app, "Locked"))
        .unwrap()
        .unwrap();
    assert_eq!(style.opacity, 0.5);
    let _ = app.press("role:combobox[Locked]");
    app.run_for(Duration::from_millis(300)).unwrap();
    assert!(!app.screen().matches("role:listbox").unwrap());
}

fn watch_styles(app: &mut Headless) -> Arc<Mutex<HashMap<NodeId, ViewStyle>>> {
    let styles = Arc::new(Mutex::new(HashMap::new()));
    let seen = styles.clone();
    app.observe(move |_, batch| {
        for item in &batch.items {
            if let MountItem::ViewStyle { id, style } = item {
                seen.lock().unwrap().insert(*id, (**style).clone());
            }
        }
    });
    styles
}

/// `Boxes` as `common::start` makes it, with every style it sends kept
/// from the first frame.
fn start_watched() -> (Headless, Arc<Mutex<HashMap<NodeId, ViewStyle>>>) {
    let mut config = EngineConfig {
        viewport: Viewport::new(px(800.0), px(900.0)),
        clock: nsh_engine::Clock::manual(std::time::SystemTime::now()),
        ..EngineConfig::new(nsh_testing::windowed_root::<Boxes>())
    };
    config.environment.os = Os::MacOs;
    let mut app = Headless::start(config).unwrap();
    let styles = watch_styles(&mut app);
    app.first_render().unwrap();
    (app, styles)
}

#[test]
fn the_field_shows_the_focus_while_the_input_has_it() {
    let (mut app, styles) = start_watched();
    let field = field_id(&app, "Framework");
    let rest = styles.lock().unwrap().get(&field).cloned().unwrap();
    let shadows = |s: &ViewStyle| s.effects.as_ref().map_or(0, |e| e.box_shadows.len());
    assert_eq!(shadows(&rest), 1, "{rest:?}");
    app.press_key("Tab").unwrap();
    app.run_for(Duration::from_millis(300)).unwrap();
    let focused = styles.lock().unwrap().get(&field).cloned().unwrap();
    assert_eq!(Some(focused.borders.colors[0].0), light(INPUT_BORDER_FOCUS));
    // The ring joins the shadow.
    assert_eq!(shadows(&focused), 2, "{focused:?}");
    // Open, the focus edge stays and the ring goes (transparent), so the 4
    // points to the list stay clear; closed, with the focus kept, it comes
    // back.
    let ring = |s: &ViewStyle| {
        s.effects
            .as_ref()
            .and_then(|e| e.box_shadows.iter().find(|b| b.spread > px(0.0)))
            .map(|b| b.color.0 & 0xff)
    };
    assert!(ring(&focused).is_some_and(|alpha| alpha > 0), "{focused:?}");
    open(&mut app, "Framework");
    assert!(app.screen().matches("role:listbox").unwrap());
    let open = styles.lock().unwrap().get(&field).cloned().unwrap();
    assert_eq!(Some(open.borders.colors[0].0), light(INPUT_BORDER_FOCUS));
    assert_eq!(ring(&open).unwrap_or(0), 0, "{open:?}");
    app.press_key("Escape").unwrap();
    app.run_for(Duration::from_millis(400)).unwrap();
    assert!(!app.screen().matches("role:listbox").unwrap());
    let closed = styles.lock().unwrap().get(&field).cloned().unwrap();
    assert!(ring(&closed).is_some_and(|alpha| alpha > 0), "{closed:?}");
}

/// The popup around the row `selector` names: its first ancestor with a
/// fill.
fn popup_of(app: &Headless, selector: &str) -> NodeId {
    let nodes = app.screen().layout_nodes();
    let mut at = app.screen().id(selector).unwrap();
    loop {
        let parent = nodes.iter().find(|n| n.id == at).unwrap().parent;
        let style = app.screen().view_style(&format!("view:{parent}")).unwrap();
        if style.is_some_and(|s| s.background_color.is_some()) {
            return parent;
        }
        at = parent;
    }
}

#[test]
fn the_list_is_capped_and_the_popup_shrinks_to_it() {
    let (mut app, _) = start_watched();
    open(&mut app, "Long");
    let popup = popup_of(&app, "role:option[Item 0]");
    let height = size(&app, &format!("view:{popup}")).1;
    // The list's 348 (its padding inside), and the edges.
    assert_eq!(height, 348.0 + 2.0, "popup {height}");
    app.press_key("Escape").unwrap();
    app.run_for(Duration::from_millis(300)).unwrap();
    // A short list: the popup as tall as its rows.
    open(&mut app, "Small");
    let popup = popup_of(&app, "role:option[Astro]");
    let height = size(&app, &format!("view:{popup}")).1;
    assert!(height < 200.0, "popup {height}");
}

/// TSX apps' declarations: the props, the slot and the item type.
#[test]
fn tsx_gets_the_combobox() {
    let registry = nsh_longbridge::registry();
    assert_eq!(registry.undocumented(), Vec::<String>::new());
    let ts = nsh_core::element::declarations(&registry, &[]);
    let start = ts
        .find("export interface ComboboxProps")
        .expect("ComboboxProps");
    let props = &ts[start..start + ts[start..].find("\n}").unwrap()];
    for wanted in [
        "items?: (SelectItem)[];",
        "value?: (string)[] | null;",
        "empty?: JSX.Element",
        "showTrigger?: boolean;",
        "creatable?: boolean;",
        "onQueryChange?:",
        "onCreate?: (event: { query: string }) => void;",
    ] {
        assert!(props.contains(wanted), "{wanted} in:\n{props}");
    }
}

/// The weight of the text `part` in the row `option` names.
fn weight(app: &Headless, option: &str, part: &str) -> u16 {
    app.screen()
        .span_style(&format!("role:option[{option}] >> text:{part}"))
        .unwrap()
        .font_weight
}

#[test]
fn the_matched_part_of_each_title_is_emphasized() {
    let mut app = start::<Boxes>(true);
    app.type_text("role:combobox[Framework]", "NU").unwrap();
    app.run_for(Duration::from_millis(300)).unwrap();
    assert_eq!(weight(&app, "Nuxt.js", "Nu"), 600);
    assert_eq!(weight(&app, "Nuxt.js", "xt.js"), 400);
    // The row is read as its title.
    assert!(app.screen().matches("role:option[Nuxt.js]").unwrap());
    // Several values too: the first match only.
    app.press_key("Escape").unwrap();
    app.type_text("role:combobox[Frameworks]", "t").unwrap();
    app.run_for(Duration::from_millis(300)).unwrap();
    assert_eq!(weight(&app, "SvelteKit", "Svel"), 400);
    assert_eq!(weight(&app, "SvelteKit", "eKit"), 400);
    assert_eq!(weight(&app, "Astro", "t"), 600);
}

/// The app filters the items itself (a server's results), keeping the
/// query.
#[component]
fn Searched() -> impl IntoElement {
    let query = signal(SharedString::default());
    let results = move || {
        let q = query.get().to_lowercase();
        frameworks()
            .into_iter()
            .filter(|i| i.title.to_lowercase().starts_with(&q))
            .collect::<Vec<_>>()
    };
    view().p(px(20.0)).w(px(320.0)).child(
        combobox()
            .items(results)
            .query(move || Some(query.get()))
            .on_query_change(move |q: &SharedString| query.set(q.clone()))
            .accessibility_label("Search"),
    )
}

#[test]
fn the_matched_part_with_the_apps_own_filter() {
    let mut app = start::<Searched>(true);
    app.type_text("role:combobox[Search]", "rem").unwrap();
    app.run_for(Duration::from_millis(300)).unwrap();
    assert_eq!(app.screen().count("role:option").unwrap(), 1);
    assert_eq!(weight(&app, "Remix", "Rem"), 600);
    assert_eq!(weight(&app, "Remix", "ix"), 400);
}

/// Tags that can be made, several; one value that can be made, whose app
/// adds nothing; what each reported.
#[component]
fn Creatable() -> impl IntoElement {
    let items = signal(frameworks());
    let tags = signal(Vec::<SharedString>::new());
    let made = signal(Vec::<SharedString>::new());
    let queries = signal(Vec::<SharedString>::new());
    let confirmed = signal(Vec::<SharedString>::new());
    view()
        .flex_col()
        .gap(px(12.0))
        .p(px(20.0))
        .w(px(320.0))
        .child(
            combobox()
                .multiple(true)
                .creatable(true)
                .items(move || items.get())
                .value(move || Some(tags.get()))
                .on_change(move |v: &Vec<SharedString>| tags.set(v.clone()))
                .on_create(move |q: &SharedString| {
                    let value = SharedString::from(format!("tag-{}", q.to_lowercase()));
                    items.update(|i| i.push(SelectItem::new(value.clone(), q.clone())));
                    tags.update(|t| t.push(value));
                })
                .on_query_change(move |q: &SharedString| queries.update(|l| l.push(q.clone())))
                .accessibility_label("Tags"),
        )
        .child(
            combobox()
                .creatable(true)
                .items(frameworks())
                .on_create(move |q: &SharedString| made.update(|m| m.push(q.clone())))
                .on_confirm(move |v: &Vec<SharedString>| confirmed.set(v.clone()))
                .accessibility_label("One"),
        )
        .child(text(move || {
            format!(
                "tags:{} made:{} confirmed:{}",
                tags.get().join("+"),
                made.get().join("+"),
                confirmed.get().join("+")
            )
        }))
        .child(text(move || format!("queries:{}", queries.get().join("|"))))
}

#[test]
fn the_create_row_makes_the_query_a_new_tag() {
    let mut app = start::<Creatable>(true);
    app.type_text("role:combobox[Tags]", "Qwik").unwrap();
    app.run_for(Duration::from_millis(300)).unwrap();
    // Alone, so highlighted: Return chooses it.
    assert_eq!(app.screen().count("role:option").unwrap(), 1);
    assert!(app.screen().matches("role:option[Create “Qwik”]").unwrap());
    assert_eq!(weight(&app, "Create “Qwik”", "Qwik"), 600);
    assert_eq!(weight(&app, "Create “Qwik”", "Create"), 400);
    app.press_key("Return").unwrap();
    app.wait_for("text:tags:tag-qwik made: confirmed:").unwrap();
    // The chip, the list still open, the query emptied (and said so).
    assert!(app.shows("Qwik"));
    assert!(app.screen().matches("role:listbox").unwrap());
    assert!(
        app.shows("queries:Q|Qw|Qwi|Qwik|"),
        "{:?}",
        app.screen().texts()
    );
    // The input is empty: what is typed next is the whole query. The new
    // item is chosen, and a row of its own now.
    app.type_text("role:combobox[Tags]", "q").unwrap();
    app.run_for(Duration::from_millis(300)).unwrap();
    assert!(
        app.shows("queries:Q|Qw|Qwi|Qwik||q"),
        "{:?}",
        app.screen().texts()
    );
    assert!(app.screen().matches("role:option[Qwik]").unwrap());
    app.type_text("role:combobox[Tags]", "wik").unwrap();
    app.run_for(Duration::from_millis(300)).unwrap();
    assert!(!app.screen().matches("role:option[Create “qwik”]").unwrap());
}

#[test]
fn no_create_row_for_a_title_there_is_or_a_blank_query() {
    let mut app = start::<Creatable>(true);
    // "astro " is "Astro", ignoring case and the spaces at its ends.
    app.type_text("role:combobox[Tags]", "astro ").unwrap();
    app.run_for(Duration::from_millis(300)).unwrap();
    assert_eq!(app.screen().count("role:option").unwrap(), 0);
    app.press_key("Escape").unwrap();
    app.run_for(Duration::from_millis(300)).unwrap();
    app.type_text("role:combobox[One]", "  ").unwrap();
    app.run_for(Duration::from_millis(300)).unwrap();
    assert!(!app.screen().matches("role:option[Create “  ”]").unwrap());
}

#[test]
fn the_create_row_comes_last_and_the_keys_reach_it() {
    let mut app = start::<Creatable>(true);
    app.type_text("role:combobox[Tags]", "st").unwrap();
    app.run_for(Duration::from_millis(300)).unwrap();
    // "Astro", then the create row; Return takes the first match.
    assert_eq!(app.screen().count("role:option").unwrap(), 2);
    app.press_key("Down").unwrap();
    app.press_key("Return").unwrap();
    app.wait_for("text:tags:tag-st made: confirmed:").unwrap();
}

#[test]
fn one_value_made_closes_the_list_and_the_value_stays_the_apps() {
    let mut app = start::<Creatable>(true);
    // Typed into the closed list: it opens, and the create row is
    // highlighted at once.
    app.type_text("role:combobox[One]", "Q").unwrap();
    app.press_key("Return").unwrap();
    app.wait_for("text:tags: made:Q confirmed:").unwrap();
    app.run_for(Duration::from_millis(300)).unwrap();
    assert!(!app.screen().matches("role:listbox").unwrap());
    // Again, the app having added nothing: made again.
    app.type_text("role:combobox[One]", "Ember").unwrap();
    app.run_for(Duration::from_millis(300)).unwrap();
    app.press_key("Return").unwrap();
    app.wait_for("text:tags: made:Q+Ember confirmed:").unwrap();
}

/// An app that watches the combobox's own query (a recent group while it
/// is empty).
#[component]
fn Watched() -> impl IntoElement {
    let typed = signal(SharedString::from("-"));
    view()
        .p(px(20.0))
        .w(px(320.0))
        .child(
            combobox()
                .items(frameworks())
                .on_query_change(move |q: &SharedString| typed.set(q.clone()))
                .accessibility_label("Watched"),
        )
        .child(text(move || format!("typed:[{}]", typed.get())))
}

#[test]
fn the_query_going_back_is_reported() {
    let mut app = start::<Watched>(true);
    app.type_text("role:combobox[Watched]", "ne").unwrap();
    app.wait_for("text:typed:[ne]").unwrap();
    app.press_key("Escape").unwrap();
    app.run_for(Duration::from_millis(500)).unwrap();
    app.wait_for("text:typed:[]").unwrap();
    // Opened again, every item shows.
    open(&mut app, "Watched");
    assert_eq!(app.screen().count("role:option").unwrap(), 5);
}
