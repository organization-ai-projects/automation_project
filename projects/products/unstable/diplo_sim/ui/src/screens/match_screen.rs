#[cfg(target_arch = "wasm32")]
use dioxus::prelude::*;

pub fn screen_subtitle() -> &'static str {
    "Deterministic diplomacy simulation UI"
}

#[cfg(target_arch = "wasm32")]
#[allow(non_snake_case)]
pub fn MatchScreen() -> Element {
    rsx! {
        section {
            h1 { "Diplo Sim" }
            p { "{screen_subtitle()}" }
        }
    }
}
