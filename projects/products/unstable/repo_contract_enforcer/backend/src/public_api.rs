use crate::protocol::request::Request;
use crate::protocol::response::Response;

#[derive(Default)]
pub struct BackendState;

impl BackendState {
    pub fn new() -> Self {
        Self
    }

    pub fn handle(&mut self, _request: Request) -> Response {
        Response
    }
}
