use dioxus::prelude::*;
use spectre::SpectreResultType;

#[component]
pub fn PasswordTypeSelector(password_type: Signal<SpectreResultType>) -> Element {
    rsx! {
        section {
            class: "mb-3",
            div {
                class: "flex items-center gap-2 mb-2",
                TargetIcon {}
                span {
                    class: "text-slate-300 font-medium",
                    "Password Type"
                }
            }
            
            div {
                class: "grid grid-cols-4 gap-2",
                PasswordTypeButton {
                    password_type: password_type,
                    result_type: SpectreResultType::LongPassword,
                    label: "Long Password"
                }
                PasswordTypeButton {
                    password_type: password_type,
                    result_type: SpectreResultType::MediumPassword,
                    label: "Medium"
                }
                PasswordTypeButton {
                    password_type: password_type,
                    result_type: SpectreResultType::BasicPassword,
                    label: "Basic"
                }
                PasswordTypeButton {
                    password_type: password_type,
                    result_type: SpectreResultType::ShortPassword,
                    label: "Short"
                }
                PasswordTypeButton {
                    password_type: password_type,
                    result_type: SpectreResultType::PIN,
                    label: "PIN"
                }
                PasswordTypeButton {
                    password_type: password_type,
                    result_type: SpectreResultType::Name,
                    label: "Name"
                }
                PasswordTypeButton {
                    password_type: password_type,
                    result_type: SpectreResultType::Phrase,
                    label: "Phrase"
                }
                PasswordTypeButton {
                    password_type: password_type,
                    result_type: SpectreResultType::MaximumSecurityPassword,
                    label: "Maximum"
                }
            }
        }
    }
}

#[component]
fn PasswordTypeButton(
    password_type: Signal<SpectreResultType>,
    result_type: SpectreResultType,
    label: &'static str,
) -> Element {
    let is_selected = *password_type.read() == result_type;
    
    let base_class = "py-2 px-4 rounded-xl text-sm font-medium transition-all border";
    let state_class = if is_selected {
        "bg-gradient-to-r from-cyan-500 to-teal-500 text-slate-900 border-transparent shadow-lg shadow-cyan-500/25"
    } else {
        "bg-slate-800/50 text-slate-400 border-slate-700 hover:bg-slate-700 hover:text-white"
    };

    rsx! {
        button {
            class: "{base_class} {state_class}",
            onclick: move |_| password_type.set(result_type),
            "{label}"
        }
    }
}

#[component]
fn TargetIcon() -> Element {
    rsx! {
        svg {
            class: "w-5 h-5 text-cyan-400",
            fill: "none",
            stroke: "currentColor",
            stroke_width: "2",
            view_box: "0 0 24 24",
            circle { cx: "12", cy: "12", r: "10" }
            circle { cx: "12", cy: "12", r: "6" }
            circle { cx: "12", cy: "12", r: "2" }
        }
    }
}

