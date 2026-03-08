//! projects/products/unstable/agent_engine/ui/src/app.rs
use dioxus::prelude::*;

const SAMPLE_TASK_JSON: &str = r#"{
  "id": "task-v0-001",
  "label": "minimal_v0_task",
  "metadata": {
    "source": "ui"
  },
  "input": {
    "prompt": "hello agent"
  },
  "steps": [
    { "kind": "Log", "message": "starting task" },
    { "kind": "CopyInput", "input_key": "prompt", "output_key": "echo" },
    { "kind": "SetOutput", "key": "status", "value": "ok" }
  ]
}"#;

pub fn app() -> Element {
    rsx! {
        main { class: "agent-engine-v0",
            h1 { "Agent Engine V0" }
            p { "Deterministic task runner foundation is ready." }
            section {
                h2 { "Run Backend CLI" }
                pre { "cargo run -p agent_engine_backend -- run projects/products/unstable/agent_engine/backend/tests/fixtures/task_minimal.json" }
            }
            section {
                h2 { "Sample Task JSON" }
                pre { "{sample_task_json()}" }
            }
        }
    }
}

pub fn sample_task_json() -> &'static str {
    SAMPLE_TASK_JSON
}
