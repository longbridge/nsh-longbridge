//! `Combobox`: a text field that filters a list and chooses one value or
//! several, on `nsh/base`'s `Combobox` (the value and the query, the input
//! and its keys, the popup at the field or the sheet on touch screens, the
//! virtualized list, the combobox, listbox and option roles). The library
//! draws the field around the input (the field look), its chevron and
//! clear buttons, the chips of several values, and the list (as Select's,
//! `foundation::menu`). API: `docs/components/combobox.md`.

use std::rc::Rc;

use nsh_base::{
    ComboboxEvent as BaseComboboxEvent, OpenChangeReason, combobox as base_combobox,
    terminal::is_terminal,
};
use nsh_core::{
    element::{InputPartStyle, Renderer, Style, StyleRefinement, TextPartStyle},
    prelude::*,
};

use crate::{
    SelectEntry, SelectItem,
    components::{button::BUTTON_GHOST_HOVER, input},
    foundation::{
        field::{
            self, INPUT_FOREGROUND, INPUT_PLACEHOLDER_FOREGROUND, INPUT_RADIUS, Kind, field_focus,
            field_rest,
        },
        icon::{IconName, icon_size_token, icon_view, provide_text_color},
        labels::{self, Label},
        menu::{ListLook, list_look},
        popup::{self, POPOVER_MARGIN, POPOVER_OFFSET},
        size::{Size, pointer},
        typography::{line_height, medium},
    },
    theme::{
        Theme,
        seeds::SPACING,
        semantic::{
            FOREGROUND, MUTED, MUTED_FOREGROUND, OPACITY_DISABLED, RADIUS_SM, TEXT_SM, TEXT_XS,
            TOUCH_TARGET,
        },
        theme,
    },
    tokens,
};

tokens! {
    /// The chevron and the cross.
    COMBOBOX_ICON_FOREGROUND: Color = "combobox-icon-foreground" => |t| t.color(MUTED_FOREGROUND),
    /// The list's greatest height.
    COMBOBOX_LIST_MAX_HEIGHT: Length = "combobox-list-max-height" => |t| t.length(SPACING) * 87.0,
    /// The words of an empty list.
    COMBOBOX_EMPTY_FOREGROUND: Color = "combobox-empty-foreground" => |t| t.color(MUTED_FOREGROUND),
    /// A chip's fill.
    COMBOBOX_CHIP: Color = "combobox-chip" => |t| t.color(MUTED),
    /// A chip's text.
    COMBOBOX_CHIP_FOREGROUND: Color = "combobox-chip-foreground" => |t| t.color(FOREGROUND),
    /// A chip's height.
    COMBOBOX_CHIP_HEIGHT: Length = "combobox-chip-height" => |t| t.length(SPACING) * 5.5,
}

/// The props of [`Combobox`].
#[derive(Debug, Clone, PartialEq, Props)]
pub struct ComboboxProps {
    /// The options, in order: a value and a title each, optionally
    /// disabled or in a group (adjacent items of one `group` show under a
    /// header with its title).
    #[prop(default)]
    pub items: Prop<Vec<SelectItem>>,
    /// Several values, shown as chips: a choice adds or removes its item
    /// and the list stays open. Read when it is made.
    #[prop(default)]
    pub multiple: Prop<bool>,
    /// The chosen values (controlled), one at most without `multiple`: the
    /// app sets them in `onChange`. Left out, the combobox keeps its own,
    /// from `defaultValue`.
    pub value: Prop<Option<Vec<SharedString>>>,
    /// The first value while `value` is left out.
    #[prop(default)]
    pub default_value: Prop<Vec<SharedString>>,
    /// What is typed (controlled), for an app that filters `items` itself:
    /// it sets it in `onQueryChange`.
    pub query: Prop<Option<SharedString>>,
    /// The query each opening starts with.
    pub default_query: Prop<Option<SharedString>>,
    /// What the input shows while it is empty.
    pub placeholder: Prop<Option<SharedString>>,
    /// Its size (`xs`, `sm`, `md`, `lg`): the field's height, text, padding
    /// and icons, as an `Input` of that size, and the rows'.
    #[prop(default)]
    pub size: Prop<Size>,
    /// It does not open, takes no focus, and is dimmed. Or'ed with an
    /// enclosing `Field`'s or `FieldSet`'s.
    #[prop(default)]
    pub disabled: Prop<bool>,
    /// The app's checks reject the value: a red edge (and ring while
    /// focused). Or'ed with an enclosing `Field`'s.
    #[prop(default)]
    pub invalid: Prop<bool>,
    /// A button that clears the value, while one is set.
    #[prop(default)]
    pub cleanable: Prop<bool>,
    /// The chevron button at the end, which opens and closes the list
    /// (hidden while the clear button shows). Read when it is made.
    #[prop(default = true)]
    pub show_trigger: Prop<bool>,
    /// Open (controlled): the app sets it in `onOpenChange`. Left out, the
    /// combobox keeps its own state.
    pub open: Prop<Option<bool>>,
    /// Open at first, while `open` is left out.
    #[prop(default)]
    pub default_open: Prop<bool>,
    /// Desktops: the side of the field the list opens on (`"bottom"` by
    /// default); the other one when only it has room.
    #[prop(default)]
    pub placement: Prop<Placement>,
    /// Desktops: how the list lines up with the field (`"start"` by
    /// default).
    #[prop(default = Align::Start)]
    pub align: Prop<Align>,
    /// A row's content in place of its title: the library still draws the
    /// row and the check. It gets what the item is, never its state.
    pub render_item: Prop<Option<Renderer<SelectEntry>>>,
    /// What the list shows while no item does; left out, "No items found."
    /// (`labels`, translatable).
    pub empty: Prop<Option<Slot>>,
    /// What screen readers call it (what the choice is about).
    pub accessibility_label: Prop<Option<SharedString>>,
    /// Tailwind classes of the text input.
    pub input_class: Prop<Option<SharedString>>,
    /// The inline style of the text input, over the library's look.
    #[prop(default)]
    pub input_style: Prop<InputPartStyle>,
    /// Tailwind classes of the chevron and the cross.
    pub icon_class: Prop<Option<SharedString>>,
    /// The inline style of the chevron and the cross.
    #[prop(default)]
    pub icon_style: Prop<Style>,
    /// Tailwind classes of each chip (`multiple`).
    pub chip_class: Prop<Option<SharedString>>,
    /// The inline style of each chip, over the library's look.
    #[prop(default)]
    pub chip_style: Prop<Style>,
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

/// The events of a [`Combobox`].
#[derive(Debug, Clone, PartialEq, Event)]
pub enum ComboboxEvent {
    /// The value changed (a choice, a toggle, the clear button, a chip's
    /// remove button): the chosen values, which the app makes `value`.
    #[args(value)]
    Change(Vec<SharedString>),
    /// The list closed: the value then, once per opening.
    #[args(value)]
    Confirm(Vec<SharedString>),
    /// The user typed: the query, which the app makes `query` when it sets
    /// one.
    #[args(query)]
    QueryChange(SharedString),
    /// The user asks to open or close the list, with why: set `open` to
    /// follow it.
    #[args(open, reason)]
    OpenChange(bool, OpenChangeReason),
    /// The list is shown (a sheet once it came in).
    Open,
    /// The list is gone, after its exit (whoever closed it).
    Close,
    /// The user closed the list without a choice that closes it (Escape,
    /// back, a press outside).
    Cancel,
}

/// A combobox: a text field that filters a list as the user types and
/// chooses one value, or several shown as chips (`multiple`). The list
/// opens as a popup under the field on desktops and as a sheet at the
/// bottom on touch screens; its options are data (`items`), so a list of
/// thousands makes only the rows in view.
#[component]
pub fn Combobox(props: ComboboxProps, cx: Scope<ComboboxEvent>) -> impl IntoElement {
    provide_text_color();
    let t = theme();
    let size = move || props.size.get();
    // `nsh/base` gives no handle on its input: a label's press reaches
    // nothing (the docs' NSH gaps).
    let joined = field::join(cx.__host(), Kind::Inert, move || props.disabled.get());
    let disabled = computed(move || props.disabled.get() || joined.disabled());
    let invalid = move || props.invalid.get() || joined.invalid();
    let multiple = props.multiple.get_untracked();
    // The value the chips and the checks show: the app's, else its own.
    let own = signal(props.default_value.get_untracked());
    let current = computed(move || props.value.get().unwrap_or_else(|| own.get()));
    let report = move |next: Vec<SharedString>| {
        if props.value.get_untracked().is_none() {
            own.set(next.clone());
        }
        cx.emit(ComboboxEvent::Change(next));
    };
    // Open, from the request to the end of the exit, the list counts among
    // the open popups (as Select's).
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

    let mut root = base_combobox()
        .items(props.items)
        .multiple(multiple)
        .value(move || Some(current.get()))
        .query(props.query)
        .default_query(props.default_query)
        .open(props.open)
        .default_open(props.default_open)
        .placeholder(props.placeholder)
        .cleanable(props.cleanable)
        .disabled(disabled)
        .accessibility_label(props.accessibility_label)
        .placement(props.placement)
        .align(props.align)
        .offset(move || t.length(POPOVER_OFFSET))
        .margin(move || t.length(POPOVER_MARGIN))
        .clear_button_accessibility_label(move || Some(labels::text(Label::Clear)))
        .on_change(move |value: &Vec<SharedString>| report(value.clone()))
        .on_confirm(move |value: &Vec<SharedString>| cx.emit(ComboboxEvent::Confirm(value.clone())))
        .on_query_change(move |query: &SharedString| {
            cx.emit(ComboboxEvent::QueryChange(query.clone()))
        })
        .on_open_change(move |e: &BaseComboboxEvent| {
            if let BaseComboboxEvent::OpenChange(open, reason) = e {
                if props.open.get_untracked().is_none() {
                    own_open.set(*open);
                }
                cx.emit(ComboboxEvent::OpenChange(*open, *reason));
            }
        })
        .on_open(move || cx.emit(ComboboxEvent::Open))
        .on_close(move || {
            if !untracked(is_open) {
                held.set(false);
            }
            cx.emit(ComboboxEvent::Close)
        })
        .on_cancel(move || cx.emit(ComboboxEvent::Cancel))
        // As wide as its parent, as Input.
        .app_class(Some(SharedString::from("w-full min-w-0")));
    if is_terminal() {
        // `nsh/base`'s terminal field and rows.
        root.__element().__props_mut().render_item = props.render_item;
        return root;
    }

    let chips = computed(move || multiple && current.with(|v| !v.is_empty()));
    let clear_shown = move || props.cleanable.get() && current.with(|v| !v.is_empty());
    let trigger_shown = move || props.show_trigger.get() && !clear_shown();
    let height = move || t.length(input::height_token(size()));
    let font_size = move || t.length(input::font_size_token(size()));
    let gap = move || t.length(SPACING) * 1.5;
    let chip_height = move || t.length(COMBOBOX_CHIP_HEIGHT);
    root = root
        .items_center()
        .min_h(height)
        // Room at the end for a button (inset 4), and around the chips.
        .pe(move || {
            if clear_shown() || trigger_shown() {
                px(4.0)
            } else {
                px(0.0)
            }
        })
        .ps(move || if chips.get() { gap() } else { px(0.0) })
        // Beside chips, the field grows as they wrap: its padding keeps the
        // buttons' inset of 4 above and below.
        .py(move || {
            if chips.get() {
                (px(4.0) - t.length(field::INPUT_BORDER_WIDTH)).max(px(0.0))
            } else {
                px(0.0)
            }
        })
        .cursor(Cursor::Default)
        .opacity(move || {
            if disabled.get() {
                t.number(OPACITY_DISABLED)
            } else {
                1.0
            }
        });
    // The field look, its focus while the input inside has it.
    root = field_rest(root, t, invalid).focus_within(move |s| field_focus(s, t, invalid));
    root = crate::foundation::motion::transition_all(root, t);
    if props.show_trigger.get_untracked() {
        root = root.trigger_button(
            button(
                t,
                size,
                IconName::ChevronDown,
                props.icon_class,
                props.icon_style,
            )
            .style_when(move || !trigger_shown(), |s| s.hidden()),
        );
    }
    root = root.clear_button(
        icon_view(t, || IconName::X, move || t.length(icon_size_token(size())))
            .text_color(move || t.color(COMBOBOX_ICON_FOREGROUND))
            .app_class(props.icon_class)
            .inline_style(move || props.icon_style.get()),
    );
    if multiple {
        root = root.prefix(chip_row(props.clone(), t, current, chips, report, disabled));
    }
    let empty_words = view()
        .py(move || t.length(SPACING) * 2.0)
        .items_center()
        .child(
            text(move || labels::text(Label::NoItems))
                .text_size(move || t.length(TEXT_SM))
                .line_height(move || line_height(t.length(TEXT_SM)))
                .text_color(move || t.color(COMBOBOX_EMPTY_FOREGROUND)),
        );
    root = match props.empty.get_untracked() {
        Some(_) => root.empty(
            view()
                .py(move || t.length(SPACING) * 2.0)
                .items_center()
                .child(props.empty.place(|e| e)),
        ),
        None => root.empty(empty_words),
    };
    let input_style = move || {
        let s = StyleRefinement::default().flex_1();
        // Beside chips, a chip's height (the chips set the field's); else
        // the field's.
        let s = if chips.get() {
            s.h(chip_height())
        } else {
            s.self_stretch()
        };
        let s = s
            .min_w(if multiple {
                t.length(SPACING) * 16.0
            } else {
                px(0.0)
            })
            .ms(if chips.get() { gap() } else { px(0.0) })
            .ps(if chips.get() {
                px(0.0)
            } else {
                t.length(input::padding_token(size()))
            })
            .pe(if clear_shown() || trigger_shown() {
                px(0.0)
            } else {
                t.length(input::padding_token(size()))
            })
            .text_size(font_size())
            .line_height(line_height(font_size()))
            .text_color(t.color(INPUT_FOREGROUND))
            .placeholder_color(t.color(INPUT_PLACEHOLDER_FOREGROUND));
        InputPartStyle::from(Style::from(s).then(&Style::from(props.input_style.get())))
    };
    let clear_style = move || Style::from(button_frame(StyleRefinement::default(), t, size));
    {
        let parts = root.__element().__props_mut();
        parts.input_class = props.input_class;
        parts.input_style = Prop::from(IntoReactive::<InputPartStyle>::into_reactive(input_style));
        parts.clear_button_style = Prop::from(IntoReactive::<Style>::into_reactive(clear_style));
        parts.clear_button_class = Prop::from(IntoReactive::<Option<SharedString>>::into_reactive(
            move || Some(SharedString::from(hover_class(t))),
        ));
    }
    list_look!(
        root,
        t,
        ListLook {
            size: props.size,
            chosen: Rc::new(move |value: &SharedString| current.with(|c| c.contains(value))),
            render_item: props.render_item,
            max_height: Some(COMBOBOX_LIST_MAX_HEIGHT),
            content_class: props.content_class,
            content_style: props.content_style,
            item_class: props.item_class,
            item_style: props.item_style,
            label_class: props.label_class,
            label_style: props.label_style,
        }
    );
    root
}

/// A square button's frame inside a field of `size`: the field's height
/// less 8, its corners concentric with the field's.
fn button_frame(
    s: StyleRefinement,
    t: Theme,
    size: impl Fn() -> Size + Copy + 'static,
) -> StyleRefinement {
    let side = button_side(t, size());
    let radius = (t.length(INPUT_RADIUS) - px(4.0)).max(px(0.0));
    s.size(side)
        .flex_shrink_0()
        .items_center()
        .justify_center()
        .rounded(radius)
}

/// A button's side in a field of `size`.
fn button_side(t: Theme, size: Size) -> Points {
    t.length(input::height_token(size)) - px(8.0)
}

/// Above and below a control `side` tall, so a finger gets `touch-target`
/// (44) points; none with a pointer. Not at the sides: the input is beside
/// it.
fn touch_slop(t: Theme, side: Points) -> EdgeInsets {
    if pointer() {
        return EdgeInsets::default();
    }
    let slop = ((t.length(TOUCH_TARGET) - side) / 2.0).max(px(0.0));
    EdgeInsets {
        top: slop,
        bottom: slop,
        ..EdgeInsets::default()
    }
}

/// The ghost button's hover fill (none pressed), as a class made from the
/// tokens, for a part only `nsh/base` knows the states of.
fn hover_class(t: Theme) -> String {
    let fill = t.color(BUTTON_GHOST_HOVER);
    format!(
        "hover:bg-[{}] dark:hover:bg-[{}] active:bg-transparent",
        fill.light.to_hex(),
        fill.dark.to_hex()
    )
}

/// The chevron button: a square of the field with the ghost button's
/// hover, holding the icon.
fn button(
    t: Theme,
    size: impl Fn() -> Size + Copy + 'static,
    name: IconName,
    icon_class: Prop<Option<SharedString>>,
    icon_style: Prop<Style>,
) -> View {
    view()
        .style_with(move |s| button_frame(s, t, size))
        .hit_slop(move || touch_slop(t, button_side(t, size())))
        .hover(move |s| s.bg(move || t.color(BUTTON_GHOST_HOVER)))
        .active(|s| s.bg(AdaptiveColor::TRANSPARENT))
        .child(
            icon_view(t, move || name, move || t.length(icon_size_token(size())))
                .text_color(move || t.color(COMBOBOX_ICON_FOREGROUND))
                .app_class(icon_class)
                .inline_style(move || icon_style.get()),
        )
}

/// The chips of several values, before the input: a row that wraps within
/// the room the input leaves, one chip per value with a remove button.
fn chip_row(
    props: ComboboxProps,
    t: Theme,
    current: Memo<Vec<SharedString>>,
    shown: Memo<bool>,
    report: impl Fn(Vec<SharedString>) + Copy + 'static,
    disabled: Memo<bool>,
) -> impl IntoElement {
    let title_of = move |value: &SharedString| {
        props.items.with(|items| {
            items
                .iter()
                .find(|i| i.value == *value)
                .map(|i| i.title.clone())
                .unwrap_or_else(|| value.clone())
        })
    };
    let chip_height = move || t.length(COMBOBOX_CHIP_HEIGHT);
    let gap = move || t.length(SPACING) * 1.5;
    view()
        .flex_row()
        .flex_wrap()
        .flex_shrink(1.0)
        .min_w(px(0.0))
        .gap(gap)
        .style_when(move || !shown.get(), |s| s.hidden())
        .child(each(
            move || current.get(),
            |value: &SharedString| value.clone(),
            move |value: ReadSignal<SharedString>, _| {
                let title = computed(move || title_of(&value.get()));
                let remove = {
                    move || {
                        let value = value.get_untracked();
                        if disabled.get_untracked() {
                            return;
                        }
                        let next: Vec<SharedString> = current
                            .get_untracked()
                            .into_iter()
                            .filter(|v| *v != value)
                            .collect();
                        report(next);
                    }
                };
                view()
                    .flex_row()
                    .items_center()
                    .min_w(px(0.0))
                    .h(chip_height)
                    .ps(gap)
                    .rounded(move || t.length(RADIUS_SM))
                    .bg(move || t.color(COMBOBOX_CHIP))
                    .app_class(props.chip_class)
                    .inline_style(move || props.chip_style.get())
                    .child(
                        text(move || title.get())
                            .truncate()
                            .text_size(move || t.length(TEXT_XS))
                            .line_height(move || line_height(t.length(TEXT_XS)))
                            .font_weight(medium(t))
                            .text_color(move || t.color(COMBOBOX_CHIP_FOREGROUND)),
                    )
                    .child(
                        view()
                            .role(AccessibilityRole::Button)
                            .label(move || {
                                SharedString::from(format!(
                                    "{} {}",
                                    labels::text(Label::Remove),
                                    title.get()
                                ))
                            })
                            .focusable(false)
                            .disabled(disabled)
                            .size(chip_height)
                            .flex_shrink_0()
                            .items_center()
                            .justify_center()
                            .hit_slop(move || touch_slop(t, chip_height()))
                            .opacity(0.5)
                            .hover(|s| s.opacity(1.0))
                            .on_press(remove)
                            .child(
                                icon_view(t, || IconName::X, move || t.length(TEXT_XS))
                                    .text_color(move || t.color(COMBOBOX_CHIP_FOREGROUND)),
                            ),
                    )
            },
        ))
}
