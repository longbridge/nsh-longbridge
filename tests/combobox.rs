//! `Combobox` in a headless engine: typing filters the list, a choice
//! sets the value and closes it, several values show as chips whose
//! buttons remove them, the clear button, the empty list's words, the
//! list's height capped (the popup shrinking with it), the field look's
//! focus around the input, sizes, disabled, and what TSX apps get.

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
    SelectItem, combobox, components::combobox::COMBOBOX_CHIP,
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
        "onQueryChange?:",
    ] {
        assert!(props.contains(wanted), "{wanted} in:\n{props}");
    }
}
