//! What a label names: a field it moves the focus to when pressed
//! (`Label`'s `htmlFor`). TSX passes the field's ref (an `InputRef`, a
//! `TextareaRef`), Rust a `ComponentRef` of one. Each field component
//! registers itself under its host while it is mounted
//! ([`register_field`]), so a ref, from either language, finds what to
//! focus.

use std::{cell::RefCell, collections::HashMap, fmt, rc::Rc};

use nsh_core::{
    NodeId,
    element::{JsIn, JsOwned, JsValue, NodeRef, PropError, TsTypes},
    prelude::*,
};

/// A field a label moves the focus to: none, or a field component's ref.
/// Fields are this library's `Input` and `Textarea` (Rust converts their
/// `ComponentRef`s into it).
#[derive(Clone, Default)]
pub struct FieldRef(Target);

#[derive(Clone, Default)]
enum Target {
    #[default]
    None,
    /// The host node of the field component (a TSX ref's `node`).
    Node(NodeId),
    /// The field's host, found when it is mounted (a Rust
    /// `ComponentRef`, which may be put on the field after the label is
    /// made).
    Lazy(Rc<dyn Fn() -> Option<NodeId>>),
}

impl FieldRef {
    /// No field.
    pub const NONE: FieldRef = FieldRef(Target::None);

    /// The field whose component host `node` is.
    pub fn node(node: NodeId) -> Self {
        Self(Target::Node(node))
    }

    /// The field `host` finds once it is mounted (a component ref's
    /// `node()`).
    pub fn lazy(host: impl Fn() -> Option<NodeId> + 'static) -> Self {
        Self(Target::Lazy(Rc::new(host)))
    }

    /// The field component's host, while one is mounted.
    pub fn host(&self) -> Option<NodeId> {
        match &self.0 {
            Target::None => None,
            Target::Node(node) => Some(*node),
            Target::Lazy(host) => host(),
        }
    }

    /// Moves the keyboard focus into the field (showing the software
    /// keyboard), unless it is disabled or not mounted.
    pub fn focus(&self) {
        let field = self
            .host()
            .and_then(|host| FIELDS.with(|f| f.borrow().get(&host).copied()));
        if let Some(field) = field
            && !field.disabled.get_untracked()
        {
            field.element.focus();
        }
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

/// TSX: the ref of an `Input` or a `Textarea` (`{ node, focus, .. }`), or
/// `null` / `undefined` for none.
impl JsValue for FieldRef {
    fn ts_type(_types: &mut TsTypes) -> String {
        "InputRef | TextareaRef".into()
    }

    fn from_js(value: JsIn<'_>) -> Result<Self, PropError> {
        if value.is_nullish() {
            return Ok(Self::NONE);
        }
        // A component's ref carries its host node as `node`.
        let node = value.field("node")?;
        match Option::<NodeRef>::from_js(node.get())? {
            Some(NodeRef(node)) => Ok(Self::node(node)),
            None => Err(value.expected("an InputRef or a TextareaRef")),
        }
    }

    fn to_js<'a>(&self, host: &'a dyn nsh_core::element::JsHost) -> Result<JsOwned<'a>, PropError> {
        match self.host() {
            Some(node) => NodeRef(node).to_js(host),
            None => Option::<NodeRef>::None.to_js(host),
        }
    }
}

/// A mounted field: the element that takes the focus, and whether it is
/// disabled.
#[derive(Clone, Copy)]
struct Field {
    element: ElementRef,
    disabled: Prop<bool>,
}

thread_local! {
    static FIELDS: RefCell<HashMap<NodeId, Field>> = RefCell::new(HashMap::new());
}

/// Registers the field component hosted at `host` (its `Scope`'s host)
/// while the current owner lives: a [`FieldRef`] naming it focuses
/// `element`, unless `disabled`. Call it in the field component's body.
pub fn register_field(host: NodeId, element: ElementRef, disabled: Prop<bool>) {
    FIELDS.with(|f| f.borrow_mut().insert(host, Field { element, disabled }));
    cleanup(move || {
        FIELDS.with(|f| f.borrow_mut().remove(&host));
    });
}
