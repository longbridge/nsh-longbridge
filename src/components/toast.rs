//! `Toaster` and `toast()`: toasts, styled with the library's tokens,
//! on `nsh/base`'s toast pieces: its `ToastManager`
//! keeps the queue and the lifecycle, its `ToastStack` lays the toasts out
//! and moves them, its `Toast` is each toast's semantic root and swipe.
//! The library adds the queue behind `toast()` (a surface global), the
//! `Toaster` that draws it, the look, the motion classes and the answer.
//! While one of the library's dialogs is open, the topmost draws the stack
//! in its modal ([`modal_outlet`]). API: `docs/components/toast.md`.

use std::{
    cell::RefCell,
    collections::HashMap,
    future::Future,
    pin::Pin,
    rc::Rc,
    task::{Context, Poll, Waker},
    time::Duration,
};

use nsh_base::{
    Anchor, SwipeDirection, ToastManager, ToastMotion, ToastOptions as ManagerOptions,
    ToastStackState, ToastTransitionStatus, toast as base_toast, toast_stack,
};
use nsh_core::{
    element::{Style, TextPartStyle, global, module, try_global},
    prelude::*,
    time::now,
};

use super::button::{ButtonVariant, button};
use crate::{
    foundation::{
        icon::{IconName, icon_view, provide_text_color},
        labels::{self, Label},
        motion, shadow,
        size::{Size, hit_slop, pointer},
        typography::{line_height, medium, reset_inherited},
    },
    theme::{
        CapturedScope, Theme, capture_scope,
        seeds::SPACING,
        semantic::{
            ACCENT, BORDER, BORDER_WIDTH, MUTED_FOREGROUND, POPOVER, POPOVER_FOREGROUND, RADIUS_LG,
        },
        theme,
    },
    tokens,
};

tokens! {
    /// A toast's fill (`popover`).
    TOAST: Color = "toast" => |t| t.color(POPOVER),
    /// A toast's text.
    TOAST_FOREGROUND: Color = "toast-foreground" => |t| t.color(POPOVER_FOREGROUND),
    /// A toast's edge, and its "×"'s.
    TOAST_BORDER: Color = "toast-border" => |t| t.color(BORDER),
    /// A toast's description.
    TOAST_DESCRIPTION_FOREGROUND: Color = "toast-description-foreground" => |t| t.color(MUTED_FOREGROUND),
    /// A toast's corner radius (`radius-lg`).
    TOAST_RADIUS: Length = "toast-radius" => |t| t.length(RADIUS_LG),
    /// A toast's shadow level (3: `md`).
    TOAST_SHADOW: Number = "toast-shadow" => |t| t.value(3.0),
    /// The stack's width with a pointer on a wide window (356).
    TOAST_WIDTH: Length = "toast-width" => |t| t.px(356.0),
    /// A toast's padding.
    TOAST_PADDING: Length = "toast-padding" => |t| t.length(SPACING) * 4.0,
    /// The gap between a toast's icon, text and buttons.
    TOAST_GAP: Length = "toast-gap" => |t| t.length(SPACING) * 1.5,
    /// A toast's text size (13).
    TOAST_FONT_SIZE: Length = "toast-font-size" => |t| t.px(13.0),
    /// The side of a toast's kind icon.
    TOAST_ICON_SIZE: Length = "toast-icon-size" => |t| t.px(16.0),
    /// The side of a toast's "×".
    TOAST_CLOSE_SIZE: Length = "toast-close-size" => |t| t.px(20.0),
    /// How far the stack is from the window's edges (inside the safe area).
    TOAST_OFFSET: Length = "toast-offset" => |t| t.pointer_touch(t.length(SPACING) * 6.0, t.length(SPACING) * 4.0),
    /// How long a toast stays by default, in milliseconds.
    TOAST_DURATION: Number = "toast-duration" => |t| t.value(4000.0),
    /// Whether toasts slide in and out (1) or show and go in one frame (0).
    TOAST_ANIMATE: Number = "toast-animate" => |t| t.value(1.0),
}

/// Narrower than this, the stack spans the window (the phone layout).
const NARROW: f32 = 600.0;
/// How often the lifecycle advances while toasts are mounted.
const TICK: Duration = Duration::from_millis(50);

// What `toast()` shows.

/// The icon before a toast's text (`toast.success(…)` and the others).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default, PropEnum)]
pub enum ToastKind {
    /// No icon.
    #[default]
    Default,
    /// A check in a circle.
    Success,
    /// An "i" in a circle.
    Info,
    /// A warning triangle.
    Warning,
    /// An "x" in an octagon.
    Error,
    /// A spinner; it never times out.
    Loading,
}

impl ToastKind {
    fn icon(self) -> Option<IconName> {
        match self {
            ToastKind::Default => None,
            ToastKind::Success => Some(IconName::CircleCheck),
            ToastKind::Info => Some(IconName::Info),
            ToastKind::Warning => Some(IconName::TriangleAlert),
            ToastKind::Error => Some(IconName::OctagonX),
            ToastKind::Loading => Some(IconName::LoaderCircle),
        }
    }
}

/// A toast's button: what it says and what `toast()` answers when it is
/// pressed.
#[derive(Debug, Clone, Default, PartialEq, JsObject)]
pub struct ToastButton {
    /// What `toast()` resolves with (`{ action: id }`): the app's name for
    /// it, never the label.
    pub id: SharedString,
    /// What it says.
    pub label: SharedString,
}

/// The options of `toast(title, options)`.
#[derive(Debug, Clone, Default, PartialEq, JsObject)]
pub struct ToastOptions {
    /// Names the toast: showing one with the `id` of a toast still shown
    /// changes that toast in place and starts its time over.
    pub id: Option<SharedString>,
    /// A second line under the title.
    pub description: Option<SharedString>,
    /// The icon before the text (default `"default"`: none).
    #[prop(default)]
    pub kind: ToastKind,
    /// How long it stays, in milliseconds (`Infinity`: until dismissed);
    /// left out, the `Toaster`'s `duration`. Paused while the stack is
    /// expanded.
    pub duration: Option<f64>,
    /// A button after the text; a press closes the toast and answers its
    /// `id`.
    pub action: Option<ToastButton>,
    /// A secondary button before the action; a press closes the toast and
    /// answers its `id`.
    pub cancel: Option<ToastButton>,
    /// The "×" at its corner; left out, the `Toaster`'s `closeButton`.
    pub close_button: Option<bool>,
}

/// What `toast()` resolves with once the toast is gone.
#[derive(Debug, Clone, Default, PartialEq, JsObject)]
pub struct ToastResult {
    /// The `id` of the button that closed it, `null` when it went any other
    /// way (its time, a swipe, the "×", `dismissToast`).
    pub action: Option<SharedString>,
}

/// A toast as the queue keeps it.
#[derive(Debug, Clone, PartialEq)]
struct Shown {
    title: SharedString,
    options: ToastOptions,
}

/// Where a toast's answer goes, shared by every call that showed the same
/// `id`.
#[derive(Default)]
struct AnswerSlot {
    result: Option<ToastResult>,
    waker: Option<Waker>,
}

/// The answer of [`ToastBuilder::show`]: awaited, the button that closed
/// the toast, once it is gone. Dropping it does not dismiss the toast (a
/// toast shown for its message alone drops it).
pub struct ToastAnswer(Rc<RefCell<AnswerSlot>>);

/// [`ToastAnswer`] awaited.
pub struct ToastAnswerFuture(Rc<RefCell<AnswerSlot>>);

impl std::future::IntoFuture for ToastAnswer {
    type Output = ToastResult;
    type IntoFuture = ToastAnswerFuture;

    fn into_future(self) -> ToastAnswerFuture {
        ToastAnswerFuture(self.0)
    }
}

impl std::fmt::Debug for ToastAnswer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("ToastAnswer")
            .field(&self.0.borrow().result)
            .finish()
    }
}

impl Future for ToastAnswerFuture {
    type Output = ToastResult;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<ToastResult> {
        let mut slot = self.0.borrow_mut();
        match slot.result.clone() {
            Some(result) => Poll::Ready(result),
            None => {
                slot.waker = Some(cx.waker().clone());
                Poll::Pending
            }
        }
    }
}

// The queue: a surface global.

/// The surface's toasts, and what reads them.
struct Inner {
    manager: ToastManager<SharedString, Shown>,
    /// The pending answers, by toast.
    answers: HashMap<SharedString, Vec<Rc<RefCell<AnswerSlot>>>>,
    /// The button that closed a toast on its way out.
    chosen: HashMap<SharedString, SharedString>,
    /// The next toast id made up for a toast shown without one.
    next: u64,
    /// The `Toaster`'s default time, in milliseconds (`None`: until
    /// dismissed).
    duration: Option<f64>,
}

/// The surface's toast queue: the manager behind a version that tells
/// readers when what is mounted changed (the lifecycle's ticks that change
/// nothing notify no one).
#[derive(Clone)]
struct Queue {
    inner: Rc<RefCell<Inner>>,
    version: u64,
}

impl Default for Queue {
    fn default() -> Self {
        Self {
            inner: Rc::new(RefCell::new(Inner {
                manager: ToastManager::new(ToastMotion::sonner()),
                answers: HashMap::new(),
                chosen: HashMap::new(),
                next: 0,
                duration: Some(4000.0),
            })),
            version: 0,
        }
    }
}

/// Changes the queue with `f`; readers hear of it when `f` says it
/// changed what is mounted.
fn change<R>(f: impl FnOnce(&mut Inner) -> (R, bool)) -> R {
    let queue = global::<Queue>();
    let inner = queue.with_untracked(|q| q.inner.clone());
    let (result, changed) = f(&mut inner.borrow_mut());
    if changed {
        queue.update(|q| q.version += 1);
    }
    result
}

/// Delivers the answers of toasts that are gone.
fn answer(gone: Vec<(SharedString, Option<SharedString>)>, inner: &mut Inner) {
    for (id, action) in gone {
        for slot in inner.answers.remove(&id).unwrap_or_default() {
            let mut slot = slot.borrow_mut();
            slot.result = Some(ToastResult {
                action: action.clone(),
            });
            if let Some(waker) = slot.waker.take() {
                waker.wake();
            }
        }
    }
}

/// Shows (or changes in place) a toast; its answer.
fn show(title: SharedString, options: ToastOptions) -> ToastAnswer {
    let slot = Rc::new(RefCell::new(AnswerSlot::default()));
    let answer_slot = slot.clone();
    change(move |inner| {
        let id = options.id.clone().unwrap_or_else(|| {
            inner.next += 1;
            format!("toast-{}", inner.next).into()
        });
        let timeout = if options.kind == ToastKind::Loading {
            None
        } else {
            options
                .duration
                .or(inner.duration)
                .filter(|ms| ms.is_finite())
                .map(|ms| Duration::from_secs_f64(ms.max(0.0) / 1000.0))
        };
        // Shown again with its id: a button pressed on its way out no
        // longer closes it.
        inner.chosen.remove(&id);
        inner.manager.push(
            id.clone(),
            Shown { title, options },
            ManagerOptions::timeout(timeout),
            now(),
        );
        inner.answers.entry(id).or_default().push(answer_slot);
        ((), true)
    });
    ToastAnswer(slot)
}

/// Starts toast `id`'s exit, answered with `action`.
fn dismiss_with(id: &SharedString, action: Option<SharedString>) {
    change(|inner| {
        let changed = inner.manager.dismiss(id, now());
        if changed && let Some(action) = action {
            inner.chosen.insert(id.clone(), action);
        }
        ((), changed)
    });
}

/// Closes toast `id`, or every toast with `None`: their exits run, then
/// they answer `{ action: None }`.
pub fn dismiss_toast(id: Option<&str>) {
    match id {
        Some(id) => dismiss_with(&SharedString::from(id.to_owned()), None),
        None => change(|inner| ((), !inner.manager.dismiss_all(now()).is_empty())),
    }
}

/// Advances the lifecycle; whether toasts are left.
fn tick(paused: bool) -> bool {
    change(|inner| {
        let advance = inner.manager.advance(now(), paused);
        let gone = advance
            .removed
            .into_iter()
            .map(|(id, _)| {
                let action = inner.chosen.remove(&id);
                (id, action)
            })
            .collect();
        answer(gone, inner);
        (!inner.manager.is_empty(), advance.changed)
    })
}

/// A toast to show, set up by its methods and shown by
/// [`show`](Self::show) (TSX: `toast(title, options)`).
#[derive(Debug, Clone)]
#[must_use = "a toast shows only with `show()`"]
pub struct ToastBuilder {
    title: SharedString,
    options: ToastOptions,
}

/// A toast saying `title`: set it up, then [`show`](ToastBuilder::show)
/// it.
pub fn toast(title: impl Into<SharedString>) -> ToastBuilder {
    ToastBuilder {
        title: title.into(),
        options: ToastOptions::default(),
    }
}

impl ToastBuilder {
    /// Names it: a toast still shown with this `id` is changed in place.
    pub fn id(mut self, id: impl Into<SharedString>) -> Self {
        self.options.id = Some(id.into());
        self
    }

    /// A second line under the title.
    pub fn description(mut self, description: impl Into<SharedString>) -> Self {
        self.options.description = Some(description.into());
        self
    }

    /// Its icon.
    pub fn kind(mut self, kind: ToastKind) -> Self {
        self.options.kind = kind;
        self
    }

    /// How long it stays (`None`: until dismissed); left out, the
    /// `Toaster`'s.
    pub fn duration(mut self, duration: Option<Duration>) -> Self {
        self.options.duration = Some(duration.map_or(f64::INFINITY, |d| d.as_secs_f64() * 1000.0));
        self
    }

    /// A button after the text, answering `id`.
    pub fn action(mut self, id: impl Into<SharedString>, label: impl Into<SharedString>) -> Self {
        self.options.action = Some(ToastButton {
            id: id.into(),
            label: label.into(),
        });
        self
    }

    /// A secondary button before the action, answering `id`.
    pub fn cancel(mut self, id: impl Into<SharedString>, label: impl Into<SharedString>) -> Self {
        self.options.cancel = Some(ToastButton {
            id: id.into(),
            label: label.into(),
        });
        self
    }

    /// The "×" at its corner; left out, the `Toaster`'s `closeButton`.
    pub fn close_button(mut self, close_button: bool) -> Self {
        self.options.close_button = Some(close_button);
        self
    }

    /// Shows it now; its answer, to await in a task, is the `id` of the
    /// button that closed it once it is gone (`None` any other way).
    pub fn show(self) -> ToastAnswer {
        show(self.title, self.options)
    }
}

/// `toast()` and `dismissToast()` for TSX.
#[module]
pub(crate) struct Toasts;

#[module(flat)]
impl Toasts {
    /// Shows a toast saying `title` (a `Toaster` draws it) and resolves
    /// once it is gone with `{ action }`: the `id` of the button that
    /// closed it, or `null`. Showing one with the `id` of a toast still
    /// shown changes that toast.
    pub(crate) async fn toast(
        title: SharedString,
        options: Option<ToastOptions>,
    ) -> Result<ToastResult, JsError> {
        Ok(show(title, options.unwrap_or_default()).await)
    }

    /// Closes the toast `id`, or every toast without one; their exits run.
    pub(crate) fn dismiss_toast(id: Option<SharedString>) {
        dismiss_toast(id.as_deref());
    }
}

// Where the stack is drawn.

/// The `Toaster`'s drawing of the stack, which the topmost open dialog
/// borrows.
#[derive(Clone, Default)]
struct ToasterSlot {
    draw: Option<Rc<dyn Fn() -> AnyElement>>,
    /// The `Toaster`s mounted (one draws).
    count: u32,
}

/// The library's open modals, newest last: the last draws the stack.
#[derive(Clone, Default)]
struct OpenModals {
    open: Vec<u64>,
    next: u64,
}

/// Where a library modal draws the stack while it is the topmost: put it
/// last in the modal's children.
pub(crate) fn modal_outlet() -> impl IntoElement {
    let modals = global::<OpenModals>();
    let id = modals.with_untracked(|m| m.next);
    modals.update(|m| {
        m.next += 1;
        m.open.push(id);
    });
    cleanup(move || {
        if let Some(modals) = try_global::<OpenModals>() {
            modals.update(|m| m.open.retain(|o| *o != id));
        }
    });
    let slot = global::<ToasterSlot>();
    let topmost = move || modals.with(|m| m.open.last() == Some(&id));
    let draw = move || slot.with(|s| s.draw.clone());
    when(
        move || topmost() && draw().is_some(),
        move || draw().map(|draw| vec![draw()]).unwrap_or_default(),
    )
}

// The `Toaster`.

/// Where the stack is.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default, PropEnum)]
pub enum ToastPlacement {
    /// The top left corner.
    TopLeft,
    /// The middle of the top edge.
    TopCenter,
    /// The top right corner.
    TopRight,
    /// The bottom left corner.
    BottomLeft,
    /// The middle of the bottom edge.
    BottomCenter,
    /// The bottom right corner.
    #[default]
    BottomRight,
}

impl ToastPlacement {
    fn anchor(self) -> Anchor {
        match self {
            ToastPlacement::TopLeft => Anchor::TopLeft,
            ToastPlacement::TopCenter => Anchor::TopCenter,
            ToastPlacement::TopRight => Anchor::TopRight,
            ToastPlacement::BottomLeft => Anchor::BottomLeft,
            ToastPlacement::BottomCenter => Anchor::BottomCenter,
            ToastPlacement::BottomRight => Anchor::BottomRight,
        }
    }

    fn top(self) -> bool {
        matches!(
            self,
            ToastPlacement::TopLeft | ToastPlacement::TopCenter | ToastPlacement::TopRight
        )
    }

    /// The direction a toast is swiped away in: the placement's side, down
    /// or up in the middle and on touch screens.
    fn swipe(self, touch: bool) -> SwipeDirection {
        match (self, touch) {
            (p, true) | (p @ (ToastPlacement::TopCenter | ToastPlacement::BottomCenter), _) => {
                if p.top() {
                    SwipeDirection::Up
                } else {
                    SwipeDirection::Down
                }
            }
            (ToastPlacement::TopLeft | ToastPlacement::BottomLeft, _) => SwipeDirection::Left,
            _ => SwipeDirection::Right,
        }
    }
}

/// The props of [`Toaster`].
#[derive(Debug, Clone, Copy, PartialEq, Props)]
pub struct ToasterProps {
    /// Where the stack is (default `"bottomRight"`); on touch screens and
    /// windows narrower than 600 it spans the window at that edge.
    #[prop(default)]
    pub placement: Prop<ToastPlacement>,
    /// How many toasts are mounted at once (default 3); older ones wait.
    #[prop(default = 3)]
    pub visible_toasts: Prop<u32>,
    /// The time a toast stays by default, in milliseconds; left out,
    /// `toast-duration` (4000).
    pub duration: Prop<Option<f64>>,
    /// Every toast shows the "×" (default false).
    #[prop(default)]
    pub close_button: Prop<bool>,
    /// Tailwind classes of the stack.
    pub class: Prop<Option<SharedString>>,
    /// The inline style of the stack.
    #[prop(default)]
    pub style: Prop<Style>,
    /// Tailwind classes of each toast's box.
    pub toast_class: Prop<Option<SharedString>>,
    /// The inline style of each toast's box.
    #[prop(default)]
    pub toast_style: Prop<Style>,
    /// Tailwind classes of the kind's icon.
    pub icon_class: Prop<Option<SharedString>>,
    /// The inline style of the kind's icon.
    #[prop(default)]
    pub icon_style: Prop<Style>,
    /// Tailwind classes of the title.
    pub title_class: Prop<Option<SharedString>>,
    /// The inline style of the title.
    #[prop(default)]
    pub title_style: Prop<TextPartStyle>,
    /// Tailwind classes of the description.
    pub description_class: Prop<Option<SharedString>>,
    /// The inline style of the description.
    #[prop(default)]
    pub description_style: Prop<TextPartStyle>,
    /// Tailwind classes of the action button.
    pub action_class: Prop<Option<SharedString>>,
    /// The inline style of the action button.
    #[prop(default)]
    pub action_style: Prop<Style>,
    /// Tailwind classes of the cancel button.
    pub cancel_class: Prop<Option<SharedString>>,
    /// The inline style of the cancel button.
    #[prop(default)]
    pub cancel_style: Prop<Style>,
    /// Tailwind classes of the "×".
    pub close_class: Prop<Option<SharedString>>,
    /// The inline style of the "×".
    #[prop(default)]
    pub close_style: Prop<Style>,
}

/// Draws the toasts `toast()` shows: place one at the app's root. Brief
/// messages at a corner of the window that stack and go by themselves;
/// while one of the library's dialogs is open, they show above it.
#[component]
pub fn Toaster(props: ToasterProps, _cx: Scope<NoEvent>) -> impl IntoElement {
    provide_text_color();
    let slot = global::<ToasterSlot>();
    let first = slot.with_untracked(|s| s.count == 0);
    slot.update(|s| s.count += 1);
    cleanup(move || {
        if let Some(slot) = try_global::<ToasterSlot>() {
            slot.update(|s| {
                s.count -= 1;
                if first {
                    s.draw = None;
                }
            });
        }
    });
    if !first {
        if cfg!(debug_assertions) {
            log::warn!(
                "nsh-longbridge: a second `Toaster` draws nothing: place one at the app's root"
            );
        }
        return view().into_element();
    }
    let t = theme();
    // The default time, for toasts shown from now on.
    effect(move || {
        let ms = props
            .duration
            .get()
            .unwrap_or_else(|| f64::from(t.number(TOAST_DURATION)));
        untracked(|| change(|inner| ((inner.duration = Some(ms)), false)));
    });
    // The lifecycle advances while toasts are there, paused while the
    // stack is expanded.
    let state = ToastStackState::new();
    let queue = global::<Queue>();
    let any = computed(move || queue.with(|q| !q.inner.borrow().manager.is_empty()));
    let owner = Owner::current();
    let ticking = stored(None::<Spawned<()>>);
    effect(move || {
        let on = any.get();
        untracked(|| match (on, ticking.replace(None)) {
            (true, None) => {
                let ticker = owner.run(|| {
                    set_interval(TICK, move || {
                        tick(state.is_expanded());
                    })
                });
                ticking.replace(Some(ticker));
            }
            (true, Some(ticker)) => {
                ticking.replace(Some(ticker));
            }
            (false, Some(ticker)) => ticker.cancel(),
            (false, None) => {}
        });
    });
    let scope = capture_scope();
    let draw: Rc<dyn Fn() -> AnyElement> =
        Rc::new(move || stack(props, scope, state).into_element());
    slot.update(|s| s.draw = Some(draw.clone()));
    let modals = global::<OpenModals>();
    view()
        .child(when(
            move || modals.with(|m| m.open.is_empty()),
            move || vec![draw()],
        ))
        .into_element()
}

/// The stack where it is drawn: an overlay at the placement's edge, the
/// `Toaster`'s scope provided.
fn stack(props: ToasterProps, scope: CapturedScope, state: ToastStackState) -> impl IntoElement {
    scope.provide();
    let t = theme();
    let queue = global::<Queue>();
    let shown = computed(move || {
        let limit = props.visible_toasts.get().max(1) as usize;
        queue.with(|q| {
            q.inner
                .borrow()
                .manager
                .visible(limit)
                .map(|(id, toast, status)| (id.clone(), toast.clone(), status))
                .collect::<Vec<_>>()
        })
    });
    // The newest toast not going: the front layer of a collapsed stack.
    let front = computed(move || {
        shown.with(|v| {
            v.iter()
                .rev()
                .find(|(_, _, status)| *status != ToastTransitionStatus::Ending)
                .map(|(id, _, _)| id.clone())
        })
    });
    let placement = move || props.placement.get();
    let spans = move || !pointer() || window().width < px(NARROW);
    let offset = move || t.length(TOAST_OFFSET);
    let inset = move || safe_area();
    overlay()
        .inset_0()
        .focusable(false)
        .pointer_events(PointerEvents::BoxNone)
        .flex_col()
        .pt(move || offset() + inset().top)
        .pb(move || offset() + inset().bottom)
        .pl(move || offset() + inset().left)
        .pr(move || offset() + inset().right)
        .style_with(move |s| {
            let p = placement();
            let s = if p.top() {
                s.justify_start()
            } else {
                s.justify_end()
            };
            if spans() {
                s.items_stretch()
            } else {
                match p {
                    ToastPlacement::TopLeft | ToastPlacement::BottomLeft => s.items_start(),
                    ToastPlacement::TopCenter | ToastPlacement::BottomCenter => s.items_center(),
                    _ => s.items_end(),
                }
            }
        })
        .child(
            toast_stack()
                .placement(move || placement().anchor())
                .state(state)
                .close_on_escape(false)
                .style_with(move |s| {
                    if spans() {
                        s.w_full()
                    } else {
                        s.w(t.length(TOAST_WIDTH))
                    }
                })
                .app_class(props.class)
                .inline_style(move || props.style.get())
                .child(each(
                    move || shown.get(),
                    |(id, _, _)| id.clone(),
                    move |item, _| one(t, props, state, front, move || item.get()),
                )),
        )
}

/// What screen readers read as a toast shows.
fn announcement(toast: &Shown) -> SharedString {
    match &toast.options.description {
        Some(description) => format!("{}. {description}", toast.title).into(),
        None => toast.title.clone(),
    }
}

type Item = (SharedString, Shown, ToastTransitionStatus);

/// One toast: `nsh/base`'s `Toast` carrying the motion and the swipe, the
/// library's box inside it.
fn one(
    t: Theme,
    props: ToasterProps,
    state: ToastStackState,
    front: Memo<Option<SharedString>>,
    item: impl Fn() -> Item + Copy + 'static,
) -> impl IntoElement {
    let id = untracked(|| item().0);
    // Behind the front layer of a collapsed stack: its content hidden, so
    // the layers read as cards behind the newest.
    let behind_id = id.clone();
    let behind =
        computed(move || !state.is_expanded() && front.with(|f| f.as_ref() != Some(&behind_id)));
    let toast = move || item().1;
    let animate = move || t.number(TOAST_ANIMATE) != 0.0 && !reduce_motion();
    let top = move || props.placement.get().top();
    let away = move || {
        if top() {
            "opacity-0 -translate-y-full"
        } else {
            "opacity-0 translate-y-full"
        }
    };
    let dismiss_id = id.clone();
    base_toast()
        .transition_status(move || item().2)
        .accessibility_label(move || Some(announcement(&toast())))
        .swipe_direction(move || Some(props.placement.get().swipe(!pointer())))
        .on_dismiss_request(move || dismiss_with(&dismiss_id, None))
        .starting_class(move || animate().then(|| SharedString::from(away())))
        .ending_class(move || {
            animate().then(|| SharedString::from(format!("{} duration-200", away())))
        })
        .swiping_class(move || Some(SharedString::from("transition-none")))
        .transition_all()
        .duration(400.0)
        .child(boxed(t, props, id, behind, toast))
}

/// `el` hidden while its toast is `behind` the front layer.
fn veiled<E: Styled>(el: E, behind: Memo<bool>) -> E {
    el.opacity(move || if behind.get() { 0.0_f32 } else { 1.0 })
        .transition_opacity()
        .duration(400.0)
}

/// A toast's box: its look, the "×", the icon, the text and the buttons.
fn boxed(
    t: Theme,
    props: ToasterProps,
    id: SharedString,
    behind: Memo<bool>,
    toast: impl Fn() -> Shown + Copy + 'static,
) -> impl IntoElement {
    let font_size = move || t.length(TOAST_FONT_SIZE);
    let close_shown = move || {
        toast()
            .options
            .close_button
            .unwrap_or_else(|| props.close_button.get())
    };
    let kind = move || toast().options.kind;
    let (close_id, cancel_id, action_id) = (id.clone(), id.clone(), id);
    let el = reset_inherited(view())
        .relative()
        .flex_row()
        .items_center()
        .gap(move || t.length(TOAST_GAP))
        .p(move || t.length(TOAST_PADDING))
        .w_full()
        .rounded(move || t.length(TOAST_RADIUS))
        .border(move || t.length(BORDER_WIDTH))
        .border_color(move || t.color(TOAST_BORDER))
        .bg(move || t.color(TOAST))
        .text_color(move || t.color(TOAST_FOREGROUND))
        .text_size(font_size)
        .line_height(move || line_height(font_size()))
        .style_with(move |s| {
            s.shadow(shadow::box_shadows(
                t,
                shadow::level(t.number(TOAST_SHADOW)),
            ))
        })
        .app_class(props.toast_class)
        .inline_style(move || props.toast_style.get());
    el.child(when(close_shown, move || {
        vec![close(t, props, close_id.clone(), behind).into_element()]
    }))
    .child(when(
        move || kind().icon().is_some(),
        move || {
            let side = move || t.length(TOAST_ICON_SIZE);
            let icon = icon_view(t, move || kind().icon().unwrap_or(IconName::Info), side);
            let icon = if kind() == ToastKind::Loading {
                motion::spin(icon)
            } else {
                icon
            };
            vec![
                veiled(icon, behind)
                    .flex_shrink_0()
                    .ml(px(-3.0))
                    .mr(px(4.0))
                    .app_class(props.icon_class)
                    .inline_style(move || props.icon_style.get())
                    .into_element(),
            ]
        },
    ))
    .child(
        veiled(view(), behind)
            .flex_col()
            .flex_1()
            .min_w_0()
            .gap(px(2.0))
            .child(
                text(move || toast().title)
                    .font_weight(medium(t))
                    .app_class(props.title_class)
                    .inline_style(move || Style::from(props.title_style.get())),
            )
            .child(when(
                move || toast().options.description.is_some(),
                move || {
                    vec![
                        text(move || toast().options.description.unwrap_or_default())
                            .text_color(move || t.color(TOAST_DESCRIPTION_FOREGROUND))
                            .app_class(props.description_class)
                            .inline_style(move || Style::from(props.description_style.get()))
                            .into_element(),
                    ]
                },
            )),
    )
    .child(when(
        move || toast().options.cancel.is_some(),
        move || {
            let id = cancel_id.clone();
            vec![
                veiled(button(), behind)
                    .size(Size::Xs)
                    .variant(ButtonVariant::Secondary)
                    .on_press(move || {
                        let answer = untracked(toast).options.cancel.map(|c| c.id);
                        dismiss_with(&id, answer);
                    })
                    .app_class(props.cancel_class)
                    .inline_style(move || props.cancel_style.get())
                    .child(text(move || {
                        toast().options.cancel.map(|c| c.label).unwrap_or_default()
                    }))
                    .into_element(),
            ]
        },
    ))
    .child(when(
        move || toast().options.action.is_some(),
        move || {
            let id = action_id.clone();
            vec![
                veiled(button(), behind)
                    .size(Size::Xs)
                    .on_press(move || {
                        let answer = untracked(toast).options.action.map(|a| a.id);
                        dismiss_with(&id, answer);
                    })
                    .app_class(props.action_class)
                    .inline_style(move || props.action_style.get())
                    .child(text(move || {
                        toast().options.action.map(|a| a.label).unwrap_or_default()
                    }))
                    .into_element(),
            ]
        },
    ))
}

/// A toast's "×": a circle on its top-start corner, moved out by 35%.
fn close(t: Theme, props: ToasterProps, id: SharedString, behind: Memo<bool>) -> impl IntoElement {
    let side = move || t.length(TOAST_CLOSE_SIZE);
    let out = move || side() * -0.35;
    let el = nsh_base::button()
        .accessibility_label(move || Some(labels::text(Label::Close)))
        .absolute()
        .top(out)
        .left(out)
        .w(side)
        .h(side)
        .items_center()
        .justify_center()
        .rounded_full()
        .border(move || t.length(BORDER_WIDTH))
        .border_color(move || t.color(TOAST_BORDER))
        .bg(move || t.color(TOAST))
        .text_color(move || t.color(TOAST_FOREGROUND))
        .hit_slop(move || hit_slop(t, side()))
        .cursor(Cursor::Default)
        .hover(move |s| s.bg(move || t.color(ACCENT)))
        .app_class(props.close_class)
        .inline_style(move || props.close_style.get())
        .on_press(move || dismiss_with(&id, None))
        .opacity(move || if behind.get() { 0.0_f32 } else { 1.0 })
        .child(icon_view(t, || IconName::X, || px(12.0)));
    motion::transition_all(el, t)
}
