# nsh-longbridge

A component library for [NSH (Native Shell)](https://github.com/longbridge/nsh) apps: the look of
[shadcn/ui](https://ui.shadcn.com), the behavior of `nsh/base`, one Rust implementation for TSX and Rust apps alike,
and a look apps customize tier by tier, from a few seed tokens down to a single part of a single component.

- **One package for both languages.** The library is a Rust nshm: the crate `nsh-longbridge` with
  `#[native_module("nsh-longbridge")]` at its root.
  - TSX apps add it to `nsh.json`'s `"dependencies"` and `import { Button } from "nsh-longbridge"`, with generated
    TypeScript declarations.
  - Rust apps depend on the crate and `use nsh_longbridge::{button, ..}`.
- **shadcn/ui's `neutral` look by default.** A product's or a brand's look is a theme and tokens, never a fork of the
  components.
- **Platforms.** The web and the desktops come first (reviewed on macOS; Windows and Linux built and spot-checked).
  Sizes have pointer and touch values from the start; phones are reviewed later. In a terminal, NSH's own terminal
  look applies.

## Layers

```
nsh-core / nsh-base        Behavior, state, focus, keys, accessibility (NSH's)
nsh-longbridge
  src/theme/               Token resolution: seed → semantic → component tokens, scopes, forced appearance
  src/foundation/          Size, radii, control heights, type, icons, focus rings, press feedback, motion, shadows
  src/components/          One component per file, drawn with theme and foundation only
themes/                    Theme files (shadcn's cssVars), which apps reference or copy in nsh.json's "theme"
gallery/                   The gallery (a TSX app, web and macOS): documentation and visual review
docs/                      tokens.md, conventions.md, components/<component>.md
```

Later, `src/blocks/` will hold compositions that are not tied to a domain. Domain components (quotes, trading)
belong in other packages.

## Customization, from coarse to fine

| Tier | How | Reaches |
|---|---|---|
| 1. Seed tokens | `radius`, `spacing`, `control-height`, `font-size`, `duration` in the theme | Every component, in proportion |
| 2. Semantic tokens | `primary`, `border`, `radius-md`, `control-height-sm`, … in the theme | Every component using that role |
| 3. Component tokens | `button-radius`, `button-default-hover`, `input-border`, … in the theme | That kind of component |
| 4. Scopes | `<ThemeScope tokens={{…}} appearance="dark">` around part of a screen | This library's components in that subtree |
| 5. Instances | `variant` and `size`; `class` / `style` on the root; `<part>Class` / `<part>Style` on each part | One component |
| 6. At run time | `setTheme(json)` in TSX, `set_tokens(..)` in Rust | The whole app, at once |

A token left unset is derived from the tier below (`button-default` is `primary`, `button-default-hover` is
`button-default` at 90%), so changing one color changes its hover, press and focus states consistently. The rules
are in [docs/tokens.md](docs/tokens.md).

## Roadmap

- **P0, the foundation**: the repository, token resolution and scopes, `foundation`, `Button` as the reference
  component, the gallery (light and dark, theme presets, live token editing), headless tests, the color-literal
  check, documentation.
- **P1, basic components**: Badge, Label, Input, Textarea, Checkbox, Switch, RadioGroup, Card, Separator, Tooltip,
  Kbd, Spinner, Skeleton.
- **P2, overlays and navigation**: Dialog, AlertDialog, Popover, DropdownMenu, Select, Tabs, Toast.
- **P3, data and forms**: Table, List, Form / Field, Combobox, DatePicker and more.

Each component goes through a spec (`docs/components/<component>.md`), its implementation, a gallery page and tests,
and a review of web and macOS screenshots in light and dark. See [docs/conventions.md](docs/conventions.md).

## Component index

Find a component by what it is for. A change that adds or removes a public component updates this table.

| Component | For (search terms) | Not for |
|---|---|---|
| `Button` | Buttons: primary, secondary, outline, ghost, destructive, link-styled, icon-only, loading | Opening a URL: use a link, not a `variant="link"` button |
