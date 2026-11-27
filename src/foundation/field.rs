//! What every form control shares.
//!
//! - **The field look**: what every control that looks like a text field
//!   shares (`Input`, `Textarea`, a select's trigger): its edge,
//!   background, shadow, focus ring and its invalid and disabled states,
//!   from the `input-*` tokens ([`field_look`]). One set of tokens, so a
//!   theme that changes `input-border` changes them all.
//! - **Joining a field**: a `Field` hands its `invalid` and `disabled`
//!   state and its description or error to the one control inside it, and
//!   a `FieldSet` its `disabled` to every control inside it ([`provide`],
//!   [`provide_set`]). Each control calls [`join`] once in its body: it
//!   registers what a label's press does to it (`foundation::field_ref`)
//!   and gets back what the field gives it. Outside a `Field` and a
//!   `FieldSet` it gets nothing, and behaves as on its own.

use std::rc::Rc;

use nsh_core::{
    NodeId,
    element::{ElementTarget, StyleRefinement},
    prelude::*,
};

use super::{
    field_ref::{Press, register_control},
    focus, motion, shadow,
};
use crate::{
    theme::{
        TRANSPARENT, Theme,
        seeds::SPACING,
        semantic::{
            BORDER_WIDTH, CONTROL_HEIGHT_LG, CONTROL_HEIGHT_MD, CONTROL_HEIGHT_SM,
            CONTROL_HEIGHT_TOUCH_LG, CONTROL_HEIGHT_TOUCH_MD, CONTROL_HEIGHT_TOUCH_SM,
            CONTROL_HEIGHT_TOUCH_XS, CONTROL_HEIGHT_XS, DESTRUCTIVE, FONT_SIZE_LG, FONT_SIZE_MD,
            FONT_SIZE_SM, FONT_SIZE_XS, FOREGROUND, INPUT, MUTED_FOREGROUND, OPACITY_DISABLED,
            RADIUS_MD, RING, RING_INVALID, RING_OPACITY, RING_WIDTH,
        },
    },
    tokens,
};

tokens! {
    /// The field's corner radius.
    INPUT_RADIUS: Length = "input-radius" => |t| t.length(RADIUS_MD),
    /// An extra small field's height.
    INPUT_HEIGHT_XS: Length = "input-height-xs" => |t| t.pointer_touch(t.length(CONTROL_HEIGHT_XS), t.length(CONTROL_HEIGHT_TOUCH_XS)),
    /// A small field's height.
    INPUT_HEIGHT_SM: Length = "input-height-sm" => |t| t.pointer_touch(t.length(CONTROL_HEIGHT_SM), t.length(CONTROL_HEIGHT_TOUCH_SM)),
    /// A medium field's height.
    INPUT_HEIGHT_MD: Length = "input-height-md" => |t| t.pointer_touch(t.length(CONTROL_HEIGHT_MD), t.length(CONTROL_HEIGHT_TOUCH_MD)),
    /// A large field's height.
    INPUT_HEIGHT_LG: Length = "input-height-lg" => |t| t.pointer_touch(t.length(CONTROL_HEIGHT_LG), t.length(CONTROL_HEIGHT_TOUCH_LG)),
    /// An extra small field's horizontal padding.
    INPUT_PADDING_X_XS: Length = "input-padding-x-xs" => |t| t.length(SPACING) * 2.0,
    /// A small field's horizontal padding.
    INPUT_PADDING_X_SM: Length = "input-padding-x-sm" => |t| t.length(SPACING) * 2.5,
    /// A medium field's horizontal padding.
    INPUT_PADDING_X_MD: Length = "input-padding-x-md" => |t| t.length(SPACING) * 3.0,
    /// A large field's horizontal padding.
    INPUT_PADDING_X_LG: Length = "input-padding-x-lg" => |t| t.length(SPACING) * 3.0,
    /// An extra small field's text size: 14 on touch screens.
    INPUT_FONT_SIZE_XS: Length = "input-font-size-xs" => |t| t.pointer_touch(t.length(FONT_SIZE_XS), t.px(14.0)),
    /// A small field's text size: 14 on touch screens.
    INPUT_FONT_SIZE_SM: Length = "input-font-size-sm" => |t| t.pointer_touch(t.length(FONT_SIZE_SM), t.px(14.0)),
    /// A medium field's text size: 16 on touch screens, where a browser
    /// zooms into a field with smaller text.
    INPUT_FONT_SIZE_MD: Length = "input-font-size-md" => |t| t.pointer_touch(t.length(FONT_SIZE_MD), t.px(16.0)),
    /// A large field's text size: 16 on touch screens.
    INPUT_FONT_SIZE_LG: Length = "input-font-size-lg" => |t| t.pointer_touch(t.length(FONT_SIZE_LG), t.px(16.0)),
    /// The width of the field's edge.
    INPUT_BORDER_WIDTH: Length = "input-border-width" => |t| t.length(BORDER_WIDTH),
    /// The field's edge at rest.
    INPUT_BORDER: Color = "input-border" => |t| t.color(INPUT),
    /// The edge while the field has the keyboard focus.
    INPUT_BORDER_FOCUS: Color = "input-border-focus" => |t| t.color(RING),
    /// The edge while the app's checks reject the value; wins over the focus edge.
    INPUT_BORDER_INVALID: Color = "input-border-invalid" => |t| t.color(DESTRUCTIVE),
    /// The field's background: transparent, `input` at 30 % in the dark.
    INPUT_BACKGROUND: Color = "input-background" => |t| t.light_dark(TRANSPARENT, t.color(INPUT).opacity(0.3)),
    /// The field's text.
    INPUT_FOREGROUND: Color = "input-foreground" => |t| t.color(FOREGROUND),
    /// The placeholder's text.
    INPUT_PLACEHOLDER_FOREGROUND: Color = "input-placeholder-foreground" => |t| t.color(MUTED_FOREGROUND),
    /// The focus ring around the field.
    INPUT_RING: Color = "input-ring" => |t| t.color(RING).opacity(t.number(RING_OPACITY)),
    /// The focus ring of a field whose value the app rejects.
    INPUT_RING_INVALID: Color = "input-ring-invalid" => |t| t.color(RING_INVALID),
    /// The focus ring's width.
    INPUT_RING_WIDTH: Length = "input-ring-width" => |t| t.length(RING_WIDTH),
    /// The field's shadow level (1: `xs`).
    INPUT_SHADOW: Number = "input-shadow" => |t| t.value(1.0),
}

/// Gives `el` the field look: the edge, background, text color, shadow,
/// focus ring (with a pointer; the edge alone on touch screens), the
/// invalid edge and ring and the disabled opacity, with their transitions.
/// The caller sets the size (`input-height-*`, `input-padding-x-*`,
/// `input-font-size-*`) and the placeholder color.
pub fn field_look<E: Styled + Interactive>(
    el: E,
    t: Theme,
    invalid: impl Fn() -> bool + Copy + 'static,
) -> E {
    let el = field_rest(el, t, invalid)
        .focus_visible(move |s| field_focus(s, t, invalid))
        .disabled_style(move |s| s.opacity(move || t.number(OPACITY_DISABLED)));
    motion::transition_all(el, t)
}

/// The field look at rest ([`field_look`] without its states): the edge,
/// background, text color and shadow. For a frame whose states the caller
/// applies itself: a frame around an input takes [`field_focus`] while the
/// input inside has the focus (`focus-within`), with the transitions.
pub fn field_rest<E: Styled + Element>(
    el: E,
    t: Theme,
    invalid: impl Fn() -> bool + Copy + 'static,
) -> E {
    el.rounded(move || t.length(INPUT_RADIUS))
        .border(move || t.length(INPUT_BORDER_WIDTH))
        .border_color(move || {
            if invalid() {
                t.color(INPUT_BORDER_INVALID)
            } else {
                t.color(INPUT_BORDER)
            }
        })
        .bg(move || t.color(INPUT_BACKGROUND))
        .text_color(move || t.color(INPUT_FOREGROUND))
        .inline_style(move || shadow::shadow(t, shadow::level(t.number(INPUT_SHADOW))))
}

/// The field look's focus: the ring (with a pointer) and the focus edge,
/// red while `invalid`.
pub fn field_focus(
    s: StyleRefinement,
    t: Theme,
    invalid: impl Fn() -> bool + Copy + 'static,
) -> StyleRefinement {
    focus::focus_ring(
        s,
        t,
        move || {
            if invalid() {
                t.color(INPUT_RING_INVALID)
            } else {
                t.color(INPUT_RING)
            }
        },
        None,
    )
    .border_color(move || {
        if invalid() {
            t.color(INPUT_BORDER_INVALID)
        } else {
            t.color(INPUT_BORDER_FOCUS)
        }
    })
}

/// What a label's press does to a control that joins a field ([`join`]).
#[derive(Clone)]
pub enum Kind {
    /// Moves the keyboard focus to this element: a text field, a select's
    /// trigger, a combobox's input.
    Focus(ElementRef),
    /// Presses the control as its own press does (`press`, which reports
    /// its `onChange`): a checkbox, a switch, a radio. `checked` says
    /// whether it is on (a choice card shows it).
    Activate {
        /// The control's press.
        press: Rc<dyn Fn()>,
        /// Whether it is on.
        checked: Rc<dyn Fn() -> bool>,
    },
    /// Nothing: a label does not reach it (a slider, a toggle group, a
    /// radio group). It still takes the field's state.
    Inert,
}

impl Kind {
    /// [`Kind::Activate`] from closures.
    pub fn activate(press: impl Fn() + 'static, checked: impl Fn() -> bool + 'static) -> Self {
        Kind::Activate {
            press: Rc::new(press),
            checked: Rc::new(checked),
        }
    }
}

/// The state a `Field` gives the parts and the control inside it.
#[derive(Clone, Copy)]
pub struct FieldState {
    invalid: Memo<bool>,
    disabled: Memo<bool>,
    control: Signal<Option<NodeId>>,
    description: Signal<Option<ElementRef>>,
    error: Signal<Option<ElementRef>>,
}

impl FieldState {
    /// The field rejects its value (the `Field`'s `invalid`).
    pub fn invalid(&self) -> bool {
        self.invalid.get()
    }

    /// The field is disabled (the `Field`'s or an enclosing `FieldSet`'s
    /// `disabled`).
    pub fn disabled(&self) -> bool {
        self.disabled.get()
    }

    /// The host of the control that joined the field, while one is mounted:
    /// what its label names.
    pub fn control(&self) -> Option<NodeId> {
        self.control.get()
    }

    /// Makes `element` the field's description while the current owner
    /// lives (a `FieldDescription` while it is mounted). The first one
    /// counts.
    pub fn register_description(&self, element: ElementRef) {
        register_part(self.description, element);
    }

    /// Makes `element` the field's error while the current owner lives (a
    /// `FieldError` while it shows).
    pub fn register_error(&self, element: ElementRef) {
        register_part(self.error, element);
    }

    /// What describes the control for screen readers: the error while it
    /// shows, else the description.
    pub fn described_by(&self) -> Option<ElementTarget> {
        self.error
            .get()
            .or_else(|| self.description.get())
            .and_then(Option::<ElementTarget>::from)
    }
}

/// Fills a part's register with `element` while the current owner lives,
/// unless another part holds it.
fn register_part(register: Signal<Option<ElementRef>>, element: ElementRef) {
    if register.get_untracked().is_some() {
        return;
    }
    register.set(Some(element));
    cleanup(move || {
        if untracked(|| register.try_get()) == Some(Some(element)) {
            register.try_set(None);
        }
    });
}

/// What a `FieldSet` gives what is inside it.
#[derive(Clone, Copy)]
struct SetState {
    disabled: Memo<bool>,
}

/// Starts a field for what is made below: its `invalid` and `disabled`
/// (or'ed with an enclosing set's), and empty registers its control and
/// parts fill. Call it in the `Field`'s body.
pub fn provide(
    invalid: impl Fn() -> bool + 'static,
    disabled: impl Fn() -> bool + 'static,
) -> FieldState {
    let set = set_disabled();
    let state = FieldState {
        invalid: computed(invalid),
        disabled: computed(move || disabled() || set.is_some_and(|s| s.get())),
        control: signal(None),
        description: signal(None),
        error: signal(None),
    };
    provide_context(Some(state));
    state
}

/// Starts a set of fields for what is made below: `disabled` (or'ed with an
/// enclosing set's) disables every control inside it. Call it in the
/// `FieldSet`'s body.
pub fn provide_set(disabled: impl Fn() -> bool + 'static) {
    let outer = set_disabled();
    let state = SetState {
        disabled: computed(move || disabled() || outer.is_some_and(|s| s.get())),
    };
    provide_context(Some(state));
}

/// Ends the field and the set for what is made below: an overlay's content
/// placed inside a field is not part of it.
pub fn clear() {
    provide_context(None::<FieldState>);
    provide_context(None::<SetState>);
}

/// The field the caller is made in, if any.
pub fn current() -> Option<FieldState> {
    use_context::<Option<FieldState>>().flatten()
}

/// The enclosing set's `disabled`, if the caller is in one.
fn set_disabled() -> Option<Memo<bool>> {
    use_context::<Option<SetState>>()
        .flatten()
        .map(|s| s.disabled)
}

/// What a control gets from the field it joined: nothing outside a field.
#[derive(Clone, Copy, Default)]
pub struct Joined {
    field: Option<FieldState>,
    set: Option<Memo<bool>>,
}

impl Joined {
    /// The field it joined, if any.
    pub fn field(&self) -> Option<FieldState> {
        self.field
    }

    /// The field rejects the value: or it with the control's `invalid`.
    pub fn invalid(&self) -> bool {
        self.field.is_some_and(|f| f.invalid())
    }

    /// The field or the set is disabled: or it with the control's
    /// `disabled` (a field cannot enable a control the app disabled).
    pub fn disabled(&self) -> bool {
        match self.field {
            Some(f) => f.disabled(),
            None => self.set.is_some_and(|s| s.get()),
        }
    }

    /// What describes the control for screen readers (the field's error
    /// while it shows, else its description); a control's own
    /// `accessibilityDescribedBy` wins over it.
    pub fn described_by(&self) -> Option<ElementTarget> {
        self.field.and_then(|f| f.described_by())
    }
}

/// Joins the control component hosted at `host` (its `Scope`'s host) to
/// the field it is made in. Call it once in the control's body, with its
/// own `disabled`:
///
/// - it registers what a label's press does to the control ([`Kind`]), for
///   a `FieldRef` naming it, while the control is mounted;
/// - inside a `Field` that has no control yet, it becomes the field's
///   control (its `FieldLabel` names it) and gets the field's `invalid`,
///   `disabled` and described-by target; inside a `FieldSet` alone, the
///   set's `disabled`.
///
/// Outside both it gets nothing: the control behaves as on its own.
pub fn join(host: NodeId, kind: Kind, disabled: impl Fn() -> bool + 'static) -> Joined {
    let field = current().filter(|f| f.control.get_untracked().is_none());
    let joined = Joined {
        field,
        set: set_disabled(),
    };
    let (press, checked): (Press, Rc<dyn Fn() -> bool>) = match kind {
        Kind::Focus(element) => (Press::Focus(element), Rc::new(|| false)),
        Kind::Activate { press, checked } => (Press::Activate(press), checked),
        Kind::Inert => (Press::Nothing, Rc::new(|| false)),
    };
    register_control(
        host,
        press,
        move || disabled() || joined.disabled(),
        move || checked(),
    );
    if let Some(f) = field {
        f.control.set(Some(host));
        cleanup(move || {
            // The field may go first (its owner disposed).
            if untracked(|| f.control.try_get()) == Some(Some(host)) {
                f.control.try_set(None);
            }
        });
    }
    joined
}
