//! `Dialog`: shadcn/ui's dialog on `nsh/base`'s `Dialog` (the core's modal:
//! its layer above the window, the focus trap and its return, Escape and
//! back, screen readers). The library adds the open state, the trigger,
//! the overlay and the panel, their motion and the closing, as `nsh/base`'s
//! `Sheet` does on the same modal. `AlertDialog` is drawn by the same
//! parts. API: `docs/components/dialog.md`.

use std::rc::Rc;

use nsh_base::{OpenChangeReason, dialog as base_modal};
use nsh_core::{
    element::{Style, TextPartStyle},
    prelude::*,
};

use super::button::{ButtonEvent, ButtonProps, ButtonVariant, draw as draw_button};
use crate::{
    foundation::{
        focus::focus_ring,
        icon::{IconName, icon_view, provide_text_color},
        labels::{self, Label},
        motion, shadow,
        size::{Size, hit_slop},
        trigger,
        typography::{line_height, reset_inherited, semibold},
    },
    theme::{
        Theme,
        seeds::{DURATION, SPACING},
        semantic::{
            BACKGROUND, BORDER, BORDER_WIDTH, FOREGROUND, ICON_SIZE_MD, MUTED_FOREGROUND, OVERLAY,
            RADIUS_LG, RADIUS_XS, RING, RING_OPACITY, TEXT_LG, TEXT_SM,
        },
        theme,
    },
    tokens,
};

tokens! {
    /// The panel's fill (shadcn's `bg-background`).
    DIALOG: Color = "dialog" => |t| t.color(BACKGROUND),
    /// The panel's text.
    DIALOG_FOREGROUND: Color = "dialog-foreground" => |t| t.color(FOREGROUND),
    /// The panel's edge.
    DIALOG_BORDER: Color = "dialog-border" => |t| t.color(BORDER),
    /// The overlay that dims the page behind it.
    DIALOG_OVERLAY: Color = "dialog-overlay" => |t| t.color(OVERLAY),
    /// The panel's corner radius.
    DIALOG_RADIUS: Length = "dialog-radius" => |t| t.length(RADIUS_LG),
    /// The panel's shadow level (4: `lg`).
    DIALOG_SHADOW: Number = "dialog-shadow" => |t| t.value(4.0),
    /// The panel's padding (shadcn's `p-6`).
    DIALOG_PADDING: Length = "dialog-padding" => |t| t.length(SPACING) * 6.0,
    /// The gap between the panel's parts (shadcn's `gap-4`).
    DIALOG_GAP: Length = "dialog-gap" => |t| t.length(SPACING) * 4.0,
    /// The room the panel keeps from the window's sides.
    DIALOG_MARGIN: Length = "dialog-margin" => |t| t.length(SPACING) * 4.0,
    /// The panel's widest (shadcn's `sm:max-w-lg`).
    DIALOG_MAX_WIDTH: Length = "dialog-max-width" => |t| t.length(SPACING) * 128.0,
    /// The gap between a `DialogHeader`'s title and description.
    DIALOG_HEADER_GAP: Length = "dialog-header-gap" => |t| t.length(SPACING) * 2.0,
    /// The gap between a `DialogFooter`'s buttons.
    DIALOG_FOOTER_GAP: Length = "dialog-footer-gap" => |t| t.length(SPACING) * 2.0,
    /// The title's size: Tailwind's `text-lg`.
    DIALOG_TITLE_FONT_SIZE: Length = "dialog-title-font-size" => |t| t.length(TEXT_LG),
    /// The description's size: Tailwind's `text-sm`.
    DIALOG_DESCRIPTION_FONT_SIZE: Length = "dialog-description-font-size" => |t| t.length(TEXT_SM),
    /// The description's text.
    DIALOG_DESCRIPTION_FOREGROUND: Color = "dialog-description-foreground" => |t| t.color(MUTED_FOREGROUND),
    /// How far the close button sits from the panel's top and right edges.
    DIALOG_CLOSE_OFFSET: Length = "dialog-close-offset" => |t| t.length(SPACING) * 4.0,
    /// The close button's opacity at rest (1 under the pointer or focused).
    DIALOG_CLOSE_OPACITY: Number = "dialog-close-opacity" => |t| t.value(0.7),
    /// How long the overlay and the panel take to come and go, in
    /// milliseconds.
    DIALOG_DURATION: Number = "dialog-duration" => |t| t.number(DURATION) * 4.0 / 3.0,
    /// Whether dialogs fade and zoom in and out (1) or show and go in one
    /// frame (0).
    DIALOG_ANIMATE: Number = "dialog-animate" => |t| t.value(1.0),
}

/// The window width from which a dialog's header is start-aligned and its
/// footer a row (Tailwind's `sm`).
const WIDE: f32 = 640.0;

/// Whether the window is at least Tailwind's `sm` wide (tracked).
pub(crate) fn wide() -> bool {
    window().width >= px(WIDE)
}

/// The easing of a dialog's motion (Tailwind's, as `foundation::motion`).
const EASE: Easing = Easing::Bezier(0.4, 0.0, 0.2, 1.0);

// The open state, controlled or not, as `nsh/base`'s popups keep it.

/// Whether a layer is open: the app's `open` when it sets one, else its own
/// state (from `defaultOpen`).
#[derive(Clone, Copy)]
pub(crate) struct OpenState {
    controlled: Prop<Option<bool>>,
    own: Signal<bool>,
    /// What was last reported to an app that controls it, until its answer
    /// came: a second request before then is not reported again.
    asked: Stored<Option<bool>>,
    owner: Owner,
}

impl OpenState {
    pub(crate) fn new(open: Prop<Option<bool>>, default_open: Prop<bool>) -> Self {
        Self {
            controlled: open,
            own: signal(default_open.get_untracked()),
            asked: stored(None),
            owner: Owner::current(),
        }
    }

    /// Whether it is open (tracked).
    pub(crate) fn get(self) -> bool {
        self.controlled.get().unwrap_or_else(|| self.own.get())
    }

    pub(crate) fn get_untracked(self) -> bool {
        self.controlled
            .get_untracked()
            .unwrap_or_else(|| self.own.get_untracked())
    }

    /// The user asks for it to be `open`: when that is a change, `report`
    /// runs (the component emits `OpenChange`), then its own state follows
    /// when the app does not control it.
    pub(crate) fn request(self, open: bool, report: impl FnOnce()) {
        if self.asked.get().unwrap_or_else(|| self.get_untracked()) == open {
            return;
        }
        report();
        if self.controlled.get_untracked().is_none() {
            self.own.set(open);
        } else {
            self.asked.replace(Some(open));
            let asked = self.asked;
            self.owner.run(|| {
                after_answer(move || {
                    asked.replace(None);
                })
            });
        }
    }
}

/// A modal's presence and motion: mounted while open and while it goes;
/// `fade` runs 0 → 1 as it comes and 1 → 0 as it goes, over
/// `dialog-duration` (at once with reduced motion or `dialog-animate` 0).
#[derive(Clone, Copy)]
pub(crate) struct Presence {
    pub(crate) mounted: Signal<bool>,
    /// On its way out: it takes no presses.
    pub(crate) closing: Signal<bool>,
    pub(crate) fade: AnimatedValue,
    generation: Stored<u64>,
}

impl Presence {
    pub(crate) fn new(open: impl Fn() -> bool + 'static, t: Theme) -> Self {
        let presence = Self {
            mounted: signal(false),
            closing: signal(false),
            fade: animated_value(0.0),
            generation: stored(0),
        };
        let open = computed(open);
        effect(move || {
            let open = open.get();
            untracked(|| presence.follow(open, t));
        });
        presence
    }

    fn follow(self, open: bool, t: Theme) {
        let animate = t.number(DIALOG_ANIMATE) != 0.0 && !reduce_motion();
        let ms = t.number(DIALOG_DURATION).max(0.0);
        if open {
            let mounted = self.mounted.get_untracked();
            let closing = self.closing.get_untracked();
            if mounted && !closing {
                return;
            }
            self.next_change();
            if closing {
                // Opened again on its way out: back in from where it is.
                self.closing.set(false);
            } else {
                self.fade.set(if animate { 0.0 } else { 1.0 });
                self.mounted.set(true);
            }
            if animate {
                let _ = self.fade.timing(1.0, ms, EASE);
            } else {
                self.fade.set(1.0);
            }
        } else if self.mounted.get_untracked() && !self.closing.get_untracked() {
            let generation = self.next_change();
            if !animate {
                self.mounted.set(false);
                return;
            }
            self.closing.set(true);
            let finish = move || {
                if self.generation.get() == generation && self.closing.get_untracked() {
                    self.closing.set(false);
                    self.mounted.set(false);
                }
            };
            let end = self.fade.timing(0.0, ms, EASE);
            // Gone when the motion ends (the platform reports it), or after
            // its time, whichever is first.
            spawn(async move {
                if end.await {
                    finish();
                }
            });
            set_timeout(std::time::Duration::from_millis(ms as u64), finish);
        }
    }

    fn next_change(self) -> u64 {
        let next = self.generation.get().wrapping_add(1);
        self.generation.replace(next);
        next
    }
}

/// `open` once the layer is all there (the commit after the one that made
/// its content, so refs inside it name their views), `close` once it is
/// all gone, as `nsh/base`'s popups' lifecycle. A layer gone before it was
/// all there neither opens nor closes.
pub(crate) fn lifecycle(
    shown: impl Fn() -> bool + 'static,
    open: impl Fn() + 'static,
    close: impl Fn() + 'static,
) {
    let open = Rc::new(open);
    let generation = stored(0u64);
    let opened = stored(false);
    let was = stored(false);
    effect(move || {
        let now = shown();
        match (was.replace(now), now) {
            (false, true) => {
                let current = generation.get().wrapping_add(1);
                generation.replace(current);
                let open = open.clone();
                untracked(|| {
                    after_answer(move || {
                        if generation.get() == current {
                            opened.replace(true);
                            untracked(|| open());
                        }
                    })
                });
            }
            (true, false) => {
                generation.replace(generation.get().wrapping_add(1));
                if opened.replace(false) {
                    untracked(&close);
                }
            }
            _ => {}
        }
    });
}

/// What a `Dialog` (or `AlertDialog`) gives the parts inside it.
#[derive(Clone)]
pub(crate) struct DialogContext {
    /// Closes it with `reason` (`DialogClose`, the alert's buttons).
    pub(crate) close: Rc<dyn Fn(OpenChangeReason)>,
    /// An attempt to close it without choosing: the "×", which closes it.
    pub(crate) cancel: Rc<dyn Fn()>,
    /// A press on the overlay.
    pub(crate) backdrop: Rc<dyn Fn()>,
    pub(crate) presence: Presence,
    /// The `DialogTitle`'s text, its accessible name.
    pub(crate) title: Signal<Option<ElementRef>>,
    /// The `DialogDescription`'s text, its accessible description.
    pub(crate) description: Signal<Option<ElementRef>>,
    /// The buttons an `AlertDialog` focuses as it opens: its cancel
    /// (`true`), else its first action.
    pub(crate) answers: Signal<Vec<(ElementRef, bool)>>,
}

/// The dialog the current component is in.
pub(crate) fn dialog_context() -> Option<DialogContext> {
    use_context::<DialogContext>()
}

/// What a modal root reports: a `Dialog`'s or an `AlertDialog`'s events.
#[derive(Debug, Clone, Copy, PartialEq)]
pub(crate) enum RootEvent {
    OpenChange(bool, OpenChangeReason),
    Open,
    Close,
    Cancel,
}

/// The props a modal root takes (`Dialog`'s, `AlertDialog`'s).
pub(crate) struct RootProps {
    pub(crate) open: Prop<Option<bool>>,
    pub(crate) default_open: Prop<bool>,
    pub(crate) trigger: Prop<Option<Slot>>,
    pub(crate) close_on_escape: Prop<bool>,
    /// `None`: an alert's overlay never closes it.
    pub(crate) close_on_backdrop_press: Option<Prop<bool>>,
    pub(crate) accessibility_label: Prop<Option<SharedString>>,
    pub(crate) children: Prop<Children>,
}

/// A modal root: the open state, the trigger and the modal while open (or
/// going), with the context its parts read.
pub(crate) fn root(
    p: RootProps,
    alert: bool,
    emit: impl Fn(RootEvent) + 'static,
) -> impl IntoElement {
    let t = theme();
    let emit: Rc<dyn Fn(RootEvent)> = Rc::new(emit);
    let open = OpenState::new(p.open, p.default_open);
    let change: Rc<dyn Fn(bool, OpenChangeReason)> = {
        let emit = emit.clone();
        Rc::new(move |next: bool, reason: OpenChangeReason| {
            let emit = emit.clone();
            open.request(next, move || emit(RootEvent::OpenChange(next, reason)));
        })
    };
    let presence = Presence::new(move || open.get(), t);
    let title = signal(None::<ElementRef>);
    let description = signal(None::<ElementRef>);
    let answers = signal(Vec::<(ElementRef, bool)>::new());
    // The title's text, read once the content is there.
    let title_text = signal(None::<SharedString>);
    {
        let emit_open = emit.clone();
        let emit_close = emit.clone();
        let label = p.accessibility_label;
        lifecycle(
            move || presence.mounted.get(),
            move || {
                if let Some(title) = title.get_untracked() {
                    spawn(async move {
                        let text = title.text_content().await;
                        title_text.set((!text.is_empty()).then(|| text.into()));
                    });
                } else if label.get_untracked().is_none() && cfg!(debug_assertions) {
                    log::warn!(
                        "nsh-longbridge: a dialog without a title and without `accessibilityLabel` has no name for screen readers"
                    );
                }
                if alert {
                    // Radix's alert dialog focuses its Cancel, else its
                    // first action.
                    let answers = answers.get_untracked();
                    let target = answers
                        .iter()
                        .find(|(_, cancel)| *cancel)
                        .or_else(|| answers.first());
                    match target {
                        Some((target, _)) => target.focus(),
                        None if cfg!(debug_assertions) => log::warn!(
                            "nsh-longbridge: an AlertDialog without AlertDialogCancel or AlertDialogAction closes by Escape and the app only"
                        ),
                        None => {}
                    }
                }
                emit_open(RootEvent::Open);
            },
            move || emit_close(RootEvent::Close),
        );
    }
    let cancel: Rc<dyn Fn()> = {
        let (emit, change) = (emit.clone(), change.clone());
        Rc::new(move || {
            emit(RootEvent::Cancel);
            change(false, OpenChangeReason::Cancel);
        })
    };
    let backdrop: Rc<dyn Fn()> = {
        let (emit, change) = (emit.clone(), change.clone());
        let closes = p.close_on_backdrop_press;
        Rc::new(move || {
            if !open.get_untracked() {
                return;
            }
            emit(RootEvent::Cancel);
            if closes.is_some_and(|c| c.get_untracked()) {
                change(false, OpenChangeReason::BackdropPress);
            }
        })
    };
    let close: Rc<dyn Fn(OpenChangeReason)> = {
        let change = change.clone();
        Rc::new(move |reason| change(false, reason))
    };
    provide_context(DialogContext {
        close,
        cancel,
        backdrop,
        presence,
        title,
        description,
        answers,
    });
    // A library `Button` as the trigger takes the press itself.
    let one = trigger::OnePress::new();
    let toggle = change.clone();
    trigger::provide(
        move || {
            if one.first() {
                toggle(!open.get_untracked(), OpenChangeReason::TriggerPress)
            }
        },
        move || open.get(),
    );
    let trigger_change = change.clone();
    let escape_change = change;
    let escape_emit = emit;
    let label = p.accessibility_label;
    let close_on_escape = p.close_on_escape;
    let children = p.children;
    view()
        .child(p.trigger.place(move |trigger| {
            let change = trigger_change.clone();
            trigger
                .accessibility_state(move || AccessibilityState {
                    expanded: Some(open.get()),
                    has_popup: true,
                    ..AccessibilityState::default()
                })
                .on_press(move || {
                    if one.first() {
                        change(!open.get_untracked(), OpenChangeReason::TriggerPress)
                    }
                })
        }))
        .child(when(presence.mounted, move || {
            trigger::clear();
            let (change, emit) = (escape_change.clone(), escape_emit.clone());
            let modal = base_modal()
                .optional_label(move || label.get().or_else(|| title_text.get()))
                .accessibility_described_by(move || description.get())
                .status_bar_translucent(true)
                .animation(ModalAnimation::None)
                .close_on_escape(false)
                .close_on_swipe(false)
                .on_cancel(move || {
                    emit(RootEvent::Cancel);
                    if close_on_escape.get_untracked() {
                        change(false, OpenChangeReason::Cancel);
                    }
                })
                .child(children)
                // Toasts show above the topmost dialog.
                .child(crate::components::toast::modal_outlet());
            let modal = if alert {
                modal.role(AccessibilityRole::AlertDialog)
            } else {
                modal
            };
            vec![modal.into_element()]
        }))
}

/// The props of [`Dialog`].
#[derive(Debug, Clone, PartialEq, Props)]
pub struct DialogProps {
    /// Open (controlled): the app owns the value and changes it in
    /// `onOpenChange`. Left out, the dialog keeps its own state.
    pub open: Prop<Option<bool>>,
    /// Open at first, while `open` is not set.
    #[prop(default)]
    pub default_open: Prop<bool>,
    /// What opens it (a `Button`, usually): pressing it asks to open the
    /// dialog. It is `expanded` while the dialog is open.
    pub trigger: Prop<Option<Slot>>,
    /// Escape and the Android back button close it (default true); either
    /// way `onCancel` hears them.
    #[prop(default = true)]
    pub close_on_escape: Prop<bool>,
    /// A press on the overlay closes it (default true); either way
    /// `onCancel` hears it.
    #[prop(default = true)]
    pub close_on_backdrop_press: Prop<bool>,
    /// Its name for screen readers, when it has no `DialogTitle` or the
    /// title should not be its name; else the title's text.
    pub accessibility_label: Prop<Option<SharedString>>,
    /// Its content: usually one `DialogContent`, made on each opening.
    #[prop(lazy)]
    pub children: Prop<Children>,
}

/// The events of a [`Dialog`].
#[derive(Debug, Clone, PartialEq, Event)]
pub enum DialogEvent {
    /// The user asks to open or close it, with why: the trigger
    /// (`"triggerPress"`), Escape, back or the "×" (`"cancel"`), the
    /// overlay (`"backdropPress"`), a `DialogClose` (`"select"`). Set
    /// `open` to follow it. Not when the app changes `open`.
    #[args(open, reason)]
    OpenChange(bool, OpenChangeReason),
    /// It is all there: refs inside it name their views, so focusing one
    /// here moves the keyboard focus into it.
    Open,
    /// It is all gone, after its exit (whoever closed it).
    Close,
    /// The user tried to close it without choosing (Escape, back, the
    /// overlay, the "×"), whether or not it closed.
    Cancel,
}

impl From<RootEvent> for DialogEvent {
    fn from(e: RootEvent) -> Self {
        match e {
            RootEvent::OpenChange(open, reason) => DialogEvent::OpenChange(open, reason),
            RootEvent::Open => DialogEvent::Open,
            RootEvent::Close => DialogEvent::Close,
            RootEvent::Cancel => DialogEvent::Cancel,
        }
    }
}

/// A modal window over the app, shadcn/ui's: a panel in the middle of the
/// window over a dimmed overlay. It keeps the keyboard focus and screen
/// readers inside it until it closes, then gives the focus back. Escape,
/// back, the overlay, the "×" and a `DialogClose` close it. Its trigger is
/// a slot; it draws nothing else in place.
#[component]
pub fn Dialog(props: DialogProps, cx: Scope<DialogEvent>) -> impl IntoElement {
    root(
        RootProps {
            open: props.open,
            default_open: props.default_open,
            trigger: props.trigger,
            close_on_escape: props.close_on_escape,
            close_on_backdrop_press: Some(props.close_on_backdrop_press),
            accessibility_label: props.accessibility_label,
            children: props.children,
        },
        false,
        move |e| cx.emit(DialogEvent::from(e)),
    )
}

/// The props of [`DialogContent`].
#[derive(Debug, Clone, PartialEq, Props)]
pub struct DialogContentProps {
    /// The "×" button at the panel's top right (default true).
    #[prop(default = true)]
    pub show_close_button: Prop<bool>,
    /// Tailwind classes of the panel, after its own look
    /// (`max-w-[425px]`).
    pub class: Prop<Option<SharedString>>,
    /// The inline style of the panel, after its own look and classes.
    #[prop(default)]
    pub style: Prop<Style>,
    /// Tailwind classes of the overlay behind the panel.
    pub overlay_class: Prop<Option<SharedString>>,
    /// The inline style of the overlay.
    #[prop(default)]
    pub overlay_style: Prop<Style>,
    /// Tailwind classes of the "×" button.
    pub close_class: Prop<Option<SharedString>>,
    /// The inline style of the "×" button.
    #[prop(default)]
    pub close_style: Prop<Style>,
    /// The panel's content: `DialogHeader`, the body, `DialogFooter`.
    pub children: Prop<Children>,
}

/// A dialog's overlay and panel: put it in a `Dialog`, its parts in it.
/// Outside a `Dialog` it draws its panel in place (a preview of the look).
#[component]
pub fn DialogContent(props: DialogContentProps, _cx: Scope<NoEvent>) -> impl IntoElement {
    provide_text_color();
    let t = theme();
    let parts = Parts {
        class: props.class,
        style: props.style,
        overlay_class: props.overlay_class,
        overlay_style: props.overlay_style,
        close_class: props.close_class,
        close_style: props.close_style,
        children: props.children,
    };
    content(t, parts, props.show_close_button, move || {
        t.length(DIALOG_MAX_WIDTH)
    })
}

/// The styled parts of a dialog's content.
#[derive(Clone, Copy)]
pub(crate) struct Parts {
    pub(crate) class: Prop<Option<SharedString>>,
    pub(crate) style: Prop<Style>,
    pub(crate) overlay_class: Prop<Option<SharedString>>,
    pub(crate) overlay_style: Prop<Style>,
    pub(crate) close_class: Prop<Option<SharedString>>,
    pub(crate) close_style: Prop<Style>,
    pub(crate) children: Prop<Children>,
}

/// The overlay and the panel (`DialogContent`'s, `AlertDialogContent`'s):
/// the panel at most `max_width` wide, with a "×" while `close_button`.
pub(crate) fn content(
    t: Theme,
    parts: Parts,
    close_button: impl IntoReactive<bool> + 'static,
    max_width: impl Fn() -> Points + Copy + 'static,
) -> AnyElement {
    let close_button = close_button.into_reactive();
    // Outside a dialog (a preview, a page that shows the look): the panel
    // in place, its buttons closing nothing.
    let Some(d) = dialog_context() else {
        return panel(t, parts, max_width)
            .child(when(
                move || close_button.get(),
                move || close(t, parts, None),
            ))
            .into_element();
    };
    let fade = d.presence.fade;
    let closing = d.presence.closing;
    let takes = move || {
        if closing.get() {
            PointerEvents::None
        } else {
            PointerEvents::Auto
        }
    };
    let backdrop = d.backdrop.clone();
    let overlay_part = overlay()
        .inset_0()
        .focusable(false)
        .accessible(false)
        .bg(move || t.color(DIALOG_OVERLAY))
        .bind_opacity(&fade)
        .pointer_events(takes)
        .app_class(parts.overlay_class)
        .inline_style(move || parts.overlay_style.get())
        .on_press(move || backdrop());
    let scale = fade.interpolate(&[0.0, 1.0], &[0.95, 1.0]);
    let cancel = d.cancel.clone();
    let layer = overlay()
        .inset_0()
        .items_center()
        .justify_center()
        .px(move || t.length(DIALOG_MARGIN))
        // Centered in the room above a software keyboard.
        .pb(keyboard_height)
        .pointer_events(PointerEvents::BoxNone)
        .child(
            panel(t, parts, max_width)
                .bind_opacity(&fade)
                .bind_scale(&scale)
                .pointer_events(takes)
                .child(when(
                    move || close_button.get(),
                    move || close(t, parts, Some(cancel.clone())),
                )),
        );
    view().child(overlay_part).child(layer).into_element()
}

/// The panel: its look, then the app's classes and style.
fn panel(t: Theme, parts: Parts, max_width: impl Fn() -> Points + 'static) -> View {
    reset_inherited(view())
        .relative()
        .flex_col()
        .w_full()
        .max_w(max_width)
        .gap(move || t.length(DIALOG_GAP))
        .p(move || t.length(DIALOG_PADDING))
        .rounded(move || t.length(DIALOG_RADIUS))
        .border(move || t.length(BORDER_WIDTH))
        .border_color(move || t.color(DIALOG_BORDER))
        .bg(move || t.color(DIALOG))
        .text_color(move || t.color(DIALOG_FOREGROUND))
        .text_size(move || t.length(TEXT_SM))
        .line_height(move || line_height(t.length(TEXT_SM)))
        .style_with(move |s| {
            s.shadow(shadow::box_shadows(
                t,
                shadow::level(t.number(DIALOG_SHADOW)),
            ))
        })
        .app_class(parts.class)
        .inline_style(move || parts.style.get())
        .child(parts.children)
}

/// The "×" button: an icon at the panel's top right, named "Close".
fn close(t: Theme, parts: Parts, cancel: Option<Rc<dyn Fn()>>) -> impl IntoElement {
    let side = move || t.length(ICON_SIZE_MD);
    let offset = move || t.length(DIALOG_CLOSE_OFFSET);
    let el = nsh_base::button()
        .accessibility_label(move || Some(labels::text(Label::Close)))
        .absolute()
        .top(offset)
        .right(offset)
        .w(side)
        .h(side)
        .items_center()
        .justify_center()
        .rounded(move || t.length(RADIUS_XS))
        .text_color(move || t.color(DIALOG_FOREGROUND))
        .opacity(move || t.number(DIALOG_CLOSE_OPACITY))
        .hit_slop(move || hit_slop(t, side()))
        .cursor(Cursor::Default)
        .hover(|s| s.opacity(1.0))
        .focus_visible(move |s| {
            focus_ring(
                s.opacity(1.0),
                t,
                move || t.color(RING).opacity(t.number(RING_OPACITY)),
                None,
            )
        })
        .app_class(parts.close_class)
        .inline_style(move || parts.close_style.get())
        .on_press(move || {
            if let Some(cancel) = &cancel {
                cancel();
            }
        })
        .child(icon_view(t, || IconName::X, side));
    motion::transition_all(el, t)
}

/// The props of [`DialogHeader`].
#[derive(Debug, Clone, PartialEq, Props)]
pub struct DialogHeaderProps {
    /// Its parts: a `DialogTitle` and a `DialogDescription`.
    pub children: Prop<Children>,
}

/// A column holding a dialog's title and description: centered while the
/// window is narrower than 640, start-aligned from 640.
#[component]
pub fn DialogHeader(props: DialogHeaderProps, _cx: Scope<NoEvent>) -> impl IntoElement {
    let t = theme();
    view()
        .flex_col()
        .gap(move || t.length(DIALOG_HEADER_GAP))
        .style_with(|s| {
            if wide() {
                s.text_left()
            } else {
                s.text_center()
            }
        })
        .child(props.children)
}

/// The props of [`DialogTitle`].
#[derive(Debug, Clone, PartialEq, Props)]
pub struct DialogTitleProps {
    /// The title: text.
    pub children: Prop<Children>,
}

/// A dialog's title: its name for screen readers, and a heading.
#[component]
pub fn DialogTitle(props: DialogTitleProps, _cx: Scope<NoEvent>) -> impl IntoElement {
    let t = theme();
    let node = register_title();
    let font_size = move || t.length(DIALOG_TITLE_FONT_SIZE);
    text("")
        .element_ref(&node)
        .role(AccessibilityRole::Header)
        .text_size(font_size)
        // shadcn's `leading-none`.
        .line_height(font_size)
        .font_weight(semibold(t))
        .child(props.children)
}

/// The ref of the current dialog's title, registered with its dialog
/// while it is there.
pub(crate) fn register_title() -> ElementRef {
    let node = element_ref();
    if let Some(d) = dialog_context() {
        d.title.set(Some(node));
        cleanup(move || {
            if d.title.get_untracked() == Some(node) {
                d.title.set(None);
            }
        });
    }
    node
}

/// The ref of the current dialog's description, registered with its
/// dialog while it is there.
pub(crate) fn register_description() -> ElementRef {
    let node = element_ref();
    if let Some(d) = dialog_context() {
        d.description.set(Some(node));
        cleanup(move || {
            if d.description.get_untracked() == Some(node) {
                d.description.set(None);
            }
        });
    }
    node
}

/// The props of [`DialogDescription`].
#[derive(Debug, Clone, PartialEq, Props)]
pub struct DialogDescriptionProps {
    /// The description: text.
    pub children: Prop<Children>,
}

/// What a dialog is about, under its title: muted text, and the dialog's
/// description for screen readers.
#[component]
pub fn DialogDescription(props: DialogDescriptionProps, _cx: Scope<NoEvent>) -> impl IntoElement {
    let t = theme();
    let node = register_description();
    description(t, node).child(props.children)
}

/// A dialog's description text (`DialogDescription`'s,
/// `AlertDialogDescription`'s).
pub(crate) fn description(t: Theme, node: ElementRef) -> Text {
    let font_size = move || t.length(DIALOG_DESCRIPTION_FONT_SIZE);
    text("")
        .element_ref(&node)
        .text_size(font_size)
        .line_height(move || line_height(font_size()))
        .text_color(move || t.color(DIALOG_DESCRIPTION_FOREGROUND))
}

/// The props of [`DialogFooter`].
#[derive(Debug, Clone, PartialEq, Props)]
pub struct DialogFooterProps {
    /// Its buttons.
    pub children: Prop<Children>,
}

/// The row of a dialog's buttons: at the end from 640 wide; narrower, a
/// column with the first button at the bottom, each as wide as the panel.
#[component]
pub fn DialogFooter(props: DialogFooterProps, _cx: Scope<NoEvent>) -> impl IntoElement {
    let t = theme();
    view()
        .gap(move || t.length(DIALOG_FOOTER_GAP))
        .style_with(|s| {
            if wide() {
                s.flex_row().justify_end()
            } else {
                s.flex_col_reverse()
            }
        })
        .child(props.children)
}

/// The props of [`DialogClose`]: `Button`'s, `outline` by default.
#[derive(Debug, Clone, PartialEq, Props)]
pub struct DialogCloseProps {
    /// Its look (`"outline"` by default).
    #[prop(default = ButtonVariant::Outline)]
    pub variant: Prop<ButtonVariant>,
    /// Its size (`xs`, `sm`, `md`, `lg`).
    #[prop(default)]
    pub size: Prop<Size>,
    /// A square button showing its icon alone; give it an
    /// `accessibilityLabel`.
    #[prop(default)]
    pub icon_only: Prop<bool>,
    /// An icon before the content.
    pub icon: Prop<Option<IconName>>,
    /// An icon after the content.
    pub icon_end: Prop<Option<IconName>>,
    /// Its action is running: a spinner, no presses.
    #[prop(default)]
    pub loading: Prop<bool>,
    /// No presses, at half opacity.
    #[prop(default)]
    pub disabled: Prop<bool>,
    /// The app's checks reject it: the destructive edge and ring.
    #[prop(default)]
    pub invalid: Prop<bool>,
    /// What screen readers say, when the content is not enough.
    pub accessibility_label: Prop<Option<SharedString>>,
    /// What pressing it does, for screen readers.
    pub accessibility_hint: Prop<Option<SharedString>>,
    /// Tailwind classes of the label part.
    pub label_class: Prop<Option<SharedString>>,
    /// The inline style of the label part.
    #[prop(default)]
    pub label_style: Prop<TextPartStyle>,
    /// Tailwind classes of the icons.
    pub icon_class: Prop<Option<SharedString>>,
    /// The inline style of the icons.
    #[prop(default)]
    pub icon_style: Prop<Style>,
    /// The content: its label (a string), or anything else.
    pub children: Prop<Children>,
}

impl DialogCloseProps {
    /// As `Button`'s props.
    pub(crate) fn button(&self) -> ButtonProps {
        ButtonProps {
            variant: self.variant,
            size: self.size,
            icon_only: self.icon_only,
            icon: self.icon,
            icon_end: self.icon_end,
            loading: self.loading,
            disabled: self.disabled,
            invalid: self.invalid,
            accessibility_label: self.accessibility_label,
            accessibility_hint: self.accessibility_hint,
            label_class: self.label_class,
            label_style: self.label_style,
            icon_class: self.icon_class,
            icon_style: self.icon_style,
            children: self.children,
        }
    }
}

/// A `Button` (`outline` by default) that closes its dialog: a press asks
/// to close it (`reason` `"select"`), then reaches `onPress`.
#[component]
pub fn DialogClose(props: DialogCloseProps, cx: Scope<ButtonEvent>) -> impl IntoElement {
    let d = dialog_context();
    draw_button(props.button(), None, move || {
        if let Some(d) = &d {
            (d.close)(OpenChangeReason::Select);
        }
        cx.emit(ButtonEvent::Press);
    })
}
