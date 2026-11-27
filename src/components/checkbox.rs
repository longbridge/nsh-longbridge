//! `Checkbox`: a checkbox on `nsh/base`'s `Checkbox` (presses on
//! the box and its label, the keyboard, the checkbox role and its checked,
//! mixed, invalid and disabled states for screen readers). API:
//! `docs/components/checkbox.md`.

use nsh_base::checkbox as base_checkbox;
use nsh_core::{
    element::{ElementTarget, Style, TextPartStyle},
    prelude::*,
};

use crate::{
    components::label::{LABEL_FONT_WEIGHT, LABEL_FOREGROUND},
    foundation::{
        choice::{choice_hit_slop, choice_label, choice_offset},
        field::{self, Joined, Kind},
        field_ref::FieldRef,
        focus::focus_ring,
        icon::{IconName, icon_view, provide_text_color},
        motion,
        press::{SMALL_SCALE, press},
        shadow,
        size::{Size, pointer},
        typography::font_weight,
    },
    theme::{
        TRANSPARENT, Theme, Token, kind,
        seeds::SPACING,
        semantic::{
            BORDER_WIDTH, CONTROL_HEIGHT_LG, CONTROL_HEIGHT_MD, CONTROL_HEIGHT_SM,
            CONTROL_HEIGHT_TOUCH_LG, CONTROL_HEIGHT_TOUCH_MD, CONTROL_HEIGHT_TOUCH_SM,
            CONTROL_HEIGHT_TOUCH_XS, CONTROL_HEIGHT_XS, DESTRUCTIVE, INPUT, OPACITY_DISABLED,
            PRIMARY, PRIMARY_FOREGROUND, RADIUS_SM, RING, RING_INVALID, RING_OPACITY,
        },
        theme,
    },
    tokens,
};

/// The group the box takes the checkbox's focus and press from.
const GROUP: &str = "nlb-checkbox";

tokens! {
    /// An extra small box's side: half the control height.
    CHECKBOX_SIZE_XS: Length = "checkbox-size-xs" => |t| t.pointer_touch(t.length(CONTROL_HEIGHT_XS) * 0.5, t.length(CONTROL_HEIGHT_TOUCH_XS) * 0.5),
    /// A small box's side.
    CHECKBOX_SIZE_SM: Length = "checkbox-size-sm" => |t| t.pointer_touch(t.length(CONTROL_HEIGHT_SM) * 0.5, t.length(CONTROL_HEIGHT_TOUCH_SM) * 0.5),
    /// A medium box's side (16, `size-4`).
    CHECKBOX_SIZE_MD: Length = "checkbox-size-md" => |t| t.pointer_touch(t.length(CONTROL_HEIGHT_MD) * 0.5, t.length(CONTROL_HEIGHT_TOUCH_MD) * 0.5),
    /// A large box's side.
    CHECKBOX_SIZE_LG: Length = "checkbox-size-lg" => |t| t.pointer_touch(t.length(CONTROL_HEIGHT_LG) * 0.5, t.length(CONTROL_HEIGHT_TOUCH_LG) * 0.5),
    /// The box's corner radius; drawn between 0 and a quarter of its side.
    CHECKBOX_RADIUS: Length = "checkbox-radius" => |t| t.length(RADIUS_SM) - 2.0,
    /// The space between the box and its label.
    CHECKBOX_GAP: Length = "checkbox-gap" => |t| t.length(SPACING) * 2.0,
    /// The box's fill while unchecked: transparent, `input` at 30 % in the
    /// dark.
    CHECKBOX: Color = "checkbox" => |t| t.light_dark(TRANSPARENT, t.color(INPUT).opacity(0.3)),
    /// The box's edge while unchecked.
    CHECKBOX_BORDER: Color = "checkbox-border" => |t| t.color(INPUT),
    /// The box's fill while checked or indeterminate.
    CHECKBOX_CHECKED: Color = "checkbox-checked" => |t| t.color(PRIMARY),
    /// The box's edge while checked or indeterminate: its fill.
    CHECKBOX_CHECKED_BORDER: Color = "checkbox-checked-border" => |t| t.color(CHECKBOX_CHECKED),
    /// The check and the dash.
    CHECKBOX_CHECKED_FOREGROUND: Color = "checkbox-checked-foreground" => |t| t.color(PRIMARY_FOREGROUND),
    /// The box's edge while focused (unchecked with a pointer; always on
    /// touch screens).
    CHECKBOX_BORDER_FOCUS: Color = "checkbox-border-focus" => |t| t.color(RING),
    /// The box's edge while `invalid` (unchecked with a pointer).
    CHECKBOX_BORDER_INVALID: Color = "checkbox-border-invalid" => |t| t.color(DESTRUCTIVE),
    /// The focus ring around the box.
    CHECKBOX_RING: Color = "checkbox-ring" => |t| t.color(RING).opacity(t.number(RING_OPACITY)),
    /// The focus ring while `invalid`.
    CHECKBOX_RING_INVALID: Color = "checkbox-ring-invalid" => |t| t.color(RING_INVALID),
    /// The box's shadow level (1: `xs`; 0 for none).
    CHECKBOX_SHADOW: Number = "checkbox-shadow" => |t| t.value(1.0),
}

/// The props of [`Checkbox`].
#[derive(Debug, Clone, PartialEq, Props)]
pub struct CheckboxProps {
    /// Checked or not. The app owns the value: a press reports the next one
    /// with `onChange`, and the checkbox shows the value passed back.
    #[prop(default)]
    pub checked: Prop<bool>,
    /// Neither checked nor unchecked (a "select all" over a partial
    /// selection): a dash, over `checked`; a press asks for checked.
    #[prop(default)]
    pub indeterminate: Prop<bool>,
    /// No presses and no press feedback, the box and its label at half
    /// opacity. Or'ed with an enclosing `Field`'s or `FieldSet`'s.
    #[prop(default)]
    pub disabled: Prop<bool>,
    /// The app's checks reject it (a required box left unchecked): a
    /// destructive edge and focus ring; screen readers say it is invalid.
    /// Or'ed with an enclosing `Field`'s.
    #[prop(default)]
    pub invalid: Prop<bool>,
    /// Its size (`xs`, `sm`, `md`, `lg`): the box and the label's text.
    #[prop(default)]
    pub size: Prop<Size>,
    /// What screen readers say, when the children have no text that says
    /// what the box selects (always for a box alone).
    pub accessibility_label: Prop<Option<SharedString>>,
    /// What checking it does, for screen readers.
    pub accessibility_hint: Prop<Option<SharedString>>,
    /// The element whose text describes it for screen readers (its error or
    /// help line), read after its name. Inside a `Field`, the field's error
    /// or description when left out.
    pub accessibility_described_by: Prop<Option<ElementTarget>>,
    /// How far outside its bounds presses still count. Left out: on touch
    /// screens a box without a label takes presses up to 44 points around
    /// it; nothing otherwise.
    pub hit_slop: Prop<Option<EdgeInsets>>,
    /// Tailwind classes of the box.
    pub indicator_class: Prop<Option<SharedString>>,
    /// The inline style of the box.
    #[prop(default)]
    pub indicator_style: Prop<Style>,
    /// Tailwind classes of the check or dash inside the box.
    pub mark_class: Prop<Option<SharedString>>,
    /// The inline style of the check or dash.
    #[prop(default)]
    pub mark_style: Prop<Style>,
    /// Tailwind classes of the label: the part the children sit in (their
    /// text takes its text classes).
    pub label_class: Prop<Option<SharedString>>,
    /// The inline style of the label part.
    #[prop(default)]
    pub label_style: Prop<TextPartStyle>,
    /// The label: a string, or anything else. A press on it checks the box.
    pub children: Prop<Children>,
}

/// The events of a [`Checkbox`].
#[derive(Debug, Clone, PartialEq, Event)]
pub enum CheckboxEvent {
    /// The checkbox was pressed (a tap, a click, Space or Return): the value
    /// it asks for, `true` from unchecked and from indeterminate, `false`
    /// from checked.
    #[args(checked)]
    Change(bool),
}

/// The methods of a [`Checkbox`]'s ref.
pub struct CheckboxApi {
    root: ElementRef,
    disabled: Prop<bool>,
    joined: Joined,
}

#[methods]
impl CheckboxApi {
    /// Takes the keyboard focus; nothing while disabled.
    pub fn focus(&self) {
        if !self.disabled.get_untracked() && !untracked(|| self.joined.disabled()) {
            self.root.focus();
        }
    }
}

/// A checkbox: a box that is checked, unchecked or
/// indeterminate (a dash), with its label beside it. Four sizes; a press on
/// the label checks it too. The keyboard focus draws a ring around the box;
/// on touch screens the box is larger, and without a label it takes presses
/// up to 44 points around it.
#[component(methods = CheckboxApi)]
pub fn Checkbox(props: CheckboxProps, cx: Scope<CheckboxEvent>) -> impl IntoElement {
    provide_text_color();
    let t = theme();
    let size = move || props.size.get();
    let side = move || t.length(size_token(size()));
    let checked = props.checked;
    let indeterminate = props.indeterminate;
    let on = move || checked.get() || indeterminate.get();
    // A label's press asks for what the box's own press does: checked from
    // unchecked and from indeterminate.
    let joined = field::join(
        cx.__host(),
        Kind::activate(
            move || {
                cx.emit(CheckboxEvent::Change(
                    indeterminate.get_untracked() || !checked.get_untracked(),
                ))
            },
            on,
        ),
        move || props.disabled.get(),
    );
    let disabled = move || props.disabled.get() || joined.disabled();
    let invalid = move || props.invalid.get() || joined.invalid();
    let root = element_ref();
    cx.methods(CheckboxApi {
        root,
        disabled: props.disabled,
        joined,
    });
    let labelled = move || !props.children.get().is_empty();

    let mark = move || {
        on().then(|| {
            let name = if indeterminate.get() {
                IconName::Minus
            } else {
                IconName::Check
            };
            icon_view(t, move || name, move || side() - px(2.0))
                .app_class(props.mark_class)
                .inline_style(move || props.mark_style.get())
        })
    };
    let indicator = view()
        .w(side)
        .h(side)
        .mt(move || choice_offset(t, labelled(), size(), side()))
        .flex_shrink_0()
        .items_center()
        .justify_center()
        .rounded(move || t.length(CHECKBOX_RADIUS).max(px(0.0)).min(side() / 4.0))
        .border(move || t.length(BORDER_WIDTH))
        .border_color(move || border(t, on(), invalid()))
        .bg(move || {
            if on() {
                t.color(CHECKBOX_CHECKED)
            } else {
                t.color(CHECKBOX)
            }
        })
        .text_color(move || t.color(CHECKBOX_CHECKED_FOREGROUND))
        .group_focus_visible_named(GROUP, move |s| {
            focus_ring(
                s,
                t,
                move || {
                    if invalid() {
                        t.color(CHECKBOX_RING_INVALID)
                    } else {
                        t.color(CHECKBOX_RING)
                    }
                },
                None,
            )
            .border_color(move || focus_border(t, on(), invalid()))
        })
        .group_active_named(GROUP, move |s| press(s, SMALL_SCALE, disabled))
        .transition_shadow()
        .app_class(props.indicator_class)
        .inline_style(move || {
            shadow::shadow(t, shadow::level(t.number(CHECKBOX_SHADOW)))
                .then(&props.indicator_style.get())
        })
        .child(dynamic(mark));
    let indicator = motion::timing(indicator, t);

    base_checkbox()
        .element_ref(&root)
        .checked(checked)
        .indeterminate(indeterminate)
        .disabled(disabled)
        .invalid(invalid)
        .accessibility_label(props.accessibility_label)
        .accessibility_hint(props.accessibility_hint)
        .accessibility_described_by(move || {
            props
                .accessibility_described_by
                .get()
                .or_else(|| joined.described_by())
        })
        .hit_slop(move || {
            props
                .hit_slop
                .get()
                .unwrap_or_else(|| choice_hit_slop(t, labelled(), side(), side()))
        })
        .on_change(move |next: &bool| cx.emit(CheckboxEvent::Change(*next)))
        .group_named(GROUP)
        .flex_row()
        .items_start()
        .gap(move || t.length(CHECKBOX_GAP))
        .cursor(Cursor::Default)
        .disabled_style(move |s| s.opacity(move || t.number(OPACITY_DISABLED)))
        .indicator_style(|s| s.flex_shrink_0())
        .indicator(indicator)
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

/// Rust: `label().html_for(terms)` with `terms` this checkbox's ref.
impl From<ComponentRef<Checkbox>> for FieldRef {
    fn from(r: ComponentRef<Checkbox>) -> Self {
        FieldRef::lazy(move || r.node())
    }
}

nsh_core::static_reactive!(FieldRef: ComponentRef<Checkbox>);

/// The box's side token of `size`.
pub fn size_token(size: Size) -> Token<kind::Length> {
    size.pick(
        CHECKBOX_SIZE_XS,
        CHECKBOX_SIZE_SM,
        CHECKBOX_SIZE_MD,
        CHECKBOX_SIZE_LG,
    )
}

/// The edge out of focus: checked > invalid > rest.
fn border(t: Theme, on: bool, invalid: bool) -> AdaptiveColor {
    if on {
        t.color(CHECKBOX_CHECKED_BORDER)
    } else if invalid {
        t.color(CHECKBOX_BORDER_INVALID)
    } else {
        t.color(CHECKBOX_BORDER)
    }
}

/// The edge while focused: with a pointer checked > invalid > focus (the
/// ring shows the focus); on touch screens, without a ring, invalid >
/// focus.
fn focus_border(t: Theme, on: bool, invalid: bool) -> AdaptiveColor {
    if invalid && (!on || !pointer()) {
        t.color(CHECKBOX_BORDER_INVALID)
    } else if on && pointer() {
        t.color(CHECKBOX_CHECKED_BORDER)
    } else {
        t.color(CHECKBOX_BORDER_FOCUS)
    }
}
