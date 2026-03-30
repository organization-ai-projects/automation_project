mod app;
mod components;
mod screens;
mod state;
mod transport;

#[cfg(test)]
mod tests;

#[cfg(target_arch = "wasm32")]
fn main() {
    dioxus::launch(app::app);
}

#[cfg(not(target_arch = "wasm32"))]
fn main() {
    println!("wasm_console_ui (wasm build only)");
}
