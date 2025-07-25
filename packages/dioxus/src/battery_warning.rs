use dioxus::prelude::*;
#[derive(Clone, PartialEq, Props)]
pub struct BatteryWarningProps {
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
    pub style: Option<String>,
}
#[component]
pub fn BatteryWarning(props: BatteryWarningProps) -> Element {
    let stroke_width = if props.absolute_stroke_width {
        props.stroke_width * 24 / props.size
    } else {
        props.stroke_width
    };
    rsx! {
        svg {
            "xmlns": "http://www.w3.org/2000/svg",
            "class": if let Some(class) = props.class { "{class}" },
            "style": if let Some(style) = props.style { "{style}" },
            "width": "{props.size}",
            "height": "{props.size}",
            "viewBox": "0 0 24 24",
            "fill": "{props.fill}",
            "stroke": "{props.color}",
            "stroke-width": "{stroke_width}",
            "stroke-linecap": "round",
            "stroke-linejoin": "round",
            path { "d": "M10 17h.01" }
            path { "d": "M10 7v6" }
            path { "d": "M14 6h2a2 2 0 0 1 2 2v8a2 2 0 0 1-2 2h-2" }
            path { "d": "M22 14v-4" }
            path { "d": "M6 18H4a2 2 0 0 1-2-2V8a2 2 0 0 1 2-2h2" }
        }
    }
}
