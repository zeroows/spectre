use dioxus::prelude::*;
use spectre::SPECTRE_COUNTER_DEFAULT;

#[component]
pub fn PasswordCounterSelector(counter: Signal<u32>) -> Element {
    rsx! {
        div {
            class: "mb-8",
            div {
                class: "flex items-center gap-2 mb-4",
                span { class: "text-2xl", "🔢" }
                h2 { class: "text-white text-xl font-light", "Password Counter" }
            }
            div {
                class: "flex items-center gap-4 mb-6",
                
                // Decrement button
                button {
                    class: "bg-slate-700/50 text-slate-300 w-24 h-24 rounded-full font-bold text-xl hover:bg-slate-600/50 transition border border-slate-600 disabled:opacity-50 disabled:cursor-not-allowed",
                    disabled: *counter.read() == 1,
                    onclick: move |_| {
                        let current = *counter.read();
                        if current > 1 {
                            counter.set(current - 1);
                        }
                    },
                    "🔽"
                }
                
                // Counter display and input
                div {
                    class: "flex-1",
                    div {
                        class: "relative",
                        input {
                            r#type: "number",
                            class: "w-full bg-slate-700/50 text-white text-center text-2xl py-3 px-6 rounded-full border border-slate-600 focus:outline-none focus:border-cyan-400 focus:ring-2 focus:ring-cyan-400/20 transition",
                            value: "{counter}",
                            min: 1,
                            max: u32::MAX,
                            oninput: move |evt| {
                                if let Ok(value) = evt.value().parse::<u32>() {
                                    if value >= 1 {
                                        counter.set(value);
                                    }
                                }
                            }
                        }
                        div {
                            class: "text-center text-slate-400 text-sm mt-2",
                            "Change counter to generate a new password for the same site"
                        }
                    }
                }
                
                // Increment button
                button {
                    class: "bg-slate-700/50 text-slate-300 w-24 h-24 rounded-full font-bold text-xl hover:bg-slate-600/50 transition border border-slate-600",
                    onclick: move |_| {
                        let current = *counter.read();
                        counter.set(current + 1);
                    },
                    "🔼"
                }
            }
            
            // Reset button
            div {
                class: "flex justify-center mt-4",
                button {
                    class: "text-slate-400 text-sm hover:text-cyan-400 transition",
                    onclick: move |_| counter.set(SPECTRE_COUNTER_DEFAULT),
                    "Reset to default (1)"
                }
            }
        }
    }
}

