//! `Tabs`, `TabsList`, `TabsTrigger` and `TabsContent`: shadcn/ui's tabs on
//! `nsh/base`'s `Tabs` and `Tab` (one stop of the keyboard focus, the
//! arrows, Home and End, the tablist and tab roles and their states for
//! screen readers). `nsh/base` holds no value and no content: `Tabs` holds
//! the value, `TabsContent` the panels, and each trigger draws its own
//! selected look. API: `docs/components/tabs.md`.

use nsh_base::{tab as base_tab, tabs as base_tabs, terminal::is_terminal};
use nsh_core::{
    element::{Style, TextPartStyle},
    geometry::Axis,
    prelude::*,
};

use crate::{
    foundation::{
        focus::focus_ring,
        icon::{IconName, icon_size_token, icon_view, provide_icon_size, provide_text_color},
        motion, shadow,
        size::Size,
        typography::{font_size_token, line_height, medium},
    },
    theme::{
        TRANSPARENT, Token, kind,
        seeds::SPACING,
        semantic::{
            BACKGROUND, BORDER_WIDTH, CONTROL_HEIGHT_LG, CONTROL_HEIGHT_MD, CONTROL_HEIGHT_SM,
            CONTROL_HEIGHT_TOUCH_LG, CONTROL_HEIGHT_TOUCH_MD, CONTROL_HEIGHT_TOUCH_SM,
            CONTROL_HEIGHT_TOUCH_XS, CONTROL_HEIGHT_XS, FOREGROUND, INPUT, MUTED, MUTED_FOREGROUND,
            OPACITY_DISABLED, RADIUS_LG, RING, RING_OPACITY,
        },
        theme,
    },
    tokens,
};

tokens! {
    /// The space between the list and the panels.
    TABS_GAP: Length = "tabs-gap" => |t| t.length(SPACING) * 2.0,
    /// The list's fill (the `default` variant).
    TABS_LIST: Color = "tabs-list" => |t| t.color(MUTED),
    /// The list's corner radius (the `default` variant).
    TABS_LIST_RADIUS: Length = "tabs-list-radius" => |t| t.length(RADIUS_LG),
    /// The space between the list's edge and its triggers (shadcn's
    /// `p-[3px]`).
    TABS_LIST_PADDING: Length = "tabs-list-padding" => |t| t.length(SPACING) * 0.75,
    /// An extra small list's height.
    TABS_LIST_HEIGHT_XS: Length = "tabs-list-height-xs" => |t| t.pointer_touch(t.length(CONTROL_HEIGHT_XS), t.length(CONTROL_HEIGHT_TOUCH_XS)),
    /// A small list's height.
    TABS_LIST_HEIGHT_SM: Length = "tabs-list-height-sm" => |t| t.pointer_touch(t.length(CONTROL_HEIGHT_SM), t.length(CONTROL_HEIGHT_TOUCH_SM)),
    /// A medium list's height.
    TABS_LIST_HEIGHT_MD: Length = "tabs-list-height-md" => |t| t.pointer_touch(t.length(CONTROL_HEIGHT_MD), t.length(CONTROL_HEIGHT_TOUCH_MD)),
    /// A large list's height (shadcn's `h-9`).
    TABS_LIST_HEIGHT_LG: Length = "tabs-list-height-lg" => |t| t.pointer_touch(t.length(CONTROL_HEIGHT_LG), t.length(CONTROL_HEIGHT_TOUCH_LG)),
    /// A trigger's corner radius: concentric with the list's. Drawn between
    /// 0 and half the trigger's height.
    TABS_TRIGGER_RADIUS: Length = "tabs-trigger-radius" => |t| t.length(TABS_LIST_RADIUS) - t.length(TABS_LIST_PADDING),
    /// A trigger's horizontal padding (shadcn's `px-2`).
    TABS_TRIGGER_PADDING_X: Length = "tabs-trigger-padding-x" => |t| t.length(SPACING) * 2.0,
    /// A trigger's text and icon at rest: `foreground` at 60 %,
    /// `muted-foreground` in the dark.
    TABS_TRIGGER_FOREGROUND: Color = "tabs-trigger-foreground" => |t| t.light_dark(t.color(FOREGROUND).opacity(0.6), t.color(MUTED_FOREGROUND)),
    /// A trigger's text and icon under the pointer.
    TABS_TRIGGER_HOVER_FOREGROUND: Color = "tabs-trigger-hover-foreground" => |t| t.color(FOREGROUND),
    /// The selected trigger's fill (the `default` variant): `background`,
    /// `input` at 30 % in the dark.
    TABS_TRIGGER_SELECTED: Color = "tabs-trigger-selected" => |t| t.light_dark(t.color(BACKGROUND), t.color(INPUT).opacity(0.3)),
    /// The selected trigger's text and icon.
    TABS_TRIGGER_SELECTED_FOREGROUND: Color = "tabs-trigger-selected-foreground" => |t| t.color(FOREGROUND),
    /// The selected trigger's edge (the `default` variant): none, `input`
    /// in the dark.
    TABS_TRIGGER_SELECTED_BORDER: Color = "tabs-trigger-selected-border" => |t| t.light_dark(TRANSPARENT, t.color(INPUT)),
    /// The selected trigger's shadow level (2: `sm`; the `default` variant).
    TABS_TRIGGER_SELECTED_SHADOW: Number = "tabs-trigger-selected-shadow" => |t| t.value(2.0),
    /// A trigger's edge while it has the keyboard focus.
    TABS_TRIGGER_BORDER_FOCUS: Color = "tabs-trigger-border-focus" => |t| t.color(RING),
    /// The focus ring around a trigger.
    TABS_TRIGGER_RING: Color = "tabs-trigger-ring" => |t| t.color(RING).opacity(t.number(RING_OPACITY)),
    /// The `line` variant's underline under the selected trigger.
    TABS_LINE_INDICATOR: Color = "tabs-line-indicator" => |t| t.color(FOREGROUND),
    /// The `line` variant's underline thickness.
    TABS_LINE_INDICATOR_WIDTH: Length = "tabs-line-indicator-width" => |t| t.px(2.0),
}

/// A tab list's look.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, PropEnum)]
pub enum TabsVariant {
    /// shadcn's segmented list: a muted track, the selected tab raised on
    /// it.
    #[default]
    Default,
    /// Tabs on no fill, the selected one underlined.
    Line,
}

nsh_core::static_reactive!(TabsVariant: TabsVariant);

/// What a `Tabs` gives the lists, triggers and panels inside it.
#[derive(Clone, Copy)]
struct TabsContext {
    /// The selected value: the app's, else the root's own.
    value: Memo<Option<SharedString>>,
    /// The root's own value, while uncontrolled.
    own: Signal<Option<SharedString>>,
    orientation: Axis,
    cx: Scope<TabsEvent>,
}

impl TabsContext {
    /// Selects `value`, reporting it when it is another.
    fn select(self, value: SharedString) {
        if self.value.get_untracked().as_ref() == Some(&value) {
            return;
        }
        self.own.set(Some(value.clone()));
        self.cx.emit(TabsEvent::Change(value));
    }
}

/// What a `TabsList` gives its triggers.
#[derive(Clone, Copy)]
struct ListContext {
    variant: Prop<TabsVariant>,
    size: Prop<Size>,
}

/// The props of [`Tabs`].
#[derive(Debug, Clone, PartialEq, Props)]
pub struct TabsProps {
    /// The selected tab's value (controlled): the app sets it in
    /// `onChange`. Left out, the tabs keep their own, from `defaultValue`.
    pub value: Prop<Option<SharedString>>,
    /// The first value while `value` is left out.
    pub default_value: Prop<Option<SharedString>>,
    /// The list's direction and arrow keys, and the layout: the list above
    /// the panels (`"horizontal"`) or beside them (`"vertical"`). Read when
    /// made.
    #[prop(default = Axis::Horizontal)]
    pub orientation: Prop<Axis>,
    /// The `TabsList` and the `TabsContent` panels, and anything around
    /// them.
    pub children: Prop<Children>,
}

/// The events of a [`Tabs`].
#[derive(Debug, Clone, PartialEq, Event)]
pub enum TabsEvent {
    /// The user selected another tab (a press, or an arrow, Home or End
    /// moving the focus to it): its value, which the app makes `value`.
    #[args(value)]
    Change(SharedString),
}

/// Tabs, shadcn/ui's: views of one page, one shown at a time, switched by
/// a row (or column) of tabs. It holds the selected value (the app's
/// `value`, or its own from `defaultValue`), a [`TabsList`] of
/// [`TabsTrigger`]s and the [`TabsContent`] panels, the selected one made.
#[component]
pub fn Tabs(props: TabsProps, cx: Scope<TabsEvent>) -> impl IntoElement {
    let t = theme();
    let orientation = props.orientation.get_untracked();
    let own = signal(props.default_value.get_untracked());
    let value = computed(move || props.value.get().or_else(|| own.get()));
    provide_context(TabsContext {
        value,
        own,
        orientation,
        cx,
    });
    view()
        .when(orientation == Axis::Horizontal, |v| v.flex_col())
        .when(orientation == Axis::Vertical, |v| v.flex_row())
        .gap(move || t.length(TABS_GAP))
        .child(props.children)
}

/// The props of [`TabsList`].
#[derive(Debug, Clone, PartialEq, Props)]
pub struct TabsListProps {
    /// Its look: `default` (shadcn's segmented list) or `line` (underlined
    /// tabs on no fill).
    #[prop(default)]
    pub variant: Prop<TabsVariant>,
    /// Its size (`xs`, `sm`, `md`, `lg`): the list's height and the
    /// triggers' text, icon and padding.
    #[prop(default)]
    pub size: Prop<Size>,
    /// What screen readers call the list ("Sections").
    pub accessibility_label: Prop<Option<SharedString>>,
    /// The `TabsTrigger`s, or views around them.
    pub children: Prop<Children>,
}

/// The row (or column) of a [`Tabs`]' triggers: one stop of the keyboard
/// focus, whose arrows move between the tabs and select them. As wide as
/// its content; `class="w-full"` stretches it.
#[component]
pub fn TabsList(props: TabsListProps, _cx: Scope<NoEvent>) -> impl IntoElement {
    let t = theme();
    let tabs = use_context::<TabsContext>();
    let orientation = tabs.map_or(Axis::Horizontal, |c| c.orientation);
    let horizontal = orientation == Axis::Horizontal;
    provide_context(ListContext {
        variant: props.variant,
        size: props.size,
    });
    let line = move || props.variant.get() == TabsVariant::Line;
    let root = base_tabs()
        .orientation(Some(orientation))
        .accessibility_label(props.accessibility_label);
    if is_terminal() {
        // `nsh/base`'s terminal tab bar.
        return root.child(props.children);
    }
    root.flex_shrink_0()
        .self_start()
        .when(horizontal, |v| {
            v.flex_row()
                .items_center()
                .justify_center()
                .h(move || t.length(list_height_token(props.size.get())))
        })
        .when(!horizontal, |v| v.flex_col().items_stretch())
        .gap(move || if line() { t.length(SPACING) } else { px(0.0) })
        .p(move || t.length(TABS_LIST_PADDING))
        .rounded(move || {
            if line() {
                px(0.0)
            } else {
                t.length(TABS_LIST_RADIUS)
            }
        })
        .bg(move || {
            if line() {
                AdaptiveColor::TRANSPARENT
            } else {
                t.color(TABS_LIST)
            }
        })
        .child(props.children)
}

/// The props of [`TabsTrigger`].
#[derive(Debug, Clone, PartialEq, Props)]
pub struct TabsTriggerProps {
    /// Its value, unique in its `Tabs`: it is selected while it is the
    /// `Tabs`' value.
    #[prop(default)]
    pub value: Prop<SharedString>,
    /// An icon before the label.
    pub icon: Prop<Option<IconName>>,
    /// No presses, passed over by the arrows, at half opacity.
    #[prop(default)]
    pub disabled: Prop<bool>,
    /// What screen readers say, when the children have no text (an
    /// icon-only tab).
    pub accessibility_label: Prop<Option<SharedString>>,
    /// More about the tab, read after its name.
    pub accessibility_description: Prop<Option<SharedString>>,
    /// Tailwind classes of the label: the part each child sits in (a
    /// string child's text takes its text classes).
    pub label_class: Prop<Option<SharedString>>,
    /// The inline style of the label part.
    #[prop(default)]
    pub label_style: Prop<TextPartStyle>,
    /// Tailwind classes of the icon.
    pub icon_class: Prop<Option<SharedString>>,
    /// The inline style of the icon.
    #[prop(default)]
    pub icon_style: Prop<Style>,
    /// Its label: a string, or anything else.
    pub children: Prop<Children>,
}

/// One tab of a [`TabsList`]: a press (or the arrows reaching it) selects
/// it. The selected one is raised on the list (`default`) or underlined
/// (`line`). Outside a `Tabs` it is never selected.
#[component]
pub fn TabsTrigger(props: TabsTriggerProps, _cx: Scope<NoEvent>) -> impl IntoElement {
    provide_text_color();
    let t = theme();
    let tabs = use_context::<TabsContext>();
    let list = use_context::<ListContext>();
    if tabs.is_none() {
        #[cfg(debug_assertions)]
        log::warn!("a `TabsTrigger` is outside a `Tabs`: it is never selected");
    }
    let horizontal = tabs.is_none_or(|c| c.orientation == Axis::Horizontal);
    let size = move || list.map(|l| l.size.get()).unwrap_or_default();
    let line = move || list.is_some_and(|l| l.variant.get() == TabsVariant::Line);
    let selected =
        move || tabs.is_some_and(|c| c.value.with(|v| v.as_ref() == Some(&props.value.get())));
    let disabled = move || props.disabled.get();
    // shadcn's `h-[calc(100%-1px)]` inside the list's padding.
    let height =
        move || t.length(list_height_token(size())) - t.length(TABS_LIST_PADDING) * 2.0 - px(1.0);
    let font_size = move || t.length(font_size_token(size()));
    let icon_side = move || t.length(icon_size_token(size()));
    let raised = move || selected() && !line();
    let foreground = move || {
        if selected() {
            t.color(TABS_TRIGGER_SELECTED_FOREGROUND)
        } else {
            t.color(TABS_TRIGGER_FOREGROUND)
        }
    };
    provide_icon_size(size);

    let root = base_tab()
        .selected(selected)
        .disabled(props.disabled)
        .accessibility_label(props.accessibility_label)
        .accessibility_description(props.accessibility_description)
        .on_press(move || {
            if let Some(c) = tabs
                && !props.disabled.get_untracked()
            {
                c.select(props.value.get_untracked());
            }
        });
    if is_terminal() {
        return root.children_each(props.children, |child, _| text("").child(child));
    }
    let root = root
        .flex_row()
        .items_center()
        // Grows from its content (`flex: 1 1 auto`): shadcn's `flex-1` relies
        // on a flex item's content minimum (`min-width: auto`), which NSH
        // does not apply, so from 0 a long label would get less than its
        // width in a list as wide as its triggers.
        .when(horizontal, |v| v.flex_auto().justify_center())
        .when(!horizontal, |v| v.justify_start())
        .gap(move || t.length(SPACING) * 1.5)
        .h(height)
        .px(move || t.length(TABS_TRIGGER_PADDING_X))
        .rounded(move || t.length(TABS_TRIGGER_RADIUS).clamp(px(0.0), height() / 2.0))
        .border(move || t.length(BORDER_WIDTH))
        .border_color(move || {
            if raised() {
                t.color(TABS_TRIGGER_SELECTED_BORDER)
            } else {
                AdaptiveColor::TRANSPARENT
            }
        })
        .bg(move || {
            if raised() {
                t.color(TABS_TRIGGER_SELECTED)
            } else {
                AdaptiveColor::TRANSPARENT
            }
        })
        .text_color(foreground)
        .text_size(font_size)
        .line_height(move || line_height(font_size()))
        .font_weight(medium(t))
        .whitespace_nowrap()
        .inline_style(move || {
            let level = if raised() {
                shadow::level(t.number(TABS_TRIGGER_SELECTED_SHADOW))
            } else {
                0
            };
            shadow::shadow(t, level)
        })
        .hover(move |s| {
            s.text_color(move || {
                if disabled() || selected() {
                    foreground()
                } else {
                    t.color(TABS_TRIGGER_HOVER_FOREGROUND)
                }
            })
        })
        .focus_visible(move |s| {
            focus_ring(
                s,
                t,
                move || t.color(TABS_TRIGGER_RING),
                Some(TABS_TRIGGER_BORDER_FOCUS),
            )
        })
        .disabled_style(move |s| s.opacity(move || t.number(OPACITY_DISABLED)));
    let root = motion::transition_all(root, t).cursor(Cursor::Default);

    // The `line` variant's underline: under the trigger in a row, along its
    // end in a column, faded in while selected.
    let underline = move || {
        line().then(|| {
            let thickness = move || t.length(TABS_LINE_INDICATOR_WIDTH);
            let indicator = view()
                .absolute()
                .pointer_events(PointerEvents::None)
                .accessibility_elements_hidden(true)
                .bg(move || t.color(TABS_LINE_INDICATOR))
                .opacity(move || if selected() { 1.0_f32 } else { 0.0 })
                .transition_opacity();
            let indicator = if horizontal {
                indicator
                    .left(px(0.0))
                    .right(px(0.0))
                    .h(thickness)
                    .bottom(move || -(t.length(TABS_LIST_PADDING) + px(2.0)))
            } else {
                indicator
                    .top(px(0.0))
                    .bottom(px(0.0))
                    .w(thickness)
                    .right(move || -(t.length(TABS_LIST_PADDING) + px(1.0)))
            };
            motion::timing(indicator, t)
        })
    };
    root.child(dynamic(underline))
        .child(dynamic(move || {
            props.icon.get().map(|name| {
                icon_view(t, move || name, icon_side)
                    .app_class(props.icon_class)
                    .inline_style(move || props.icon_style.get())
            })
        }))
        .children_each(props.children, move |child, _| {
            text("")
                .app_class(props.label_class)
                .inline_style(move || Style::from(props.label_style.get()))
                .child(child)
        })
}

/// The props of [`TabsContent`].
#[derive(Debug, Clone, PartialEq, Props)]
pub struct TabsContentProps {
    /// The value of the tab it belongs to.
    #[prop(default)]
    pub value: Prop<SharedString>,
    /// What screen readers call the panel, when its first heading does not
    /// name it.
    pub accessibility_label: Prop<Option<SharedString>>,
    /// Its content: made while its tab is selected, released otherwise.
    pub children: Prop<Children>,
}

/// The panel of one tab of a [`Tabs`]: shown, and its content made, while
/// that tab is selected.
#[component]
pub fn TabsContent(props: TabsContentProps, _cx: Scope<NoEvent>) -> impl IntoElement {
    let tabs = use_context::<TabsContext>();
    if tabs.is_none() {
        #[cfg(debug_assertions)]
        log::warn!("a `TabsContent` is outside a `Tabs`: it is never shown");
    }
    let selected = computed(move || {
        tabs.is_some_and(|c| c.value.with(|v| v.as_ref() == Some(&props.value.get())))
    });
    view()
        .role(AccessibilityRole::TabPanel)
        .optional_label(props.accessibility_label)
        .flex_1()
        .flex_col()
        .style_when(move || !selected.get(), |s| s.hidden())
        .child(dynamic(move || selected.get().then_some(props.children)))
}

/// The list height token of `size`.
pub fn list_height_token(size: Size) -> Token<kind::Length> {
    size.pick(
        TABS_LIST_HEIGHT_XS,
        TABS_LIST_HEIGHT_SM,
        TABS_LIST_HEIGHT_MD,
        TABS_LIST_HEIGHT_LG,
    )
}
