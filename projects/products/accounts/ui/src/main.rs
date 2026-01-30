#[cfg(not(target_arch = "wasm32"))]
fn main() {
    println!("accounts-ui (wasm build only)");
}

#[cfg(target_arch = "wasm32")]
fn main() {
    run();
}

#[cfg(target_arch = "wasm32")]
mod setup_admin_input;

#[cfg(target_arch = "wasm32")]
mod parse_json;

#[cfg(target_arch = "wasm32")]
mod components;

#[cfg(target_arch = "wasm32")]
mod permission_picker;

#[cfg(target_arch = "wasm32")]
mod user_actions;

#[cfg(target_arch = "wasm32")]
mod app;

#[cfg(target_arch = "wasm32")]
fn run() {
    dioxus::launch(app::app);
}
