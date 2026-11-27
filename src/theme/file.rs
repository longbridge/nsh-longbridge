//! The registry of every token the library declares, and what is made of
//! it: `themes/default.json` and the token tables of `docs/components/`.

use std::fmt::Write as _;

use nsh_core::{Color, JsonValue, color::Hsla, element::Tokens};

use super::{
    resolve::Cx,
    token::{ColorExpr, DeclDefault, Derive, NumExpr, TokenDecl},
};

/// Every token the library declares, seeds first, then semantic tokens,
/// then each component's: the order of `themes/default.json`.
pub fn registry() -> Vec<TokenDecl> {
    [
        super::seeds::TOKENS,
        super::semantic::TOKENS,
        crate::foundation::field::TOKENS,
        crate::components::badge::TOKENS,
        crate::components::button::TOKENS,
        crate::components::label::TOKENS,
        crate::components::textarea::TOKENS,
        crate::components::checkbox::TOKENS,
        crate::components::switch::TOKENS,
        crate::components::radio_group::TOKENS,
        crate::components::card::TOKENS,
        crate::components::kbd::TOKENS,
        crate::components::separator::TOKENS,
        crate::components::skeleton::TOKENS,
        crate::components::tooltip::TOKENS,
        crate::foundation::popup::TOKENS,
        crate::components::popover::TOKENS,
        crate::components::dialog::TOKENS,
        crate::components::alert_dialog::TOKENS,
        crate::components::toast::TOKENS,
        crate::foundation::menu::TOKENS,
        crate::components::tabs::TOKENS,
        crate::components::select::TOKENS,
        crate::components::calendar::TOKENS,
        crate::components::date_picker::TOKENS,
        crate::components::table::TOKENS,
        crate::components::data_table::TOKENS,
        crate::components::avatar::TOKENS,
        crate::components::progress::TOKENS,
        crate::components::pagination::TOKENS,
        crate::components::toggle::TOKENS,
        crate::components::toggle_group::TOKENS,
        crate::components::slider::TOKENS,
        crate::components::combobox::TOKENS,
    ]
    .concat()
}

/// The tokens the app would have with no theme of its own: `nsh/base`'s
/// defaults (the `neutral` palette).
fn default_app_tokens() -> Tokens {
    nsh_base::Theme::default().into_tokens()
}

/// `nsh/base`'s default theme file, for the values of the base tokens.
fn base_theme() -> JsonValue {
    serde_json::from_str(nsh_base::DEFAULT_THEME).expect("nsh/base's default theme is JSON")
}

/// A token's default as CSS, in one appearance (`dark`), or `None` when
/// neither the library nor `nsh/base` gives one.
pub(crate) fn default_css(decl: &TokenDecl, dark: bool) -> Option<String> {
    let app = default_app_tokens();
    let cx = Cx::new(None, &app, true);
    match decl.default {
        DeclDefault::Color(Some(f)) => Some(color_css(&f(&Derive(())), dark, &cx)),
        DeclDefault::Length(Some(f)) | DeclDefault::Number(Some(f)) => {
            Some(num_css_top(&f(&Derive(())), true))
        }
        DeclDefault::Color(None) | DeclDefault::Length(None) | DeclDefault::Number(None) => {
            let base = base_theme();
            let vars = &base["cssVars"];
            let section = if dark { "dark" } else { "light" };
            vars[section][decl.name]
                .as_str()
                .or_else(|| vars["light"][decl.name].as_str())
                .or_else(|| vars["theme"][decl.name].as_str())
                .map(str::to_owned)
        }
    }
}

fn color_css(expr: &ColorExpr, dark: bool, cx: &Cx<'_>) -> String {
    match expr {
        ColorExpr::Token(t) => format!("var(--{})", t.name()),
        ColorExpr::Css(css) => (*css).to_owned(),
        ColorExpr::Fixed(c) => hex(if dark { c.dark } else { c.light }),
        ColorExpr::Opacity(c, alpha) => {
            // A theme file has no number references in colors: an alpha
            // from a token is written with its default value.
            let alpha = cx.eval_num(alpha);
            format!(
                "color-mix(in oklab, {} {}%, transparent)",
                color_css(c, dark, cx),
                number(alpha * 100.0)
            )
        }
        ColorExpr::LightDark(l, d) => color_css(if dark { d } else { l }, dark, cx),
    }
}

/// A number default as a theme file writes it: a literal as it is, an
/// expression in `calc()` (which NSH leaves out of the app's tokens, so the
/// library derives it; a bare `var()` would be read as a color).
fn num_css_top(expr: &NumExpr, pointer: bool) -> String {
    match expr {
        NumExpr::Value { .. } => num_css(expr, pointer),
        _ => format!("calc({})", num_css(expr, pointer)),
    }
}

fn num_css(expr: &NumExpr, pointer: bool) -> String {
    match expr {
        NumExpr::Token { name, .. } => format!("var(--{name})"),
        NumExpr::Value { value, px } => unit(*value, *px),
        NumExpr::Add(a, b) => {
            let sign = if *b < 0.0 { '-' } else { '+' };
            format!("{} {sign} {}", num_css(a, pointer), unit(b.abs(), is_px(a)))
        }
        NumExpr::Mul(a, k) => match **a {
            NumExpr::Add(..) | NumExpr::Diff(..) | NumExpr::Sum(..) => {
                format!("({}) * {}", num_css(a, pointer), number(*k))
            }
            _ => format!("{} * {}", num_css(a, pointer), number(*k)),
        },
        NumExpr::Div(a, k) => match **a {
            NumExpr::Add(..) | NumExpr::Diff(..) | NumExpr::Sum(..) => {
                format!("({}) / {}", num_css(a, pointer), number(*k))
            }
            _ => format!("{} / {}", num_css(a, pointer), number(*k)),
        },
        NumExpr::Diff(a, b) => format!("{} - {}", num_css(a, pointer), num_css_operand(b, pointer)),
        NumExpr::Sum(a, b) => format!("{} + {}", num_css(a, pointer), num_css_operand(b, pointer)),
        // A theme file is the same on every device: it writes the
        // pointer's value (`<name>-touch` sets the touch one).
        NumExpr::PointerTouch(p, t) => num_css(if pointer { p } else { t }, pointer),
        NumExpr::Max(a, b) => format!("max({}, {})", num_css(a, pointer), num_css(b, pointer)),
        NumExpr::Min(a, b) => format!("min({}, {})", num_css(a, pointer), num_css(b, pointer)),
    }
}

/// The right operand of a sum or a difference: a sum in parentheses.
fn num_css_operand(expr: &NumExpr, pointer: bool) -> String {
    match expr {
        NumExpr::Add(..) | NumExpr::Diff(..) | NumExpr::Sum(..) => {
            format!("({})", num_css(expr, pointer))
        }
        _ => num_css(expr, pointer),
    }
}

fn is_px(expr: &NumExpr) -> bool {
    match expr {
        NumExpr::Token { px, .. } | NumExpr::Value { px, .. } => *px,
        NumExpr::Add(a, _)
        | NumExpr::Mul(a, _)
        | NumExpr::Div(a, _)
        | NumExpr::PointerTouch(a, _)
        | NumExpr::Max(a, _)
        | NumExpr::Min(a, _)
        | NumExpr::Diff(a, _)
        | NumExpr::Sum(a, _) => is_px(a),
    }
}

fn unit(value: f32, px: bool) -> String {
    if px {
        format!("{}px", number(value))
    } else {
        number(value)
    }
}

/// `value` with at most four decimals and no trailing zeros.
fn number(value: f32) -> String {
    let s = format!("{:.4}", value);
    let s = s.trim_end_matches('0').trim_end_matches('.');
    if s == "-0" { "0".into() } else { s.into() }
}

fn hex(c: Hsla) -> String {
    format!("#{:08x}", Color::from(c).0)
}

fn is_color(decl: &TokenDecl) -> bool {
    matches!(decl.default, DeclDefault::Color(_))
}

/// `themes/default.json`: every token the library declares with its
/// default, in the `cssVars` format. Numbers go in `theme`, colors in
/// `light` and `dark`. Derived lengths are written as `calc()`, which NSH
/// does not read (the library derives them); derived colors as `var()` and
/// `color-mix()`, which NSH reads.
pub fn default_theme_json() -> String {
    let decls = registry();
    let mut out = String::from("{\n  \"name\": \"nsh-longbridge default\",\n  \"cssVars\": {\n");
    let section = |out: &mut String, name: &str, entries: Vec<(&str, String)>, last: bool| {
        let _ = writeln!(out, "    \"{name}\": {{");
        let n = entries.len();
        for (i, (key, value)) in entries.into_iter().enumerate() {
            let comma = if i + 1 < n { "," } else { "" };
            let _ = writeln!(
                out,
                "      \"{key}\": \"{}\"{comma}",
                value.replace('"', "\\\"")
            );
        }
        let _ = writeln!(out, "    }}{}", if last { "" } else { "," });
    };
    let numbers = decls
        .iter()
        .filter(|d| !is_color(d))
        .filter_map(|d| default_css(d, false).map(|v| (d.name, v)))
        .collect();
    let colors = |dark: bool| {
        decls
            .iter()
            .filter(|d| is_color(d))
            .filter_map(|d| default_css(d, dark).map(|v| (d.name, v)))
            .collect()
    };
    section(&mut out, "theme", numbers, false);
    section(&mut out, "light", colors(false), false);
    section(&mut out, "dark", colors(true), true);
    out.push_str("  }\n}\n");
    out
}

/// A Markdown table of `decls`, the format of the specs in
/// `docs/components/`: name, kind, the derived default as an expression
/// over other tokens, and what that gives with `nsh/base`'s default theme
/// (a length or number; with touch's when it differs).
pub fn token_table(decls: &[TokenDecl]) -> String {
    let app = default_app_tokens();
    let mut out = String::from("| Token | Type | Derived default | Value |\n|---|---|---|---|\n");
    for decl in decls {
        let (kind, expr, value) = match decl.default {
            DeclDefault::Color(f) => (
                "color",
                f.map(|f| color_expr(&f(&Derive(())))),
                String::new(),
            ),
            DeclDefault::Length(f) | DeclDefault::Number(f) => {
                let kind = if matches!(decl.default, DeclDefault::Length(_)) {
                    "length"
                } else {
                    "number"
                };
                let value = f
                    .map(|f| {
                        let expr = f(&Derive(()));
                        let pointer = number(Cx::new(None, &app, true).eval_num(&expr));
                        let touch = number(Cx::new(None, &app, false).eval_num(&expr));
                        if pointer == touch {
                            pointer
                        } else {
                            format!("{pointer} (touch {touch})")
                        }
                    })
                    .unwrap_or_default();
                (kind, f.map(|f| num_expr(&f(&Derive(())), false)), value)
            }
        };
        let expr = expr.map_or_else(|| "the theme's".to_owned(), |e| format!("`{e}`"));
        let _ = writeln!(out, "| `{}` | {kind} | {expr} | {value} |", decl.name);
    }
    out
}

/// A color default in the specs' notation: `light_dark(a, b)`, `a × 90%`.
fn color_expr(expr: &ColorExpr) -> String {
    match expr {
        ColorExpr::Token(t) => t.name().to_owned(),
        ColorExpr::Css(css) => (*css).to_owned(),
        ColorExpr::Fixed(c) if c.light == c.dark => hex(c.light),
        ColorExpr::Fixed(c) => format!("light_dark({}, {})", hex(c.light), hex(c.dark)),
        ColorExpr::Opacity(c, alpha) => match **alpha {
            NumExpr::Value { value, .. } => {
                format!("{} × {}%", color_expr(c), number(value * 100.0))
            }
            _ => format!("{} × {}", color_expr(c), num_expr(alpha, true)),
        },
        ColorExpr::LightDark(l, d) => format!("light_dark({}, {})", color_expr(l), color_expr(d)),
    }
}

/// A number default in the specs' notation: `spacing × 2`,
/// `pointer_touch(a, b)` (`inner`: an operand, a sum in parentheses).
fn num_expr(expr: &NumExpr, inner: bool) -> String {
    match expr {
        NumExpr::Token { name, .. } => (*name).to_owned(),
        NumExpr::Value { value, .. } => number(*value),
        NumExpr::Add(a, b) => {
            let sign = if *b < 0.0 { '−' } else { '+' };
            let sum = format!("{} {sign} {}", num_expr(a, false), number(b.abs()));
            if inner { format!("({sum})") } else { sum }
        }
        // A negation: `−(a + b)`.
        NumExpr::Mul(a, k) if *k == -1.0 => format!("−{}", num_expr(a, true)),
        NumExpr::Mul(a, k) => format!("{} × {}", num_expr(a, true), number(*k)),
        NumExpr::Div(a, k) => format!("{} / {}", num_expr(a, true), number(*k)),
        NumExpr::Diff(a, b) => {
            let diff = format!("{} − {}", num_expr(a, false), num_expr(b, true));
            if inner { format!("({diff})") } else { diff }
        }
        NumExpr::Sum(a, b) => {
            let sum = format!("{} + {}", num_expr(a, false), num_expr(b, true));
            if inner { format!("({sum})") } else { sum }
        }
        NumExpr::PointerTouch(p, t) => {
            format!(
                "pointer_touch({}, {})",
                num_expr(p, false),
                num_expr(t, false)
            )
        }
        NumExpr::Max(a, b) => format!("max({}, {})", num_expr(a, false), num_expr(b, false)),
        NumExpr::Min(a, b) => format!("min({}, {})", num_expr(a, false), num_expr(b, false)),
    }
}
