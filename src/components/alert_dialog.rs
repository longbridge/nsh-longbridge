//! `AlertDialog`: shadcn/ui's alert dialog, a `Dialog` with an alert's
//! rules (role `alertdialog`, the overlay never closes it, no "×", the
//! focus on Cancel as it opens), drawn by `Dialog`'s parts. API:
//! `docs/components/alert-dialog.md`.

use nsh_base::OpenChangeReason;
use nsh_core::{element::Style, prelude::*};

use super::{
    button::{ButtonEvent, ButtonProps, draw as draw_button},
    dialog::{
        DIALOG_FOOTER_GAP, DIALOG_MAX_WIDTH, DIALOG_TITLE_FONT_SIZE, DialogCloseProps, Parts,
        RootEvent, RootProps, content, description, dialog_context, register_description,
        register_title, root, wide,
    },
};
use crate::{
    foundation::{
        icon::provide_icon_side,
        size::Size,
        typography::{line_height, semibold},
    },
    theme::{
        Theme,
        seeds::SPACING,
        semantic::{MUTED, RADIUS_MD},
        theme,
    },
    tokens,
};

tokens! {
    /// The panel's widest at size `sm` and `xs` (shadcn's `max-w-xs`).
    ALERT_DIALOG_MAX_WIDTH_SM: Length = "alert-dialog-max-width-sm" => |t| t.length(SPACING) * 80.0,
    /// The gap between the header's media, title and description
    /// (shadcn's `gap-1.5`).
    ALERT_DIALOG_HEADER_GAP: Length = "alert-dialog-header-gap" => |t| t.length(SPACING) * 1.5,
    /// The media tile's fill.
    ALERT_DIALOG_MEDIA: Color = "alert-dialog-media" => |t| t.color(MUTED),
    /// The media tile's side.
    ALERT_DIALOG_MEDIA_SIZE: Length = "alert-dialog-media-size" => |t| t.length(SPACING) * 16.0,
    /// The side of icons in the media tile.
    ALERT_DIALOG_MEDIA_ICON_SIZE: Length = "alert-dialog-media-icon-size" => |t| t.length(SPACING) * 8.0,
}

/// The props of [`AlertDialog`].
#[derive(Debug, Clone, PartialEq, Props)]
pub struct AlertDialogProps {
    /// Open (controlled): the app owns the value and changes it in
    /// `onOpenChange`. Left out, it keeps its own state.
    pub open: Prop<Option<bool>>,
    /// Open at first, while `open` is not set.
    #[prop(default)]
    pub default_open: Prop<bool>,
    /// What opens it (a `Button`, usually). It is `expanded` while the
    /// alert is open.
    pub trigger: Prop<Option<Slot>>,
    /// Escape and the Android back button close it (default true); either
    /// way `onCancel` hears them.
    #[prop(default = true)]
    pub close_on_escape: Prop<bool>,
    /// Its name for screen readers, when it has no `AlertDialogTitle`;
    /// else the title's text.
    pub accessibility_label: Prop<Option<SharedString>>,
    /// Its content: usually one `AlertDialogContent`, made on each opening.
    #[prop(lazy)]
    pub children: Prop<Children>,
}

/// The events of an [`AlertDialog`].
#[derive(Debug, Clone, PartialEq, Event)]
pub enum AlertDialogEvent {
    /// The user asks to open or close it, with why: the trigger
    /// (`"triggerPress"`), Escape, back or `AlertDialogCancel`
    /// (`"cancel"`), `AlertDialogAction` (`"select"`). Set `open` to follow
    /// it. Not when the app changes `open`.
    #[args(open, reason)]
    OpenChange(bool, OpenChangeReason),
    /// It is all there, the focus on its Cancel (else its first action).
    Open,
    /// It is all gone, after its exit (whoever closed it).
    Close,
    /// The user tried to close it without answering (Escape, back, a press
    /// on the overlay, which does not close it).
    Cancel,
}

impl From<RootEvent> for AlertDialogEvent {
    fn from(e: RootEvent) -> Self {
        match e {
            RootEvent::OpenChange(open, reason) => AlertDialogEvent::OpenChange(open, reason),
            RootEvent::Open => AlertDialogEvent::Open,
            RootEvent::Close => AlertDialogEvent::Close,
            RootEvent::Cancel => AlertDialogEvent::Cancel,
        }
    }
}

/// A modal dialog that asks an important question and waits for the
/// answer: a title, a sentence, a Cancel and an action. Only its buttons,
/// Escape and back close it; it opens with the focus on Cancel.
#[component]
pub fn AlertDialog(props: AlertDialogProps, cx: Scope<AlertDialogEvent>) -> impl IntoElement {
    root(
        RootProps {
            open: props.open,
            default_open: props.default_open,
            trigger: props.trigger,
            close_on_escape: props.close_on_escape,
            close_on_backdrop_press: None,
            accessibility_label: props.accessibility_label,
            children: props.children,
        },
        true,
        move |e| cx.emit(AlertDialogEvent::from(e)),
    )
}

/// The props of [`AlertDialogContent`].
#[derive(Debug, Clone, PartialEq, Props)]
pub struct AlertDialogContentProps {
    /// `sm` (and `xs`): a narrow alert, centered, its buttons side by side
    /// (shadcn's `size="sm"`); `md` (and `lg`): as wide as a dialog.
    #[prop(default)]
    pub size: Prop<Size>,
    /// Tailwind classes of the panel, after its own look.
    pub class: Prop<Option<SharedString>>,
    /// The inline style of the panel, after its own look and classes.
    #[prop(default)]
    pub style: Prop<Style>,
    /// Tailwind classes of the overlay behind the panel.
    pub overlay_class: Prop<Option<SharedString>>,
    /// The inline style of the overlay.
    #[prop(default)]
    pub overlay_style: Prop<Style>,
    /// The panel's content: `AlertDialogHeader` and `AlertDialogFooter`.
    pub children: Prop<Children>,
}

/// Whether the current alert is `sm` / `xs` (its header centered, its
/// footer two columns at every width).
#[derive(Clone, Copy)]
struct Small(Memo<bool>);

fn small() -> bool {
    use_context::<Small>().is_some_and(|Small(small)| small.get())
}

/// Whether the current alert's header is start-aligned (from 640 wide, at
/// size `md` / `lg`) rather than centered (tracked).
fn start_aligned() -> bool {
    wide() && !small()
}

/// An alert dialog's overlay and panel (`Dialog`'s, without the "×"): put
/// it in an `AlertDialog`, its parts in it.
#[component]
pub fn AlertDialogContent(props: AlertDialogContentProps, _cx: Scope<NoEvent>) -> impl IntoElement {
    let t = theme();
    let size = props.size;
    provide_context(Small(computed(move || {
        matches!(size.get(), Size::Xs | Size::Sm)
    })));
    let parts = Parts {
        class: props.class,
        style: props.style,
        overlay_class: props.overlay_class,
        overlay_style: props.overlay_style,
        close_class: Prop::new(None),
        close_style: Prop::new(Style::default()),
        children: props.children,
    };
    let max_width = move || {
        if matches!(size.get(), Size::Xs | Size::Sm) {
            t.length(ALERT_DIALOG_MAX_WIDTH_SM)
        } else {
            t.length(DIALOG_MAX_WIDTH)
        }
    };
    content(t, parts, false, max_width)
}

/// Whether the current header has an `AlertDialogMedia`.
#[derive(Clone, Copy)]
struct HasMedia(Signal<bool>);

fn has_media() -> bool {
    use_context::<HasMedia>().is_some_and(|HasMedia(media)| media.get())
}

/// Whether the media is a column on the left of the title and the
/// description (tracked).
fn media_beside() -> bool {
    start_aligned() && has_media()
}

/// The props of [`AlertDialogHeader`].
#[derive(Debug, Clone, PartialEq, Props)]
pub struct AlertDialogHeaderProps {
    /// Its parts: an `AlertDialogMedia`, an `AlertDialogTitle`, an
    /// `AlertDialogDescription`.
    pub children: Prop<Children>,
}

/// The alert's media, title and description: centered on a narrow window
/// or at size `sm`, else start-aligned with the media on the left.
#[component]
pub fn AlertDialogHeader(props: AlertDialogHeaderProps, _cx: Scope<NoEvent>) -> impl IntoElement {
    let t = theme();
    let media = signal(false);
    provide_context(HasMedia(media));
    view()
        .grid()
        .gap_y(move || t.length(ALERT_DIALOG_HEADER_GAP))
        .gap_x(move || {
            if media.get() {
                t.length(SPACING) * 6.0
            } else {
                px(0.0)
            }
        })
        .style_with(move |s| {
            let beside = start_aligned() && media.get();
            let s = if start_aligned() {
                s.text_left()
            } else {
                s.text_center()
            };
            if beside {
                // shadcn's `grid-rows-[auto_1fr]`; `minmax(0, 1fr)`, as
                // NSH 0.7.0 gives an `auto` row nothing next to a `1fr`
                // one an item spans (alert-dialog.md, NSH gaps 2).
                s.grid_template_columns("auto minmax(0, 1fr)")
                    .grid_template_rows("auto minmax(0, 1fr)")
            } else {
                s.grid_template_columns("minmax(0, 1fr)")
            }
        })
        .child(props.children)
}

/// The props of [`AlertDialogMedia`].
#[derive(Debug, Clone, PartialEq, Props)]
pub struct AlertDialogMediaProps {
    /// Its content: usually an `Icon`, which takes the media's icon size.
    pub children: Prop<Children>,
}

/// An icon tile above the title (centered) or on its left: decorative,
/// hidden from screen readers.
#[component]
pub fn AlertDialogMedia(props: AlertDialogMediaProps, _cx: Scope<NoEvent>) -> impl IntoElement {
    let t = theme();
    if let Some(HasMedia(media)) = use_context::<HasMedia>() {
        media.set(true);
        cleanup(move || media.set(false));
    }
    let side = move || t.length(ALERT_DIALOG_MEDIA_SIZE);
    provide_icon_side(move || t.length(ALERT_DIALOG_MEDIA_ICON_SIZE));
    let beside = computed(media_beside);
    view()
        .accessible(false)
        .accessibility_elements_hidden(true)
        .w(side)
        .h(side)
        .items_center()
        .justify_center()
        .rounded(move || t.length(RADIUS_MD))
        .bg(move || t.color(ALERT_DIALOG_MEDIA))
        .text_color(move || t.color(super::dialog::DIALOG_FOREGROUND))
        // Centered above the title, 8 above it; beside it, across its
        // and the description's rows.
        .mx(move || {
            if beside.get() {
                Length::from(px(0.0))
            } else {
                Length::Auto
            }
        })
        .mb(move || {
            if beside.get() {
                px(0.0)
            } else {
                t.length(SPACING) * 2.0
            }
        })
        .row_span(move || if beside.get() { 2u16 } else { 1u16 })
        .child(props.children)
}

/// In the column right of the media while it is beside them.
fn after_media<E: Styled>(el: E) -> E {
    el.style_with(|s| if media_beside() { s.col_start(2) } else { s })
}

/// The props of [`AlertDialogTitle`].
#[derive(Debug, Clone, PartialEq, Props)]
pub struct AlertDialogTitleProps {
    /// The question: text.
    pub children: Prop<Children>,
}

/// The alert's question: its name for screen readers, and a heading.
#[component]
pub fn AlertDialogTitle(props: AlertDialogTitleProps, _cx: Scope<NoEvent>) -> impl IntoElement {
    let t = theme();
    let node = register_title();
    let font_size = move || t.length(DIALOG_TITLE_FONT_SIZE);
    after_media(
        text("")
            .element_ref(&node)
            .role(AccessibilityRole::Header)
            .text_size(font_size)
            .line_height(move || line_height(font_size()))
            .font_weight(semibold(t)),
    )
    .child(props.children)
}

/// The props of [`AlertDialogDescription`].
#[derive(Debug, Clone, PartialEq, Props)]
pub struct AlertDialogDescriptionProps {
    /// The consequences: text.
    pub children: Prop<Children>,
}

/// What answering does, under the title: muted text, and the alert's
/// description for screen readers.
#[component]
pub fn AlertDialogDescription(
    props: AlertDialogDescriptionProps,
    _cx: Scope<NoEvent>,
) -> impl IntoElement {
    let t: Theme = theme();
    let node = register_description();
    after_media(description(t, node)).child(props.children)
}

/// The props of [`AlertDialogFooter`].
#[derive(Debug, Clone, PartialEq, Props)]
pub struct AlertDialogFooterProps {
    /// Its buttons: an `AlertDialogCancel`, then an `AlertDialogAction`.
    pub children: Prop<Children>,
}

/// The alert's buttons: at size `sm`, two equal columns; else a row at the
/// end from 640 wide, and narrower a column with the first button at the
/// bottom.
#[component]
pub fn AlertDialogFooter(props: AlertDialogFooterProps, _cx: Scope<NoEvent>) -> impl IntoElement {
    let t = theme();
    view()
        .gap(move || t.length(DIALOG_FOOTER_GAP))
        .style_with(|s| {
            if small() {
                s.grid().grid_cols(2)
            } else if wide() {
                s.flex().flex_row().justify_end()
            } else {
                s.flex().flex_col_reverse()
            }
        })
        .child(props.children)
}

/// Registers `node` as one of the current alert's answers (`cancel`: its
/// Cancel) while it is there.
fn register_answer(node: ElementRef, cancel: bool) {
    if let Some(d) = dialog_context() {
        d.answers.update(|a| a.push((node, cancel)));
        cleanup(move || d.answers.update(|a| a.retain(|(n, _)| *n != node)));
    }
}

/// A `Button` (`outline` by default) that closes the alert without acting:
/// a press asks to close it (`reason` `"cancel"`), then reaches `onPress`.
/// The alert opens with the focus on it.
#[component]
pub fn AlertDialogCancel(props: DialogCloseProps, cx: Scope<ButtonEvent>) -> impl IntoElement {
    let d = dialog_context();
    let node = element_ref();
    register_answer(node, true);
    draw_button(props.button(), Some(node), move || {
        if let Some(d) = &d {
            (d.close)(OpenChangeReason::Cancel);
        }
        cx.emit(ButtonEvent::Press);
    })
}

/// A `Button` (`default` by default; `destructive` for a delete) that
/// answers the alert: a press asks to close it (`reason` `"select"`), then
/// reaches `onPress`, where the app acts.
#[component]
pub fn AlertDialogAction(props: ButtonProps, cx: Scope<ButtonEvent>) -> impl IntoElement {
    let d = dialog_context();
    let node = element_ref();
    register_answer(node, false);
    draw_button(props, Some(node), move || {
        if let Some(d) = &d {
            (d.close)(OpenChangeReason::Select);
        }
        cx.emit(ButtonEvent::Press);
    })
}
