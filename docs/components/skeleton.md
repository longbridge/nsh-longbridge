# Skeleton

加载占位块：内容还没到时，用一块缓慢呼吸的灰块占住它将来的位置和形状。对照 shadcn/ui new-york v4 `skeleton.tsx`。

## 用途

- 列表、卡片、头像、正文在首次加载时的占位，让版面不跳。
- 搜：骨架屏、加载占位、占位块；skeleton, loading placeholder, shimmer placeholder。

何时不用：

- 一个按钮、一小块区域在等：用 `Spinner`（按钮自己的 `loading` 已经带）。
- 不知道内容长什么样、或者加载很快（< 300 ms）：什么都不放，或者放 `Spinner`；骨架的形状和真实内容差太远反而更跳。
- 文字扫光效果：shadcn 没有，本库不做。

## 基于

`nsh/base` 没有对应组件，本库用一个 `view` 画。没有行为。

宽、高、圆角形状由应用用 `class` / `style` 给，和 shadcn 一样（`<Skeleton className="h-4 w-[250px]" />`）。组件不给默认
宽高：没给尺寸时它是 0 × 0，看不见。

## API

| TSX | Rust | 类型 | 默认 | 说明 |
|---|---|---|---|---|
| `class` / `style` | `class` / `style` | | | 根，即占位块；宽高、`rounded-full` 在这里给 |

没有事件，没有 children，没有部件。

## 变体和尺寸

- 无 `variant`。
- 无 `size`（`xs` / `sm` / `md` / `lg` 都不支持）：尺寸就是占位的形状，由应用定。

## 外观

shadcn：`animate-pulse rounded-md bg-accent`。

| | 值 |
|---|---|
| 底 | `skeleton` |
| 圆角 | `skeleton-radius`；圆形头像占位给 `class="rounded-full"`（`class` 最后合并，盖过 token） |
| 动画 | Tailwind 的 `animate-pulse`：2 s 一轮，`cubic-bezier(0.4, 0, 0.6, 1)`，透明度 1 → 0.5 → 1，无限循环。NSH 支持 `animate-pulse` |

动画不做成 token：周期和曲线是 Tailwind 的固定值，NSH 的 token 也存不了曲线。

## 状态

- **减弱动态效果**：不呼吸，停在不透明（`motion-reduce:animate-none`）。和 conventions.md "减弱动态效果时过渡归零"
  一致；骨架的呼吸只是装饰，停了不丢信息。shadcn 没处理这一点。
- 无 hover / active / focus / disabled。

## 指针与触屏

没有差别。

## 键盘与读屏

- 不是 Tab 站，不接指针。
- 不进读屏树（`accessible={false}`）。"正在加载"要由外层容器说：给外层容器设 `accessibilityState={{ busy: true }}`
  或一个读屏名（例："Loading comments"）。组件不自己念，因为一屏常有十几块骨架，各念一遍没法用。

## Token

| token | 类型 | 推导默认 | 亮 / 暗 |
|---|---|---|---|
| `skeleton` | 颜色 | `accent` | 跟着 `accent`（shadcn neutral 亮 `oklch(0.97 0 0)`，暗 `oklch(0.269 0 0)`） |
| `skeleton-radius` | 长度 | `radius-md`（8） | 同 |

## 示例

```tsx
import { Skeleton } from "nsh-longbridge";

<View class="flex-row items-center gap-4" accessibilityLabel="Loading profile" accessibilityState={{ busy: true }}>
  <Skeleton class="size-12 rounded-full" />
  <View class="gap-2">
    <Skeleton class="h-4 w-[250px]" />
    <Skeleton class="h-4 w-[200px]" />
  </View>
</View>
```

```rust
use nsh_longbridge::skeleton;

view().flex_row().items_center().gap_4()
    .child(skeleton().size_12().rounded_full())
    .child(
        view().gap_2()
            .child(skeleton().h_4().w(px(250.0)))
            .child(skeleton().h_4().w(px(200.0))),
    )
```

Rust 里 `skeleton()` 实现 `Styled`，宽高、圆角直接链在上面（盖过组件的默认圆角，同 TSX 的 `class`）。

## NSH 缺口

无。`animate-pulse` 和 `motion-reduce:` 都是 NSH 支持的类（`docs/dev/styles.md`）。实现时确认：窗口不可见、
滚出视口时平台是否停掉循环动画；不停的话是 NSH 的性能问题，不在本库绕。

## 需要拍板

无。
