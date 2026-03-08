#[cfg(not(target_arch = "wasm32"))]
fn main() {
    println!("diplo_sim_ui (wasm build target)");
}

#[cfg(target_arch = "wasm32")]
fn main() {
    dioxus::launch(App);
}

#[cfg(target_arch = "wasm32")]
#[allow(non_snake_case)]
fn App() -> dioxus::prelude::Element {
    use dioxus::prelude::*;
    rsx! {
        main {
            h1 { "Diplo Sim" }
        }
    }
}
