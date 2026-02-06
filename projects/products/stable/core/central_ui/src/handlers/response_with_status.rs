// projects/products/stable/core/central_ui/src/handlers/response_with_status.rs
use warp::{http, reply::Response};

pub(crate) fn response_with_status(body: bytes::Bytes, status: http::StatusCode) -> Response {
    let mut resp = Response::new(body.into());
    *resp.status_mut() = status;
    resp
}
