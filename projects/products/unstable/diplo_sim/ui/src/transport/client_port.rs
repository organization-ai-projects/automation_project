use super::request::Request;
use super::response::Response;

pub trait ClientPort: Send + Sync {
    fn send(&self, request: Request) -> Response;
}
