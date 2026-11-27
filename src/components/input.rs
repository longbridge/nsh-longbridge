//! `Input`: a text input on NSH's host text field (editing, input
//! methods, the software keyboard, autofill and screen readers are the
//! host's). The look is foundation's field look. API:
//! `docs/components/input.md`.

use nsh_core::{
    element::{
        AutoFill, EdgeInsets, JsIn, JsOwned, JsValue, PropError, Selection, TsTypes,
        input as host_input,
    },
    js::host::HostValue,
    prelude::*,
};

use std::{cell::Cell, rc::Rc};

use super::input_group;
use crate::{
    foundation::{
        field::{
            self, INPUT_FONT_SIZE_LG, INPUT_FONT_SIZE_MD, INPUT_FONT_SIZE_SM, INPUT_FONT_SIZE_XS,
            INPUT_FOREGROUND, INPUT_HEIGHT_LG, INPUT_HEIGHT_MD, INPUT_HEIGHT_SM, INPUT_HEIGHT_XS,
            INPUT_PADDING_X_LG, INPUT_PADDING_X_MD, INPUT_PADDING_X_SM, INPUT_PADDING_X_XS,
            INPUT_PLACEHOLDER_FOREGROUND, Joined, Kind, field_look,
        },
        field_ref::FieldRef,
        size::{Size, pointer},
    },
    theme::{Theme, Token, kind, semantic::TOUCH_TARGET, theme},
};

/// What an input holds, for the system's autofill and suggestions (TSX:
/// React Native's `autoComplete` names, `"email"`, `"password"`,
/// `"one-time-code"`, ...): NSH's `AutoFill`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct AutoComplete(pub AutoFill);

impl From<AutoFill> for AutoComplete {
    fn from(fill: AutoFill) -> Self {
        Self(fill)
    }
}

nsh_core::static_reactive!(AutoComplete: AutoComplete, AutoFill);

impl JsValue for AutoComplete {
    fn ts_type(types: &mut TsTypes) -> String {
        <AutoFill as HostValue>::ts_type(types)
    }

    fn from_js(value: JsIn<'_>) -> Result<Self, PropError> {
        if value.is_nullish() {
            return Ok(Self::default());
        }
        value
            .as_string()
            .and_then(|name| AutoFill::parse(&name))
            .map(Self)
            .ok_or_else(|| value.expected("an autoComplete value"))
    }

    fn to_js<'a>(&self, host: &'a dyn nsh_core::element::JsHost) -> Result<JsOwned<'a>, PropError> {
        Option::<String>::None.to_js(host)
    }
}

/// The props of [`Input`].
#[derive(Debug, Clone, PartialEq, Props)]
pub struct InputProps {
    /// The value (controlled): the app owns it, `onChange` reports each
    /// edit and the app sets it back. Unset when the input is made, the
    /// input keeps its own text, from `defaultValue`.
    pub value: Prop<Option<SharedString>>,
    /// The first text of an input that keeps its own (no `value`).
    #[prop(default)]
    pub default_value: Prop<SharedString>,
    /// Shown while it is empty (the app's words).
    pub placeholder: Prop<Option<SharedString>>,
    /// Its size (`xs`, `sm`, `md`, `lg`): height, text and padding change
    /// together. Inside an `InputGroup`, the group's size is taken instead.
    #[prop(default)]
    pub size: Prop<Size>,
    /// No editing and no focus, out of the Tab order, at half opacity. Or'ed
    /// with an enclosing `Field`'s or `FieldSet`'s. Inside an `InputGroup`,
    /// the whole group dims and its buttons are disabled too.
    #[prop(default)]
    pub disabled: Prop<bool>,
    /// Its text can be focused, selected and copied, not changed; it looks
    /// as at rest.
    #[prop(default)]
    pub readonly: Prop<bool>,
    /// The app's checks reject the value: a destructive edge (and ring
    /// while focused); screen readers say it is invalid. Or'ed with an
    /// enclosing `Field`'s. Inside an `InputGroup`, the group's edge.
    #[prop(default)]
    pub invalid: Prop<bool>,
    /// Hides what is typed: a password field. An `InputGroupPasswordToggle`
    /// in its group shows it while pressed.
    #[prop(default)]
    pub masked: Prop<bool>,
    /// The software keyboard's layout (`email-address`, `url`,
    /// `phone-pad`, ...).
    #[prop(default)]
    pub keyboard_type: Prop<KeyboardType>,
    /// The return key's label (the platform's by default).
    #[prop(default)]
    pub return_key_type: Prop<ReturnKeyType>,
    /// Automatic capitals (the platform's by default).
    pub auto_capitalize: Prop<Option<AutoCapitalize>>,
    /// The system's corrections (the platform's by default).
    pub auto_correct: Prop<Option<bool>>,
    /// What it holds, for autofill (`email`, `password`, ...).
    #[prop(default)]
    pub auto_complete: Prop<AutoComplete>,
    /// At most this many characters.
    pub max_length: Prop<Option<u32>>,
    /// Focused when it appears (not while `disabled`).
    #[prop(default)]
    pub auto_focus: Prop<bool>,
    /// Selects its whole text when it takes the focus.
    #[prop(default)]
    pub select_text_on_focus: Prop<bool>,
    /// What screen readers call it. Set it even beside a visible `Label`:
    /// a label does not name its field for screen readers.
    pub accessibility_label: Prop<Option<SharedString>>,
}

/// The events of an [`Input`].
#[derive(Debug, Clone, PartialEq, Event)]
pub enum InputEvent {
    /// The text changed (each edit): the new text.
    #[args(value)]
    Change(String),
    /// Return was pressed (per `returnKeyType`): the text. A Return that
    /// ends an input method's composition does not count.
    #[args(value)]
    Submit(String),
    /// It took the keyboard focus.
    Focus,
    /// It lost the keyboard focus.
    Blur,
}

/// The methods of an [`Input`]'s ref.
pub struct InputApi {
    field: ElementRef,
    disabled: Prop<bool>,
    joined: Joined,
}

#[methods]
impl InputApi {
    /// Takes the keyboard focus (showing the software keyboard); nothing
    /// while disabled.
    pub fn focus(&self) {
        if !self.disabled.get_untracked() && !untracked(|| self.joined.disabled()) {
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

/// A text field: one line of text, four sizes, an invalid
/// state, read-only and disabled. On touch screens it is taller, its text
/// 16 points at `md` and `lg`, and it takes presses up to 44 points tall.
#[component(methods = InputApi)]
pub fn Input(props: InputProps, cx: Scope<InputEvent>) -> impl IntoElement {
    let t = theme();
    // Inside an `InputGroup`: the group draws the frame and sets the size.
    let group = input_group::current();
    let size = move || group.map_or_else(|| props.size.get(), |g| g.size());
    let height = move || t.length(height_token(size()));
    let field = element_ref();
    let joined = field::join(cx.__host(), Kind::Focus(field), move || {
        props.disabled.get()
    });
    let disabled = move || props.disabled.get() || joined.disabled();
    let invalid = move || props.invalid.get() || joined.invalid();
    let main = group.is_some_and(|g| g.join(field, invalid, disabled));
    cx.methods(InputApi {
        field,
        disabled: props.disabled,
        joined,
    });
    // The group's password toggle shows a masked field.
    let masked = move || props.masked.get() && !(main && group.is_some_and(|g| g.shown()));
    let focused = signal(false);
    let report_focus = move |on: bool| {
        focused.set(on);
        if main && let Some(g) = group {
            g.set_focused(on);
        }
    };

    let controlled = props.value.get_untracked().is_some();
    let el = host_input()
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
        .masked(masked)
        .keyboard_type(props.keyboard_type)
        .return_key_type(props.return_key_type)
        .when(props.auto_capitalize.get_untracked().is_some(), |el| {
            el.auto_capitalize(move || props.auto_capitalize.get().unwrap_or_default())
        })
        .when(props.auto_correct.get_untracked().is_some(), |el| {
            el.auto_correct(move || props.auto_correct.get().unwrap_or(true))
        })
        .auto_fill(move || props.auto_complete.get().0)
        .when(props.max_length.get_untracked().is_some(), |el| {
            el.max_length(move || props.max_length.get().unwrap_or(u32::MAX))
        })
        .auto_focus(move || props.auto_focus.get() && !disabled())
        .select_text_on_focus(props.select_text_on_focus)
        // A disabled field takes no edits either (the platforms' text
        // fields stay editable under a disabled view), as `NumberInput`.
        .readonly(move || props.readonly.get() || disabled())
        .disabled(disabled)
        .optional_label(props.accessibility_label)
        .accessibility_state(move || AccessibilityState {
            invalid: invalid(),
            ..AccessibilityState::default()
        })
        .accessibility_described_by_target(move || joined.described_by())
        .on_change(move |text: &str| cx.emit(InputEvent::Change(text.to_owned())))
        .on_submit(move |text: &str| cx.emit(InputEvent::Submit(text.to_owned())))
        .on_focus(move || {
            report_focus(true);
            cx.emit(InputEvent::Focus)
        })
        .on_blur(move || {
            report_focus(false);
            cx.emit(InputEvent::Blur)
        })
        .hit_slop(move || vertical_hit_slop(t, height()))
        .min_w(px(0.0))
        .text_size(move || t.length(font_size_token(size())))
        .style_when(true, move |s| {
            s.placeholder_color(move || t.color(INPUT_PLACEHOLDER_FOREGROUND))
        })
        .disabled_style(|s| s.cursor(Cursor::Default));
    let el = if props.masked.get_untracked() {
        keep_selection(el, field, masked, focused)
    } else {
        el
    };
    match group {
        // The group draws the frame: a transparent field in the room its
        // addons leave, its padding by its place among them.
        Some(g) => el
            .style_when(move || !g.column(), |s| s.flex_1())
            .h(move || input_group::field_height(t, size()))
            .app_class(move || Some(input_group::field_class(t, g)))
            .bg(AdaptiveColor::TRANSPARENT)
            .text_color(move || t.color(INPUT_FOREGROUND)),
        None => {
            let el = el
                .w_full()
                // Beside a label in a horizontal field: an equal share of
                // the row.
                .style_when(move || joined.in_row(), |s| s.flex_1())
                .h(height)
                .px(move || t.length(padding_token(size())));
            field_look(el, t, invalid)
        }
    }
}

/// Puts the selection back after `masked` changes while the field has the
/// focus: a platform may rebuild its field for it (AppKit's secure and
/// plain fields are separate classes), which puts the caret at the end.
/// The selection last seen is set again on the next frame.
fn keep_selection(
    el: nsh_core::element::Input,
    field: ElementRef,
    masked: impl Fn() -> bool + 'static,
    focused: Signal<bool>,
) -> nsh_core::element::Input {
    let seen = Rc::new(Cell::new(None::<(u32, u32)>));
    let before = Cell::new(None::<bool>);
    let at = seen.clone();
    effect(move || {
        let now = masked();
        let changed = before.replace(Some(now)).is_some_and(|was| was != now);
        if changed
            && untracked(|| focused.get())
            && let Some((start, end)) = at.get()
        {
            request_animation_frame(move |_| field.set_selection(start, end));
        }
    });
    el.on_selection_change(move |s: &Selection| seen.set(Some((s.start, s.end))))
}

/// Rust: `label().html_for(email)` with `email` this input's ref.
impl From<ComponentRef<Input>> for FieldRef {
    fn from(r: ComponentRef<Input>) -> Self {
        FieldRef::lazy(move || r.node())
    }
}

nsh_core::static_reactive!(FieldRef: ComponentRef<Input>);

/// The field's height token at `size`.
pub fn height_token(size: Size) -> Token<kind::Length> {
    size.pick(
        INPUT_HEIGHT_XS,
        INPUT_HEIGHT_SM,
        INPUT_HEIGHT_MD,
        INPUT_HEIGHT_LG,
    )
}

/// The field's horizontal padding token at `size` (Input's and
/// Textarea's).
pub fn padding_token(size: Size) -> Token<kind::Length> {
    size.pick(
        INPUT_PADDING_X_XS,
        INPUT_PADDING_X_SM,
        INPUT_PADDING_X_MD,
        INPUT_PADDING_X_LG,
    )
}

/// The field's text size token at `size` (Input's and Textarea's).
pub fn font_size_token(size: Size) -> Token<kind::Length> {
    size.pick(
        INPUT_FONT_SIZE_XS,
        INPUT_FONT_SIZE_SM,
        INPUT_FONT_SIZE_MD,
        INPUT_FONT_SIZE_LG,
    )
}

/// Above and below a field `height` tall, so a finger gets `touch-target`
/// (44) points; none with a pointer. Not at the sides: a field is wide
/// already, and fields side by side would take each other's presses.
pub(crate) fn vertical_hit_slop(t: Theme, height: Points) -> EdgeInsets {
    if pointer() {
        return EdgeInsets::default();
    }
    let slop = ((t.length(TOUCH_TARGET) - height) / 2.0).max(px(0.0));
    EdgeInsets {
        top: slop,
        bottom: slop,
        ..EdgeInsets::default()
    }
}
