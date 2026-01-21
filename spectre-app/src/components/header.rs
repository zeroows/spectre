use dioxus::prelude::*;

#[component]
pub fn Header() -> Element {
    rsx! {
        header {
            class: "text-center mb-8",
            p {
                class: "text-slate-400 text-sm uppercase tracking-widest mb-2",
                "Try out the"
            }
            h1 {
                class: "text-3xl font-light text-white",
                "Spectre "
                span {
                    class: "text-cyan-400 font-medium",
                    "algorithm"
                }
            }
        }
    }
}

