//! `Field` and its parts: the layout of a form (a control with its label,
//! description and error; columns of fields; titled sets; choice cards),
//! drawn with views of this library's. A `Field` hands its `invalid` and
//! `disabled`, and its description or error, to the control inside it
//! (`foundation::field`). API: `docs/components/field.md`.

use nsh_core::{
    LiveRegion,
    element::{JsHost, JsIn, JsObject, JsOwned, JsValue, PropError, Style, TsTypes},
    prelude::*,
};

use crate::{
    components::{
        label::{LABEL_FONT_SIZE, LABEL_FONT_WEIGHT, LABEL_FOREGROUND, LABEL_GAP},
        separator::separator,
    },
    foundation::{
        field::{self, FieldState},
        field_ref::FieldRef,
        icon::provide_text_color,
        typography::{line_height, medium, weight},
    },
    theme::{
        seeds::SPACING,
        semantic::{
            BACKGROUND, BORDER, BORDER_WIDTH, DESTRUCTIVE, FOREGROUND, MUTED_FOREGROUND,
            OPACITY_DISABLED, PRIMARY, RADIUS_MD, TEXT_BASE, TEXT_SM,
        },
        theme,
    },
    tokens,
};

tokens! {
    /// The space between a field's parts.
    FIELD_GAP: Length = "field-gap" => |t| t.length(SPACING) * 3.0,
    /// The space between a `FieldContent`'s parts.
    FIELD_CONTENT_GAP: Length = "field-content-gap" => |t| t.length(SPACING) * 1.5,
    /// The space between a `FieldGroup`'s fields.
    FIELD_GROUP_GAP: Length = "field-group-gap" => |t| t.length(SPACING) * 7.0,
    /// The space between the fields of a `FieldGroup` directly in another.
    FIELD_GROUP_NESTED_GAP: Length = "field-group-nested-gap" => |t| t.length(SPACING) * 4.0,
    /// The space between a `FieldSet`'s parts.
    FIELD_SET_GAP: Length = "field-set-gap" => |t| t.length(SPACING) * 6.0,
    /// The `FieldGroup` width from which `responsive` fields are horizontal.
    FIELD_RESPONSIVE_WIDTH: Length = "field-responsive-width" => |t| t.px(448.0),
    /// The space under a `FieldLegend`.
    FIELD_LEGEND_MARGIN_BOTTOM: Length = "field-legend-margin-bottom" => |t| t.length(SPACING) * 3.0,
    /// A `FieldLegend`'s text size (`variant="legend"`).
    FIELD_LEGEND_FONT_SIZE: Length = "field-legend-font-size" => |t| t.length(TEXT_BASE),
    /// A `FieldLegend`'s text.
    FIELD_LEGEND_FOREGROUND: Color = "field-legend-foreground" => |t| t.color(FOREGROUND),
    /// A label's and a title's text while their field is invalid.
    FIELD_LABEL_FOREGROUND_INVALID: Color = "field-label-foreground-invalid" => |t| t.color(DESTRUCTIVE),
    /// A `FieldDescription`'s text size.
    FIELD_DESCRIPTION_FONT_SIZE: Length = "field-description-font-size" => |t| t.length(TEXT_SM),
    /// A `FieldDescription`'s text.
    FIELD_DESCRIPTION_FOREGROUND: Color = "field-description-foreground" => |t| t.color(MUTED_FOREGROUND),
    /// A `FieldError`'s text size.
    FIELD_ERROR_FONT_SIZE: Length = "field-error-font-size" => |t| t.length(TEXT_SM),
    /// A `FieldError`'s text.
    FIELD_ERROR_FOREGROUND: Color = "field-error-foreground" => |t| t.color(DESTRUCTIVE),
    /// The word on a `FieldSeparator`.
    FIELD_SEPARATOR_CONTENT_FOREGROUND: Color = "field-separator-content-foreground" => |t| t.color(MUTED_FOREGROUND),
    /// The fill behind the word on a `FieldSeparator`: what it is drawn on.
    FIELD_SEPARATOR_CONTENT_BACKGROUND: Color = "field-separator-content-background" => |t| t.color(BACKGROUND),
    /// A choice card's corners.
    FIELD_CHOICE_CARD_RADIUS: Length = "field-choice-card-radius" => |t| t.length(RADIUS_MD),
    /// The padding inside a choice card.
    FIELD_CHOICE_CARD_PADDING: Length = "field-choice-card-padding" => |t| t.length(SPACING) * 4.0,
    /// A choice card's edge.
    FIELD_CHOICE_CARD_BORDER: Color = "field-choice-card-border" => |t| t.color(BORDER),
    /// A checked choice card's fill.
    FIELD_CHOICE_CARD_CHECKED: Color = "field-choice-card-checked" => |t| t.light_dark(t.color(PRIMARY).opacity(0.05), t.color(PRIMARY).opacity(0.1)),
    /// A checked choice card's edge.
    FIELD_CHOICE_CARD_CHECKED_BORDER: Color = "field-choice-card-checked-border" => |t| t.color(PRIMARY),
}

/// How a field lays out its label and control.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, PropEnum)]
pub enum FieldOrientation {
    /// The label above the control.
    #[default]
    Vertical,
    /// The label beside the control.
    Horizontal,
    /// Vertical until the enclosing `FieldGroup` is `field-responsive-width`
    /// (448) wide, horizontal from there.
    Responsive,
}

/// A `FieldLegend`'s size.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, PropEnum)]
pub enum FieldLegendVariant {
    /// A set's title (16).
    #[default]
    Legend,
    /// A label's size (14): a set that reads as one field (a group of
    /// checkboxes).
    Label,
}

/// The messages of a [`FieldError`]: TSX takes what validation libraries
/// give (`{ message }` objects, `undefined` for none); Rust collects
/// messages (`FieldErrors::from_iter(["Too short."])`). Empty messages are
/// skipped and repeated ones shown once.
#[derive(Debug, Clone, Default, PartialEq)]
pub struct FieldErrors(Vec<SharedString>);

impl FieldErrors {
    /// The messages to show, in order: no empty ones, each once.
    pub fn messages(&self) -> Vec<SharedString> {
        let mut shown: Vec<SharedString> = Vec::new();
        for m in &self.0 {
            if !m.trim().is_empty() && !shown.contains(m) {
                shown.push(m.clone());
            }
        }
        shown
    }
}

impl<S: Into<SharedString>> FromIterator<S> for FieldErrors {
    fn from_iter<I: IntoIterator<Item = S>>(iter: I) -> Self {
        Self(iter.into_iter().map(Into::into).collect())
    }
}

/// One validation message, as validation libraries give them.
#[derive(Debug, Clone, Default, PartialEq, JsObject)]
pub struct FieldErrorMessage {
    /// The message.
    pub message: Option<SharedString>,
}

impl JsValue for FieldErrors {
    fn ts_type(_types: &mut TsTypes) -> String {
        "({ message?: string | null } | null | undefined)[]".into()
    }

    fn from_js(value: JsIn<'_>) -> Result<Self, PropError> {
        if value.is_nullish() {
            return Ok(Self::default());
        }
        let Some(len) = value.array_len()? else {
            return Err(value.expected("a list of `{ message }` objects"));
        };
        let mut messages = Vec::with_capacity(len);
        for i in 0..len {
            let item = value.index(i)?;
            let item = item.get();
            if item.is_nullish() {
                continue;
            }
            let message = item.field("message")?;
            if let Some(text) = message.get().as_string() {
                messages.push(SharedString::from(text));
            }
        }
        Ok(Self(messages))
    }

    fn to_js<'a>(&self, host: &'a dyn JsHost) -> Result<JsOwned<'a>, PropError> {
        self.0
            .iter()
            .map(|m| FieldErrorMessage {
                message: Some(m.clone()),
            })
            .collect::<Vec<_>>()
            .to_js(host)
    }
}

nsh_core::static_reactive!(FieldErrors: FieldErrors);

/// What a `Field` tells its parts.
#[derive(Clone, Copy)]
struct FieldCx {
    state: FieldState,
    horizontal: Memo<bool>,
    /// How many `FieldContent`s it holds.
    contents: Signal<usize>,
}

/// What a `FieldLabel` tells a `Field` inside it: it is a choice card
/// around that field.
#[derive(Clone, Copy)]
struct CardCx {
    field: Signal<Option<FieldState>>,
}

/// What a `FieldGroup` tells what is inside it.
#[derive(Clone, Copy)]
struct GroupCx {
    /// Its width, once laid out.
    width: Signal<Option<Points>>,
    /// How many responsive fields read it.
    responsive: Signal<usize>,
}

/// Whether a part is directly in a `FieldGroup` (no `Field` between).
#[derive(Clone, Copy)]
struct InGroup(bool);

/// Whether a part is directly in a `FieldSet` (no `Field` or `FieldGroup`
/// between).
#[derive(Clone, Copy)]
struct InSet(bool);

/// The props of [`Field`].
#[derive(Debug, Clone, PartialEq, Props)]
pub struct FieldProps {
    /// The label above the control (`vertical`), beside it
    /// (`horizontal`), or above it until the enclosing `FieldGroup` is 448
    /// wide (`responsive`).
    #[prop(default)]
    pub orientation: Prop<FieldOrientation>,
    /// The value is rejected: the label turns red, and so does the control
    /// (its invalid look and state).
    #[prop(default)]
    pub invalid: Prop<bool>,
    /// The label dims and the control is disabled. Or'ed with an
    /// enclosing `FieldSet`'s.
    #[prop(default)]
    pub disabled: Prop<bool>,
    /// The parts and the control, in the order shown.
    pub children: Prop<Children>,
}

/// A field: one control with its label (`FieldLabel`), its description
/// (`FieldDescription`) and its error (`FieldError`). Its `invalid` and
/// `disabled` reach the label and the control (the first control of the
/// library made inside it), which its error, while it shows, else its
/// description, describes for screen readers.
#[component]
pub fn Field(props: FieldProps, _cx: Scope<NoEvent>) -> impl IntoElement {
    let t = theme();
    let state = field::provide(move || props.invalid.get(), move || props.disabled.get());
    // A choice card around it: the card's label presses its control.
    let card = use_context::<Option<CardCx>>().flatten();
    if let Some(card) = card {
        card.field.set(Some(state));
        cleanup(move || {
            if untracked(|| card.field.try_get()).flatten().is_some() {
                card.field.try_set(None);
            }
        });
    }
    let group = use_context::<Option<GroupCx>>().flatten();
    if let Some(g) = group
        && props.orientation.get_untracked() == FieldOrientation::Responsive
    {
        g.responsive.update(|n| *n += 1);
        cleanup(move || {
            if let Some(n) = g.responsive.try_get() {
                g.responsive.try_set(n.saturating_sub(1));
            }
        });
    }
    let horizontal = computed(move || match props.orientation.get() {
        FieldOrientation::Vertical => false,
        FieldOrientation::Horizontal => true,
        // Outside a group there is no width to switch on: vertical.
        FieldOrientation::Responsive => group
            .and_then(|g| g.width.get())
            .is_some_and(|w| w >= t.length(FIELD_RESPONSIVE_WIDTH)),
    });
    let contents = signal(0usize);
    let with_content = move || horizontal.get() && contents.get() > 0;
    effect(move || state.set_beside_content(with_content()));
    effect(move || state.set_in_row(horizontal.get()));
    provide_context(Some(FieldCx {
        state,
        horizontal,
        contents,
    }));
    provide_context(None::<CardCx>);
    provide_context(InGroup(false));
    provide_context(InSet(false));
    view()
        .role(AccessibilityRole::Group)
        .w_full()
        .gap(move || t.length(FIELD_GAP))
        .style_with(move |s| {
            if horizontal.get() {
                let s = s.flex_row();
                if with_content() {
                    s.items_start()
                } else {
                    s.items_center()
                }
            } else {
                s.flex_col()
            }
        })
        .style_when(
            move || card.is_some(),
            move |s| s.p(move || t.length(FIELD_CHOICE_CARD_PADDING)),
        )
        .child(props.children)
}

/// The props of [`FieldLabel`].
#[derive(Debug, Clone, PartialEq, Props)]
pub struct FieldLabelProps {
    /// The control a press reaches. Left out: its field's control (around a
    /// `Field`, that field's: a choice card).
    #[prop(default)]
    pub html_for: Prop<FieldRef>,
    /// Dimmed, and a press reaches nothing. Or'ed with its field's.
    #[prop(default)]
    pub disabled: Prop<bool>,
    /// The label's words; or a `Field` (a choice card).
    pub children: Prop<Children>,
}

/// A field's label: a `Label` that knows its field. A press focuses or
/// presses the field's control; it turns red while the field is invalid
/// and dims while it is disabled. Around a `Field`, it is a **choice
/// card**: a bordered card whose whole area presses that field's control,
/// highlighted while the control is on.
#[component]
pub fn FieldLabel(props: FieldLabelProps, _cx: Scope<NoEvent>) -> impl IntoElement {
    provide_text_color();
    let t = theme();
    let outer = use_context::<Option<FieldCx>>().flatten();
    let card = CardCx {
        field: signal(None),
    };
    provide_context(Some(card));
    let is_card = move || card.field.with(Option::is_some);
    let field = move || card.field.get().or(outer.map(|f| f.state));
    let target = move || {
        let given = props.html_for.get();
        if given.is_some() {
            return given;
        }
        field()
            .and_then(|f| f.control())
            .map(FieldRef::node)
            .unwrap_or_default()
    };
    let disabled = move || props.disabled.get() || field().is_some_and(|f| f.disabled());
    let invalid = move || !is_card() && field().is_some_and(|f| f.invalid());
    let checked = move || is_card() && target().checked();
    let horizontal = move || outer.is_some_and(|f| f.horizontal.get());
    let font_size = move || t.length(LABEL_FONT_SIZE);
    view()
        .disabled(disabled)
        .focusable(false)
        .role(move || {
            if is_card() {
                AccessibilityRole::None
            } else {
                AccessibilityRole::Text
            }
        })
        .on_press(move || {
            if !untracked(disabled) {
                untracked(target).press();
            }
        })
        .gap(move || t.length(LABEL_GAP))
        .text_size(font_size)
        .line_height(move || line_height(font_size()))
        .font_weight(weight(t.number(LABEL_FONT_WEIGHT)))
        .text_color(move || {
            if invalid() {
                t.color(FIELD_LABEL_FOREGROUND_INVALID)
            } else {
                t.color(LABEL_FOREGROUND)
            }
        })
        .select_none()
        .cursor(Cursor::Default)
        .style_with(move |s| {
            if is_card() {
                // A column as wide as its parent, the card's edge and fill.
                s.flex_col()
                    .w_full()
                    .rounded(t.length(FIELD_CHOICE_CARD_RADIUS))
                    .border(t.length(BORDER_WIDTH))
                    .border_color(if checked() {
                        t.color(FIELD_CHOICE_CARD_CHECKED_BORDER)
                    } else {
                        t.color(FIELD_CHOICE_CARD_BORDER)
                    })
                    .bg(if checked() {
                        t.color(FIELD_CHOICE_CARD_CHECKED)
                    } else {
                        AdaptiveColor::TRANSPARENT
                    })
            } else {
                // Beside the control, an equal share of the row with a
                // full-width control (so a group's rows line up in
                // columns), the room left beside a switch or a box; never
                // narrower than its longest word, its children stacked:
                // NSH's layout does not shrink a text in a row, so a row
                // here would let a long label run under the control
                // rather than wrap.
                if horizontal() {
                    s.flex_col()
                        .justify_center()
                        .flex_1()
                        .min_w(nsh_core::geometry::auto())
                } else {
                    s.flex_row().items_center()
                }
            }
        })
        // As wide as its words in a column.
        .app_class(move || (!is_card() && !horizontal()).then(|| SharedString::from("w-fit")))
        .disabled_style(move |s| s.opacity(move || t.number(OPACITY_DISABLED)))
        .child(props.children)
}

/// The props of [`FieldDescription`].
#[derive(Debug, Clone, PartialEq, Props)]
pub struct FieldDescriptionProps {
    /// The words (links are the app's elements).
    pub children: Prop<Children>,
}

/// Muted help text under a label, a control or a legend. In a field, it
/// describes the control for screen readers (the first description, while
/// no error shows).
#[component]
pub fn FieldDescription(props: FieldDescriptionProps, _cx: Scope<NoEvent>) -> impl IntoElement {
    let t = theme();
    let font_size = move || t.length(FIELD_DESCRIPTION_FONT_SIZE);
    let element = element_ref();
    if let Some(f) = field::current() {
        f.register_description(element);
    }
    let in_set = use_context::<InSet>().is_some_and(|s| s.0);
    text("")
        .element_ref(&element)
        .text_size(font_size)
        .line_height(move || line_height(font_size()))
        .text_color(move || t.color(FIELD_DESCRIPTION_FOREGROUND))
        // Closer to what is above it: after a set's legend, or under a
        // label (unless it ends its field).
        .app_class(move || {
            let pull = if in_set {
                t.length(SPACING) * 1.5
            } else {
                t.length(SPACING)
            };
            let class = if in_set {
                format!("-mt-[{}px]", pull.0)
            } else {
                format!("-mt-[{}px] last:mt-0", pull.0)
            };
            Some(SharedString::from(class))
        })
        .child(props.children)
}

/// The props of [`FieldError`].
#[derive(Debug, Clone, PartialEq, Props)]
pub struct FieldErrorProps {
    /// The message, shown before `errors`.
    pub children: Prop<Children>,
    /// The messages of a validation library, shown while there are no
    /// children: one as text, several as a list.
    #[prop(default)]
    pub errors: Prop<FieldErrors>,
}

/// The validation message of a field, in red: its children, else its
/// `errors`; with neither it makes nothing. While it shows, screen readers
/// hear it at once and read it with the field's control.
#[component]
pub fn FieldError(props: FieldErrorProps, _cx: Scope<NoEvent>) -> impl IntoElement {
    let t = theme();
    let field = field::current();
    let font_size = move || t.length(FIELD_ERROR_FONT_SIZE);
    let own = move || !props.children.with(|c| c.is_empty());
    let messages = computed(move || props.errors.with(FieldErrors::messages));
    let shown = move || own() || messages.with(|m| !m.is_empty());
    dynamic(move || {
        shown().then(|| {
            let element = element_ref();
            if let Some(f) = field {
                f.register_error(element);
            }
            let body: AnyElement = if untracked(own) {
                text("").child(props.children).into_element()
            } else {
                dynamic(move || {
                    let list = messages.get();
                    if list.len() == 1 {
                        text(list[0].clone()).into_element()
                    } else {
                        view()
                            .flex_col()
                            .gap(move || t.length(SPACING))
                            .children(list.into_iter().map(move |m| {
                                view()
                                    .flex_row()
                                    .child(text("•").w(move || t.length(SPACING) * 4.0))
                                    .child(text(m).flex_1())
                            }))
                            .into_element()
                    }
                })
            };
            view()
                .element_ref(&element)
                .role(AccessibilityRole::Alert)
                .accessibility_live_region(|| LiveRegion::Assertive)
                .text_size(font_size)
                .line_height(move || line_height(font_size()))
                .text_color(move || t.color(FIELD_ERROR_FOREGROUND))
                .child(body)
        })
    })
}

/// The props of [`FieldContent`].
#[derive(Debug, Clone, PartialEq, Props)]
pub struct FieldContentProps {
    /// A label (or a title) and a description.
    pub children: Prop<Children>,
}

/// A column for a label (or a title) and a description beside a control,
/// in a horizontal field: half the row beside a full-width control (so a
/// group's rows line up), the room left beside a switch or a box; its
/// field aligns to the top, the control centered on its first line.
#[component]
pub fn FieldContent(props: FieldContentProps, _cx: Scope<NoEvent>) -> impl IntoElement {
    let t = theme();
    if let Some(f) = use_context::<Option<FieldCx>>().flatten() {
        f.contents.update(|n| *n += 1);
        cleanup(move || {
            if let Some(n) = f.contents.try_get() {
                f.contents.try_set(n.saturating_sub(1));
            }
        });
    }
    // `flex: 1 1 0`: an equal share of the row with a full-width control
    // (which takes `flex: 1 1 0` there too), so the rows of a group line
    // up in columns; the room left beside a switch or a box. Never
    // narrower than its longest word (CSS's automatic minimum).
    view()
        .flex_col()
        .flex_1()
        .min_w(nsh_core::geometry::auto())
        .gap(move || t.length(FIELD_CONTENT_GAP))
        .child(props.children)
}

/// The props of [`FieldTitle`].
#[derive(Debug, Clone, PartialEq, Props)]
pub struct FieldTitleProps {
    /// The title's words.
    pub children: Prop<Children>,
}

/// A title that looks like a label but is none: inside a choice card,
/// where the card is the label. It follows its field's `invalid` and
/// `disabled` as a `FieldLabel`.
#[component]
pub fn FieldTitle(props: FieldTitleProps, _cx: Scope<NoEvent>) -> impl IntoElement {
    let t = theme();
    let field = field::current();
    let font_size = move || t.length(LABEL_FONT_SIZE);
    view()
        .flex_row()
        .items_center()
        .gap(move || t.length(LABEL_GAP))
        .text_size(font_size)
        .line_height(move || line_height(font_size()))
        .font_weight(weight(t.number(LABEL_FONT_WEIGHT)))
        .text_color(move || {
            if field.is_some_and(|f| f.invalid()) {
                t.color(FIELD_LABEL_FOREGROUND_INVALID)
            } else {
                t.color(LABEL_FOREGROUND)
            }
        })
        .opacity(move || {
            if field.is_some_and(|f| f.disabled()) {
                t.number(OPACITY_DISABLED)
            } else {
                1.0
            }
        })
        .app_class(Some(SharedString::from("w-fit")))
        .child(props.children)
}

/// The props of [`FieldGroup`].
#[derive(Debug, Clone, PartialEq, Props)]
pub struct FieldGroupProps {
    /// The fields (and separators, nested groups).
    pub children: Prop<Children>,
}

/// A column of fields, 28 apart (16 for a group directly in another). Its
/// width is what `responsive` fields inside it switch on.
#[component]
pub fn FieldGroup(props: FieldGroupProps, _cx: Scope<NoEvent>) -> impl IntoElement {
    let t = theme();
    let nested = use_context::<InGroup>().is_some_and(|g| g.0);
    let group = GroupCx {
        width: signal(None),
        responsive: signal(0),
    };
    provide_context(Some(group));
    provide_context(InGroup(true));
    provide_context(InSet(false));
    view()
        .flex_col()
        .w_full()
        .gap(move || {
            if nested {
                t.length(FIELD_GROUP_NESTED_GAP)
            } else {
                t.length(FIELD_GROUP_GAP)
            }
        })
        .on_layout(move |frame: &Bounds<Points>| {
            let width = Some(frame.size.width);
            if group.width.get_untracked() != width {
                group.width.set(width);
            }
        })
        .child(props.children)
}

/// The props of [`FieldSet`].
#[derive(Debug, Clone, PartialEq, Props)]
pub struct FieldSetProps {
    /// Every field inside it disabled.
    #[prop(default)]
    pub disabled: Prop<bool>,
    /// What screen readers call the set: its legend's words.
    pub accessibility_label: Prop<Option<SharedString>>,
    /// A `FieldLegend`, a `FieldDescription`, a `FieldGroup` of fields.
    pub children: Prop<Children>,
}

/// A set of related fields under a legend, 24 apart. Its `disabled`
/// disables every control inside it.
#[component]
pub fn FieldSet(props: FieldSetProps, _cx: Scope<NoEvent>) -> impl IntoElement {
    let t = theme();
    field::provide_set(move || props.disabled.get());
    provide_context(InSet(true));
    provide_context(InGroup(false));
    view()
        .role(AccessibilityRole::Group)
        .optional_label(props.accessibility_label)
        .flex_col()
        .w_full()
        .gap(move || t.length(FIELD_SET_GAP))
        .child(props.children)
}

/// The props of [`FieldLegend`].
#[derive(Debug, Clone, PartialEq, Props)]
pub struct FieldLegendProps {
    /// `legend` (16) or `label` (14: a set that reads as one field).
    #[prop(default)]
    pub variant: Prop<FieldLegendVariant>,
    /// The set's title.
    pub children: Prop<Children>,
}

/// A set's title.
#[component]
pub fn FieldLegend(props: FieldLegendProps, _cx: Scope<NoEvent>) -> impl IntoElement {
    let t = theme();
    let font_size = move || match props.variant.get() {
        FieldLegendVariant::Legend => t.length(FIELD_LEGEND_FONT_SIZE),
        FieldLegendVariant::Label => t.length(LABEL_FONT_SIZE),
    };
    text("")
        .mb(move || t.length(FIELD_LEGEND_MARGIN_BOTTOM))
        .text_size(font_size)
        .line_height(move || line_height(font_size()))
        .font_weight(medium(t))
        .text_color(move || t.color(FIELD_LEGEND_FOREGROUND))
        .child(props.children)
}

/// The props of [`FieldSeparator`].
#[derive(Debug, Clone, PartialEq, Props)]
pub struct FieldSeparatorProps {
    /// The word on the line ("Or continue with"), optional.
    pub children: Prop<Children>,
    /// Tailwind classes of the word's box.
    pub content_class: Prop<Option<SharedString>>,
    /// The inline style of the word's box.
    #[prop(default)]
    pub content_style: Prop<Style>,
}

/// A line between groups of fields, optionally with a word on it.
#[component]
pub fn FieldSeparator(props: FieldSeparatorProps, _cx: Scope<NoEvent>) -> impl IntoElement {
    let t = theme();
    let worded = !props.children.get_untracked().is_empty();
    view()
        .w_full()
        .h(move || t.length(SPACING) * 5.0)
        .my(move || -(t.length(SPACING) * 2.0))
        .items_center()
        .justify_center()
        .child(
            separator()
                .absolute()
                .left(px(0.0))
                .right(px(0.0))
                .top(move || t.length(SPACING) * 2.5 - px(0.5)),
        )
        .when(worded, move |v| {
            v.child(
                text("")
                    .px(move || t.length(SPACING) * 2.0)
                    .text_size(move || t.length(TEXT_SM))
                    .line_height(move || line_height(t.length(TEXT_SM)))
                    .text_color(move || t.color(FIELD_SEPARATOR_CONTENT_FOREGROUND))
                    .bg(move || t.color(FIELD_SEPARATOR_CONTENT_BACKGROUND))
                    .app_class(props.content_class)
                    .inline_style(move || props.content_style.get())
                    .child(props.children),
            )
        })
}
