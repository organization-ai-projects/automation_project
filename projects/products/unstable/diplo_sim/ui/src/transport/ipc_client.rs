use super::request::Request;
use super::response::Response;

#[derive(Debug, Default)]
pub struct IpcClient;

impl IpcClient {
    pub fn new() -> Self {
        Self
    }

    pub fn send(&self, request: Request) -> Response {
        match request {
            Request::Health => Response::Ok,
            Request::RunMatch => Response::Ok,
            Request::ReplayMatch => Response::Error("replay not wired yet".to_string()),
        }
    }
}
