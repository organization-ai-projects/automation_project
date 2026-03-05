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
            h1 { "colony_manager_ui" }
            p { "Deterministic simulation UI (WASM/Dioxus)." }
        }
    }
}
