#[allow(dead_code)]
pub struct IpcClient;

#[allow(dead_code)]
impl IpcClient {
    pub fn new() -> Self {
        Self
    }

    pub fn send_request(&self, request: &str) -> String {
        // Placeholder: in a real implementation this would write to the backend process stdin
        // and read from stdout.
        format!("{{\"type\":\"Ok\"}}")
    }
}
