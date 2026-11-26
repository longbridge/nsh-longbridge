# Label

> 状态：API 单（P1，已评审，按 2026-11-26 结论修订）。表单里和 [Input](input.md) / [Textarea](textarea.md) 一起用；
> Label 不画框，只跟随字段的 disabled。`label-*` token 也被 Checkbox / Switch / RadioGroup 的 `label` 部件读（见 §7）。

## 1. 用途

控件旁边的标签文字：表单字段的名字、Checkbox / Switch 旁边的说明。外观是 shadcn/ui new-york v4 的 `Label`
（14 号、中等字重、行高 1、可以和图标或控件横排）。按下 Label 会把焦点交给它标的输入框（`htmlFor`）。

何时不用：

| 需要 | 改用 |
|---|---|
| Checkbox、Switch、RadioGroup 项旁边的文字 | 直接写成它们的 children（本库这几个组件的 `label` 部件，见各自的 API 单）：按文字也切换、读屏名也对；不要在旁边另放一个 Label |
| 字段的说明、错误提示、必填标记、字数 | Form / Field（P3：FieldLabel、FieldDescription、FieldError）；P1 先用 `Text` |
| 普通正文、标题、分组标题 | `Text` 加 Tailwind 类（`text-sm`、`font-semibold`…） |
| 状态、计数这种小块 | Badge |

## 2. 基于什么

没有对应的 `nsh/base` 组件，也没有宿主元素：Label 就是一个横排的 `view` 加上应用的 children，本库自己写。
shadcn 背后是 Radix `Label.Root`（一个 `<label>`）：按它就聚焦 `for` 指向的控件、双击不选中文字。本库只做这两件事：

- `htmlFor` 指着一个输入框时，按下 Label 调它的 `focus()`；`disabled` 时不调。
- 文字不可选中（`select-none`），双击不会选中标签文字。

不做的：

- **不做读屏关联。** NSH 没有 labelledby（[Input §9 G1](input.md#缺口)），Label 不能成为输入框的读屏名。Label 照常作为普通文字被读到，
  输入框的名字由输入框的 `accessibilityLabel` 给。Label 也不对读屏隐藏（评审已定，见 §9）。
- 不是 Tab 停靠点，不是按钮：没有 `button` 角色，键盘到不了它（它要聚焦的输入框自己在 Tab 顺序里）。

## 3. API

### Props

| TSX | Rust | 类型 | 默认 | 说明 |
|---|---|---|---|---|
| `children` | `.child(..)` / `.children(..)` | 元素 / 文字 | — | 标签内容：文字，可以带图标、Badge（`gap` 隔开） |
| `htmlFor` | `html_for` | TSX：`InputRef \| TextareaRef`（同一个 ref 变量）；Rust：`ComponentRef` | — | 按下时聚焦的目标。名字照 shadcn / React 的 `htmlFor`。实现依赖 G2 |
| `disabled` | `disabled` | `boolean` | `false` | 变暗（`opacity-disabled`），按下无效。shadcn 靠 `peer-disabled` / `group-data-[disabled]` 自动跟随，NSH 没有 `peer-*`，所以要应用显式传，见 G3 |
| `class` | `class` | `string` | — | 根的类，最后合并 |
| `style` | `style` | `StyleProp<ViewStyle>` / `Style` | — | 根的样式；文字的 `color`、`fontSize` 写在这里会传给里面的文字 |

没有 `size`、`variant`、`required`、`invalid`：shadcn 的 Label 都没有。必填星号、出错变红属于 Field（P3，shadcn 的 `FieldLabel`
是 `data-[invalid=true]:text-destructive`）。

### 事件

无。（按下聚焦是 `htmlFor` 的内建行为，不报事件；要别的行为的应用用自己的 `View` + `onPress`。）

### 部件

只有根（`view`，横排）。文字样式（字号、字重、颜色、行高）设在根上，由里面的 `Text` 继承，所以 `class` / `style` 改根就改了文字，
不另设 `textClass`。

## 4. 变体与尺寸

无变体。**不支持 `size`**：shadcn 的 Label 只有 `text-sm` 一档，各种尺寸的字段上面都是它；小字段旁边要小标签的用 `class="text-xs"`。

## 5. 状态

| 状态 | 外观 | shadcn 原类 |
|---|---|---|
| 常态 | 字号 `label-font-size`（14），字重 `label-font-weight`（500），颜色 `label-foreground`（`foreground`），行高 = 字号（`leading-none`），横排居中、子项间距 `label-gap`（8），文字不可选 | `flex items-center gap-2 text-sm leading-none font-medium select-none` |
| hover | 不变；光标箭头（不是手型：它不是链接，gpui-kit Design Guides） | 无 |
| active（按下） | 不变，不做按下反馈（它不是按钮）；效果是目标输入框得到焦点、出现输入框的焦点环 | 无 |
| focus-visible | 不适用：Label 不接焦点 | — |
| disabled | 整体 `opacity-disabled`（0.5）；按下不聚焦 | `peer-disabled:opacity-50 group-data-[disabled=true]:opacity-50 group-data-[disabled=true]:pointer-events-none` |
| invalid / loading | 不适用（见 §3） | — |

`peer-disabled:cursor-not-allowed` 不照搬：桌面上禁用控件的标签保持箭头光标（同 Input 的 disabled）。

行高 1 对中文要验收：`leading-none` 下中日韩字形和带下行部的拉丁字母（g、y）在某些平台可能被裁。展示台放一个中文标签和一个 `gy` 标签，
截图看亮暗两档；被裁的话改用 foundation「字号 → 行高」函数给的行高，并在这里写明偏离 shadcn。
（行高 = 字号是 shadcn 的 `leading-none`，是 Label 有意为之，所以不走那个函数；这是全库「组件不各写行高」规则下唯一的例外。）

## 6. 指针与触屏

| | 指针 | 触屏 |
|---|---|---|
| 字号 | 14 | 14（不跟输入框的触屏 16 变：shadcn 的 Label 在窄屏也是 `text-sm`） |
| 命中区 | 标签自己的框 | 标签自己的框，**不补到 44**：它不是独立控件，旁边的输入框已经满足命中区；补了反而会和上下相邻的字段抢点击 |
| 按下 | 一次完整的点击（抬起时）聚焦目标，同浏览器 `<label>` 的 click | 同左（轻点抬起时），弹出软键盘；滑动滚动不触发 |

## 7. 组件 token

| token | 类型 | 推导默认值 | 亮 / 暗 |
|---|---|---|---|
| `label-font-size` | 长度 | `font-size-md`（14） | — |
| `label-font-weight` | 数字 | `font-weight-medium`（500） | — |
| `label-foreground` | 颜色 | `foreground` | 同 `foreground` |
| `label-gap` | 长度 | `spacing` × 2（8） | — |

`label-font-size` 引用 `font-size-md` 而不是固定的 Tailwind `text-sm`（14）：这样主题把 `font-size` 种子调成 13 做紧凑密度时，标签和字段一起变小。
默认值都是 14，所以和 shadcn 没有可见差别（评审已定）。

勾选类控件（Checkbox、Switch、RadioGroup 的项）的 `label` 部件读 `label-foreground` 和 `label-font-weight`，字号用它们自己的
`font-size-<size>`，不读 `label-font-size`（它们有 `size`，Label 没有）。所以改 `label-foreground` 会同时改这些控件的标签文字。

disabled 的透明度用语义 token `opacity-disabled`，不另设。

```rust
tokens! {
    /// The label's text size.
    LABEL_FONT_SIZE: Length = "label-font-size" => |t| t.length(FONT_SIZE_MD),
    /// The label's text weight.
    LABEL_FONT_WEIGHT: Number = "label-font-weight" => |t| t.number(FONT_WEIGHT_MEDIUM),
    /// The label's text color.
    LABEL_FOREGROUND: Color = "label-foreground" => |t| t.color(FOREGROUND),
    /// The space between the label's children (an icon and its text).
    LABEL_GAP: Length = "label-gap" => |t| t.length(SPACING) * 2.0,
}
```

## 8. 示例

TSX：

```tsx
import { Text, View } from "nsh";
import { Badge, Checkbox, Input, type InputRef, Label } from "nsh-longbridge";

let name: InputRef | undefined;

<View class="gap-2">
  <Label htmlFor={name}>Display name</Label>
  <Input ref={name} placeholder="Ada" accessibilityLabel="Display name" />
</View>

// 带别的元素（Badge 的 API 以它自己的 API 单为准）
<Label htmlFor={name}>
  API key <Badge variant="secondary">Beta</Badge>
</Label>

// 跟随字段的禁用
<Label htmlFor={name} disabled={locked()}>Display name</Label>
<Input ref={name} disabled={locked()} accessibilityLabel="Display name" />

// Checkbox 旁边不用 Label，文字写进 Checkbox 的 children
<Checkbox checked={terms()} onChange={({ checked }) => terms.set(checked)}>
  <Text>Accept terms and conditions</Text>
</Checkbox>
```

Rust：

```rust
use nsh::prelude::*;
use nsh_longbridge::{input, label};

fn display_name(locked: Signal<bool>) -> impl IntoElement {
    let name = ComponentRef::new();
    view()
        .gap_2()
        .child(label().html_for(name).disabled(locked).child("Display name"))
        .child(
            input()
                .component_ref(name)
                .placeholder("Ada")
                .disabled(locked)
                .accessibility_label("Display name"),
        )
}
```

（Checkbox 的写法照 [checkbox.md](checkbox.md)：标签是它的 children，事件照 NSH 叫 `onChange({ checked })`。）

## 9. NSH 能力缺口与已定的取舍

### 缺口

- **G1 没有 labelledby**：见 [Input §9](input.md#缺口)。Label 对读屏只是文字。
- **G2 组件 prop 能不能收另一个组件的 ref。** `nsh/base` 里收 ref 的 prop（`Scrollbar` 的 `scroll`）收的是宿主节点的 ref
  （JS 的 `NodeRef`、Rust 的 `ElementRef`）；`htmlFor` 要收的是本库 `Input` / `Textarea` 这个 Rust 组件的 ref（它的方法对象）。
  NSH 的 props 能否收组件 ref、收到后能否调它的 `focus()`，要在实现时确认。
  绕过（按优先顺序）：① Input / Textarea 的 ref 对象除了方法，再带上里面那个 `input()` 节点，`htmlFor` 按 `NodeRef` 收，
  Label 直接对节点 `focus()`（节点的 `focus()` 是宿主就有的）；② 都不行时 Label 改为报 `onPress`，由应用自己 `ref.focus()`——这会让
  API 和 shadcn 不一样，要回到评审。
- **G3 没有 `peer-*`。** NSH 明确不支持 `peer-*` 变体，Label 没法从旁边的输入框自动得知 disabled。绕过：`disabled` 显式传；
  以后 Field（P3）把 disabled 同时下发给标签和控件，应用就不用写两遍（shadcn 的 `group-data-[disabled=true]` 就是这个用法）。

### 已定（2026-11-26 评审）

- Label 不对读屏隐藏：读屏会先念一遍标签文字、再念「Display name，文本框」，换来应用漏写 `accessibilityLabel` 时字段仍有名字可听。
- `label-font-size` 引用 `font-size-md`，随密度种子变。
- 勾选类控件的标签部件读 `label-foreground`、`label-font-weight`（§7）。
