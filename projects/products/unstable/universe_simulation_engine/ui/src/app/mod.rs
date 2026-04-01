pub mod action;
pub mod app_state;
pub mod reducer;

#[cfg(test)]
mod tests;

#[cfg(target_arch = "wasm32")]
use dioxus::prelude::*;

#[cfg(target_arch = "wasm32")]
#[component]
pub fn app() -> Element {
    rsx! {
        main {
            h1 { "Universe Simulation Engine" }
            p { "Deterministic universe simulation UI (WASM/Dioxus)." }
        }
    }
}
