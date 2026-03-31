use crate::transport::response::Response;

#[test]
fn ok_response() {
    let r = Response::ok(1);
    let json = common_json::to_string(&r).unwrap();
    assert!(json.contains("Ok"));
}

#[test]
fn error_response() {
    let r = Response::error(1, "TEST", "test error");
    let json = common_json::to_string(&r).unwrap();
    assert!(json.contains("Error"));
}
