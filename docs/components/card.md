# Card

卡片：一块有边、有底、有轻投影的容器，里面按 头 / 内容 / 脚 排。对照 shadcn/ui new-york v4 `card.tsx`。

## 用途

- 设置分组、表单块、一个对象的摘要（一个账户、一个订单）、仪表盘上的一格。
- 搜：卡片、面板、信息卡、内容块、分组框；card, panel, surface, tile, group box。

何时不用：

- 只要一块有边框的区域、没有标题结构：直接给 `view` 写 `class="rounded-xl border bg-card"`，不用 Card。
- 提示、警告、错误信息块：以后的 `Alert`（P2 之后），不用 Card 改颜色。
- 整张卡可点、可选中：shadcn 的 Card 不可交互，本库也不做（评审已定）。可点的东西放进 `CardAction` 或
  `CardFooter` 里的 `Button`。
- 弹出来的面板：`Popover` / `Dialog`（P2）。

## 基于

`nsh/base` 没有对应组件，本库用 `view` 和 `text` 拼。没有行为：不可聚焦、不接按压；里面的控件照常工作。

照约定"组合优于开关"，结构由子组件拼，和 shadcn 一一对应：

```
Card
├─ CardHeader
│  ├─ CardTitle
│  ├─ CardDescription
│  └─ CardAction        （可选，占右上角，跨标题和说明两行）
├─ CardContent
└─ CardFooter
```

每个部件是一个组件，各自接 `class` / `style`，所以 Card 本身不开 `headerClass` 这类部件属性（约定里的
`<部件>Class` 是给"不是单独组件"的部件用的）。

## API

| 组件 | TSX | Rust | 接受 | 说明 |
|---|---|---|---|---|
| `Card` | `<Card>` | `card()` | `children`、`class`、`style` | 根 |
| `CardHeader` | `<CardHeader>` | `card_header()` | 同上 | 标题区 |
| `CardTitle` | `<CardTitle>` | `card_title()` | 同上 | 标题；字符串 children 包成一段文字 |
| `CardDescription` | `<CardDescription>` | `card_description()` | 同上 | 说明；字符串 children 包成一段文字 |
| `CardAction` | `<CardAction>` | `card_action()` | 同上 | 右上角的操作（按钮、菜单触发器）；只能放在 `CardHeader` 里 |
| `CardContent` | `<CardContent>` | `card_content()` | 同上 | 主体 |
| `CardFooter` | `<CardFooter>` | `card_footer()` | 同上 | 底部一行（按钮、说明） |

没有事件，没有受控值。部件都可以省；顺序就是 children 的顺序。

## 变体和尺寸

- 无 `variant`：shadcn 只有一种。灰底卡、无投影卡用 `class` 或作用域 token（`card-shadow: 0`）。
- 无 `size`（`xs` / `sm` / `md` / `lg` 都不支持）：卡片的内边距是 token，整体紧凑改 `card-padding` / `card-gap`
  或 `spacing` 种子。

## 外观

| 部件 | shadcn 类 | 本库 |
|---|---|---|
| `Card` | `flex flex-col gap-6 rounded-xl border bg-card py-6 text-card-foreground shadow-sm` | 竖排；部件间隔 `card-gap`；上下内边距 `card-padding`；圆角 `card-radius`；边 `border-width` × `card-border`；底 `card`；字色 `card-foreground`（里面的文字继承）；投影档 `card-shadow` |
| `CardHeader` | `grid auto-rows-min grid-rows-[auto_auto] items-start gap-2 px-6`，有 action 时 `grid-cols-[1fr_auto]`；`[.border-b]:pb-6` | grid：行 `auto auto`，`items-start`，间隔 `card-header-gap`；左右 `card-padding`。有 `CardAction` 时列 `1fr auto`（见下「实现」） |
| `CardTitle` | `leading-none font-semibold` | 字号 `card-title-font-size`、行高 1（shadcn 的 `leading-none`，明确不走 foundation 的字号 → 行高）、`font-weight-semibold` |
| `CardDescription` | `text-sm text-muted-foreground` | 字号 `card-description-font-size`、行高由 foundation 的「字号 → 行高」给（14 → 20，同 `text-sm`）、色 `card-description-foreground` |
| `CardAction` | `col-start-2 row-span-2 row-start-1 self-start justify-self-end` | 同 |
| `CardContent` | `px-6` | 左右 `card-padding` |
| `CardFooter` | `flex items-center px-6 [.border-t]:pt-6` | 横排、竖向居中；左右 `card-padding` |

**带分隔线的头 / 脚**：shadcn 的 `[.border-b]:pb-6` 是"应用给头加了 `border-b` 时自动补下内边距"。NSH 不能按"有没有
某个类"匹配，所以由应用连着写：`<CardHeader class="border-b pb-6">`、`<CardFooter class="border-t pt-6">`。
文档、展示台都这样写，示例见下。

和 shadcn 的差别：没有。桌面密度上 24 的内边距偏松，但约定是外观照 shadcn；本库的紧凑预设主题里把 `card-padding`、
`card-gap` 调到 16（P0 的 `themes/` 预设，不在本单定）。

## 状态

无 hover / active / focus / disabled：Card 不可交互。里面的控件各有各的状态。

## 指针与触屏

外观没有差别。触屏上卡片里的按钮自己按 `control-height-touch-*` 变大，卡片不另管。

## 键盘与读屏

- Card 和它的部件都不是 Tab 站；Tab 在卡内的控件之间照常走。
- `CardTitle` 的读屏角色是 `header`（标题），读屏可以按标题跳到每张卡。shadcn 的 `CardTitle` 是 `div`，没有
  标题语义；本库加上，因为 NSH 的读屏导航靠 `header` 角色，没有它卡片在读屏里是一堆散字。
- 其余部件没有角色（`none`），文字照常念。

## Token

| token | 类型 | 推导默认 | 亮 / 暗 |
|---|---|---|---|
| `card` | 颜色 | 语义 `card`（同名同义，照 tokens.md 直接用语义 token，见下） | 跟着 `card` |
| `card-foreground` | 颜色 | 语义 `card-foreground` | 同 |
| `card-border` | 颜色 | `border` | 同 |
| `card-description-foreground` | 颜色 | `muted-foreground` | 同 |
| `card-radius` | 长度 | `radius-xl`（14） | 同 |
| `card-shadow` | 数字 | 2（`sm`，同 shadcn `shadow-sm`；0 无、1 xs、2 sm、3 md、4 lg） | 投影色 `shadow` 亮暗各一 |
| `card-padding` | 长度 | `spacing` × 6（24） | 同 |
| `card-gap` | 长度 | `spacing` × 6（24） | 同 |
| `card-header-gap` | 长度 | `spacing` × 2（8） | 同 |
| `card-title-font-size` | 长度 | 16（Tailwind `text-base`） | 同 |
| `card-description-font-size` | 长度 | 14（Tailwind `text-sm`） | 同 |

- `card` / `card-foreground` 本来就是 shadcn 的语义 token，组件就直接用它们，**不另声明**同名组件 token
  （在 `tokens!` 里不出现这两行，表里列出来只是告诉读者底色从哪来）。想只改卡片的底，就是改语义 `card`，
  这本来就只影响卡片类的表面。
- `card-title-font-size` 用 Tailwind 的固定字号，不引用 `font-size-*`：tokens.md 规定正文和标题用 Tailwind 的值，
  `font-size-*` 是控件字号。

## 实现

- **CardHeader 的两列**：shadcn 用 `has-data-[slot=card-action]:grid-cols-[1fr_auto]`。NSH 没有 `:has()`，本库用上下文：
  `CardHeader` 提供一个"有 action"的信号，`CardAction` 挂上时置真、卸下时（`cleanup`）置假；`CardHeader` 的列定义
  绑这个信号。不总是用 `1fr auto`：没有 action 时第二列虽然宽 0，列间隔 `gap-2` 仍在，标题会被白白挤窄 8。
- `CardAction` 放在 `CardHeader` 之外（读不到上下文）：照常渲染，开发构建打一条警告。

## 示例

```tsx
import { Button, Card, CardAction, CardContent, CardDescription, CardFooter, CardHeader, CardTitle } from "nsh-longbridge";

<Card class="w-[360px]">
  <CardHeader>
    <CardTitle>Login to your account</CardTitle>
    <CardDescription>Enter your email below to login to your account</CardDescription>
    <CardAction>
      <Button variant="link">Sign Up</Button>
    </CardAction>
  </CardHeader>
  <CardContent>
    {/* fields */}
  </CardContent>
  <CardFooter class="border-t pt-6 gap-2">
    <Button class="flex-1">Login</Button>
  </CardFooter>
</Card>
```

```rust
use nsh_longbridge::{button, card, card_action, card_content, card_description, card_footer, card_header, card_title, ButtonVariant};

card().w(px(360.0))
    .child(
        card_header()
            .child(card_title().child("Login to your account"))
            .child(card_description().child("Enter your email below to login to your account"))
            .child(card_action().child(button().variant(ButtonVariant::Link).child("Sign Up"))),
    )
    .child(card_content().child(/* fields */))
    .child(card_footer().class("border-t pt-6 gap-2").child(button().flex_1().child("Login")))
```

## NSH 缺口

- **没有 `:has()`、没有"按自己有没有某个类"匹配**（shadcn 的 `has-data-[slot=card-action]:`、`[.border-b]:`）。
  绕过：action 用上下文登记；分隔线的补白由应用连着写（见上）。
- **文字颜色继承**：Card 根上的 `card-foreground` 要靠 NSH 的文字颜色继承传到 children 里应用自己的 `text`。实现时
  确认 NSH 的 `text` 不写颜色时继承父 `view` 的字色（`docs/dev/styles.md` 里文字样式按 CSS 继承）；如果不继承，
  `CardTitle` / `CardDescription` 自己设色不受影响，应用自己的 `text` 要自己写 `text-card-foreground`，记为缺口。

## 已定（评审结论）

1. 不加可点 / 可选中的整卡。
2. `CardTitle` 加 `header` 读屏角色。
3. `card-shadow` = 2（`sm`）。
