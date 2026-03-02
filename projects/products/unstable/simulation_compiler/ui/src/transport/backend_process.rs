// projects/products/unstable/simulation_compiler/ui/src/transport/backend_process.rs

/// Represents a handle to the backend subprocess.
/// In this implementation the backend is NOT spawned; messages are
/// prepared here and would be sent over stdio IPC.
pub struct BackendProcess {
    pub binary_path: String,
}

impl BackendProcess {
    pub fn new(binary_path: impl Into<String>) -> Self {
        Self {
            binary_path: binary_path.into(),
        }
    }
}
