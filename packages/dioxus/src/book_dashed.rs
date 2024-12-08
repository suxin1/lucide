use dioxus::prelude::*;
#[derive(Clone, PartialEq, Props)]
pub struct BookDashedProps {
    #[props(default = 24)]
    pub size: usize,
    #[props(default = "currentColor".to_owned())]
    pub color: String,
    #[props(default = "none".to_owned())]
    pub fill: String,
    #[props(default = 2)]
    pub stroke_width: usize,
    #[props(default = false)]
    pub absolute_stroke_width: bool,
    pub class: Option<String>,
}
#[component]
pub fn BookDashed(props: BookDashedProps) -> Element {
    let stroke_width = if props.absolute_stroke_width {
        props.stroke_width * 24 / props.size
    } else {
        props.stroke_width
    };
    rsx! {
        svg {
            "xmlns": "http://www.w3.org/2000/svg",
            "class": if let Some(class) = props.class { "{class}" },
            "width": "{props.size}",
            "height": "{props.size}",
            "viewBox": "0 0 24 24",
            "fill": "{props.fill}",
            "stroke": "{props.color}",
            "stroke-width": "{stroke_width}",
            "stroke-linecap": "round",
            "stroke-linejoin": "round",
            path { "d": "M12 17h2" }
            path { "d": "M12 22h2" }
            path { "d": "M12 2h2" }
            path { "d": "M18 22h1a1 1 0 0 0 1-1" }
            path { "d": "M18 2h1a1 1 0 0 1 1 1v1" }
            path { "d": "M20 15v2h-2" }
            path { "d": "M20 8v3" }
            path { "d": "M4 11V9" }
            path { "d": "M4 19.5V15" }
            path { "d": "M4 5v-.5A2.5 2.5 0 0 1 6.5 2H8" }
            path { "d": "M8 22H6.5a1 1 0 0 1 0-5H8" }
        }
    }
}
