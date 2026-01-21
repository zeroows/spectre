use dioxus::prelude::*;
use wasm_bindgen_futures::JsFuture;
use web_sys::window;
use gloo_timers::future::sleep;
use std::time::Duration;

#[component]
pub fn SitePassword(
    generated_password: Signal<String>,
    is_generating: Signal<bool>,
    on_generate: EventHandler<()>,
) -> Element {
    let mut copied = use_signal(|| false);
    
    let copy_password = move |_| {
        let password = generated_password.read().clone();
        if !password.is_empty() {
            spawn(async move {
                if let Some(window) = window() {
                    let clipboard = window.navigator().clipboard();
                    let promise = clipboard.write_text(&password);
                    if let Ok(_) = JsFuture::from(promise).await {
                        copied.set(true);
                        sleep(Duration::from_secs(2)).await;
                        copied.set(false);
                    }
                }
            });
        }
    };

    rsx! {
        section {
            div {
                class: "flex items-center gap-2 mb-4",
                KeyIcon {}
                span {
                    class: "text-slate-300 font-medium",
                    "Site Password"
                }
            }
            
            // Generated Password Display
            if !generated_password.read().is_empty() {
                div {
                    class: "bg-slate-800/50 border border-slate-700 rounded-xl p-4 mb-4",
                    div {
                        class: "flex items-center justify-between",
                        code {
                            class: "text-lg font-mono text-cyan-400 tracking-wide",
                            "{generated_password}"
                        }
                        button {
                            class: "p-2 text-slate-400 hover:text-cyan-400 transition-colors",
                            onclick: copy_password,
                            if *copied.read() {
                                CheckIcon {}
                            } else {
                                CopyIcon {}
                            }
                        }
                    }
                }
            }
            
            // Generate Button
            button {
                class: "w-full py-4 rounded-xl font-medium text-slate-900 bg-gradient-to-r from-cyan-400 to-teal-400 hover:from-cyan-300 hover:to-teal-300 transition-all shadow-lg shadow-cyan-500/25 cursor-pointer",
                onclick: move |_| on_generate.call(()),
                if *is_generating.read() {
                    span {
                        class: "flex items-center justify-center gap-2",
                        span { class: "animate-spin", "⏳" }
                        "Generating..."
                    }
                } else {
                    "Generate Password"
                }
            }
            
            p {
                class: "text-center text-xs text-slate-500 mt-3 italic",
                "This information never leaves this page."
            }
        }
    }
}

#[component]
fn KeyIcon() -> Element {
    rsx! {
        svg {
            class: "w-5 h-5 text-cyan-400",
            fill: "none",
            stroke: "currentColor",
            stroke_width: "2",
            view_box: "0 0 24 24",
            path { d: "M21 2l-2 2m-7.61 7.61a5.5 5.5 0 1 1-7.778 7.778 5.5 5.5 0 0 1 7.777-7.777zm0 0L15.5 7.5m0 0l3 3L22 7l-3-3m-3.5 3.5L19 4" }
        }
    }
}

#[component]
fn CopyIcon() -> Element {
    rsx! {
        svg {
            class: "w-5 h-5",
            fill: "none",
            stroke: "currentColor",
            stroke_width: "2",
            view_box: "0 0 24 24",
            rect { x: "9", y: "9", width: "13", height: "13", rx: "2" }
            path { d: "M5 15H4a2 2 0 0 1-2-2V4a2 2 0 0 1 2-2h9a2 2 0 0 1 2 2v1" }
        }
    }
}

#[component]
fn CheckIcon() -> Element {
    rsx! {
        svg {
            class: "w-5 h-5 text-green-400",
            fill: "none",
            stroke: "currentColor",
            stroke_width: "2",
            view_box: "0 0 24 24",
            path { d: "M20 6 9 17l-5-5" }
        }
    }
}
