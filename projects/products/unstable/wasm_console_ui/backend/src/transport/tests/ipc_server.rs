use crate::transport::ipc_server::IpcServer;
use crate::transport::request::Request;
use crate::transport::response::Response;

#[test]
fn server_starts_with_empty_state() {
    let server = IpcServer::new();
    assert!(server.state().panels.is_empty());
    assert!(server.state().active_panel.is_none());
}

#[test]
fn server_has_builtin_plugins() {
    let server = IpcServer::new();
    assert_eq!(server.registry().len(), 3);
}

#[test]
fn server_load_log_file() {
    let mut server = IpcServer::new();
    let response = server.handle(&Request::LoadLogFile {
        path: "test.json".to_string(),
    });
    assert!(matches!(response, Response::OperationSuccess { .. }));
    assert!(server.state().active_panel.is_some());
}

#[test]
fn server_export_then_import_snapshot() {
    let mut server = IpcServer::new();
    server.handle(&Request::LoadLogFile {
        path: "log.json".to_string(),
    });
    let export_response = server.handle(&Request::ExportSnapshot);
    let snapshot_json = match export_response {
        Response::StateSnapshot { json } => json,
        other => panic!("Expected StateSnapshot, got: {other:?}"),
    };

    // Create a new server and import the snapshot
    let mut server2 = IpcServer::new();
    let import_response = server2.handle(&Request::ImportSnapshot {
        data: snapshot_json,
    });
    assert!(matches!(import_response, Response::OperationSuccess { .. }));
    assert_eq!(server.state(), server2.state());
}

#[test]
fn server_import_invalid_snapshot() {
    let mut server = IpcServer::new();
    let response = server.handle(&Request::ImportSnapshot {
        data: "not valid json".to_string(),
    });
    assert!(matches!(response, Response::Error { .. }));
}
