use std::io::{BufRead, Write};
use serde::{Deserialize, Serialize};
use crate::diagnostics::UiError;

pub struct IpcClient {
    pub req_counter: u64,
}

impl IpcClient {
    pub fn new() -> Self {
        IpcClient { req_counter: 0 }
    }

    pub fn send_request<W: Write, R: BufRead, T: Serialize, U: for<'de> Deserialize<'de>>(
        &mut self,
        writer: &mut W,
        reader: &mut R,
        payload: &T,
    ) -> Result<U, UiError> {
        self.req_counter += 1;
        let id = self.req_counter;
        let request = IpcEnvelope { id, payload };
        let json = common_json::to_string(&request)
            .map_err(|e| UiError::Ipc(e.to_string()))?;
        writeln!(writer, "{}", json).map_err(|e| UiError::Ipc(e.to_string()))?;
        writer.flush().map_err(|e| UiError::Ipc(e.to_string()))?;

        let mut line = String::new();
        reader.read_line(&mut line).map_err(|e| UiError::Ipc(e.to_string()))?;
        let response: U = common_json::from_json_str(line.trim())
            .map_err(|e| UiError::Ipc(e.to_string()))?;
        Ok(response)
    }
}

#[derive(Serialize)]
struct IpcEnvelope<'a, T: Serialize> {
    id: u64,
    payload: &'a T,
}
