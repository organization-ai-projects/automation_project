use crate::transport::response::{Response, ResponsePayload};

#[test]
fn error_response_has_message() {
    let resp = Response::error(Some("req-1".to_string()), "something went wrong");
    match &resp.payload {
        ResponsePayload::Error { message } => {
            assert_eq!(message, "something went wrong");
        }
        _ => panic!("expected error payload"),
    }
}

#[test]
fn serialization_roundtrip() {
    let resp = Response::error(None, "test error");
    let json = common_json::to_json_string(&resp).unwrap();
    let restored: Response = common_json::from_str(&json).unwrap();
    assert_eq!(resp, restored);
}
