use super::client_port::ClientPort;
use super::ipc_client::IpcClient;

pub struct BackendProcess {
    client: Box<dyn ClientPort>,
}

impl BackendProcess {
    pub fn new() -> Self {
        Self::with_client(Box::new(IpcClient::new()))
    }

    pub fn with_client(client: Box<dyn ClientPort>) -> Self {
        Self { client }
    }

    pub fn client(&self) -> &dyn ClientPort {
        self.client.as_ref()
    }
}

impl Default for BackendProcess {
    fn default() -> Self {
        Self::new()
    }
}
