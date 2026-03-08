// projects/products/unstable/agent_engine/backend/src/engine/step_spec.rs
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, PartialEq, Eq)]
#[serde(tag = "kind")]
pub enum StepSpec {
    Log {
        message: String,
    },
    SetOutput {
        key: String,
        value: String,
    },
    CopyInput {
        input_key: String,
        output_key: String,
    },
}
