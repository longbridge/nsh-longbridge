# Token 规范

本库外观的唯一来源。组件里不写颜色字面量、不写散落的尺寸数字：颜色和会被人调的尺寸都是 token，
固定的几何关系（比如图标和文字的间距随 `size` 变）写在 foundation 的函数里，函数也从 token 取值。

## NSH 给的底座（约束）

- NSH 的 token 只有三种值：颜色（亮 / 暗各一）、数字（点，或无单位）、字体族。阴影、渐变、缓动曲线不能做 token。
- 主题文件是 shadcn 的 `cssVars` 格式（`theme` / `light` / `dark` 三段），`nsh.json` 的 `"theme"` 指向它，
  盖在 `nsh-base` 的默认（shadcn neutral）之上。主题里新加的名字就是应用自己的 token。
- token 是整个界面一份，读取会订阅：`setTheme` / `set_tokens` 换掉后，读过它的绑定会重跑。核心没有按子树覆盖，
  作用域由本库自己做（见下「作用域」）。
- Tailwind 类和 TSX 的 `token()` 只认主题文件里真有的 token；本库的推导默认值只在本库组件里生效。应用想在自己的
  Tailwind 类里用 `bg-button-default` 这类名字，要把它写进主题文件（从 `themes/default.json` 复制）。

## 三层

命名照 shadcn / Tailwind 4 的变量名，kebab-case，不带 `--`。

### 1. 种子：少数几个旋钮，牵动全部

| token | 默认 | 推出什么 |
|---|---|---|
| `radius` | 10（0.625rem，shadcn） | `radius-sm/md/lg/xl` |
| `spacing` | 4 | 组件内边距、间距（Tailwind 4 的 `--spacing`） |
| `font-size` | 14 | 控件字号 `font-size-xs/sm/md/lg` |
| `control-height` | 32 | 指针设备上 `control-height-xs/sm/md/lg` |
| `control-height-touch` | 40 | 触屏上的同一组 |
| `duration` | 150（毫秒） | `duration-fast/slow` |
| `font-sans` / `font-mono` | 系统字体 | 文字 |

改 `radius: 0` 整套变直角；改 `control-height: 28` 整套变紧凑。

### 2. 语义：shadcn 的名字

- **颜色**：shadcn 全套（`background`、`foreground`、`card`、`popover`、`primary`、`secondary`、`muted`、`accent`、
  各自的 `-foreground`，`destructive`、`border`、`input`、`ring`、`chart-1..5`、`sidebar-*`），加上本库补的：
  `destructive-foreground`、`success` / `warning` / `info` 及其 `-foreground`、`overlay`（模态遮罩）、`shadow`（投影色）。
- **圆角**：`radius-sm` = radius − 4，`radius-md` = radius − 2，`radius-lg` = radius，`radius-xl` = radius + 4（shadcn v4 的算法）。
- **控件高度**：`control-height-xs/sm/md/lg` = control-height − 8 / − 4 / ± 0 / + 4（24 / 28 / 32 / 36）；
  `control-height-touch-xs/sm/md/lg` = control-height-touch − 8 / − 4 / ± 0 / + 8（32 / 36 / 40 / 48）。
  触屏另有 `touch-target` 44：控件比它小时补命中区，不放大外观。
- **字号**：控件用 `font-size-xs/sm/md/lg` = font-size − 2 / − 1 / ± 0 / ± 0（12 / 13 / 14 / 14，随控件 `size`）；
  正文和标题照 Tailwind 的值（`text-xs` 12、`text-sm` 14、`text-base` 16、`text-lg` 18，名字和值都不改，
  免得和 NSH 的 Tailwind 类对不上）；`font-weight-medium` 500、`font-weight-semibold` 600。
- **线与焦点**：`border-width` 1、`ring-width` 3、`ring-opacity` 0.5（shadcn 的 `focus-visible:ring-ring/50 ring-[3px]`）。
- **状态**：`opacity-disabled` 0.5。
- **动效**：`duration-fast` = duration × 2/3、`duration-slow` = duration × 5/3。缓动曲线固定在 foundation，
  减弱动态效果时按下不缩放、过渡归零。
- **阴影**：几何按档位固定在 foundation（`xs` / `sm` / `md` / `lg`，照 Tailwind），颜色用 `shadow` token；
  组件用 `<组件>-shadow` 这个数字 token 选档：0 无、1 `xs`、2 `sm`、3 `md`、4 `lg`。
- **无效**：`ring-invalid` = `destructive` 的 20%（暗色 40%），表单控件 invalid 且聚焦时的外环。

### 3. 组件：只影响一种组件

命名：`<组件>[-<变体>][-<部件>]-<属性>[-<状态>]`。颜色沿用 shadcn 的写法：底色用裸名，文字加 `-foreground`，
描边加 `-border`；状态后缀只用 `-hover`、`-active`、`-focus`、`-invalid`、`-disabled`。

- 控件本体（框、轨道、圆圈）用裸名；次要部件带部件名（`switch-thumb`、`radio-group-item-dot`）。
- 裸名撞上语义 token 时：意思相同就直接用语义 token，不再声明（`card`、`card-foreground`，否则会循环引用）；
  意思不同就加 `-background`（`input` 是语义的输入框边色，输入框的底色叫 `input-background`）。
- 焦点环统一叫 `<组件>-border-focus`（聚焦时的边）、`<组件>-ring`（外环）、`<组件>-ring-invalid`（默认引用 `ring-invalid`）。
- 类型：颜色；长度（字号、圆角、尺寸、间距，单位点）；数字（毫秒、档位、透明度、缩放）。

例：

```
button-radius              = radius-md
button-height-sm           = control-height-sm（触屏取 control-height-touch-sm）
button-default             = primary
button-default-foreground  = primary-foreground
button-default-hover       = button-default 的 90%
button-outline-border      = border（暗色 input）
input-border               = input
card-radius                = radius-xl
card-shadow                = 2（sm）
```

每个组件的 token 表写在它的 API 单里（`docs/components/<组件>.md`），和代码里的声明一一对应，测试守着。

## 解析顺序

组件读一个 token 时，从上往下，第一个有值的胜出：

1. 最近的 `ThemeScope` 给的值；
2. 应用的 token（主题文件、`setTheme`、`set_tokens`，嵌入时宿主给的）；
3. 本库声明的推导默认值（引用语义 / 种子 token，**走同一套解析**，所以作用域里改了 `primary`，`button-default` 也跟着变）。

推导默认值和 token 名一起在 Rust 里声明，就近写在组件文件里：

```rust
tokens! {
    /// The default variant's fill.
    BUTTON_DEFAULT: Color = "button-default" => |t| t.color(PRIMARY),
    /// The default variant's fill under the pointer.
    BUTTON_DEFAULT_HOVER: Color = "button-default-hover" => |t| t.color(BUTTON_DEFAULT).opacity(0.9),
    BUTTON_RADIUS: Length = "button-radius" => |t| t.length(RADIUS_MD),
}
```

推导默认值可以分亮暗、分指针 / 触屏，这是通用规则，不是例外：

```rust
INPUT_BACKGROUND: Color = "input-background" => |t| t.light_dark(TRANSPARENT, t.color(INPUT).opacity(0.3)),
BUTTON_HEIGHT_SM: Length = "button-height-sm" => |t| t.pointer_touch(t.length(CONTROL_HEIGHT_SM), t.length(CONTROL_HEIGHT_TOUCH_SM)),
```

应用在主题文件里写的值，亮暗照主题文件的 `light` / `dark` 段；指针 / 触屏的不同取值，应用要分开改时另写
`<名字>-touch`（有就用于触屏）。

`tokens!` 生成名字常量、默认值和登记表；登记表用来生成 `themes/default.json`（全量、带 `var()` 引用，给应用复制）
和文档里的 token 表，测试检查三者一致。

组件里这样读（在组件函数体里拿句柄，绑定里读值）：

```rust
let t = nsh_longbridge::theme();           // 捕获当前作用域
view().bg(move || t.color(BUTTON_DEFAULT)).rounded(move || t.length(BUTTON_RADIUS))
```

## 作用域

```tsx
<ThemeScope tokens={{ primary: "#2563eb", "button-radius": "0px" }} appearance="dark">
  …这里面的本库组件用这套值，暗色…
</ThemeScope>
```

- `tokens`：值写 CSS（颜色、`px` / `rem`、数字），解析失败的忽略并告警；可嵌套，内层盖外层。
- `appearance`：`"light"` / `"dark"` / 不写（跟随）。强制时本库把颜色的亮暗两值都取成那一档；
  不在本库里画的东西（应用自己的 Tailwind 类、NSH 原生控件）不受影响。

## 主题文件

- `themes/default.json`：全量 token，默认值，带引用。是文档也是起点。
- `themes/*.json`：预设（例如 shadcn 的 zinc / blue / rose 配色、一份紧凑密度）。
- 应用的做法：`nsh.json` 的 `"theme"` 指向自己的文件，只写要改的 token；要在 Tailwind 类里用本库 token 的，
  把对应条目复制过去。
