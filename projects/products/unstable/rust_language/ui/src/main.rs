mod models;

#[cfg(test)]
mod tests;

#[cfg(target_arch = "wasm32")]
fn main() {
    dioxus::launch(app);
}

#[cfg(not(target_arch = "wasm32"))]
fn main() {
    println!("rust_language_ui (wasm build only)");
    println!("Use `dx serve` with wasm32 target to launch the editor.");
}

#[cfg(target_arch = "wasm32")]
fn app() -> dioxus::prelude::Element {
    use dioxus::prelude::*;

    let mut source_code = use_signal(|| String::from("fn main() {\n    let x = 42;\n}\n"));
    let mut transpiled_output = use_signal(String::new);
    let mut error_output = use_signal(String::new);

    let on_transpile = move |_| {
        let code = source_code.read().clone();
        match compile_rhl(&code) {
            Ok(rust_code) => {
                transpiled_output.set(rust_code);
                error_output.set(String::new());
            }
            Err(msg) => {
                transpiled_output.set(String::new());
                error_output.set(msg);
            }
        }
    };

    rsx! {
        div { style: "font-family: monospace; padding: 20px;",
            h1 { "RHL — Rust High Language Editor" }
            div { style: "display: flex; gap: 20px;",
                div { style: "flex: 1;",
                    h2 { "Source (.rhl)" }
                    textarea {
                        style: "width: 100%; height: 300px; font-family: monospace;",
                        value: "{source_code}",
                        oninput: move |evt| source_code.set(evt.value()),
                    }
                    button {
                        style: "margin-top: 10px; padding: 8px 16px;",
                        onclick: on_transpile,
                        "Transpile to Rust"
                    }
                }
                div { style: "flex: 1;",
                    h2 { "Transpiled Rust" }
                    pre {
                        style: "background: #f4f4f4; padding: 10px; min-height: 300px; white-space: pre-wrap;",
                        "{transpiled_output}"
                    }
                }
            }
            if !error_output.read().is_empty() {
                div { style: "color: red; margin-top: 10px; padding: 10px; background: #fff0f0;",
                    h3 { "Errors" }
                    pre { "{error_output}" }
                }
            }
        }
    }
}

#[cfg(target_arch = "wasm32")]
fn compile_rhl(source: &str) -> Result<String, String> {
    use models::editor_state::EditorState;
    let state = EditorState::new(source.to_string());
    state.validate().map_err(|e| e.to_string())?;
    Ok(format!("// Transpiled from RHL\n{source}"))
}
