# Lucide Leptos

Implementation of the Lucide icon library for [Leptos](https://leptos.dev/) applications.

## Installation

Install the icons from your command line.

```shell
# Selective Icons
cargo add lucide-leptos --features camera,file-image,moon,sun

# All Icons
cargo add lucide-leptos --features full
```

-   [View on crates.io](https://crates.io/crates/lucide-leptos)
-   [View on docs.rs](https://docs.rs/lucide-leptos/latest/lucide_leptos/)
-   [View source](https://github.com/RustForWeb/lucide/tree/main/packages/leptos)

## Usage

```rust,ignore
use leptos::prelude::*;
use lucide_leptos::Camera;

#[component]
fn App() -> impl IntoView {
    view! {
        <Camera color="red" size=48 />
    }
}
```

## Props

| Name                    | Type             | Default          |
| ----------------------- | ---------------- | ---------------- |
| `size`                  | `Signal<usize>`  | `24`             |
| `color`                 | `Signal<String>` | `"currentColor"` |
| `fill`                  | `Signal<String>` | `"none"`         |
| `stroke_width`          | `Signal<usize>`  | `2`              |
| `absolute_stroke_width` | `Signal<bool>`   | `false`          |

## Icons

```toml,trunk
package = "lucide-leptos-book"
features = ["icons"]
files = ["src/icons.rs"]
```
