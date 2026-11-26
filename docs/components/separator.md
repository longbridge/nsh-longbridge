# Separator

一条把内容分开的细线，横的或竖的。对照 shadcn/ui new-york v4 `separator.tsx`。

## 用途

- 卡片、面板、表单里的段落之间（横线）；工具条、一行文字链接之间（竖线）。
- 搜：分隔线、分割线、横线、竖线；separator, divider, hr, rule。

何时不用：

- 菜单里的分隔用 P2 的 `DropdownMenu` 自带的分隔项（`nsh/base` 的 `MenuSeparator`），工具条里用 `nsh/base` 的
  `ToolbarSeparator`：它们在读屏、键盘移动里有自己的含义。
- 只是想给一块加个边：用那一块的 `class`（`border-b`），不要在它下面再放一根 `Separator`。
- 「线 · 字 · 线」的带字分隔（"或"）：shadcn 没有，本库不做；用两根 `Separator` 夹一个 `text` 自己排。

## 基于

`nsh/base` 没有 Separator（只有 `MenuSeparator`、`ToolbarSeparator`），本库用一个 `view` 画。没有行为：不可聚焦、
不接指针（`pointer-events-none`）。

读屏：一律当装饰，不进读屏树（shadcn 的默认 `decorative`）。NSH 的读屏角色里没有 `separator`，所以不提供
shadcn 的 `decorative={false}`，见文末「缺口」。

## API

| TSX | Rust | 类型 | 默认 | 说明 |
|---|---|---|---|---|
| `orientation` | `orientation` | `"horizontal"` \| `"vertical"`（`nsh_core::geometry::Axis`） | `"horizontal"` | 方向 |
| `class` / `style` | `class` / `style` | | | 根，就是线本身 |

没有事件，没有子元素，没有部件（`*Class` / `*Style`）。

## 变体和尺寸

- 无 `variant`。
- 无 `size`（`xs` / `sm` / `md` / `lg` 都不支持）：线的粗细是 token，长短由布局给。

## 外观

| | 横 | 竖 |
|---|---|---|
| shadcn | `shrink-0 bg-border h-px w-full` | `shrink-0 bg-border h-full w-px` |
| 本库 | `shrink-0`，高 `separator-thickness`，`self-stretch`（撑满父的宽） | `shrink-0`，宽 `separator-thickness`，`self-stretch`（撑满父的高） |
| 颜色 | `separator` | `separator` |

和 shadcn 的差别：用 `self-stretch` 而不是 `w-full` / `h-full`。竖线放在横排里时，shadcn 的 `h-full` 要父元素有确定的
高才生效（常见的坑是竖线高为 0，要应用补 `h-4`）；NSH 是 flex 布局，`self-stretch` 直接撑满这一行的高。要短一点的竖线
照 shadcn 的写法给 `class="h-4"`（`self-stretch` 被 `class` 里的高盖掉，因为 `class` 最后合并）——实现时要确认
显式高度生效时 `self-stretch` 不再拉伸（Taffy 的 `align-self: stretch` 遇到确定的高就不拉，和 CSS 一致）。

## 状态

无 hover / active / focus / disabled：不可交互。

## 指针与触屏

没有差别。触屏不放大，不补命中区（不可点）。

## 键盘与读屏

- 不是 Tab 站，不接指针。
- 不进读屏树（`accessible(false)` / TSX `accessible={false}`）。

## Token

| token | 类型 | 推导默认 | 亮 / 暗 |
|---|---|---|---|
| `separator` | 颜色 | `border` | 跟着 `border` 变（shadcn 暗色 `border` 是白 10%） |
| `separator-thickness` | 长度 | `border-width`（1） | 同 |

## 示例

```tsx
import { Separator } from "nsh-longbridge";

<View class="gap-4">
  <Text>Radix Primitives</Text>
  <Separator />
  <View class="flex-row h-5 items-center gap-4">
    <Text>Blog</Text>
    <Separator orientation="vertical" />
    <Text>Docs</Text>
  </View>
</View>
```

```rust
use nsh_core::geometry::Axis;
use nsh_longbridge::separator;

view().gap_4()
    .child(text("Radix Primitives"))
    .child(separator())
    .child(
        view().flex_row().h_5().items_center().gap_4()
            .child(text("Blog"))
            .child(separator().orientation(Axis::Vertical))
            .child(text("Docs")),
    )
```

## NSH 缺口

- **没有 `separator` 读屏角色**（`nsh_core::props` 的角色表里没有）。shadcn 的 `decorative={false}`（读屏念"分隔符"）
  做不到。绕过：本库只做装饰线，不提供 `decorative`。真需要读屏知道"这里分了段"的，用标题（`accessibilityRole="header"`）
  分段，这本来也是更好的做法。

## 需要拍板

无。
