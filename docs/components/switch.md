# Switch

状态：API 单，评审已过（2026-11-26），待实现。外观照 shadcn/ui new-york v4 `switch.tsx`，行为包 `nsh/base` 的 `Switch`。

焦点、禁用、invalid、标签、命中区的处理和 [`Checkbox`](checkbox.md)、[`RadioGroup`](radio-group.md) 一致；
改其中一份的这些规则，三份一起改。

## 1. 用途

开关：改了立即生效的设置（开启通知、深色模式、自动续订）。

何时不用：

| 场景 | 改用 |
|---|---|
| 要等提交才生效的选项、同意条款、必填项 | `Checkbox` |
| 多选一 | `RadioGroup` / `Select`（P2） |
| 工具栏里按下 / 弹起的按钮 | `Toggle`（以后） |
| 手机分组列表里的系统开关 | 以后的列表行组件；本库的 Switch 在触屏上也画 shadcn 外观，不换系统控件 |

## 2. 基于 `nsh/base` 的 `Switch`

根元素就是 `nsh/base` 的 `Switch`（`crates/nsh-base/src/switch.rs`），本库只给外观，下面这些它已经做了，本库不重写：

- 按压：点轨道、点标签（`children` 在按压目标里）、聚焦后 Space / Return，都报一次 `Change(!checked)`。
  值由应用持有，组件只显示应用传回来的值。
- 焦点：键盘焦点的一站；`disabled` 时不接按压、无按下反馈。
- 读屏：角色 `switch`，状态 on / off；`disabled` 念「变暗」；名字取 `children` 里的文字，没有时用
  `accessibilityLabel`；`accessibilityHint` 原样转给它。
- 部件：轨道（track）和里面的滑块（thumb），两者都跟随 checked / disabled，并在开关被按住时（不论按在哪，标签也算）
  跟随 active（`project_group_active`）。
- `hitSlop`：命中区外扩，不改布局和外观。
- 终端：`nsh/base` 用勾选框的记号代替轨道和滑块，本库画的轨道在终端不出现，标签照常。

本库加的：轨道、滑块的几何与颜色，滑块位移动画，标签排版，尺寸，触屏命中区，按下反馈。

## 3. API

### Props

| TSX | Rust | 类型 | 默认 | 说明 |
|---|---|---|---|---|
| `checked` | `checked` | `boolean` / `Prop<bool>` | `false` | 受控值 |
| `disabled` | `disabled` | `boolean` / `Prop<bool>` | `false` | |
| `size` | `size` | `Size` | `"md"` | `xs` / `sm` / `md` / `lg` |
| `accessibilityLabel` | `accessibility_label` | `string?` | — | 没有 `children` 文字时必填 |
| `accessibilityHint` | `accessibility_hint` | `string?` | — | 转给 `nsh/base` |
| `hitSlop` | `hit_slop` | `EdgeInsets?` | 自动（见 §6） | 给了就用给的 |
| `children` | `children` | 字符串或子元素 | — | 标签，放进 `label` 部件；字符串自动包成文字 |
| `class` / `style` | `class` / `style` | | | 根（整行：轨道 + 标签），最后合并 |
| `trackClass` / `trackStyle` | `track_class` / `track_style` | | | 轨道 |
| `thumbClass` / `thumbStyle` | `thumb_class` / `thumb_style` | | | 滑块 |
| `labelClass` / `labelStyle` | `label_class` / `label_style` | | | 包 `children` 的那层；没有 `children` 时不建 |

只受控，不提供 `defaultChecked`。不做 `invalid`（评审定的）：开关立即生效，没有「提交时校验」；shadcn 的开关没有
invalid 外观，`nsh/base` 的 `Switch` 也没有 `invalid` / `accessibilityDescribedBy`。需要校验的选项用 `Checkbox`。

### 事件

| TSX | Rust | 何时 |
|---|---|---|
| `onChange({ checked })` | `SwitchEvent::Change(bool)`（`#[args(checked)]`），builder `.on_change(\|checked: &bool\| ..)` | `nsh/base` 报 `Change(next)` 时原样转出 |

名字和载荷照 NSH，和 `nsh/base` 的 `Switch`、本库的 `Checkbox` 相同。

### 部件结构

```
Switch 根（nsh/base Switch，flex-row items-start，group/nlb-switch）
├─ track（轨道）
│   └─ thumb（滑块）
└─ label（本库包的一层，里面是 children）
```

## 4. 变体与尺寸

没有变体。`size` 四档都支持。几何全部由滑块直径 `t`（token `switch-thumb-size-<size>`）和边宽
`b`（`border-width`，1）算出来，写在组件里：

- 轨道宽 = 2t，高 = t + 2b；`b` 宽的透明边，焦点时换色。
- 轨道圆角 = `min(switch-radius, 轨道高 / 2)`；滑块圆角 = `max(0, 轨道圆角 − b)`，和轨道同心。默认胶囊。
- 滑块直径 t，关时贴着起点（左），开时右移 t − 2b。

| size | 滑块 t（指针） | 轨道（指针） | 滑块 t（触屏） | 轨道（触屏） | 标签字号 |
|---|---|---|---|---|---|
| `xs` | 12 | 24 × 14（= shadcn `size="sm"`） | 16 | 32 × 18 | `font-size-xs` 12 |
| `sm` | 14 | 28 × 16 | 18 | 36 × 20 | `font-size-sm` 13 |
| `md` | 16 | 32 × 18（= shadcn `default`） | 20 | 40 × 22 | `font-size-md` 14 |
| `lg` | 18 | 36 × 20 | 24 | 48 × 26 | `font-size-lg` 14 |

t = 对应控件高度的一半（指针 `control-height-<size>`，触屏 `control-height-touch-<size>`），和 Checkbox 的框同边长，
所以同一 `size` 的开关滑块和勾选框一样大。shadcn 的 default 轨道高 `1.15rem`（18.4），这里取整数 18；
滑块位移 14 和 shadcn 的 `translate-x-[calc(100%-2px)]` 相同。

右到左布局时轨道整体镜像（`rtl:-scale-x-100`），滑块关时在起点、开时在终点。

标签部件和 Checkbox 相同：根 `flex-row items-start`，轨道和标签之间 `switch-gap`；`label` 的 `min-height` = 轨道高、
`justify-center`；文字字号 `font-size-<size>`、字重 `label-font-weight`、颜色 `label-foreground`，行高用 foundation 的
「字号 → 行高」。轨道 `flex-shrink: 0`。shadcn 示例里开关和 Label 用 `space-x-2`（8），本库一致。

圆角不跟 `radius` 种子（照 shadcn 的 `rounded-full`），但应用可以改 `switch-radius`（评审定的）。

## 5. 状态

颜色全部走 §7 的 token；括号里是 shadcn 对应的类。只做 shadcn 或平台有的状态。

| 状态 | 轨道 | 滑块 | 其他 |
|---|---|---|---|
| 关 | `switch`（亮 `input`；暗 `input` 80%，`data-[state=unchecked]:bg-input dark:…bg-input/80`） | `switch-thumb`（亮 `background`；暗 `foreground`） | 轨道阴影 `switch-shadow`（1，`shadow-xs`） |
| 开 | `switch-checked`（`bg-primary`） | `switch-checked-thumb`（亮 `background`；暗 `primary-foreground`） | |
| hover | 不做 | | shadcn 没有 |
| active（按住） | | 横向拉长到 1.25 倍，关时以左端、开时以右端为原点（跟 `nsh/base` 投下来的 `active:`；macOS / iOS 开关的做法） | 减弱动态效果时不拉长 |
| focus-visible（指针） | 边从透明换成 `switch-border-focus`（`focus-visible:border-ring`） | | 轨道外焦点环：宽 `ring-width` 3，色 `switch-ring`（`ring-[3px] ring-ring/50`），不占布局 |
| focus-visible（触屏） | 边换成 `switch-border-focus` | | 不画外环 |
| invalid | 不做 | | |
| disabled | | | 根整体 `opacity-disabled`；无按下反馈 |
| readonly、loading | 不支持 | | shadcn 和 `nsh/base` 都没有 |

过渡：滑块位移和拉长 `transform` 过渡，轨道底色、边、焦点环过渡，都用 `duration`（shadcn `transition-all` /
`transition-transform`，默认 150ms），缓动用 foundation 的；减弱动态效果时归零。值从别处改变（不是按出来的）时同样动画。

光标：一直是默认箭头。

实现要点：

- 颜色用绑定读 `checked` 来算，不用 `checked:` 变体：`nsh/base` 的 `checked:` 只在「开且未禁用」时生效，
  禁用的开着的开关会画成关。
- 按住的拉长用 `nsh/base` 已投到滑块上的 `active:`，不用 foundation 的 `press(scale)`（那是等比缩小）。
- 焦点环画在轨道上，但 `nsh/base` 不把 focus-visible 投到部件：根上加 `group/nlb-switch`，轨道用
  `group-focus-visible/nlb-switch`（不借 `nsh/base` 内部的 `nsh-switch` 组名）。**实现的第一件事是验证这条**；
  不行就退到环画在整行根上并回报，见 [checkbox.md §9](checkbox.md#9-nsh-能力缺口) 缺口 1。

## 6. 指针与触屏

| | 指针 | 触屏 |
|---|---|---|
| 尺寸 | 见 §4 指针列 | 见 §4 触屏列 |
| 命中区 | 不补 | 没有 `children` 时，上下各补 `max(0, (touch-target − 轨道高) / 2)`，左右各补 `max(0, (touch-target − 轨道宽) / 2)`，外观不变；有标签时标签就是命中区，不补 |
| 焦点 | 轨道外环 + 边换色 | 只换边色 |
| 悬停 | 无 | 无 |

## 7. 组件 token

| token | 类型 | 推导默认 | 值 |
|---|---|---|---|
| `switch-thumb-size-xs` | 长度 | `pointer_touch(control-height-xs × 0.5, control-height-touch-xs × 0.5)` | 12（触屏 16） |
| `switch-thumb-size-sm` | 长度 | `pointer_touch(control-height-sm × 0.5, control-height-touch-sm × 0.5)` | 14（触屏 18） |
| `switch-thumb-size-md` | 长度 | `pointer_touch(control-height-md × 0.5, control-height-touch-md × 0.5)` | 16（触屏 20） |
| `switch-thumb-size-lg` | 长度 | `pointer_touch(control-height-lg × 0.5, control-height-touch-lg × 0.5)` | 18（触屏 24） |
| `switch-radius` | 长度 | 9999（胶囊，实际取 `min(它, 轨道高 / 2)`） | 不跟 `radius` 种子 |
| `switch-gap` | 长度 | `spacing` × 2 | 8 |
| `switch` | 颜色 | `light_dark(input, input × 80%)` | |
| `switch-checked` | 颜色 | `primary` | |
| `switch-thumb` | 颜色 | `light_dark(background, foreground)` | 暗色下关着的滑块是亮的 |
| `switch-checked-thumb` | 颜色 | `light_dark(background, primary-foreground)` | |
| `switch-border-focus` | 颜色 | `ring` | |
| `switch-ring` | 颜色 | `ring` × `ring-opacity`（50%） | |
| `switch-shadow` | 数字 | 1（`xs`） | 0 关掉阴影 |

- 应用要单独改触屏的滑块，写 `switch-thumb-size-<size>-touch`。
- `checked` 放在变体的位置（`switch-checked-thumb`），因为状态后缀只用 `-hover` / `-active` / `-focus` / `-invalid` /
  `-disabled`；选中是值，不是交互状态。
- 没有 invalid，所以没有 `-invalid` token。标签文字用 Label 的 `label-*` token，不另设。焦点环的几何用 foundation 的。

## 8. 示例

TSX：

```tsx
import { signal, View } from "nsh";
import { Switch } from "nsh-longbridge";

const airplane = signal(false);

<View class="gap-3">
  <Switch checked={airplane()} onChange={({ checked }) => airplane.set(checked)}>
    Airplane mode
  </Switch>

  {/* 标签在别处、只有轨道：要读屏名 */}
  <Switch size="xs" checked={airplane()} onChange={({ checked }) => airplane.set(checked)} accessibilityLabel="Airplane mode" />

  <Switch disabled checked>Managed by your organization</Switch>
</View>
```

Rust：

```rust
use nsh_core::prelude::*;
use nsh_longbridge::{Size, switch};

let airplane = signal(false);

view()
    .gap_3()
    .child(
        switch()
            .checked(move || airplane.get())
            .on_change(move |checked: &bool| airplane.set(*checked))
            .child(text("Airplane mode")),
    )
    .child(
        switch()
            .size(Size::Xs)
            .checked(move || airplane.get())
            .on_change(move |checked: &bool| airplane.set(*checked))
            .accessibility_label("Airplane mode"),
    )
```

## 9. NSH 能力缺口

1. **部件不跟随 focus-visible。** 同 [checkbox.md §9](checkbox.md#9-nsh-能力缺口) 缺口 1；`Switch` 已投 active，只缺 focus-visible。
2. **`checked:` 投影在禁用时不生效。** 同 Checkbox，颜色用绑定。
3. **没有 `invalid` / `accessibilityDescribedBy`。** `nsh/base` 的 `Switch` 两项都没有（`Checkbox` 有）。本库不做
   invalid，不需要绕；以后要做，读屏部分得等 `nsh/base` 加。
