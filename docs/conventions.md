# 组件约定

每个组件都守这些约定。改约定先改这份文档。

## 设计依据

| 方面 | 以谁为准 |
|---|---|
| 外观（颜色、圆角、字号、阴影、间距） | shadcn/ui new-york v4，值走 token（[tokens.md](tokens.md)） |
| 行为、键盘、焦点、无障碍 | `nsh/base`：有对应的 base 组件就包它，不自己重写行为 |
| 交互形态 | 所在平台的惯例（桌面弹层、触屏底部面板），`nsh/base` 已按平台做的直接用 |
| 桌面习惯 | gpui-kit 的 Design Guides：按钮用默认箭头光标，链接才用手型；中等密度为默认；同心圆角；状态都要看得见 |

不照抄 Web 习惯：悬停才出现的关键操作、处处手型光标、移动优先的单列布局。

## API

- **一个组件一个文件**，`src/components/<名字>.rs`；在根的 `#[native_module]` 里注册。JS 导出名就是类型名。
- **props 是结构体**，事件是枚举，选项是 `PropEnum`。TS 声明从这里生成，所以每个字段写英文文档注释。
- **受控**：值由应用持有，组件报变化（`onChange`、`onOpenChange`、`onCheckedChange`）。
- **`variant` + `size`**：变体名照 shadcn（`default` / `secondary` / `outline` / `ghost` / `destructive` / `link`，
  组件用不到的不加）；`size` 只用 foundation 的 `Size`（`xs` / `sm` / `md` / `lg`，默认 `md`），高度、字号、图标、
  内边距一起变，不加单独的高度属性。
- **部件可定制**：根接受 `class` / `style`（最后合并，盖过组件自己的）；每个有意义的部件给一对 `<部件>Class` /
  `<部件>Style`（`labelClass`、`iconStyle`），名字写进 API 单。
- **组合优于开关**：结构由子组件拼（`Card` + `CardHeader` + `CardTitle` + `CardContent`），不用一堆布尔属性控制显隐。
- **内容用 slot / children**，不把 markup 写死。
- **组件自带的文案**（读屏名、"Loading"）：英文默认值，宿主用 `labels::set_translator` 注入翻译。
- **新组件、新增或改动的公开 API 要评审**：先写 API 单再写代码。

## 外观实现

- 颜色、圆角、尺寸都从 `theme()` 句柄读 token；组件的 token 用 `tokens!` 就近声明，推导默认值引用语义 token。
  `src/components/` 里出现颜色字面量，测试会挂。
- 状态都要做：hover、active（按下）、focus-visible（焦点环 = `ring` 色、`ring-width`、`ring-opacity`，加 `border` 换 `ring` 色）、
  disabled（`opacity-disabled`，无悬停无按下）、invalid（`destructive` 的边和环）、loading（不可点、不变暗）。
- 按下反馈走 foundation 的 `press`：轻微下沉，减弱动态效果时不缩放；颜色过渡用 `duration`。
- 触屏：外观尺寸取 `control-height-touch-*`；小于 `touch-target` 的补命中区（hit slop），不放大外观。
- 亮暗两套都要好看，暗色照 shadcn 的暗色处理（例如 outline 在暗色下用 `input` 的边和 `input` 30% 的底）。

## 每个组件的交付

1. `docs/components/<名字>.md`：用途、何时不用、props / 事件 / 部件、变体和尺寸、状态、键盘与读屏、token 表、TSX 和 Rust 示例。
2. `src/components/<名字>.rs`：实现 + `tokens!` 声明。
3. 展示台一页：变体 × 尺寸 × 状态的矩阵，亮 / 暗都看，能在页面里改该组件的 token。
4. 测试：headless 测试行为（按下、禁用、受控值、键盘、无障碍角色与状态）；token 表和 `themes/default.json` 一致。
5. 截图验收：Web 和 macOS，亮 / 暗，各一张，放进 PR 描述。
6. README 组件索引加一行。

## 不做

- 业务组件（行情、交易、账户）不进本库。
- 不包一个组件只为了换个名字；不为一个页面加一个变体（用 `class` 或作用域 token）。
- 不在组件里判断具体产品或品牌。
