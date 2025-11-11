use dioxus::prelude::*;

#[component]
pub fn Footer() -> Element {
    rsx! {
        p {
            class: "text-center text-slate-500 italic text-sm mt-6",
            "This information never leaves this page."
        }
    }
}

