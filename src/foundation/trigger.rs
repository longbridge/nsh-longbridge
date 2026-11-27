//! A trigger slot's press, reaching a library pressable placed in it.
//!
//! An overlay (`Popover`, `Dialog`, `DropdownMenu`, ...) places the app's
//! `trigger` in a slot whose view takes the presses. A plain view given as
//! the trigger has no press of its own, so a press on it reaches the slot's
//! view. A library `Button` given as the trigger is a pressable itself: the
//! press stops at it, and the slot's view never gets it. So the overlay
//! provides its toggle and its open state to the slot's content
//! ([`provide`]); a `Button` there runs the toggle after its own press and
//! is lit while the overlay is open. The slot's view keeps the trigger's
//! state for screen readers (expanded, has a popup).
//!
//! The slot's view can hear the same press as the `Button` in it (seen
//! once an alert dialog has closed), so the overlay lets one of them
//! through ([`OnePress`]).

use std::{rc::Rc, time::Duration};

use nsh_core::{prelude::*, time::Instant};

/// Presses on a trigger closer than this are one press heard twice.
const ONE_PRESS: Duration = Duration::from_millis(50);

/// Lets through the first of the presses that are one ([`ONE_PRESS`]):
/// the slot's view's and the `Button`'s.
#[derive(Clone, Copy)]
pub(crate) struct OnePress(Signal<Option<Instant>>);

impl OnePress {
    /// None heard yet.
    pub(crate) fn new() -> Self {
        Self(signal(None))
    }

    /// Whether this press is the first of its group (and to act on).
    pub(crate) fn first(&self) -> bool {
        let now = nsh_core::time::now();
        if self
            .0
            .get_untracked()
            .is_some_and(|at| now.saturating_duration_since(at) < ONE_PRESS)
        {
            return false;
        }
        self.0.set(Some(now));
        true
    }
}

/// The overlay whose trigger slot a pressable is in.
#[derive(Clone)]
pub(crate) struct Trigger {
    toggle: Rc<dyn Fn()>,
    open: Rc<dyn Fn() -> bool>,
}

impl Trigger {
    /// Opens the overlay, or closes it if open, as a press on the trigger.
    pub(crate) fn toggle(&self) {
        (self.toggle)()
    }

    /// Whether the overlay is open.
    pub(crate) fn open(&self) -> bool {
        (self.open)()
    }
}

/// Gives what is below the overlay's `toggle` (a press on the trigger) and
/// `open` state. Call it in the overlay's body: the app's trigger, made in
/// JS, takes the context of the component it is in, not of where its slot
/// is placed. Its content then [`clear`]s it.
pub(crate) fn provide(toggle: impl Fn() + 'static, open: impl Fn() -> bool + 'static) {
    provide_context(Some(Trigger {
        toggle: Rc::new(toggle),
        open: Rc::new(open),
    }));
}

/// Ends the trigger for what is below: an overlay's content (its own
/// buttons are not its trigger).
pub(crate) fn clear() {
    provide_context(None::<Trigger>);
}

/// The overlay whose trigger slot the caller is in, if any.
pub(crate) fn current() -> Option<Trigger> {
    use_context::<Option<Trigger>>().flatten()
}
