use crate::config::EngineConfig;
use crate::transport::ipc_server::IpcServer;
use crate::transport::request::{Request, RequestPayload};
use crate::transport::response::ResponsePayload;

#[test]
fn handle_shutdown_request() {
    let server = IpcServer::new(EngineConfig::default());
    let req = Request {
        id: Some("req-1".to_string()),
        payload: RequestPayload::Shutdown,
    };
    let resp = server.handle_request(&req);
    assert_eq!(resp.id, Some("req-1".to_string()));
}

#[test]
fn handle_invalid_asset_json() {
    let server = IpcServer::new(EngineConfig::default());
    let req = Request {
        id: Some("req-2".to_string()),
        payload: RequestPayload::AnalyzeAsset {
            asset_json: "not valid".to_string(),
            market_json: "{}".to_string(),
        },
    };
    let resp = server.handle_request(&req);
    match resp.payload {
        ResponsePayload::Error { message } => {
            assert!(message.contains("invalid asset"));
        }
        _ => panic!("expected error response"),
    }
}
