use leptos::{prelude::*, svg::Svg};
#[component]
pub fn BookDashed(
    #[prop(default = 24.into(), into)] size: Signal<usize>,
    #[prop(default = "currentColor".into(), into)] color: Signal<String>,
    #[prop(default = "none".into(), into)] fill: Signal<String>,
    #[prop(default = 2.into(), into)] stroke_width: Signal<usize>,
    #[prop(default = false.into(), into)] absolute_stroke_width: Signal<bool>,
    #[prop(optional)] node_ref: NodeRef<Svg>,
) -> impl IntoView {
    let stroke_width = Signal::derive(move || {
        if absolute_stroke_width.get() {
            stroke_width.get() * 24 / size.get()
        } else {
            stroke_width.get()
        }
    });
    view! {
        <svg
            node_ref=node_ref
            class:lucide=true
            xmlns="http://www.w3.org/2000/svg"
            width=size
            height=size
            viewBox="0 0 24 24"
            fill=fill
            stroke=color
            stroke-width=stroke_width
            stroke-linecap="round"
            stroke-linejoin="round"
        >
            <path d="M12 17h2"></path>
            <path d="M12 22h2"></path>
            <path d="M12 2h2"></path>
            <path d="M18 22h1a1 1 0 0 0 1-1"></path>
            <path d="M18 2h1a1 1 0 0 1 1 1v1"></path>
            <path d="M20 15v2h-2"></path>
            <path d="M20 8v3"></path>
            <path d="M4 11V9"></path>
            <path d="M4 19.5V15"></path>
            <path d="M4 5v-.5A2.5 2.5 0 0 1 6.5 2H8"></path>
            <path d="M8 22H6.5a1 1 0 0 1 0-5H8"></path>
        </svg>
    }
}
