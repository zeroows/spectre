use dioxus::prelude::*;

#[component]
pub fn IdenticonDisplay(identicon: Signal<String>) -> Element {
    rsx! {
        if !identicon.read().is_empty() {
            div {
                class: "flex items-center justify-center gap-3 my-6 p-4 bg-slate-700/30 rounded-2xl border border-slate-600/30",
                span { 
                    class: "text-slate-400 text-sm font-light",
                    "Your Identity:"
                }
                div {
                    class: "text-4xl flex items-center gap-1",
                    "{identicon}"
                }
            }
        }
    }
}

