//! Semantic tokens: shadcn/ui's names (whose defaults are `nsh/base`'s
//! theme, shadcn's `neutral`), the colors this library adds, and the scales
//! the seeds derive.

use crate::{
    theme::seeds::{CONTROL_HEIGHT, CONTROL_HEIGHT_TOUCH, DURATION, FONT_SIZE, RADIUS},
    tokens,
};

tokens! {
    /// The app's background.
    BACKGROUND: Color = "background",
    /// Text and icons on `background`.
    FOREGROUND: Color = "foreground",
    /// Cards.
    CARD: Color = "card",
    /// Text on `card`.
    CARD_FOREGROUND: Color = "card-foreground",
    /// Popovers, menus, tooltips.
    POPOVER: Color = "popover",
    /// Text on `popover`.
    POPOVER_FOREGROUND: Color = "popover-foreground",
    /// The main action.
    PRIMARY: Color = "primary",
    /// Text on `primary`.
    PRIMARY_FOREGROUND: Color = "primary-foreground",
    /// Secondary actions.
    SECONDARY: Color = "secondary",
    /// Text on `secondary`.
    SECONDARY_FOREGROUND: Color = "secondary-foreground",
    /// Muted surfaces.
    MUTED: Color = "muted",
    /// Secondary text: hints, captions, placeholders.
    MUTED_FOREGROUND: Color = "muted-foreground",
    /// What is hovered or highlighted: menu items, ghost buttons.
    ACCENT: Color = "accent",
    /// Text on `accent`.
    ACCENT_FOREGROUND: Color = "accent-foreground",
    /// Destructive actions and errors.
    DESTRUCTIVE: Color = "destructive",
    /// Borders and dividers.
    BORDER: Color = "border",
    /// The borders of inputs (and outline buttons in the dark).
    INPUT: Color = "input",
    /// Focus rings.
    RING: Color = "ring",
    /// A chart's first series.
    CHART_1: Color = "chart-1",
    /// A chart's second series.
    CHART_2: Color = "chart-2",
    /// A chart's third series.
    CHART_3: Color = "chart-3",
    /// A chart's fourth series.
    CHART_4: Color = "chart-4",
    /// A chart's fifth series.
    CHART_5: Color = "chart-5",
    /// The sidebar.
    SIDEBAR: Color = "sidebar",
    /// Text on `sidebar`.
    SIDEBAR_FOREGROUND: Color = "sidebar-foreground",
    /// The sidebar's main action.
    SIDEBAR_PRIMARY: Color = "sidebar-primary",
    /// Text on `sidebar-primary`.
    SIDEBAR_PRIMARY_FOREGROUND: Color = "sidebar-primary-foreground",
    /// What is hovered in the sidebar.
    SIDEBAR_ACCENT: Color = "sidebar-accent",
    /// Text on `sidebar-accent`.
    SIDEBAR_ACCENT_FOREGROUND: Color = "sidebar-accent-foreground",
    /// The sidebar's borders.
    SIDEBAR_BORDER: Color = "sidebar-border",
    /// Focus rings in the sidebar.
    SIDEBAR_RING: Color = "sidebar-ring",

    /// Text on `destructive` (shadcn writes `text-white`).
    DESTRUCTIVE_FOREGROUND: Color = "destructive-foreground" => |t| t.css("oklch(1 0 0)"),
    /// Success: a confirmed result, a positive state.
    SUCCESS: Color = "success" => |t| t.light_dark(t.css("oklch(0.627 0.194 149.214)"), t.css("oklch(0.723 0.219 149.579)")),
    /// Text on `success`.
    SUCCESS_FOREGROUND: Color = "success-foreground" => |t| t.light_dark(t.css("oklch(0.985 0 0)"), t.css("oklch(0.266 0.065 152.934)")),
    /// Warning: something needs attention.
    WARNING: Color = "warning" => |t| t.light_dark(t.css("oklch(0.769 0.188 70.08)"), t.css("oklch(0.828 0.189 84.429)")),
    /// Text on `warning`.
    WARNING_FOREGROUND: Color = "warning-foreground" => |t| t.css("oklch(0.279 0.077 45.635)"),
    /// Information.
    INFO: Color = "info" => |t| t.light_dark(t.css("oklch(0.546 0.245 262.881)"), t.css("oklch(0.623 0.214 259.815)")),
    /// Text on `info`.
    INFO_FOREGROUND: Color = "info-foreground" => |t| t.css("oklch(0.985 0 0)"),
    /// The veil behind a modal (shadcn's `bg-black/50`).
    OVERLAY: Color = "overlay" => |t| t.css("oklch(0 0 0 / 50%)"),
    /// The color of shadows; each shadow level scales its alpha.
    SHADOW: Color = "shadow" => |t| t.css("oklch(0 0 0)"),

    /// The smallest corners (a dialog's close button, Tailwind's
    /// `rounded-xs`): `radius` − 8, never below 0.
    RADIUS_XS: Length = "radius-xs" => |t| t.max(t.length(RADIUS) - 8.0, t.px(0.0)),
    /// Small corners (badges, checkboxes): `radius` − 4.
    RADIUS_SM: Length = "radius-sm" => |t| t.length(RADIUS) - 4.0,
    /// Controls' corners (buttons, inputs): `radius` − 2.
    RADIUS_MD: Length = "radius-md" => |t| t.length(RADIUS) - 2.0,
    /// Larger surfaces (popovers): `radius`.
    RADIUS_LG: Length = "radius-lg" => |t| t.length(RADIUS),
    /// Cards and dialogs: `radius` + 4.
    RADIUS_XL: Length = "radius-xl" => |t| t.length(RADIUS) + 4.0,

    /// Extra small controls with a pointer: `control-height` − 8.
    CONTROL_HEIGHT_XS: Length = "control-height-xs" => |t| t.length(CONTROL_HEIGHT) - 8.0,
    /// Small controls with a pointer: `control-height` − 4.
    CONTROL_HEIGHT_SM: Length = "control-height-sm" => |t| t.length(CONTROL_HEIGHT) - 4.0,
    /// Medium controls with a pointer: `control-height`.
    CONTROL_HEIGHT_MD: Length = "control-height-md" => |t| t.length(CONTROL_HEIGHT),
    /// Large controls with a pointer: `control-height` + 4.
    CONTROL_HEIGHT_LG: Length = "control-height-lg" => |t| t.length(CONTROL_HEIGHT) + 4.0,
    /// Extra small controls on touch screens: `control-height-touch` − 8.
    CONTROL_HEIGHT_TOUCH_XS: Length = "control-height-touch-xs" => |t| t.length(CONTROL_HEIGHT_TOUCH) - 8.0,
    /// Small controls on touch screens: `control-height-touch` − 4.
    CONTROL_HEIGHT_TOUCH_SM: Length = "control-height-touch-sm" => |t| t.length(CONTROL_HEIGHT_TOUCH) - 4.0,
    /// Medium controls on touch screens: `control-height-touch`.
    CONTROL_HEIGHT_TOUCH_MD: Length = "control-height-touch-md" => |t| t.length(CONTROL_HEIGHT_TOUCH),
    /// Large controls on touch screens: `control-height-touch` + 8.
    CONTROL_HEIGHT_TOUCH_LG: Length = "control-height-touch-lg" => |t| t.length(CONTROL_HEIGHT_TOUCH) + 8.0,
    /// The smallest target a finger gets: a smaller control takes presses
    /// this far around it, without looking bigger.
    TOUCH_TARGET: Length = "touch-target" => |t| t.px(44.0),

    /// Extra small controls' text: `font-size` − 2.
    FONT_SIZE_XS: Length = "font-size-xs" => |t| t.length(FONT_SIZE) - 2.0,
    /// Small controls' text: `font-size` − 1.
    FONT_SIZE_SM: Length = "font-size-sm" => |t| t.length(FONT_SIZE) - 1.0,
    /// Medium controls' text: `font-size`.
    FONT_SIZE_MD: Length = "font-size-md" => |t| t.length(FONT_SIZE),
    /// Large controls' text: `font-size`.
    FONT_SIZE_LG: Length = "font-size-lg" => |t| t.length(FONT_SIZE),
    /// Extra small controls' icons: `font-size` − 2 (12).
    ICON_SIZE_XS: Length = "icon-size-xs" => |t| t.length(FONT_SIZE) - 2.0,
    /// Small controls' icons: `font-size` (14).
    ICON_SIZE_SM: Length = "icon-size-sm" => |t| t.length(FONT_SIZE),
    /// Medium controls' icons: `font-size` + 2 (16, shadcn's `size-4`).
    ICON_SIZE_MD: Length = "icon-size-md" => |t| t.length(FONT_SIZE) + 2.0,
    /// Large controls' icons: `font-size` + 2 (16: shadcn's buttons use
    /// `size-4` at every size but `xs`).
    ICON_SIZE_LG: Length = "icon-size-lg" => |t| t.length(FONT_SIZE) + 2.0,

    /// Tailwind's `text-xs`.
    TEXT_XS: Length = "text-xs" => |t| t.px(12.0),
    /// Tailwind's `text-sm`.
    TEXT_SM: Length = "text-sm" => |t| t.px(14.0),
    /// Tailwind's `text-base`.
    TEXT_BASE: Length = "text-base" => |t| t.px(16.0),
    /// Tailwind's `text-lg`.
    TEXT_LG: Length = "text-lg" => |t| t.px(18.0),
    /// Medium weight: controls' labels.
    FONT_WEIGHT_MEDIUM: Number = "font-weight-medium" => |t| t.value(500.0),
    /// Semibold weight: titles.
    FONT_WEIGHT_SEMIBOLD: Number = "font-weight-semibold" => |t| t.value(600.0),

    /// Borders' width.
    BORDER_WIDTH: Length = "border-width" => |t| t.px(1.0),
    /// The focus ring's width (shadcn's `ring-[3px]`).
    RING_WIDTH: Length = "ring-width" => |t| t.px(3.0),
    /// The focus ring's opacity over `ring` (shadcn's `ring-ring/50`).
    RING_OPACITY: Number = "ring-opacity" => |t| t.value(0.5),
    /// The ring of a focused control whose value the app rejects:
    /// `destructive` at 20 % (40 % in the dark).
    RING_INVALID: Color = "ring-invalid" => |t| t.light_dark(t.color(DESTRUCTIVE).opacity(0.2), t.color(DESTRUCTIVE).opacity(0.4)),
    /// A disabled control's opacity.
    OPACITY_DISABLED: Number = "opacity-disabled" => |t| t.value(0.5),

    /// Quick transitions in milliseconds: `duration` × 2/3.
    DURATION_FAST: Number = "duration-fast" => |t| t.number(DURATION) * (2.0 / 3.0),
    /// Slow transitions in milliseconds: `duration` × 5/3.
    DURATION_SLOW: Number = "duration-slow" => |t| t.number(DURATION) * (5.0 / 3.0),
}
