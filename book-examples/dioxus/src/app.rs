use dioxus::prelude::*;

#[component]
pub fn App() -> Element {
    #[allow(unused_mut)]
    let mut children: Vec<Option<VNode>> = vec![];

    #[cfg(feature = "icons")]
    {
        use crate::icons::Icons;

        children.push(rsx! {
            Icons {}
        });
    }

    rsx! {
        div { class: "w-full h-full flex justify-center items-center", {children.iter()} }
    }
}
