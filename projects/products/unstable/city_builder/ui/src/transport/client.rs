use crate::diagnostics::ui_error::UiError;
use crate::transport::backend_process::BackendProcess;
use crate::transport::request::Request;
use crate::transport::response::Response;

pub struct Client {
    process: BackendProcess,
}

impl Client {
    pub fn new(process: BackendProcess) -> Self {
        Self { process }
    }

    pub fn backend_bin(&self) -> &str {
        self.process.backend_bin()
    }

    pub fn send(&self, request: Request) -> Result<Response, UiError> {
        let code = self.process.run(&request.as_args())?;
        Ok(Response { exit_code: code })
    }
}
