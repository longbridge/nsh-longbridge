# Spinner

转圈的加载指示：一件事正在进行、还不知道要多久。对照 shadcn/ui new-york v4 `spinner.tsx`。

## 用途

- 一小块区域、一行、一个面板正在加载或提交。
- 搜：转圈、加载中、菊花、加载指示器；spinner, loading indicator, activity indicator, loader。

何时不用：

- 按钮在提交：用 `Button` 的 `loading`（它内部就是这个 Spinner，还管着不可点）。
- 整块内容的首次加载、知道内容的形状：用 `Skeleton`。
- 知道进度（百分比）：用进度条（`nsh/base` 的 `Progress`，本库以后做 `Progress`），不用 Spinner。

## 基于

`nsh/base` 没有对应组件，本库用 foundation 的图标画。图标是 lucide 的 `loader-circle`（shadcn 用的 `Loader2Icon`
就是它），用 Tailwind 的 `animate-spin` 转（NSH 支持）。没有行为。

## API

| TSX | Rust | 类型 | 默认 | 说明 |
|---|---|---|---|---|
| `size` | `size` | `Size`：`"xs"` \| `"sm"` \| `"md"` \| `"lg"` | 从上下文取，没有是 `"md"` | 图标边长，见下 |
| `accessibilityLabel` | `accessibility_label` | `string` | `"Loading"`（经 `labels::set_translator` 翻译） | 读屏名 |
| `class` / `style` | `class` / `style` | | | 根，即图标；`class="size-8"` 可以超出四档 |

没有事件，没有 children，没有部件。

## 变体和尺寸

- 无 `variant`。颜色跟字色（见下）。
- `size`：

| `size` | 边长 | shadcn 对照 |
|---|---|---|
| `xs` | 12 | `size-3`，xs 按钮里的图标 |
| `sm` | 14 | `size-3.5` |
| `md` | 16 | `size-4`，shadcn 的默认 |
| `lg` | 20 | `size-5` |

边长就是 foundation 的图标边长（随 `Size`），Spinner 不另起 token。没给 `size` 时和图标一样从上下文取默认，
所以放进 `size="sm"` 的按钮里自动是 14。

shadcn 只有 `size-4` 一档，其余靠 `className`。本库按约定给 `size` 四档，这样 Spinner 放进同 `size` 的按钮、输入框时
和图标一样大。更大的（页面正中的 32）照 shadcn 用 `class="size-8"`。触屏不变大：它不是控件。

## 外观

shadcn：`size-4 animate-spin`，颜色 `currentColor`（跟所在文字）。

| | 值 |
|---|---|
| 形状 | `loader-circle` 图标，描边宽沿用图标集的（lucide 2） |
| 颜色 | 跟随所在文字（foundation 图标的规则，同 shadcn 的 `currentColor`）：放在按钮里就是按钮的字色，放在 `text-muted-foreground` 的行里就是弱色。要别的颜色给 `class="text-…"` |
| 动画 | `animate-spin`：1 s 一圈，线性，无限 |

颜色不设 token：跟字色走才能在按钮各变体里自动对；要统一改颜色的，改所在位置的字色。

## 状态

- **减弱动态效果**：照转（已定）。转圈本身就是"还在进行"的信息，停住的图标像卡死；macOS、iOS 系统的转圈在
  减弱动态效果下也照转，WCAG 2.3.3 也豁免表达必要信息的动画。这是 conventions.md "减弱动态效果时过渡归零"的例外：
  它不是过渡。
- 无 hover / active / focus / disabled。

## 指针与触屏

没有差别。

## 键盘与读屏

- 不是 Tab 站，不接指针。
- 读屏：角色 `progressbar`（不定进度），读屏名 `accessibilityLabel`（默认 "Loading"），状态 `busy`。
  shadcn 用的是 `role="status"`；NSH 没有 `status` 角色，不定进度的 `progressbar` 也是 macOS / iOS 系统转圈
  （`NSProgressIndicator`、`UIActivityIndicatorView`）给读屏的角色，所以取它。
- 放进 `Button` 的 `loading` 时由 Button 去掉 Spinner 自己的读屏节点，只在按钮上报 busy，避免念两遍
  （P0 Button 的事，这里记一笔）。

## Token

无组件 token：边长用 foundation 的图标边长（随 `Size`，12 / 14 / 16 / 20），颜色跟随文字，动画是 Tailwind 的
固定值（NSH 的 token 存不了曲线）。要统一改大小或颜色，改所在位置的 `size` 或字色。

## 示例

```tsx
import { Spinner, Button } from "nsh-longbridge";

<View class="flex-row items-center gap-2">
  <Spinner size="sm" />
  <Text class="text-sm text-muted-foreground">Syncing…</Text>
</View>

<Spinner class="size-8 text-primary" accessibilityLabel="Loading orders" />
```

```rust
use nsh_longbridge::{spinner, Size};

view().flex_row().items_center().gap_2()
    .child(spinner().size(Size::Sm))
    .child(text("Syncing…").class("text-sm text-muted-foreground"))

spinner().class("size-8 text-primary").accessibility_label("Loading orders")
```

## NSH 缺口

- **没有 `status` 读屏角色**。绕过：用不定进度的 `progressbar` + `busy`（见上）。

## 已定

- 减弱动态效果时照转（评审结论）。
