# nsh-longbridge

长桥各端共用的 NSH 组件库：shadcn/ui 风格的外观，`nsh/base` 的行为，一份 Rust 实现同时给 TSX 和 Rust 应用用，
外观可以从全局到单个部件逐层自定义。

- 形态：一个 Rust nshm 包（crate 名 `nsh-longbridge`，根上 `#[native_module("nsh-longbridge")]`）。
  - TSX 应用：`nsh.json` 的 `"dependencies"` 加上本仓库，`import { Button } from "nsh-longbridge"`，类型自动生成。
  - Rust 应用：Cargo 依赖本 crate，`use nsh_longbridge::{button, ..}`。
- 默认外观：shadcn/ui `neutral`（new-york v4）。品牌、产品的外观用主题和 token 覆盖，不改组件源码。
- 平台：先做 Web 和桌面（macOS 验收，Windows / Linux 编译通过 + 抽查）；尺寸从第一天起分指针 / 触屏两套，
  手机端以后验收。终端由 NSH 的 terminal look 负责，本库不另做。

## 分层

```
nsh-core / nsh-base        行为、状态、焦点、键盘、无障碍（NSH 的，不是我们的）
nsh-longbridge
  src/theme/               token 解析：种子 → 语义 → 组件三层，作用域覆盖，强制亮 / 暗
  src/foundation/          Size、圆角档、控件高度、字号、焦点环、按下反馈、动效、阴影、图标
  src/components/          一个组件一个文件，只用 theme 和 foundation，不互相乱引
themes/                    主题文件（shadcn cssVars 格式），应用在 nsh.json 的 "theme" 里引用或复制
gallery/                   展示台（TSX 应用，Web + macOS），也是文档和视觉验收
docs/                      契约：tokens.md、conventions.md、components/<组件>.md
```

以后：`src/blocks/` 放与业务无关的组合块。行情、交易等业务组件另起包，不进本库。

## 自定义，从粗到细

| 层 | 怎么改 | 影响范围 |
|---|---|---|
| 1. 种子 token | 主题里改 `radius`、`spacing`、`control-height`、`font-size`、`duration` | 整个应用的所有组件按比例一起变 |
| 2. 语义 token | 主题里改 `primary`、`border`、`radius-md`、`control-height-sm`… | 用到这个语义的所有组件 |
| 3. 组件 token | 主题里改 `button-radius`、`button-default-hover`、`input-border`… | 只这一种组件 |
| 4. 作用域 | `<ThemeScope tokens={{…}} appearance="dark">` 包住一块界面 | 只这棵子树里的本库组件 |
| 5. 实例 | `variant`、`size` 属性；根上的 `class` / `style`；各部件的 `*Class` / `*Style` | 只这一个组件 |
| 6. 运行时 | TSX `setTheme(json)`、Rust `set_tokens(..)` | 整个应用即时换肤 |

没写的 token 由下一层推出来（`button-default` 没写就是 `primary`，`button-default-hover` 没写就是它的 90%），
所以改一个颜色，悬停、按下、焦点这些状态会跟着一致地变。完整规则见 [docs/tokens.md](docs/tokens.md)。

## 分期

- **P0 地基**：仓库、token 解析与作用域、foundation、Button 作为参照实现、展示台（亮 / 暗、主题预设、实时调 token）、
  headless 测试、颜色字面量检查、文档。
- **P1 基础件**：Badge、Label、Input、Textarea、Checkbox、Switch、RadioGroup、Card、Separator、Tooltip、Kbd、
  Spinner、Skeleton。
- **P2 弹层与导航**：Dialog、AlertDialog、Popover、DropdownMenu、Select、Tabs、Toast。
- **P3 数据与表单**：Table、List、Form / Field、Combobox、DatePicker 等。

每个组件的流程：API 单（`docs/components/<组件>.md`）→ 实现 → 展示台 + 测试 → Web / macOS 亮暗截图验收。见
[docs/conventions.md](docs/conventions.md)。

## 组件索引

按用途找组件。新增或删掉对外组件时，同一次改动里改这张表。

| 组件 | 用途（找组件时搜这些叫法） | 不该用它的时候 |
|---|---|---|
| `Button` | 按钮、主按钮、次按钮、描边按钮、幽灵按钮、危险按钮、链接样式按钮、图标按钮、加载中按钮；button, icon button, loading | 打开网址用链接，不用 `variant="link"` 的按钮做外链 |
