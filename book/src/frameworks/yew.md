# Lucide Yew

Implementation of the Lucide icon library for [Yew](https://yew.rs/) applications.

## Installation

Install the icons from your command line.

```shell
# Selective Icons
cargo add lucide-yew --features camera,file-image,moon,sun

# All Icons
cargo add lucide-yew --features full
```

-   [View on crates.io](https://crates.io/crates/lucide-yew)
-   [View on docs.rs](https://docs.rs/lucide-yew/latest/lucide_yew/)
-   [View source](https://github.com/RustForWeb/lucide/tree/main/packages/yew)

## Usage

```rust,ignore
use lucide_yew::Camera;
use yew::prelude::*;

#[component]
fn App() -> Html {
    html! {
        <Camera color="red" size=48 />
    }
}
```

## Props

| Name                    | Type     | Default          |
| ----------------------- | -------- | ---------------- |
| `size`                  | `usize`  | `24`             |
| `color`                 | `String` | `"currentColor"` |
| `fill`                  | `String` | `"none"`         |
| `stroke_width`          | `usize`  | `2`              |
| `absolute_stroke_width` | `bool`   | `false`          |

## Icons

```toml,trunk
package = "lucide-yew-book"
features = ["icons"]
files = ["src/icons.rs"]
```
