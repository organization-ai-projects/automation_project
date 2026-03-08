use super::ipc_client::IpcClient;

#[derive(Debug, Default)]
pub struct BackendProcess {
    client: IpcClient,
}

impl BackendProcess {
    pub fn new() -> Self {
        Self {
            client: IpcClient::new(),
        }
    }

    pub fn client(&self) -> &IpcClient {
        &self.client
    }
}
