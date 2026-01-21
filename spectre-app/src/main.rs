use dioxus::prelude::*;
use spectre::{spectre_user_key, spectre_site_result, spectre_identicon_render, spectre_identicon_preview};
use spectre::{SpectreResultType, SpectreKeyPurpose, SPECTRE_ALGORITHM_CURRENT, SpectreUserKey, SPECTRE_COUNTER_DEFAULT};
use gloo_timers::future::sleep;
use std::time::Duration;

mod components;
mod worker;
use crate::components::*;
use crate::worker::KeyWorker;

#[derive(Debug, Clone, Routable, PartialEq)]
#[rustfmt::skip]
enum Route {
    #[route("/")]
    Home {},
}

const FAVICON: Asset = asset!("/assets/favicon.png");
const TAILWIND_CSS: Asset = asset!("/assets/tailwind.css");

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    rsx! {
        document::Link { rel: "icon", href: FAVICON }
        document::Link { rel: "stylesheet", href: TAILWIND_CSS }
        style { "body {{ background-color: #0f172a; }}" } // bg-slate-900 equivalent
        Router::<Route> {}
    }
}

/// Home page - Spectre Landing Page
#[component]
fn Home() -> Element {
    let full_name = use_signal(|| String::new());
    let secret = use_signal(|| String::new());
    let site_domain = use_signal(|| String::new());
    let password_type = use_signal(|| SpectreResultType::LongPassword);
    let counter = use_signal(|| SPECTRE_COUNTER_DEFAULT);
    let mut generated_password = use_signal(|| String::new());
    let mut identicon = use_signal(|| String::new());
    let mut is_generating = use_signal(|| false);
    let mut is_computing_key = use_signal(|| false);
    
    // Cache the user key to avoid recomputing scrypt for every site
    // The expensive operation (scrypt) only runs when name or secret changes
    let mut cached_user_key = use_signal(|| Option::<(String, String, SpectreUserKey)>::None);
    
    // Initialize Web Worker for background key generation
    // Falls back to main thread if worker initialization fails
    // Use Arc to share the worker across async contexts
    use std::sync::Arc;
    let key_worker = use_signal(|| {
        KeyWorker::new().ok().map(Arc::new)
    });

    // Compute validation states (memoized)
    let _name_valid = use_memo(move || {
        let name = full_name.read();
        !name.is_empty() && name.len() >= 3
    });
    let _secret_valid = use_memo(move || {
        let sec = secret.read();
        !sec.is_empty() && sec.len() >= 4
    });
    let _domain_valid = use_memo(move || {
        let site = site_domain.read();
        !site.is_empty() && site.len() >= 3
    });

    // Track when to trigger key computation
    let mut trigger_key_computation = use_signal(|| 0u32);
    
    // Preview identicon - shows immediately as user types (uses fast SHA256)
    // This gives instant visual feedback before the expensive scrypt computation
    use_effect(move || {
        let name = full_name();
        let sec = secret();
        
        // Validate name and secret
        let name_is_valid = !name.is_empty() && name.len() >= 3;
        let secret_is_valid = !sec.is_empty() && sec.len() >= 4;
        
        if name_is_valid && secret_is_valid {
            // Generate fast preview identicon (SHA256 - instant)
            let preview_bytes = spectre_identicon_preview(&name, &sec);
            identicon.set(spectre_identicon_render(preview_bytes));
        } else {
            identicon.set(String::new());
        }
    });
    
    // Eager user key generation - starts when user focuses on site field or leaves secret/name field
    // This precomputes the expensive scrypt operation before user tries to type
    use_effect(move || {
        let trigger = trigger_key_computation();
        if trigger == 0 {
            return; // Don't compute on initial render
        }
        
        // Capture values once when trigger changes - don't subscribe to signal changes
        let name = full_name.peek().clone();
        let sec = secret.peek().clone();
        let worker_ref = (*key_worker.peek()).clone(); // Clone the Arc reference
        
        spawn(async move {
            // Wait a bit to let the UI update
            sleep(Duration::from_millis(100)).await;
            
            // Validate name and secret
            let name_is_valid = !name.is_empty() && name.len() >= 3;
            let secret_is_valid = !sec.is_empty() && sec.len() >= 4;
            
            // If valid, precompute the user key (expensive scrypt operation)
            if name_is_valid && secret_is_valid {
                // Check if we already have this cached
                let needs_computation = {
                    let cache = cached_user_key.read();
                    if let Some((ref cached_name, ref cached_secret, _)) = *cache {
                        cached_name != &name || cached_secret != &sec
                    } else {
                        true
                    }
                };
                
                if needs_computation {
                    // Keep preview identicon visible while computing real key
                    // Show that we're computing the key
                    is_computing_key.set(true);
                    
                    // Small delay to let the UI update
                    sleep(Duration::from_millis(50)).await;
                    
                    // Try to use Web Worker for background computation, fallback to main thread
                    let key_result = if let Some(ref worker) = worker_ref {
                        // Use Web Worker (runs in background thread)
                        // If worker fails, fallback to main thread
                        worker.generate_key(name.clone(), sec.clone()).await
                            .or_else(|_| {
                                // Worker failed, fallback to main thread
                                spectre_user_key(&name, &sec, SPECTRE_ALGORITHM_CURRENT)
                                    .map_err(|e| format!("Key generation failed: {:?}", e))
                            })
                    } else {
                        // Fallback to main thread if worker not available
                        spectre_user_key(&name, &sec, SPECTRE_ALGORITHM_CURRENT)
                            .map_err(|e| format!("Key generation failed: {:?}", e))
                    };
                    
                    if let Ok(key) = key_result {
                        cached_user_key.set(Some((name.clone(), sec.clone(), key.clone())));
                        
                        // Generate identicon immediately after key is computed
                        let mut identicon_bytes = [0u8; 4];
                        identicon_bytes.copy_from_slice(&key.key_data[0..4]);
                        identicon.set(spectre_identicon_render(identicon_bytes));
                    }
                    
                    // Always reset the computing state, even on error
                    is_computing_key.set(false);
                }
            } else {
                // Clear identicon if inputs are invalid
                identicon.set(String::new());
            }
        });
    });
    
    // Password generation - uses the precomputed user key
    // Note: Spectre uses scrypt with N=32768 which is computationally expensive (by design for security)
    // We precompute this in the effect above, so password generation is fast
    use_effect(move || {
        // Clone the values we need
        let name = full_name();
        let sec = secret();
        let site = site_domain();
        let result_type = password_type();
        let counter_value = counter();
        let worker_ref = (*key_worker.read()).clone(); // Clone the Arc reference
        
        // Spawn an async task with a delay
        spawn(async move {
            // Wait for 500ms before generating the password (shorter now since key is precomputed)
            sleep(Duration::from_millis(500)).await;
            
            // Validate fields
            let name_is_valid = !name.is_empty() && name.len() >= 3;
            let secret_is_valid = !sec.is_empty() && sec.len() >= 4;
            let domain_is_valid = !site.is_empty() && site.len() >= 3;

            // Generate password if all fields are valid
            if name_is_valid && secret_is_valid && domain_is_valid {
                // Check if we have a cached user key for this name+secret combination
                let user_key: Option<SpectreUserKey> = {
                    let cache = cached_user_key.read();
                    if let Some((ref cached_name, ref cached_secret, ref key)) = *cache {
                        if cached_name == &name && cached_secret == &sec {
                            // Cache hit! Use the precomputed key (no scrypt computation needed)
                            Some(key.clone())
                        } else {
                            None
                        }
                    } else {
                        None
                    }
                };
                
                // If no cache hit, compute the user key (expensive scrypt operation)
                // This should rarely happen since we precompute the key above
                let user_key = if let Some(key) = user_key {
                    Ok(key)
                } else {
                    is_generating.set(true);
                    // Small delay to show loading state
                    sleep(Duration::from_millis(50)).await;
                    
                    // Try to use Web Worker for background computation, fallback to main thread
                    let result = if let Some(ref worker) = worker_ref {
                        // Use Web Worker (runs in background thread)
                        worker.generate_key(name.clone(), sec.clone()).await
                    } else {
                        // Fallback to main thread if worker not available
                        spectre_user_key(&name, &sec, SPECTRE_ALGORITHM_CURRENT)
                            .map_err(|e| format!("Key generation failed: {:?}", e))
                    };
                    
                    if let Ok(ref key) = result {
                        // Cache the result for future use
                        cached_user_key.set(Some((name.clone(), sec.clone(), key.clone())));
                        
                        // Generate identicon if not already set
                        if identicon.peek().is_empty() {
                            let mut identicon_bytes = [0u8; 4];
                            identicon_bytes.copy_from_slice(&key.key_data[0..4]);
                            identicon.set(spectre_identicon_render(identicon_bytes));
                        }
                    }
                    result
                };
                
                if let Ok(user_key) = user_key {
                    if let Ok(password) = spectre_site_result(
                        &user_key,
                        &site,
                        result_type,
                        None,
                        counter_value,
                        SpectreKeyPurpose::Authentication,
                        None,
                    ) {
                        generated_password.set(password);
                    } else {
                        generated_password.set(String::new());
                    }
                } else {
                    generated_password.set(String::new());
                }
                
                is_generating.set(false);
            } else {
                generated_password.set(String::new());
                is_generating.set(false);
            }
        });
    });

    rsx! {
        div {
            class: "min-h-screen bg-slate-900 flex items-center justify-center p-4 font-sans",
            
            div {
                class: "w-full max-w-2xl",
                
                // Main Card
                div {
                    class: "bg-slate-900/80 backdrop-blur-sm border border-slate-800 rounded-2xl p-8 shadow-2xl space-y-6",
                    
                    Header {}
                    
                    PasswordTypeSelector { password_type }
                    
                    PasswordCounterSelector { counter }
                    
                    // Divider
                    div { class: "border-t border-slate-800 my-2" }
                    
                    // Identity Section
                    div {
                        class: "space-y-5",
                        
                        FullNameInput {
                            full_name,
                            on_blur: move |_| {
                                // Trigger key computation when user leaves full name field
                                trigger_key_computation.set(trigger_key_computation() + 1);
                            }
                        }
                        
                        SpectreSecretInput {
                            secret,
                            on_blur: move |_| {
                                // Trigger key computation when user leaves secret field
                                trigger_key_computation.set(trigger_key_computation() + 1);
                            }
                        }
                        
                        IdenticonDisplay { identicon }
                        
                        SiteDomainInput {
                            site_domain,
                            is_computing_key,
                            on_focus: move |_| {
                                // Trigger key computation when user focuses site field
                                trigger_key_computation.set(trigger_key_computation() + 1);
                            }
                        }
                    }
                    
                    // Divider
                    div { class: "border-t border-slate-800 my-6" }
                    
                    SitePassword {
                        generated_password,
                        is_generating
                    }
                }
                
                // Footer
                p {
                    class: "text-center text-xs text-slate-500 mt-4",
                    "Powered by the Spectre algorithm for deterministic password generation"
                }
            }
        }
    }
}
