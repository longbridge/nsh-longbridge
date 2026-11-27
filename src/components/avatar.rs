//! `Avatar`, `AvatarBadge`, `AvatarGroup` and `AvatarGroupCount`:
//! an avatar on `nsh/base`'s `Avatar` and `AvatarGroup` (the
//! image's loading state, the initials, the image role and the group for
//! screen readers, which avatars of a group show). The library adds the
//! look: the circle and its sizes, the fallback, the badge, the group's
//! overlap and rings, the count. API: `docs/components/avatar.md`.
//!
//! The look is classes generated from the tokens (`foundation::class`):
//! the image and the fallback are `nsh/base`'s parts, and the app's
//! classes (`size-12`, `bg-green-600`) win over the library's.

use nsh_base::{avatar as base_avatar, avatar_group as base_avatar_group};
use nsh_core::{
    element::{ImageSrc, Style},
    prelude::*,
};

pub use nsh_base::AvatarStatus;

use crate::{
    foundation::{
        class::Classes,
        icon::{provide_icon_side, provide_text_color},
        size::Size,
        typography::line_height,
    },
    theme::{
        Theme, Token, kind,
        seeds::SPACING,
        semantic::{BACKGROUND, MUTED, MUTED_FOREGROUND, PRIMARY, PRIMARY_FOREGROUND},
        theme,
    },
    tokens,
};

tokens! {
    /// An extra small avatar's side (dense rows).
    AVATAR_SIZE_XS: Length = "avatar-size-xs" => |t| t.length(SPACING) * 5.0,
    /// A small avatar's side.
    AVATAR_SIZE_SM: Length = "avatar-size-sm" => |t| t.length(SPACING) * 6.0,
    /// A medium avatar's side.
    AVATAR_SIZE_MD: Length = "avatar-size-md" => |t| t.length(SPACING) * 8.0,
    /// A large avatar's side.
    AVATAR_SIZE_LG: Length = "avatar-size-lg" => |t| t.length(SPACING) * 10.0,
    /// The corner radius of an avatar, its image and fallback: a circle at
    /// any side.
    AVATAR_RADIUS: Length = "avatar-radius" => |t| t.px(9999.0),
    /// The fallback's fill.
    AVATAR_FALLBACK: Color = "avatar-fallback" => |t| t.color(MUTED),
    /// The fallback's text (the initials) and icons.
    AVATAR_FALLBACK_FOREGROUND: Color = "avatar-fallback-foreground" => |t| t.color(MUTED_FOREGROUND),
    /// An extra small avatar's initials and a count: about two fifths of
    /// the side, so two letters fit.
    AVATAR_FALLBACK_FONT_SIZE_XS: Length = "avatar-fallback-font-size-xs" => |t| t.px(8.0),
    /// A small avatar's initials and a count.
    AVATAR_FALLBACK_FONT_SIZE_SM: Length = "avatar-fallback-font-size-sm" => |t| t.px(10.0),
    /// A medium avatar's initials and a count (`text-sm`).
    AVATAR_FALLBACK_FONT_SIZE_MD: Length = "avatar-fallback-font-size-md" => |t| t.px(14.0),
    /// A large avatar's initials and a count.
    AVATAR_FALLBACK_FONT_SIZE_LG: Length = "avatar-fallback-font-size-lg" => |t| t.px(16.0),
    /// A badge's fill.
    AVATAR_BADGE: Color = "avatar-badge" => |t| t.color(PRIMARY),
    /// A badge's icon.
    AVATAR_BADGE_FOREGROUND: Color = "avatar-badge-foreground" => |t| t.color(PRIMARY_FOREGROUND),
    /// The badge of an extra small avatar.
    AVATAR_BADGE_SIZE_XS: Length = "avatar-badge-size-xs" => |t| t.length(SPACING) * 1.5,
    /// The badge of a small avatar.
    AVATAR_BADGE_SIZE_SM: Length = "avatar-badge-size-sm" => |t| t.length(SPACING) * 2.0,
    /// The badge of a medium avatar.
    AVATAR_BADGE_SIZE_MD: Length = "avatar-badge-size-md" => |t| t.length(SPACING) * 2.5,
    /// The badge of a large avatar.
    AVATAR_BADGE_SIZE_LG: Length = "avatar-badge-size-lg" => |t| t.length(SPACING) * 3.0,
    /// The ring around an avatar in a group, a badge and a count: the
    /// page's color, so they read as cut out.
    AVATAR_RING: Color = "avatar-ring" => |t| t.color(BACKGROUND),
    /// The ring's width.
    AVATAR_RING_WIDTH: Length = "avatar-ring-width" => |t| t.px(2.0),
    /// How far each avatar of an extra small group lies over the one before
    /// it: less than a larger group's, so the initials before stay clear.
    AVATAR_GROUP_OVERLAP_XS: Length = "avatar-group-overlap-xs" => |t| t.length(SPACING),
    /// How far each avatar of a small group lies over the one before it.
    AVATAR_GROUP_OVERLAP_SM: Length = "avatar-group-overlap-sm" => |t| t.length(SPACING),
    /// How far each avatar of a medium group lies over the one before it.
    AVATAR_GROUP_OVERLAP_MD: Length = "avatar-group-overlap-md" => |t| t.length(SPACING) * 2.0,
    /// How far each avatar of a large group lies over the one before it.
    AVATAR_GROUP_OVERLAP_LG: Length = "avatar-group-overlap-lg" => |t| t.length(SPACING) * 2.0,
}

/// The side token of `size`.
pub fn size_token(size: Size) -> Token<kind::Length> {
    size.pick(
        AVATAR_SIZE_XS,
        AVATAR_SIZE_SM,
        AVATAR_SIZE_MD,
        AVATAR_SIZE_LG,
    )
}

fn fallback_font_size_token(size: Size) -> Token<kind::Length> {
    size.pick(
        AVATAR_FALLBACK_FONT_SIZE_XS,
        AVATAR_FALLBACK_FONT_SIZE_SM,
        AVATAR_FALLBACK_FONT_SIZE_MD,
        AVATAR_FALLBACK_FONT_SIZE_LG,
    )
}

fn group_overlap_token(size: Size) -> Token<kind::Length> {
    size.pick(
        AVATAR_GROUP_OVERLAP_XS,
        AVATAR_GROUP_OVERLAP_SM,
        AVATAR_GROUP_OVERLAP_MD,
        AVATAR_GROUP_OVERLAP_LG,
    )
}

fn badge_size_token(size: Size) -> Token<kind::Length> {
    size.pick(
        AVATAR_BADGE_SIZE_XS,
        AVATAR_BADGE_SIZE_SM,
        AVATAR_BADGE_SIZE_MD,
        AVATAR_BADGE_SIZE_LG,
    )
}

/// The size of the avatar around (its badge reads it).
#[derive(Clone, Copy)]
struct AvatarSize(Memo<Size>);

/// The group an avatar is in: its size, for those without their own.
#[derive(Clone, Copy)]
struct InGroup(Memo<Size>);

/// The ring of an avatar in a group, a badge, a count.
fn ring(c: Classes, t: Theme) -> Classes {
    c.length("", "ring", t.length(AVATAR_RING_WIDTH))
        .color("", "ring", t.color(AVATAR_RING))
}

/// The props of [`Avatar`].
#[derive(Debug, Clone, PartialEq, Props)]
pub struct AvatarProps {
    /// The picture: a URL, an `asset:/` URI, a `data:` URI or an imported
    /// image. The fallback shows while it loads, when it fails and without
    /// one.
    pub src: Prop<Option<ImageSrc>>,
    /// The person's name: what screen readers say, and the initials of the
    /// fallback ("Ada Lovelace" is "AL").
    pub name: Prop<Option<SharedString>>,
    /// What screen readers say, over `name`; `""` makes the avatar
    /// decorative (beside the name it shows).
    pub accessibility_label: Prop<Option<SharedString>>,
    /// Its size (`xs` 20, `sm` 24, `md` 32, `lg` 40): its side and its
    /// initials. Left out, its group's, else `md`.
    pub size: Prop<Option<Size>>,
    /// The fallback's content in place of the initials (an icon).
    pub fallback: Prop<Option<Slot>>,
    /// Milliseconds the fallback waits while the image loads (`Infinity`:
    /// not while it loads).
    #[prop(default)]
    pub fallback_delay: Prop<f64>,
    /// Tailwind classes of the image, after its look.
    pub image_class: Prop<Option<SharedString>>,
    /// The inline style of the image.
    #[prop(default)]
    pub image_style: Prop<Style>,
    /// Tailwind classes of the fallback, after its look.
    pub fallback_class: Prop<Option<SharedString>>,
    /// The inline style of the fallback.
    #[prop(default)]
    pub fallback_style: Prop<Style>,
    /// Over the picture: an `AvatarBadge`, or the app's own mark.
    pub children: Prop<Children>,
}

/// The events of an [`Avatar`].
#[derive(Debug, Clone, PartialEq, Event)]
pub enum AvatarEvent {
    /// The image's status changed: `idle`, `loading`, `loaded`, `error`.
    #[args(status)]
    StatusChange(AvatarStatus),
}

/// An avatar: a person's picture in a circle, or their
/// initials on a muted fill while it loads, when it fails and without one.
/// Four sizes; a badge at its bottom end; in an `AvatarGroup`, a ring and
/// an overlap.
#[component]
pub fn Avatar(props: AvatarProps, cx: Scope<AvatarEvent>) -> impl IntoElement {
    let t = theme();
    let group = use_context::<InGroup>();
    let size = computed(move || {
        props
            .size
            .get()
            .or_else(|| group.map(|g| g.0.get()))
            .unwrap_or_default()
    });
    provide_context(AvatarSize(size));
    let radius = move || t.length(AVATAR_RADIUS);
    let mut root = base_avatar()
        .src(props.src)
        .name(props.name)
        .accessibility_label(props.accessibility_label)
        .fallback_delay(props.fallback_delay)
        .fallback(props.fallback.place(|content| content))
        .on_status_change(move |status: &AvatarStatus| {
            cx.emit(AvatarEvent::StatusChange(*status));
        })
        .app_class(move || {
            let c = Classes::new()
                .push("relative flex-row shrink-0 select-none")
                .length("", "size", t.length(size_token(size.get())))
                .length("", "rounded", radius());
            // In a group: the ring, and each after the first over the one
            // before by the group's overlap, whatever this avatar's own size.
            let c = if let Some(InGroup(group_size)) = group {
                let group_size = group_size.get();
                ring(c, t)
                    .length(
                        "",
                        "ml",
                        px(0.0) - t.length(group_overlap_token(group_size)),
                    )
                    .push("first:ml-0")
            } else {
                c
            };
            Some(c.build())
        })
        .image_class(move || {
            Some(
                Classes::new()
                    .push("size-full object-cover overflow-hidden")
                    .length("", "rounded", radius())
                    .then(props.image_class.get()),
            )
        })
        .fallback_class(move || {
            Some(fallback_class(t, size.get()).then(props.fallback_class.get()))
        })
        .child(props.children);
    // The app's part styles, bound (the setters take a style built once).
    let parts = root.__element().__props_mut();
    parts.image_style = props.image_style;
    parts.fallback_style = props.fallback_style;
    root
}

/// The fallback's look (and a count's): the muted circle, its text
/// centered.
fn fallback_class(t: Theme, size: Size) -> Classes {
    let font_size = t.length(fallback_font_size_token(size));
    Classes::new()
        .push("flex-row items-center justify-center overflow-hidden")
        .length("", "rounded", t.length(AVATAR_RADIUS))
        .color("", "bg", t.color(AVATAR_FALLBACK))
        .color("", "text", t.color(AVATAR_FALLBACK_FOREGROUND))
        .length("", "text", font_size)
        .length("", "leading", line_height(font_size))
}

/// The props of [`AvatarBadge`] and [`AvatarGroupCount`].
#[derive(Debug, Clone, PartialEq, Props)]
pub struct AvatarBadgeProps {
    /// Its content: an icon (a badge's is hidden at `xs` and `sm`), a
    /// count.
    pub children: Prop<Children>,
}

/// A dot at an [`Avatar`]'s bottom end, ringed in the page's color: a
/// presence, a status. Say what it means in the avatar's
/// `accessibilityLabel`; the badge is only drawn.
#[component]
pub fn AvatarBadge(props: AvatarBadgeProps, _cx: Scope<NoEvent>) -> impl IntoElement {
    provide_text_color();
    let t = theme();
    let size = use_context::<AvatarSize>().map_or_else(|| computed(|| Size::Md), |s| s.0);
    let side = move || t.length(badge_size_token(size.get()));
    provide_icon_side(move || side() - px(2.0));
    let small = move || matches!(size.get(), Size::Xs | Size::Sm);
    view()
        .accessibility_elements_hidden(true)
        .app_class(move || {
            let c = Classes::new()
                .push("absolute right-0 bottom-0 z-10 flex-row items-center justify-center")
                .length("", "size", side())
                .push("rounded-full")
                .color("", "bg", t.color(AVATAR_BADGE))
                .color("", "text", t.color(AVATAR_BADGE_FOREGROUND));
            Some(ring(c, t).build())
        })
        .children_each(props.children, move |child, _| {
            view().style_when(small, |s| s.hidden()).child(child)
        })
}

/// The props of [`AvatarGroup`].
#[derive(Debug, Clone, PartialEq, Props)]
pub struct AvatarGroupProps {
    /// The size of its avatars without their own, and of the count.
    #[prop(default)]
    pub size: Prop<Size>,
    /// At most this many avatars show, the first ones.
    #[prop(default = 3)]
    pub limit: Prop<u32>,
    /// How many people it stands for, when the app passes only the first
    /// (default its children).
    pub total: Prop<Option<u32>>,
    /// What stands for the rest (an `AvatarGroupCount`), shown while there
    /// are more than it shows.
    pub overflow: Prop<Option<Slot>>,
    /// What screen readers say of the group ("Ada, Grace and 5 others").
    pub accessibility_label: Prop<Option<SharedString>>,
    /// What they say of the overflow ("5 more people").
    pub overflow_accessibility_label: Prop<Option<SharedString>>,
    /// Tailwind classes of the overflow's box.
    pub overflow_class: Prop<Option<SharedString>>,
    /// The inline style of the overflow's box.
    #[prop(default)]
    pub overflow_style: Prop<Style>,
    /// The avatars, in order.
    pub children: Prop<Children>,
}

/// A row of overlapping avatars: the first `limit`, each over
/// the one before with a ring in the page's color, then the overflow (an
/// `AvatarGroupCount`) for the rest.
#[component]
pub fn AvatarGroup(props: AvatarGroupProps, _cx: Scope<NoEvent>) -> impl IntoElement {
    let t = theme();
    let size = computed(move || props.size.get());
    provide_context(InGroup(size));
    provide_context(AvatarSize(size));
    let mut root = base_avatar_group()
        .limit(props.limit)
        .total(props.total)
        .overflow(props.overflow.place(|content| content))
        .accessibility_label(props.accessibility_label)
        .overflow_accessibility_label(props.overflow_accessibility_label)
        // The count's circle is the overflow's box, which knows the group's
        // size: the overflow slot's content does not see the group's
        // context.
        .overflow_class(move || {
            let c = fallback_class(t, size.get())
                .push("shrink-0")
                .length("", "size", t.length(size_token(size.get())))
                .length(
                    "",
                    "ml",
                    px(0.0) - t.length(group_overlap_token(size.get())),
                );
            Some(ring(c, t).then(props.overflow_class.get()))
        })
        .app_class(Some(SharedString::from("flex-row items-center")))
        .child(props.children);
    root.__element().__props_mut().overflow_style = props.overflow_style;
    root
}

/// The content of an [`AvatarGroup`]'s overflow, the circle that stands
/// for the people it does not show ("+3", an icon): the group draws the
/// circle (the fallback's look at its size, ringed); this centers the count
/// in it, in the fallback's type.
#[component]
pub fn AvatarGroupCount(props: AvatarBadgeProps, _cx: Scope<NoEvent>) -> impl IntoElement {
    provide_text_color();
    let t = theme();
    let size = use_context::<AvatarSize>().map_or_else(|| computed(|| Size::Md), |s| s.0);
    provide_icon_side(move || t.length(size_token(size.get())) * 0.5);
    view()
        .app_class(Some(SharedString::from(
            "size-full flex-row items-center justify-center",
        )))
        .children_each(props.children, move |child, _| text("").child(child))
}
