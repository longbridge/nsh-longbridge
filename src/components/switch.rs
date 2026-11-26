//! `Switch`: shadcn/ui's switch on `nsh/base`'s `Switch` (presses on the
//! track and its label, the keyboard, the switch role and its on / off and
//! disabled states for screen readers). API: `docs/components/switch.md`.

use nsh_base::{switch as base_switch, terminal::is_terminal};
use nsh_core::{
    element::{Style, TextPartStyle, layout_rtl},
    geometry::DefiniteLength,
    prelude::*,
};

use crate::{
    components::label::{LABEL_FONT_WEIGHT, LABEL_FOREGROUND},
    foundation::{
        choice::{choice_hit_slop, choice_label, choice_offset},
        focus::focus_ring,
        motion, shadow,
        size::Size,
        typography::font_weight,
    },
    theme::{
        Token, kind,
        seeds::SPACING,
        semantic::{
            BACKGROUND, BORDER_WIDTH, CONTROL_HEIGHT_LG, CONTROL_HEIGHT_MD, CONTROL_HEIGHT_SM,
            CONTROL_HEIGHT_TOUCH_LG, CONTROL_HEIGHT_TOUCH_MD, CONTROL_HEIGHT_TOUCH_SM,
            CONTROL_HEIGHT_TOUCH_XS, CONTROL_HEIGHT_XS, FOREGROUND, INPUT, OPACITY_DISABLED,
            PRIMARY, PRIMARY_FOREGROUND, RING, RING_OPACITY,
        },
        theme,
    },
    tokens,
};

/// The group the track and thumb take the switch's focus and press from.
const GROUP: &str = "nlb-switch";

/// How far the thumb stretches while the switch is held.
const STRETCH: f32 = 1.25;

tokens! {
    /// An extra small switch's thumb: half the control height. The track is
    /// twice as wide and a border taller.
    SWITCH_THUMB_SIZE_XS: Length = "switch-thumb-size-xs" => |t| t.pointer_touch(t.length(CONTROL_HEIGHT_XS) * 0.5, t.length(CONTROL_HEIGHT_TOUCH_XS) * 0.5),
    /// A small switch's thumb.
    SWITCH_THUMB_SIZE_SM: Length = "switch-thumb-size-sm" => |t| t.pointer_touch(t.length(CONTROL_HEIGHT_SM) * 0.5, t.length(CONTROL_HEIGHT_TOUCH_SM) * 0.5),
    /// A medium switch's thumb (a 32 × 18 track, shadcn's).
    SWITCH_THUMB_SIZE_MD: Length = "switch-thumb-size-md" => |t| t.pointer_touch(t.length(CONTROL_HEIGHT_MD) * 0.5, t.length(CONTROL_HEIGHT_TOUCH_MD) * 0.5),
    /// A large switch's thumb.
    SWITCH_THUMB_SIZE_LG: Length = "switch-thumb-size-lg" => |t| t.pointer_touch(t.length(CONTROL_HEIGHT_LG) * 0.5, t.length(CONTROL_HEIGHT_TOUCH_LG) * 0.5),
    /// The track's corner radius, at most half its height (a pill); the
    /// thumb's corners are concentric with it.
    SWITCH_RADIUS: Length = "switch-radius" => |t| t.px(9999.0),
    /// The space between the track and its label.
    SWITCH_GAP: Length = "switch-gap" => |t| t.length(SPACING) * 2.0,
    /// The track while off: `input`, at 80 % in the dark.
    SWITCH: Color = "switch" => |t| t.light_dark(t.color(INPUT), t.color(INPUT).opacity(0.8)),
    /// The track while on.
    SWITCH_CHECKED: Color = "switch-checked" => |t| t.color(PRIMARY),
    /// The thumb while off: `background`, `foreground` in the dark.
    SWITCH_THUMB: Color = "switch-thumb" => |t| t.light_dark(t.color(BACKGROUND), t.color(FOREGROUND)),
    /// The thumb while on: `background`, `primary-foreground` in the dark.
    SWITCH_CHECKED_THUMB: Color = "switch-checked-thumb" => |t| t.light_dark(t.color(BACKGROUND), t.color(PRIMARY_FOREGROUND)),
    /// The track's edge while focused (transparent otherwise).
    SWITCH_BORDER_FOCUS: Color = "switch-border-focus" => |t| t.color(RING),
    /// The focus ring around the track.
    SWITCH_RING: Color = "switch-ring" => |t| t.color(RING).opacity(t.number(RING_OPACITY)),
    /// The track's shadow level (1: `xs`; 0 for none).
    SWITCH_SHADOW: Number = "switch-shadow" => |t| t.value(1.0),
}

/// The props of [`Switch`].
#[derive(Debug, Clone, PartialEq, Props)]
pub struct SwitchProps {
    /// On or off. The app owns the value: a press reports the next one with
    /// `onChange`, and the switch shows the value passed back.
    #[prop(default)]
    pub checked: Prop<bool>,
    /// No presses and no press feedback, the track and its label at half
    /// opacity.
    #[prop(default)]
    pub disabled: Prop<bool>,
    /// Its size (`xs`, `sm`, `md`, `lg`): the track, the thumb and the
    /// label's text.
    #[prop(default)]
    pub size: Prop<Size>,
    /// What screen readers say, when the children have no text that says
    /// what the switch turns on (always for a track alone).
    pub accessibility_label: Prop<Option<SharedString>>,
    /// What turning it on does, for screen readers.
    pub accessibility_hint: Prop<Option<SharedString>>,
    /// How far outside its bounds presses still count. Left out: on touch
    /// screens a track without a label takes presses up to 44 points around
    /// it; nothing otherwise.
    pub hit_slop: Prop<Option<EdgeInsets>>,
    /// Tailwind classes of the track.
    pub track_class: Prop<Option<SharedString>>,
    /// The inline style of the track.
    #[prop(default)]
    pub track_style: Prop<Style>,
    /// Tailwind classes of the thumb.
    pub thumb_class: Prop<Option<SharedString>>,
    /// The inline style of the thumb.
    #[prop(default)]
    pub thumb_style: Prop<Style>,
    /// Tailwind classes of the label: the part the children sit in (their
    /// text takes its text classes).
    pub label_class: Prop<Option<SharedString>>,
    /// The inline style of the label part.
    #[prop(default)]
    pub label_style: Prop<TextPartStyle>,
    /// The label: a string, or anything else. A press on it toggles the
    /// switch.
    pub children: Prop<Children>,
}

/// The events of a [`Switch`].
#[derive(Debug, Clone, PartialEq, Event)]
pub enum SwitchEvent {
    /// The switch was pressed (a tap, a click, Space or Return): the value
    /// it asks for, the opposite of `checked`.
    #[args(checked)]
    Change(bool),
}

/// A switch, shadcn/ui's: a setting on or off that takes effect at once, a
/// thumb moving along a track, with its label beside it. Four sizes; a
/// press on the label toggles it too, and the thumb stretches while it is
/// held (not with reduced motion). The keyboard focus draws a ring around
/// the track; on touch screens it is larger, and without a label it takes
/// presses up to 44 points around it.
#[component]
pub fn Switch(props: SwitchProps, cx: Scope<SwitchEvent>) -> impl IntoElement {
    let t = theme();
    let size = move || props.size.get();
    let checked = props.checked;
    let disabled = props.disabled;
    let labelled = move || !props.children.get().is_empty();
    let thumb = move || t.length(thumb_token(size()));
    let border = move || t.length(BORDER_WIDTH);
    let track_w = move || thumb() * 2.0;
    let track_h = move || thumb() + border() * 2.0;
    let track_radius = move || t.length(SWITCH_RADIUS).min(track_h() / 2.0).max(px(0.0));
    // Toward the end while on: right, left right to left.
    let travel = move || {
        if !checked.get() {
            return px(0.0);
        }
        let d = thumb() - border() * 2.0;
        if layout_rtl() { -d } else { d }
    };
    // It stretches from the end it rests at.
    let from_right = move || checked.get() != layout_rtl();

    let thumb_view = view()
        .w(thumb)
        .h(thumb)
        .flex_shrink_0()
        .rounded(move || (track_radius() - border()).max(px(0.0)))
        .bg(move || {
            if checked.get() {
                t.color(SWITCH_CHECKED_THUMB)
            } else {
                t.color(SWITCH_THUMB)
            }
        })
        .translate_x(move || DefiniteLength::from(travel()))
        .origin_left()
        .style_when(from_right, |s| s.origin_right())
        .group_active_named(GROUP, move |s| {
            s.scale_x(move || {
                if disabled.get() || reduce_motion() {
                    1.0_f32
                } else {
                    STRETCH
                }
            })
        })
        .transition_transform()
        .app_class(props.thumb_class)
        .inline_style(move || props.thumb_style.get());
    let thumb_view = motion::timing(thumb_view, t);

    let track = view()
        .flex_row()
        .mt(move || choice_offset(t, labelled(), size(), track_h()))
        .items_center()
        .flex_shrink_0()
        .w(track_w)
        .h(track_h)
        .rounded(track_radius)
        .border(border)
        .border_color(AdaptiveColor::TRANSPARENT)
        .bg(move || {
            if checked.get() {
                t.color(SWITCH_CHECKED)
            } else {
                t.color(SWITCH)
            }
        })
        .group_focus_visible_named(GROUP, move |s| {
            focus_ring(
                s,
                t,
                move || t.color(SWITCH_RING),
                Some(SWITCH_BORDER_FOCUS),
            )
        })
        .app_class(props.track_class)
        .inline_style(move || {
            shadow::shadow(t, shadow::level(t.number(SWITCH_SHADOW))).then(&props.track_style.get())
        })
        .child(thumb_view);
    let track = motion::transition_all(track, t);

    base_switch()
        .checked(checked)
        .disabled(disabled)
        .accessibility_label(props.accessibility_label)
        .accessibility_hint(props.accessibility_hint)
        .hit_slop(move || {
            props
                .hit_slop
                .get()
                .unwrap_or_else(|| choice_hit_slop(t, labelled(), track_w(), track_h()))
        })
        .on_change(move |next: &bool| cx.emit(SwitchEvent::Change(*next)))
        .group_named(GROUP)
        .flex_row()
        .items_start()
        .gap(move || t.length(SWITCH_GAP))
        .cursor(Cursor::Default)
        .disabled_style(move |s| s.opacity(move || t.number(OPACITY_DISABLED)))
        // `nsh/base`'s track takes no style prop from the app here (a
        // Rust setter takes a fixed style): the track is drawn as a child,
        // where `trackClass` and `trackStyle` reach it.
        .track_style(|s| s.hidden())
        // The terminal shows `nsh/base`'s own switch.
        .when(!is_terminal(), |s| s.child(track))
        .child(choice_label(
            t,
            size,
            track_h,
            font_weight(t, LABEL_FONT_WEIGHT),
            move || t.color(LABEL_FOREGROUND),
            props.label_class,
            props.label_style,
            props.children,
        ))
}

/// The thumb's side token of `size`.
pub fn thumb_token(size: Size) -> Token<kind::Length> {
    size.pick(
        SWITCH_THUMB_SIZE_XS,
        SWITCH_THUMB_SIZE_SM,
        SWITCH_THUMB_SIZE_MD,
        SWITCH_THUMB_SIZE_LG,
    )
}
