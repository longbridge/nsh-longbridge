# nsh-longbridge

A component library for [NSH](https://github.com/longbridge/nsh) apps: a neutral look on `nsh/base`'s behavior, one
Rust implementation for TSX and Rust apps, customizable through design tokens.

See every component, and make a theme, in the [gallery](https://longbridge.github.io/nsh-longbridge/).

## Install

```sh
nsh add github:longbridge/nsh-longbridge
```

```tsx
import { Button } from "nsh-longbridge";

<Button variant="outline">Save</Button>;
```

Rust apps depend on the crate (`nsh-longbridge = { git = "https://github.com/longbridge/nsh-longbridge" }`) and
`use nsh_longbridge::*`.

It requires NSH 0.7. Within `0.1.x` the public API only grows.

## Platforms

The web and the desktops first: reviewed on macOS, built and spot-checked on Windows and Linux. Sizes have pointer and
touch values throughout; phones are reviewed later.

## Customize

Every look is a token. Seed tokens (`radius`, `spacing`, `control-height`, `font-size`, `duration`) reshape every
component in proportion, semantic tokens (`primary`, `border`) restyle a role, and component tokens (`button-radius`)
one kind of component. `ThemeScope` themes part of a screen; `class` and `style` reach a single instance. A token
left unset derives from the tier below, so hover, press and focus states follow a changed color. Theme files are in
`themes/`.

## Documentation

- [Components](docs/components/README.md): find one by what it is for; each has a spec
- [Tokens](docs/tokens.md): the tiers, the names and how values derive
- [Conventions](docs/conventions.md): how a component is built and drawn
- `gallery/`: an app showing every component in light and dark, with its tokens editable live

## License

Copyright 2026 Longbridge. Licensed under the [Apache License, Version 2.0](LICENSE).

## Acknowledgments

The default look and the theme variables' names draw on [shadcn/ui](https://ui.shadcn.com) (MIT), so its themes can be
used as theme files here.
