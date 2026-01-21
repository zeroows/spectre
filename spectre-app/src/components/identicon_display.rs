use dioxus::prelude::*;

#[component]
pub fn IdenticonDisplay(identicon: Signal<String>) -> Element {
    rsx! {
        if !identicon.read().is_empty() {
            div {
                class: "flex items-center justify-center gap-2 py-1 text-slate-400 text-sm",
                "Your Identity:"
                div {
                    class: "flex items-center gap-1 bg-slate-800/50 px-3 py-1 rounded-lg border border-slate-700 text-xl",
                    "{identicon}"
                }
            }
        }
    }
}

