# Input

> 状态：API 单（P1，已评审，按 2026-11-26 结论修订）。外观用 foundation 的**字段外观**（P0 实现：边、底、阴影、焦点、
> invalid、readonly、disabled），读本文件 §7 的 `input-*` token；[Textarea](textarea.md) 和以后的 Select 触发器、NumberInput、
> Combobox 用同一份。

## 1. 用途

单行文本输入：姓名、邮箱、密码、搜索词、金额以外的短文本。外观是 shadcn/ui new-york v4 的 `Input`。

何时不用：

| 需要 | 改用 |
|---|---|
| 多行文本、备注、可换行的内容 | [Textarea](textarea.md) |
| 数字，带步进、范围、键盘上下调 | `nsh/base` 的 `NumberInput`（本库以后包一层，P3），不用 `Input` + `keyboardType="numeric"` 自己校验 |
| 从固定列表里选 | Select（P2）/ Combobox（P3） |
| 输入框前后带图标、单位、按钮 | InputGroup（shadcn v4 的 `input-group`，以后做）；P1 不给 `Input` 加 prefix / suffix |
| 验证码、一格一位 | `nsh/base` 的 `OtpInput` |
| 字段的标签、说明、错误行 | 标签用 [Label](label.md)；说明和错误行等 Form / Field（P3） |
| 文件选择（shadcn 的 `type="file"`） | 不支持：NSH 没有文件输入框，用系统的文件对话框 |

## 2. 基于什么

基于 NSH 的宿主元素 `Input`（TSX 从 `nsh/base` 导入，Rust 是 nsh-core element 模块的 `input()`；原生文本框：AppKit / UIKit / EditText / Win32 / GTK / 浏览器 `<input>`）。
`nsh/base` 没有「输入框」组件，`NumberInput` 内部也是直接用这个元素，本库照它的做法包一层：组件的根就是这个
`input()` 元素本身，不在外面套 view（`class` / `style` 直接落在输入框上，和 shadcn 一样只有一个元素）。

宿主元素已经提供、本库**不重写**的行为：

- 编辑本身：光标、选区、输入法组合（组合期间 `value` 由原生端暂存，结束后再应用）、撤销、复制粘贴、拖拽选择、
  平台的编辑菜单、拼写检查、自动填充。
- 受控：每次原生编辑带编辑计数，应用回写的 `value` 只在基于最新编辑时才应用，快速打字不会回跳。
- 键盘：可编辑的输入框永远是 Tab 停靠点；Return 按 `submitBehavior` 触发 `onSubmit`；编辑键归输入框，祖先的快捷键截不走。
- 焦点：`focus-visible:` 对**可编辑的输入框聚焦时总是成立**（NSH 的规则：输入框不管是点进来的还是 Tab 进来的都显示焦点），
  所以本库的焦点环就挂在 `focus-visible:`，和 shadcn 一致，不用再区分键盘 / 指针。
- 读屏：文本框角色、值、`accessibilityLabel`；`invalid` 状态各平台都映射（macOS `AXInvalid`、Windows `IsDataValidForForm`、
  AT-SPI invalid entry、Android `setContentInvalid`、Web `aria-invalid`，iOS 读在值里）；`disabled` 读作变暗。
- 软键盘：`keyboardType`、`returnKeyType`、`autoCapitalize`、`autoCorrect`；滚动区里的输入框被键盘挡住时原生自动让开。

本库只加：外观（token）、`size`、`invalid` 的外观和读屏状态、`disabled` 的整套处理（照 `NumberInput`：
`readonly` + `disabled` 一起设，不可编辑、不可聚焦、`disabled:` 生效）。

## 3. API

### Props

| TSX | Rust（字段 / setter） | 类型 | 默认 | 说明 |
|---|---|---|---|---|
| `value` | `value` | `string` / `Prop<Option<SharedString>>` | — | 受控值。写了就受控：应用持有，`onChange` 报变化，应用回写。不写则非受控，从 `defaultValue` 起 |
| `defaultValue` | `default_value` | `string` | `""` | 非受控时的初值 |
| `placeholder` | `placeholder` | `string` | — | 空时显示的提示（应用的文案） |
| `size` | `size` | `"xs" \| "sm" \| "md" \| "lg"` / `Size` | `"md"` | 见 §4 |
| `disabled` | `disabled` | `boolean` | `false` | 不可编辑、不可聚焦、不在 Tab 顺序里；外观见 §5 |
| `readonly` | `readonly` | `boolean` | `false` | 可聚焦、可选中复制，不可改；外观同常态 |
| `invalid` | `invalid` | `boolean` | `false` | 应用的校验没通过：红边，聚焦时红环；读屏报「无效」（`accessibilityState.invalid`，`aria-invalid:` 类生效） |
| `masked` | `masked` | `boolean` | `false` | 密码框（shadcn 的 `type="password"`） |
| `keyboardType` | `keyboard_type` | NSH 的 `KeyboardType` | `"default"` | 软键盘类型（`email-address`、`url`、`phone-pad`…），原样透传 |
| `returnKeyType` | `return_key_type` | NSH 的 `ReturnKeyType` | 平台默认 | 原样透传 |
| `autoCapitalize` | `auto_capitalize` | NSH 的 `AutoCapitalize` | 平台默认 | 原样透传 |
| `autoCorrect` | `auto_correct` | `boolean` | 平台默认 | 原样透传 |
| `autoComplete` | `auto_complete` | NSH 的 `AutoFill` | — | 自动填充的种类（用户名、密码、邮箱…），原样透传（宿主 Rust 名 `auto_fill`） |
| `maxLength` | `max_length` | `number` | — | 原样透传 |
| `autoFocus` | `auto_focus` | `boolean` | `false` | 原样透传；`disabled` 时忽略 |
| `selectTextOnFocus` | `select_text_on_focus` | `boolean` | `false` | 原样透传 |
| `accessibilityLabel` | `accessibility_label` | `string` | — | 读屏名。**有可见标签时也要写**（NSH 没有元素间的 labelledby，见 §9 G1） |
| `class` | `class` | `string` | — | 根（就是输入框）的 Tailwind 类，最后合并，盖过组件的 |
| `style` | `style` | `StyleProp<InputStyle>` / `InputPartStyle` | — | 根的内联样式，盖过 `class`；可写 `placeholderColor`、`caretColor`、`selectionColor` |

没有 `type` 属性：shadcn 靠 HTML `type` 区分的东西在 NSH 里是独立的属性（`masked`、`keyboardType`、`autoComplete`），照 NSH 的名字。
没有 `variant`：shadcn 的 Input 只有一种外观。

### 事件

| TSX | Rust（`InputEvent` 变体 / setter） | 数据 | 说明 |
|---|---|---|---|
| `onChange` | `Change(String)` / `.on_change(move \|value: &String\| ..)` | `{ value }` | 每次编辑 |
| `onSubmit` | `Submit(String)` / `.on_submit(..)` | `{ value }` | Return（按 `returnKeyType`），输入法提交组合的那次回车不算 |
| `onFocus` | `Focus` / `.on_focus(..)` | — | 得到键盘焦点 |
| `onBlur` | `Blur` / `.on_blur(..)` | — | 失去键盘焦点 |

宿主元素还有 `onSelectionChange`、`onCaretMove`、`onPaste`、`onKeyPress`、`onCompositionChange` 等，P1 不透传
（不是 shadcn Input 的常用面）；需要它们的应用 P1 先直接用 `nsh/base` 的 `Input`，见 §8 末尾。
以后有真实需要再按名字原样加，不改名。

### Ref

TSX `let email: InputRef | undefined;` 再 `ref={email}`（NSH 的 ref 写法）拿到 `InputRef`；Rust `ComponentRef<InputApi>`。

| 方法 | 说明 |
|---|---|
| `focus()` | 聚焦（弹出软键盘）；`disabled` 时不动 |
| `blur()` | 失焦 |
| `setSelection(start, end)` | 选区，UTF-16 下标，同宿主 |
| `insertText(text)` | 在光标处插入，同宿主 |

### 部件

只有根一个元素，没有 `<部件>Class` / `<部件>Style`。占位文字、光标、选区的颜色是输入框的样式键
（`placeholderColor` / `caretColor` / `selectionColor`，或类 `placeholder:text-*` / `caret-*` / `selection:bg-*`），写在根的 `class` / `style` 里。

## 4. 变体与尺寸

无变体。尺寸四档都支持，高度、字号、左右内边距一起变：

| `size` | 高（指针 / 触屏） | 字号（指针 / 触屏） | 左右内边距 | 对照 |
|---|---|---|---|---|
| `xs` | 24 / 32（`control-height-xs` / `-touch-xs`） | 12 / 14 | 8（spacing × 2） | 表格内编辑、工具栏里的过滤框 |
| `sm` | 28 / 36 | 13 / 14 | 10（spacing × 2.5） | 紧凑表单、侧栏 |
| `md` | 32 / 40 | 14 / 16 | 12（spacing × 3） | 默认 |
| `lg` | 36 / 48 | 14 / 16 | 12（spacing × 3） | 指针下 = shadcn 的 `h-9 px-3 md:text-sm` |

和 shadcn 的差别：shadcn 只有一档 `h-9`（36）。本库默认 `md` 是 32，和 Button 的 `md` 同高（桌面中等密度，gpui-kit
Design Guides）；要 shadcn 原样的高度用 `size="lg"`，或主题里 `control-height: 36`。同一行里的 Input 和 Button 用同一个
`size` 就对齐。触屏 md / lg 用 16 号字是 conventions.md 的规定（iOS Safari 对小于 16 的输入框会放大页面，也同 shadcn 窄屏的 `text-base`）。

宽度：`w-full min-w-0`（占满父级、在 flex 里可以缩），同 shadcn；要固定宽用 `class`。
圆角所有尺寸都是 `input-radius`（与 Button 同档；若 P0 的 Button 给 `xs` 用了更小的圆角，Input 跟随同一规则）。
单行文字在固定高度里垂直居中，不设行高。

## 5. 状态

外观由 foundation 的**字段外观**画（P0 实现，conventions.md「外观实现」），本组件只把 `size`、`invalid`、`readonly`、`disabled`
和焦点交给它。下表是用默认 token 时应得到的结果，也是展示台和截图验收的口径：

| 状态 | 边 | 背景 | 阴影 / 环 | 其他 | shadcn 原类 |
|---|---|---|---|---|---|
| 常态 | `input-border-width`（1）`input-border` | `input-background`（亮：透明；暗：`input` 30%） | `input-shadow` = 1（`xs`） | 文字 `input-foreground`，占位 `input-placeholder-foreground` | `border border-input bg-transparent shadow-xs dark:bg-input/30 placeholder:text-muted-foreground` |
| hover | 不变（没有悬停态，同 shadcn） | 不变 | 不变 | 光标 I 形（原生） | 无 |
| focus-visible | `input-border-focus`（`ring`） | 不变 | 外环 `input-ring-width`（3）`input-ring`（`ring` 50%），画在框外，叠在 `xs` 阴影上 | 触屏只换边色，不画外环 | `focus-visible:border-ring focus-visible:ring-[3px] focus-visible:ring-ring/50` |
| invalid | `input-border-invalid`（`destructive`） | 不变 | 不聚焦时无环 | 读屏「无效」 | `aria-invalid:border-destructive` |
| invalid + focus-visible | `input-border-invalid`（红边压过 `ring` 边） | 不变 | 外环 3 `input-ring-invalid`（= `ring-invalid`：`destructive` 20%，暗 40%）；触屏无外环 | — | `aria-invalid:ring-destructive/20 dark:aria-invalid:ring-destructive/40` |
| readonly | 同常态 | 同常态 | 聚焦时同 focus-visible | 可聚焦、文字可选中复制，不可改 | 无（shadcn 不区分） |
| disabled | 同常态 | 同常态 | 无环（不可聚焦） | 整体 `opacity-disabled`（0.5），光标箭头 | `disabled:pointer-events-none disabled:cursor-not-allowed disabled:opacity-50` |

本组件这边要注意的：

- 优先级（后者胜）：常态 < focus-visible < invalid < disabled。invalid 只换边和环的颜色，环仍然只在聚焦时出现。
- NSH 的输入框只要聚焦就是 `focus-visible`（点进来的也算），所以指针设备上点进输入框也出外环，同浏览器。
- 外环画在框外、不占布局。贴着滚动区、对话框正文、`overflow-hidden` 容器边缘的输入框，环会被裁掉一截（实际应用里出现过）：
  由容器留出 `ring-width` 的内边距，展示台放一个「贴边」的例子验收。
- 过渡：边色、背景、阴影 / 环在 `duration` 内过渡，减弱动态效果时归零。shadcn 只过渡 `color, box-shadow`、边色瞬变；
  NSH 的过渡按「颜色 / 阴影」分组拆不开，边色和环一起过渡。
- 选区：shadcn Input 写了 `selection:bg-primary selection:text-primary-foreground`。NSH 只能设选区底色、不能设选中文字的颜色，
  各平台还各自加透明度（Android 55%、Linux 30%、iOS 用 tint），照抄会在亮色下出现深底深字。所以**默认不设 `selectionColor`，用平台的**，
  见 §9 G4。Textarea 在 shadcn 里本来就没设。
- 光标：shadcn 不设，本库也不设（平台默认）。
- 加载中：输入框没有 loading 态（shadcn 没有）；异步校验的转圈放在 InputGroup 里（以后）。

## 6. 指针与触屏

| | 指针（桌面、Web 桌面） | 触屏 |
|---|---|---|
| 高 | `control-height-*`：24 / 28 / 32 / 36 | `control-height-touch-*`：32 / 36 / 40 / 48 |
| 字号 | 12 / 13 / 14 / 14 | 14 / 14 / 16 / 16 |
| 命中区 | 外观即命中区 | 高小于 `touch-target`（44）时上下补命中区（`hitSlop` = (44 − 高) / 2），外观不变 |
| 焦点 | 边换 `ring` + 3pt 外环 | 只换边色，不画外环（照 iOS / Android 系统文本框；iPad 接实体键盘也一样） |
| 光标 | I 形（原生）；disabled 箭头 | — |

指针 / 触屏的取值用 foundation 的 `pointer_touch` 推导。软键盘、键盘让位、编辑菜单都是原生的，本库不管。

## 7. 组件 token

命名照 tokens.md。这些是字段外观的全部旋钮，Textarea 读同一组（不另起 `textarea-border` 这类名字），主题里改 `input-border`
两者一起变；只想改其中一个时用 `ThemeScope` 或 `class`。指针 / 触屏取值不同的 token，应用要单独改触屏的值时写 `<名字>-touch`。

| token | 类型 | 推导默认值 | 亮 / 暗、指针 / 触屏 |
|---|---|---|---|
| `input-radius` | 长度 | `radius-md`（8） | — |
| `input-height-xs` / `-sm` / `-md` / `-lg` | 长度 | `pointer_touch(control-height-xs, control-height-touch-xs)` 等（24 / 28 / 32 / 36；触屏 32 / 36 / 40 / 48） | 分指针 / 触屏 |
| `input-padding-x-xs` / `-sm` / `-md` / `-lg` | 长度 | `spacing` × 2 / × 2.5 / × 3 / × 3（8 / 10 / 12 / 12） | — |
| `input-font-size-xs` / `-sm` / `-md` / `-lg` | 长度 | 指针 `font-size-xs/sm/md/lg`（12 / 13 / 14 / 14）；触屏 14 / 14 / 16 / 16 | 分指针 / 触屏 |
| `input-border-width` | 长度 | `border-width`（1） | — |
| `input-border` | 颜色 | `input` | 同 `input` |
| `input-border-focus` | 颜色 | `ring` | 同 `ring` |
| `input-border-invalid` | 颜色 | `destructive` | 同 `destructive` |
| `input-background` | 颜色 | `light_dark(透明, input 的 30%)` | 分亮暗 |
| `input-foreground` | 颜色 | `foreground` | 同 `foreground` |
| `input-placeholder-foreground` | 颜色 | `muted-foreground` | 同 `muted-foreground` |
| `input-ring` | 颜色 | `ring` × `ring-opacity`（50%） | 同 `ring` |
| `input-ring-invalid` | 颜色 | `ring-invalid`（`destructive` 20%，暗 40%） | 同 `ring-invalid` |
| `input-ring-width` | 长度 | `ring-width`（3） | — |
| `input-shadow` | 数字 | 1（`xs`） | — |

触屏字号 14 / 16 是 `font-size` 种子之外的固定值：tokens.md 没有触屏字号的语义 token，所以写在这里的推导里；
主题要改用 `input-font-size-md-touch` 这类名字。disabled 的透明度直接用语义 token `opacity-disabled`，不另设（和 Button 一致）。

Rust 声明（示意，名字和上表一一对应）：

```rust
tokens! {
    /// The field's corner radius.
    INPUT_RADIUS: Length = "input-radius" => |t| t.length(RADIUS_MD),
    /// The field's height at `Size::Md`.
    INPUT_HEIGHT_MD: Length = "input-height-md" =>
        |t| t.pointer_touch(t.length(CONTROL_HEIGHT_MD), t.length(CONTROL_HEIGHT_TOUCH_MD)),
    /// The text size at `Size::Md`: 16 on touch screens, where smaller fields get zoomed.
    INPUT_FONT_SIZE_MD: Length = "input-font-size-md" => |t| t.pointer_touch(t.length(FONT_SIZE_MD), px(16.0)),
    /// The field's edge at rest.
    INPUT_BORDER: Color = "input-border" => |t| t.color(INPUT),
    /// The field's fill: none in light, `input` at 30% in dark.
    INPUT_BACKGROUND: Color = "input-background" => |t| t.light_dark(TRANSPARENT, t.color(INPUT).opacity(0.3)),
    /// The edge while the field has the keyboard focus.
    INPUT_BORDER_FOCUS: Color = "input-border-focus" => |t| t.color(RING),
    /// The edge while the app's checks reject the value; wins over the focus edge.
    INPUT_BORDER_INVALID: Color = "input-border-invalid" => |t| t.color(DESTRUCTIVE),
    /// The focus ring around the field.
    INPUT_RING: Color = "input-ring" => |t| t.color(RING).opacity(t.number(RING_OPACITY)),
    /// The focus ring while the value is invalid.
    INPUT_RING_INVALID: Color = "input-ring-invalid" => |t| t.color(RING_INVALID),
    // …其余同表
}
```

## 8. 示例

TSX：

```tsx
import { signal, View } from "nsh";
import { Input, type InputRef, Label } from "nsh-longbridge";

function EmailField() {
  const email = signal("");
  const invalid = () => email().length > 0 && !email().includes("@");
  let field: InputRef | undefined;
  return (
    <View class="gap-2">
      <Label htmlFor={field}>Email</Label>
      <Input
        ref={field}
        value={email()}
        onChange={({ value }) => email.set(value)}
        placeholder="name@example.com"
        keyboardType="email-address"
        autoComplete="email"
        autoCapitalize="none"
        invalid={invalid()}
        accessibilityLabel="Email"
      />
    </View>
  );
}

// 尺寸、密码、只读、禁用
<Input size="sm" placeholder="Filter" />
<Input masked autoComplete="password" accessibilityLabel="Password" />
<Input readonly value={accountId()} accessibilityLabel="Account ID" />
<Input disabled value="Read from server" />
```

Rust：

```rust
use nsh::prelude::*;
use nsh_longbridge::{input, label, Size};

fn email_field() -> impl IntoElement {
    let email = signal(String::new());
    let invalid = move || email.with(|e| !e.is_empty() && !e.contains('@'));
    let field = ComponentRef::new();
    view()
        .gap_2()
        .child(label().html_for(field).child("Email"))
        .child(
            input()
                .component_ref(field)
                .value(move || email.get())
                .on_change(move |value: &String| email.set(value.clone()))
                .placeholder("name@example.com")
                .keyboard_type(KeyboardType::EmailAddress)
                .invalid(invalid)
                .accessibility_label("Email"),
        )
}

// 尺寸
input().size(Size::Sm).placeholder("Filter")
```

需要宿主元素的全部能力（`onPaste`、`selectedRange`、`onCaretMove`…）的应用，P1 先直接用 `nsh/base` 的 `Input` 自己写样式；
TSX 是否导出一份字段外观（`fieldLook`）等 P1 实现后再定。

TSX 里本库的 `Input` 和 `nsh/base` 的 `Input` 同名，两个都要用时照 conventions.md 起别名（`import { Input as BaseInput } from "nsh/base"`）。
Rust 同理：`nsh_longbridge::input` 和 nsh-core 的 `element::input` 同名，不要两个都 glob 引入。

## 9. NSH 能力缺口与已定的取舍

### 缺口

- **G1 没有元素间的 labelledby。** NSH 没有一个元素引用另一个元素做读屏名的关系（NSH 自己的 API 命名文档也这么说）。
  绕过：Input 的 `accessibilityLabel` 由应用写，和可见的 Label 同文；文档和展示台示例都这样写。Label 的处理见 [label.md](label.md)。
- **G4 选区只能设底色。** 不能设选中文字颜色，选区透明度各平台不同，iOS 的选区和光标共用一个 tint。
  绕过：默认不设 `selectionColor`，用平台默认；应用要品牌色自己在 `style` 里写。

### 已定（2026-11-26 评审）

- 触屏字号：md / lg 16，xs / sm 14。
- 焦点环画在框外；触屏上只换边色，不画外环。
- Textarea 共用 `input-*` token。
- 输入框没有悬停态。
- 字段外观的实现在 foundation（P0），本文件引用它并保留 token 表。
- 有 `readonly` 状态。TSX 的 `fieldLook` 暂不做。
