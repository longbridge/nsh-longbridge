//! `Label`: shadcn/ui's label. The text naming a field; pressing it moves
//! the focus into the field `htmlFor` names. API:
//! `docs/components/label.md`.

use nsh_core::prelude::*;

use crate::{
    foundation::{field_ref::FieldRef, icon::provide_text_color, typography::weight},
    theme::{
        seeds::SPACING,
        semantic::{FONT_SIZE_MD, FONT_WEIGHT_MEDIUM, FOREGROUND, OPACITY_DISABLED},
        theme,
    },
    tokens,
};

tokens! {
    /// The label's text size.
    LABEL_FONT_SIZE: Length = "label-font-size" => |t| t.length(FONT_SIZE_MD),
    /// The label's text weight (also the choice controls' label parts').
    LABEL_FONT_WEIGHT: Number = "label-font-weight" => |t| t.number(FONT_WEIGHT_MEDIUM),
    /// The label's text color (also the choice controls' label parts').
    LABEL_FOREGROUND: Color = "label-foreground" => |t| t.color(FOREGROUND),
    /// The space between the label's children (an icon and its text).
    LABEL_GAP: Length = "label-gap" => |t| t.length(SPACING) * 2.0,
}

/// The props of [`Label`].
#[derive(Debug, Clone, PartialEq, Props)]
pub struct LabelProps {
    /// The field a press focuses: an `Input`'s or a `Textarea`'s ref.
    #[prop(default)]
    pub html_for: Prop<FieldRef>,
    /// Dimmed, and a press focuses nothing: pass the field's `disabled`.
    #[prop(default)]
    pub disabled: Prop<bool>,
    /// The label: its text, possibly with an icon or a badge.
    pub children: Prop<Children>,
}

/// A label, shadcn/ui's: medium 14-point text in a row with what it holds.
/// Pressing it moves the focus into the field `htmlFor` names. It is no
/// control: no focus, no press feedback, its text not selectable.
#[component]
pub fn Label(props: LabelProps, _cx: Scope<NoEvent>) -> impl IntoElement {
    provide_text_color();
    let t = theme();
    let font_size = move || t.length(LABEL_FONT_SIZE);
    view()
        .disabled(props.disabled)
        .focusable(false)
        .role(AccessibilityRole::Text)
        .on_press(move || {
            if !props.disabled.get_untracked() {
                props.html_for.get_untracked().focus();
            }
        })
        .flex_row()
        .items_center()
        .gap(move || t.length(LABEL_GAP))
        .text_size(font_size)
        // shadcn's `leading-none`: the one line height a component sets
        // itself.
        .line_height(font_size)
        // Not bindable: read when it is built.
        .font_weight(weight(t.number(LABEL_FONT_WEIGHT)))
        .text_color(move || t.color(LABEL_FOREGROUND))
        .select_none()
        .cursor(Cursor::Default)
        .disabled_style(move |s| s.opacity(move || t.number(OPACITY_DISABLED)))
        .child(props.children)
}
