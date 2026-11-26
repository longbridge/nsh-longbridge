# Kbd

按键字样：把一个键或一组组合键画成小键帽。对照 shadcn/ui new-york v4 `kbd.tsx`（`Kbd`、`KbdGroup`）。

## 用途

- 菜单项、按钮、提示（`Tooltip`）里标快捷键；说明文字里提到一个键。
- 搜：快捷键、按键提示、键帽、组合键；kbd, keyboard shortcut, hotkey, key hint。

何时不用：

- 只读的状态小标记用 `Badge`。
- 不是按键的等宽代码片段用应用自己的 `text`（`font-mono`），不用 `Kbd`。
- 屏幕上可以按的虚拟键盘按键是按钮，用 `Button`。

## 基于

`nsh/base` 没有对应组件，本库用 `view` + `text` 画。没有行为：不可聚焦、不接指针（shadcn 的 `pointer-events-none`）、
文字不可选（`select-none`）。

键名怎么写是应用的事：Kbd 不解析 `"mod+k"`。要按平台显示（Apple 平台 `⌘K`，其余 `Ctrl+K`）的，用 NSH 的
`keymap.label("mod+k")`（Rust `nsh_core::element::key_label("mod+k")`）算好再放进去，见示例。

## API

### `Kbd`

| TSX | Rust | 类型 | 默认 | 说明 |
|---|---|---|---|---|
| `children` | `.child(..)` | 文字或图标 | | 键名；可以是一个本库图标（`⌘`、方向键这类有图标的） |
| `class` / `style` | `class` / `style` | | | 根，即键帽 |

### `KbdGroup`

把几个 `Kbd` 排成一行（`Ctrl` `+` `B`），间隔 `kbd-group-gap`。

| TSX | Rust | 类型 | 说明 |
|---|---|---|---|
| `children` | `.child(..)` | `Kbd`、文字 | 键和键之间的 `+`、`then` 这类字是应用自己放的 `text` |
| `class` / `style` | `class` / `style` | | 根 |

两者都没有事件，没有别的部件（键帽就是根，字就是 children，不另开 `textClass`）。

## 变体和尺寸

- 无 `variant`。
- 无 `size`（`xs` / `sm` / `md` / `lg` 都不支持）：shadcn 只有一档（高 20），它跟着所在的行，不随控件尺寸变。
  放进 `size="xs"` 的按钮里嫌大，用 `class` 或作用域 token 改 `kbd-height`。

## 外观

shadcn：`inline-flex h-5 w-fit min-w-5 items-center justify-center gap-1 rounded-sm bg-muted px-1 font-sans text-xs
font-medium text-muted-foreground select-none`，图标 `size-3`。

| | 值 |
|---|---|
| 高 / 最小宽 | `kbd-height`（20），`min-w` 同高：单个字符是正方形 |
| 内边距 | 左右 `kbd-padding-x`（4），竖向居中 |
| 圆角 | `kbd-radius` |
| 底 / 字 | `kbd` / `kbd-foreground` |
| 字 | `kbd-font-size`（12）、`font-weight-medium`、`font-sans`，行高由 foundation 的「字号 → 行高」给，单行不折 |
| 图标 | 12：Kbd 给图标的上下文尺寸是 `xs`（foundation 的图标边长随 `Size`），颜色跟随文字；图标和字之间 `gap-1`（4） |

### 在 `Tooltip` 里

shadcn 用选择器 `[[data-slot=tooltip-content]_&]` 改成 `bg-background/20 text-background`（暗色 `bg-background/10`），
因为 Tooltip 的底是反色（`foreground`）。NSH 没有"祖先是谁"的选择器，本库用上下文：`Tooltip` 提供一个
"在提示里"的上下文，`Kbd` 读到它就改用 `kbd-tooltip` / `kbd-tooltip-foreground`。应用不用做任何事。

## 状态

无 hover / active / focus / disabled：不可交互。所在的菜单项或按钮禁用时，整行的 `opacity-disabled` 已经盖到它身上，
Kbd 自己不再变暗。

## 指针与触屏

外观没有差别。触屏上通常没有硬件键盘，快捷键提示本身意义不大：要不要显示是应用的事（例如只在
`platform.desktop` 时放 `Kbd`），组件不自动隐藏。

## 键盘与读屏

- 不是 Tab 站，不接指针。
- 读屏把它当普通文字念（`text`），不设 `keyboardkey` 角色：NSH 的 `keyboardkey` 是"像按键一样能按的元素"，
  这里只是字样。Kbd 通常放在已有读屏名的按钮、菜单项里，读屏名应由那一层写全。

## Token

| token | 类型 | 推导默认 | 亮 / 暗 |
|---|---|---|---|
| `kbd` | 颜色 | `muted` | 跟着 `muted` |
| `kbd-foreground` | 颜色 | `muted-foreground` | 跟着 `muted-foreground` |
| `kbd-tooltip` | 颜色 | `light_dark(tooltip-foreground 的 20%, tooltip-foreground 的 10%)` | shadcn `bg-background/20`、`dark:bg-background/10`，用 tooltip 的字色而不是 `background`，这样改了 tooltip 的配色键帽跟着走 |
| `kbd-tooltip-foreground` | 颜色 | `tooltip-foreground` | 同 |
| `kbd-radius` | 长度 | `radius-sm`（6） | 同 |
| `kbd-height` | 长度 | 20（`spacing` × 5） | 同 |
| `kbd-padding-x` | 长度 | `spacing`（4） | 同 |
| `kbd-font-size` | 长度 | `font-size-xs`（12） | 同 |
| `kbd-group-gap` | 长度 | `spacing`（4） | 同 |

`kbd-height` 不跟 `control-height` 走：它是标在控件里的小字样，shadcn 也是固定 `h-5`；整体紧凑时改 `spacing`
它会一起缩。

## 示例

```tsx
import { keymap } from "nsh";
import { Kbd, KbdGroup } from "nsh-longbridge";

<KbdGroup>
  <Kbd>{keymap.label("mod")}</Kbd>
  <Kbd>K</Kbd>
</KbdGroup>

<KbdGroup>
  <Kbd>Ctrl</Kbd>
  <Text class="text-xs text-muted-foreground">+</Text>
  <Kbd>B</Kbd>
</KbdGroup>

<Button tooltip={<Tooltip>Save <Kbd>{keymap.label("mod+s")}</Kbd></Tooltip>}>Save</Button>
```

```rust
use nsh_core::element::key_label;
use nsh_longbridge::{kbd, kbd_group};

kbd_group()
    .child(kbd().child(key_label("mod")))
    .child(kbd().child("K"))
```

## NSH 缺口

- **没有按祖先匹配的选择器**（shadcn 的 `[[data-slot=tooltip-content]_&]`）。绕过：`Tooltip` 提供上下文，`Kbd` 读。
  TSX 里应用作为 children 传进 `Tooltip` 的 `Kbd`，是在 `Tooltip` 放子元素的地方被采纳的，读得到这个上下文
  （`docs/dev/components.md` "The host node and ids"）。实现时用 headless 测试守住 TSX 和 Rust 两条路。

## 已定（评审结论）

- 不加 `keys="mod+k"`：组件不解析键名，应用用 `keymap.label(..)`。
