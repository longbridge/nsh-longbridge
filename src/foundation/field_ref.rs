//! What a label names: a form control its press reaches (`Label`'s and
//! `FieldLabel`'s `htmlFor`). A press focuses a text control (an `Input`,
//! a `Select`'s trigger) and presses a checkbox-like one (a `Checkbox`
//! toggles, as a browser's `<label>` does). TSX passes the control's ref
//! (an `InputRef`, a `CheckboxRef`, ...), Rust a `ComponentRef` of one.
//! Each control registers itself under its host while it is mounted
//! ([`register_control`], which `field::join` calls), so a ref, from
//! either language, finds what to do.

use std::{cell::RefCell, collections::HashMap, fmt, rc::Rc};

use nsh_core::{
    NodeId,
    element::{JsIn, JsOwned, JsValue, NodeRef, PropError, TsTypes},
    prelude::*,
};

/// The refs a label takes in TSX: every control a press reaches, one per
/// line.
const TS_TYPES: &[&str] = &[
    "InputRef",
    "TextareaRef",
    "SelectRef",
    "CheckboxRef",
    "SwitchRef",
    "DatePickerRef",
];

/// A control a label's press reaches: none, or a control component's ref.
/// Controls are this library's `Input`, `Textarea`, `Select`, `Checkbox`,
/// `Switch` and `DatePicker`, and later ones that register (Rust converts their
/// `ComponentRef`s into it).
#[derive(Clone, Default)]
pub struct FieldRef(Target);

#[derive(Clone, Default)]
enum Target {
    #[default]
    None,
    /// The host node of the control component (a TSX ref's `node`).
    Node(NodeId),
    /// The control's host, found when it is mounted (a Rust
    /// `ComponentRef`, which may be put on the control after the label is
    /// made).
    Lazy(Rc<dyn Fn() -> Option<NodeId>>),
}

impl FieldRef {
    /// No control.
    pub const NONE: FieldRef = FieldRef(Target::None);

    /// The control whose component host `node` is.
    pub fn node(node: NodeId) -> Self {
        Self(Target::Node(node))
    }

    /// The control `host` finds once it is mounted (a component ref's
    /// `node()`).
    pub fn lazy(host: impl Fn() -> Option<NodeId> + 'static) -> Self {
        Self(Target::Lazy(Rc::new(host)))
    }

    /// Whether it names a control (mounted or not).
    pub fn is_some(&self) -> bool {
        !matches!(self.0, Target::None)
    }

    /// The control component's host, while one is mounted.
    pub fn host(&self) -> Option<NodeId> {
        match &self.0 {
            Target::None => None,
            Target::Node(node) => Some(*node),
            Target::Lazy(host) => host(),
        }
    }

    /// What a label's press does to the control: it moves the keyboard
    /// focus into a text control (showing the software keyboard) and
    /// presses a checkbox-like one (reporting its `onChange`). Nothing
    /// while it is disabled or not mounted, or for a control a label does
    /// not reach (a slider).
    pub fn press(&self) {
        if let Some(control) = self.control() {
            match &control.press {
                Press::Focus(element) => element.focus(),
                Press::Activate(activate) => activate(),
                Press::Nothing => {}
            }
        }
    }

    /// Moves the keyboard focus into the control, unless it is disabled,
    /// not mounted or takes no focus from a label.
    pub fn focus(&self) {
        if let Some(Control {
            press: Press::Focus(element),
            ..
        }) = self.control()
        {
            element.focus();
        }
    }

    /// Whether the named control is mounted and checked (a checkbox, a
    /// switch, a radio that is on).
    pub fn checked(&self) -> bool {
        self.host()
            .and_then(|host| CONTROLS.with(|c| c.borrow().get(&host).cloned()))
            .is_some_and(|control| (control.checked)())
    }

    /// The mounted control, while it is enabled.
    fn control(&self) -> Option<Control> {
        let control = self
            .host()
            .and_then(|host| CONTROLS.with(|c| c.borrow().get(&host).cloned()))?;
        (!(control.disabled)()).then_some(control)
    }
}

impl PartialEq for FieldRef {
    fn eq(&self, other: &Self) -> bool {
        match (&self.0, &other.0) {
            (Target::None, Target::None) => true,
            (Target::Node(a), Target::Node(b)) => a == b,
            (Target::Lazy(a), Target::Lazy(b)) => Rc::ptr_eq(a, b),
            _ => false,
        }
    }
}

impl fmt::Debug for FieldRef {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self.0 {
            Target::None => f.write_str("FieldRef(None)"),
            _ => write!(f, "FieldRef({:?})", self.host()),
        }
    }
}

nsh_core::static_reactive!(FieldRef: FieldRef);

/// TSX: the ref of a control (`InputRef`, `SelectRef`, `CheckboxRef`, ...:
/// `{ node, focus, .. }`), or `null` / `undefined` for none.
impl JsValue for FieldRef {
    fn ts_type(_types: &mut TsTypes) -> String {
        TS_TYPES.join(" | ")
    }

    fn from_js(value: JsIn<'_>) -> Result<Self, PropError> {
        if value.is_nullish() {
            return Ok(Self::NONE);
        }
        // A component's ref carries its host node as `node`.
        let node = value.field("node")?;
        match Option::<NodeRef>::from_js(node.get())? {
            Some(NodeRef(node)) => Ok(Self::node(node)),
            None => Err(value.expected("a control's ref (an InputRef, a CheckboxRef, ...)")),
        }
    }

    fn to_js<'a>(&self, host: &'a dyn nsh_core::element::JsHost) -> Result<JsOwned<'a>, PropError> {
        match self.host() {
            Some(node) => NodeRef(node).to_js(host),
            None => Option::<NodeRef>::None.to_js(host),
        }
    }
}

/// What a label's press does to a control.
#[derive(Clone)]
pub enum Press {
    /// Moves the keyboard focus to this element (a text field, a select's
    /// trigger).
    Focus(ElementRef),
    /// Presses the control, as its own press does (a checkbox toggles and
    /// reports `onChange`).
    Activate(Rc<dyn Fn()>),
    /// Nothing: a label does not reach it (a slider, a toggle group).
    Nothing,
}

impl fmt::Debug for Press {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Press::Focus(_) => f.write_str("Press::Focus"),
            Press::Activate(_) => f.write_str("Press::Activate"),
            Press::Nothing => f.write_str("Press::Nothing"),
        }
    }
}

/// A mounted control: what a press does, whether it is disabled, and
/// whether it is checked.
#[derive(Clone)]
struct Control {
    press: Press,
    disabled: Rc<dyn Fn() -> bool>,
    checked: Rc<dyn Fn() -> bool>,
}

thread_local! {
    static CONTROLS: RefCell<HashMap<NodeId, Control>> = RefCell::new(HashMap::new());
}

/// Registers the control component hosted at `host` (its `Scope`'s host)
/// while the current owner lives: a [`FieldRef`] naming it does `press`,
/// unless `disabled` (read untracked, at the press), and reads `checked`.
/// Call it in the control component's body (`field::join` does).
pub fn register_control(
    host: NodeId,
    press: Press,
    disabled: impl Fn() -> bool + 'static,
    checked: impl Fn() -> bool + 'static,
) {
    let control = Control {
        press,
        disabled: Rc::new(move || untracked(&disabled)),
        checked: Rc::new(checked),
    };
    CONTROLS.with(|c| c.borrow_mut().insert(host, control));
    cleanup(move || {
        CONTROLS.with(|c| c.borrow_mut().remove(&host));
    });
}

/// Registers the field component hosted at `host`: a [`FieldRef`] naming
/// it focuses `element`, unless `disabled`. [`register_control`] for a
/// text field.
pub fn register_field(host: NodeId, element: ElementRef, disabled: Prop<bool>) {
    register_control(
        host,
        Press::Focus(element),
        move || disabled.get(),
        || false,
    );
}
