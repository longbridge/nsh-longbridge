//! `Pagination`: a pager on `nsh/base`'s `Pagination` (the pages shown and
//! the ellipses, the keyboard and the focus, the navigation landmark, the
//! current page for screen readers). The library draws the items with
//! `Button`'s tokens: ghost pages, the current page outlined, Previous and
//! Next with a chevron and a word, the ellipsis icon. API:
//! `docs/components/pagination.md`.

use nsh_base::{pagination as base_pagination, terminal::is_terminal};
use nsh_core::{
    element::{Renderer, Style, StyleRefinement, TextPartStyle},
    prelude::*,
};

pub use nsh_base::{PaginationItemKind, PaginationItemState, pagination_items};

use crate::{
    components::button::{
        BUTTON_BORDER_FOCUS, BUTTON_GHOST_FOREGROUND, BUTTON_GHOST_HOVER,
        BUTTON_GHOST_HOVER_FOREGROUND, BUTTON_HEIGHT_LG, BUTTON_HEIGHT_MD, BUTTON_HEIGHT_SM,
        BUTTON_HEIGHT_XS, BUTTON_OUTLINE, BUTTON_OUTLINE_BORDER, BUTTON_OUTLINE_FOREGROUND,
        BUTTON_OUTLINE_HOVER, BUTTON_OUTLINE_HOVER_FOREGROUND, BUTTON_OUTLINE_SHADOW,
        BUTTON_RADIUS, BUTTON_RING,
    },
    foundation::{
        class::Classes,
        icon::{IconName, icon_size_token, icon_view, provide_icon_size, provide_text_color},
        labels::{self, Label},
        motion,
        press::BUTTON_SCALE,
        size::{Size, pointer},
        typography::{font_size_token, line_height, medium},
    },
    theme::{
        Theme, Token, kind,
        seeds::SPACING,
        semantic::{BORDER_WIDTH, OPACITY_DISABLED, RING_WIDTH},
        theme,
    },
    tokens,
};

tokens! {
    /// The space between the items.
    PAGINATION_GAP: Length = "pagination-gap" => |t| t.length(SPACING),
    /// Previous's and Next's padding at the sides.
    PAGINATION_STEP_PADDING_X: Length = "pagination-step-padding-x" => |t| t.length(SPACING) * 2.5,
    /// The space between Previous's and Next's chevron and word.
    PAGINATION_STEP_GAP: Length = "pagination-step-gap" => |t| t.length(SPACING),
}

fn height_token(size: Size) -> Token<kind::Length> {
    size.pick(
        BUTTON_HEIGHT_XS,
        BUTTON_HEIGHT_SM,
        BUTTON_HEIGHT_MD,
        BUTTON_HEIGHT_LG,
    )
}

/// The props of [`Pagination`].
#[derive(Debug, Clone, PartialEq, Props)]
pub struct PaginationProps {
    /// The current page, from 1 (controlled: the app sets it in
    /// `onChange`).
    pub page: Prop<u32>,
    /// How many pages there are, at least 1.
    pub total_pages: Prop<u32>,
    /// The most pages shown at once, the first and the last included (at
    /// least 5; odd numbers center the current page).
    #[prop(default = 5)]
    pub visible_pages: Prop<u32>,
    /// Previous, the children and Next only: a pager for a narrow place.
    #[prop(default)]
    pub compact: Prop<bool>,
    /// The items' height, type and icons: `Button`'s sizes.
    #[prop(default)]
    pub size: Prop<Size>,
    /// No item takes a press or the focus; dimmed.
    #[prop(default)]
    pub disabled: Prop<bool>,
    /// What screen readers call the navigation ("Pagination" by default):
    /// name what it pages.
    pub accessibility_label: Prop<Option<SharedString>>,
    /// Previous's name for screen readers ("Go to previous page").
    pub previous_accessibility_label: Prop<Option<SharedString>>,
    /// Next's name for screen readers ("Go to next page").
    pub next_accessibility_label: Prop<Option<SharedString>>,
    /// An item's content in place of the library's (a page in the app's
    /// words): the library still draws the item.
    pub render_item: Prop<Option<Renderer<PaginationItemState>>>,
    /// Tailwind classes of each item, after the library's look.
    pub item_class: Prop<Option<SharedString>>,
    /// The inline style of each item, merged after the library's.
    #[prop(default)]
    pub item_style: Prop<TextPartStyle>,
    /// After the pages, before Next: the page in words while `compact`.
    pub children: Prop<Children>,
}

/// The events of a [`Pagination`].
#[derive(Debug, Clone, PartialEq, Event)]
pub enum PaginationEvent {
    /// The user picked a page: a page, Previous or Next.
    #[args(page)]
    Change(u32),
}

/// A pagination: Previous, the pages around the current one with ellipses
/// for the runs left out, Next. Pages are ghost buttons, the current one
/// outlined; Previous and Next show a chevron and their word (the word
/// hidden in a narrow window). Four sizes, `Button`'s.
#[component]
pub fn Pagination(props: PaginationProps, cx: Scope<PaginationEvent>) -> impl IntoElement {
    provide_text_color();
    let t = theme();
    let size = computed(move || props.size.get());
    provide_icon_size(move || size.get());
    let mut root = base_pagination()
        .page(props.page)
        .total_pages(props.total_pages)
        .visible_pages(props.visible_pages)
        .compact(props.compact)
        .disabled(props.disabled)
        .accessibility_label(move || {
            props
                .accessibility_label
                .get()
                .or_else(|| Some(labels::text(Label::Pagination)))
        })
        .previous_accessibility_label(move || {
            props
                .previous_accessibility_label
                .get()
                .or_else(|| Some(labels::text(Label::GoToPreviousPage)))
        })
        .next_accessibility_label(move || {
            props
                .next_accessibility_label
                .get()
                .or_else(|| Some(labels::text(Label::GoToNextPage)))
        })
        .on_change(move |page: &u32| cx.emit(PaginationEvent::Change(*page)))
        .app_class(move || {
            Some(
                Classes::new()
                    .push("flex-row w-full items-center justify-center")
                    .length("", "gap", t.length(PAGINATION_GAP))
                    .build(),
            )
        })
        .child(props.children);
    if is_terminal() {
        // `nsh/base`'s terminal row of words.
        root.__element().__props_mut().render_item = props.render_item;
        return root;
    }
    root = root
        .item_class(move || Some(item_class(t).then(props.item_class.get())))
        .render_item(content(props.clone(), t, size));
    let app_style = props.item_style;
    root.__element().__props_mut().item_style =
        Prop::from(IntoReactive::<TextPartStyle>::into_reactive(move || {
            TextPartStyle::from(
                Style::from(item_geometry(t, size.get())).then(&Style::from(app_style.get())),
            )
        }));
    root
}

/// The items' states as classes (an inline style cannot take them): the
/// ghost look at rest and under the pointer, the outline look on the
/// current page (`selected:`), the focus ring and edge, dimmed when
/// disabled, the press's sink.
fn item_class(t: Theme) -> Classes {
    let mut c = Classes::new()
        .push("border-solid")
        .color("", "text", t.color(BUTTON_GHOST_FOREGROUND))
        .color("hover", "bg", t.color(BUTTON_GHOST_HOVER))
        .color("hover", "text", t.color(BUTTON_GHOST_HOVER_FOREGROUND))
        .color("active", "bg", t.color(BUTTON_GHOST_HOVER))
        .length("selected", "border", t.length(BORDER_WIDTH))
        .color("selected", "border", t.color(BUTTON_OUTLINE_BORDER))
        .color("selected", "bg", t.color(BUTTON_OUTLINE))
        .color("selected", "text", t.color(BUTTON_OUTLINE_FOREGROUND))
        .color("selected:hover", "bg", t.color(BUTTON_OUTLINE_HOVER))
        .color(
            "selected:hover",
            "text",
            t.color(BUTTON_OUTLINE_HOVER_FOREGROUND),
        )
        .number("disabled", "opacity", t.number(OPACITY_DISABLED))
        .push("disabled:bg-transparent");
    if t.number(BUTTON_OUTLINE_SHADOW) > 0.0 {
        c = c.push("selected:shadow-xs");
    }
    if !reduce_motion() {
        c = c.number("active", "scale", BUTTON_SCALE);
    }
    c = c.color(
        "selected:focus-visible",
        "border",
        t.color(BUTTON_BORDER_FOCUS),
    );
    if pointer() {
        c = c
            .length("focus-visible", "ring", t.length(RING_WIDTH))
            .color("focus-visible", "ring", t.color(BUTTON_RING));
    }
    c.push("transition-all ease-in-out")
        .push(&format!("duration-[{}ms]", motion::duration(t).round()))
}

/// The items' geometry: a square of the button height (Previous and Next
/// as wide as their content), its corners, its type.
fn item_geometry(t: Theme, size: Size) -> StyleRefinement {
    let height = t.length(height_token(size));
    let font_size = t.length(font_size_token(size));
    StyleRefinement::default()
        .flex_row()
        .items_center()
        .justify_center()
        .flex_shrink_0()
        .h(height)
        .min_w(height)
        .rounded(t.length(BUTTON_RADIUS))
        .text_size(font_size)
        .line_height(line_height(font_size))
        .font_weight(medium(t))
        .whitespace_nowrap()
        .cursor(Cursor::Default)
}

/// Each item's content: the page's number, Previous's and Next's chevron
/// and word, the ellipsis icon; the app's `renderItem` in their place.
fn content(
    props: PaginationProps,
    t: Theme,
    size: Memo<Size>,
) -> impl Fn(&PaginationItemState) -> View + 'static {
    let side = move || t.length(icon_size_token(size.get()));
    move |state: &PaginationItemState| {
        let state = state.clone();
        if let Some(render) = props.render_item.get() {
            let key = format!("{:?}-{}", state.kind, state.page);
            return view()
                .flex_row()
                .items_center()
                .justify_center()
                .child(render.render(key, state));
        }
        let step = |icon: IconName, word: Label, before: bool| {
            let icon = icon_view(t, move || icon, side);
            let word = text(move || labels::text(word)).app_class(Some("max-sm:hidden".into()));
            let row = view()
                .flex_row()
                .items_center()
                .gap(move || t.length(PAGINATION_STEP_GAP))
                .px(move || t.length(PAGINATION_STEP_PADDING_X));
            if before {
                row.child(icon).child(word)
            } else {
                row.child(word).child(icon)
            }
        };
        match state.kind {
            PaginationItemKind::Previous => step(IconName::ChevronLeft, Label::Previous, true),
            PaginationItemKind::Next => step(IconName::ChevronRight, Label::Next, false),
            PaginationItemKind::Ellipsis => view()
                .accessibility_elements_hidden(true)
                .child(icon_view(t, || IconName::Ellipsis, side)),
            _ => view().child(text(state.page.to_string())),
        }
    }
}
