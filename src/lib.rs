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
        alert_dialog::{
            AlertDialog, AlertDialogAction, AlertDialogActionElement, AlertDialogCancel,
            AlertDialogCancelElement, AlertDialogContent, AlertDialogContentElement,
            AlertDialogContentProps, AlertDialogDescription, AlertDialogDescriptionElement,
            AlertDialogDescriptionProps, AlertDialogElement, AlertDialogEvent, AlertDialogFooter,
            AlertDialogFooterElement, AlertDialogFooterProps, AlertDialogHeader,
            AlertDialogHeaderElement, AlertDialogHeaderProps, AlertDialogMedia,
            AlertDialogMediaElement, AlertDialogMediaProps, AlertDialogProps, AlertDialogTitle,
            AlertDialogTitleElement, AlertDialogTitleProps, alert_dialog, alert_dialog_action,
            alert_dialog_cancel, alert_dialog_content, alert_dialog_description,
            alert_dialog_footer, alert_dialog_header, alert_dialog_media, alert_dialog_title,
        },
        badge::{Badge, BadgeElement, BadgeProps, BadgeVariant, badge},
        button::{Button, ButtonElement, ButtonEvent, ButtonProps, ButtonVariant, button},
        card::{
            Card, CardAction, CardActionElement, CardActionProps, CardContent, CardContentElement,
            CardContentProps, CardDescription, CardDescriptionElement, CardDescriptionProps,
            CardElement, CardFooter, CardFooterElement, CardFooterProps, CardHeader,
            CardHeaderElement, CardHeaderProps, CardProps, CardTitle, CardTitleElement,
            CardTitleProps, card, card_action, card_content, card_description, card_footer,
            card_header, card_title,
        },
        checkbox::{Checkbox, CheckboxElement, CheckboxEvent, CheckboxProps, checkbox},
        dialog::{
            Dialog, DialogClose, DialogCloseElement, DialogCloseProps, DialogContent,
            DialogContentElement, DialogContentProps, DialogDescription, DialogDescriptionElement,
            DialogDescriptionProps, DialogElement, DialogEvent, DialogFooter, DialogFooterElement,
            DialogFooterProps, DialogHeader, DialogHeaderElement, DialogHeaderProps, DialogProps,
            DialogTitle, DialogTitleElement, DialogTitleProps, dialog, dialog_close,
            dialog_content, dialog_description, dialog_footer, dialog_header, dialog_title,
        },
        dropdown_menu::{
            DropdownMenu, DropdownMenuCheckboxItem, DropdownMenuCheckboxItemElement,
            DropdownMenuCheckboxItemEvent, DropdownMenuCheckboxItemProps, DropdownMenuElement,
            DropdownMenuEvent, DropdownMenuGroup, DropdownMenuGroupElement, DropdownMenuGroupProps,
            DropdownMenuItem, DropdownMenuItemElement, DropdownMenuItemEvent,
            DropdownMenuItemProps, DropdownMenuLabel, DropdownMenuLabelElement,
            DropdownMenuLabelProps, DropdownMenuProps, DropdownMenuRadioGroup,
            DropdownMenuRadioGroupElement, DropdownMenuRadioGroupEvent,
            DropdownMenuRadioGroupProps, DropdownMenuRadioItem, DropdownMenuRadioItemElement,
            DropdownMenuRadioItemProps, DropdownMenuSeparator, DropdownMenuSeparatorElement,
            DropdownMenuSeparatorProps, DropdownMenuShortcut, DropdownMenuShortcutElement,
            DropdownMenuShortcutProps, DropdownMenuSub, DropdownMenuSubElement,
            DropdownMenuSubEvent, DropdownMenuSubProps, DropdownMenuSubTrigger,
            DropdownMenuSubTriggerElement, DropdownMenuSubTriggerProps, MenuItemVariant,
            dropdown_menu, dropdown_menu_checkbox_item, dropdown_menu_group, dropdown_menu_item,
            dropdown_menu_label, dropdown_menu_radio_group, dropdown_menu_radio_item,
            dropdown_menu_separator, dropdown_menu_shortcut, dropdown_menu_sub,
            dropdown_menu_sub_trigger,
        },
        input::{AutoComplete, Input, InputApi, InputElement, InputEvent, InputProps, input},
        kbd::{
            Kbd, KbdElement, KbdGroup, KbdGroupElement, KbdGroupProps, KbdProps, kbd, kbd_group,
        },
        label::{Label, LabelElement, LabelProps, label},
        popover::{
            Popover, PopoverDescription, PopoverDescriptionElement, PopoverDescriptionProps,
            PopoverElement, PopoverEvent, PopoverHeader, PopoverHeaderElement, PopoverHeaderProps,
            PopoverProps, PopoverTitle, PopoverTitleElement, PopoverTitleProps, popover,
            popover_description, popover_header, popover_title,
        },
        radio_group::{
            RadioGroup, RadioGroupElement, RadioGroupEvent, RadioGroupItem, RadioGroupItemElement,
            RadioGroupItemProps, RadioGroupProps, radio_group, radio_group_item,
        },
        select::{
            Select, SelectElement, SelectEntry, SelectEvent, SelectItem, SelectProps, SelectValue,
            select,
        },
        separator::{Separator, SeparatorElement, SeparatorProps, separator},
        skeleton::{Skeleton, SkeletonElement, SkeletonProps, skeleton},
        spinner::{Spinner, SpinnerElement, SpinnerProps, spinner},
        switch::{Switch, SwitchElement, SwitchEvent, SwitchProps, switch},
        tabs::{
            Tabs, TabsContent, TabsContentElement, TabsContentProps, TabsElement, TabsEvent,
            TabsList, TabsListElement, TabsListProps, TabsProps, TabsTrigger, TabsTriggerElement,
            TabsTriggerProps, TabsVariant, tabs, tabs_content, tabs_list, tabs_trigger,
        },
        textarea::{
            Textarea, TextareaApi, TextareaElement, TextareaEvent, TextareaProps, textarea,
        },
        toast::{
            ToastAnswer, ToastAnswerFuture, ToastBuilder, ToastButton, ToastKind, ToastOptions,
            ToastPlacement, ToastResult, Toaster, ToasterElement, ToasterProps, dismiss_toast,
            toast, toaster,
        },
        tooltip::{Tooltip, TooltipElement, TooltipEvent, TooltipProps, tooltip},
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
        .register::<AlertDialog>()
        .register::<AlertDialogAction>()
        .register::<AlertDialogCancel>()
        .register::<AlertDialogContent>()
        .register::<AlertDialogDescription>()
        .register::<AlertDialogFooter>()
        .register::<AlertDialogHeader>()
        .register::<AlertDialogMedia>()
        .register::<AlertDialogTitle>()
        .register::<Badge>()
        .register::<Button>()
        .register::<Card>()
        .register::<CardAction>()
        .register::<CardContent>()
        .register::<CardDescription>()
        .register::<CardFooter>()
        .register::<CardHeader>()
        .register::<CardTitle>()
        .register::<Checkbox>()
        .register::<Dialog>()
        .register::<DialogClose>()
        .register::<DialogContent>()
        .register::<DialogDescription>()
        .register::<DialogFooter>()
        .register::<DialogHeader>()
        .register::<DialogTitle>()
        .register::<DropdownMenu>()
        .register::<DropdownMenuCheckboxItem>()
        .register::<DropdownMenuGroup>()
        .register::<DropdownMenuItem>()
        .register::<DropdownMenuLabel>()
        .register::<DropdownMenuRadioGroup>()
        .register::<DropdownMenuRadioItem>()
        .register::<DropdownMenuSeparator>()
        .register::<DropdownMenuShortcut>()
        .register::<DropdownMenuSub>()
        .register::<DropdownMenuSubTrigger>()
        .register::<Icon>()
        .register::<Input>()
        .register::<Kbd>()
        .register::<KbdGroup>()
        .register::<Label>()
        .register::<Popover>()
        .register::<PopoverDescription>()
        .register::<PopoverHeader>()
        .register::<PopoverTitle>()
        .register::<RadioGroup>()
        .register::<RadioGroupItem>()
        .register::<Select>()
        .register::<Separator>()
        .register::<Skeleton>()
        .register::<Spinner>()
        .register::<Switch>()
        .register::<Tabs>()
        .register::<TabsContent>()
        .register::<TabsList>()
        .register::<TabsTrigger>()
        .register::<Textarea>()
        .register::<ThemeScope>()
        .register::<Tooltip>()
        .register::<Toaster>()
        .module::<foundation::labels::Labels>()
        .module::<components::toast::Toasts>();
}
