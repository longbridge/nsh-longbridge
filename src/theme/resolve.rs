//! Reading tokens: the [`Theme`] handle and the resolution order.
//!
//! A read walks, first value wins:
//!
//! 1. the nearest `ThemeScope`'s values (each scope already holds its
//!    parents' under its own);
//! 2. the app's tokens (the theme file, `setTheme`, `set_tokens`, an
//!    embedding host's);
//! 3. the token's derived default, resolved through this same order, so a
//!    scope's `primary` reaches `button-default`.
//!
//! A scope that forces an appearance gives every color both values of that
//! appearance.

use std::{cell::Cell, sync::Arc};

use nsh_core::{
    Color,
    color::Hsla,
    element::{AdaptiveColor, Memo, Tokens, is_desktop, tokens, use_context},
    geometry::{Points, px},
};

use super::{
    scope::{ScopeContext, Scoped, ThemeAppearance},
    token::{ColorExpr, DefaultFn, NumExpr, Token, kind},
};

/// The theme where a component is: its scope, captured when the handle is
/// made. Get it in a component's body with [`theme()`] and read tokens in
/// bindings, so a change of the app's tokens or of the scope's runs only
/// them again:
///
/// ```ignore
/// let t = theme();
/// view().bg(move || t.color(BUTTON_DEFAULT)).rounded(move || t.length(BUTTON_RADIUS))
/// ```
#[derive(Debug, Clone, Copy, Default)]
pub struct Theme {
    scope: Option<Memo<Arc<Scoped>>>,
}

/// The theme of the current scope (call it in a component's body: the
/// nearest `ThemeScope` above is found through context).
pub fn theme() -> Theme {
    Theme {
        scope: use_context::<ScopeContext>().map(|c| c.0),
    }
}

impl Theme {
    /// The theme of `scope` (`ThemeScope`'s own surface).
    pub(crate) fn of_scope(scope: Memo<Arc<Scoped>>) -> Self {
        Self { scope: Some(scope) }
    }

    /// The color `token` (subscribes to what it read).
    pub fn color(&self, token: Token<kind::Color>) -> AdaptiveColor {
        let scoped = self.scope.map(|s| s.get());
        let app = tokens();
        let cx = Cx::new(scoped.as_deref(), &app, is_desktop());
        let color = cx.color(token.name(), token.default_fn());
        force(color, scoped.as_ref().and_then(|s| s.appearance))
    }

    /// The length `token`, in points.
    pub fn length(&self, token: Token<kind::Length>) -> Points {
        px(self.num(token.name(), token.default_fn()))
    }

    /// The number `token`.
    pub fn number(&self, token: Token<kind::Number>) -> f32 {
        self.num(token.name(), token.default_fn())
    }

    /// The appearance the scope forces, if any.
    pub fn forced_appearance(&self) -> Option<ThemeAppearance> {
        self.scope.and_then(|s| s.get().appearance)
    }

    /// A color expression (a foundation rule over tokens) resolved here.
    pub fn eval_color(&self, expr: &ColorExpr) -> AdaptiveColor {
        let scoped = self.scope.map(|s| s.get());
        let app = tokens();
        let cx = Cx::new(scoped.as_deref(), &app, is_desktop());
        force(
            cx.eval_color(expr),
            scoped.as_ref().and_then(|s| s.appearance),
        )
    }

    fn num(&self, name: &'static str, default: Option<DefaultFn<NumExpr>>) -> f32 {
        let scoped = self.scope.map(|s| s.get());
        let app = tokens();
        Cx::new(scoped.as_deref(), &app, is_desktop()).number(name, default)
    }
}

/// `color` with both values of the forced appearance.
fn force(color: AdaptiveColor, appearance: Option<ThemeAppearance>) -> AdaptiveColor {
    match appearance {
        Some(ThemeAppearance::Dark) => AdaptiveColor::new(color.dark, color.dark),
        Some(ThemeAppearance::Light) => AdaptiveColor::new(color.light, color.light),
        None => color,
    }
}

/// How deep defaults may refer to each other before a cycle is assumed.
const MAX_DEPTH: u32 = 32;

/// One resolution: the scope's values, the app's tokens and whether a
/// pointer drives the UI. Plain data, so the theme file's generator and
/// tests resolve without an engine.
pub(crate) struct Cx<'a> {
    scoped: Option<&'a Scoped>,
    app: &'a Tokens,
    pointer: bool,
    depth: Cell<u32>,
}

impl<'a> Cx<'a> {
    pub(crate) fn new(scoped: Option<&'a Scoped>, app: &'a Tokens, pointer: bool) -> Self {
        Self {
            scoped,
            app,
            pointer,
            depth: Cell::new(0),
        }
    }

    fn unscoped(&self) -> Cx<'a> {
        Cx::new(None, self.app, self.pointer)
    }

    fn enter(&self, name: &str) -> bool {
        let depth = self.depth.get() + 1;
        self.depth.set(depth);
        if depth > MAX_DEPTH {
            log::warn!("nsh-longbridge: token `{name}` refers to itself through its defaults");
            false
        } else {
            true
        }
    }

    fn leave(&self) {
        self.depth.set(self.depth.get() - 1);
    }

    pub(crate) fn color(
        &self,
        name: &'static str,
        default: Option<DefaultFn<ColorExpr>>,
    ) -> AdaptiveColor {
        if let Some(c) = self.scoped.and_then(|s| s.colors.get(name)) {
            return *c;
        }
        if !self.enter(name) {
            return AdaptiveColor::TRANSPARENT;
        }
        let derived = |cx: &Cx<'_>| default.map(|d| cx.eval_color(&d(&super::token::Derive(()))));
        let out = match self.app.color(name) {
            // The app's value is what the default gives without the scope
            // (a theme file that copied `themes/default.json`, whose
            // `var()`s were resolved when it was read): it is no override,
            // so the default follows the scope.
            Some(app)
                if self.scoped.is_some_and(|s| !s.is_empty())
                    && derived(&self.unscoped()).is_some_and(|d| same(d, app)) =>
            {
                derived(self).unwrap_or(app)
            }
            Some(app) => app,
            None => derived(self)
                .or_else(|| base_color(name))
                .unwrap_or_else(|| {
                    log::warn!("nsh-longbridge: no color token `{name}`");
                    AdaptiveColor::TRANSPARENT
                }),
        };
        self.leave();
        out
    }

    pub(crate) fn number(&self, name: &'static str, default: Option<DefaultFn<NumExpr>>) -> f32 {
        // On touch screens `<name>-touch`, where the scope or the app sets
        // it, wins over `<name>`.
        if !self.pointer {
            let touch = format!("{name}-touch");
            if let Some(n) = self
                .scoped
                .and_then(|s| s.numbers.get(touch.as_str()))
                .copied()
                .or_else(|| self.app.number(&touch))
            {
                return n;
            }
        }
        if let Some(n) = self.scoped.and_then(|s| s.numbers.get(name)) {
            return *n;
        }
        if let Some(n) = self.app.number(name) {
            return n;
        }
        if !self.enter(name) {
            return 0.0;
        }
        let out = match default {
            Some(d) => self.eval_num(&d(&super::token::Derive(()))),
            None => nsh_base::Theme::default().number(name).unwrap_or_else(|| {
                log::warn!("nsh-longbridge: no number token `{name}`");
                0.0
            }),
        };
        self.leave();
        out
    }

    pub(crate) fn eval_color(&self, expr: &ColorExpr) -> AdaptiveColor {
        match expr {
            ColorExpr::Token(t) => self.color(t.name(), t.default_fn()),
            ColorExpr::Css(css) => parse_color(css)
                .map(AdaptiveColor::from)
                .unwrap_or_else(|| {
                    log::warn!("nsh-longbridge: `{css}` is no color");
                    AdaptiveColor::TRANSPARENT
                }),
            ColorExpr::Fixed(c) => *c,
            ColorExpr::Opacity(c, a) => self.eval_color(c).opacity(self.eval_num(a)),
            ColorExpr::LightDark(l, d) => {
                AdaptiveColor::new(self.eval_color(l).light, self.eval_color(d).dark)
            }
        }
    }

    pub(crate) fn eval_num(&self, expr: &NumExpr) -> f32 {
        match expr {
            NumExpr::Token { name, default, .. } => self.number(name, *default),
            NumExpr::Value { value, .. } => *value,
            NumExpr::Add(a, b) => self.eval_num(a) + b,
            NumExpr::Mul(a, b) => self.eval_num(a) * b,
            NumExpr::Div(a, b) => self.eval_num(a) / b,
            NumExpr::Diff(a, b) => self.eval_num(a) - self.eval_num(b),
            NumExpr::Sum(a, b) => self.eval_num(a) + self.eval_num(b),
            NumExpr::PointerTouch(p, t) => {
                if self.pointer {
                    self.eval_num(p)
                } else {
                    self.eval_num(t)
                }
            }
            NumExpr::Max(a, b) => self.eval_num(a).max(self.eval_num(b)),
            NumExpr::Min(a, b) => self.eval_num(a).min(self.eval_num(b)),
        }
    }
}

/// shadcn's color `name` in `nsh/base`'s default theme.
fn base_color(name: &str) -> Option<AdaptiveColor> {
    nsh_base::Theme::default().color(name)
}

/// A CSS color.
pub(crate) fn parse_color(css: &str) -> Option<Hsla> {
    Color::parse(css).map(Hsla::from)
}

/// Equal as 8-bit RGBA in both appearances (what a theme file round trip
/// keeps).
pub(crate) fn same(a: AdaptiveColor, b: AdaptiveColor) -> bool {
    let q = |c: Hsla| Color::from(c).0;
    q(a.light) == q(b.light) && q(a.dark) == q(b.dark)
}

/// Resolution without an engine, for tests and tools: the color `token`
/// with the app's tokens `app`, under `scopes` (outermost first, each its
/// tokens and forced appearance), with a pointer or on touch.
#[doc(hidden)]
pub fn resolve_color_in(
    app: &Tokens,
    scopes: &[(super::ScopeTokens, Option<ThemeAppearance>)],
    pointer: bool,
    token: Token<kind::Color>,
) -> AdaptiveColor {
    let scoped = scoped_chain(scopes);
    let cx = Cx::new(scoped.as_ref(), app, pointer);
    force(
        cx.color(token.name(), token.default_fn()),
        scoped.and_then(|s| s.appearance),
    )
}

/// [`resolve_color_in`] for a length or number token (by name and default).
#[doc(hidden)]
pub fn resolve_number_in<K: super::Kind<Expr = NumExpr>>(
    app: &Tokens,
    scopes: &[(super::ScopeTokens, Option<ThemeAppearance>)],
    pointer: bool,
    token: Token<K>,
) -> f32 {
    let scoped = scoped_chain(scopes);
    Cx::new(scoped.as_ref(), app, pointer).number(token.name(), token.default_fn())
}

fn scoped_chain(scopes: &[(super::ScopeTokens, Option<ThemeAppearance>)]) -> Option<Scoped> {
    scopes
        .iter()
        .fold(None, |parent: Option<Scoped>, (tokens, appearance)| {
            Some(Scoped::over(parent.as_ref(), tokens, *appearance))
        })
}
