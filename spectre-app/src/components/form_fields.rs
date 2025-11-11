use dioxus::prelude::*;

#[component]
pub fn FullNameInput(full_name: Signal<String>) -> Element {
    rsx! {
        div {
            label {
                class: "flex items-center gap-2 text-slate-400 text-xs tracking-widest mb-2",
                "YOUR FULL NAME"
                div {
                    class: "group relative",
                    span {
                        class: "text-slate-500 cursor-help",
                        "?"
                    }
                    div {
                        class: "absolute left-0 top-6 hidden w-64 rounded-xl border border-slate-600 bg-slate-800/95 p-3 text-xs font-normal text-slate-300 shadow-xl group-hover:block z-50",
                        p {
                            class: "text-slate-300",
                            strong { "Your full name" }
                            " is used to generate your master key. Use the same name consistently across all your devices."
                        }
                        p {
                            class: "text-slate-400 mt-2",
                            "Example: Robert Lee Mitchell"
                        }
                    }
                }
            }
            input {
                class: "w-full bg-slate-700/40 text-slate-300 placeholder-slate-500 px-6 py-4 rounded-full border border-slate-600/50 focus:outline-none focus:border-cyan-400/50 focus:bg-slate-700/60 transition",
                r#type: "text",
                placeholder: "eg. Robert Lee Mitchell",
                value: "{full_name}",
                oninput: move |e| full_name.set(e.value())
            }
        }
    }
}

#[component]
pub fn SpectreSecretInput(
    secret: Signal<String>,
    on_blur: EventHandler<()>,
) -> Element {
    rsx! {
        div {
            div {
                class: "flex items-center justify-between mb-2",
                label {
                    class: "flex items-center gap-2 text-slate-400 text-xs tracking-widest",
                    "YOUR SPECTRE SECRET"
                    div {
                        class: "group relative",
                        span {
                            class: "text-slate-500 cursor-help",
                            "?"
                        }
                        div {
                            class: "absolute left-0 top-6 hidden w-64 rounded-xl border border-slate-600 bg-slate-800/95 p-3 text-xs font-normal text-slate-300 shadow-xl group-hover:block z-50",
                            p {
                                class: "text-slate-300",
                                strong { "Your master password" }
                                " is the only password you need to remember. It's combined with your name to generate unique passwords for each site."
                            }
                            p {
                                class: "text-slate-400 mt-2",
                                "💡 Tip: Use a memorable phrase like \"banana colored duckling\""
                            }
                            p {
                                class: "text-yellow-400 mt-2",
                                "⚠️ Never share this with anyone!"
                            }
                        }
                    }
                }
                span { class: "text-2xl", "🔒" }
            }
            input {
                class: "w-full bg-slate-700/40 text-slate-300 placeholder-slate-500 px-6 py-4 rounded-full border border-slate-600/50 focus:outline-none focus:border-cyan-400/50 focus:bg-slate-700/60 transition",
                r#type: "password",
                placeholder: "eg. banana colored duckling",
                value: "{secret}",
                oninput: move |e| secret.set(e.value()),
                onblur: move |_| on_blur.call(())
            }
        }
    }
}

#[component]
pub fn SiteDomainInput(
    site_domain: Signal<String>,
    is_computing_key: Signal<bool>,
    on_focus: EventHandler<()>,
) -> Element {
    rsx! {
        div {
            div {
                class: "flex items-center justify-center mb-2",
                span { class: "text-cyan-400 text-2xl", "↓" }
            }
            label {
                class: "flex items-center justify-between gap-2 text-slate-400 text-xs tracking-widest mb-2",
                div {
                    class: "flex items-center gap-2",
                    "SITE DOMAIN"
                    div {
                        class: "group relative",
                        span {
                            class: "text-slate-500 cursor-help",
                            "?"
                        }
                        div {
                            class: "absolute left-0 top-6 hidden w-72 rounded-xl border border-slate-600 bg-slate-800/95 p-3 text-xs font-normal text-slate-300 shadow-xl group-hover:block z-50",
                            p {
                                class: "text-slate-300",
                                strong { "The website domain" }
                                " you want to generate a password for. Each site gets a unique password."
                            }
                            p {
                                class: "text-slate-400 mt-2",
                                "Examples: github.com, google.com, facebook.com"
                            }
                            p {
                                class: "text-cyan-400 mt-2",
                                "💡 Use consistent domain names (e.g., always \"google.com\", not \"www.google.com\")"
                            }
                        }
                    }
                }
                if *is_computing_key.read() {
                    span {
                        class: "inline-flex items-center gap-2 rounded-full bg-yellow-400/10 px-3 py-1 text-[11px] font-medium text-yellow-200",
                        span { class: "animate-spin", "⏳" }
                        "Computing key"
                    }
                }
            }
            input {
                class: if *is_computing_key.read() {
                    "w-full bg-slate-700/40 text-slate-300 placeholder-slate-500 px-6 py-4 rounded-full border border-yellow-400/50 focus:outline-none focus:border-yellow-400/50 focus:bg-slate-700/60 transition"
                } else {
                    "w-full bg-slate-700/40 text-slate-300 placeholder-slate-500 px-6 py-4 rounded-full border border-slate-600/50 focus:outline-none focus:border-cyan-400/50 focus:bg-slate-700/60 transition"
                },
                r#type: "text",
                placeholder: if *is_computing_key.read() {
                    "eg. wikipedia.org (computing key in background...)"
                } else {
                    "eg. wikipedia.org"
                },
                value: "{site_domain}",
                oninput: move |e| site_domain.set(e.value()),
                onfocus: move |_| on_focus.call(()),
            }
            if *is_computing_key.read() {
                p {
                    class: "text-xs text-yellow-200/90 mt-2 pl-1",
                    "⚡ Generating secure encryption key…"
                }
            }
        }
    }
}

