//! `Toggle`: a button that is on or off, on `nsh/base`'s `Toggle`
//! (presses, the keyboard, the disabled state, the toggle button role and
//! its pressed state for screen readers, its place in a toolbar). This file
//! owns the toggle look (the `toggle-*` tokens), which `ToggleGroup`'s
//! items share. API: `docs/components/toggle.md`.

use nsh_base::toggle as base_toggle;
use nsh_core::{element::Style, element::TextPartStyle, prelude::*};

use crate::{
    components::button::content_gap,
    foundation::{
        focus::focus_ring,
        icon::{provide_icon_size, provide_text_color},
        motion,
        press::{BUTTON_SCALE, press},
        shadow,
        size::{Size, hit_slop},
        typography::{font_size_token, line_height, medium},
    },
    theme::{
        Token, kind,
        seeds::SPACING,
        semantic::{
            ACCENT, ACCENT_FOREGROUND, BORDER_WIDTH, CONTROL_HEIGHT_LG, CONTROL_HEIGHT_MD,
            CONTROL_HEIGHT_SM, CONTROL_HEIGHT_TOUCH_LG, CONTROL_HEIGHT_TOUCH_MD,
            CONTROL_HEIGHT_TOUCH_SM, CONTROL_HEIGHT_TOUCH_XS, CONTROL_HEIGHT_XS, FOREGROUND, INPUT,
            MUTED, MUTED_FOREGROUND, OPACITY_DISABLED, RADIUS_MD, RING, RING_OPACITY,
        },
        theme,
    },
    tokens,
};

tokens! {
    /// The corner radius.
    TOGGLE_RADIUS: Length = "toggle-radius" => |t| t.length(RADIUS_MD),
    /// An extra small toggle's height (and least width).
    TOGGLE_HEIGHT_XS: Length = "toggle-height-xs" => |t| t.pointer_touch(t.length(CONTROL_HEIGHT_XS), t.length(CONTROL_HEIGHT_TOUCH_XS)),
    /// A small toggle's height.
    TOGGLE_HEIGHT_SM: Length = "toggle-height-sm" => |t| t.pointer_touch(t.length(CONTROL_HEIGHT_SM), t.length(CONTROL_HEIGHT_TOUCH_SM)),
    /// A medium toggle's height.
    TOGGLE_HEIGHT_MD: Length = "toggle-height-md" => |t| t.pointer_touch(t.length(CONTROL_HEIGHT_MD), t.length(CONTROL_HEIGHT_TOUCH_MD)),
    /// A large toggle's height.
    TOGGLE_HEIGHT_LG: Length = "toggle-height-lg" => |t| t.pointer_touch(t.length(CONTROL_HEIGHT_LG), t.length(CONTROL_HEIGHT_TOUCH_LG)),
    /// An extra small toggle's horizontal padding.
    TOGGLE_PADDING_X_XS: Length = "toggle-padding-x-xs" => |t| t.length(SPACING),
    /// A small toggle's horizontal padding.
    TOGGLE_PADDING_X_SM: Length = "toggle-padding-x-sm" => |t| t.length(SPACING) * 1.5,
    /// A medium toggle's horizontal padding.
    TOGGLE_PADDING_X_MD: Length = "toggle-padding-x-md" => |t| t.length(SPACING) * 2.0,
    /// A large toggle's horizontal padding.
    TOGGLE_PADDING_X_LG: Length = "toggle-padding-x-lg" => |t| t.length(SPACING) * 2.5,
    /// The text and icons while off.
    TOGGLE_FOREGROUND: Color = "toggle-foreground" => |t| t.color(FOREGROUND),
    /// The fill under the pointer while off (`default` variant).
    TOGGLE_HOVER: Color = "toggle-hover" => |t| t.color(MUTED),
    /// The text under the pointer while off (`default` variant).
    TOGGLE_HOVER_FOREGROUND: Color = "toggle-hover-foreground" => |t| t.color(MUTED_FOREGROUND),
    /// The fill while on.
    TOGGLE_PRESSED: Color = "toggle-pressed" => |t| t.color(ACCENT),
    /// The text and icons while on.
    TOGGLE_PRESSED_FOREGROUND: Color = "toggle-pressed-foreground" => |t| t.color(ACCENT_FOREGROUND),
    /// The `outline` variant's edge.
    TOGGLE_OUTLINE_BORDER: Color = "toggle-outline-border" => |t| t.color(INPUT),
    /// The fill under the pointer while off (`outline` variant).
    TOGGLE_OUTLINE_HOVER: Color = "toggle-outline-hover" => |t| t.color(ACCENT),
    /// The text under the pointer while off (`outline` variant).
    TOGGLE_OUTLINE_HOVER_FOREGROUND: Color = "toggle-outline-hover-foreground" => |t| t.color(ACCENT_FOREGROUND),
    /// The `outline` variant's shadow level (1: `xs`; 0 for none).
    TOGGLE_OUTLINE_SHADOW: Number = "toggle-outline-shadow" => |t| t.value(1.0),
    /// The edge while focused (`outline` variant).
    TOGGLE_BORDER_FOCUS: Color = "toggle-border-focus" => |t| t.color(RING),
    /// The focus ring.
    TOGGLE_RING: Color = "toggle-ring" => |t| t.color(RING).opacity(t.number(RING_OPACITY)),
}

/// A toggle's look.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, PropEnum)]
pub enum ToggleVariant {
    /// No fill or edge while off: toolbars.
    #[default]
    Default,
    /// An edge and a shadow: a toggle standing alone, a segmented row.
    Outline,
}

/// The props of [`Toggle`].
#[derive(Debug, Clone, PartialEq, Props)]
pub struct ToggleProps {
    /// On or off. The app owns the value: a press reports the next one with
    /// `onChange`, and the toggle shows the value passed back.
    #[prop(default)]
    pub pressed: Prop<bool>,
    /// Its look: `default` (no fill while off) or `outline` (an edge).
    #[prop(default)]
    pub variant: Prop<ToggleVariant>,
    /// Its size (`xs`, `sm`, `md`, `lg`): height, text, icons and padding
    /// change together.
    #[prop(default)]
    pub size: Prop<Size>,
    /// No presses, no hover or press feedback, at half opacity.
    #[prop(default)]
    pub disabled: Prop<bool>,
    /// What screen readers say: what it turns on. Required when its content
    /// is an icon alone.
    pub accessibility_label: Prop<Option<SharedString>>,
    /// How far outside its bounds presses still count. Left out: on touch
    /// screens a toggle smaller than 44 points takes presses up to 44
    /// points around it; nothing otherwise.
    pub hit_slop: Prop<Option<EdgeInsets>>,
    /// Tailwind classes of the label parts (one around each child).
    pub label_class: Prop<Option<SharedString>>,
    /// The inline style of the label parts.
    #[prop(default)]
    pub label_style: Prop<TextPartStyle>,
    /// The content: an icon (the app's), a word, or both.
    pub children: Prop<Children>,
}

/// The events of a [`Toggle`].
#[derive(Debug, Clone, PartialEq, Event)]
pub enum ToggleEvent {
    /// It was pressed (a tap, a click, Space or Return): the value it asks
    /// for, the opposite of `pressed`.
    #[args(pressed)]
    Change(bool),
}

/// A toggle: a button that is on or off (bold, a favorite, a view option),
/// two variants, four sizes. On, it takes the `accent` fill; under the
/// pointer, a muted one; a press sinks it slightly (not with reduced
/// motion); the keyboard focus draws a ring around it. On touch screens it
/// is taller and takes presses up to 44 points around it.
#[component]
pub fn Toggle(props: ToggleProps, cx: Scope<ToggleEvent>) -> impl IntoElement {
    draw(props, None, move |pressed| {
        cx.emit(ToggleEvent::Change(pressed))
    })
}

/// What a [`ToggleGroup`](super::toggle_group::ToggleGroup)'s item draws
/// apart from a lone toggle.
#[derive(Clone, Copy)]
pub(crate) struct ItemLook {
    /// The items touch (`spacing` 0): one shape.
    pub joined: Memo<bool>,
    /// The gap between the items.
    pub gap: Memo<Points>,
    /// The item's horizontal padding.
    pub padding_x: Token<kind::Length>,
}

/// The drawing of a toggle with `props`, alone or as a group's item
/// (`item`), whose presses report the value they ask for to `on_change`.
pub(crate) fn draw(
    props: ToggleProps,
    item: Option<ItemLook>,
    on_change: impl Fn(bool) + 'static,
) -> impl IntoElement {
    let t = theme();
    let variant = move || props.variant.get();
    let size = move || props.size.get();
    let pressed = props.pressed;
    let disabled = props.disabled;
    let outline = move || variant() == ToggleVariant::Outline;
    let joined = move || item.is_some_and(|i| i.joined.get());
    let height = move || t.length(height_token(size()));
    let font_size = move || t.length(font_size_token(size()));
    provide_icon_size(size);
    provide_text_color();

    let rest_fill = move || {
        if pressed.get() {
            t.color(TOGGLE_PRESSED)
        } else {
            AdaptiveColor::TRANSPARENT
        }
    };
    let rest_foreground = move || {
        if pressed.get() {
            t.color(TOGGLE_PRESSED_FOREGROUND)
        } else {
            t.color(TOGGLE_FOREGROUND)
        }
    };
    // On wins over the pointer; a disabled toggle shows no hover.
    let hover_fill = move || {
        if pressed.get() || disabled.get() {
            rest_fill()
        } else if outline() {
            t.color(TOGGLE_OUTLINE_HOVER)
        } else {
            t.color(TOGGLE_HOVER)
        }
    };
    let hover_foreground = move || {
        if pressed.get() || disabled.get() {
            rest_foreground()
        } else if outline() {
            t.color(TOGGLE_OUTLINE_HOVER_FOREGROUND)
        } else {
            t.color(TOGGLE_HOVER_FOREGROUND)
        }
    };
    let border = move || {
        if outline() {
            t.length(BORDER_WIDTH)
        } else {
            px(0.0)
        }
    };
    // Counted from the outer edge: an outline toggle's edge takes its width
    // off the padding, so an icon alone is as square in `outline` as in
    // `default` (24 / 28 / 32 / 36 at any size).
    let padding_x = move || match item {
        Some(i) => t.length(i.padding_x),
        None => t.length(padding_token(size())) - border(),
    };
    let slop = move || {
        if let Some(given) = props.hit_slop.get() {
            return given;
        }
        let around = hit_slop(t, height());
        match item {
            // Side by side: the slop reaches at most half the gap sideways.
            Some(i) => {
                let side = if i.joined.get() {
                    px(0.0)
                } else {
                    around.left.min(i.gap.get() / 2.0)
                };
                EdgeInsets {
                    left: side,
                    right: side,
                    ..around
                }
            }
            None => around,
        }
    };
    // A joined item's corners and start edge depend on its place in the
    // row: `first:` and `last:` classes, made from the tokens.
    let place_class = move || {
        if !joined() {
            return None;
        }
        let radius = t.length(TOGGLE_RADIUS).0;
        let mut class = format!("first:rounded-s-[{radius}px] last:rounded-e-[{radius}px]");
        if outline() {
            let width = t.length(BORDER_WIDTH).0;
            class.push_str(&format!(" border-s-0 first:border-s-[{width}px]"));
        }
        Some(SharedString::from(class))
    };

    let root = base_toggle()
        .pressed(pressed)
        .disabled(disabled)
        .accessibility_label(props.accessibility_label)
        .hit_slop(slop)
        .on_change(move |next: &bool| on_change(*next))
        .flex_row()
        .items_center()
        .justify_center()
        .flex_shrink_0()
        .gap(move || content_gap(t, size()))
        .h(height)
        .min_w(move || if item.is_some() { px(0.0) } else { height() })
        .px(padding_x)
        .rounded(move || {
            if joined() {
                px(0.0)
            } else {
                t.length(TOGGLE_RADIUS)
            }
        })
        .border_t(border)
        .border_b(border)
        .border_e(border)
        // The start edge of a joined item is its class's.
        .style_when(move || !joined(), move |s| s.border_s(border))
        .border_color(move || t.color(TOGGLE_OUTLINE_BORDER))
        .bg(rest_fill)
        .text_color(rest_foreground)
        .text_size(font_size)
        .line_height(move || line_height(font_size()))
        .font_weight(medium(t))
        .whitespace_nowrap()
        .cursor(Cursor::Default)
        .inline_style(move || {
            // A joined row carries the shadow; its items none.
            let level = if outline() && !joined() {
                shadow::level(t.number(TOGGLE_OUTLINE_SHADOW))
            } else {
                0
            };
            shadow::shadow(t, level)
        })
        .hover(move |s| s.bg(hover_fill).text_color(hover_foreground))
        .active(move |s| {
            press(
                s.bg(hover_fill).text_color(hover_foreground),
                BUTTON_SCALE,
                move || disabled.get(),
            )
        })
        .focus_visible(move |s| {
            let s = focus_ring(s, t, move || t.color(TOGGLE_RING), None);
            // The edge, where there is one; raised over its neighbors so
            // the ring is not covered.
            s.border_color(move || {
                if outline() {
                    t.color(TOGGLE_BORDER_FOCUS)
                } else {
                    AdaptiveColor::TRANSPARENT
                }
            })
            .z_index(10)
        })
        .disabled_style(move |s| s.opacity(move || t.number(OPACITY_DISABLED)))
        .app_class(place_class);
    motion::transition_all(root, t).children_each(props.children, move |child, _| {
        text("")
            .app_class(props.label_class)
            .inline_style(move || Style::from(props.label_style.get()))
            .child(child)
    })
}

/// The height token of `size` (also a lone toggle's least width).
pub fn height_token(size: Size) -> Token<kind::Length> {
    size.pick(
        TOGGLE_HEIGHT_XS,
        TOGGLE_HEIGHT_SM,
        TOGGLE_HEIGHT_MD,
        TOGGLE_HEIGHT_LG,
    )
}

fn padding_token(size: Size) -> Token<kind::Length> {
    size.pick(
        TOGGLE_PADDING_X_XS,
        TOGGLE_PADDING_X_SM,
        TOGGLE_PADDING_X_MD,
        TOGGLE_PADDING_X_LG,
    )
}
