// projects/products/unstable/autonomy_orchestrator_ai/src/pending_validation_invocation.rs
#[derive(Clone, Debug)]
pub struct PendingValidationInvocation {
    pub command: String,
    pub args: Vec<String>,
    pub env: Vec<(String, String)>,
}
