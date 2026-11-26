//! `Skeleton`: shadcn/ui's skeleton, a pulsing block holding the place of
//! content that has not arrived. No behavior, out of the accessibility
//! tree. API: `docs/components/skeleton.md`.

use nsh_core::prelude::*;

use crate::{
    theme::{
        semantic::{ACCENT, RADIUS_MD},
        theme,
    },
    tokens,
};

tokens! {
    /// The block's fill.
    SKELETON: Color = "skeleton" => |t| t.color(ACCENT),
    /// The block's corner radius (`class="rounded-full"` for a round one).
    SKELETON_RADIUS: Length = "skeleton-radius" => |t| t.length(RADIUS_MD),
}

/// The props of [`Skeleton`].
#[derive(Debug, Clone, PartialEq, Props)]
pub struct SkeletonProps {}

/// A loading placeholder, shadcn/ui's: a block in the shape of the content
/// to come, pulsing slowly (not with reduced motion). It has no size of its
/// own: give it one (`class="h-4 w-[250px]"`, `class="size-12
/// rounded-full"`). Screen readers skip it: say "loading" on its container
/// (`accessibilityState={{ busy: true }}`).
#[component]
pub fn Skeleton(_props: SkeletonProps, _cx: Scope<NoEvent>) -> impl IntoElement {
    let t = theme();
    view()
        .rounded(move || t.length(SKELETON_RADIUS))
        .bg(move || t.color(SKELETON))
        .animate_pulse()
        .pointer_events(PointerEvents::None)
        .accessible(false)
        .accessibility_elements_hidden(true)
}
