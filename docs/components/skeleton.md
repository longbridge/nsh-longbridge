# Skeleton

Status: implemented (P1). A loading placeholder: while content has not arrived, a slowly pulsing block holds its future place and shape.
Reference: shadcn/ui new-york v4 `skeleton.tsx`.

## Usage

- Placeholders for lists, cards, avatars and text on first load, so the layout does not jump.
- Search terms: skeleton, loading placeholder, shimmer placeholder.

## When not to use

- A button or a small area is waiting: use `Spinner` (a button's own `loading` already has one).
- The shape of the content is unknown, or loading is fast (< 300 ms): show nothing, or a `Spinner`; a skeleton far from
  the real shape makes the layout jump more, not less.
- Shimmering text: shadcn has none and neither does this library.

## Built on

`nsh/base` has no counterpart; this library draws a `view`. It has no behavior.

Width, height and corner shape are the app's, given with `class` / `style` as in shadcn
(`<Skeleton className="h-4 w-[250px]" />`). There is no default size: without one it is 0 × 0 and invisible.

## API

| TSX | Rust | Type | Default | Description |
|---|---|---|---|---|
| `class` / `style` | `class` / `style` | | | The root, which is the block; size and `rounded-full` go here |

No events, no children, no parts.

## Variants and sizes

- No `variant`.
- No `size` (none of `xs` / `sm` / `md` / `lg`): its size is the placeholder's shape, set by the app.

### Look

shadcn: `animate-pulse rounded-md bg-accent`.

| | Value |
|---|---|
| Fill | `skeleton` |
| Corners | `skeleton-radius`; a round avatar placeholder takes `class="rounded-full"` (`class` is merged last and wins over the token) |
| Animation | Tailwind's `animate-pulse`: 2 s per cycle, `cubic-bezier(0.4, 0, 0.6, 1)`, opacity 1 → 0.5 → 1, endless. NSH supports `animate-pulse` |

The animation is not a token: its period and curve are Tailwind's fixed values, and NSH tokens cannot hold a curve.

## States

- **Reduced motion**: no pulse, it stays opaque (`motion-reduce:animate-none`), in line with conventions.md (transitions
  go to zero under reduced motion). NSH's core starts no keyframe animation under reduced motion, so the component needs
  nothing of its own for it. The pulse is decoration only, so stopping it loses nothing. shadcn does not handle
  this.
- No hover / active / focus / disabled.

## Pointer and touch

No difference.

## Accessibility

- No Tab stop, no pointer events.
- Out of the accessibility tree (`accessible={false}`). The container says "loading": give it
  `accessibilityState={{ busy: true }}` or a label ("Loading comments"). The skeleton itself says nothing, since a screen
  often has a dozen of them and hearing each one is useless.

## Tokens

Generated from the code's declarations (`NSH_LONGBRIDGE_WRITE=1 cargo test --test tokens`); a test keeps it current.
Values are those of `nsh/base`'s default theme.

<!-- tokens:skeleton -->
| Token | Type | Derived default | Value |
|---|---|---|---|
| `skeleton` | color | `accent` |  |
| `skeleton-radius` | length | `radius-md` | 8 |
<!-- /tokens -->

`skeleton` follows `accent` (shadcn neutral: light `oklch(0.97 0 0)`, dark `oklch(0.269 0 0)`).
## Examples

```tsx
import { Skeleton } from "nsh-longbridge";

<View class="flex-row items-center gap-4" accessibilityLabel="Loading profile" accessibilityState={{ busy: true }}>
  <Skeleton class="size-12 rounded-full" />
  <View class="gap-2">
    <Skeleton class="h-4 w-[250px]" />
    <Skeleton class="h-4 w-[200px]" />
  </View>
</View>
```

```rust
use nsh_longbridge::skeleton;

view().flex_row().items_center().gap_4()
    .child(skeleton().size_12().rounded_full())
    .child(
        view().gap_2()
            .child(skeleton().h_4().w(px(250.0)))
            .child(skeleton().h_4().w(px(200.0))),
    )
```

In Rust `skeleton()` implements `Styled`: size and corners chain on it and win over the default corners, as TSX's
`class` does.

## NSH gaps

None. `animate-pulse` and `motion-reduce:` are classes NSH supports (`docs/dev/styles.md`). To verify in the
implementation: whether the platforms stop endless animations in a hidden window or off screen. If they do not, that is
an NSH performance issue, not worked around here.

## Decisions

None.
