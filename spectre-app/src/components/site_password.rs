use dioxus::prelude::*;
use wasm_bindgen_futures::JsFuture;
use web_sys::window;
use gloo_timers::future::sleep;
use std::time::Duration;

#[component]
pub fn SitePassword(
    generated_password: Signal<String>,
    is_generating: Signal<bool>,
) -> Element {
    let mut copied = use_signal(|| false);
    
    rsx! {
        div {
            div {
                class: "flex items-center justify-center mb-4",
                span { class: "text-slate-600 text-2xl", "═" }
            }
            div {
                class: "flex items-center gap-2 mb-4",
                span { class: "text-2xl", "🔑" }
                h2 { class: "text-white text-xl font-light", "Site Password" }
            }
            button {
                class: "w-full bg-gradient-to-r from-cyan-400 to-emerald-400 text-slate-900 py-4 px-6 rounded-full text-lg font-medium hover:from-cyan-300 hover:to-emerald-300 transition shadow-lg shadow-cyan-500/20 cursor-pointer",
                onclick: move |_| {
                    let password = generated_password.read().clone();
                    if !password.is_empty() {
                        spawn(async move {
                            if let Some(window) = window() {
                                let clipboard = window.navigator().clipboard();
                                let promise = clipboard.write_text(&password);
                                if let Ok(_) = JsFuture::from(promise).await {
                                    copied.set(true);
                                    // Reset copied state after 2 seconds
                                    sleep(Duration::from_secs(2)).await;
                                    copied.set(false);
                                }
                            }
                        });
                    }
                },
                if *is_generating.read() {
                    span {
                        class: "flex items-center justify-center gap-2",
                        span { class: "animate-spin", "⏳" }
                        "Generating..."
                    }
                } else if *copied.read() {
                    span {
                        class: "flex items-center justify-center gap-2",
                        "✓ Copied to clipboard!"
                    }
                } else if generated_password.read().is_empty() {
                    "Generate your password"
                } else {
                    "{generated_password}"
                }
            }
        }
    }
}

