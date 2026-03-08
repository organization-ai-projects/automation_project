#[cfg(target_arch = "wasm32")]
mod app;
#[cfg(target_arch = "wasm32")]
mod components;
#[cfg(target_arch = "wasm32")]
mod diagnostics;
#[cfg(target_arch = "wasm32")]
mod screens;
#[cfg(target_arch = "wasm32")]
mod transport;

#[cfg(target_arch = "wasm32")]
use app::render;

#[cfg(not(target_arch = "wasm32"))]
fn main() {
    println!("docforge_ui (wasm build target)");
}

#[cfg(target_arch = "wasm32")]
fn main() {
    dioxus::launch(render);
}
