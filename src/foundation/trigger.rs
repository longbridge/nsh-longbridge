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
//!
//! A container that looks different while an overlay inside it is open (a
//! `TableRow` holding an open menu; NSH has no `:has()`) counts them
//! ([`provide_open_inside`]): each overlay reports its open state to the
//! nearest one ([`report_open`]).

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
    let open: Rc<dyn Fn() -> bool> = Rc::new(open);
    let reported = open.clone();
    report_open(move || reported());
    provide_context(Some(Trigger {
        toggle: Rc::new(toggle),
        open,
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

/// How many overlays are open inside a container that shows it.
#[derive(Clone, Copy)]
struct OpenInside(Signal<u32>);

/// Counts, in `count`, the overlays below that are open (a `TableRow`
/// lights while one is). Call it in the container's body.
pub(crate) fn provide_open_inside(count: Signal<u32>) {
    provide_context(Some(OpenInside(count)));
}

/// Reports `open`, an overlay's open state, to the nearest container
/// counting open overlays ([`provide_open_inside`]), while the overlay is
/// mounted: nothing outside one. Call it in the overlay's body; overlays
/// with a trigger slot do through [`provide`].
pub(crate) fn report_open(open: impl Fn() -> bool + 'static) {
    let Some(OpenInside(count)) = use_context::<Option<OpenInside>>().flatten() else {
        return;
    };
    let counted = stored(false);
    effect(move || {
        let now = open();
        untracked(|| {
            if now != counted.get() {
                counted.replace(now);
                count.update(|n| {
                    *n = if now { *n + 1 } else { n.saturating_sub(1) };
                });
            }
        });
    });
    cleanup(move || {
        if counted.get() {
            count.update(|n| *n = n.saturating_sub(1));
        }
    });
}
