// projects/products/unstable/colony_manager/ui/src/main.rs
mod app;

#[cfg(not(target_arch = "wasm32"))]
mod public_api;
#[cfg(not(target_arch = "wasm32"))]
mod ui_error;

#[cfg(not(target_arch = "wasm32"))]
use public_api::UiApi;

#[cfg(not(target_arch = "wasm32"))]
fn main() {
    if let Err(error) = UiApi::run_from_args(std::env::args().collect()) {
        eprintln!("error: {error}");
        std::process::exit(error.exit_code());
    }
}

#[cfg(target_arch = "wasm32")]
fn main() {
    dioxus::launch(app::app);
}
