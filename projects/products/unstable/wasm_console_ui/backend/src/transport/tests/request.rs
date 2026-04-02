use crate::transport::request::Request;

#[test]
fn request_load_log_file_serializable() {
    let request = Request::LoadLogFile {
        path: "test.json".to_string(),
    };
    let json = common_json::to_string(&request).unwrap();
    let deserialized: Request = common_json::from_str(&json).unwrap();
    assert_eq!(request, deserialized);
}

#[test]
fn request_export_snapshot_serializable() {
    let request = Request::ExportSnapshot;
    let json = common_json::to_string(&request).unwrap();
    let deserialized: Request = common_json::from_str(&json).unwrap();
    assert_eq!(request, deserialized);
}
