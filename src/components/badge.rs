//! `Badge`: shadcn/ui's badge, a small mark for a status, a category or a
//! count. No behavior. API: `docs/components/badge.md`.

use nsh_core::{element::Style, element::TextPartStyle, prelude::*};

use crate::{
    foundation::{
        icon::{provide_icon_size, provide_text_color},
        size::Size,
        typography::{line_height, medium},
    },
    theme::{
        Theme,
        seeds::SPACING,
        semantic::{
            BORDER, BORDER_WIDTH, DESTRUCTIVE, DESTRUCTIVE_FOREGROUND, FONT_SIZE_XS, FOREGROUND,
            PRIMARY, PRIMARY_FOREGROUND, SECONDARY, SECONDARY_FOREGROUND,
        },
        theme,
    },
    tokens,
};

tokens! {
    /// The default variant's fill.
    BADGE_DEFAULT: Color = "badge-default" => |t| t.color(PRIMARY),
    /// The default variant's text and icons.
    BADGE_DEFAULT_FOREGROUND: Color = "badge-default-foreground" => |t| t.color(PRIMARY_FOREGROUND),
    /// The secondary variant's fill.
    BADGE_SECONDARY: Color = "badge-secondary" => |t| t.color(SECONDARY),
    /// The secondary variant's text and icons.
    BADGE_SECONDARY_FOREGROUND: Color = "badge-secondary-foreground" => |t| t.color(SECONDARY_FOREGROUND),
    /// The destructive variant's fill: `destructive`, at 60 % in the dark.
    BADGE_DESTRUCTIVE: Color = "badge-destructive" => |t| t.light_dark(t.color(DESTRUCTIVE), t.color(DESTRUCTIVE).opacity(0.6)),
    /// The destructive variant's text and icons.
    BADGE_DESTRUCTIVE_FOREGROUND: Color = "badge-destructive-foreground" => |t| t.color(DESTRUCTIVE_FOREGROUND),
    /// The outline variant's edge.
    BADGE_OUTLINE_BORDER: Color = "badge-outline-border" => |t| t.color(BORDER),
    /// The outline variant's text and icons.
    BADGE_OUTLINE_FOREGROUND: Color = "badge-outline-foreground" => |t| t.color(FOREGROUND),
    /// The corner radius: a pill (shadcn's `rounded-full`), not following
    /// the `radius` seed.
    BADGE_RADIUS: Length = "badge-radius" => |t| t.px(9999.0),
    /// The horizontal padding (shadcn `px-2`).
    BADGE_PADDING_X: Length = "badge-padding-x" => |t| t.length(SPACING) * 2.0,
    /// The vertical padding (shadcn `py-0.5`).
    BADGE_PADDING_Y: Length = "badge-padding-y" => |t| t.length(SPACING) * 0.5,
    /// The gap between an icon and the text.
    BADGE_GAP: Length = "badge-gap" => |t| t.length(SPACING),
    /// The text size.
    BADGE_FONT_SIZE: Length = "badge-font-size" => |t| t.length(FONT_SIZE_XS),
}

/// A badge's colors (shadcn's variants).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, PropEnum)]
pub enum BadgeVariant {
    /// The `primary` fill.
    #[default]
    Default,
    /// The `secondary` fill: a quieter mark.
    Secondary,
    /// The `destructive` fill: an error, a failure.
    Destructive,
    /// An edge and no fill.
    Outline,
}

nsh_core::static_reactive!(BadgeVariant: BadgeVariant);

/// The props of [`Badge`].
#[derive(Debug, Clone, PartialEq, Props)]
pub struct BadgeProps {
    /// Its colors: `default`, `secondary`, `destructive` or `outline`.
    #[prop(default)]
    pub variant: Prop<BadgeVariant>,
    /// Tailwind classes of the label: the part each child sits in (a
    /// string child's text takes its text classes).
    pub label_class: Prop<Option<SharedString>>,
    /// The inline style of the label part.
    #[prop(default)]
    pub label_style: Prop<TextPartStyle>,
    /// What it says: a few words, and the library's `Icon` before or after
    /// them.
    pub children: Prop<Children>,
}

/// A badge, shadcn/ui's: a small pill marking a status, a category, a
/// version or a count. Four variants, one size, one line (truncated when
/// too long). Not interactive: it is not pressed or focused.
#[component]
pub fn Badge(props: BadgeProps, _cx: Scope<NoEvent>) -> impl IntoElement {
    provide_text_color();
    let t = theme();
    let variant = move || props.variant.get();
    let font_size = move || t.length(BADGE_FONT_SIZE);
    provide_icon_size(|| Size::Xs);
    view()
        .flex_row()
        .items_center()
        .justify_center()
        // shadcn's `w-fit`: as wide as its content where a column stretches
        // its children, while a row's `items-center` still centers it (an
        // `align-self` would override the row's alignment). A class, as
        // `fit-content` has no typed setter; the app's class goes over it.
        .app_class(Some(SharedString::from("w-fit")))
        .flex_shrink_0()
        // A long badge is cut to its parent's width with an ellipsis.
        .max_w(Length::Definite(relative(1.0)))
        .overflow_hidden()
        .gap(move || t.length(BADGE_GAP))
        .px(move || t.length(BADGE_PADDING_X))
        .py(move || t.length(BADGE_PADDING_Y))
        .rounded(move || t.length(BADGE_RADIUS))
        .border(move || t.length(BORDER_WIDTH))
        .border_color(move || border(t, variant()))
        .bg(move || fill(t, variant()))
        .text_color(move || foreground(t, variant()))
        .text_size(font_size)
        .line_height(move || line_height(font_size()))
        .font_weight(medium(t))
        .whitespace_nowrap()
        .children_each(props.children, move |child, _| {
            text("")
                .number_of_lines(1u32)
                .flex_shrink(1.0)
                .min_w(px(0.0))
                .app_class(props.label_class)
                .inline_style(move || Style::from(props.label_style.get()))
                .child(child)
        })
}

fn fill(t: Theme, variant: BadgeVariant) -> AdaptiveColor {
    match variant {
        BadgeVariant::Default => t.color(BADGE_DEFAULT),
        BadgeVariant::Secondary => t.color(BADGE_SECONDARY),
        BadgeVariant::Destructive => t.color(BADGE_DESTRUCTIVE),
        BadgeVariant::Outline => AdaptiveColor::TRANSPARENT,
    }
}

fn foreground(t: Theme, variant: BadgeVariant) -> AdaptiveColor {
    match variant {
        BadgeVariant::Default => t.color(BADGE_DEFAULT_FOREGROUND),
        BadgeVariant::Secondary => t.color(BADGE_SECONDARY_FOREGROUND),
        BadgeVariant::Destructive => t.color(BADGE_DESTRUCTIVE_FOREGROUND),
        BadgeVariant::Outline => t.color(BADGE_OUTLINE_FOREGROUND),
    }
}

fn border(t: Theme, variant: BadgeVariant) -> AdaptiveColor {
    match variant {
        BadgeVariant::Outline => t.color(BADGE_OUTLINE_BORDER),
        _ => AdaptiveColor::TRANSPARENT,
    }
}
