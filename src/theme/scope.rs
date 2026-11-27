//! `ThemeScope`: other token values (and a forced appearance) for the
//! library's components in one subtree.

use std::{collections::HashMap, sync::Arc};

use nsh_core::{
    JsonValue,
    element::{JsHost, JsIn, JsOwned, PropError, TsTypes},
    prelude::*,
};

use super::{
    resolve::{Theme, parse_color},
    semantic::{BACKGROUND, FOREGROUND},
};

/// The appearance a `ThemeScope` forces on the library's components in it.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PropEnum)]
pub enum ThemeAppearance {
    /// The light values of every color.
    Light,
    /// The dark values of every color.
    Dark,
}

/// The token values of a `ThemeScope`, by name: a CSS color (`"#2563eb"`,
/// `"oklch(0.55 0.2 260)"`), a length (`"0px"`, `"0.5rem"`, `12`), a
/// number (`0.5`, `"150ms"`), or a color per appearance (`{ light, dark }`).
/// A value that is none of these is ignored and logged.
#[derive(Debug, Clone, Default, PartialEq)]
pub struct ScopeTokens(JsonValue);

impl ScopeTokens {
    /// No values.
    pub fn new() -> Self {
        Self(JsonValue::Object(Default::default()))
    }

    /// With `name` set to `value` (any CSS value a scope takes).
    pub fn set(mut self, name: &str, value: impl Into<JsonValue>) -> Self {
        if !self.0.is_object() {
            self.0 = JsonValue::Object(Default::default());
        }
        if let JsonValue::Object(map) = &mut self.0 {
            map.insert(name.to_owned(), value.into());
        }
        self
    }

    /// With `name` set to one color per appearance.
    pub fn set_light_dark(self, name: &str, light: &str, dark: &str) -> Self {
        let mut pair = serde_json_map();
        pair.insert("light".into(), light.into());
        pair.insert("dark".into(), dark.into());
        self.set(name, JsonValue::Object(pair))
    }
}

fn serde_json_map() -> serde_json::Map<String, JsonValue> {
    Default::default()
}

impl JsValue for ScopeTokens {
    fn ts_type(_types: &mut TsTypes) -> String {
        "Record<string, string | number | { light: string; dark: string }>".into()
    }

    fn from_js(value: JsIn<'_>) -> Result<Self, PropError> {
        JsonValue::from_js(value).map(Self)
    }

    fn to_js<'a>(&self, host: &'a dyn JsHost) -> Result<JsOwned<'a>, PropError> {
        self.0.to_js(host)
    }
}

/// The values a scope gives, its parents' under its own.
#[derive(Debug, Clone, Default, PartialEq)]
pub(crate) struct Scoped {
    pub(crate) colors: HashMap<Box<str>, AdaptiveColor>,
    pub(crate) numbers: HashMap<Box<str>, f32>,
    pub(crate) appearance: Option<ThemeAppearance>,
}

impl Scoped {
    /// Whether it changes no token (it may still force an appearance).
    pub(crate) fn is_empty(&self) -> bool {
        self.colors.is_empty() && self.numbers.is_empty()
    }

    /// `parent` with `tokens` and `appearance` over it.
    pub(crate) fn over(
        parent: Option<&Scoped>,
        tokens: &ScopeTokens,
        appearance: Option<ThemeAppearance>,
    ) -> Scoped {
        let mut out = parent.cloned().unwrap_or_default();
        if let JsonValue::Object(map) = &tokens.0 {
            for (name, value) in map {
                match parse_value(value) {
                    Some(Value::Color(c)) => {
                        out.numbers.remove(name.as_str());
                        out.colors.insert(name.as_str().into(), c);
                    }
                    Some(Value::Number(n)) => {
                        out.colors.remove(name.as_str());
                        out.numbers.insert(name.as_str().into(), n);
                    }
                    None => log::warn!(
                        "ThemeScope: `{name}`: {value} is no color, length or number, ignored"
                    ),
                }
            }
        } else if !tokens.0.is_null() {
            log::warn!("ThemeScope: `tokens` is no object, ignored");
        }
        if appearance.is_some() {
            out.appearance = appearance;
        }
        out
    }
}

enum Value {
    Color(AdaptiveColor),
    Number(f32),
}

/// A scope's value: a color, a number or length, or a color per appearance.
fn parse_value(value: &JsonValue) -> Option<Value> {
    match value {
        JsonValue::Number(n) => n.as_f64().map(|n| Value::Number(n as f32)),
        JsonValue::String(s) => parse_color(s)
            .map(|c| Value::Color(c.into()))
            .or_else(|| parse_number(s).map(Value::Number)),
        JsonValue::Object(pair) => {
            let light = parse_color(pair.get("light")?.as_str()?)?;
            let dark = parse_color(pair.get("dark")?.as_str()?)?;
            Some(Value::Color(AdaptiveColor::new(light, dark)))
        }
        _ => None,
    }
}

/// A CSS length or number: `12`, `12px`, `0.75rem` (16 points each),
/// `150ms`.
pub(crate) fn parse_number(s: &str) -> Option<f32> {
    let s = s.trim();
    let (n, scale) = if let Some(n) = s.strip_suffix("rem") {
        (n, 16.0)
    } else if let Some(n) = s.strip_suffix("px") {
        (n, 1.0)
    } else if let Some(n) = s.strip_suffix("ms") {
        (n, 1.0)
    } else {
        (s, 1.0)
    };
    n.trim()
        .parse::<f32>()
        .ok()
        .filter(|n| n.is_finite())
        .map(|n| n * scale)
}

/// What a `ThemeScope` provides to what is inside it.
#[derive(Debug, Clone, Copy)]
pub(crate) struct ScopeContext(pub(crate) Memo<Arc<Scoped>>);

/// The theme scope where it was captured, to provide again under another
/// parent: a toast stack drawn in a dialog's modal reads its `Toaster`'s
/// scope, not the dialog's.
#[derive(Debug, Clone, Copy)]
pub(crate) struct CapturedScope(Option<ScopeContext>);

/// The scope around the current component (none: the app's theme).
pub(crate) fn capture_scope() -> CapturedScope {
    CapturedScope(use_context::<ScopeContext>())
}

impl CapturedScope {
    /// Provides it to what the current component makes, over any scope
    /// around it.
    pub(crate) fn provide(self) {
        let scope = self.0.unwrap_or_else(|| {
            ScopeContext(computed(|| {
                Arc::new(Scoped::over(None, &ScopeTokens::new(), None))
            }))
        });
        provide_context(scope);
    }
}

/// Provides, to what the current component makes, the scope around it
/// with `colors` over it (token names and values, read in a binding) and
/// `appearance` forced when it gives one, and returns its theme: a part
/// that draws the library's components in other colors (a selection bar's
/// destructive action, a dark bar) without a `ThemeScope` view in the
/// layout.
pub(crate) fn provide_over(
    colors: impl Fn() -> Vec<(&'static str, AdaptiveColor)> + 'static,
    appearance: impl Fn() -> Option<ThemeAppearance> + 'static,
) -> Theme {
    let parent_scope = use_context::<ScopeContext>();
    let merged = computed(move || {
        let parent = parent_scope.map(|p| p.0.get());
        let mut out = Scoped::over(parent.as_deref(), &ScopeTokens::new(), appearance());
        for (name, color) in colors() {
            out.numbers.remove(name);
            out.colors.insert(name.into(), color);
        }
        Arc::new(out)
    });
    provide_context(ScopeContext(merged));
    Theme::of_scope(merged)
}

/// The props of [`ThemeScope`].
#[derive(Debug, Clone, PartialEq, Props)]
pub struct ThemeScopeProps {
    /// Token values for the library's components inside, by name (kebab-case,
    /// as in a theme file): `{ primary: "#2563eb", "button-radius": "0px" }`.
    /// Inner scopes win over outer ones. Tokens derived from the ones set
    /// follow them (`button-default` follows `primary`).
    #[prop(default)]
    pub tokens: Prop<ScopeTokens>,
    /// Draw the library's components inside in this appearance (`"light"`
    /// or `"dark"`); left out, they follow the app's.
    pub appearance: Prop<Option<ThemeAppearance>>,
    /// Paint the scope's `background` behind its content and give its text
    /// the scope's `foreground`: a block that shows another theme as a
    /// surface of its own.
    #[prop(default)]
    pub surface: Prop<bool>,
    /// The content.
    pub children: Prop<Children>,
}

/// Other token values, and optionally a forced light or dark appearance,
/// for the library's components inside it. Scopes nest: an inner one's
/// values win. What the library does not draw (the app's own Tailwind
/// classes, NSH's native controls) keeps the app's theme. The app's `class`
/// and `style` lay it out; it is a plain view.
#[component]
pub fn ThemeScope(props: ThemeScopeProps, _cx: Scope<NoEvent>) -> impl IntoElement {
    let parent_scope = use_context::<ScopeContext>();
    let merged = computed(move || {
        let parent = parent_scope.map(|p| p.0.get());
        Arc::new(Scoped::over(
            parent.as_deref(),
            &props.tokens.get(),
            props.appearance.get(),
        ))
    });
    provide_context(ScopeContext(merged));
    let t = Theme::of_scope(merged);
    view()
        .style_when(props.surface, move |s| {
            s.bg(move || t.color(BACKGROUND))
                .text_color(move || t.color(FOREGROUND))
        })
        .child(props.children)
}
