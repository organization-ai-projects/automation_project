#[cfg(test)]
mod tests;

#[cfg(target_arch = "wasm32")]
fn main() {
    dioxus::launch(app);
}

#[cfg(not(target_arch = "wasm32"))]
fn main() {
    println!("roguelite_arena_ui (wasm build only)");
}

#[cfg(target_arch = "wasm32")]
fn app() -> dioxus::prelude::Element {
    use dioxus::prelude::*;
    rsx! {
        div {
            h1 { "Roguelite Arena" }
            p { "Deterministic roguelite arena simulation - coming soon" }
        }
    }
}
