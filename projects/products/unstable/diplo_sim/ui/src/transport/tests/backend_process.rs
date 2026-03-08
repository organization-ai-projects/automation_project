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
    let maps_response = backend.client().send(Request::ListMaps);
    assert_eq!(
        maps_response,
        Response::Maps {
            map_ids: vec!["tiny_triangle".to_string()]
        }
    );
    let run_response = backend.client().send(Request::RunMatch {
        map_id: "tiny_triangle".to_string(),
        turns: 5,
        seed: 42,
        players: 2,
    });
    let run_id = match run_response {
        Response::MatchRun { run_id } => run_id,
        other => panic!("expected MatchRun response, got {other:?}"),
    };
    let replay_response = backend.client().send(Request::ReplayMatch { run_id });
    assert_eq!(replay_response, Response::ReplayReady { run_id });
    let status_response = backend.client().send(Request::GetRunStatus { run_id });
    assert_eq!(
        status_response,
        Response::RunStatus {
            run_id,
            replayed: true
        }
    );
}

#[cfg(target_arch = "wasm32")]
#[test]
fn backend_process_types_are_available_on_wasm() {
    let response = crate::transport::response::Response::Ok;
    assert_eq!(response, crate::transport::response::Response::Ok);
}
