#[cfg(not(target_arch = "wasm32"))]
use crate::transport::backend_process::BackendProcess;
#[cfg(not(target_arch = "wasm32"))]
use crate::transport::request::Request;
#[cfg(not(target_arch = "wasm32"))]
use crate::transport::response::Response;

#[cfg(not(target_arch = "wasm32"))]
#[test]
fn backend_process_client_is_usable() {
    let backend = BackendProcess::new();
    let response = backend.client().send(Request::RunMatch);
    assert_eq!(response, Response::Ok);
}

#[cfg(target_arch = "wasm32")]
#[test]
fn backend_process_types_are_available_on_wasm() {
    let response = crate::transport::response::Response::Ok;
    assert_eq!(response, crate::transport::response::Response::Ok);
}
