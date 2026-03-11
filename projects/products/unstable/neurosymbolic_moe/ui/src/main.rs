#[cfg(target_arch = "wasm32")]
fn main() {
    dioxus::launch(|| dioxus::prelude::rsx! { div { "neurosymbolic_moe" } });
}

#[cfg(not(target_arch = "wasm32"))]
fn main() {
    println!("neurosymbolic_moe_ui (native stub)");
}
