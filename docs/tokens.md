# Tokens

Tokens are the one source of the library's look. Components hold no color literals and no scattered sizes: every
color and every size someone may want to tune is a token. Fixed geometric relationships (the gap between an icon and
its label growing with `size`) are functions in `foundation`, and those functions read tokens too.

## What NSH provides (the constraints)

- An NSH token is a color (a light and a dark value), a number (points, or unitless) or a font family. Shadows,
  gradients and easing curves cannot be tokens.
- A theme file is shadcn's `cssVars` (`theme`, `light` and `dark` sections). `nsh.json`'s `"theme"` names it, over
  `nsh-base`'s default (shadcn's `neutral`). A name the theme adds is one of the app's own tokens.
- A surface has one set of tokens, and reading one subscribes: after `setTheme` / `set_tokens`, the bindings that read
  it run again. The core has no per-subtree tokens; this library scopes them itself ("Scopes" below).
- Tailwind classes and TSX's `token()` see only the tokens a theme file has; this library's derived defaults apply in
  its own components only. An app that wants `bg-button-default` in its own classes writes that token into its theme
  file (copied from `themes/default.json`).

## Three tiers

Names follow shadcn's and Tailwind 4's variables: kebab-case, without `--`.

### 1. Seeds: a few knobs that move everything

| Token | Default | Derives |
|---|---|---|
| `radius` | 10 (0.625rem, shadcn's) | `radius-sm/md/lg/xl` |
| `spacing` | 4 | Paddings and gaps (Tailwind 4's `--spacing`) |
| `font-size` | 14 | The controls' `font-size-xs/sm/md/lg` and `icon-size-*` |
| `control-height` | 32 | `control-height-xs/sm/md/lg` with a pointer |
| `control-height-touch` | 40 | The same on a touch screen |
| `duration` | 150 (ms) | `duration-fast/slow` |
| `font-sans` / `font-mono` | The system's | Text |

`radius: 0` squares the whole library; `control-height: 28` makes it compact.

### 2. Semantic: shadcn's names

- **Colors**: shadcn's full set (`background`, `foreground`, `card`, `popover`, `primary`, `secondary`, `muted`,
  `accent`, each with its `-foreground`, `destructive`, `border`, `input`, `ring`, `chart-1..5`, `sidebar-*`), and
  this library's additions: `destructive-foreground`, `success` / `warning` / `info` with their `-foreground`,
  `overlay` (behind a modal) and `shadow` (the color of drop shadows).
- **Radii**: `radius-sm` = radius − 4, `radius-md` = radius − 2, `radius-lg` = radius, `radius-xl` = radius + 4
  (shadcn v4's).
- **Control heights**: `control-height-xs/sm/md/lg` = control-height − 8 / − 4 / ± 0 / + 4 (24 / 28 / 32 / 36);
  `control-height-touch-xs/sm/md/lg` = control-height-touch − 8 / − 4 / ± 0 / + 8 (32 / 36 / 40 / 48).
  `touch-target` (44) is the smallest target on a touch screen: a smaller control gets a hit slop, not a larger look.
- **Type**: controls use `font-size-xs/sm/md/lg` = font-size − 2 / − 1 / ± 0 / ± 0 (12 / 13 / 14 / 14, by the
  control's `size`). Body text and headings keep Tailwind's names and values (`text-xs` 12, `text-sm` 14, `text-base`
  16, `text-lg` 18), so they agree with NSH's Tailwind classes. `font-weight-medium` 500, `font-weight-semibold` 600.
  Line heights are no tokens: `foundation`'s `line_height(size)` pairs them as Tailwind does (12→16, 13→18, 14→20,
  16→24, 18→28).
- **Icons**: an icon in a control is paired with its type, `icon-size-xs/sm/md/lg` = font-size − 2 / ± 0 / + 2 / + 2
  (12 / 14 / 16 / 16, by the control's `size`; shadcn's buttons use `size-4` at every size but `xs`). An icon without
  a size takes its control's; its color is its text's.
- **Lines and focus**: `border-width` 1, `ring-width` 3, `ring-opacity` 0.5 (shadcn's `focus-visible:ring-ring/50
  ring-[3px]`).
- **Invalid**: `ring-invalid` = `destructive` at 20% (40% in dark): the ring of a focused invalid field.
- **States**: `opacity-disabled` 0.5.
- **Motion**: `duration-fast` = duration × 2/3, `duration-slow` = duration × 5/3. Easing curves are fixed in
  `foundation`; with reduced motion a press does not scale and transitions take no time.
- **Shadows**: the geometry of each tier is fixed in `foundation` (`xs`, `sm`, `md`, `lg`, as Tailwind's), colored
  with the `shadow` token. A component picks a tier with its number token `<component>-shadow`: 0 none, 1 `xs`,
  2 `sm`, 3 `md`, 4 `lg`.

### 3. Component tokens: one kind of component

Names: `<component>[-<variant>][-<part>]-<property>[-<state>]`. Colors follow shadcn: a fill is the bare name, the
text on it `-foreground`, an edge `-border`. The only state suffixes are `-hover`, `-active`, `-focus`, `-invalid` and
`-disabled`.

- A control's form (`checked`, `indeterminate`, `open`) goes where a variant goes, not in a state suffix:
  `checkbox-checked-border`, `switch-checked-thumb`.
- The control itself (a box, a track, a ring) has the bare name; a secondary part adds its name (`switch-thumb`,
  `radio-group-item-dot`).
- Where a bare name is a semantic token: if it means the same, the component uses the semantic token and declares
  none (`card`, `card-foreground`; declaring it would refer to itself); if not, it adds `-background` (`input` is
  the semantic edge color of fields, so a field's fill is `input-background`).
- Focus tokens are `<component>-border-focus` (the edge while focused), `<component>-ring` (the ring) and
  `<component>-ring-invalid` (`ring-invalid` by default).
- Kinds: colors; lengths (type sizes, radii, sizes, gaps, in points); numbers (milliseconds, tiers, opacities,
  scales).

For example:

```
button-radius              = radius-md
button-height-sm           = control-height-sm (control-height-touch-sm on a touch screen)
button-default             = primary
button-default-foreground  = primary-foreground
button-default-hover       = button-default at 90%
button-outline-border      = border (input in dark)
input-border               = input
card-radius                = radius-xl
card-shadow                = 2 (sm)
```

Each component's tokens are listed in its spec (`docs/components/<component>.md`), one row per declaration in its
code; tests keep the two the same.

## Resolution

A component reading a token takes the first of these that has a value:

1. the nearest `ThemeScope`'s;
2. the app's (its theme file, `setTheme`, `set_tokens`, an embedding host's);
3. the default this library derives (from semantic and seed tokens, **resolved the same way**: a scope that changes
   `primary` changes `button-default` too).

Derived defaults are declared in Rust with their names, in the component's file:

```rust
tokens! {
    /// The default variant's fill.
    BUTTON_DEFAULT: Color = "button-default" => |t| t.color(PRIMARY),
    /// The default variant's fill under the pointer.
    BUTTON_DEFAULT_HOVER: Color = "button-default-hover" => |t| t.color(BUTTON_DEFAULT).opacity(0.9),
    BUTTON_RADIUS: Length = "button-radius" => |t| t.length(RADIUS_MD),
}
```

A default may differ between appearances and between pointer and touch; that is a general rule, not an exception:

```rust
INPUT_BACKGROUND: Color = "input-background" => |t| t.light_dark(TRANSPARENT, t.color(INPUT).opacity(0.3)),
BUTTON_HEIGHT_SM: Length = "button-height-sm" => |t| t.pointer_touch(t.length(CONTROL_HEIGHT_SM), t.length(CONTROL_HEIGHT_TOUCH_SM)),
```

A value an app's theme file sets has its light and dark values from the file's `light` and `dark` sections. To set a
different value for touch screens, the app also sets `<name>-touch`, which a touch screen uses when it has one.

`tokens!` makes the name constants, the defaults and a registry. The registry generates `themes/default.json` (every
token, with `var()` references, for apps to copy) and the token tables of the docs; a test keeps the three the same.

A component takes the handle in its body and reads values in its bindings:

```rust
let t = nsh_longbridge::theme();           // captures the current scope
view().bg(move || t.color(BUTTON_DEFAULT)).rounded(move || t.length(BUTTON_RADIUS))
```

## Scopes

```tsx
<ThemeScope tokens={{ primary: "#2563eb", "button-radius": "0px" }} appearance="dark">
  {/* this library's components in here use these values, in dark */}
</ThemeScope>
```

- `tokens`: CSS values (colors, `px` / `rem`, numbers); one that does not parse is ignored with a warning. Scopes
  nest, the inner one winning.
- `appearance`: `"light"`, `"dark"`, or unset (the app's). When set, this library takes both values of a color from
  that appearance. What this library does not draw (the app's own Tailwind classes, NSH's native controls) is not
  affected.

## Theme files

- `themes/default.json`: every token at its default, with references. Both a reference and a starting point.
- `themes/*.json`: presets (shadcn's zinc, blue or rose colors, a compact density).
- An app points `nsh.json`'s `"theme"` at its own file and writes only the tokens it changes; to use this library's
  tokens in its Tailwind classes, it copies their entries too.
