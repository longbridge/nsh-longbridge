# Badge

小标记：一个状态、一个类别、一个数。对照 shadcn/ui new-york v4 `badge.tsx`。

## 用途

- 列表行、卡片标题旁的状态（"已成交"、"草稿"）、类别、版本号、计数。
- 搜：徽标、状态标记、小标签、计数、胶囊标签；badge, status badge, pill, label, count。

何时不用：

- 能点、能删、能选中的标签（筛选条件、输入的关键词）：Badge 不可交互，那是以后的 Tag / Toggle，现在用 `Button`
  的 `size="xs"`。
- 快捷键字样用 `Kbd`。
- 只有一个圆点（在线、未读）：用一个 `view` 的圆点，不用空 Badge。
- 涨跌、盈亏这类行情颜色：业务组件，不进本库；不要为它给 Badge 加变体（用作用域 token 或 `class`）。

## 基于

`nsh/base` 没有对应组件，本库用 `view` + `text` 画。没有行为：不可聚焦、不可点。

shadcn 的 `asChild`（把 Badge 渲染成 `<a>`，于是有 `[a&]:hover` 的悬停色和焦点环）本库**不做**：NSH 没有 `asChild`
这种把样式套到别的元素上的机制，要可点的就不该是 Badge。所以 shadcn 只对链接生效的悬停、焦点外观本库都没有，
只为链接存在的 `ghost`、`link` 两个变体也不加（见「变体」）。

## API

| TSX | Rust | 类型 | 默认 | 说明 |
|---|---|---|---|---|
| `variant` | `variant` | `"default"` \| `"secondary"` \| `"destructive"` \| `"outline"`（`BadgeVariant`） | `"default"` | 配色 |
| `children` | `.child(..)` | 文字、图标 | | 内容。文字可以是字符串；图标是本库的 `Icon`，放在字前或字后都行 |
| `class` / `style` | `class` / `style` | | | 根 |
| `labelClass` / `labelStyle` | `label_class` / `label_style` | | | 字符串 children 生成的那段文字 |

没有事件。

children 是字符串时，Badge 自己包一个 `text`（单行、不折、超长截断），这段文字就是 `label` 部件；children 是元素时
原样放，`labelClass` 不起作用。

## 变体和尺寸

| `variant` | 底 | 字 | 边 | shadcn |
|---|---|---|---|---|
| `default` | `badge-default` | `badge-default-foreground` | 透明 | `bg-primary text-primary-foreground` |
| `secondary` | `badge-secondary` | `badge-secondary-foreground` | 透明 | `bg-secondary text-secondary-foreground` |
| `destructive` | `badge-destructive` | `badge-destructive-foreground` | 透明 | `bg-destructive text-white dark:bg-destructive/60` |
| `outline` | 透明 | `badge-outline-foreground` | `badge-outline-border` | `border-border text-foreground` |

- 不加 shadcn 的 `ghost`、`link`：它们和别的变体的差别只在 `[a&]:hover`（作为链接时的悬停），本库 Badge 不可交互，
  这两个变体剩下的就是"透明底 + 正文色"和"透明底 + `primary` 字"，用 `outline` + `class="border-transparent"`、
  或 `class` 改字色就能得到，不值得各占一个变体。
- `success` / `warning` / `info` 不加：shadcn 没有。本库语义 token 里有 `success` 等，需要时用作用域 token 或
  `class="bg-success text-success-foreground"`。评审已定：不加，P2 做 Alert 时再一起看状态色。
- **无 `size`**（`xs` / `sm` / `md` / `lg` 都不支持）：shadcn 只有一档。它是标在内容旁的字样，不和控件对齐高度。

## 外观

shadcn：`inline-flex w-fit shrink-0 items-center justify-center gap-1 overflow-hidden rounded-full border
border-transparent px-2 py-0.5 text-xs font-medium whitespace-nowrap`，图标 `size-3`。

| | 值 |
|---|---|
| 布局 | 横排、竖向居中、水平居中、宽随内容（`self-start`，不被父的 stretch 拉宽）、`shrink-0` |
| 边 | `border-width`（1），颜色按变体（非 outline 透明：保留这 1 的位置，四个变体一样高） |
| 内边距 | 左右 `badge-padding-x`（8），上下 `badge-padding-y`（2） |
| 圆角 | `badge-radius` |
| 字 | `badge-font-size`（12）、`font-weight-medium`、行高由 foundation 的「字号 → 行高」给（12 → 16，同 Tailwind `text-xs`）、单行不折 |
| 图标 | 12：Badge 给图标的上下文尺寸是 `xs`（foundation 的图标边长随 `Size`），颜色跟随文字；图标与字间隔 `badge-gap`（4） |
| 总高 | 12 号字行高 16 + 上下 2 + 边 2 = 22，和 shadcn 一致 |

超长文字：截断加省略号（`numberOfLines={1}`），shadcn 是 `overflow-hidden whitespace-nowrap` 直接裁掉；截断加省略号
在桌面更常见，读屏仍念全文。

## 状态

无 hover / active / focus-visible / disabled：不可交互。

shadcn 的 `aria-invalid:border-destructive aria-invalid:ring-destructive/20` 是给链接 Badge 在表单里用的，本库不做。

## 指针与触屏

外观没有差别，不补命中区（不可点）。

## 键盘与读屏

- 不是 Tab 站。
- 读屏念它的文字（`text`）。只有图标、没有字的 Badge，应用要给 `accessibilityLabel`。

## Token

| token | 类型 | 推导默认 | 亮 / 暗 |
|---|---|---|---|
| `badge-default` | 颜色 | `primary` | 跟着 `primary` |
| `badge-default-foreground` | 颜色 | `primary-foreground` | 同 |
| `badge-secondary` | 颜色 | `secondary` | 同 |
| `badge-secondary-foreground` | 颜色 | `secondary-foreground` | 同 |
| `badge-destructive` | 颜色 | `light_dark(destructive, destructive 的 60%)` | shadcn `dark:bg-destructive/60` |
| `badge-destructive-foreground` | 颜色 | `destructive-foreground` | shadcn 写死 `text-white`；本库用补的语义 token，默认白 |
| `badge-outline-border` | 颜色 | `border` | 同 |
| `badge-outline-foreground` | 颜色 | `foreground` | 同 |
| `badge-radius` | 长度 | 9999（全圆，shadcn `rounded-full`） | 同 |
| `badge-padding-x` | 长度 | `spacing` × 2（8） | 同 |
| `badge-padding-y` | 长度 | `spacing` × 0.5（2） | 同 |
| `badge-gap` | 长度 | `spacing`（4） | 同 |
| `badge-font-size` | 长度 | `font-size-xs`（12） | 同 |

`badge-radius` 不引用 `radius-*`：shadcn v4 的 Badge 是胶囊（`rounded-full`），不随 `radius` 种子变。应用想要
随种子变的方角标签，主题里写 `"badge-radius": "var(--radius-sm)"`。注意 `radius: 0` 整套变直角时 Badge 仍是胶囊，
这和 shadcn 的行为一致。

## 示例

```tsx
import { Badge, Icon } from "nsh-longbridge";

<View class="flex-row gap-2">
  <Badge>Badge</Badge>
  <Badge variant="secondary">Secondary</Badge>
  <Badge variant="destructive">Destructive</Badge>
  <Badge variant="outline">Outline</Badge>
  <Badge variant="secondary">
    <Icon name="badge-check" />
    Verified
  </Badge>
  <Badge class="min-w-5 px-1 tabular-nums">8</Badge>
</View>
```

```rust
use nsh_longbridge::{badge, icon, BadgeVariant};

view().flex_row().gap_2()
    .child(badge().child("Badge"))
    .child(badge().variant(BadgeVariant::Secondary).child("Secondary"))
    .child(badge().variant(BadgeVariant::Destructive).child("Destructive"))
    .child(badge().variant(BadgeVariant::Outline).child("Outline"))
    .child(
        badge().variant(BadgeVariant::Secondary)
            .child(icon("badge-check"))
            .child("Verified"),
    )
```

计数 Badge（`min-w-5 px-1 tabular-nums`）是 shadcn 文档里的写法，用 `class`，不另加变体。

## 实现备注

- Badge 里的图标边长：Badge 用 foundation 的图标尺寸上下文给 `xs`，本库的 `Icon` 没给尺寸时取 12（相当于 shadcn 的
  `[&>svg]:size-3`）。不另起 `badge-icon-size` token。
- 字符串 children 的识别：TSX 的字符串 children 到 Rust 是文字节点；Rust 的 `.child("…")` 是 `&str`。两条路都要
  包成 `label` 部件。

## NSH 缺口

- **没有 `asChild` / 按标签名匹配的选择器**（shadcn 的 `[a&]:hover`）。绕过：本库 Badge 不可交互，不做链接 Badge。

## 已定（评审结论）

1. 不加 `href`（链接 Badge）。
2. 不加 `success` / `warning` / `info` 变体，P2 做 Alert 时一起看。
