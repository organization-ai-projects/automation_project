//! projects/products/stable/code_agent_sandbox/ui/src/app.rs
#[cfg(target_arch = "wasm32")]
use dioxus::prelude::*;

#[cfg(target_arch = "wasm32")]
const APP_STYLE: &str = r#"
:root { color-scheme: light; }
body { font-family: ui-sans-serif, system-ui, sans-serif; margin: 0; background: #f4f6f8; color: #0f172a; }
.page { max-width: 880px; margin: 0 auto; padding: 24px; }
.card { background: white; border: 1px solid #dbe2ea; border-radius: 12px; padding: 16px; margin-bottom: 16px; }
.row { display: flex; gap: 12px; align-items: center; flex-wrap: wrap; }
.pill { background: #e7edf5; color: #1e293b; border-radius: 999px; padding: 4px 10px; font-size: 12px; }
button { border: 0; border-radius: 10px; padding: 8px 12px; background: #0f172a; color: white; cursor: pointer; }
button:hover { background: #1e293b; }
select { padding: 8px 10px; border-radius: 8px; border: 1px solid #cbd5e1; }
h1 { margin: 0 0 10px 0; }
p { margin: 0; }
"#;

#[cfg(target_arch = "wasm32")]
pub(crate) fn launch() {
    dioxus::launch(App);
}

#[cfg(target_arch = "wasm32")]
#[component]
fn App() -> Element {
    let mut mode = use_signal(|| "assist".to_string());
    let mut runs = use_signal(|| 0_u32);

    rsx! {
        main {
            style { "{APP_STYLE}" }

            div { class: "page",
                section { class: "card",
                    h1 { "Code Agent Sandbox" }
                    p { "UI shell ready. Backend wiring is intentionally deferred in stable." }
                }

                section { class: "card",
                    div { class: "row",
                        span { class: "pill", "Mode: {mode}" }
                        span { class: "pill", "Local runs: {runs}" }
                    }
                }

                section { class: "card",
                    div { class: "row",
                        label { "Workspace mode" }
                        select {
                            value: "{mode}",
                            onchange: move |evt| mode.set(evt.value().clone()),
                            option { value: "assist", "assist" }
                            option { value: "learn", "learn" }
                        }
                        button {
                            onclick: move |_| {
                                let next = *runs.read() + 1;
                                runs.set(next);
                            },
                            "Simulate Run"
                        }
                    }
                }
            }
        }
    }
}
