#[cfg(not(target_arch = "wasm32"))]
fn main() {
    eprintln!("neurosymbolic_moe_ui: build this crate for wasm32 to run the Dioxus UI");
}

#[cfg(target_arch = "wasm32")]
mod app;

#[cfg(target_arch = "wasm32")]
fn main() {
    app::launch();
}

#[cfg(test)]
mod tests;
