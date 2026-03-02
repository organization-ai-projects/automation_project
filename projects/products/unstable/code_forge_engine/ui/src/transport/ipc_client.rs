// projects/products/unstable/code_forge_engine/ui/src/transport/ipc_client.rs
use crate::transport::backend_process::BackendProcess;

pub struct IpcClient {
    pub process: BackendProcess,
}

impl IpcClient {
    pub fn new(process: BackendProcess) -> Self {
        Self { process }
    }
}
