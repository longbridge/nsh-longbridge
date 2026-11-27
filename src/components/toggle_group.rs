//! `ToggleGroup` and `ToggleGroupItem`: a row of toggles holding one value
//! or several, on `nsh/base`'s `ToggleGroup` and `Toggle` (one stop of the
//! keyboard focus whose arrows move between the toggles, the toolbar and
//! toggle button roles, the disabled state). The group holds the value,
//! which `nsh/base`'s does not; its items take the toggle look. API:
//! `docs/components/toggle-group.md`.

use nsh_base::toggle_group as base_toggle_group;
use nsh_core::{element::TextPartStyle, prelude::*};

use super::toggle::{
    ItemLook, TOGGLE_OUTLINE_SHADOW, TOGGLE_RADIUS, ToggleProps, ToggleVariant, draw,
};
use crate::{
    foundation::{
        field::{self, Kind},
        shadow,
        size::Size,
    },
    theme::{seeds::SPACING, theme},
    tokens,
};

tokens! {
    /// An item's horizontal padding, at every size.
    TOGGLE_GROUP_ITEM_PADDING_X: Length = "toggle-group-item-padding-x" => |t| t.length(SPACING) * 3.0,
}

/// What an item finds of its group.
#[derive(Clone, Copy)]
struct GroupContext {
    value: Memo<Vec<SharedString>>,
    set: Stored<Box<dyn Fn(Vec<SharedString>)>>,
    multiple: bool,
    variant: Prop<ToggleVariant>,
    size: Prop<Size>,
    disabled: Memo<bool>,
    look: ItemLook,
}

/// The props of [`ToggleGroup`].
#[derive(Debug, Clone, PartialEq, Props)]
pub struct ToggleGroupProps {
    /// Several values; without it, one at most. Read when it is made.
    #[prop(default)]
    pub multiple: Prop<bool>,
    /// The pressed items' values (controlled): one at most without
    /// `multiple`. The app sets it in `onChange`. Left out, the group keeps
    /// its own, from `defaultValue`.
    pub value: Prop<Option<Vec<SharedString>>>,
    /// The first value while `value` is left out.
    #[prop(default)]
    pub default_value: Prop<Vec<SharedString>>,
    /// The items' look (`default` or `outline`).
    #[prop(default)]
    pub variant: Prop<ToggleVariant>,
    /// The items' size (`xs`, `sm`, `md`, `lg`).
    #[prop(default)]
    pub size: Prop<Size>,
    /// The gap between the items, in `spacing` units (2 is 8); 0 joins them
    /// into one shape.
    #[prop(default)]
    pub spacing: Prop<f32>,
    /// Every item disabled. Or'ed with an enclosing `Field`'s or
    /// `FieldSet`'s.
    #[prop(default)]
    pub disabled: Prop<bool>,
    /// What screen readers call the set ("Text alignment"), said as they
    /// enter it.
    pub accessibility_label: Prop<Option<SharedString>>,
    /// The items.
    pub children: Prop<Children>,
}

/// The events of a [`ToggleGroup`].
#[derive(Debug, Clone, PartialEq, Event)]
pub enum ToggleGroupEvent {
    /// The user pressed an item and the value changed: the new value. A
    /// single group asks for `[]` when its pressed item is pressed again.
    #[args(value)]
    Change(Vec<SharedString>),
}

/// A toggle group: a row of toggles of one look and size, holding one value
/// (an alignment, a view, a segmented control) or several (`multiple`:
/// bold, italic, underline). Its items touch as one shape (`spacing` 0) or
/// stand apart. It is one stop of the keyboard focus; the arrow keys move
/// between its items without pressing them.
#[component]
pub fn ToggleGroup(props: ToggleGroupProps, cx: Scope<ToggleGroupEvent>) -> impl IntoElement {
    let t = theme();
    let joined_field = field::join(cx.__host(), Kind::Inert, move || props.disabled.get());
    let disabled = computed(move || props.disabled.get() || joined_field.disabled());
    let own = signal(props.default_value.get_untracked());
    let value = computed(move || props.value.get().unwrap_or_else(|| own.get()));
    let set: Box<dyn Fn(Vec<SharedString>)> = Box::new(move |next: Vec<SharedString>| {
        if untracked(|| value.get()) == next {
            return;
        }
        if props.value.get_untracked().is_none() {
            own.set(next.clone());
        }
        cx.emit(ToggleGroupEvent::Change(next));
    });
    let set = stored(set);
    let joined = computed(move || props.spacing.get() <= 0.0);
    let gap = computed(move || t.length(SPACING) * props.spacing.get().max(0.0));
    let outline = move || props.variant.get() == ToggleVariant::Outline;
    provide_context(GroupContext {
        value,
        set,
        multiple: props.multiple.get_untracked(),
        variant: props.variant,
        size: props.size,
        disabled,
        look: ItemLook {
            joined,
            gap,
            padding_x: TOGGLE_GROUP_ITEM_PADDING_X,
        },
    });
    let label = props.accessibility_label;
    // `nsh/base`'s toolbar takes no name: the root is a group around it
    // that carries the set's name, and the shape (a joined outline row's
    // corners and `xs` shadow), which the app's `class` and `style` then
    // override.
    view()
        .role(move || {
            if label.with(Option::is_some) {
                AccessibilityRole::Group
            } else {
                AccessibilityRole::None
            }
        })
        .optional_label(label)
        .accessibility_described_by_target(move || joined_field.described_by())
        .flex_row()
        .rounded(move || {
            if joined.get() {
                t.length(TOGGLE_RADIUS)
            } else {
                px(0.0)
            }
        })
        .inline_style(move || {
            let level = if joined.get() && outline() {
                shadow::level(t.number(TOGGLE_OUTLINE_SHADOW))
            } else {
                0
            };
            shadow::shadow(t, level)
        })
        .app_class(Some(SharedString::from("w-fit")))
        .child(
            base_toggle_group()
                .disabled(disabled)
                .flex_1()
                .flex_row()
                .items_center()
                .gap(move || gap.get())
                .child(props.children),
        )
}

/// The props of [`ToggleGroupItem`].
#[derive(Debug, Clone, PartialEq, Props)]
pub struct ToggleGroupItemProps {
    /// Its value, unique in its group: pressed while the group's value holds
    /// it.
    #[prop(default)]
    pub value: Prop<SharedString>,
    /// This item disabled (or'ed with its group's).
    #[prop(default)]
    pub disabled: Prop<bool>,
    /// What screen readers call it: required when its content is an icon
    /// alone.
    pub accessibility_label: Prop<Option<SharedString>>,
    /// Tailwind classes of the label parts (one around each child).
    pub label_class: Prop<Option<SharedString>>,
    /// The inline style of the label parts.
    #[prop(default)]
    pub label_style: Prop<TextPartStyle>,
    /// The content: an icon (the app's), a word, or both.
    pub children: Prop<Children>,
}

/// One option of a [`ToggleGroup`]: a toggle of the group's look and size,
/// pressed while the group's value holds its `value`. A press sets the
/// group's value: without `multiple` to this item alone (or none, when it
/// was pressed), with `multiple` adding or removing it. Outside a
/// `ToggleGroup` it is never pressed and reports nothing.
#[component]
pub fn ToggleGroupItem(props: ToggleGroupItemProps, _cx: Scope<NoEvent>) -> impl IntoElement {
    let group = use_context::<GroupContext>();
    if group.is_none() {
        #[cfg(debug_assertions)]
        log::warn!("a `ToggleGroupItem` is outside a `ToggleGroup`: it is never pressed");
    }
    let mine = props.value;
    let pressed =
        computed(move || group.is_some_and(|g| g.value.with(|v| v.contains(&mine.get()))));
    let disabled =
        computed(move || props.disabled.get() || group.is_some_and(|g| g.disabled.get()));
    let variant = computed(move || group.map(|g| g.variant.get()).unwrap_or_default());
    let size = computed(move || group.map(|g| g.size.get()).unwrap_or_default());
    let toggle = ToggleProps {
        pressed: Prop::from(IntoReactive::<bool>::into_reactive(pressed)),
        variant: Prop::from(IntoReactive::<ToggleVariant>::into_reactive(variant)),
        size: Prop::from(IntoReactive::<Size>::into_reactive(size)),
        disabled: Prop::from(IntoReactive::<bool>::into_reactive(disabled)),
        accessibility_label: props.accessibility_label,
        hit_slop: Prop::from(IntoReactive::<Option<EdgeInsets>>::into_reactive(None)),
        label_class: props.label_class,
        label_style: props.label_style,
        children: props.children,
    };
    draw(toggle, group.map(|g| g.look), move |on: bool| {
        let Some(g) = group else { return };
        let value = mine.get_untracked();
        let current = untracked(|| g.value.get());
        let next = if g.multiple {
            let mut next: Vec<SharedString> = current.into_iter().filter(|v| *v != value).collect();
            if on {
                next.push(value);
            }
            next
        } else if on {
            vec![value]
        } else {
            Vec::new()
        };
        g.set.with(|set| set(next));
    })
}
