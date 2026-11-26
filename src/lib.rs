//! nsh-longbridge: Longbridge's shared NSH component library. shadcn/ui's
//! look (`new-york` v4, `neutral` by default) on `nsh/base`'s behavior, one
//! Rust implementation for TSX apps (`import { Button } from
//! "nsh-longbridge"`) and Rust apps (`use nsh_longbridge::button`).
//!
//! - [`theme`]: tokens in three layers (seeds, semantic, per component),
//!   how a read resolves (a `ThemeScope`, the app's tokens, the derived
//!   default), and the registry `themes/default.json` is made of.
//! - [`foundation`]: `Size`, control heights and hit slop, text sizes and
//!   line heights, focus ring, press feedback, motion, shadows, icons, the
//!   field look, the library's own words.
//! - [`components`]: one file per component.
//!
//! Contracts: `docs/tokens.md`, `docs/conventions.md`,
//! `docs/components/<component>.md`.

pub mod components;
pub mod foundation;
pub mod theme;

use nsh_core::prelude::*;

pub use crate::{
    components::{
        button::{Button, ButtonElement, ButtonEvent, ButtonProps, ButtonVariant, button},
        checkbox::{Checkbox, CheckboxElement, CheckboxEvent, CheckboxProps, checkbox},
        input::{AutoComplete, Input, InputApi, InputElement, InputEvent, InputProps, input},
        label::{Label, LabelElement, LabelProps, label},
        radio_group::{
            RadioGroup, RadioGroupElement, RadioGroupEvent, RadioGroupItem, RadioGroupItemElement,
            RadioGroupItemProps, RadioGroupProps, radio_group, radio_group_item,
        },
        switch::{Switch, SwitchElement, SwitchEvent, SwitchProps, switch},
        textarea::{
            Textarea, TextareaApi, TextareaElement, TextareaEvent, TextareaProps, textarea,
        },
    },
    foundation::{
        FieldRef, Icon, IconElement, IconName, IconProps, Size, icon,
        labels::{self, set_translator},
    },
    theme::{
        ScopeTokens, Theme, ThemeAppearance, ThemeScope, ThemeScopeElement, ThemeScopeProps, theme,
        theme_scope,
    },
};

/// The library's UI set: what TSX imports from `nsh-longbridge`.
#[native_module("nsh-longbridge")]
fn native(registry: &mut Registry) {
    registry
        .register::<Button>()
        .register::<Checkbox>()
        .register::<Icon>()
        .register::<Input>()
        .register::<Label>()
        .register::<RadioGroup>()
        .register::<RadioGroupItem>()
        .register::<Switch>()
        .register::<Textarea>()
        .register::<ThemeScope>()
        .module::<foundation::labels::Labels>();
}
