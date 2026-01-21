use dioxus::prelude::*;
use spectre::SPECTRE_COUNTER_DEFAULT;

#[component]
pub fn PasswordCounterSelector(counter: Signal<u32>) -> Element {
    rsx! {
        section {
            class: "mb-1",
            div {
                class: "flex items-center gap-2 mb-2",
                CounterIcon {}
                span {
                    class: "text-slate-300 font-medium",
                    "Password Counter"
                }
            }
            
            div {
                class: "flex items-center justify-center gap-4",
                button {
                    class: "w-12 h-12 rounded-full border border-slate-700 bg-slate-800/50 text-slate-400 hover:bg-slate-700 hover:text-white transition-all flex items-center justify-center",
                    onclick: move |_| {
                        let current = *counter.read();
                        if current > 1 {
                            counter.set(current - 1);
                        }
                    },
                    ChevronDown {}
                }
                
                div {
                    class: "flex-1 max-w-xs",
                    div {
                        class: "bg-slate-800/50 border border-slate-700 rounded-xl py-2 text-center",
                        span {
                            class: "text-2xl font-semibold text-white",
                            "{counter}"
                        }
                    }
                }
                
                button {
                    class: "w-12 h-12 rounded-full border border-slate-700 bg-slate-800/50 text-slate-400 hover:bg-slate-700 hover:text-white transition-all flex items-center justify-center",
                    onclick: move |_| {
                        let current = *counter.read();
                        counter.set(current + 1);
                    },
                    ChevronUp {}
                }
            }
            
            div {
                class: "text-xs text-slate-500 text-center mt-2",
                "Change counter to generate a new password for the same site"
            }
            
            if *counter.read() > 1 {
                button {
                    class: "block mx-auto mt-3 text-sm text-slate-500 hover:text-cyan-400 transition-colors",
                    onclick: move |_| counter.set(SPECTRE_COUNTER_DEFAULT),
                    "Reset to default (1)"
                }
            }
        }
    }
}

#[component]
fn CounterIcon() -> Element {
    rsx! {
        svg {
            class: "w-5 h-5 text-cyan-400",
            fill: "none",
            stroke: "currentColor",
            stroke_width: "2",
            view_box: "0 0 24 24",
            rect { x: "4", y: "4", width: "16", height: "16", rx: "2" }
            path { d: "M9 9h.01M15 9h.01M9 15h.01M15 15h.01" }
        }
    }
}

#[component]
fn ChevronDown() -> Element {
    rsx! {
        svg {
            class: "w-5 h-5",
            fill: "none",
            stroke: "currentColor",
            stroke_width: "2",
            view_box: "0 0 24 24",
            path { d: "m19 9-7 7-7-7" }
        }
    }
}

#[component]
fn ChevronUp() -> Element {
    rsx! {
        svg {
            class: "w-5 h-5",
            fill: "none",
            stroke: "currentColor",
            stroke_width: "2",
            view_box: "0 0 24 24",
            path { d: "m5 15 7-7 7 7" }
        }
    }
}

