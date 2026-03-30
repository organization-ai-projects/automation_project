use crate::transport::ipc_client::{ClientRequest, ClientResponse, IpcClient};

#[test]
fn serialize_request_roundtrip() {
    let request = ClientRequest::LoadLogFile {
        path: "test.json".to_string(),
    };
    let json = IpcClient::serialize_request(&request).unwrap();
    let deserialized: ClientRequest = common_json::from_str(&json).unwrap();
    assert_eq!(request, deserialized);
}

#[test]
fn deserialize_response_success() {
    let response = ClientResponse::OperationSuccess {
        message: "ok".to_string(),
    };
    let json = common_json::to_string(&response).unwrap();
    let deserialized = IpcClient::deserialize_response(&json).unwrap();
    assert_eq!(response, deserialized);
}

#[test]
fn deserialize_response_error_on_bad_json() {
    let result = IpcClient::deserialize_response("not json");
    assert!(result.is_err());
}

#[test]
fn client_last_response_initially_none() {
    let client = IpcClient::new();
    assert!(client.last_response().is_none());
}

#[test]
fn client_stores_last_response() {
    let mut client = IpcClient::new();
    let response = ClientResponse::OperationSuccess {
        message: "done".to_string(),
    };
    client.set_last_response(response.clone());
    assert_eq!(client.last_response(), Some(&response));
}

#[test]
fn export_snapshot_request_serializable() {
    let request = ClientRequest::ExportSnapshot;
    let json = IpcClient::serialize_request(&request).unwrap();
    let deserialized: ClientRequest = common_json::from_str(&json).unwrap();
    assert_eq!(request, deserialized);
}
