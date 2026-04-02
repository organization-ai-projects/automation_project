use crate::transport::response::Response;

#[test]
fn response_state_snapshot_serializable() {
    let response = Response::StateSnapshot {
        json: "{}".to_string(),
    };
    let json = common_json::to_string(&response).unwrap();
    let deserialized: Response = common_json::from_str(&json).unwrap();
    assert_eq!(response, deserialized);
}

#[test]
fn response_error_serializable() {
    let response = Response::Error {
        message: "something went wrong".to_string(),
    };
    let json = common_json::to_string(&response).unwrap();
    let deserialized: Response = common_json::from_str(&json).unwrap();
    assert_eq!(response, deserialized);
}

#[test]
fn response_operation_success_serializable() {
    let response = Response::OperationSuccess {
        message: "done".to_string(),
    };
    let json = common_json::to_string(&response).unwrap();
    let deserialized: Response = common_json::from_str(&json).unwrap();
    assert_eq!(response, deserialized);
}
