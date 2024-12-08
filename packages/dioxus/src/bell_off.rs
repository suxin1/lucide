use dioxus::prelude::*;
#[derive(Clone, PartialEq, Props)]
pub struct BellOffProps {
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
pub fn BellOff(props: BellOffProps) -> Element {
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
            path { "d": "M8.7 3A6 6 0 0 1 18 8a21.3 21.3 0 0 0 .6 5" }
            path { "d": "M17 17H3s3-2 3-9a4.67 4.67 0 0 1 .3-1.7" }
            path { "d": "M10.3 21a1.94 1.94 0 0 0 3.4 0" }
            path { "d": "m2 2 20 20" }
        }
    }
}
