#[cfg(target_arch = "wasm32")]
use crate::screens::campaign_screen::CampaignScreen;
#[cfg(target_arch = "wasm32")]
use dioxus::prelude::*;

#[cfg(target_arch = "wasm32")]
#[component]
pub fn App() -> Element {
    let mut seed = use_signal(|| 42_u64);
    let mut days = use_signal(|| 30_u32);
    let mut summary = use_signal(|| "No run yet".to_string());

    rsx! {
        main {
            class: "princeps-app",
            h1 { "Princeps" }
            p { "Deterministic campaign simulation" }

            section {
                h2 { "Run Simulation" }
                label {
                    "Seed"
                    input {
                        r#type: "number",
                        value: "{seed}",
                        oninput: move |event| {
                            if let Ok(parsed) = event.value().parse::<u64>() {
                                seed.set(parsed);
                            }
                        }
                    }
                }
                label {
                    "Days"
                    input {
                        r#type: "number",
                        value: "{days}",
                        oninput: move |event| {
                            if let Ok(parsed) = event.value().parse::<u32>() {
                                days.set(parsed);
                            }
                        }
                    }
                }
                button {
                    onclick: move |_| {
                        let screen = CampaignScreen::new(seed(), days(), "ready".to_string());
                        summary.set(screen.summary_line());
                    },
                    "Run"
                }
            }

            section {
                h2 { "Latest Run Summary" }
                p { "{summary}" }
            }
        }
    }
}
