use dioxus::prelude::*;

#[component]
pub fn FullNameInput(
    full_name: Signal<String>,
    on_blur: EventHandler<()>,
) -> Element {
    rsx! {
        div {
            label {
                class: "block text-xs tracking-wider text-slate-500 mb-2",
                "Your Full Name"
            }
            input {
                class: "w-full bg-slate-800/50 border border-slate-700 rounded-xl px-4 py-2 text-white placeholder-slate-500 focus:outline-none focus:ring-2 focus:ring-cyan-500/50 focus:border-cyan-500/50 transition-all",
                r#type: "text",
                value: "{full_name}",
                placeholder: "Enter your full name",
                oninput: move |evt| full_name.set(evt.value()),
                onblur: move |_| on_blur.call(()),
            }
        }
    }
}

#[component]
pub fn SpectreSecretInput(
    secret: Signal<String>,
    on_blur: EventHandler<()>,
) -> Element {
    let mut show_secret = use_signal(|| false);
    
    rsx! {
        div {
            label {
                class: "block text-xs tracking-wider text-slate-500 mb-2",
                "Your Spectre Secret (Your known password)"
            }
            div {
                class: "relative",
                input {
                    class: "w-full bg-slate-800/50 border border-slate-700 rounded-xl px-4 py-2 pr-12 text-white placeholder-slate-500 focus:outline-none focus:ring-2 focus:ring-cyan-500/50 focus:border-cyan-500/50 transition-all",
                    r#type: if show_secret() { "text" } else { "password" },
                    value: "{secret}",
                    placeholder: "Enter your secret",
                    oninput: move |evt| secret.set(evt.value()),
                    onblur: move |_| on_blur.call(()),
                }
                button {
                    class: "absolute right-3 top-1/2 -translate-y-1/2 text-slate-500 hover:text-cyan-400 transition-colors",
                    r#type: "button",
                    onclick: move |_| {
                        let current = *show_secret.read();
                        show_secret.set(!current);
                    },
                    if show_secret() {
                        EyeOffIcon {}
                    } else {
                        EyeIcon {}
                    }
                }
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
            label {
                class: "block text-xs tracking-wider text-slate-500 mb-2",
                "Site Domain"
            }
            input {
                class: "w-full bg-slate-800/50 border border-slate-700 rounded-xl px-4 py-2 text-white placeholder-slate-500 focus:outline-none focus:ring-2 focus:ring-cyan-500/50 focus:border-cyan-500/50 transition-all",
                r#type: "text",
                value: "{site_domain}",
                placeholder: "example.com",
                oninput: move |evt| site_domain.set(evt.value()),
                onfocus: move |_| on_focus.call(()),
            }
        }
    }
}

#[component]
fn EyeIcon() -> Element {
    rsx! {
        svg {
            class: "w-5 h-5",
            fill: "none",
            stroke: "currentColor",
            stroke_width: "2",
            view_box: "0 0 24 24",
            path { d: "M2 12s3-7 10-7 10 7 10 7-3 7-10 7-10-7-10-7Z" }
            circle { cx: "12", cy: "12", r: "3" }
        }
    }
}

#[component]
fn EyeOffIcon() -> Element {
    rsx! {
        svg {
            class: "w-5 h-5",
            fill: "none",
            stroke: "currentColor",
            stroke_width: "2",
            view_box: "0 0 24 24",
            path { d: "M9.88 9.88a3 3 0 1 0 4.24 4.24" }
            path { d: "M10.73 5.08A10.43 10.43 0 0 1 12 5c7 0 10 7 10 7a13.16 13.16 0 0 1-1.67 2.68" }
            path { d: "M6.61 6.61A13.526 13.526 0 0 0 2 12s3 7 10 7a9.74 9.74 0 0 0 5.39-1.61" }
            line { x1: "2", x2: "22", y1: "2", y2: "22" }
        }
    }
}
