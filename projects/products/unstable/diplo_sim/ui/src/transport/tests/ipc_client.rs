#[cfg(not(target_arch = "wasm32"))]
use crate::transport::ipc_client::IpcClient;
#[cfg(not(target_arch = "wasm32"))]
use crate::transport::request::Request;
#[cfg(not(target_arch = "wasm32"))]
use crate::transport::response::Response;

#[cfg(not(target_arch = "wasm32"))]
#[test]
fn ipc_client_requires_run_before_replay() {
    let client = IpcClient::new();
    let response = client.send(Request::ReplayMatch { run_id: 1 });
    assert_eq!(
        response,
        Response::Error("replay requires a prior run; no run has been executed yet".to_string())
    );
}

#[cfg(not(target_arch = "wasm32"))]
#[test]
fn ipc_client_returns_run_id_and_replays_matching_run() {
    let client = IpcClient::new();
    let run_response = client.send(Request::RunMatch {
        map_id: "tiny_triangle".to_string(),
        turns: 4,
        seed: 9,
        players: 2,
    });
    let run_id = match run_response {
        Response::MatchRun { run_id } => run_id,
        other => panic!("expected MatchRun response, got {other:?}"),
    };

    let replay_response = client.send(Request::ReplayMatch { run_id });
    assert_eq!(replay_response, Response::ReplayReady { run_id });
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
