# RadioGroup

状态：API 单，评审已过（2026-11-26），待实现。外观照 shadcn/ui new-york v4 `radio-group.tsx`（`RadioGroup` +
`RadioGroupItem`），行为包 `nsh/base` 的 `RadioGroup` 和 `Radio`。

焦点、禁用、invalid、标签、命中区的处理和 [`Checkbox`](checkbox.md)、[`Switch`](switch.md) 一致；
改其中一份的这些规则，三份一起改。

## 1. 用途

单选组：一小组都看得见的选项里选且只选一个（套餐、排序方式、通知频率）。

何时不用：

| 场景 | 改用 |
|---|---|
| 选项多（大约 5 个以上）或位置紧 | `Select`（P2） |
| 能多选，或能一个都不选 | `Checkbox`，或以后的 `ToggleGroup` |
| 开 / 关一项设置 | `Switch` |
| 切换页面或视图 | `Tabs`（P2） |
| 按钮外观的二选一 / 多选一（分段控件） | 以后的 `ToggleGroup` / 分段控件；本组件不加按钮变体 |
| 整张卡片可点的选项 | `RadioGroupItem` 放进 `Field`（P3）；本组件不加卡片变体 |

## 2. 基于 `nsh/base` 的 `RadioGroup` / `Radio`

`RadioGroup` 的根是 `nsh/base` 的 `RadioGroup`，每个 `RadioGroupItem` 的根是 `nsh/base` 的 `Radio`
（`crates/nsh-base/src/radio.rs`）。下面这些它已经做了，本库不重写：

- 按压：点未选中的项（圆圈或标签，`children` 在按压目标里），或聚焦后 Space / Return，报 `Change(true)`；已选中的项
  再按什么也不报（单选不能取消自己）。
- 键盘（WAI-ARIA radio group，macOS 和 Web 上可按的视图是焦点站时）：整组是 Tab 的一站，落在选中项上，没有选中时
  落在第一个未禁用的项；↓ / → 到下一项、↑ / ← 到上一项，首尾循环、跳过禁用项，焦点移到哪项就报那项 `Change(true)`；
  两组方向键在横排和竖排里都有效。
- 读屏：组的角色 `radiogroup`，名字是 `accessibilityLabel`，进组时念；每项角色 `radio`，状态 checked 和 selected
  同时给（不同读屏各读一个）；`disabled` 念「变暗」；项的名字取 `children` 里的文字，没有时用项的 `accessibilityLabel`。
- `hitSlop`：项的命中区外扩，不改布局和外观。
- 终端：`nsh/base` 在项前画 `[●]` / `[ ]`；本库在终端里不画自己的圆圈（`platform.idiom === "terminal"` 时不建
  `indicator` 部件），标签照常。

`nsh/base` 的 `RadioGroup` 不持有值，每个 `Radio` 各自收 `checked`。本库照 shadcn 把值放到组上：`RadioGroup` 收
`value`，通过上下文告诉每项；项的 `checked` = 组的 `value` 等于项的 `value`；项报 `Change(true)` 时，组报
`onChange({ value: 项的 value })`。

本库加的：组上的值与事件、圆圈和圆点、标签排版、各状态的颜色、尺寸、触屏命中区、按下反馈、组的 `disabled` 和 `invalid`。

## 3. API

### `RadioGroup` props

| TSX | Rust | 类型 | 默认 | 说明 |
|---|---|---|---|---|
| `value` | `value` | `string?` / `Prop<Option<SharedString>>` | 无 | 受控；没有值或没有项匹配时都不选中 |
| `disabled` | `disabled` | `boolean` / `Prop<bool>` | `false` | 所有项禁用（和各项自己的 `disabled` 取或）；全禁用时整组不是焦点站 |
| `invalid` | `invalid` | `boolean` / `Prop<bool>` | `false` | 所有项的外观，见「状态」；读屏念不到，见 §9 缺口 1 |
| `size` | `size` | `Size` | `"md"` | 所有项一起，`xs` / `sm` / `md` / `lg` |
| `accessibilityLabel` | `accessibility_label` | `string?` | — | 组的读屏名，建议总是给 |
| `children` | `children` | 子元素 | — | 项，以及应用排在项之间的东西 |
| `class` / `style` | `class` / `style` | | | 组的根，最后合并；横排写 `class="flex-row"` |

### `RadioGroupItem` props

| TSX | Rust | 类型 | 默认 | 说明 |
|---|---|---|---|---|
| `value` | `value` | `string` / `Prop<SharedString>` | 必填 | 这一项代表的值，组内唯一 |
| `disabled` | `disabled` | `boolean` / `Prop<bool>` | `false` | |
| `accessibilityLabel` | `accessibility_label` | `string?` | — | 没有 `children` 文字时必填 |
| `hitSlop` | `hit_slop` | `EdgeInsets?` | 自动（见 §6） | 给了就用给的 |
| `children` | `children` | 字符串或子元素 | — | 标签，放进 `label` 部件；字符串自动包成文字 |
| `class` / `style` | `class` / `style` | | | 项的根（整行：圆圈 + 标签），最后合并 |
| `indicatorClass` / `indicatorStyle` | `indicator_class` / `indicator_style` | | | 圆圈 |
| `dotClass` / `dotStyle` | `dot_class` / `dot_style` | | | 圆圈里的圆点，只在选中时建 |
| `labelClass` / `labelStyle` | `label_class` / `label_style` | | | 包 `children` 的那层；没有 `children` 时不建 |

`invalid`、`size` 只在组上设，项跟组（评审定的）：单选的校验（必选）和尺寸是整组的事。`RadioGroupItem` 必须在
`RadioGroup` 里；不在时 debug 构建告警，项画成未选中、按了不报事件。

不提供 `defaultValue`（只受控）、`orientation`（`nsh/base` 两组方向键都认，方向只是布局，用 `class`）、
`required` / `name`（NSH 没有表单提交，必选由应用校验后设 `invalid`）。

### 事件

| 组件 | TSX | Rust | 何时 |
|---|---|---|---|
| `RadioGroup` | `onChange({ value })` | `RadioGroupEvent::Change(SharedString)`（`#[args(value)]`），builder `.on_change(\|value: &SharedString\| ..)` | 某项报 `Change(true)`：按了未选中的项，或方向键把焦点移到它 |

`RadioGroupItem` 没有事件。名字和载荷照 NSH 的值状态规矩（`value` + `onChange({ value })`），不用 Radix 的 `onValueChange`。

### 部件结构

```
RadioGroup 根（nsh/base RadioGroup，flex-col，gap radio-group-gap）
└─ RadioGroupItem 根（nsh/base Radio，flex-row items-start，group/nlb-radio）
    ├─ indicator（本库画的圆圈）
    │   └─ dot（圆点，选中时建）
    └─ label（本库包的一层，里面是 children）
```

## 4. 变体与尺寸

没有变体（shadcn 也没有）。`size` 四档都支持，在组上设：

| size | 圆圈（指针） | 圆圈（触屏） | 圆点 | 标签字号 |
|---|---|---|---|---|
| `xs` | 12 | 16 | 圆圈 / 2 | `font-size-xs` 12 |
| `sm` | 14 | 18 | 圆圈 / 2 | `font-size-sm` 13 |
| `md` | 16 | 20 | 圆圈 / 2（8，shadcn `size-2`） | `font-size-md` 14 |
| `lg` | 18 | 24 | 圆圈 / 2 | `font-size-lg` 14 |

圆圈边长 = 对应控件高度的一半（指针 `control-height-<size>`，触屏 `control-height-touch-<size>`），和 Checkbox 的框、
Switch 的滑块同边长，同一 `size` 的三种控件并排时对齐。`md` 指针 16 = shadcn 的 `size-4`。

固定几何（写在组件里）：

- 圆圈正圆（圆角 = 边长 / 2，不跟 `radius` 种子），`border-width` 宽的边，`flex-shrink: 0`；圆点正圆，居中。
- 项的根 `flex-row items-start`，圆圈和标签之间 `radio-group-item-gap`。
- `label` 部件和 Checkbox 相同：`min-height` = 圆圈边长、`justify-center`；文字字号 `font-size-<size>`、字重
  `label-font-weight`、颜色 `label-foreground`，行高用 foundation 的「字号 → 行高」，文字子节点继承。
- 组的根：`flex-col`，项之间 `radio-group-gap`（shadcn `grid gap-3`，单列时和 flex 列效果相同）。

和 shadcn 的不同：shadcn 的项只有圆圈，标签是另一个 `Label`；本库把标签放进项的 `children`，因为 `nsh/base` 把它放进
按压目标和读屏名。

## 5. 状态

颜色全部走 §7 的 token；括号里是 shadcn 对应的类。只做 shadcn 有的状态。

| 状态 | 圆圈底 | 圆圈边 | 圆点 | 其他 |
|---|---|---|---|---|
| 未选中 | `radio-group-item`（亮：透明；暗：`input` 30%，`dark:bg-input/30`） | `radio-group-item-border`（`border-input`） | 无 | 阴影 `radio-group-item-shadow`（1，`shadow-xs`） |
| 选中 | 不变 | `radio-group-item-checked-border`（默认同未选中：shadcn 选中时边不变） | `radio-group-item-dot`（`fill-primary`） | |
| hover | 不做 | | | shadcn 没有 |
| active（按下） | | | | foundation `press(0.9)` 作用在圆圈上；减弱动态效果时不缩 |
| focus-visible（指针） | | `radio-group-item-border-focus`（`focus-visible:border-ring`） | | 圆圈外焦点环：宽 `ring-width` 3，色 `radio-group-item-ring`（`ring-[3px] ring-ring/50`），不占布局 |
| focus-visible（触屏） | | `radio-group-item-border-focus` | | 不画外环 |
| invalid | | `radio-group-item-border-invalid`（`aria-invalid:border-destructive`），选中时也是 | | 指针聚焦时外环换成 `radio-group-item-ring-invalid`（`aria-invalid:ring-destructive/20 dark:…/40`） |
| disabled | | | | 项的根整体 `opacity-disabled`（圆圈和标签一起淡）；无按下反馈；方向键跳过 |
| readonly、loading | 不支持 | | | shadcn 和 `nsh/base` 都没有 |

圆圈边的优先级：invalid > focus-visible > 选中 > 默认（shadcn 没有选中边的规则，aria 变体排在 focus-visible 后面）。
指针和触屏相同。

过渡：圆圈的边色和焦点环过渡（shadcn `transition-[color,box-shadow]`，`duration`）；圆点随值立即出现 / 消失。

光标：一直是默认箭头。

实现要点：

- 项的颜色、圆点用绑定读本库算出的 `checked`（组的 `value` == 项的 `value`），不用 `checked:` 变体
  （`nsh/base` 的 `checked:` 只在「选中且未禁用」时生效）。
- 焦点环和按下画在圆圈上：项的根加 `group/nlb-radio`，圆圈用 `group-focus-visible/nlb-radio`、
  `group-active/nlb-radio`。**实现的第一件事是验证这条**；不行就退到环画在项的根上并回报，见
  [checkbox.md §9](checkbox.md#9-nsh-能力缺口) 缺口 1。
- 组的 `disabled` 传给每个 `nsh/base` `Radio` 的 `disabled`（取或），`nsh/base` 的 roving 才会跳过它们。

## 6. 指针与触屏

| | 指针 | 触屏 |
|---|---|---|
| 圆圈边长 | `control-height-<size>` / 2（12 / 14 / 16 / 18） | `control-height-touch-<size>` / 2（16 / 18 / 20 / 24） |
| 命中区 | 不补 | 项没有 `children` 时，各边补 `max(0, (touch-target − 圆圈) / 2)`，外观不变；有标签时标签就是命中区，不补 |
| 焦点 | 圆圈外环 + 边换色 | 只换边色 |
| 悬停 | 无 | 无 |
| 项间距 | `radio-group-gap` 12 | 同 |

## 7. 组件 token

| token | 类型 | 推导默认 | 值 |
|---|---|---|---|
| `radio-group-gap` | 长度 | `spacing` × 3 | 12（跟 `spacing` 种子走） |
| `radio-group-item-size-xs` | 长度 | `pointer_touch(control-height-xs × 0.5, control-height-touch-xs × 0.5)` | 12（触屏 16） |
| `radio-group-item-size-sm` | 长度 | `pointer_touch(control-height-sm × 0.5, control-height-touch-sm × 0.5)` | 14（触屏 18） |
| `radio-group-item-size-md` | 长度 | `pointer_touch(control-height-md × 0.5, control-height-touch-md × 0.5)` | 16（触屏 20） |
| `radio-group-item-size-lg` | 长度 | `pointer_touch(control-height-lg × 0.5, control-height-touch-lg × 0.5)` | 18（触屏 24） |
| `radio-group-item-gap` | 长度 | `spacing` × 2 | 8 |
| `radio-group-item` | 颜色 | `light_dark(transparent, input × 30%)` | 亮透明 |
| `radio-group-item-border` | 颜色 | `input` | |
| `radio-group-item-checked-border` | 颜色 | `radio-group-item-border` | |
| `radio-group-item-border-focus` | 颜色 | `ring` | |
| `radio-group-item-border-invalid` | 颜色 | `destructive` | |
| `radio-group-item-ring` | 颜色 | `ring` × `ring-opacity`（50%） | |
| `radio-group-item-ring-invalid` | 颜色 | `ring-invalid` | 亮 `destructive` 20%，暗 40% |
| `radio-group-item-dot` | 颜色 | `primary` | |
| `radio-group-item-shadow` | 数字 | 1（`xs`） | 0 关掉阴影 |

- 应用要单独改触屏的圆圈，写 `radio-group-item-size-<size>-touch`。
- `checked` 放在变体的位置（`radio-group-item-checked-border`），因为状态后缀只用 `-hover` / `-active` / `-focus` /
  `-invalid` / `-disabled`；选中是值，不是交互状态。
- 主题想要「选中时边也变主色」：`radio-group-item-checked-border: var(primary)`。
- 标签文字用 Label 的 `label-*` token，不另设。焦点环的几何用 foundation 的。

## 8. 示例

TSX：

```tsx
import { signal } from "nsh";
import { RadioGroup, RadioGroupItem } from "nsh-longbridge";

const plan = signal<string | undefined>("monthly");
const sort = signal("name");

<RadioGroup
  accessibilityLabel="Billing"
  value={plan()}
  invalid={plan() === undefined}
  onChange={({ value }) => plan.set(value)}
>
  <RadioGroupItem value="monthly">Monthly</RadioGroupItem>
  <RadioGroupItem value="yearly">Yearly</RadioGroupItem>
  <RadioGroupItem value="lifetime" disabled>Lifetime</RadioGroupItem>
</RadioGroup>

{/* 横排、小号 */}
<RadioGroup class="flex-row" size="sm" accessibilityLabel="Sort by" value={sort()} onChange={({ value }) => sort.set(value)}>
  <RadioGroupItem value="name">Name</RadioGroupItem>
  <RadioGroupItem value="date">Date</RadioGroupItem>
</RadioGroup>
```

Rust：

```rust
use nsh_core::prelude::*;
use nsh_longbridge::{radio_group, radio_group_item};

let plan = signal(Some(SharedString::from("monthly")));

radio_group()
    .accessibility_label("Billing")
    .value(move || plan.get())
    .invalid(move || plan.get().is_none())
    .on_change(move |value: &SharedString| plan.set(Some(value.clone())))
    .child(radio_group_item().value("monthly").child(text("Monthly")))
    .child(radio_group_item().value("yearly").child(text("Yearly")))
    .child(radio_group_item().value("lifetime").disabled(true).child(text("Lifetime")))
```

## 9. NSH 能力缺口

1. **`Radio` / `RadioGroup` 没有 `invalid`、`accessibilityDescribedBy`。** `nsh/base` 的 `Checkbox` 有这两项，
   `Radio` 和 `RadioGroup` 都没有（`Radio` 也没有 `accessibilityHint`）。本库的 `invalid` 只能画外观，读屏不念
   「无效」，错误行也挂不上去。不在 `nsh/base` 的元素上另设读屏状态：会盖掉它自己给的 checked / selected。
   绕法：应用把错误文字放在组旁边并在出现时播报（P3 的 `Field` 会做）。需要 `nsh/base` 给 `RadioGroup` 加
   `invalid` 和 `accessibilityDescribedBy`（WAI-ARIA 里两者都在 radiogroup 上）。
2. **`RadioGroup` 不持有值、没有 `disabled`。** 本库用自己的上下文补上（§2、§5），不算缺陷；记下免得以为
   `nsh/base` 会管。
3. **部件不跟随 focus-visible、active。** 同 Checkbox 缺口 1；`Radio` 连指示器部件都没有（圆圈是本库放进
   `children` 的），只能靠 named group。
4. **`checked:` 投影在禁用时不生效。** 同 Checkbox，用绑定。
