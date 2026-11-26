# Textarea

> 状态：API 单（P1，已评审，按 2026-11-26 结论修订）。外观是 foundation 的**字段外观**，读 [Input](input.md) 的 `input-*` token；
> 本文件只写多行特有的部分。

## 1. 用途

多行文本输入：备注、描述、反馈、消息草稿。外观是 shadcn/ui new-york v4 的 `Textarea`：和 Input 同一副框，
高度随内容长（shadcn 的 `field-sizing-content`），最矮 64。

何时不用：

| 需要 | 改用 |
|---|---|
| 一行就够（名字、邮箱、搜索词） | [Input](input.md) |
| 富文本、Markdown 编辑、代码 | `nsh/base` 的 `RichEditor` / `Editor` |
| 聊天输入框（发送键、附件、回车发送） | 以后的 Composer 一类组合块（`src/blocks/`），不在 P1；临时用 Textarea + `submitBehavior="submit"` |
| 字数统计、说明、错误行 | 等 Form / Field（P3） |

## 2. 基于什么

同 Input：NSH 的宿主元素 `Input`，打开 `multiline`（原生 UITextView / NSTextView / EditText 多行 / `<textarea>`…）。
组件的根就是这个元素。宿主提供、本库不重写的行为见 [Input §2](input.md#2-基于什么)，多行另外有：

- Return 换行；`submitBehavior="submit"` 时 Return 提交，Shift / Option / Alt+Return 仍换行；输入法提交组合的回车不提交。
- 内容超过框高时在框内滚动（原生），滚轮先给多行输入框。
- `onContentSizeChange`：每次文本或宽度变化后的内容尺寸，本库用它做「随内容长高」（见 §5）。

## 3. API

### Props

与 [Input 的 props](input.md#props) 相同的这些，含义、类型、默认都一样：
`value`、`defaultValue`、`placeholder`、`size`、`disabled`、`readonly`、`invalid`、`autoCapitalize`、`autoCorrect`、
`maxLength`、`autoFocus`、`accessibilityLabel`、`class`、`style`。

不要的：`masked`、`keyboardType`、`returnKeyType`、`autoComplete`、`selectTextOnFocus`（多行文本用不到，或在多行上各平台不一致）。

多行特有：

| TSX | Rust | 类型 | 默认 | 说明 |
|---|---|---|---|---|
| `submitBehavior` | `submit_behavior` | NSH 的 `SubmitBehavior`：`"newline"` / `"submit"` / `"blurAndSubmit"` | `"newline"` | Return 做什么，原样透传（同宿主的规则） |

没有 `rows` / `minRows` / `maxRows`：shadcn 不用 `rows`，靠 `min-h-16` 加内容撑高；最大高度由应用用 `class`（`max-h-40`）或
`style`（`maxHeight`）给，超过后框内滚动。要固定高度就写 `h-32`：应用的 `class` / `style` 最后合并，盖过组件算出来的高度，
自动长高随之失效。

### 事件

同 Input：`onChange({ value })`、`onSubmit({ value })`（只在 `submitBehavior` 不是 `"newline"` 时会触发）、`onFocus`、`onBlur`。
Rust 是 `TextareaEvent` 的 `Change(String)` / `Submit(String)` / `Focus` / `Blur`。

### Ref

同 Input：`TextareaRef` / `ComponentRef<TextareaApi>`，方法 `focus()`、`blur()`、`setSelection(start, end)`、`insertText(text)`。

### 部件

只有根，没有 `<部件>Class` / `<部件>Style`（同 Input）。

## 4. 变体与尺寸

无变体。`size` 四档都支持，**只改字号、行高和左右内边距**，让同一个表单里的 Input 和 Textarea 文字对齐；高度由内容决定，不随 `size`：

| `size` | 字号（指针 / 触屏） | 左右内边距 | 上下内边距 |
|---|---|---|---|
| `xs` | 12 / 14 | 8 | 8 |
| `sm` | 13 / 14 | 10 | 8 |
| `md` | 14 / 16 | 12 | 8 |
| `lg` | 14 / 16 | 12 | 8 |

- 字号、左右内边距读 Input 的 `input-font-size-*`、`input-padding-x-*`（字号分指针 / 触屏，同 Input）。
- 行高由 foundation 的「字号 → 行高」函数给（14 → 20，同 shadcn 的 `text-sm`），本组件不写行高，不是 token。
- 上下内边距 `textarea-padding-y`（8，shadcn `py-2`），各档相同，触屏也不加大。

和 shadcn 的差别：shadcn 的 Textarea 没有尺寸。加 `size` 的理由是 conventions.md 要求尺寸走 `Size`，且表单里 `size="sm"` 的 Input 旁边
放一个 14 号字的 Textarea 会不齐。最矮高度 `textarea-min-height` 不随尺寸变（shadcn 只有一个 `min-h-16`）。

宽度：`w-full`，同 shadcn。

## 5. 状态

边、背景、阴影、焦点环、invalid、readonly、disabled、过渡、选区、光标：**全部同 [Input §5](input.md#5-状态)**（foundation 的字段外观；无悬停；外环画在框外；触屏只换边色），读同一组
`input-*` token，shadcn 的类也一模一样（`border-input bg-transparent shadow-xs focus-visible:border-ring focus-visible:ring-[3px]
focus-visible:ring-ring/50 aria-invalid:… disabled:opacity-50 dark:bg-input/30`）。

多行特有：

- **高度**：`max(textarea-min-height, 内容高 + 2 × textarea-padding-y + 2 × input-border-width)`。内容高来自宿主的
  `onContentSizeChange`（空框按一行算）。应用的 `max-h-*` / `maxHeight` 限住上限，超过后框内滚动；应用的 `h-*` / `height` 盖过整条规则。
- **文字从左上开始**（不垂直居中），同 shadcn。
- 高度变化**不做动画**（shadcn 的 `field-sizing-content` 也是瞬变）；减弱动态效果下自然也不动。
- 滚动条：宿主默认（多数平台是叠加式滚动条，滚动时出现），本库不设。

## 6. 指针与触屏

| | 指针 | 触屏 |
|---|---|---|
| 字号 | 12 / 13 / 14 / 14 | 14 / 14 / 16 / 16（同 Input） |
| 最矮高度 | 64（`textarea-min-height`） | 64，已超过 `touch-target`，不补命中区 |
| 焦点 | 边换 `ring` + 3pt 外环 | 只换边色，不画外环 |
| 上下内边距 | 8 | 8（不加大） |
| 键盘 | — | Return 键的文字由平台按 `submitBehavior` 决定；滚动区里被软键盘挡住时原生让开 |

## 7. 组件 token

字段外观的 token 全部读 Input 的（`input-radius`、`input-border*`、`input-background`、`input-foreground`、
`input-placeholder-foreground`、`input-ring*`、`input-shadow`、`input-font-size-*`、`input-padding-x-*`），见 [Input §7](input.md#7-组件-token)
（评审已定：共用，不另起 `textarea-*` 的同类名字）。Textarea 自己只有：

| token | 类型 | 推导默认值 | 亮 / 暗 |
|---|---|---|---|
| `textarea-min-height` | 长度 | `spacing` × 16（64，shadcn `min-h-16`） | — |
| `textarea-padding-y` | 长度 | `spacing` × 2（8，shadcn `py-2`） | — |

```rust
tokens! {
    /// The textarea's least height; it grows with its content from there.
    TEXTAREA_MIN_HEIGHT: Length = "textarea-min-height" => |t| t.length(SPACING) * 16.0,
    /// The space above and below the text.
    TEXTAREA_PADDING_Y: Length = "textarea-padding-y" => |t| t.length(SPACING) * 2.0,
}
```

## 8. 示例

TSX：

```tsx
import { signal, View } from "nsh";
import { Label, Textarea, type TextareaRef } from "nsh-longbridge";

function Feedback() {
  const text = signal("");
  let field: TextareaRef | undefined;
  return (
    <View class="gap-2">
      <Label htmlFor={field}>Your message</Label>
      <Textarea
        ref={field}
        class="max-h-40"
        value={text()}
        onChange={({ value }) => text.set(value)}
        placeholder="Type your message here."
        maxLength={500}
        accessibilityLabel="Your message"
      />
    </View>
  );
}

// 回车发送、Shift+Return 换行
<Textarea submitBehavior="submit" onSubmit={({ value }) => send(value)} />
// 固定高度、禁用
<Textarea class="h-32" disabled value={log()} />
```

Rust：

```rust
use nsh::prelude::*;
use nsh_longbridge::{label, textarea};

fn feedback() -> impl IntoElement {
    let text = signal(String::new());
    let field = ComponentRef::new();
    view()
        .gap_2()
        .child(label().html_for(field).child("Your message"))
        .child(
            textarea()
                .component_ref(field)
                .max_h(px(160.0))
                .value(move || text.get())
                .on_change(move |value: &String| text.set(value.clone()))
                .placeholder("Type your message here.")
                .max_length(500)
                .accessibility_label("Your message"),
        )
}
```

## 9. NSH 能力缺口与已定的取舍

缺口 G1（读屏名要应用自己写）、G4（选区）同 [Input §9](input.md#9-nsh-能力缺口与已定的取舍)；外环贴边会被裁的处理同 [Input §5](input.md#5-状态)。多行另有：

- **G8 长高晚一帧。** `onContentSizeChange` 在布局之后才报，组件再改高度又是一轮布局，所以换行那一刻文字可能先在旧高度里
  滚一下再撑开。绕过：实现时先在 headless 测试和 macOS 上看；如果明显，就在高度上预留一行（内容高 + 一行行高，到上限为止），
  这要人确认是否接受「比内容多一行空白」。展示台放一个连续回车的例子验收。
- **G9 Web 上的长高（待验证）。** 长高一律走 `onContentSizeChange`，不用 CSS 的 `field-sizing`（其他平台没有）；Web 上要截图确认
  NSH 报的内容高和浏览器实际排版一致。

已定（2026-11-26 评审）：

- Textarea 有 `size`，只改字号、行高、左右内边距（§4）。
- 触屏上下内边距不加大。
- 共用 `input-*` token；触屏字号、焦点、无悬停同 Input。
