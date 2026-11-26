//! What the choice controls (`Checkbox`, `Switch`, `RadioGroupItem`) share:
//! their label part beside the control and their touch hit area. They are
//! one family: a rule changed here changes all three.

use nsh_core::{
    element::{Style, TextPartStyle},
    geometry::{Points, px},
    prelude::*,
};

use super::{
    size::{Size, pointer},
    typography::{font_size_token, line_height},
};
use crate::theme::{Theme, semantic::TOUCH_TARGET};

/// The label part of a choice control: a view after the control holding
/// its `children`, at least `min_height` tall (the control's height) with
/// its content centered on it, so one line sits level with the control and
/// more start at its top. Its text is `font-size-<size>` with its line
/// height, in `weight` and `color`; text children inherit them, each child
/// wrapped in a text (a string child takes the text style). The app's
/// `class` and `style` go last. Made only while there are children.
#[allow(clippy::too_many_arguments)]
pub fn choice_label(
    t: Theme,
    size: impl Fn() -> Size + Copy + 'static,
    min_height: impl Fn() -> Points + Copy + 'static,
    weight: FontWeight,
    color: impl Fn() -> AdaptiveColor + Copy + 'static,
    class: Prop<Option<SharedString>>,
    style: Prop<TextPartStyle>,
    children: Prop<Children>,
) -> AnyElement {
    dynamic(move || {
        (!children.get().is_empty()).then(|| {
            let font_size = move || t.length(font_size_token(size()));
            view()
                .flex_col()
                .justify_center()
                .flex_shrink(1.0)
                .min_h(min_height)
                .text_size(font_size)
                .line_height(move || line_height(font_size()))
                .font_weight(weight)
                .text_color(color)
                .app_class(class)
                .inline_style(move || Style::from(style.get()))
                .children_each(children, |child, _| text("").child(child))
        })
    })
}

/// How far the control sits below the top of its row, so it is centered on
/// the label's first line: half of what the line (`font-size-<size>`'s line
/// height) is taller than the control `height`, none when it is not or
/// without a label (`labelled`). Read it in a binding, as the control's top
/// margin.
pub fn choice_offset(t: Theme, labelled: bool, size: Size, height: Points) -> Points {
    if !labelled {
        return px(0.0);
    }
    let line = line_height(t.length(font_size_token(size)));
    ((line - height) / 2.0).max(px(0.0))
}

/// How far presses count around a choice control `width` × `height` on a
/// touch screen so a finger gets `touch-target` (44) points each way,
/// without the control looking bigger: none with a pointer, and none with
/// a label (`labelled`), which is the target itself (slop would reach into
/// the next row). Read it in a binding.
pub fn choice_hit_slop(t: Theme, labelled: bool, width: Points, height: Points) -> EdgeInsets {
    if pointer() || labelled {
        return EdgeInsets::default();
    }
    let target = t.length(TOUCH_TARGET);
    let x = ((target - width) / 2.0).max(px(0.0));
    let y = ((target - height) / 2.0).max(px(0.0));
    EdgeInsets {
        top: y,
        right: x,
        bottom: y,
        left: x,
    }
}
