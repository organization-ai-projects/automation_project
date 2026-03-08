#[cfg(not(target_arch = "wasm32"))]
use crate::transport::ipc_client::IpcClient;
#[cfg(not(target_arch = "wasm32"))]
use crate::transport::request::Request;
#[cfg(not(target_arch = "wasm32"))]
use crate::transport::response::Response;

#[cfg(not(target_arch = "wasm32"))]
#[test]
fn ipc_client_handles_replay_as_error() {
    let client = IpcClient::new();
    let response = client.send(Request::ReplayMatch);
    assert_eq!(
        response,
        Response::Error("replay not wired yet".to_string())
    );
}

#[cfg(target_arch = "wasm32")]
#[test]
fn ipc_client_request_types_are_available_on_wasm() {
    let request = crate::transport::request::Request::Health;
    assert!(matches!(
        request,
        crate::transport::request::Request::Health
    ));
}
