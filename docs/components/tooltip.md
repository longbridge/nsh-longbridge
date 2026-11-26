# Tooltip

提示气泡：指针停在一个元素上、或键盘焦点移到它上面时，在它旁边出现一句说明。本库第一个弹层。
对照 shadcn/ui new-york v4 `tooltip.tsx`（Radix Tooltip）。

## 用途

- 图标按钮的名字（"Refresh"）、被截断的文字的全文、一个控件的快捷键（配 `Kbd`）。
- 搜：提示、悬停提示、气泡提示、工具提示；tooltip, hint, hover hint。

何时不用：

- **必须看到的信息**：触屏上基本不出（见「触屏」），读屏也未必念。说明文字直接写在界面上，或用 `Popover`（P2）
  做"点一下看说明"。
- 里面要放可点的东西（链接、按钮）：提示不接指针，点不到；用 `Popover`（P2）或以后的 `HoverCard`。
- 多段内容、图片、预览卡片：以后的 `HoverCard`。
- 表单校验错误：写在字段下面（P3 的 Field），不用提示。
- 代替读屏名：图标按钮的读屏名写在按钮自己的 `accessibilityLabel` 上，提示只是给看得见的人的。

## 基于

`nsh/base` 的 `Tooltip`（`crates/nsh-base/src/tooltip.rs`），本库只加外观和动效，行为全用它的：

| `nsh/base` 已经提供（本库不重写） | |
|---|---|
| 出现 | 指针在元素上停 `delay` 毫秒后出现；离开或按下就消失 |
| 连读 | 指针离开一个**已经出了提示**的元素后 300 ms 内进入另一个元素，那一个立刻出（核心固定的宽限期，等于 Radix 的 `skipDelayDuration` 默认值），工具条上的按钮可以一个个扫过去；按下或 Escape 结束宽限期 |
| 键盘 | 键盘把焦点移到元素上（Tab、方向键、快捷键、按键后应用的 `focus()`）时**立刻**出现，焦点离开消失；点击带来的焦点不算（同 `focus-visible:`），所以点完按钮提示不会赖着不走；Escape 关掉 |
| 位置 | 放在元素的 `placement` 那一侧，那一侧放不下而对侧放得下时翻到对侧；沿边按 `align` 对齐；离元素 `offset`，离安全区边缘至少 `margin`；在所有内容之上 |
| 不挡操作 | 提示不接指针（`pointer-events: none`），元素照常可点、可悬停 |
| 受控 | `open` / `defaultOpen` / `onOpenChange`，`disabled` |
| 出入场 | `startingClass` / `endingClass`：出现时从起始外观过渡到常态，消失时过渡到结束外观再卸载；用户要求减弱动态效果时跳过过渡 |
| 触屏 | 没有悬停；只在硬件键盘把焦点移到元素上时出现，否则只有应用设 `open` 才出 |
| 终端 | 在终端里显示成状态行的一行字，用终端的颜色，本库外观不生效 |

### 和 shadcn 的结构不同

shadcn（Radix）是 `TooltipProvider > Tooltip > TooltipTrigger + TooltipContent` 四件套。NSH 的提示是**元素的 `tooltip`
属性**（同 GPUI 的 `.tooltip(..)`）：触发元素就是带这个属性的元素，提示本身是属性的值。本库照 NSH 的模型：

- 没有 `TooltipTrigger`：任何元素（本库的 `Button`、应用的 `View`）写 `tooltip={<Tooltip>…</Tooltip>}` 就是触发元素。
- 没有 `TooltipContent`：`Tooltip` 本身就是气泡。
- 没有 `TooltipProvider`：shadcn 用它统一设 `delayDuration`，本库用 token `tooltip-delay` 统一设，宽限期由 NSH 固定。

这样做是因为 NSH 只认元素 `tooltip` 属性里的提示（核心把它当成挂在元素上的提示层），放在 children 里的提示不会被
跟踪悬停和焦点。

## API

### 挂法

```tsx
<Button variant="ghost" size="sm" accessibilityLabel="Refresh" tooltip={<Tooltip>Refresh</Tooltip>}>
  <Icon name="refresh-cw" />
</Button>
```

```rust
button().variant(ButtonVariant::Ghost).accessibility_label("Refresh")
    .tooltip(tooltip().child("Refresh"))
    .child(icon("refresh-cw"))
```

TSX 里 `tooltip` 是每个组件、每个 `View` 都有的宿主属性（`ComponentHostProps`）；Rust 里是 `Interactive::tooltip`，
本库的 `TooltipElement` 实现 `IntoTooltip`（见「实现」）。

**挂在可聚焦的那一层**：提示跟的是"焦点落在这个元素或它里面"。挂在按钮外层的 `View` 上没问题；挂在按钮**里面**的
图标上，键盘聚焦到按钮时不会出（焦点在图标外面）。

### Props

| TSX | Rust | 类型 | 默认 | 说明 |
|---|---|---|---|---|
| `children` | `.child(..)` | 文字、`Kbd`、元素 | | 说的话；字符串包成 `label` 部件 |
| `placement` | `placement` | `"top"` \| `"bottom"` \| `"left"` \| `"right"`（`nsh_core::overlay::Placement`） | `"top"` | 放在哪一侧；放不下时自动翻到对侧 |
| `align` | `align` | `"start"` \| `"center"` \| `"end"`（`Align`） | `"center"` | 沿那一侧怎么对齐；`start` / `end` 随布局方向（RTL 反过来） |
| `delay` | `delay` | `number`（毫秒） | `tooltip-delay`（500） | 指针停多久出现；`0` 立刻；`Infinity` 指针和焦点都不触发，只有应用设 `open` 才出。键盘焦点总是立刻出，不看它（除了 `Infinity`） |
| `open` | `open` | `boolean` | 不受控 | 受控：应用持有，在 `onOpenChange` 里改 |
| `defaultOpen` | `default_open` | `boolean` | `false` | 不受控时一开始是否显示 |
| `disabled` | `disabled` | `boolean` | `false` | 不出提示，也不发 `onOpenChange` |
| `class` / `style` | `class` / `style` | | | 气泡（根） |
| `labelClass` / `labelStyle` | `label_class` / `label_style` | | | 字符串 children 生成的那段文字 |

`offset`（离元素的距离）、`margin`（离屏幕边的距离）不开属性：分别是 token `tooltip-offset` 和 `nsh/base` 的默认 4。
shadcn 开放了 `sideOffset`，但全应用应该一致，要改走 token。

`startingClass` / `endingClass` 不开放：出入场是本库的外观，由本库设（见「动效」）；要关掉动效，作用域里设
`tooltip-animate: 0`（见 token 表）。

### 事件

| TSX | Rust（`TooltipEvent`） | 参数 | 何时 |
|---|---|---|---|
| `onOpenChange` | `OpenChange(open, reason)` | `{ open: boolean, reason: "hover" \| "focus" \| "cancel" }` | 用户要求打开或关闭：指针停够 / 离开 / 按下（`"hover"`），键盘焦点来去（`"focus"`），Escape（`"cancel"`）。应用自己改 `open` 不报 |
| `onOpen` | `Open` | | 已经显示出来 |
| `onClose` | `Close` | | 已经完全消失（出场动画之后），不管谁关的 |

全部是 `nsh/base` `Tooltip` 的事件原样转出。

### 部件

| 部件 | 属性 | 是什么 |
|---|---|---|
| 气泡 | `class` / `style` | 根：底色、圆角、内边距 |
| 文字 | `labelClass` / `labelStyle` | 字符串 children 生成的 `text`；children 是元素时不生成，这对属性不起作用 |

没有箭头部件（评审已定，理由见「已定」1）。

## 变体和尺寸

- 无 `variant`：shadcn 只有一种（反色气泡）。
- 无 `size`（`xs` / `sm` / `md` / `lg` 都不支持）：提示的大小不跟触发元素走。

## 外观

shadcn：`w-fit rounded-md bg-foreground px-3 py-1.5 text-xs text-balance text-background`，加一个
`size-2.5 rotate-45 rounded-[2px] bg-foreground` 的箭头，`sideOffset` 0（箭头填在空隙里）。

| | 值 |
|---|---|
| 底 / 字 | `tooltip` / `tooltip-foreground`：默认反色（亮色主题里黑底白字，暗色主题里白底黑字），照 shadcn |
| 圆角 | `tooltip-radius`（`radius-md`，8） |
| 内边距 | 左右 `tooltip-padding-x`（12），上下 `tooltip-padding-y`（6） |
| 字 | `tooltip-font-size`（12），行高由 foundation 的「字号 → 行高」给（12 → 16），常规字重 |
| 布局 | 横排、竖向居中、间隔 `spacing` × 2（8）：文字后面跟 `Kbd` 时有间隔 |
| 宽 | 随内容，最宽 `tooltip-max-width`（320），超出折行 |
| 离元素 | `tooltip-offset`（4） |
| 投影 | 无（shadcn 无） |
| 单行高 | 16 + 6 × 2 = 28 |

和 shadcn 的差别：

- **没有箭头**，所以 `offset` 是 4 而不是 0。理由见「已定」1。
- **有最宽**：shadcn 不限宽（靠 `text-balance` 让多行均匀）。不限宽时一句长说明会横着铺到窗口边（实际应用里出现过：长说明
  冲出窗口）。320 在 12 号字下约 24 个汉字一行。NSH 有没有 `text-balance` 实现时查，没有就按普通折行。
- 字重常规，同 shadcn（gpui-kit 用 medium，不跟）。

### 动效

| | shadcn（tw-animate-css） | 本库 |
|---|---|---|
| 出现 | `fade-in-0 zoom-in-95`，再按实际所在侧 `slide-in-from-*-2`（8），150 ms | 从 `opacity-0 scale-95` 过渡到常态，`duration`（150 ms），不滑 |
| 消失 | `fade-out-0 zoom-out-95` | 过渡到 `opacity-0 scale-95` 后卸载 |
| 缩放原点 | Radix 算的、朝向触发元素的点 | 气泡中心 |
| 减弱动态效果 | 不处理 | 不过渡，立刻出、立刻消失（`nsh/base` 自带） |

不滑入、原点取中心：两者都要知道气泡**最终放在哪一侧**（可能翻到对侧），而 `nsh/base` 的 `Tooltip` 不报位置
（`Popover`、`HoverCard` 有 `onPosition`，`Tooltip` 没有），按请求的 `placement` 滑会在翻转时滑反。见「缺口」。

连读时（宽限期内从一个提示换到下一个），新提示也走一遍出现的过渡；gpui-kit 那种"气泡从上一个位置滑到下一个"不做，
`nsh/base` 没有这个能力。

## 状态

| 状态 | 外观 |
|---|---|
| 出现中 / 消失中 | 见「动效」 |
| 触发元素 hover / focus-visible | 提示不改变触发元素的外观；那是触发元素自己的状态 |
| 触发元素 disabled | 照 `nsh/base`：提示跟的是指针和焦点，和元素禁不禁用无关；禁用按钮多半拿不到焦点，所以只有悬停能出。想解释"为什么不能点"可以挂提示；不想出就给 `disabled`。实现时用 headless 测试确认禁用的 `Button` 上悬停能出提示 |

没有 invalid / loading。

## 指针与触屏

| | 桌面（含桌面浏览器） | 手机、平板 |
|---|---|---|
| 怎么出 | 指针停 `delay`；键盘焦点立刻 | 硬件键盘把焦点移过来时立刻；否则只有应用设 `open` |
| 怎么走 | 指针离开、按下；焦点离开；Escape | 焦点离开；Escape；应用设 `open=false` |
| 长按 | — | **不出**。NSH 不做，Radix、iOS、Android 的系统控件也都没有"长按出提示"（Android 的长按提示是给工具条图标的系统行为，不归应用画）；长按在触屏上另有含义（上下文菜单、拖动） |
| 外形 | 气泡 | 同一个气泡，尺寸不变（只在接了键盘时出，用户离屏幕的距离和桌面相近） |
| 命中区 | 就是触发元素自己 | 同 |

所以在手机上**几乎看不到提示**：所有靠提示说明的东西，在触屏上要有别的出路（读屏名、界面上的文字、Popover）。
不要用受控的 `open` 在点按时打开提示来模拟"点一下看说明"——点按同时会触发元素自己的动作，而且下一次按下又会
请求关掉它；这种需求用 `Popover`（P2）。

## 键盘与读屏

- 提示不是 Tab 站，不拿焦点；Tab 照常在触发元素之间走，每停到一个就出它的提示。
- Escape 关掉当前提示（`onOpenChange` 的 `reason` 是 `"cancel"`），焦点不动。
- 读屏：见「缺口」第 2 条。在确认之前，本库的要求是：**触发元素自己写全读屏名**（图标按钮的 `accessibilityLabel`），
  需要补充说明的写 `accessibilityHint`（`nsh/base` 的 `Button` 有），不指望读屏念提示。

## Token

| token | 类型 | 推导默认 | 亮 / 暗 |
|---|---|---|---|
| `tooltip` | 颜色 | `foreground` | 亮：近黑；暗：近白（跟着 `foreground` 反过来，shadcn 的反色气泡） |
| `tooltip-foreground` | 颜色 | `background` | 同上，反过来 |
| `tooltip-radius` | 长度 | `radius-md`（8） | 同 |
| `tooltip-padding-x` | 长度 | `spacing` × 3（12） | 同 |
| `tooltip-padding-y` | 长度 | `spacing` × 1.5（6） | 同 |
| `tooltip-font-size` | 长度 | 12（Tailwind `text-xs`） | 同 |
| `tooltip-max-width` | 长度 | 320 | 同 |
| `tooltip-offset` | 长度 | `spacing`（4） | 同 |
| `tooltip-delay` | 数字 | 500（毫秒） | 同 |
| `tooltip-animate` | 数字 | 1（1 = 有出入场，0 = 没有） | 同 |

- `tooltip-font-size` 用 Tailwind 的固定 12，不引用 `font-size-xs`：提示是说明文字，不是控件，tokens.md 规定说明文字
  用 Tailwind 的值。
- 想要 gpui-kit / 系统风格的浅色提示（`popover` 底、`border` 边）：主题里写 `"tooltip": "var(--popover)"`、
  `"tooltip-foreground": "var(--popover-foreground)"`，边用 `class="border"`。本库不另做变体。
- `tooltip-animate` 是开关，不是时长：时长跟 `duration` 种子走。设 0 时本库不给 `nsh/base` 传 `startingClass` /
  `endingClass`，出现消失都在同一帧完成。

## 实现

- **组件结构**：`#[component] fn Tooltip`，根就是 `nsh_base::tooltip()`，props 原样转给它，外观用 `theme()` 的 token 设在
  根上（根就是气泡）；`startingClass` / `endingClass` 由本库按 `tooltip-animate` 设；提供 `Kbd` 读的"在提示里"上下文。
- **TSX 能否被核心认成提示层**：核心认的是"`tooltip` 属性里的元素，根是带 `hint` 的 overlay"（`Core::adopt_hint_tooltips`）。
  本库的 `Tooltip` 是包了 `nsh/base` `Tooltip` 的组件，宿主节点会被改成根的类型（`Core::retag`，嵌套组件共用一个节点），
  按文档应能认出来。**P1 实现的第一个 headless 测试就测这个**：`<View tooltip={<Tooltip>…</Tooltip>}>` 悬停 500 ms 后
  出现、位置在元素上方、`delay` 生效。认不出来的话，退回方案是本库的 `Tooltip` 不做组件、做一个返回预设好外观的
  `nsh/base` `Tooltip` 的函数（TSX 用 `tooltipLook()` 给 `class`），在汇报里升级为 NSH 缺口。
- **Rust 的 `IntoTooltip`**：`ComponentElement<C>` 的通用实现会把它当普通元素，再包一层核心自己的提示层（位置、延迟用
  核心默认，本库的 props 全失效）。所以本库要给自己的 `TooltipElement` 单独实现 `IntoTooltip`，调用
  `nsh_core::element::__hint_tooltip`，同 `nsh/base` 的做法。这个函数是 `#[doc(hidden)]`，见「缺口」1。
- **作用域**：提示层挂在触发元素里（核心把它放进元素的 children），所以读得到触发元素所在的 `ThemeScope`：
  深色作用域里的按钮，提示也用深色的值。测试守住。

## 示例

```tsx
import { keymap } from "nsh";
import { Button, Icon, Kbd, Tooltip } from "nsh-longbridge";

// 图标按钮
<Button variant="ghost" size="sm" accessibilityLabel="Add to watchlist" tooltip={<Tooltip>Add to watchlist</Tooltip>}>
  <Icon name="plus" />
</Button>

// 带快捷键，放在下方
<Button
  variant="outline"
  tooltip={
    <Tooltip placement="bottom">
      Save <Kbd>{keymap.label("mod+s")}</Kbd>
    </Tooltip>
  }
>
  Save
</Button>

// 截断文字的全文，立刻出
<Text numberOfLines={1} tooltip={<Tooltip delay={0}>{fullName}</Tooltip>}>{fullName}</Text>

// 受控
const [open, setOpen] = useState(false);
<View tooltip={<Tooltip open={open} onOpenChange={(e) => setOpen(e.open)}>Synced 2 min ago</Tooltip>}>…</View>
```

```rust
use nsh_core::{element::key_label, overlay::Placement};
use nsh_longbridge::{button, kbd, tooltip, ButtonVariant, TooltipEvent};

button().variant(ButtonVariant::Outline)
    .tooltip(
        tooltip().placement(Placement::Bottom)
            .child("Save")
            .child(kbd().child(key_label("mod+s"))),
    )
    .child("Save")

let open = signal(false);
view().tooltip(
    tooltip().open(move || Some(open.get()))
        .on_open_change(move |e: &TooltipEvent| {
            if let TooltipEvent::OpenChange(next, _reason) = e {
                open.set(*next);
            }
        })
        .child("Synced 2 min ago"),
)
```

## NSH 缺口

1. **第三方 Rust 组件做提示要用隐藏函数**：`IntoTooltip::apply_tooltip` 和 `__hint_tooltip` 都是 `#[doc(hidden)]`，
   `ComponentElement` 的通用实现又会多包一层。绕过：照 `nsh/base` 调 `__hint_tooltip`，锁定 NSH 版本，升级时测试守住。
   希望 NSH 给一个公开的办法（例如"根是提示层的组件自动被认成提示"）。
2. **读屏念不念提示没确认**：Radix 用 `aria-describedby` 让读屏把提示当触发元素的描述念。NSH 文档没说提示是否进触发
   元素的读屏描述。绕过：本库要求触发元素自己写读屏名 / `accessibilityHint`（见「键盘与读屏」）；实现时在 macOS
   VoiceOver 上实测，结果写进汇报。
3. **`Tooltip` 没有 `onPosition`**：不知道翻转后实际在哪一侧，做不了指向元素的箭头、按侧滑入、朝向元素的缩放原点。
   绕过：不要箭头，不滑，原点取中心。希望 NSH 给 `Tooltip` 也报 `onPosition`（`Popover`、`HoverCard` 已有）。
4. **提示压在后来打开的弹层上**（P2 才会碰到，先记下）：实际应用里遇到过：按钮打开了 Popover / 菜单 / 对话框，
   焦点留在按钮上时，按钮的提示会出在弹层**之上**，Escape 也先被提示吃掉。应用可以自己数弹层层数绕过。P2 做
   `Popover` 时先在 NSH 上复现，再定是 NSH 修还是本库绕。
5. **带指针的 iPad 没有悬停提示**：`nsh/base` 把平板一律当触屏。iPadOS 接触控板时系统有指针提示。低优先，记下。

## 已定（评审结论）

1. **不要箭头，`offset` 4**。`Tooltip` 不报实际所在侧（缺口 3），翻转后箭头会指反；gpui-kit、NSH gallery 的提示
   也都没有箭头。等 NSH 给了 `onPosition` 再议，加的时候是一个 `arrowClass` / `arrowStyle` 部件。
2. **默认延迟 500 ms**（`tooltip-delay`）。shadcn 的 `TooltipProvider` 设成 0 是网页习惯；桌面上指针扫过工具条时不该一路
   闪提示，宽限期已经保证连读不用再等。截断全文这类场景在那一处写 `delay={0}`。
3. **配色照 shadcn 反色**（`foreground` 底）。浅色（`popover` 底 + 边）作为主题里两行 token 的改法写在 token 表下面。
4. **API 跟 NSH 的 `tooltip` 属性走**，不做 shadcn 的四件套。TSX 包装能否被核心认成提示层，是实现时的第一个 headless 测试。
