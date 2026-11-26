//! Token names, their kinds and their derived defaults: what `tokens!`
//! declares.
//!
//! A default is an expression over other tokens ([`ColorExpr`],
//! [`NumExpr`]), built by a closure from a [`Derive`] (`|t|
//! t.color(PRIMARY).opacity(0.9)`). Being data rather than code, the same
//! expression is resolved at run time (through the same order as any read:
//! scope, app, default) and written into `themes/default.json` as CSS
//! (`color-mix(in oklab, var(--primary) 90%, transparent)`).

use std::{
    fmt,
    marker::PhantomData,
    ops::{Add, Mul, Sub},
};

use nsh_core::element::AdaptiveColor;

/// The kinds a token can be.
pub mod kind {
    /// A color with a light and a dark value.
    #[derive(Debug, Clone, Copy)]
    pub struct Color;
    /// A length in points (`px` in a theme file).
    #[derive(Debug, Clone, Copy)]
    pub struct Length;
    /// A unitless number (a weight, an opacity, a level, milliseconds).
    #[derive(Debug, Clone, Copy)]
    pub struct Number;
}

/// A token kind and the expression its defaults are written in.
pub trait Kind: 'static {
    /// What a default of this kind is.
    type Expr;
}

impl Kind for kind::Color {
    type Expr = ColorExpr;
}

impl Kind for kind::Length {
    type Expr = NumExpr;
}

impl Kind for kind::Number {
    type Expr = NumExpr;
}

/// A derived default: builds the expression from a [`Derive`].
pub type DefaultFn<E> = fn(&Derive) -> E;

/// A token's name and its derived default (`None`: shadcn's own tokens,
/// whose defaults are `nsh/base`'s theme). Made by `tokens!`, read through
/// [`crate::theme::Theme`].
pub struct Token<K: Kind> {
    name: &'static str,
    default: Option<DefaultFn<K::Expr>>,
    kind: PhantomData<fn() -> K>,
}

impl<K: Kind> Clone for Token<K> {
    fn clone(&self) -> Self {
        *self
    }
}

impl<K: Kind> Copy for Token<K> {}

impl<K: Kind> fmt::Debug for Token<K> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Token({})", self.name)
    }
}

impl<K: Kind> PartialEq for Token<K> {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
    }
}

impl<K: Kind> Token<K> {
    /// A token named `name` (kebab-case, without `--`), derived by
    /// `default` when nothing sets it.
    pub const fn new(name: &'static str, default: Option<DefaultFn<K::Expr>>) -> Self {
        Self {
            name,
            default,
            kind: PhantomData,
        }
    }

    /// Its name, as a theme file and Tailwind classes write it.
    pub const fn name(self) -> &'static str {
        self.name
    }

    /// Its derived default.
    pub const fn default_fn(self) -> Option<DefaultFn<K::Expr>> {
        self.default
    }
}

impl Token<kind::Color> {
    /// Its registry entry.
    pub const fn decl(self, doc: &'static str) -> TokenDecl {
        TokenDecl {
            name: self.name,
            doc,
            default: DeclDefault::Color(self.default),
        }
    }
}

impl Token<kind::Length> {
    /// Its registry entry.
    pub const fn decl(self, doc: &'static str) -> TokenDecl {
        TokenDecl {
            name: self.name,
            doc,
            default: DeclDefault::Length(self.default),
        }
    }
}

impl Token<kind::Number> {
    /// Its registry entry.
    pub const fn decl(self, doc: &'static str) -> TokenDecl {
        TokenDecl {
            name: self.name,
            doc,
            default: DeclDefault::Number(self.default),
        }
    }
}

/// A token as the registry lists it: what `themes/default.json` and the
/// token tables in `docs/components/` are made of.
#[derive(Debug, Clone, Copy)]
pub struct TokenDecl {
    /// The token's name.
    pub name: &'static str,
    /// Its doc comment.
    pub doc: &'static str,
    /// Its kind and derived default.
    pub default: DeclDefault,
}

/// A registry entry's kind and derived default.
#[derive(Debug, Clone, Copy)]
pub enum DeclDefault {
    /// A color.
    Color(Option<DefaultFn<ColorExpr>>),
    /// A length in points.
    Length(Option<DefaultFn<NumExpr>>),
    /// A unitless number.
    Number(Option<DefaultFn<NumExpr>>),
}

/// What a default closure is given: reads of other tokens and literals,
/// as expressions.
#[derive(Debug)]
pub struct Derive(pub(crate) ());

impl Derive {
    /// The color token `token`, resolved where the default is.
    pub fn color(&self, token: Token<kind::Color>) -> ColorExpr {
        ColorExpr::Token(token)
    }

    /// The length token `token`.
    pub fn length(&self, token: Token<kind::Length>) -> NumExpr {
        NumExpr::Token {
            name: token.name,
            default: token.default,
            px: true,
        }
    }

    /// The number token `token`.
    pub fn number(&self, token: Token<kind::Number>) -> NumExpr {
        NumExpr::Token {
            name: token.name,
            default: token.default,
            px: false,
        }
    }

    /// A CSS color, the same in both appearances (`"oklch(1 0 0)"`).
    pub fn css(&self, color: &'static str) -> ColorExpr {
        ColorExpr::Css(color)
    }

    /// `light` in the light appearance, `dark` in the dark one.
    pub fn light_dark(&self, light: ColorExpr, dark: ColorExpr) -> ColorExpr {
        ColorExpr::LightDark(Box::new(light), Box::new(dark))
    }

    /// A length of `points`.
    pub fn px(&self, points: f32) -> NumExpr {
        NumExpr::Value {
            value: points,
            px: true,
        }
    }

    /// A unitless number.
    pub fn value(&self, value: f32) -> NumExpr {
        NumExpr::Value { value, px: false }
    }

    /// `pointer` where a pointer drives the UI (desktop and the desktop
    /// web), `touch` on touch screens.
    pub fn pointer_touch(&self, pointer: NumExpr, touch: NumExpr) -> NumExpr {
        NumExpr::PointerTouch(Box::new(pointer), Box::new(touch))
    }
}

/// A color default.
#[derive(Debug, Clone)]
pub enum ColorExpr {
    /// Another token.
    Token(Token<kind::Color>),
    /// A CSS color literal.
    Css(&'static str),
    /// A color already resolved.
    Fixed(AdaptiveColor),
    /// A color with its alpha multiplied.
    Opacity(Box<ColorExpr>, Box<NumExpr>),
    /// One color per appearance.
    LightDark(Box<ColorExpr>, Box<ColorExpr>),
}

impl ColorExpr {
    /// This color with its alpha multiplied by `alpha` (a number, or a
    /// number token's expression).
    pub fn opacity(self, alpha: impl Into<NumExpr>) -> ColorExpr {
        ColorExpr::Opacity(Box::new(self), Box::new(alpha.into()))
    }
}

/// A length or number default.
#[derive(Debug, Clone)]
pub enum NumExpr {
    /// Another token.
    Token {
        /// Its name.
        name: &'static str,
        /// Its default.
        default: Option<DefaultFn<NumExpr>>,
        /// A length (`px`) rather than a unitless number.
        px: bool,
    },
    /// A literal.
    Value {
        /// The number.
        value: f32,
        /// A length (`px`) rather than a unitless number.
        px: bool,
    },
    /// A sum.
    Add(Box<NumExpr>, f32),
    /// A product.
    Mul(Box<NumExpr>, f32),
    /// One value with a pointer, another on touch screens.
    PointerTouch(Box<NumExpr>, Box<NumExpr>),
}

impl From<f32> for NumExpr {
    fn from(value: f32) -> Self {
        NumExpr::Value { value, px: false }
    }
}

impl Add<f32> for NumExpr {
    type Output = NumExpr;

    fn add(self, rhs: f32) -> NumExpr {
        NumExpr::Add(Box::new(self), rhs)
    }
}

impl Sub<f32> for NumExpr {
    type Output = NumExpr;

    fn sub(self, rhs: f32) -> NumExpr {
        NumExpr::Add(Box::new(self), -rhs)
    }
}

impl Mul<f32> for NumExpr {
    type Output = NumExpr;

    fn mul(self, rhs: f32) -> NumExpr {
        NumExpr::Mul(Box::new(self), rhs)
    }
}

/// Declares tokens: a constant per name (typed by kind), its derived
/// default, and the module's part of the registry (`TOKENS`).
///
/// ```ignore
/// tokens! {
///     /// The default variant's fill.
///     BUTTON_DEFAULT: Color = "button-default" => |t| t.color(PRIMARY),
///     /// The default variant's fill under the pointer.
///     BUTTON_DEFAULT_HOVER: Color = "button-default-hover" => |t| t.color(BUTTON_DEFAULT).opacity(0.9),
///     BUTTON_RADIUS: Length = "button-radius" => |t| t.length(RADIUS_MD),
/// }
/// ```
///
/// One `tokens!` per module; list the module's `TOKENS` in
/// `theme::registry()`.
#[macro_export]
macro_rules! tokens {
    ($( $(#[doc = $doc:literal])* $name:ident : $kind:ident = $key:literal $(=> $default:expr)? ),* $(,)?) => {
        $(
            $(#[doc = $doc])*
            #[doc = concat!("\n\nToken `", $key, "`.")]
            pub const $name: $crate::theme::Token<$crate::theme::kind::$kind> =
                $crate::theme::Token::new($key, $crate::tokens!(@default $($default)?));
        )*
        /// The tokens this module declares, in order: its part of the
        /// registry.
        pub const TOKENS: &[$crate::theme::TokenDecl] = &[
            $( $name.decl(concat!($($doc, "\n",)*)) ),*
        ];
    };
    (@default) => { None };
    (@default $default:expr) => { Some($default) };
}

/// Transparent, as a default (`t.light_dark(TRANSPARENT, ..)`).
pub const TRANSPARENT: ColorExpr = ColorExpr::Css("transparent");
