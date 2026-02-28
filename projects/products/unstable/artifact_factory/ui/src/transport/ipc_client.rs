use crate::diagnostics::error::UiError;
use crate::transport::backend_process::BackendProcess;

/// Thin IPC client that serializes requests to JSON and deserializes responses.
/// Uses a simple sequential message ID counter.
pub struct IpcClient {
    process: BackendProcess,
    next_id: u64,
}

impl IpcClient {
    pub fn new(process: BackendProcess) -> Self {
        Self {
            process,
            next_id: 1,
        }
    }

    fn next_id(&mut self) -> u64 {
        let id = self.next_id;
        self.next_id += 1;
        id
    }

    fn send_recv(&mut self, request_json: &str) -> Result<String, UiError> {
        self.process.send_line(request_json)?;
        self.process.recv_line()
    }

    pub fn send_load_inputs(&mut self, paths: Vec<String>) -> Result<(), UiError> {
        let id = self.next_id();
        let paths_json: Vec<String> = paths.iter().map(|p| format!("\"{}\"", p)).collect();
        let req = format!(
            r#"{{"id":{},"request":{{"type":"load_inputs","paths":[{}]}}}}"#,
            id,
            paths_json.join(",")
        );
        self.send_recv(&req)?;
        Ok(())
    }

    pub fn send_analyze(&mut self) -> Result<(), UiError> {
        let id = self.next_id();
        let req = format!(r#"{{"id":{},"request":{{"type":"analyze"}}}}"#, id);
        self.send_recv(&req)?;
        Ok(())
    }

    pub fn send_render_docs(&mut self) -> Result<(), UiError> {
        let id = self.next_id();
        let req = format!(r#"{{"id":{},"request":{{"type":"render_docs"}}}}"#, id);
        self.send_recv(&req)?;
        Ok(())
    }

    pub fn send_build_bundle(&mut self) -> Result<(), UiError> {
        let id = self.next_id();
        let req = format!(r#"{{"id":{},"request":{{"type":"build_bundle"}}}}"#, id);
        self.send_recv(&req)?;
        Ok(())
    }

    pub fn send_get_bundle(&mut self) -> Result<Option<(String, Vec<String>)>, UiError> {
        let id = self.next_id();
        let req = format!(r#"{{"id":{},"request":{{"type":"get_bundle"}}}}"#, id);
        let raw = self.send_recv(&req)?;
        // Simple parse: check for "hash" field
        if raw.contains("\"hash\"") {
            // Extract hash and manifest from response JSON
            let hash = extract_string_field(&raw, "hash");
            let manifest = extract_string_array_field(&raw, "manifest");
            Ok(Some((hash, manifest)))
        } else {
            Ok(None)
        }
    }
}

fn extract_string_field(json: &str, field: &str) -> String {
    let key = format!("\"{}\":", field);
    if let Some(start) = json.find(&key) {
        let rest = &json[start + key.len()..];
        let rest = rest.trim_start();
        if let Some(rest) = rest.strip_prefix('"') {
            if let Some(end) = rest.find('"') {
                return rest[..end].to_string();
            }
        }
    }
    String::new()
}

fn extract_string_array_field(json: &str, field: &str) -> Vec<String> {
    let key = format!("\"{}\":", field);
    if let Some(start) = json.find(&key) {
        let rest = &json[start + key.len()..];
        let rest = rest.trim_start();
        if let Some(rest) = rest.strip_prefix('[') {
            if let Some(end) = rest.find(']') {
                let inner = &rest[..end];
                return inner
                    .split(',')
                    .filter_map(|s| {
                        let s = s.trim().trim_matches('"');
                        if s.is_empty() {
                            None
                        } else {
                            Some(s.to_string())
                        }
                    })
                    .collect();
            }
        }
    }
    Vec::new()
}
