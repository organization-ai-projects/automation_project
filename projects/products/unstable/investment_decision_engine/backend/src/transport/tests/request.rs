use crate::transport::request::{Request, RequestPayload};

#[test]
fn serialize_analyze_asset_request() {
    let req = Request {
        id: Some("req-1".to_string()),
        payload: RequestPayload::AnalyzeAsset {
            asset_json: "{}".to_string(),
            market_json: "{}".to_string(),
        },
    };
    let json = common_json::to_json_string(&req).unwrap();
    let restored: Request = common_json::from_str(&json).unwrap();
    assert_eq!(req, restored);
}

#[test]
fn serialize_shutdown_request() {
    let req = Request {
        id: None,
        payload: RequestPayload::Shutdown,
    };
    let json = common_json::to_json_string(&req).unwrap();
    assert!(json.contains("Shutdown"));
}
