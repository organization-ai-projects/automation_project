#[cfg(test)]
mod tests;

#[cfg(target_arch = "wasm32")]
fn main() {
    dioxus::launch(app);
}

#[cfg(not(target_arch = "wasm32"))]
fn main() {
    println!("econ_sandbox_ui (wasm build only)");
}

#[cfg(target_arch = "wasm32")]
fn app() -> dioxus::prelude::Element {
    use dioxus::prelude::*;
    rsx! {
        div {
            h1 { "Econ Sandbox" }
            p { "Deterministic economy simulator - coming soon" }
        }
    }
}
