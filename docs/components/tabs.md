# Tabs

Status: implemented (P2). A set of views of one page, one shown at a time, switched by a row of tabs:
a segmented tab list, and an underlined `line` variant.

## Usage

- The sections of a settings page, the views of one object (Overview, Activity, Settings), a chart's periods, the panes
  of a dialog.
- Search terms: tabs, tab bar, tab list, segmented control, segmented tabs, underline tabs.

## When not to use

| Need | Use instead |
|---|---|
| Navigating between pages or windows of the app | The app's navigation (`nsh/base`'s navigation, a sidebar) |
| Long-lived documents the user opens and closes | A dock or tab area of the app's (gpui-kit's document tabs), not these tabs |
| Choosing a value a form submits | [RadioGroup](radio-group.md), [Select](select.md) |
| The system's segmented control on phones | `nsh/mobile`'s `SegmentedControl` (native on iOS and Android) |
| A bar at the bottom of a phone app | `nsh/mobile`'s `TabBar` |

## Built on

`nsh/base`'s `Tabs` (the list) and `Tab` (`crates/nsh-base/src/tabs.rs`, keys in `roving.rs`). `nsh/base` holds no value
and no content: each tab is told whether it is selected and reports `onPress`. This library adds what tabs
need around them: the value (`Tabs`, controlled or not), the panels (`TabsContent`) and the look.

| Provided by `nsh/base` (not rewritten here) | |
|---|---|
| Keyboard | The list is one Tab stop, at the selected tab (else the first enabled); with an orientation, Left / Right (a row) or Up / Down (a column) move between tabs around the ends, the other pair left to the page; Home / End; disabled tabs passed over |
| Activation | The tab the focus moves to reports `onPress` (automatic activation, the common default); Space and Return press the focused tab; presses from the pointer and touch |
| Accessibility | The list a `tablist` named by `accessibilityLabel`, with its orientation; each tab a `tab` with `selected`; `disabled` |
| States | `selected:` and `disabled:` on each tab; `focus-visible:` for keyboard focus |
| Terminal | Tabs in a row two Cells apart, the selected one bold on a band; this library's look does not apply there |

`nsh/base`'s indicator (a view that travels to the selected tab with a spring) is **not used**: see Decisions 2.

### Implementation

- **`Tabs`** (the root) is a view of this library's. It holds the value (the app's `value`, else its own signal from
  `defaultValue`) and provides a `TabsContext` (the value, the setter that reports `onChange`, the orientation).
- **`TabsList`** is `nsh_base::tabs()`, with `orientation` from the root's context (read when made, as `nsh/base`
  reads it) and `accessibilityLabel`; it provides the `variant` and `size` to its triggers.
- **`TabsTrigger`** is `nsh_base::tab()` with `selected = context.value == value` and `onPress` setting the value
  (reported only when it changes). Its look is set on its root element with bindings; the `line` variant's underline is
  a view of this library's inside it. A trigger outside a `Tabs` renders unselected and logs a warning in development
  builds.
- **`TabsContent`** is a view of this library's, made only while its value is the selected one (a hidden panel
  holds nothing), with the `tabpanel` role.

## API

### Components

| Component | TSX | Rust | What it is |
|---|---|---|---|
| `Tabs` | `<Tabs>` | `tabs()` | The root: the value, the list and the panels |
| `TabsList` | `<TabsList>` | `tabs_list()` | The row (or column) of triggers |
| `TabsTrigger` | `<TabsTrigger>` | `tabs_trigger()` | One tab |
| `TabsContent` | `<TabsContent>` | `tabs_content()` | The panel shown while its tab is selected |

TSX apps that also use `nsh/base`'s `Tabs` rename one (`import { Tabs as BaseTabs } from "nsh/base"`), as conventions
say.

### `Tabs`

| TSX | Rust | Type | Default | Description |
|---|---|---|---|---|
| `value` | `value` | `string` / `Prop<Option<SharedString>>` | Uncontrolled | The selected tab's value (controlled): the app sets it in `onChange` |
| `defaultValue` | `default_value` | `string` | — | The first value while uncontrolled |
| `orientation` | `orientation` | `"horizontal"` \| `"vertical"` (`Axis`) | `"horizontal"` | The list's direction and keys; the layout: the list above the panels (horizontal) or beside them (vertical). Read when made |
| `children` | `.child(..)` | `TabsList`, `TabsContent`, elements | — | |
| `class` / `style` | `class` / `style` | | | The root |

Event: `onChange({ value })` (`TabsEvent::Change(SharedString)`, `#[args(value)]`), when the user selects another tab
(a press, or an arrow, Home or End moving the focus to it). NSH's name (conventions), not `onValueChange`.

### `TabsList`

| TSX | Rust | Type | Default | Description |
|---|---|---|---|---|
| `variant` | `variant` | `"default"` \| `"line"` (`TabsVariant`) | `"default"` | `default`: a segmented list; `line`: underlined tabs on no fill |
| `size` | `size` | `Size` | `"md"` | The list's height and the triggers' type, icon and padding |
| `accessibilityLabel` | `accessibility_label` | `string` | — | What screen readers call the list ("Sections") |
| `children` | `.child(..)` | `TabsTrigger`s, elements | — | The triggers, or views around them (a badge beside one) |
| `class` / `style` | `class` / `style` | | | The list |

### `TabsTrigger`

| TSX | Rust | Type | Default | Description |
|---|---|---|---|---|
| `value` | `value` | `string` | — | Its value, unique in its `Tabs` |
| `children` | `.child(..)` | text, elements | — | Its label, in a `label` part |
| `icon` | `icon` | `IconName` | — | An icon before the label |
| `disabled` | `disabled` | `boolean` | `false` | No presses, passed over by the keys, dimmed |
| `accessibilityLabel` | `accessibility_label` | `string` | — | Its name, when its children have no text (an icon-only tab) |
| `accessibilityDescription` | `accessibility_description` | `string` | — | Read after the name |
| `class` / `style` | `class` / `style` | | | The tab |
| `labelClass` / `labelStyle`, `iconClass` / `iconStyle` | `label_class` …, `icon_class` … | | | The `label` and `icon` parts |

No events of its own: the root reports `onChange`.

### `TabsContent`

| TSX | Rust | Type | Description |
|---|---|---|---|
| `value` | `value` | `string` | The tab it belongs to |
| `accessibilityLabel` | `accessibility_label` | `string` | What screen readers call the panel, when its first heading does not name it (Accessibility) |
| `children` | `.child(..)` | elements | Made while selected, released otherwise |
| `class` / `style` | `class` / `style` | | The panel (`flex: 1`) |

A `forceMount` (keeping a hidden panel's state) is not offered in P2: an app that must keep a panel's state keeps
it in its own signals, or shows and hides its own views with the value.

### Parts

| Part | What | Customize with |
|---|---|---|
| list | `nsh/base`'s tab list | `TabsList`'s `class` / `style` |
| trigger | `nsh/base`'s tab | `TabsTrigger`'s `class` / `style` |
| `label` | The text wrapping a trigger's children | `labelClass` / `labelStyle` |
| `icon` | A trigger's icon | `iconClass` / `iconStyle` |
| content | A panel | `TabsContent`'s `class` / `style` |

## Variants and sizes

In classes (the list's `h-9` is `lg`):

- Root: `flex gap-2`, a column when horizontal.
- List: `inline-flex w-fit items-center justify-center rounded-lg p-0.5 text-muted-foreground h-9` (horizontal),
  `h-fit flex-col` (vertical); `default`: `bg-muted`; `line`: `gap-1 bg-transparent rounded-none`.
- Trigger: `h-full flex-1 items-center justify-center gap-2 rounded-md border border-transparent px-2
  py-1 text-sm font-medium whitespace-nowrap text-foreground/60 hover:text-foreground transition-all`, icons `size-4`;
  vertical `w-full justify-start`; selected `bg-background text-foreground shadow-sm`, dark `border-input
  bg-input/30 text-foreground`; dark rest `text-muted-foreground`. `line`: selected has no fill, border or shadow, and
  an underline (`after:`) `bg-foreground h-0.5` at `bottom-[-3px]`, on the list's bottom edge, fading in; vertical: no
  underline, the selected trigger filled `bg-accent` with a selected tab's text (`tabs-trigger-selected-foreground`, so a
  hovered tab, `foreground`, is never darker than the selected one) (Decisions 6). `gap-1` at `xs` and `sm`.
- Content: `flex-1 outline-none`.

### Variants

| | `default` (segmented) | `line` |
|---|---|---|
| List | Fill `tabs-list`, corners `tabs-list-radius` (10), padding `tabs-list-padding` (2) | No fill, square, the same padding, triggers `spacing` (4) apart |
| Trigger at rest | No fill, a 1 pt edge in transparent, text `tabs-trigger-foreground` | Same |
| Trigger hovered | Text `tabs-trigger-hover-foreground` | Same |
| Trigger selected | Fill `tabs-trigger-selected`, edge `tabs-trigger-selected-border`, shadow tier `tabs-trigger-selected-shadow` (2, `sm`), text `tabs-trigger-selected-foreground` | Text `tabs-trigger-selected-foreground`; an underline `tabs-line-indicator-width` (2) thick in `tabs-line-indicator`, as long as the trigger, its bottom edge on the list's, at opacity 1 (0 otherwise); a column: fill `tabs-line-selected`, text `tabs-line-selected-foreground`, no underline |
| Dark | Rest text `muted-foreground`; selected fill `input` at 30%, edge `input` | Rest text `muted-foreground`; no selected edge |

### Sizes

| `size` | List height (pointer / touch) | Trigger height | Text | Icon |
|---|---|---|---|---|
| `xs` | 24 / 32 | 20 / 28 | 12 | 12 |
| `sm` | 28 / 36 | 24 / 32 | 13 | 14 |
| `md` | 32 / 40 | 28 / 36 | 14 | 16 |
| `lg` | 36 / 48 | 32 / 44 | 14 | 16 |

- The list's height is `tabs-list-height-<size>` (`control-height-<size>`): 32 at `md`, 36 at `lg`,
  as every control here. A vertical list sizes to its triggers instead, each as tall as in a row.
- A trigger fills the list's height inside its padding, its content centered: the band around the selected trigger is
  `tabs-list-padding` (2) on every side, so its corners are concentric with the list's (gpui-kit's Design Guides: the
  selected fill shares the container's silhouette). It has no vertical padding (a fixed `py-1` would fit one size
  only). Horizontal padding `tabs-trigger-padding-x` (8), icon-to-label gap 4 at `xs` and `sm`, 8 at `md` and `lg`
  (Button's), text `font-size-<size>` at `font-weight-medium`, one line.
- Triggers share the list's width, each growing from its content (`flex: 1 1 auto`; see [NSH gaps](#nsh-gaps) 5); the
  list is as wide as its content (`w-fit`, and `h-fit` beside vertical panels: a root that stretches its children does
  not stretch the list, and a row's `items-center` still centers it), or as the app's `class` sets (`w-full`,
  `w-[400px]`). In a column each trigger takes the full width and starts its content at the start.
- Corners: the list `tabs-list-radius` (`radius-lg`, 10), the trigger `tabs-trigger-radius` = list radius − padding
  (8, `radius-md`): concentric (conventions).
- Root: the list and the panels `tabs-gap` (8) apart, in a column (horizontal) or a row (vertical).

## States

| State | Look |
|---|---|
| Hover (pointer, enabled) | Text `tabs-trigger-hover-foreground`; the selected tab unchanged |
| Pressed | No look of its own (the selection follows at once) |
| Selected | See Variants |
| Focus-visible | Edge `tabs-trigger-border-focus` and a `ring-width` (3) ring in `tabs-trigger-ring` outside the trigger, taking no layout (`focus-visible:border-ring ring-[3px] ring-ring/50`; an extra `outline-1 outline-ring` would be the same edge drawn again and is not added). Touch: the edge only |
| Disabled | `opacity-disabled`, no hover; the arrow cursor |

Colors, the edge, the fill and the shadow transition over `duration` (`transition-all`), none with reduced
motion; the `line` underline fades over `duration`. The cursor is the arrow (a tab is not a link). The list's ring needs
`ring-width` of room around it: a `TabsList` against the edge of an `overflow-hidden` container gets its ring clipped
(as Input says).

## Pointer and touch

| | Pointer | Touch |
|---|---|---|
| List height | 24 / 28 / 32 / 36 | 32 / 36 / 40 / 48 |
| Hit area | The trigger | The trigger (no hit slop: NSH gaps 4) |
| Focus | Edge and ring | Edge only |
| Hover | Yes | No |
| Many tabs | — | The app puts the `TabsList` in a horizontal scroll view (`nsh/base` keeps the arrow pair across the row for the page) |

The same look on every platform; a phone app that wants the system's segmented control uses `nsh/mobile`'s
`SegmentedControl`.

## Accessibility

- `nsh/base`'s: the list is a `tablist` with its orientation and `accessibilityLabel`; each trigger a `tab` with
  `selected` and `disabled`; one Tab stop, the arrows, Home and End (see Built on).
- `TabsContent` has the `tabpanel` role. NSH has no labelledby or controls link between elements a component does
  not own (Input's G1), so a panel is not named by its tab; give the panel an `accessibilityLabel` when its first
  heading does not name it.
- Icons and the underline are hidden from screen readers. An icon-only trigger needs `accessibilityLabel`.

## Tokens

Generated from the code's declarations (`NSH_LONGBRIDGE_WRITE=1 cargo test --test tokens`); a test keeps it current.
Values are those of `nsh/base`'s default theme.

<!-- tokens:tabs -->
| Token | Type | Derived default | Value |
|---|---|---|---|
| `tabs-gap` | length | `spacing × 2` | 8 |
| `tabs-list` | color | `muted` |  |
| `tabs-list-radius` | length | `radius-lg` | 10 |
| `tabs-list-padding` | length | `spacing × 0.5` | 2 |
| `tabs-list-height-xs` | length | `pointer_touch(control-height-xs, control-height-touch-xs)` | 24 (touch 32) |
| `tabs-list-height-sm` | length | `pointer_touch(control-height-sm, control-height-touch-sm)` | 28 (touch 36) |
| `tabs-list-height-md` | length | `pointer_touch(control-height-md, control-height-touch-md)` | 32 (touch 40) |
| `tabs-list-height-lg` | length | `pointer_touch(control-height-lg, control-height-touch-lg)` | 36 (touch 48) |
| `tabs-trigger-radius` | length | `tabs-list-radius − tabs-list-padding` | 8 |
| `tabs-trigger-padding-x` | length | `spacing × 2` | 8 |
| `tabs-trigger-foreground` | color | `light_dark(foreground × 60%, muted-foreground)` |  |
| `tabs-trigger-hover-foreground` | color | `foreground` |  |
| `tabs-trigger-selected` | color | `light_dark(background, input × 30%)` |  |
| `tabs-trigger-selected-foreground` | color | `foreground` |  |
| `tabs-trigger-selected-border` | color | `light_dark(transparent, input)` |  |
| `tabs-trigger-selected-shadow` | number | `2` | 2 |
| `tabs-trigger-border-focus` | color | `ring` |  |
| `tabs-trigger-ring` | color | `ring × ring-opacity` |  |
| `tabs-line-indicator` | color | `foreground` |  |
| `tabs-line-indicator-width` | length | `2` | 2 |
| `tabs-line-selected` | color | `accent` |  |
| `tabs-line-selected-foreground` | color | `tabs-trigger-selected-foreground` |  |
<!-- /tokens -->

- `selected` is the trigger's form, so it goes where a variant goes (`tabs-trigger-selected-border`), as Checkbox's
  `checkbox-checked-border`; `tabs-trigger-hover-foreground` follows Button's `button-outline-hover-foreground`.
- The `line` variant uses the same trigger tokens and ignores the selected fill, edge and shadow; a theme that wants a
  filled line tab writes `class` on its triggers.
- `tabs-trigger-radius` is drawn between 0 and half the trigger's height.

Other tokens it reads: `spacing`, `font-size-*`, `icon-size-*`, `font-weight-medium`, `border-width`, `ring-width`,
`opacity-disabled`, `touch-target`, `shadow`, `duration`.

## Examples

```tsx
import { signal } from "nsh";
import { Card, CardContent, Tabs, TabsContent, TabsList, TabsTrigger } from "nsh-longbridge";

const tab = signal("account");

<Tabs value={tab()} onChange={({ value }) => tab.set(value)} class="w-[400px]">
  <TabsList accessibilityLabel="Settings">
    <TabsTrigger value="account">Account</TabsTrigger>
    <TabsTrigger value="password">Password</TabsTrigger>
    <TabsTrigger value="billing" disabled>Billing</TabsTrigger>
  </TabsList>
  <TabsContent value="account">
    <Card><CardContent>{/* account form */}</CardContent></Card>
  </TabsContent>
  <TabsContent value="password">
    <Card><CardContent>{/* password form */}</CardContent></Card>
  </TabsContent>
</Tabs>

// Underlined, uncontrolled, small, with icons.
<Tabs defaultValue="overview">
  <TabsList variant="line" size="sm" accessibilityLabel="Views">
    <TabsTrigger value="overview" icon="layout-dashboard">Overview</TabsTrigger>
    <TabsTrigger value="activity" icon="activity">Activity</TabsTrigger>
  </TabsList>
  <TabsContent value="overview">{/* … */}</TabsContent>
  <TabsContent value="activity">{/* … */}</TabsContent>
</Tabs>

// Vertical.
<Tabs defaultValue="general" orientation="vertical">
  <TabsList accessibilityLabel="Preferences">
    <TabsTrigger value="general">General</TabsTrigger>
    <TabsTrigger value="appearance">Appearance</TabsTrigger>
  </TabsList>
  <TabsContent value="general">{/* … */}</TabsContent>
  <TabsContent value="appearance">{/* … */}</TabsContent>
</Tabs>
```

With NSH 0.7.0, write a string child inside `<Text>` for now (Button's G0). The icons named here are lucide names the
library adds to `IconName` as needed.

```rust
use nsh_core::prelude::*;
use nsh_longbridge::{tabs, tabs_content, tabs_list, tabs_trigger, Size, TabsVariant};

let tab = signal(SharedString::from("account"));

tabs()
    .value(move || Some(tab.get()))
    .on_change(move |value: &SharedString| tab.set(value.clone()))
    .child(
        tabs_list()
            .variant(TabsVariant::Line)
            .size(Size::Sm)
            .accessibility_label("Settings")
            .child(tabs_trigger().value("account").child("Account"))
            .child(tabs_trigger().value("password").child("Password")),
    )
    .child(tabs_content().value("account").child(account_form()))
    .child(tabs_content().value("password").child(password_form()))
```

## NSH gaps

1. **`nsh/base`'s indicator takes a fixed style in Rust and its class loses to its style.** `Tabs`' `indicatorStyle`
   setter builds a style once (bound values inside stay bound, a shadow's layers do not), and the library's look
   there would win over the app's `indicatorClass`. Workaround: no `nsh/base` indicator; each trigger draws its own
   selected look (Decisions 2). A reactive `indicatorStyle` would let a sliding segment be offered.
2. **No link between a tab and its panel** (the web's `aria-controls` / `aria-labelledby`): NSH has no labelledby
   between elements (Input's G1). Workaround: the panel's own `accessibilityLabel` when needed.
3. **String children** (Button's G0): the gallery wraps them in `<Text>` until NSH's fix is released.
4. **No hit slop on a tab.** `nsh/base`'s `Tab` takes no `hitSlop`, and a component's element takes none from its
   parent, so a trigger on a touch screen is hit within its own bounds (33 tall in a 40 list at `md`), not the
   vertical (44 − list height) / 2 Input has. A `hitSlop` prop on `Tab` would add it.
5. **No content minimum for flex items** (CSS's `min-width: auto`). In a browser a `flex-1` trigger never shrinks below
   its label, so a list as wide as its content gives each label at least its padding. In NSH a trigger growing from 0
   could get less than its label (a long one lost most of its padding). Workaround: triggers grow from their content
   (`flex: 1 1 auto`): in a list as wide as its content each label has `tabs-trigger-padding-x` on both sides; in a
   wider list they share the extra room equally, where a browser makes them nearer to equal widths.

## Decisions

1. **The value lives in this library's `Tabs`** (controlled or not), with `TabsContent` panels; `nsh/base`
   keeps tabs stateless and its `Tab` / `Tabs` are used for the keys and accessibility only.
2. **The selected look is drawn on each trigger, not `nsh/base`'s travelling indicator**: the look exactly (the
   selected trigger's fill and shadow; `line`'s underline fading), drawn by the library so its tokens stay bound and the
   app's classes win (conventions: a part the app must restyle is drawn by the library). `nsh/base`'s spring-moving
   segment (gpui-kit's) is the better motion; offer it once its style takes bindings (gap 1).
3. **`line` is supported**: `nsh/base`'s tabs need nothing more for it.
4. **No `forceMount`**: inactive panels are released.

5. **The list is as long as its triggers through `w-fit` / `h-fit`, not `align-self: flex-start`**: an `align-self` of
   its own would override the cross-axis alignment of a row it sits in (a toolbar), putting it at the top instead of
   the middle (as `Badge`'s decision 3).
6. **A vertical `line` list fills the selected trigger instead of drawing a bar along its edge.** A row of section
   tabs is underlined, as most apps do; a column of them reads as a sidebar's section list, which apps mark with the
   row's own fill. An edge bar breaks the trigger's rounded shape and adds a second edge to a column that already
   aligns on its text (gpui-kit's Design Guides, interaction states).
