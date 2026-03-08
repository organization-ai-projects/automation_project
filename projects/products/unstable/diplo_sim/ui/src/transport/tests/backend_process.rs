#[cfg(not(target_arch = "wasm32"))]
use crate::transport::backend_process::BackendProcess;
#[cfg(not(target_arch = "wasm32"))]
use crate::transport::client_port::ClientPort;
#[cfg(not(target_arch = "wasm32"))]
use crate::transport::request::Request;
#[cfg(not(target_arch = "wasm32"))]
use crate::transport::response::Response;

#[cfg(not(target_arch = "wasm32"))]
struct StubClient;

#[cfg(not(target_arch = "wasm32"))]
impl ClientPort for StubClient {
    fn send(&self, request: Request) -> Response {
        match request {
            Request::Health => Response::Ok,
            _ => Response::Error("stub only supports health".to_string()),
        }
    }
}

#[cfg(not(target_arch = "wasm32"))]
#[test]
fn backend_process_accepts_injected_client() {
    let backend = BackendProcess::with_client(Box::new(StubClient));
    let health_response = backend.client().send(Request::Health);
    assert_eq!(health_response, Response::Ok);
    let maps_response = backend.client().send(Request::ListMaps);
    assert_eq!(
        maps_response,
        Response::Error("stub only supports health".to_string())
    );
}

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
    let map_info_response = backend.client().send(Request::GetMapInfo {
        map_id: "tiny_triangle".to_string(),
    });
    assert_eq!(
        map_info_response,
        Response::MapInfo {
            map_id: "tiny_triangle".to_string(),
            territory_count: 3,
            adjacency_count: 3,
            starting_unit_count: 2
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
