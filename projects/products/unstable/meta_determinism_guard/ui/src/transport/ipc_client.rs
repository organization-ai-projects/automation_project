// projects/products/unstable/meta_determinism_guard/ui/src/transport/ipc_client.rs
use crate::diagnostics::ui_error::UiError;
use std::io::{BufRead, BufReader, Write};
use std::net::TcpStream;

pub struct IpcClient {
    pub address: String,
}

impl IpcClient {
    pub fn new(address: impl Into<String>) -> Self {
        Self {
            address: address.into(),
        }
    }

    pub fn send(&self, message: &str) -> Result<String, UiError> {
        let mut stream = TcpStream::connect(&self.address)
            .map_err(|err| UiError::Transport(format!("connect {}: {}", self.address, err)))?;
        stream
            .write_all(format!("{message}\n").as_bytes())
            .map_err(|err| UiError::Transport(format!("write {}: {}", self.address, err)))?;
        stream
            .flush()
            .map_err(|err| UiError::Transport(format!("flush {}: {}", self.address, err)))?;

        let mut reader = BufReader::new(stream);
        let mut line = String::new();
        reader
            .read_line(&mut line)
            .map_err(|err| UiError::Transport(format!("read {}: {}", self.address, err)))?;
        Ok(line.trim().to_string())
    }
}
