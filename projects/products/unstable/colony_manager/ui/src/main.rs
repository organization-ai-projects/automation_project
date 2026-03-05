// projects/products/unstable/colony_manager/ui/src/main.rs
mod app;

#[cfg(not(target_arch = "wasm32"))]
mod controller;
#[cfg(not(target_arch = "wasm32"))]
mod ui_error;

#[cfg(not(target_arch = "wasm32"))]
use controller::Controller;
#[cfg(not(target_arch = "wasm32"))]
use ui_error::UiError;

#[cfg(not(target_arch = "wasm32"))]
fn main() {
    if let Err(error) = run_native(std::env::args().collect()) {
        eprintln!("error: {error}");
        std::process::exit(error.exit_code());
    }
}

#[cfg(not(target_arch = "wasm32"))]
fn run_native(args: Vec<String>) -> Result<(), UiError> {
    if args.len() < 2 {
        return Err(UiError::Usage);
    }
    Controller::run_command(&args[1], &args[2..])
}

#[cfg(target_arch = "wasm32")]
fn main() {
    dioxus::launch(app::app);
}
