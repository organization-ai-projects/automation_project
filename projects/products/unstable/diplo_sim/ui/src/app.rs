#[cfg(target_arch = "wasm32")]
use dioxus::prelude::*;

#[cfg(target_arch = "wasm32")]
use crate::screens::match_screen::MatchScreen;

#[cfg(target_arch = "wasm32")]
#[allow(non_snake_case)]
pub fn App() -> Element {
    rsx! {
        main {
            MatchScreen {}
        }
    }
}
