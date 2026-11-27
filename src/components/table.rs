//! `Table` and its parts: a table on `nsh/base`'s `Table` parts
//! (the table, row group, row, column header and cell roles for screen
//! readers). The library adds the look, `size`, the header's `sticky` form
//! and the row's `selected` form. Owns the `table-*` tokens, which
//! `DataTable` reads too. API: `docs/components/table.md`.
//!
//! The parts' look is classes generated from the tokens
//! (`foundation::class`), so the app's classes (a column's `w-24
//! flex-none`, a number column's `items-end`, `pr-0`, `border-b-0`) win
//! over it.

use nsh_base::{
    table as base_table, table_body as base_table_body, table_caption as base_table_caption,
    table_cell as base_table_cell, table_footer as base_table_footer,
    table_head as base_table_head, table_header as base_table_header,
};
use nsh_core::{element::TextPartStyle, prelude::*};

use crate::{
    foundation::{
        class::Classes,
        icon::{provide_icon_size, provide_text_color},
        motion,
        size::{Size, pointer},
        trigger,
        typography::{font_size_token, line_height, medium},
    },
    theme::{
        Theme, Token, kind,
        seeds::SPACING,
        semantic::{
            BACKGROUND, BORDER, BORDER_WIDTH, CONTROL_HEIGHT_LG, CONTROL_HEIGHT_MD,
            CONTROL_HEIGHT_SM, CONTROL_HEIGHT_TOUCH_LG, CONTROL_HEIGHT_TOUCH_MD,
            CONTROL_HEIGHT_TOUCH_SM, CONTROL_HEIGHT_TOUCH_XS, CONTROL_HEIGHT_XS, FOREGROUND, MUTED,
            MUTED_FOREGROUND, TEXT_SM,
        },
        theme,
    },
    tokens,
};

tokens! {
    /// An extra small table's body row height (a minimum): the control
    /// height, so a control of the same size fits a row.
    TABLE_ROW_HEIGHT_XS: Length = "table-row-height-xs" => |t| t.pointer_touch(t.length(CONTROL_HEIGHT_XS), t.length(CONTROL_HEIGHT_TOUCH_XS)),
    /// A small table's body row height.
    TABLE_ROW_HEIGHT_SM: Length = "table-row-height-sm" => |t| t.pointer_touch(t.length(CONTROL_HEIGHT_SM), t.length(CONTROL_HEIGHT_TOUCH_SM)),
    /// A medium table's body row height.
    TABLE_ROW_HEIGHT_MD: Length = "table-row-height-md" => |t| t.pointer_touch(t.length(CONTROL_HEIGHT_MD), t.length(CONTROL_HEIGHT_TOUCH_MD)),
    /// A large table's body row height.
    TABLE_ROW_HEIGHT_LG: Length = "table-row-height-lg" => |t| t.pointer_touch(t.length(CONTROL_HEIGHT_LG), t.length(CONTROL_HEIGHT_TOUCH_LG)),
    /// An extra small table's header row height: `spacing` over the body's.
    TABLE_HEAD_HEIGHT_XS: Length = "table-head-height-xs" => |t| t.length(TABLE_ROW_HEIGHT_XS) + t.length(SPACING),
    /// A small table's header row height.
    TABLE_HEAD_HEIGHT_SM: Length = "table-head-height-sm" => |t| t.length(TABLE_ROW_HEIGHT_SM) + t.length(SPACING),
    /// A medium table's header row height.
    TABLE_HEAD_HEIGHT_MD: Length = "table-head-height-md" => |t| t.length(TABLE_ROW_HEIGHT_MD) + t.length(SPACING),
    /// A large table's header row height.
    TABLE_HEAD_HEIGHT_LG: Length = "table-head-height-lg" => |t| t.length(TABLE_ROW_HEIGHT_LG) + t.length(SPACING),
    /// A cell's padding at the sides.
    TABLE_CELL_PADDING_X: Length = "table-cell-padding-x" => |t| t.length(SPACING) * 2.0,
    /// A cell's padding above and below.
    TABLE_CELL_PADDING_Y: Length = "table-cell-padding-y" => |t| t.length(SPACING),
    /// The edges between rows and above the footer.
    TABLE_BORDER: Color = "table-border" => |t| t.color(BORDER),
    /// A column title's text.
    TABLE_HEAD_FOREGROUND: Color = "table-head-foreground" => |t| t.color(FOREGROUND),
    /// A row's fill under the pointer, and while a popup it holds is open.
    TABLE_ROW_HOVER: Color = "table-row-hover" => |t| t.color(MUTED).opacity(0.5),
    /// A selected row's fill.
    TABLE_ROW_SELECTED: Color = "table-row-selected" => |t| t.color(MUTED),
    /// The footer's fill.
    TABLE_FOOTER: Color = "table-footer" => |t| t.color(MUTED).opacity(0.5),
    /// A `sticky` header's fill, so the rows do not show through it.
    TABLE_STICKY_HEADER: Color = "table-sticky-header" => |t| t.color(BACKGROUND),
    /// The caption's text.
    TABLE_CAPTION_FOREGROUND: Color = "table-caption-foreground" => |t| t.color(MUTED_FOREGROUND),
    /// The space between the table and its caption.
    TABLE_CAPTION_GAP: Length = "table-caption-gap" => |t| t.length(SPACING) * 4.0,
}

/// The body row height token of `size`.
pub fn row_height_token(size: Size) -> Token<kind::Length> {
    size.pick(
        TABLE_ROW_HEIGHT_XS,
        TABLE_ROW_HEIGHT_SM,
        TABLE_ROW_HEIGHT_MD,
        TABLE_ROW_HEIGHT_LG,
    )
}

/// The header row height token of `size`.
pub fn head_height_token(size: Size) -> Token<kind::Length> {
    size.pick(
        TABLE_HEAD_HEIGHT_XS,
        TABLE_HEAD_HEIGHT_SM,
        TABLE_HEAD_HEIGHT_MD,
        TABLE_HEAD_HEIGHT_LG,
    )
}

/// The table's size, for its parts.
#[derive(Clone, Copy)]
struct TableSize(Memo<Size>);

/// The size of the table around: `md` outside one.
fn table_size() -> Size {
    use_context::<TableSize>().map_or(Size::Md, |s| s.0.get())
}

/// Which row group a row is in.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Group {
    Header,
    Body,
    Footer,
}

/// The library controls a cell holds that want no end padding beside them
/// (a `Checkbox`), and whether it has held a control at all.
#[derive(Clone, Copy)]
struct CellControls(Signal<u32>, Signal<bool>);

/// Registers a library control with the `TableHead` / `TableCell` it is
/// in, while it is mounted: the cell drops its end padding (NSH has no
/// `:has()`) and places its content as it is ([`cell_content`]). Nothing
/// outside a cell. Call it in the control's body.
pub(crate) fn register_cell_control() {
    if let Some(CellControls(count, holds)) = use_context::<Option<CellControls>>().flatten() {
        if !holds.get_untracked() {
            holds.set(true);
        }
        count.update(|n| *n += 1);
        cleanup(move || count.update(|n| *n = n.saturating_sub(1)));
    }
}

/// Registers a library element that is not text (a `Skeleton`, a
/// `Progress`) with the `TableHead` / `TableCell` it is in: the cell places
/// its content as it is ([`cell_content`]), keeping its padding, so the
/// element takes the cell's width (inside a text it has none). Nothing
/// outside a cell. Call it in the element's body.
pub(crate) fn register_cell_block() {
    if let Some(CellControls(_, holds)) = use_context::<Option<CellControls>>().flatten()
        && !holds.get_untracked()
    {
        holds.set(true);
    }
}

/// The props of [`Table`].
#[derive(Debug, Clone, PartialEq, Props)]
pub struct TableProps {
    /// Its size (`xs`, `sm`, `md`, `lg`): the row heights and type of
    /// every part.
    #[prop(default)]
    pub size: Prop<Size>,
    /// What screen readers call the table. A `TableCaption` is shown, not
    /// its name: set both when there is one.
    pub accessibility_label: Prop<Option<SharedString>>,
    /// Its parts: a `TableHeader`, `TableBody`, `TableFooter` and
    /// `TableCaption`.
    pub children: Prop<Children>,
}

/// A table of rows the app writes: a header, a body, a footer
/// and a caption, rows of cells sharing the width. Four sizes; rows lit
/// under the pointer and when `selected`. For sorting, selection, many
/// rows or columns the user resizes, use `DataTable`.
#[component]
pub fn Table(props: TableProps, _cx: Scope<NoEvent>) -> impl IntoElement {
    provide_text_color();
    let t = theme();
    let size = computed(move || props.size.get());
    provide_context(TableSize(size));
    provide_icon_size(move || size.get());
    provide_context(None::<CellControls>);
    base_table()
        .accessibility_label(props.accessibility_label)
        .cursor(Cursor::Default)
        .app_class(move || {
            let font_size = t.length(font_size_token(size.get()));
            Some(
                Classes::new()
                    .push("flex-col w-full tabular-nums")
                    .color("", "text", t.color(FOREGROUND))
                    .length("", "text", font_size)
                    .length("", "leading", line_height(font_size))
                    .build(),
            )
        })
        .child(props.children)
}

/// The props of [`TableHeader`].
#[derive(Debug, Clone, PartialEq, Props)]
pub struct TableHeaderProps {
    /// Stays at the top of the nearest scroll view while the body scrolls
    /// under it, filled with `table-sticky-header` so the rows do not show
    /// through.
    #[prop(default)]
    pub sticky: Prop<bool>,
    /// Its rows of `TableHead`s.
    pub children: Prop<Children>,
}

/// The header of a [`Table`]: the rows of column titles. Its rows keep
/// their bottom edge and take no hover.
#[component]
pub fn TableHeader(props: TableHeaderProps, _cx: Scope<NoEvent>) -> impl IntoElement {
    let t = theme();
    provide_context(Group::Header);
    base_table_header()
        .app_class(move || {
            Some(
                Classes::new()
                    .push("flex-col")
                    .push_if(props.sticky.get(), "sticky top-0 z-10")
                    .color(
                        "",
                        "bg",
                        if props.sticky.get() {
                            t.color(TABLE_STICKY_HEADER)
                        } else {
                            AdaptiveColor::TRANSPARENT
                        },
                    )
                    .build(),
            )
        })
        .child(props.children)
}

/// The props of [`TableBody`] and [`TableFooter`].
#[derive(Debug, Clone, PartialEq, Props)]
pub struct TableBodyProps {
    /// Its rows.
    pub children: Prop<Children>,
}

/// The body of a [`Table`]: its rows of data. Its last row draws no bottom
/// edge.
#[component]
pub fn TableBody(props: TableBodyProps, _cx: Scope<NoEvent>) -> impl IntoElement {
    provide_context(Group::Body);
    base_table_body()
        .app_class(Some(SharedString::from("flex-col")))
        .child(props.children)
}

/// The footer of a [`Table`]: totals, a summary row, on a muted fill under
/// an edge, in the medium weight. Its last row draws no bottom edge.
#[component]
pub fn TableFooter(props: TableBodyProps, _cx: Scope<NoEvent>) -> impl IntoElement {
    let t = theme();
    provide_context(Group::Footer);
    base_table_footer()
        .font_weight(medium(t))
        .app_class(move || {
            Some(
                Classes::new()
                    .push("flex-col")
                    .length("", "border-t", t.length(BORDER_WIDTH))
                    .color("", "border", t.color(TABLE_BORDER))
                    .color("", "bg", t.color(TABLE_FOOTER))
                    .build(),
            )
        })
        .child(props.children)
}

/// The props of [`TableRow`].
#[derive(Debug, Clone, PartialEq, Props)]
pub struct TableRowProps {
    /// The row is selected: the selected fill; screen readers hear it
    /// selected.
    #[prop(default)]
    pub selected: Prop<bool>,
    /// Its cells: `TableHead`s in the header, `TableCell`s elsewhere.
    pub children: Prop<Children>,
}

/// A row of a [`Table`]: its cells side by side over a bottom edge, lit
/// under the pointer (not in the header), while a popup it holds is open,
/// and when `selected`.
#[component]
pub fn TableRow(props: TableRowProps, _cx: Scope<NoEvent>) -> impl IntoElement {
    let t = theme();
    let group = use_context::<Group>().unwrap_or(Group::Body);
    let size = table_size;
    // The library's overlays in it report here while open (NSH has no
    // `has-aria-expanded:`).
    let open = signal(0u32);
    trigger::provide_open_inside(open);
    let selected = props.selected;
    // `nsh/base`'s `TableRow` is a row view with the row role; drawn here
    // so it can also say it is selected.
    let row = view()
        .flex_row()
        .role(AccessibilityRole::Row)
        .accessibility_state(move || AccessibilityState {
            selected: selected.get(),
            ..AccessibilityState::default()
        })
        .app_class(move || Some(row_class(t, group, size(), selected.get(), open.get() > 0)))
        .transition_colors();
    motion::timing(row, t).child(props.children)
}

/// A row's classes: its height, its bottom edge (none on the last row of
/// the body or the footer), its fill.
fn row_class(t: Theme, group: Group, size: Size, selected: bool, lit: bool) -> SharedString {
    let height = match group {
        Group::Header => t.length(head_height_token(size)),
        _ => t.length(row_height_token(size)),
    };
    let fill = if selected {
        t.color(TABLE_ROW_SELECTED)
    } else if lit {
        t.color(TABLE_ROW_HOVER)
    } else {
        AdaptiveColor::TRANSPARENT
    };
    let mut c = Classes::new()
        .push("flex-row items-stretch")
        .length("", "min-h", height)
        .length("", "border-b", t.length(BORDER_WIDTH))
        .color("", "border", t.color(TABLE_BORDER))
        .push_if(group != Group::Header, "last:border-b-0")
        .color("", "bg", fill);
    if group != Group::Header && pointer() {
        let hover = if selected {
            t.color(TABLE_ROW_SELECTED)
        } else {
            t.color(TABLE_ROW_HOVER)
        };
        c = c.color("hover", "bg", hover);
    }
    c.build()
}

/// The props of [`TableHead`] and [`TableCell`].
#[derive(Debug, Clone, PartialEq, Props)]
pub struct TableCellProps {
    /// Tailwind classes of the label: the text around each child (one
    /// line, cut with an ellipsis; `whitespace-normal` wraps it).
    pub label_class: Prop<Option<SharedString>>,
    /// The inline style of the label.
    #[prop(default)]
    pub label_style: Prop<TextPartStyle>,
    /// Its content: a string, or any element.
    pub children: Prop<Children>,
}

/// A column's title cell, in a [`TableHeader`]'s row: medium weight, one
/// line. Every cell takes an equal share of the row; a column of its own
/// width gives each of its cells the same classes (`w-24 flex-none`).
#[component]
pub fn TableHead(props: TableCellProps, _cx: Scope<NoEvent>) -> impl IntoElement {
    let t = theme();
    let controls = signal(0u32);
    let holds = signal(false);
    provide_context(Some(CellControls(controls, holds)));
    trigger::provide_holds_overlay(holds);
    base_table_head()
        .font_weight(medium(t))
        .app_class(move || Some(cell_class(t, controls.get() > 0, true)))
        .children_each(props.children, move |child, _| {
            cell_content(holds, props.label_class, props.label_style, child)
        })
}

/// A cell of a [`TableRow`]: one line, vertically centered, at the start
/// (`items-end` puts it at the end). Every cell takes an equal share of the
/// row; a column of its own width gives each of its cells the same classes
/// (`w-24 flex-none`).
#[component]
pub fn TableCell(props: TableCellProps, _cx: Scope<NoEvent>) -> impl IntoElement {
    let t = theme();
    let controls = signal(0u32);
    let holds = signal(false);
    provide_context(Some(CellControls(controls, holds)));
    trigger::provide_holds_overlay(holds);
    base_table_cell()
        .app_class(move || Some(cell_class(t, controls.get() > 0, false)))
        .children_each(props.children, move |child, _| {
            cell_content(holds, props.label_class, props.label_style, child)
        })
}

/// A cell's classes: an equal share of the row, its padding (none at the
/// end beside a control), its content centered on the row at the start.
fn cell_class(t: Theme, control: bool, head: bool) -> SharedString {
    let x = t.length(TABLE_CELL_PADDING_X);
    let c = Classes::new()
        .push("flex-1 min-w-0 flex-col justify-center items-start")
        .length("", "pl", x)
        .length("", "pr", if control { px(0.0) } else { x })
        .length("", "py", t.length(TABLE_CELL_PADDING_Y));
    if head {
        c.color("", "text", t.color(TABLE_HEAD_FOREGROUND)).build()
    } else {
        c.build()
    }
}

/// A cell's child: in a text part ([`cell_label`]); once the cell holds a
/// library control or an overlay, as it is, so the control is centered on
/// the row as the text is (inside a text it would sit on the baseline,
/// higher). A cell that held one keeps that form.
fn cell_content(
    holds: Signal<bool>,
    class: Prop<Option<SharedString>>,
    style: Prop<TextPartStyle>,
    child: Child,
) -> AnyElement {
    dynamic(move || {
        if holds.get() {
            child.into_element()
        } else {
            cell_label(class, style, child).into_element()
        }
    })
}

/// The text part around a cell's child: one line cut with an ellipsis,
/// then the app's `labelClass`. Tailwind's `truncate` written out: NSH's
/// `truncate` also clamps the text to one line, which a later
/// `whitespace-normal` does not lift.
fn cell_label(
    class: Prop<Option<SharedString>>,
    style: Prop<TextPartStyle>,
    child: impl IntoElement + 'static,
) -> impl IntoElement {
    text("")
        .app_class(move || {
            Some(
                Classes::new()
                    .push("whitespace-nowrap overflow-hidden text-ellipsis max-w-full")
                    .then(class.get()),
            )
        })
        .inline_style(move || nsh_core::element::Style::from(style.get()))
        .child(child)
}

/// The props of [`TableCaption`].
#[derive(Debug, Clone, PartialEq, Props)]
pub struct TableCaptionProps {
    /// Tailwind classes of the label: the text around each child.
    pub label_class: Prop<Option<SharedString>>,
    /// The inline style of the label.
    #[prop(default)]
    pub label_style: Prop<TextPartStyle>,
    /// The caption: a string, or any element.
    pub children: Prop<Children>,
}

/// Text about a [`Table`], under it: muted, centered. Screen readers read
/// it as text after the table; it does not name it (`accessibilityLabel`
/// does).
#[component]
pub fn TableCaption(props: TableCaptionProps, _cx: Scope<NoEvent>) -> impl IntoElement {
    let t = theme();
    base_table_caption()
        .app_class(move || {
            let font_size = t.length(TEXT_SM);
            Some(
                Classes::new()
                    .push("flex-col items-center")
                    .length("", "mt", t.length(TABLE_CAPTION_GAP))
                    .length("", "text", font_size)
                    .length("", "leading", line_height(font_size))
                    .color("", "text", t.color(TABLE_CAPTION_FOREGROUND))
                    .build(),
            )
        })
        .children_each(props.children, move |child, _| {
            text("")
                .app_class(move || {
                    Some(
                        Classes::new()
                            .push("text-center")
                            .then(props.label_class.get()),
                    )
                })
                .inline_style(move || nsh_core::element::Style::from(props.label_style.get()))
                .child(child)
        })
}
