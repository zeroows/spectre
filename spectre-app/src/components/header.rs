use dioxus::prelude::*;

#[component]
pub fn Header() -> Element {
    rsx! {
        div {
            class: "text-center mb-8",
            p { class: "text-slate-300 text-sm tracking-widest mb-2", "TRY OUT THE" }
            h1 {
                class: "text-4xl md:text-5xl font-light",
                span { class: "text-white", "Spectre" }
                span { class: "text-cyan-400", " algorithm" }
            }
        }
    }
}

