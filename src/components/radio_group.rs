//! `RadioGroup` and `RadioGroupItem`: shadcn/ui's radio group on
//! `nsh/base`'s `RadioGroup` and `Radio` (the keyboard's roving focus and
//! arrows, presses on the circle and its label, the radiogroup and radio
//! roles and their states for screen readers). The group holds the value,
//! which `nsh/base`'s does not. API: `docs/components/radio-group.md`.

use nsh_base::{radio as base_radio, radio_group as base_radio_group, terminal::is_terminal};
use nsh_core::{
    element::{Style, TextPartStyle},
    prelude::*,
};

use crate::{
    components::label::{LABEL_FONT_WEIGHT, LABEL_FOREGROUND},
    foundation::{
        choice::{choice_hit_slop, choice_label, choice_offset},
        focus::focus_ring,
        icon::provide_text_color,
        motion,
        press::{SMALL_SCALE, press},
        shadow,
        size::Size,
        typography::font_weight,
    },
    theme::{
        TRANSPARENT, Theme, Token, kind,
        seeds::SPACING,
        semantic::{
            BORDER_WIDTH, CONTROL_HEIGHT_LG, CONTROL_HEIGHT_MD, CONTROL_HEIGHT_SM,
            CONTROL_HEIGHT_TOUCH_LG, CONTROL_HEIGHT_TOUCH_MD, CONTROL_HEIGHT_TOUCH_SM,
            CONTROL_HEIGHT_TOUCH_XS, CONTROL_HEIGHT_XS, DESTRUCTIVE, INPUT, OPACITY_DISABLED,
            PRIMARY, RING, RING_INVALID, RING_OPACITY,
        },
        theme,
    },
    tokens,
};

/// The group an item's circle takes the item's focus and press from.
const GROUP: &str = "nlb-radio";

tokens! {
    /// The space between the items (shadcn's `gap-3`).
    RADIO_GROUP_GAP: Length = "radio-group-gap" => |t| t.length(SPACING) * 3.0,
    /// An extra small item's circle: half the control height.
    RADIO_GROUP_ITEM_SIZE_XS: Length = "radio-group-item-size-xs" => |t| t.pointer_touch(t.length(CONTROL_HEIGHT_XS) * 0.5, t.length(CONTROL_HEIGHT_TOUCH_XS) * 0.5),
    /// A small item's circle.
    RADIO_GROUP_ITEM_SIZE_SM: Length = "radio-group-item-size-sm" => |t| t.pointer_touch(t.length(CONTROL_HEIGHT_SM) * 0.5, t.length(CONTROL_HEIGHT_TOUCH_SM) * 0.5),
    /// A medium item's circle (16, shadcn's `size-4`).
    RADIO_GROUP_ITEM_SIZE_MD: Length = "radio-group-item-size-md" => |t| t.pointer_touch(t.length(CONTROL_HEIGHT_MD) * 0.5, t.length(CONTROL_HEIGHT_TOUCH_MD) * 0.5),
    /// A large item's circle.
    RADIO_GROUP_ITEM_SIZE_LG: Length = "radio-group-item-size-lg" => |t| t.pointer_touch(t.length(CONTROL_HEIGHT_LG) * 0.5, t.length(CONTROL_HEIGHT_TOUCH_LG) * 0.5),
    /// The space between an item's circle and its label.
    RADIO_GROUP_ITEM_GAP: Length = "radio-group-item-gap" => |t| t.length(SPACING) * 2.0,
    /// The circle's fill: transparent, `input` at 30 % in the dark.
    RADIO_GROUP_ITEM: Color = "radio-group-item" => |t| t.light_dark(TRANSPARENT, t.color(INPUT).opacity(0.3)),
    /// The circle's edge.
    RADIO_GROUP_ITEM_BORDER: Color = "radio-group-item-border" => |t| t.color(INPUT),
    /// The circle's edge while checked: the unchecked one (shadcn's).
    RADIO_GROUP_ITEM_CHECKED_BORDER: Color = "radio-group-item-checked-border" => |t| t.color(RADIO_GROUP_ITEM_BORDER),
    /// The circle's edge while focused.
    RADIO_GROUP_ITEM_BORDER_FOCUS: Color = "radio-group-item-border-focus" => |t| t.color(RING),
    /// The circle's edge while the group is `invalid`, checked or not.
    RADIO_GROUP_ITEM_BORDER_INVALID: Color = "radio-group-item-border-invalid" => |t| t.color(DESTRUCTIVE),
    /// The focus ring around the circle.
    RADIO_GROUP_ITEM_RING: Color = "radio-group-item-ring" => |t| t.color(RING).opacity(t.number(RING_OPACITY)),
    /// The focus ring while the group is `invalid`.
    RADIO_GROUP_ITEM_RING_INVALID: Color = "radio-group-item-ring-invalid" => |t| t.color(RING_INVALID),
    /// The dot of the checked item.
    RADIO_GROUP_ITEM_DOT: Color = "radio-group-item-dot" => |t| t.color(PRIMARY),
    /// The circle's shadow level (1: `xs`; 0 for none).
    RADIO_GROUP_ITEM_SHADOW: Number = "radio-group-item-shadow" => |t| t.value(1.0),
}

/// What an item finds of its group.
#[derive(Clone, Copy)]
struct GroupContext {
    value: Prop<Option<SharedString>>,
    disabled: Prop<bool>,
    invalid: Prop<bool>,
    size: Prop<Size>,
    cx: Scope<RadioGroupEvent>,
}

/// The props of [`RadioGroup`].
#[derive(Debug, Clone, PartialEq, Props)]
pub struct RadioGroupProps {
    /// The value of the checked item; none is checked when it is left out or
    /// matches no item. The app owns it: a press reports the item's value
    /// with `onChange`.
    pub value: Prop<Option<SharedString>>,
    /// Disables every item (with each item's own `disabled`); the arrows
    /// skip disabled items.
    #[prop(default)]
    pub disabled: Prop<bool>,
    /// The app's checks reject the choice (a required choice left empty):
    /// every item's circle takes a destructive edge. Not announced.
    #[prop(default)]
    pub invalid: Prop<bool>,
    /// Every item's size (`xs`, `sm`, `md`, `lg`): its circle and its
    /// label's text.
    #[prop(default)]
    pub size: Prop<Size>,
    /// What screen readers call the set ("Plan", "Sort by"), said as they
    /// enter it. Give it always.
    pub accessibility_label: Prop<Option<SharedString>>,
    /// The items, and whatever the app lays out between them.
    pub children: Prop<Children>,
}

/// The events of a [`RadioGroup`].
#[derive(Debug, Clone, PartialEq, Event)]
pub enum RadioGroupEvent {
    /// An unchecked item was pressed, or an arrow key moved the keyboard
    /// focus to it: its value, which the app makes the group's.
    #[args(value)]
    Change(SharedString),
}

/// A radio group, shadcn/ui's: exactly one of a small set of options, all
/// visible. It holds the value and the size, disabled and invalid states
/// its [`RadioGroupItem`]s follow. It is one stop of the keyboard focus;
/// the arrow keys move between its items and check them. A column by
/// default; `class="flex-row"` makes a row.
#[component]
pub fn RadioGroup(props: RadioGroupProps, cx: Scope<RadioGroupEvent>) -> impl IntoElement {
    let t = theme();
    provide_context(GroupContext {
        value: props.value,
        disabled: props.disabled,
        invalid: props.invalid,
        size: props.size,
        cx,
    });
    base_radio_group()
        .accessibility_label(props.accessibility_label)
        .flex_col()
        .gap(move || t.length(RADIO_GROUP_GAP))
        .child(props.children)
}

/// The props of [`RadioGroupItem`].
#[derive(Debug, Clone, PartialEq, Props)]
pub struct RadioGroupItemProps {
    /// The value it stands for, unique in its group: it is checked while it
    /// is the group's `value`.
    #[prop(default)]
    pub value: Prop<SharedString>,
    /// No presses and no press feedback, the circle and its label at half
    /// opacity; the arrows skip it.
    #[prop(default)]
    pub disabled: Prop<bool>,
    /// What screen readers say, when the children have no text that says
    /// the option.
    pub accessibility_label: Prop<Option<SharedString>>,
    /// How far outside its bounds presses still count. Left out: on touch
    /// screens a circle without a label takes presses up to 44 points
    /// around it; nothing otherwise.
    pub hit_slop: Prop<Option<EdgeInsets>>,
    /// Tailwind classes of the circle.
    pub indicator_class: Prop<Option<SharedString>>,
    /// The inline style of the circle.
    #[prop(default)]
    pub indicator_style: Prop<Style>,
    /// Tailwind classes of the dot inside the circle of the checked item.
    pub dot_class: Prop<Option<SharedString>>,
    /// The inline style of the dot.
    #[prop(default)]
    pub dot_style: Prop<Style>,
    /// Tailwind classes of the label: the part the children sit in (their
    /// text takes its text classes).
    pub label_class: Prop<Option<SharedString>>,
    /// The inline style of the label part.
    #[prop(default)]
    pub label_style: Prop<TextPartStyle>,
    /// The label: a string, or anything else. A press on it checks the
    /// item.
    pub children: Prop<Children>,
}

/// One option of a [`RadioGroup`]: a circle, with a dot while it is the
/// group's value, and its label beside it. A press on the label checks it
/// too. The keyboard focus draws a ring around the circle; on touch screens
/// it is larger, and without a label it takes presses up to 44 points
/// around it. Outside a `RadioGroup` it is never checked and reports
/// nothing.
#[component]
pub fn RadioGroupItem(props: RadioGroupItemProps, _cx: Scope<NoEvent>) -> impl IntoElement {
    provide_text_color();
    let t = theme();
    let group = use_context::<GroupContext>();
    if group.is_none() {
        #[cfg(debug_assertions)]
        log::warn!("a `RadioGroupItem` is outside a `RadioGroup`: it is never checked");
    }
    let size = move || group.map(|g| g.size.get()).unwrap_or_default();
    let side = move || t.length(size_token(size()));
    let checked = move || {
        group.is_some_and(|g| {
            g.value
                .get()
                .is_some_and(|value| value == props.value.get())
        })
    };
    let disabled = move || props.disabled.get() || group.is_some_and(|g| g.disabled.get());
    let invalid = move || group.is_some_and(|g| g.invalid.get());
    let labelled = move || !props.children.get().is_empty();

    let dot = move || {
        checked().then(|| {
            view()
                .w(move || side() / 2.0)
                .h(move || side() / 2.0)
                .rounded_full()
                .bg(move || t.color(RADIO_GROUP_ITEM_DOT))
                .app_class(props.dot_class)
                .inline_style(move || props.dot_style.get())
        })
    };
    let circle = view()
        .w(side)
        .h(side)
        .mt(move || choice_offset(t, labelled(), size(), side()))
        .flex_shrink_0()
        .items_center()
        .justify_center()
        .rounded_full()
        .border(move || t.length(BORDER_WIDTH))
        .border_color(move || border(t, checked(), invalid()))
        .bg(move || t.color(RADIO_GROUP_ITEM))
        .group_focus_visible_named(GROUP, move |s| {
            focus_ring(
                s,
                t,
                move || {
                    if invalid() {
                        t.color(RADIO_GROUP_ITEM_RING_INVALID)
                    } else {
                        t.color(RADIO_GROUP_ITEM_RING)
                    }
                },
                None,
            )
            .border_color(move || {
                if invalid() {
                    t.color(RADIO_GROUP_ITEM_BORDER_INVALID)
                } else {
                    t.color(RADIO_GROUP_ITEM_BORDER_FOCUS)
                }
            })
        })
        .group_active_named(GROUP, move |s| press(s, SMALL_SCALE, disabled))
        .transition()
        .app_class(props.indicator_class)
        .inline_style(move || {
            shadow::shadow(t, shadow::level(t.number(RADIO_GROUP_ITEM_SHADOW)))
                .then(&props.indicator_style.get())
        })
        .child(dynamic(dot));
    let circle = motion::timing(circle, t);

    base_radio()
        .checked(checked)
        .disabled(disabled)
        .accessibility_label(props.accessibility_label)
        .hit_slop(move || {
            props
                .hit_slop
                .get()
                .unwrap_or_else(|| choice_hit_slop(t, labelled(), side(), side()))
        })
        .on_change(move |_: &bool| {
            if let Some(g) = group {
                g.cx.emit(RadioGroupEvent::Change(props.value.get_untracked()));
            }
        })
        .group_named(GROUP)
        .flex_row()
        .items_start()
        .gap(move || t.length(RADIO_GROUP_ITEM_GAP))
        .cursor(Cursor::Default)
        .disabled_style(move |s| s.opacity(move || t.number(OPACITY_DISABLED)))
        // The terminal shows `nsh/base`'s own radio.
        .when(!is_terminal(), |s| s.child(circle))
        .child(choice_label(
            t,
            size,
            side,
            font_weight(t, LABEL_FONT_WEIGHT),
            move || t.color(LABEL_FOREGROUND),
            props.label_class,
            props.label_style,
            props.children,
        ))
}

/// The circle's side token of `size`.
pub fn size_token(size: Size) -> Token<kind::Length> {
    size.pick(
        RADIO_GROUP_ITEM_SIZE_XS,
        RADIO_GROUP_ITEM_SIZE_SM,
        RADIO_GROUP_ITEM_SIZE_MD,
        RADIO_GROUP_ITEM_SIZE_LG,
    )
}

/// The edge out of focus: invalid > checked > rest.
fn border(t: Theme, checked: bool, invalid: bool) -> AdaptiveColor {
    if invalid {
        t.color(RADIO_GROUP_ITEM_BORDER_INVALID)
    } else if checked {
        t.color(RADIO_GROUP_ITEM_CHECKED_BORDER)
    } else {
        t.color(RADIO_GROUP_ITEM_BORDER)
    }
}
