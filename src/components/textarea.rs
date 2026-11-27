//! `Textarea`: a text area on NSH's host text field with several
//! lines. Foundation's field look with Input's tokens; it grows with its
//! text from `textarea-min-height`. API: `docs/components/textarea.md`.

use nsh_core::{element::input as host_input, prelude::*};

use super::input::{font_size_token, padding_token};
use crate::{
    foundation::{
        field::{INPUT_BORDER_WIDTH, INPUT_PLACEHOLDER_FOREGROUND, field_look},
        field_ref::{FieldRef, register_field},
        size::Size,
        typography::line_height,
    },
    theme::{seeds::SPACING, theme},
    tokens,
};

tokens! {
    /// The textarea's least height; it grows with its text from there.
    TEXTAREA_MIN_HEIGHT: Length = "textarea-min-height" => |t| t.length(SPACING) * 16.0,
    /// The space above and below the text.
    TEXTAREA_PADDING_Y: Length = "textarea-padding-y" => |t| t.length(SPACING) * 2.0,
}

/// The props of [`Textarea`].
#[derive(Debug, Clone, PartialEq, Props)]
pub struct TextareaProps {
    /// The value (controlled): the app owns it, `onChange` reports each
    /// edit and the app sets it back. Unset when the textarea is made, it
    /// keeps its own text, from `defaultValue`.
    pub value: Prop<Option<SharedString>>,
    /// The first text of a textarea that keeps its own (no `value`).
    #[prop(default)]
    pub default_value: Prop<SharedString>,
    /// Shown while it is empty (the app's words).
    pub placeholder: Prop<Option<SharedString>>,
    /// Its size (`xs`, `sm`, `md`, `lg`): the text and the horizontal
    /// padding, so its text lines up with an Input's of the same size. Its
    /// height follows its text.
    #[prop(default)]
    pub size: Prop<Size>,
    /// No editing and no focus, out of the Tab order, at half opacity.
    #[prop(default)]
    pub disabled: Prop<bool>,
    /// Its text can be focused, selected and copied, not changed; it looks
    /// as at rest.
    #[prop(default)]
    pub readonly: Prop<bool>,
    /// The app's checks reject the value: a destructive edge (and ring
    /// while focused); screen readers say it is invalid.
    #[prop(default)]
    pub invalid: Prop<bool>,
    /// What Return does: a line break (the default), or submit
    /// (Shift / Option / Alt + Return still break the line).
    #[prop(default = SubmitBehavior::Newline)]
    pub submit_behavior: Prop<SubmitBehavior>,
    /// Automatic capitals (the platform's by default).
    pub auto_capitalize: Prop<Option<AutoCapitalize>>,
    /// The system's corrections (the platform's by default).
    pub auto_correct: Prop<Option<bool>>,
    /// At most this many characters.
    pub max_length: Prop<Option<u32>>,
    /// Focused when it appears (not while `disabled`).
    #[prop(default)]
    pub auto_focus: Prop<bool>,
    /// What screen readers call it. Set it even beside a visible `Label`:
    /// a label does not name its field for screen readers.
    pub accessibility_label: Prop<Option<SharedString>>,
}

/// The events of a [`Textarea`].
#[derive(Debug, Clone, PartialEq, Event)]
pub enum TextareaEvent {
    /// The text changed (each edit): the new text.
    #[args(value)]
    Change(String),
    /// Return was pressed with `submitBehavior` `submit` or
    /// `blurAndSubmit`: the text.
    #[args(value)]
    Submit(String),
    /// It took the keyboard focus.
    Focus,
    /// It lost the keyboard focus.
    Blur,
}

/// The methods of a [`Textarea`]'s ref.
pub struct TextareaApi {
    field: ElementRef,
    disabled: Prop<bool>,
}

#[methods]
impl TextareaApi {
    /// Takes the keyboard focus (showing the software keyboard); nothing
    /// while disabled.
    pub fn focus(&self) {
        if !self.disabled.get_untracked() {
            self.field.focus();
        }
    }

    /// Gives up the keyboard focus.
    pub fn blur(&self) {
        self.field.blur();
    }

    /// Selects from `start` to `end` (UTF-16 units; a caret when equal).
    pub fn set_selection(&self, start: u32, end: u32) {
        self.field.set_selection(start, end);
    }

    /// Inserts `text` at the caret, over the selection.
    pub fn insert_text(&self, text: String) {
        self.field.insert_text(text);
    }
}

/// A text field of several lines: Input's look, growing with
/// its text from 64 points (an app's `max-h-*` caps it, past which it
/// scrolls inside; an `h-*` fixes it).
#[component(methods = TextareaApi)]
pub fn Textarea(props: TextareaProps, cx: Scope<TextareaEvent>) -> impl IntoElement {
    let t = theme();
    let size = move || props.size.get();
    let font_size = move || t.length(font_size_token(size()));
    let field = element_ref();
    cx.methods(TextareaApi {
        field,
        disabled: props.disabled,
    });
    register_field(cx.__host(), field, props.disabled);

    let controlled = props.value.get_untracked().is_some();
    let el = host_input()
        .multiline(true)
        .element_ref(&field)
        .when(controlled, |el| {
            el.value(move || props.value.get().map(|s| s.to_string()).unwrap_or_default())
        })
        .when(!controlled, |el| {
            el.default_value(move || props.default_value.get().to_string())
        })
        .placeholder(move || {
            props
                .placeholder
                .get()
                .map(|s| s.to_string())
                .unwrap_or_default()
        })
        .submit_behavior(props.submit_behavior)
        .when(props.auto_capitalize.get_untracked().is_some(), |el| {
            el.auto_capitalize(move || props.auto_capitalize.get().unwrap_or_default())
        })
        .when(props.auto_correct.get_untracked().is_some(), |el| {
            el.auto_correct(move || props.auto_correct.get().unwrap_or(true))
        })
        .when(props.max_length.get_untracked().is_some(), |el| {
            el.max_length(move || props.max_length.get().unwrap_or(u32::MAX))
        })
        .auto_focus(move || props.auto_focus.get() && !props.disabled.get())
        .readonly(move || props.readonly.get() || props.disabled.get())
        .disabled(props.disabled)
        .optional_label(props.accessibility_label)
        .accessibility_state(move || AccessibilityState {
            invalid: props.invalid.get(),
            ..AccessibilityState::default()
        })
        .on_change(move |text: &str| cx.emit(TextareaEvent::Change(text.to_owned())))
        .on_submit(move |text: &str| cx.emit(TextareaEvent::Submit(text.to_owned())))
        .on_focus(move || cx.emit(TextareaEvent::Focus))
        .on_blur(move || cx.emit(TextareaEvent::Blur))
        .w_full()
        // No height: the host measures its text as it lays out, so the
        // field grows in the same layout as the edit (no frame late).
        .min_h(move || {
            t.length(TEXTAREA_MIN_HEIGHT)
                .max(line_height(font_size()) + t.length(INPUT_BORDER_WIDTH) * 2.0)
        })
        .px(move || t.length(padding_token(size())))
        .py(move || t.length(TEXTAREA_PADDING_Y))
        .text_size(font_size)
        .line_height(move || line_height(font_size()))
        .style_when(true, move |s| {
            s.placeholder_color(move || t.color(INPUT_PLACEHOLDER_FOREGROUND))
        })
        .disabled_style(|s| s.cursor(Cursor::Default));
    field_look(el, t, move || props.invalid.get())
}

/// Rust: `label().html_for(notes)` with `notes` this textarea's ref.
impl From<ComponentRef<Textarea>> for FieldRef {
    fn from(r: ComponentRef<Textarea>) -> Self {
        FieldRef::lazy(move || r.node())
    }
}

nsh_core::static_reactive!(FieldRef: ComponentRef<Textarea>);
