# Avatar

Status: spec (P3), not implemented.

A person's picture, or their initials while it loads, when it fails and when there is none; a badge over it; a
group of them that shows the first few and a count. The look is shadcn/ui new-york v4 `avatar.tsx` (`Avatar`,
`AvatarImage`, `AvatarFallback`, `AvatarBadge`, `AvatarGroup`, `AvatarGroupCount`); the behavior is `nsh/base`'s
`Avatar` and `AvatarGroup`.

## Usage

- The account in a header, the author of a comment, the members of a project, the people in a thread, an assignee
  in a table row.
- Search terms: avatar, profile picture, user picture, initials, avatar group, avatar stack, presence dot.

## When not to use

| Need | Use instead |
|---|---|
| A logo, a product image, a thumbnail of a file | NSH's `Image` with the app's classes |
| A status mark or a count on its own | `Badge` |
| A pressable avatar (an account menu) | The avatar inside a `Button variant="ghost" iconOnly` (the trigger of a `DropdownMenu`), not a pressable avatar |
| A presence that must be read out | Say it in `accessibilityLabel` ("Ada Lovelace, online"); the badge is only drawn |

## Built on

`nsh/base`'s `Avatar` and `AvatarGroup` (`crates/nsh-base/src/avatar.rs`). They own the behavior and have no look:

| Provided by `nsh/base` (not rewritten here) | |
|---|---|
| Loading | The image loads natively; it shows once loaded (transparent until then), the fallback while it loads (after `fallbackDelay`), when it fails and without `src`; `onStatusChange({ status })`: `idle`, `loading`, `loaded`, `error`. A new `src` loads from the start |
| Initials | Without a `fallback`, the initials of `name`: the first letters of the first two words, upper case ("Ada Lovelace" → "AL"), a single word's first two letters, a script without letter case its first character ("王小明" → "王") |
| Accessibility | One `image` element named by `accessibilityLabel`, else `name`; an empty label makes it decorative (hidden), as HTML's `alt=""`; its initials and children are not read apart |
| Group | `AvatarGroup` shows its first `limit` children (default 3), makes no views for the others, shows `overflow` after them while `total` (default the number of children) is more than it shows; a `group` for screen readers, `overflowAccessibilityLabel` |

This library adds the look: the circle and its sizes, the fallback's fill and type, the image's fit, the badge, the
group's overlap and rings, and the count's look.

### Differs from shadcn's structure

shadcn composes `<Avatar><AvatarImage src /><AvatarFallback>CN</AvatarFallback></Avatar>`. `nsh/base`'s `Avatar`
holds the image and the fallback itself (it decides which shows from the image's load state, natively), so here:

| shadcn | This library |
|---|---|
| `<AvatarImage src alt>` | `Avatar`'s `src` and `name` (`alt` is the name screen readers hear) |
| `<AvatarFallback>CN</AvatarFallback>` | The initials of `name` by default; `fallback={…}` for other content (an icon) |
| `<Avatar size="sm">` | `size="sm"` (and `xs`, `md`, `lg`) |
| `<AvatarBadge />` | `<AvatarBadge />` as a child of `Avatar` |
| `<AvatarGroup>` + `<AvatarGroupCount>+3</AvatarGroupCount>` | `<AvatarGroup limit total overflow={<AvatarGroupCount>+3</AvatarGroupCount>}>` |
| `className="rounded-lg"` (a square avatar) | The same `class`, or `avatar-radius` in a theme |

The image and the fallback are parts styled with `imageClass` / `fallbackClass` (`nsh/base`'s), not components:
a component cannot fill another's place in NSH (an `AvatarImage` child could not reach `nsh/base`'s image).

### Implementation

- `#[component] fn Avatar` whose root is `nsh_base::avatar()`, `src`, `name`, `accessibilityLabel`, `fallbackDelay`,
  `fallback` and `onStatusChange` passed through. The root is sized and clipped by the library (`overflow: hidden`,
  the radius); the image takes `object-fit: cover` (`aspect-square size-full`); the fallback its fill, text color and
  centered type. These are classes generated from the resolved tokens (as DataTable's, data-table.md NSH gaps 1),
  put before the app's `imageClass` / `fallbackClass`; the app's `imageStyle` / `fallbackStyle` pass through and win
  over both.
- `Avatar` provides its `size` in context (`AvatarSize`): `AvatarBadge` reads it.
- `AvatarGroup` provides "in a group" and its `size` in context: an `Avatar` inside it without a `size` of its own
  takes the group's, draws its ring, and every one after the first overlaps the one before (a negative start margin;
  the group counts its children's places through `nsh/base`, which lays them out in order).
- The ring around an avatar in a group, and around a badge, is drawn as the focus ring is (`foundation::focus_ring`'s
  geometry: outside the box, taking no layout), in `avatar-ring`, so the overlap does not change the avatars' size.

## API

### Components

| Component | TSX | Rust | Description |
|---|---|---|---|
| `Avatar` | `<Avatar>` | `avatar()` | The picture or the initials |
| `AvatarBadge` | `<AvatarBadge>` | `avatar_badge()` | A dot or an icon at the avatar's bottom end, as its child |
| `AvatarGroup` | `<AvatarGroup>` | `avatar_group()` | A row of overlapping avatars, the first `limit` |
| `AvatarGroupCount` | `<AvatarGroupCount>` | `avatar_group_count()` | The circle for the people not shown ("+3", an icon), in `AvatarGroup`'s `overflow` |

`nsh/base` exports `Avatar` and `AvatarGroup` too; an app using both renames one.

### `Avatar` props

| TSX | Rust | Type | Default | Description |
|---|---|---|---|---|
| `src` | `src` | `ImageSrc` (a URL, `asset:/`, `data:`, an imported image) | — | The picture |
| `name` | `name` | `string` | — | The person's name: what screen readers say, and the initials |
| `accessibilityLabel` | `accessibility_label` | `string` | — | Over `name` for screen readers; `""` makes the avatar decorative (beside the name it shows) |
| `size` | `size` | `Size?` | The group's, else `"md"` | `xs` / `sm` / `md` / `lg`; see Sizes |
| `fallback` | `fallback` | element (slot) | The initials | The fallback's content in place of the initials (an icon) |
| `fallbackDelay` | `fallback_delay` | `number` (ms) | `0` | How long the fallback waits while the image loads (`Infinity`: not while it loads) |
| `imageClass` / `imageStyle` | `image_class` / `image_style` | | — | The image, after its look |
| `fallbackClass` / `fallbackStyle` | `fallback_class` / `fallback_style` | | — | The fallback, after its look |
| `class` / `style` | `class` / `style` | | — | The root: size, shape (`rounded-lg`), merged last |
| `children` | `.child(..)` | elements | — | Over the picture: an `AvatarBadge`, or the app's own mark |

`size` is optional (Rust `Option<Size>`) so an avatar in a group follows the group unless it says otherwise.

### Events

| TSX | Rust (`AvatarEvent`) | Payload | When |
|---|---|---|---|
| `onStatusChange` | `StatusChange(AvatarStatus)` | `{ status }`: `"idle"` \| `"loading"` \| `"loaded"` \| `"error"` | The image's status changed (`nsh/base`'s, passed on; Radix's `onLoadingStatusChange`) |

### `AvatarBadge` props

`class`, `style`, `children` (an icon: `<Icon name="check" />`; hidden at `xs` and `sm`, as shadcn's at `sm`). No
size of its own: it follows the avatar's.

### `AvatarGroup` props

| TSX | Rust | Type | Default | Description |
|---|---|---|---|---|
| `size` | `size` | `Size` | `"md"` | The size of its avatars (those without their own) and of the count |
| `limit` | `limit` | `number` | `3` | At most this many avatars show, the first ones |
| `total` | `total` | `number?` | Its children | How many people it stands for, when the app passes only the first ones |
| `overflow` | `overflow` | element (slot) | — | What stands for the rest (an `AvatarGroupCount`), shown while there are more than it shows; the app writes the count |
| `accessibilityLabel` | `accessibility_label` | `string` | — | What screen readers say of the group ("Ada, Grace and 5 others") |
| `overflowAccessibilityLabel` | `overflow_accessibility_label` | `string` | — | What they say of the overflow ("5 more people") |
| `overflowClass` / `overflowStyle` | `overflow_class` / `overflow_style` | | — | The overflow's box (`nsh/base`'s) |
| `class` / `style` | `class` / `style` | | — | The row |
| `children` | `.child(..)` | `Avatar`s | — | The avatars, in order |

### `AvatarGroupCount` props

`class`, `style`, `children`: the count ("+3") or an icon. It takes the group's size.

### Parts

| Part | What | Customize with |
|---|---|---|
| root | The circle: size, radius, clip | `class` / `style` |
| image | The picture, covering the circle | `imageClass` / `imageStyle` |
| fallback | The initials or `fallback`, on the muted fill | `fallbackClass` / `fallbackStyle` |
| `AvatarBadge` | The dot at the bottom end | its `class` / `style` |
| `AvatarGroup` | The row | its `class` / `style` |
| `AvatarGroupCount` | The count's circle | its `class` / `style` |

## Variants and sizes

No variants (shadcn has none).

### Look

| Part | shadcn classes | This library |
|---|---|---|
| `Avatar` | `relative flex size-8 shrink-0 overflow-hidden rounded-full select-none` | Side `avatar-size-<size>`, corners `avatar-radius` (a circle), clipped, `flex-shrink: 0`, text not selectable |
| image | `aspect-square size-full` | Covers the circle (`object-fit: cover`) |
| fallback | `flex size-full items-center justify-center rounded-full bg-muted text-sm text-muted-foreground group-data-[size=sm]/avatar:text-xs` | Fill `avatar-fallback`, text `avatar-fallback-foreground` at `avatar-fallback-font-size-<size>`, centered |
| `AvatarBadge` | `absolute right-0 bottom-0 z-10 inline-flex items-center justify-center rounded-full bg-primary text-primary-foreground ring-2 ring-background`; `size-2` / `size-2.5` / `size-3`, its icon `size-2`, none at `sm` | At the bottom end over the avatar, a circle `avatar-badge-size-<size>`, fill `avatar-badge`, its content `avatar-badge-foreground`, a ring of `avatar-ring-width` in `avatar-ring`; an icon `avatar-badge-size-<size> − 2`, hidden at `xs` and `sm` |
| `AvatarGroup` | `flex -space-x-2 *:data-[slot=avatar]:ring-2 *:data-[slot=avatar]:ring-background` | A row; each avatar after the first `avatar-group-overlap` (8) over the one before; each with a ring of `avatar-ring-width` (2) in `avatar-ring` |
| `AvatarGroupCount` | `relative flex size-8 shrink-0 items-center justify-center rounded-full bg-muted text-sm text-muted-foreground ring-2 ring-background`, icon `size-4` (`size-3` at `sm`, `size-5` at `lg`) | The fallback's look at the group's size, with the ring; an icon half the side |

### Sizes

| `size` | Side | Initials | Badge | Count's icon | shadcn |
|---|---|---|---|---|---|
| `xs` | 20 | 12 | 6, no icon | 10 | — |
| `sm` | 24 | 12 | 8, no icon | 12 | `sm`: `size-6`, `text-xs`, badge `size-2` |
| `md` | 32 | 14 | 10 | 16 | `default`: `size-8`, `text-sm`, badge `size-2.5` |
| `lg` | 40 | 14 | 12 | 20 | `lg`: `size-10`, badge `size-3` |

- shadcn's sizes as they are: an avatar is no control, so it does not step down with the control heights; `md` (32)
  is as tall as a `md` control, `lg` (40) as a touch one. `xs` (20) is added for dense rows (a `Table` of `xs` / `sm`).
- The same on touch screens: an avatar is not a press target.
- Any other side is a class (`class="size-12"`); the fallback's type then stays the `size`'s.

## States

| State | Look |
|---|---|
| No `src`, or the image failed | The fallback |
| Loading | The fallback after `fallbackDelay` (none before); the image transparent |
| Loaded | The image; the fallback gone. No fade by default (shadcn has none); `imageClass="transition-opacity"` fades it in (`nsh/base`) |

No hover, focus or press: an avatar is not interactive.

## Pointer and touch

The same on both: an avatar is not a press target. A pressable avatar is a `Button` around it, which takes Button's
pointer and touch rules (`iconOnly`, a hit slop on touch screens).

## Accessibility

`nsh/base`'s: one `image` named by `accessibilityLabel` or `name`; decorative (hidden) with an empty label or no
name; its initials and badge are not read apart (say a presence in the label). A group is a `group` named by
`accessibilityLabel`; its overflow is read by `overflowAccessibilityLabel`, else its text ("+3"). On iOS nothing
inside an avatar can be reached: put what can be pressed beside it.

## Tokens

Generated from the code's declarations (`NSH_LONGBRIDGE_WRITE=1 cargo test --test tokens`); a test keeps it current.
Values are those of `nsh/base`'s default theme.

<!-- tokens:avatar -->
| Token | Type | Derived default | Value |
|---|---|---|---|
| `avatar-size-xs` | length | `spacing × 5` | 20 |
| `avatar-size-sm` | length | `spacing × 6` | 24 |
| `avatar-size-md` | length | `spacing × 8` | 32 |
| `avatar-size-lg` | length | `spacing × 10` | 40 |
| `avatar-radius` | length | `9999` | 9999 |
| `avatar-fallback` | color | `muted` |  |
| `avatar-fallback-foreground` | color | `muted-foreground` |  |
| `avatar-fallback-font-size-xs` | length | `12` | 12 |
| `avatar-fallback-font-size-sm` | length | `12` | 12 |
| `avatar-fallback-font-size-md` | length | `14` | 14 |
| `avatar-fallback-font-size-lg` | length | `14` | 14 |
| `avatar-badge` | color | `primary` |  |
| `avatar-badge-foreground` | color | `primary-foreground` |  |
| `avatar-badge-size-xs` | length | `spacing × 1.5` | 6 |
| `avatar-badge-size-sm` | length | `spacing × 2` | 8 |
| `avatar-badge-size-md` | length | `spacing × 2.5` | 10 |
| `avatar-badge-size-lg` | length | `spacing × 3` | 12 |
| `avatar-ring` | color | `background` |  |
| `avatar-ring-width` | length | `2` | 2 |
| `avatar-group-overlap` | length | `spacing × 2` | 8 |
<!-- /tokens -->

- `avatar-radius` is a circle at any side (shadcn's `rounded-full`, a radius past half the side); `radius-lg` there
  gives every avatar of the app shadcn's square example (`rounded-lg`).
- The fallback's type is Tailwind's fixed `text-xs` / `text-sm` (as `select-label-font-size`), not the controls'
  `font-size-*`.
- `avatar-ring` is the page's color: on a `Card` set it to `card` in a scope, or the rings read as gaps of another
  color (shadcn's `ring-background` has the same limit).
- `AvatarGroupCount` reads `avatar-fallback` and `avatar-fallback-foreground`: it is a fallback with a count.
- Other tokens it reads: `spacing`. The initials are regular weight, as shadcn's.

## Examples

TSX:

```tsx
import { Text, View } from "nsh";
import { Avatar, AvatarBadge, AvatarGroup, AvatarGroupCount, Button, DropdownMenu } from "nsh-longbridge";

<Avatar src="https://github.com/shadcn.png" name="shadcn" />
<Avatar name="Ada Lovelace" size="lg" />                                   {/* "AL" */}
<Avatar src={photo} name="Ada Lovelace" class="rounded-lg" />

// Online: the badge is drawn; the label says it.
<Avatar src={photo} name="Ada Lovelace" accessibilityLabel="Ada Lovelace, online">
  <AvatarBadge class="bg-green-600" />
</Avatar>

// Beside the name it shows: decorative.
<View class="flex-row items-center gap-3">
  <Avatar src={photo} name="Olivia Martin" accessibilityLabel="" />
  <Text>Olivia Martin</Text>
</View>

// The first three of eight members, then "+5".
<AvatarGroup
  size="sm"
  limit={3}
  total={members.length}
  accessibilityLabel={`${members[0].name}, ${members[1].name} and ${members.length - 2} others`}
  overflow={<AvatarGroupCount>{`+${members.length - 3}`}</AvatarGroupCount>}
  overflowAccessibilityLabel={`${members.length - 3} more people`}
>
  {members.slice(0, 3).map((m) => <Avatar src={m.photo} name={m.name} />)}
</AvatarGroup>

// An account menu: the avatar in a ghost icon button.
<DropdownMenu trigger={<Button variant="ghost" iconOnly class="rounded-full" accessibilityLabel="Account"><Avatar name="Ada Lovelace" accessibilityLabel="" /></Button>}>
  …
</DropdownMenu>
```

With NSH 0.7.0, write a string child inside `<Text>` for now (see [button.md, NSH gaps](button.md#nsh-gaps), G0).

Rust:

```rust
use nsh_core::prelude::*;
use nsh_longbridge::{Size, avatar, avatar_badge, avatar_group, avatar_group_count};

avatar().src("https://github.com/shadcn.png").name("shadcn").size(Size::Lg)

avatar_group()
    .size(Size::Sm)
    .total(8)
    .overflow(avatar_group_count().child("+5"))
    .child(avatar().name("Ada Lovelace"))
    .child(avatar().name("Grace Hopper"))
    .child(avatar().name("Alan Turing"))
```

### The gallery

The showcase's `Avatar` (a `Badge variant="secondary"` made round, `size-8`) becomes `<Avatar name={m.name} />` with
the member's name: the initials come from it, and the members' card gets the library's fallback look (a scope still
colors it: `avatar-fallback` is `muted`).

## NSH gaps

1. **The image and the fallback are `nsh/base`'s parts**, whose Rust `imageStyle` / `fallbackStyle` setters take a
   style built once (Checkbox's NSH gaps 3), and whose classes lose to inline styles. Workaround: the look as classes
   generated from the tokens in a binding, before the app's `imageClass` / `fallbackClass` (Implementation), so
   `fallbackClass="bg-primary"` works and a scope reaches the look.
2. **No `*:` child selector** (shadcn's `*:data-[slot=avatar]:ring-2` on the group). Workaround: the group's context;
   an avatar of the library inside reads it. An app's own element in the group gets no ring.
3. **The overflow's count is the app's** (`nsh/base`: it knows the count). shadcn's `AvatarGroupCount` is the app's
   too; the library only draws it.
4. **String children** follow Button's G0 in TSX (`AvatarGroupCount`'s "+3").

## Decisions

Proposed; for review.

1. **`src` and `name` on `Avatar`**, not `AvatarImage` / `AvatarFallback` children (Differs from shadcn's
   structure): `nsh/base` decides which shows natively.
2. **The initials come from `name`** (`nsh/base`'s rule, gpui-component's), so one prop gives both the screen
   readers' name and the fallback; `fallback` replaces them.
3. **Four sizes**, shadcn's three and `xs`, at shadcn's sides (not stepped down: an avatar is not a control).
4. **`size` is optional on `Avatar`** (Rust `Option<Size>`), so an avatar follows its group; elsewhere unset is `md`.
