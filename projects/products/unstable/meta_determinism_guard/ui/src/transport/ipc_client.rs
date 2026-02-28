use anyhow::Result;

/// Placeholder for future socket-based IPC client.
pub struct IpcClient {
    pub address: String,
}

impl IpcClient {
    pub fn new(address: impl Into<String>) -> Self {
        Self { address: address.into() }
    }

    pub fn send(&self, _message: &str) -> Result<String> {
        anyhow::bail!("IPC client not yet implemented")
    }
}
