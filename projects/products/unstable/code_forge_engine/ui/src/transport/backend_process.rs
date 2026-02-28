// projects/products/unstable/code_forge_engine/ui/src/transport/backend_process.rs
pub struct BackendProcess {
    pub bin: String,
}

impl BackendProcess {
    pub fn new(bin: impl Into<String>) -> Self {
        Self { bin: bin.into() }
    }
}
