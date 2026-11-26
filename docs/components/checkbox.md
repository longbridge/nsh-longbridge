# Checkbox

状态：API 单，评审已过（2026-11-26），待实现。外观照 shadcn/ui new-york v4 `checkbox.tsx`，行为包 `nsh/base` 的 `Checkbox`。

`Checkbox`、[`Switch`](switch.md)、[`RadioGroup`](radio-group.md) 是一组选择控件，焦点、禁用、invalid、标签、
命中区的处理三份单写法一致；改其中一份的这些规则，三份一起改。

## 1. 用途

勾选框：表单里的同意条款、多选列表的一项、「全选」行（半选）。值在提交或确认时才生效。

何时不用：

| 场景 | 改用 |
|---|---|
| 改了立即生效的设置（开 / 关） | `Switch` |
| 一组里必须且只能选一个 | `RadioGroup`；选项多时用 `Select`（P2） |
| 工具栏里的开关按钮、格式按钮 | `Toggle` / `ToggleGroup`（以后） |
| 菜单里的勾选项 | `DropdownMenu` 的勾选项（P2），不在菜单里放 `Checkbox` |
| 整张卡片可点的选项、带说明和错误行的表单项 | `Checkbox` 放进 `Field`（P3）；本组件不加卡片变体 |

## 2. 基于 `nsh/base` 的 `Checkbox`

根元素就是 `nsh/base` 的 `Checkbox`（`crates/nsh-base/src/checkbox.rs`），本库只给外观，下面这些它已经做了，本库不重写：

- 按压：点框、点标签（`children` 在按压目标里）、聚焦后 Space / Return，都报一次 `Change(next)`：未选中或半选时要
  `true`，选中时要 `false`。值由应用持有，组件只显示应用传回来的值。
- 焦点：可按的视图是键盘焦点的一站（macOS、Web）；`disabled` 时不接按压、无按下反馈。
- 读屏：角色 `checkbox`；状态 checked / unchecked / mixed（`indeterminate` 盖过 `checked`）；`disabled` 念「变暗」；
  `invalid` 每次走到都念「无效」；名字取 `children` 里的文字，没有文字时用 `accessibilityLabel`；
  `accessibilityHint`、`accessibilityDescribedBy`（错误或说明行，念完名字接着念）原样转给它。
- `hitSlop`：命中区外扩，不改布局和外观。
- 终端：`nsh/base` 用 `[x]` / `[ ]` / `[-]` 代替指示器，本库画的框和勾在终端不出现，标签照常。

本库加的：框、勾 / 横杠、标签的排版、各状态的颜色、尺寸、触屏命中区、按下反馈。

## 3. API

### Props

| TSX | Rust | 类型 | 默认 | 说明 |
|---|---|---|---|---|
| `checked` | `checked` | `boolean` / `Prop<bool>` | `false` | 受控值 |
| `indeterminate` | `indeterminate` | `boolean` / `Prop<bool>` | `false` | 半选，盖过 `checked`；按一下要的是 `true` |
| `disabled` | `disabled` | `boolean` / `Prop<bool>` | `false` | |
| `invalid` | `invalid` | `boolean` / `Prop<bool>` | `false` | 外观见「状态」，读屏念「无效」 |
| `size` | `size` | `Size` | `"md"` | `xs` / `sm` / `md` / `lg` |
| `accessibilityLabel` | `accessibility_label` | `string?` / `Prop<Option<SharedString>>` | — | 没有 `children` 文字时必填 |
| `accessibilityHint` | `accessibility_hint` | `string?` | — | 转给 `nsh/base` |
| `accessibilityDescribedBy` | `accessibility_described_by` | `ElementTarget?` | — | 错误 / 说明行，转给 `nsh/base` |
| `hitSlop` | `hit_slop` | `EdgeInsets?` | 自动（见 §6） | 给了就用给的，不再自动算 |
| `children` | `children` | 字符串或子元素 | — | 标签，放进 `label` 部件；字符串自动包成文字 |
| `class` / `style` | `class` / `style` | | | 根（整行：框 + 标签），最后合并 |
| `indicatorClass` / `indicatorStyle` | `indicator_class` / `indicator_style` | | | 框 |
| `markClass` / `markStyle` | `mark_class` / `mark_style` | | | 框里的勾 / 横杠图标 |
| `labelClass` / `labelStyle` | `label_class` / `label_style` | | | 包 `children` 的那层；没有 `children` 时不建 |

只受控，不提供 `defaultChecked`（约定：值由应用持有）。不转出 `nsh/base` 的 `role`、`indicator` 槽、`checkedStyle`、
`indeterminateStyle`：菜单勾选项走 `DropdownMenu`；勾的图形由本库画，颜色和大小用 `markClass` / `markStyle` 调，
状态的颜色用 §7 的 token。

### 事件

| TSX | Rust | 何时 |
|---|---|---|
| `onChange({ checked })` | `CheckboxEvent::Change(bool)`（`#[args(checked)]`），builder `.on_change(\|checked: &bool\| ..)` | `nsh/base` 报 `Change(next)` 时原样转出 |

名字和载荷照 NSH（约定「受控」一条），和 `nsh/base` 的 `Checkbox` 相同。

### 部件结构

```
Checkbox 根（nsh/base Checkbox，flex-row items-start，group/nlb-checkbox）
├─ indicator（nsh/base 的指示器视图：框）
│   └─ mark（勾 check / 横杠 minus 图标，只在选中或半选时建）
└─ label（本库包的一层，里面是 children）
```

## 4. 变体与尺寸

没有变体（shadcn 也没有）。`size` 四档都支持，框边长取组件 token `checkbox-size-<size>`，其余随它定：

| size | 框（指针） | 框（触屏） | 勾 / 横杠图标 | 标签字号 |
|---|---|---|---|---|
| `xs` | 12 | 16 | 框 − 2 | `font-size-xs` 12 |
| `sm` | 14 | 18 | 框 − 2 | `font-size-sm` 13 |
| `md` | 16 | 20 | 框 − 2（14，shadcn 的 `size-3.5`） | `font-size-md` 14 |
| `lg` | 18 | 24 | 框 − 2 | `font-size-lg` 14 |

框边长 = 对应控件高度的一半（指针 `control-height-<size>`，触屏 `control-height-touch-<size>`），所以改
`control-height` 种子，框跟着按比例变。`md` 指针 16 = shadcn 的 `size-4`。

固定几何（写在组件里，不是 token）：

- 勾和横杠用 foundation 的图标，显式给边长 = 框 − 2（不取 `Size` 的默认图标边长 12 / 14 / 16 / 20，那是按钮里的尺寸），
  笔画用图标的默认粗细，居中。图标颜色跟随文字：框的文字色设成 `checkbox-checked-foreground`，勾跟着它。
- 圆角 `checkbox-radius`，并且不超过框边长的 1/4（大圆角主题下框不变成圆，免得和 radio 混淆）。
- 根是 `flex-row items-start`，框和标签之间 `checkbox-gap`。
- `label` 部件：`min-height` = 框边长、`justify-center`，单行标签和框垂直居中；多行时第一行贴着框的上沿。
  文字字号 `font-size-<size>`、字重 `label-font-weight`、颜色 `label-foreground`（和 Label 组件同一组 token），
  行高用 foundation 的「字号 → 行高」；文字子节点继承。
- 框 `flex-shrink: 0`。

和 shadcn 的不同：shadcn 的勾选框只有 16 一档，标签是另一个 `Label` 组件；本库按约定四档，标签放在 `children` 里，
因为 `nsh/base` 把 `children` 放进按压目标和读屏名，点标签就是点框，不需要 `htmlFor`。框和标签间距 8（shadcn 示例
`gap-3` 12，Label 内部 `gap-2` 8）：取桌面的中等密度。

## 5. 状态

颜色全部走 §7 的 token；下面括号里是 shadcn 对应的类。只做 shadcn 有的状态（约定「状态」一条）。

| 状态 | 框底 | 框边 | 勾 | 其他 |
|---|---|---|---|---|
| 未选中 | `checkbox`（亮：透明；暗：`input` 30%，`dark:bg-input/30`） | `checkbox-border`（`border-input`） | 无 | 阴影 `checkbox-shadow`（1，`shadow-xs`） |
| 选中 | `checkbox-checked`（`bg-primary`） | `checkbox-checked-border`（`border-primary`） | `check`，`checkbox-checked-foreground`（`text-primary-foreground`） | |
| 半选 | 同选中 | 同选中 | `minus`，同色 | shadcn v4 没画半选，`nsh/base` 有这一态，本库补横杠 |
| hover | 不做 | | | shadcn 没有 |
| active（按下） | | | | foundation `press(0.9)` 作用在框上；减弱动态效果时不缩 |
| focus-visible（指针） | | 未选中时 `checkbox-border-focus`（`focus-visible:border-ring`）；选中 / 半选保持选中的边 | | 框外焦点环：宽 `ring-width` 3，色 `checkbox-ring`（`ring-[3px] ring-ring/50`），不占布局 |
| focus-visible（触屏） | | 所有状态都换成 `checkbox-border-focus`（invalid 时保持 `checkbox-border-invalid`） | | 不画外环 |
| invalid | | 未选中时 `checkbox-border-invalid`（`aria-invalid:border-destructive`）；选中 / 半选保持选中的边 | | 指针聚焦时外环换成 `checkbox-ring-invalid`（`aria-invalid:ring-destructive/20 dark:…/40`） |
| disabled | | | | 根整体 `opacity-disabled`（框和标签一起淡，等于 shadcn 的 `disabled:opacity-50` 加 Label 的 `peer-disabled:opacity-50`）；无按下反馈 |
| readonly、loading | 不支持 | | | shadcn 和 `nsh/base` 都没有 |

指针上框边的优先级（shadcn 类在 Tailwind 4 里的先后）：选中 / 半选 > invalid > focus-visible > 默认。触屏上焦点没有外环，
所以 focus-visible 的边压过选中的边，只让给 invalid。

过渡：只有焦点环过渡（shadcn `transition-shadow`，`duration`）；底色、边、勾随值立即换（shadcn 指示器 `transition-none`）。

光标：一直是默认箭头，禁用时也是。

实现要点：

- 选中 / 半选的颜色用绑定读 `checked` / `indeterminate` 来算，不用 `nsh/base` 投到指示器上的 `checked:` 变体：
  它的 `checked:` 只在「选中且未禁用」时生效，禁用的已选框会画成未选中。
- 焦点环和按下要画在框上，但 `nsh/base` 只把 checked / indeterminate / disabled 投到指示器。本库在根上加
  `group/nlb-checkbox`，框用 `group-focus-visible/nlb-checkbox`、`group-active/nlb-checkbox`
  （nsh-core 有 `group_focus_visible_named` / `group_active_named`）。**实现的第一件事是验证这条**（§9 缺口 1）。

## 6. 指针与触屏

| | 指针 | 触屏 |
|---|---|---|
| 框边长 | `control-height-<size>` / 2（12 / 14 / 16 / 18） | `control-height-touch-<size>` / 2（16 / 18 / 20 / 24） |
| 命中区 | 不补 | 没有 `children` 时，各边补 `max(0, (touch-target − 框) / 2)`，到 44 见方，外观不变；有标签时标签就是命中区，不补（补出去会伸进相邻行） |
| 焦点 | 框外环 + 边换色 | 只换边色 |
| 悬停 | 无 | 无 |

指针 / 触屏的判断和推导用 foundation 的 `pointer_touch`。应用给了 `hitSlop` 就用应用的。

## 7. 组件 token

| token | 类型 | 推导默认 | 值 |
|---|---|---|---|
| `checkbox-size-xs` | 长度 | `pointer_touch(control-height-xs × 0.5, control-height-touch-xs × 0.5)` | 12（触屏 16） |
| `checkbox-size-sm` | 长度 | `pointer_touch(control-height-sm × 0.5, control-height-touch-sm × 0.5)` | 14（触屏 18） |
| `checkbox-size-md` | 长度 | `pointer_touch(control-height-md × 0.5, control-height-touch-md × 0.5)` | 16（触屏 20） |
| `checkbox-size-lg` | 长度 | `pointer_touch(control-height-lg × 0.5, control-height-touch-lg × 0.5)` | 18（触屏 24） |
| `checkbox-radius` | 长度 | `max(radius-sm − 2, 0)` | 4（shadcn `rounded-[4px]`） |
| `checkbox-gap` | 长度 | `spacing` × 2 | 8 |
| `checkbox` | 颜色 | `light_dark(transparent, input × 30%)` | 亮透明，暗 `dark:bg-input/30` |
| `checkbox-border` | 颜色 | `input` | |
| `checkbox-checked` | 颜色 | `primary` | |
| `checkbox-checked-border` | 颜色 | `checkbox-checked` | 跟着填色 |
| `checkbox-checked-foreground` | 颜色 | `primary-foreground` | |
| `checkbox-border-focus` | 颜色 | `ring` | |
| `checkbox-border-invalid` | 颜色 | `destructive` | |
| `checkbox-ring` | 颜色 | `ring` × `ring-opacity`（50%） | |
| `checkbox-ring-invalid` | 颜色 | `ring-invalid` | 亮 `destructive` 20%，暗 40% |
| `checkbox-shadow` | 数字 | 1（`xs`） | 0 关掉阴影 |

- 应用要单独改触屏的边长，写 `checkbox-size-<size>-touch`（tokens.md 的通用规则）。
- `checked` 放在变体的位置（`checkbox-checked-border`），因为状态后缀只用 `-hover` / `-active` / `-focus` / `-invalid` /
  `-disabled`；选中是值，不是交互状态。
- 标签文字用 Label 的 `label-font-weight`、`label-foreground` 和语义 `font-size-<size>`，不另设。焦点环的几何用 foundation 的。
- 改 `primary`，选中的填色、边跟着变；改 `checkbox-checked`，边也跟着变（`checkbox-checked-border` 引用它）。

## 8. 示例

TSX：

```tsx
import { signal, View } from "nsh";
import { Checkbox } from "nsh-longbridge";

const agreed = signal(false);
const items = signal<string[]>(["mentions"]);
const all = ["mentions", "approvals", "summary"];

<View class="gap-3">
  <Checkbox checked={agreed()} invalid={!agreed()} onChange={({ checked }) => agreed.set(checked)}>
    Accept terms and conditions
  </Checkbox>

  {/* 全选行：只选了一部分时半选 */}
  <Checkbox
    checked={items().length === all.length}
    indeterminate={items().length > 0 && items().length < all.length}
    onChange={({ checked }) => items.set(checked ? all : [])}
  >
    All notifications
  </Checkbox>

  {/* 只有框：要读屏名 */}
  <Checkbox size="sm" checked={agreed()} onChange={({ checked }) => agreed.set(checked)} accessibilityLabel="Select row" />

  <Checkbox disabled checked>Locked option</Checkbox>
</View>
```

Rust：

```rust
use nsh_core::prelude::*;
use nsh_longbridge::{Size, checkbox};

let agreed = signal(false);

view()
    .gap_3()
    .child(
        checkbox()
            .checked(move || agreed.get())
            .invalid(move || !agreed.get())
            .on_change(move |checked: &bool| agreed.set(*checked))
            .child(text("Accept terms and conditions")),
    )
    .child(
        checkbox()
            .size(Size::Sm)
            .checked(move || agreed.get())
            .on_change(move |checked: &bool| agreed.set(*checked))
            .accessibility_label("Select row"),
    )
```

## 9. NSH 能力缺口

1. **指示器不跟随根的 focus-visible、hover、active。** `nsh/base` 的 `Checkbox` 只把 checked / indeterminate /
   disabled 投到指示器（`checkbox.rs` 的 `project_*`），`Switch` 多投了 active，都没有 focus-visible。
   绕法：根上挂 `group/nlb-checkbox`，框用 named group 变体（§5）。实现时第一件事验证：`nsh/base` 组件元素上加的
   `group` 声明能合并到它的根上、`indicatorStyle` 里能带 group 变体。任一不行，退到焦点环画在整行根上
   （gpui-kit 的做法），并回报。
2. **`checked:` 投影在禁用时不生效。** 「选中且未禁用」才算 checked，禁用的已选框用变体画会变成未选中。绕法：
   颜色用绑定读值（§5）。不算 bug，记下免得实现时踩。
