#[cfg(not(target_arch = "wasm32"))]
mod app;
#[cfg(not(target_arch = "wasm32"))]
mod ui_error;

#[cfg(not(target_arch = "wasm32"))]
fn main() {
    if let Err(error) = app::run() {
        eprintln!("error: {error}");
        std::process::exit(1);
    }
}

#[cfg(target_arch = "wasm32")]
mod app;

#[cfg(target_arch = "wasm32")]
fn main() {
    dioxus::launch(app::app);
}
