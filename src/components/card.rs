//! `Card` and its parts: a card, a bordered surface laid out as
//! a header (title, description, action), content and footer. No behavior.
//! API: `docs/components/card.md`.

use nsh_core::prelude::*;

use crate::{
    foundation::{icon::provide_text_color, shadow, typography::line_height, typography::semibold},
    theme::{
        seeds::SPACING,
        semantic::{BORDER, BORDER_WIDTH, CARD, CARD_FOREGROUND, MUTED_FOREGROUND, RADIUS_XL},
        theme,
    },
    tokens,
};

tokens! {
    /// The card's edge.
    CARD_BORDER: Color = "card-border" => |t| t.color(BORDER),
    /// The description's text.
    CARD_DESCRIPTION_FOREGROUND: Color = "card-description-foreground" => |t| t.color(MUTED_FOREGROUND),
    /// The card's corner radius.
    CARD_RADIUS: Length = "card-radius" => |t| t.length(RADIUS_XL),
    /// The card's shadow level (2: `sm`).
    CARD_SHADOW: Number = "card-shadow" => |t| t.value(2.0),
    /// The card's padding: above the first part and below the last, and on
    /// both sides of each part.
    CARD_PADDING: Length = "card-padding" => |t| t.length(SPACING) * 6.0,
    /// The gap between the card's parts.
    CARD_GAP: Length = "card-gap" => |t| t.length(SPACING) * 6.0,
    /// The gap between the title and the description (and the action).
    CARD_HEADER_GAP: Length = "card-header-gap" => |t| t.length(SPACING) * 2.0,
    /// The title's text size: Tailwind's `text-base`.
    CARD_TITLE_FONT_SIZE: Length = "card-title-font-size" => |t| t.px(16.0),
    /// The description's text size: Tailwind's `text-sm`.
    CARD_DESCRIPTION_FONT_SIZE: Length = "card-description-font-size" => |t| t.px(14.0),
}

/// What a `Card` tells what is inside it: a `Calendar` there drops its
/// fill, the card's surface showing through.
#[derive(Debug, Clone, Copy)]
pub(crate) struct InCard;

/// Whether the current component is inside a library `Card`.
pub(crate) fn in_card() -> bool {
    use_context::<InCard>().is_some()
}

/// What a `CardHeader` gives its parts: how many `CardAction`s it holds, so
/// it makes a column for them.
#[derive(Debug, Clone, Copy)]
struct HeaderActions(Signal<u32>);

/// The props of [`Card`].
#[derive(Debug, Clone, PartialEq, Props)]
pub struct CardProps {
    /// Its parts: `CardHeader`, `CardContent`, `CardFooter`, in order.
    pub children: Prop<Children>,
}

/// A card: a bordered surface with a light shadow, laid out
/// as `CardHeader` (`CardTitle`, `CardDescription`, `CardAction`),
/// `CardContent` and `CardFooter`. Not interactive: the controls inside
/// work as usual.
#[component]
pub fn Card(props: CardProps, _cx: Scope<NoEvent>) -> impl IntoElement {
    provide_text_color();
    provide_context(InCard);
    let t = theme();
    view()
        .flex_col()
        .gap(move || t.length(CARD_GAP))
        .py(move || t.length(CARD_PADDING))
        .rounded(move || t.length(CARD_RADIUS))
        .border(move || t.length(BORDER_WIDTH))
        .border_color(move || t.color(CARD_BORDER))
        .bg(move || t.color(CARD))
        .text_color(move || t.color(CARD_FOREGROUND))
        .inline_style(move || shadow::shadow(t, shadow::level(t.number(CARD_SHADOW))))
        .child(props.children)
}

/// The props of [`CardHeader`].
#[derive(Debug, Clone, PartialEq, Props)]
pub struct CardHeaderProps {
    /// Its parts: `CardTitle`, `CardDescription`, `CardAction`.
    pub children: Prop<Children>,
}

/// A card's title area: the title over the description, and an action at
/// the top right when it holds a `CardAction`. For a divider below it, give
/// it `class="border-b pb-6"`.
#[component]
pub fn CardHeader(props: CardHeaderProps, _cx: Scope<NoEvent>) -> impl IntoElement {
    let t = theme();
    let actions = signal(0u32);
    provide_context(HeaderActions(actions));
    view()
        .grid()
        .grid_template_rows("auto auto")
        .grid_template_columns(move || {
            SharedString::from(if actions.get() > 0 { "1fr auto" } else { "1fr" })
        })
        .items_start()
        .gap(move || t.length(CARD_HEADER_GAP))
        .px(move || t.length(CARD_PADDING))
        .child(props.children)
}

/// The props of [`CardTitle`].
#[derive(Debug, Clone, PartialEq, Props)]
pub struct CardTitleProps {
    /// The title: text.
    pub children: Prop<Children>,
}

/// A card's title: semibold, a heading for screen readers.
#[component]
pub fn CardTitle(props: CardTitleProps, _cx: Scope<NoEvent>) -> impl IntoElement {
    let t = theme();
    let font_size = move || t.length(CARD_TITLE_FONT_SIZE);
    text("")
        .role(AccessibilityRole::Header)
        .text_size(font_size)
        // `leading-none`: the line exactly as tall as the text.
        .line_height(font_size)
        .font_weight(semibold(t))
        .child(props.children)
}

/// The props of [`CardDescription`].
#[derive(Debug, Clone, PartialEq, Props)]
pub struct CardDescriptionProps {
    /// The description: text.
    pub children: Prop<Children>,
}

/// A card's description, under its title: smaller, muted text.
#[component]
pub fn CardDescription(props: CardDescriptionProps, _cx: Scope<NoEvent>) -> impl IntoElement {
    let t = theme();
    let font_size = move || t.length(CARD_DESCRIPTION_FONT_SIZE);
    text("")
        .text_size(font_size)
        .line_height(move || line_height(font_size()))
        .text_color(move || t.color(CARD_DESCRIPTION_FOREGROUND))
        .child(props.children)
}

/// The props of [`CardAction`].
#[derive(Debug, Clone, PartialEq, Props)]
pub struct CardActionProps {
    /// The action: a button, a menu's trigger.
    pub children: Prop<Children>,
}

/// A card's action, at the top right of its `CardHeader`, beside the title
/// and the description. Only inside a `CardHeader`.
#[component]
pub fn CardAction(props: CardActionProps, _cx: Scope<NoEvent>) -> impl IntoElement {
    match use_context::<HeaderActions>() {
        Some(HeaderActions(actions)) => {
            actions.update(|n| *n += 1);
            cleanup(move || actions.update(|n| *n = n.saturating_sub(1)));
        }
        None => {
            if cfg!(debug_assertions) {
                log::warn!("CardAction: not inside a CardHeader, so not placed beside its title");
            }
        }
    }
    // The span first: it sets the row's start too, which `row_start` then
    // replaces (`col-start-2 row-span-2 row-start-1`).

    view()
        .row_span(2u16)
        .row_start(1i16)
        .col_start(2i16)
        .self_start()
        .child(props.children)
}

/// The props of [`CardContent`].
#[derive(Debug, Clone, PartialEq, Props)]
pub struct CardContentProps {
    /// The body.
    pub children: Prop<Children>,
}

/// A card's body.
#[component]
pub fn CardContent(props: CardContentProps, _cx: Scope<NoEvent>) -> impl IntoElement {
    let t = theme();
    view()
        .px(move || t.length(CARD_PADDING))
        .child(props.children)
}

/// The props of [`CardFooter`].
#[derive(Debug, Clone, PartialEq, Props)]
pub struct CardFooterProps {
    /// Its content: buttons, a note.
    pub children: Prop<Children>,
}

/// A card's bottom row: buttons, notes. For a divider above it, give it
/// `class="border-t pt-6"`.
#[component]
pub fn CardFooter(props: CardFooterProps, _cx: Scope<NoEvent>) -> impl IntoElement {
    let t = theme();
    view()
        .flex_row()
        .items_center()
        .px(move || t.length(CARD_PADDING))
        .child(props.children)
}
