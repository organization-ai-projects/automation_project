#[cfg(not(target_arch = "wasm32"))]
use crate::transport::backend_process::BackendProcess;
#[cfg(not(target_arch = "wasm32"))]
use crate::transport::request::Request;
#[cfg(not(target_arch = "wasm32"))]
use crate::transport::response::Response;
#[cfg(not(target_arch = "wasm32"))]
use crate::ui_app::product_name;

#[cfg(not(target_arch = "wasm32"))]
#[test]
fn app_transport_health_roundtrip_is_ok() {
    let backend = BackendProcess::new();
    let response = backend.client().send(Request::Health);
    assert_eq!(response, Response::Ok);
    assert_eq!(product_name(), "Diplo Sim");
}
