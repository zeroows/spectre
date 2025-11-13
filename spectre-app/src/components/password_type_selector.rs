use dioxus::prelude::*;
use spectre::SpectreResultType;

#[component]
pub fn PasswordTypeSelector(password_type: Signal<SpectreResultType>) -> Element {
    rsx! {
        div {
            class: "mb-8",
            div {
                class: "flex items-center gap-2 mb-4",
                span { class: "text-2xl", "🎯" }
                h2 { class: "text-white text-xl font-light", "Password Type" }
            }
            div {
                class: "grid grid-cols-2 gap-3 mb-6",
                
                PasswordTypeButton {
                    password_type: password_type,
                    result_type: SpectreResultType::LongPassword,
                    label: "LONG PASSWORD"
                }
                
                PasswordTypeButton {
                    password_type: password_type,
                    result_type: SpectreResultType::MediumPassword,
                    label: "MEDIUM"
                }
                
                PasswordTypeButton {
                    password_type: password_type,
                    result_type: SpectreResultType::BasicPassword,
                    label: "BASIC"
                }
                
                PasswordTypeButton {
                    password_type: password_type,
                    result_type: SpectreResultType::ShortPassword,
                    label: "SHORT"
                }
                
                PasswordTypeButton {
                    password_type: password_type,
                    result_type: SpectreResultType::PIN,
                    label: "PIN"
                }
                
                PasswordTypeButton {
                    password_type: password_type,
                    result_type: SpectreResultType::Name,
                    label: "NAME"
                }
                
                PasswordTypeButton {
                    password_type: password_type,
                    result_type: SpectreResultType::Phrase,
                    label: "PHRASE"
                }
                
                PasswordTypeButton {
                    password_type: password_type,
                    result_type: SpectreResultType::MaximumSecurityPassword,
                    label: "MAXIMUM"
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
    
    rsx! {
        button {
            class: if is_selected {
                "bg-cyan-400 text-slate-900 py-3 px-6 rounded-full font-medium tracking-wide hover:bg-cyan-300 transition"
            } else {
                "bg-slate-700/50 text-slate-300 py-3 px-6 rounded-full font-medium tracking-wide hover:bg-slate-600/50 transition border border-slate-600"
            },
            onclick: move |_| password_type.set(result_type),
            "{label}"
        }
    }
}

