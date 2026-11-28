//! `Select`: a select on `nsh/base`'s `Select` (the value, the
//! popup at the trigger or over it, the sheet on touch screens, the keys,
//! typeahead, the virtualized list, the combobox, listbox and option roles
//! and their states for screen readers). The library draws the trigger
//! (the field look), the list's surface (`foundation::popup`), its rows and
//! headers (`foundation::menu`), the check on the chosen row, and with
//! `searchable` / `cleanable` the look of `nsh/base`'s search field, empty
//! state and clear button. API: `docs/components/select.md`.

use std::rc::Rc;

use nsh_base::{
    OpenChangeReason, SelectEvent as BaseSelectEvent, select as base_select, terminal::is_terminal,
};
use nsh_core::{
    element::{InputPartStyle, Renderer, Style, StyleRefinement, TextPartStyle, screen_reader},
    keymap::When,
    prelude::*,
};

pub use nsh_base::{SelectEntry, SelectItem, SelectValue};

use crate::{
    components::input::{self, vertical_hit_slop},
    foundation::{
        class::Classes,
        field::{
            self, INPUT_BACKGROUND, INPUT_BORDER_FOCUS, INPUT_BORDER_INVALID, INPUT_FOREGROUND,
            INPUT_PLACEHOLDER_FOREGROUND, Joined, Kind, field_button_frame, field_button_hover,
            popup_field_look,
        },
        field_ref::FieldRef,
        icon::{IconName, icon_size_token, icon_view, provide_text_color},
        labels::{self, Label},
        menu::{
            self, ListLook, MENU_ITEM_PADDING_X, MENU_PADDING, MENU_SEPARATOR, MENU_SHEET_PADDING,
            list_look,
        },
        popup::{self, POPOVER_MARGIN, POPOVER_OFFSET},
        size::Size,
        trigger,
        typography::line_height,
    },
    theme::{
        Theme,
        seeds::SPACING,
        semantic::{
            BORDER_WIDTH, INPUT, MUTED_FOREGROUND, OPACITY_DISABLED, POPOVER_FOREGROUND, TEXT_SM,
        },
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
    /// The clear button's cross (`cleanable`): a button of its own,
    /// stronger than the chevron.
    SELECT_CLEAR_FOREGROUND: Color = "select-clear-foreground" => |t| t.color(MUTED_FOREGROUND),
    /// The words of a list no option shows in (a search that matches
    /// nothing).
    SELECT_EMPTY_FOREGROUND: Color = "select-empty-foreground" => |t| t.color(MUTED_FOREGROUND),
    /// A searchable list's greatest height on desktops.
    SELECT_LIST_MAX_HEIGHT: Length = "select-list-max-height" => |t| t.length(SPACING) * 87.0,
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
    /// It does not open, takes no focus, and is dimmed. Or'ed with an
    /// enclosing `Field`'s or `FieldSet`'s.
    #[prop(default)]
    pub disabled: Prop<bool>,
    /// The app's checks reject the value: a red edge (and ring while
    /// focused); screen readers say it is invalid. Or'ed with an enclosing
    /// `Field`'s.
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
    /// over it on macOS, on a side elsewhere; a searchable list on a side
    /// everywhere.
    pub cover: Prop<Option<bool>>,
    /// A search field at the top of the list filters the options by title
    /// (ignoring case); the trigger stays a button showing the value.
    #[prop(default)]
    pub searchable: Prop<bool>,
    /// The search field's placeholder, which screen readers also hear as
    /// its name ("Search countries"). Left out, "Search" (translatable).
    pub search_placeholder: Prop<Option<SharedString>>,
    /// What the list shows while no option does (a search that matches
    /// nothing). Left out, "No items found." (translatable).
    pub empty: Prop<Option<Slot>>,
    /// A clear button on the trigger while a value is set: a press (or
    /// Delete or Backspace on the focused trigger) reports a `null` value.
    #[prop(default)]
    pub cleanable: Prop<bool>,
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
    /// Tailwind classes of the trigger's chevron and the clear button's
    /// cross.
    pub icon_class: Prop<Option<SharedString>>,
    /// The inline style of the trigger's chevron and the clear button's
    /// cross.
    #[prop(default)]
    pub icon_style: Prop<Style>,
    /// Tailwind classes of the search field (`searchable`).
    pub search_class: Prop<Option<SharedString>>,
    /// The inline style of the search field, over the library's look: its
    /// text styles too (`color`, `fontSize`, `placeholderColor`).
    #[prop(default)]
    pub search_style: Prop<InputPartStyle>,
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
    /// closed trigger), or cleared it (`null`: the clear button, Delete or
    /// Backspace): its value, which the app makes `value`.
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

/// The methods of a [`Select`]'s ref.
pub struct SelectApi {
    trigger: ElementRef,
    disabled: Prop<bool>,
    joined: Joined,
}

#[methods]
impl SelectApi {
    /// Moves the keyboard focus to the trigger (the list stays closed);
    /// nothing while disabled.
    pub fn focus(&self) {
        if !self.disabled.get_untracked() && !untracked(|| self.joined.disabled()) {
            self.trigger.focus();
        }
    }
}

/// A select: one value chosen from a list. A trigger drawn as
/// a field shows the chosen item's title (or the placeholder) and opens the
/// list: a popup at the trigger on desktops (over it on macOS), a sheet at
/// the bottom on touch screens. Its options are data (`items`), so a list
/// of thousands makes only the rows in view.
#[component(methods = SelectApi)]
pub fn Select(props: SelectProps, cx: Scope<SelectEvent>) -> impl IntoElement {
    provide_text_color();
    let t = theme();
    let size = move || props.size.get();
    // A label's press focuses the trigger, as a browser's label does.
    let trigger_ref = element_ref();
    let joined = field::join(cx.__host(), Kind::Focus(trigger_ref), move || {
        props.disabled.get()
    });
    // One line of text in the middle of the trigger.
    joined.report_first_line(move || t.length(input::height_token(size())) / 2.0);
    cx.methods(SelectApi {
        trigger: trigger_ref,
        disabled: props.disabled,
        joined,
    });
    let disabled = move || props.disabled.get() || joined.disabled();
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
    // A `TableRow` holding it stays lit while it is open.
    trigger::report_open(is_open);
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
    // The search field's words: its placeholder and its name.
    let search_words = move || {
        props
            .search_placeholder
            .get()
            .unwrap_or_else(|| labels::text(Label::Search))
    };
    let mut root = base_select()
        .items(props.items)
        .value(move || SelectValue::Controlled(current.get()))
        .open(props.open)
        .default_open(props.default_open)
        .placeholder(props.placeholder)
        .disabled(disabled)
        .accessibility_label(props.accessibility_label)
        .placement(props.placement)
        .align(props.align)
        // A searchable list drops to a side, also on macOS: a field typed
        // into stays put while the query moves the rows (Decisions 6).
        .cover(move || {
            props
                .cover
                .get()
                .or_else(|| props.searchable.get().then_some(false))
        })
        .offset(move || t.length(POPOVER_OFFSET))
        .margin(move || t.length(POPOVER_MARGIN))
        .searchable(props.searchable)
        .search_placeholder(move || Some(search_words()))
        .search_accessibility_label(move || Some(search_words()))
        .cleanable(props.cleanable)
        .clear_button_accessibility_label(move || Some(labels::text(Label::Clear)))
        .on_change(move |value: &Option<SharedString>| {
            if props.value.get_untracked() == SelectValue::Uncontrolled {
                own.set(value.clone());
            }
            cx.emit(SelectEvent::Change(value.clone()));
            // Only the clear button reports none: the button the focus (or
            // a screen reader's cursor) was on is gone, so it goes to the
            // trigger, which says the placeholder (Decisions 11).
            if value.is_none() && (is_desktop() || untracked(screen_reader)) {
                trigger_ref.focus();
            }
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

    let clear_shown = move || props.cleanable.get() && current.with(Option::is_some);
    root = root.trigger(trigger(
        props.clone(),
        t,
        current,
        trigger_ref,
        joined,
        clear_shown,
        is_open,
    ));
    // Delete and Backspace on the focused trigger clear the value: the
    // keyboard's way to the clear button, which is no stop of Tab
    // (Decisions 9).
    let focused = signal(false);
    trigger_ref.on_focus_in(move || focused.set(true));
    trigger_ref.on_focus_out(move || focused.set(false));
    effect(move || {
        if !focused.get() || is_open() || disabled() || !clear_shown() {
            return;
        }
        for key in ["backspace", "delete"] {
            on_key_with(
                key,
                KeyOptions {
                    when: When::NotEditing,
                    repeat: false,
                    for_focus: true,
                    ..Default::default()
                },
                move |_| {
                    if props.value.get_untracked() == SelectValue::Uncontrolled {
                        own.set(None);
                    }
                    cx.emit(SelectEvent::Change(None));
                },
            );
        }
    });
    root = root.clear_button(
        icon_view(t, || IconName::X, move || t.length(icon_size_token(size())))
            .text_color(move || t.color(SELECT_CLEAR_FOREGROUND))
            .app_class(props.icon_class)
            .inline_style(move || props.icon_style.get()),
    );
    // The words sit `spacing × 3` from the search field's line and from
    // the surface's edge: the empty list above keeps its padding (twice
    // `menu-padding`), which the space above them counts. The app's
    // `empty` inherits their color and size.
    let empty_space = move || t.length(SPACING) * 3.0;
    root = root.empty(
        view()
            .pt(move || {
                let list = if menu::list_as_sheet() {
                    t.length(MENU_SHEET_PADDING)
                } else {
                    t.length(MENU_PADDING)
                };
                (empty_space() - list * 2.0).max(px(0.0))
            })
            .pb(empty_space)
            .px(move || t.length(MENU_ITEM_PADDING_X))
            .items_center()
            .text_size(move || t.length(TEXT_SM))
            .line_height(move || line_height(t.length(TEXT_SM)))
            .text_color(move || t.color(SELECT_EMPTY_FOREGROUND))
            .child(match props.empty.get_untracked() {
                Some(_) => props.empty.place(|e| e).into_element(),
                None => text(move || labels::text(Label::NoItems)).into_element(),
            }),
    );
    list_look!(
        root,
        t,
        ListLook {
            size: props.size,
            chosen: Rc::new(move |value: &SharedString| {
                current.with(|c| c.as_ref() == Some(value))
            }),
            render_item: props.render_item,
            items: computed(move || props.items.get()),
            title_weight: None,
            max_height: None,
            content_class: props.content_class,
            content_style: props.content_style,
            item_class: props.item_class,
            item_style: props.item_style,
            label_class: props.label_class,
            label_style: props.label_style,
        }
    );
    {
        let parts = root.__element().__props_mut();
        // A searchable list is capped on desktops (Decisions 6); the sheet
        // keeps its own height.
        let list_style = parts.list_style;
        parts.list_style = Prop::from(IntoReactive::<Style>::into_reactive(move || {
            let look = list_style.get();
            if props.searchable.get() && !menu::list_as_sheet() {
                look.then(&Style::from(
                    StyleRefinement::default().max_h(t.length(SELECT_LIST_MAX_HEIGHT)),
                ))
            } else {
                look
            }
        }));
        parts.search_class = props.search_class;
        parts.search_style = Prop::from(IntoReactive::<InputPartStyle>::into_reactive(move || {
            search_style(t, size(), props.search_style.get())
        }));
        let disabled = computed(disabled);
        parts.clear_button_style = Prop::from(IntoReactive::<Style>::into_reactive(move || {
            clear_style(t, size(), disabled.get())
        }));
        parts.clear_button_class = Prop::from(IntoReactive::<Option<SharedString>>::into_reactive(
            move || Some(clear_class(t, disabled.get())),
        ));
    }
    root
}

/// The search field: a band across the surface above the rows, as tall as
/// the trigger (a row in the sheet), its text where the rows' titles
/// start, the menu's row text, no fill, edge or ring of its own and the
/// menu's separator under it; the app's style last.
fn search_style(t: Theme, size: Size, app: InputPartStyle) -> InputPartStyle {
    let sheet = menu::list_as_sheet();
    let height = if sheet {
        t.length(menu::item_height_token(size))
    } else {
        t.length(input::height_token(size))
    };
    let inset = if sheet {
        t.length(MENU_SHEET_PADDING)
    } else {
        t.length(MENU_PADDING)
    } + t.length(MENU_ITEM_PADDING_X);
    let font_size = t.length(menu::item_font_size_token(size));
    let look = StyleRefinement::default()
        .flex_shrink_0()
        .h(height)
        .px(inset)
        .text_size(font_size)
        .line_height(line_height(font_size))
        .text_color(t.color(POPOVER_FOREGROUND))
        .placeholder_color(t.color(INPUT_PLACEHOLDER_FOREGROUND))
        .bg(AdaptiveColor::TRANSPARENT)
        .border_b(t.length(BORDER_WIDTH))
        .border_color(t.color(MENU_SEPARATOR));
    InputPartStyle::from(Style::from(look).then(&Style::from(app)))
}

/// The clear button's frame over the trigger's end: the square of a button
/// in a field, inset 4 (its place is `clear_class`'s); dimmed with the
/// trigger while disabled. Combobox's cross, drawn the same way.
fn clear_style(t: Theme, size: Size, disabled: bool) -> Style {
    let look = field_button_frame(StyleRefinement::default(), t, size)
        .cursor(Cursor::Default)
        .opacity(if disabled {
            t.number(OPACITY_DISABLED)
        } else {
            1.0
        });
    Style::from(look)
}

/// The clear button's place (over the trigger's end, `end` so a
/// right-to-left layout mirrors it) and the ghost button's hover fill
/// (none while pressed or disabled), as classes made from the tokens: only
/// `nsh/base` knows the button's states.
fn clear_class(t: Theme, disabled: bool) -> SharedString {
    let classes = Classes::new().push("absolute top-[4px] end-[4px]");
    if disabled {
        return classes.build();
    }
    field_button_hover(classes, t).build()
}

/// The trigger: a field showing the chosen title or the placeholder, and
/// the chevron. `nsh/base` makes it the combobox (its role, value, states,
/// keys and press) and places the list against it.
/// The trigger's view carries `trigger_ref` and the field's described-by
/// target: `nsh/base`'s slot binds them to the combobox element it makes of
/// it.
fn trigger(
    props: SelectProps,
    t: Theme,
    current: Memo<Option<SharedString>>,
    trigger_ref: ElementRef,
    joined: Joined,
    clear_shown: impl Fn() -> bool + Copy + 'static,
    open: impl Fn() -> bool + Copy + 'static,
) -> impl IntoElement {
    let size = move || props.size.get();
    let height = move || t.length(input::height_token(size()));
    let font_size = move || t.length(input::font_size_token(size()));
    let invalid = move || props.invalid.get() || joined.invalid();
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
        .element_ref(&trigger_ref)
        .accessibility_described_by_target(move || joined.described_by())
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
        // Open, it keeps the focus edge, and its focus no ring
        // (`popup_field_look`).
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
    // The clear button takes its place: kept transparent, so the value is
    // cut at the same width (Decisions 10).
    .opacity(move || if clear_shown() { 0.0_f32 } else { 1.0 })
    .app_class(props.icon_class)
    .inline_style(move || props.icon_style.get());
    popup_field_look(el, t, invalid, open)
        .app_class(props.trigger_class)
        .inline_style(move || props.trigger_style.get())
        .child(value)
        .child(chevron)
}

/// Rust: `label().html_for(country)` with `country` this select's ref.
impl From<ComponentRef<Select>> for FieldRef {
    fn from(r: ComponentRef<Select>) -> Self {
        FieldRef::lazy(move || r.node())
    }
}

nsh_core::static_reactive!(FieldRef: ComponentRef<Select>);
