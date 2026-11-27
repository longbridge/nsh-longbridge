//! `Spinner`: a spinner, a turning loader: something is going on
//! and how long it takes is unknown. API: `docs/components/spinner.md`.

use nsh_core::prelude::*;

use crate::{
    foundation::{
        icon::{IconName, icon_size_here, icon_size_token, icon_view, text_color_here},
        labels::{self, Label},
        motion,
        size::Size,
    },
    theme::{semantic::FOREGROUND, theme},
};

/// The props of [`Spinner`].
#[derive(Debug, Clone, PartialEq, Props)]
pub struct SpinnerProps {
    /// Its size: 12 / 14 / 16 / 16 points for `xs` / `sm` / `md` / `lg`
    /// (`icon-size-*`). Left out, the size of the control it is in (`md`
    /// elsewhere). Larger: `class="size-8"`.
    pub size: Prop<Option<Size>>,
    /// What screen readers say ("Loading" when left out, translated with
    /// `setTranslator`).
    pub accessibility_label: Prop<Option<SharedString>>,
}

/// A loading spinner: lucide's `loader-circle` turning once a
/// second, in the color of the text around it inside this library's parts
/// (a button, a card, a menu), and `foreground` outside them, where NSH
/// passes no color (a `class` color on it wins either way). It keeps
/// turning with reduced motion (its turning is what it says). Screen
/// readers hear a busy progress indicator named "Loading".
#[component]
pub fn Spinner(props: SpinnerProps, _cx: Scope<NoEvent>) -> impl IntoElement {
    let t = theme();
    let here = icon_size_here();
    let side = move || {
        t.length(icon_size_token(
            props.size.get().unwrap_or_else(|| here.get()),
        ))
    };
    let root = icon_view(t, || IconName::LoaderCircle, side)
        .accessibility_elements_hidden(false)
        .accessible(true)
        .role(AccessibilityRole::Progressbar)
        .label(move || {
            props
                .accessibility_label
                .get()
                .unwrap_or_else(|| labels::text(Label::Loading))
        })
        .accessibility_state(AccessibilityState {
            busy: true,
            ..Default::default()
        })
        .pointer_events(PointerEvents::None);
    // Inside a part that colors its text, it takes that color; elsewhere
    // the text color NSH passes is black whatever the appearance.
    let root = if text_color_here() {
        root
    } else {
        root.text_color(move || t.color(FOREGROUND))
    };
    motion::spin(root)
}
