//! `Select`: a select on `nsh/base`'s `Select` (the value, the
//! popup at the trigger or over it, the sheet on touch screens, the keys,
//! typeahead, the virtualized list, the combobox, listbox and option roles
//! and their states for screen readers). The library draws the trigger
//! (the field look), the list's surface (`foundation::popup`), its rows and
//! headers (`foundation::menu`) and the check on the chosen row. API:
//! `docs/components/select.md`.

use nsh_base::{
    OpenChangeReason, SelectEvent as BaseSelectEvent, select as base_select, terminal::is_terminal,
};
use nsh_core::{
    element::{Renderer, Style, StyleRefinement, TextPartStyle, safe_area},
    prelude::*,
};

pub use nsh_base::{SelectEntry, SelectItem, SelectValue};

use crate::{
    components::{
        dropdown_menu::sheet_look,
        input::{self, vertical_hit_slop},
    },
    foundation::{
        field::{
            INPUT_BACKGROUND, INPUT_BORDER_FOCUS, INPUT_BORDER_INVALID, INPUT_FOREGROUND,
            INPUT_PLACEHOLDER_FOREGROUND, field_look,
        },
        icon::{IconName, icon_size_token, icon_view, provide_text_color},
        menu::{
            MENU_ITEM_FOCUS, MENU_ITEM_FOCUS_FOREGROUND, MENU_ITEM_GAP, MENU_ITEM_PADDING_X,
            MENU_MIN_WIDTH, MENU_PADDING, MENU_SHEET_PADDING, item_height_token, item_row,
            list_as_sheet,
        },
        popup::{self, POPOVER_MARGIN, POPOVER_OFFSET, set_popup_style},
        size::Size,
        typography::line_height,
    },
    theme::{
        Theme,
        seeds::SPACING,
        semantic::{INPUT, MUTED_FOREGROUND, OPACITY_DISABLED, OVERLAY},
        theme,
    },
    tokens,
};

tokens! {
    /// The trigger's fill under the pointer: unchanged, `input` at 50 % in
    /// the dark.
    SELECT_TRIGGER_HOVER: Color = "select-trigger-hover" => |t| t.light_dark(t.color(INPUT_BACKGROUND), t.color(INPUT).opacity(0.5)),
    /// The trigger's chevron.
    SELECT_ICON_FOREGROUND: Color = "select-icon-foreground" => |t| t.color(MUTED_FOREGROUND).opacity(0.5),
    /// A group header's text.
    SELECT_LABEL_FOREGROUND: Color = "select-label-foreground" => |t| t.color(MUTED_FOREGROUND),
    /// A group header's text size: Tailwind's `text-xs`.
    SELECT_LABEL_FONT_SIZE: Length = "select-label-font-size" => |t| t.px(12.0),
}

/// The props of [`Select`].
#[derive(Debug, Clone, PartialEq, Props)]
pub struct SelectProps {
    /// The options, in order: a value and a title each, optionally
    /// disabled or in a group (adjacent items of one `group` show under a
    /// header with its title).
    #[prop(default)]
    pub items: Prop<Vec<SelectItem>>,
    /// The chosen item's value (controlled), `null` for none: the app sets
    /// it in `onChange`. Left out, the select keeps its own, from
    /// `defaultValue`.
    #[prop(default)]
    pub value: Prop<SelectValue>,
    /// The first value while `value` is left out.
    pub default_value: Prop<Option<SharedString>>,
    /// What the trigger shows while nothing is chosen.
    pub placeholder: Prop<Option<SharedString>>,
    /// Its size (`xs`, `sm`, `md`, `lg`): the trigger's height, text,
    /// padding and chevron, as an `Input` of that size, and the rows'.
    #[prop(default)]
    pub size: Prop<Size>,
    /// It does not open, takes no focus, and is dimmed.
    #[prop(default)]
    pub disabled: Prop<bool>,
    /// The app's checks reject the value: a red edge (and ring while
    /// focused); screen readers say it is invalid.
    #[prop(default)]
    pub invalid: Prop<bool>,
    /// Open (controlled): the app sets it in `onOpenChange`. Left out, the
    /// select keeps its own state.
    pub open: Prop<Option<bool>>,
    /// Open at first, while `open` is left out.
    #[prop(default)]
    pub default_open: Prop<bool>,
    /// Desktops, the list on a side of the trigger: the side (`"bottom"`
    /// by default); the other one when only it has room.
    #[prop(default)]
    pub placement: Prop<Placement>,
    /// Desktops, the list on a side of the trigger: how it lines up with
    /// the trigger (`"start"` by default).
    #[prop(default = Align::Start)]
    pub align: Prop<Align>,
    /// Desktops: the list opens with the chosen row over the trigger
    /// (`true`) or on a side of it (`false`). Left out, the platform's way:
    /// over it on macOS, on a side elsewhere.
    pub cover: Prop<Option<bool>>,
    /// A row's content in place of its title (an icon and a title): the
    /// library still draws the row and the check. It gets what the item is,
    /// never its state.
    pub render_item: Prop<Option<Renderer<SelectEntry>>>,
    /// What screen readers call it (what the choice is about).
    pub accessibility_label: Prop<Option<SharedString>>,
    /// Tailwind classes of the trigger, over the field look.
    pub trigger_class: Prop<Option<SharedString>>,
    /// The inline style of the trigger, over the field look.
    #[prop(default)]
    pub trigger_style: Prop<Style>,
    /// Tailwind classes of the trigger's text (the title or the
    /// placeholder).
    pub value_class: Prop<Option<SharedString>>,
    /// The inline style of the trigger's text.
    #[prop(default)]
    pub value_style: Prop<TextPartStyle>,
    /// Tailwind classes of the trigger's chevron.
    pub icon_class: Prop<Option<SharedString>>,
    /// The inline style of the trigger's chevron.
    #[prop(default)]
    pub icon_style: Prop<Style>,
    /// Tailwind classes of the popup (or sheet): they add to the library's
    /// look but cannot override what it sets inline (fill, edge, corners,
    /// shadow).
    pub content_class: Prop<Option<SharedString>>,
    /// The inline style of the popup (or sheet), over the library's look.
    #[prop(default)]
    pub content_style: Prop<Style>,
    /// Tailwind classes of each row: they add to the library's look (its
    /// highlight is a class too, which the app's follow).
    pub item_class: Prop<Option<SharedString>>,
    /// The inline style of each row, over the library's look.
    #[prop(default)]
    pub item_style: Prop<TextPartStyle>,
    /// Tailwind classes of each group header.
    pub label_class: Prop<Option<SharedString>>,
    /// The inline style of each group header, over the library's look.
    #[prop(default)]
    pub label_style: Prop<Style>,
}

/// The events of a [`Select`].
#[derive(Debug, Clone, PartialEq, Event)]
pub enum SelectEvent {
    /// The user chose another item (a press, Return, typeahead on the
    /// closed trigger): its value, which the app makes `value`.
    #[args(value)]
    Change(Option<SharedString>),
    /// The user asks to open or close the list, with why: set `open` to
    /// follow it.
    #[args(open, reason)]
    OpenChange(bool, OpenChangeReason),
    /// The list is shown (a sheet once it came in).
    Open,
    /// The list is gone, after its exit (whoever closed it).
    Close,
}

/// A select: one value chosen from a list. A trigger drawn as
/// a field shows the chosen item's title (or the placeholder) and opens the
/// list: a popup at the trigger on desktops (over it on macOS), a sheet at
/// the bottom on touch screens. Its options are data (`items`), so a list
/// of thousands makes only the rows in view.
#[component]
pub fn Select(props: SelectProps, cx: Scope<SelectEvent>) -> impl IntoElement {
    provide_text_color();
    let t = theme();
    let size = move || props.size.get();
    // The value the trigger and the checks show: the app's, else its own.
    let own = signal(props.default_value.get_untracked());
    let current = computed(move || match props.value.get() {
        SelectValue::Uncontrolled => own.get(),
        SelectValue::Controlled(value) => value,
    });
    // Open, from the request to the end of the exit, the list counts among
    // the open popups (as a popover's content does), so the trigger's
    // tooltip stays hidden over it. `nsh/base` makes no content of the
    // library's on each opening, so the select follows its open state.
    let own_open = signal(props.default_open.get_untracked());
    let is_open = move || props.open.get().unwrap_or_else(|| own_open.get());
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
    let terminal = is_terminal();
    let mut root = base_select()
        .items(props.items)
        .value(move || SelectValue::Controlled(current.get()))
        .open(props.open)
        .default_open(props.default_open)
        .placeholder(props.placeholder)
        .disabled(props.disabled)
        .accessibility_label(props.accessibility_label)
        .placement(props.placement)
        .align(props.align)
        .cover(props.cover)
        .offset(move || t.length(POPOVER_OFFSET))
        .margin(move || t.length(POPOVER_MARGIN))
        .on_change(move |value: &Option<SharedString>| {
            if props.value.get_untracked() == SelectValue::Uncontrolled {
                own.set(value.clone());
            }
            cx.emit(SelectEvent::Change(value.clone()));
        })
        .on_open_change(move |e: &BaseSelectEvent| {
            if let BaseSelectEvent::OpenChange(open, reason) = e {
                if props.open.get_untracked().is_none() {
                    own_open.set(*open);
                }
                cx.emit(SelectEvent::OpenChange(*open, *reason));
            }
        })
        .on_open(move || cx.emit(SelectEvent::Open))
        .on_close(move || {
            if !untracked(is_open) {
                held.set(false);
            }
            cx.emit(SelectEvent::Close)
        })
        // `w-fit`: as wide as its content where a column stretches
        // its children, while a row's `items-center` still centers it (an
        // `align-self` would override the row's alignment). The app's class
        // goes over it.
        .app_class(Some(SharedString::from("w-fit")));
    if terminal {
        // `nsh/base`'s terminal trigger and rows.
        root.__element().__props_mut().render_item = props.render_item;
        return root;
    }

    root = root
        .trigger(trigger(props.clone(), t, current))
        .popup_class(props.content_class)
        // No `onPosition` on a select: no slide, zooming from the middle.
        .popup_starting_class(move || popup::ending_class(popup::animate(t)))
        .popup_ending_class(move || popup::ending_class(popup::animate(t)))
        .backdrop_style(move |s| s.bg(move || t.color(OVERLAY)))
        .item_class(move || Some(item_class(t, props.item_class.get())))
        .group_class(props.label_class)
        .render_item(row_content(props.clone(), t, current));
    let content_style = props.content_style;
    set_popup_style(&mut root, move || {
        let look = if list_as_sheet() {
            sheet_look(t).p(px(0.0)).pb(safe_area().bottom)
        } else {
            popup::surface(StyleRefinement::default(), t).min_w(move || t.length(MENU_MIN_WIDTH))
        };
        Style::from(look).then(&content_style.get())
    });
    let parts = root.__element().__props_mut();
    parts.list_style = Prop::from(IntoReactive::<Style>::into_reactive(move || {
        let padding = if list_as_sheet() {
            t.length(MENU_SHEET_PADDING)
        } else {
            t.length(MENU_PADDING)
        };
        Style::from(StyleRefinement::default().p(padding))
    }));
    parts.item_style = Prop::from(IntoReactive::<TextPartStyle>::into_reactive(move || {
        let size = size();
        // One child, the row's content (`renderItem`'s view), stretched
        // across it: the content lays out the title and the check.
        let look = item_row(StyleRefinement::default(), t, move || size)
            .flex_col()
            .items_stretch()
            .justify_center();
        TextPartStyle::from(Style::from(look).then(&Style::from(props.item_style.get())))
    }));
    parts.group_style = Prop::from(IntoReactive::<Style>::into_reactive(move || {
        let size = size();
        let font_size = t.length(SELECT_LABEL_FONT_SIZE);
        let look = StyleRefinement::default()
            .flex_row()
            .items_center()
            .flex_shrink_0()
            .h(t.length(item_height_token(size)))
            .px(t.length(MENU_ITEM_PADDING_X))
            .text_size(font_size)
            .line_height(line_height(font_size))
            .font_weight(FontWeight::NORMAL)
            .whitespace_nowrap()
            .text_color(t.color(SELECT_LABEL_FOREGROUND))
            .cursor(Cursor::Default);
        Style::from(look).then(&props.label_style.get())
    }));
    root
}

/// The trigger: a field showing the chosen title or the placeholder, and
/// the chevron. `nsh/base` makes it the combobox (its role, value, states,
/// keys and press) and places the list against it.
fn trigger(props: SelectProps, t: Theme, current: Memo<Option<SharedString>>) -> impl IntoElement {
    let size = move || props.size.get();
    let height = move || t.length(input::height_token(size()));
    let font_size = move || t.length(input::font_size_token(size()));
    let invalid = move || props.invalid.get();
    let title = computed(move || {
        current.with(|value| {
            let value = value.as_ref()?;
            props.items.with(|items| {
                items
                    .iter()
                    .find(|i| i.value == *value)
                    .map(|i| i.title.clone())
            })
        })
    });
    let el = view()
        .flex_row()
        .items_center()
        .justify_between()
        .gap(move || t.length(SPACING) * 2.0)
        .h(height)
        .px(move || t.length(input::padding_token(size())))
        .text_size(font_size)
        .line_height(move || line_height(font_size()))
        .whitespace_nowrap()
        .cursor(Cursor::Default)
        .hit_slop(move || vertical_hit_slop(t, height()))
        .accessibility_state(move || AccessibilityState {
            invalid: invalid(),
            ..AccessibilityState::default()
        })
        .hover(move |s| s.bg(move || t.color(SELECT_TRIGGER_HOVER)))
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
    let value = text(move || {
        title
            .get()
            .or_else(|| props.placeholder.get())
            .unwrap_or_default()
    })
    .flex_1()
    .min_w(px(0.0))
    .truncate()
    .text_color(move || {
        if title.with(Option::is_some) {
            t.color(INPUT_FOREGROUND)
        } else {
            t.color(INPUT_PLACEHOLDER_FOREGROUND)
        }
    })
    .app_class(props.value_class)
    .inline_style(move || Style::from(props.value_style.get()));
    let chevron = icon_view(
        t,
        || IconName::ChevronDown,
        move || t.length(icon_size_token(size())),
    )
    .text_color(move || t.color(SELECT_ICON_FOREGROUND))
    .app_class(props.icon_class)
    .inline_style(move || props.icon_style.get());
    field_look(el, t, invalid)
        .app_class(props.trigger_class)
        .inline_style(move || props.trigger_style.get())
        .child(value)
        .child(chevron)
}

/// The rows' classes: the highlight (`focus:`, and `active:` for a finger)
/// from the tokens, which an inline style cannot take, then the app's.
fn item_class(t: Theme, app: Option<SharedString>) -> SharedString {
    let fill = t.color(MENU_ITEM_FOCUS);
    let text = t.color(MENU_ITEM_FOCUS_FOREGROUND);
    let mut class = String::new();
    for state in ["focus", "active"] {
        class.push_str(&format!(
            "{state}:bg-[{}] dark:{state}:bg-[{}] {state}:text-[{}] dark:{state}:text-[{}] ",
            fill.light.to_hex(),
            fill.dark.to_hex(),
            text.light.to_hex(),
            text.dark.to_hex(),
        ));
    }
    if let Some(app) = app {
        class.push_str(&app);
    }
    class.truncate(class.trim_end().len());
    class.into()
}

/// A row's content: the title (or the app's `renderItem`), then a box
/// holding the check while the item is the value.
fn row_content(
    props: SelectProps,
    t: Theme,
    current: Memo<Option<SharedString>>,
) -> impl Fn(&SelectEntry) -> View + 'static {
    let side = move || t.length(icon_size_token(props.size.get()));
    move |entry: &SelectEntry| {
        let value = entry.value.clone();
        let chosen = move || current.with(|c| c.as_ref() == Some(&value));
        let label = match props.render_item.get() {
            Some(render) => render
                .render(format!("item:{}", entry.index), entry.clone())
                .flex_1()
                .min_w(px(0.0))
                .into_element(),
            None => text(entry.title.clone())
                .flex_1()
                .min_w(px(0.0))
                .truncate()
                .into_element(),
        };
        let check = icon_view(t, || IconName::Check, side)
            .opacity(move || if chosen() { 1.0_f32 } else { 0.0 });
        view()
            .flex_row()
            .items_center()
            .gap(move || t.length(MENU_ITEM_GAP))
            .when(entry.disabled, |v| {
                v.opacity(move || t.number(OPACITY_DISABLED))
            })
            .child(label)
            .child(check)
    }
}
