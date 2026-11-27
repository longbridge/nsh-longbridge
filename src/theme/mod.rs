//! Tokens: their kinds and declarations (`tokens!`), the seed and semantic
//! ones, how a read resolves (scope → app → derived default), `ThemeScope`
//! and the registry `themes/default.json` is made of. Contract:
//! `docs/tokens.md`.

mod file;
mod resolve;
mod scope;
pub mod seeds;
pub mod semantic;
mod token;

pub use file::{default_theme_json, registry, token_table};
pub use resolve::{Theme, resolve_color_in, resolve_number_in, theme};
pub(crate) use scope::{CapturedScope, capture_scope, provide_over};
pub use scope::{
    ScopeTokens, ThemeAppearance, ThemeScope, ThemeScopeElement, ThemeScopeProps, theme_scope,
};
pub use token::{
    ColorExpr, DeclDefault, DefaultFn, Derive, Kind, NumExpr, TRANSPARENT, Token, TokenDecl, kind,
};
