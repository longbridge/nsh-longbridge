//! `Separator`: a separator, a thin line between content,
//! horizontal or vertical. Decorative: no behavior, out of the
//! accessibility tree. API: `docs/components/separator.md`.

use nsh_core::{geometry::Axis, prelude::*};

use crate::{
    theme::{
        semantic::{BORDER, BORDER_WIDTH},
        theme,
    },
    tokens,
};

tokens! {
    /// The line's color.
    SEPARATOR: Color = "separator" => |t| t.color(BORDER),
    /// The line's thickness.
    SEPARATOR_THICKNESS: Length = "separator-thickness" => |t| t.length(BORDER_WIDTH),
}

/// The props of [`Separator`].
#[derive(Debug, Clone, PartialEq, Props)]
pub struct SeparatorProps {
    /// Its direction: `"horizontal"` (the default) spans its parent's
    /// width, `"vertical"` its parent's height (`class="h-4"` for a
    /// shorter one).
    #[prop(default = Axis::Horizontal)]
    pub orientation: Prop<Axis>,
}

/// A separator: a one-point line in the `border` color that
/// divides sections (horizontal) or the items of a row (vertical). It
/// stretches across its parent; it takes no pointer events and screen
/// readers skip it.
#[component]
pub fn Separator(props: SeparatorProps, _cx: Scope<NoEvent>) -> impl IntoElement {
    let t = theme();
    let vertical = move || props.orientation.get() == Axis::Vertical;
    let thickness = move || t.length(SEPARATOR_THICKNESS);
    view()
        .flex_shrink_0()
        .self_stretch()
        .style_when(move || !vertical(), move |s| s.h(thickness))
        .style_when(vertical, move |s| s.w(thickness))
        .bg(move || t.color(SEPARATOR))
        .pointer_events(PointerEvents::None)
        .accessible(false)
        .accessibility_elements_hidden(true)
}
