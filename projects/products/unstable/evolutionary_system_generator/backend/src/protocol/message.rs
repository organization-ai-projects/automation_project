use crate::protocol::request::Request;
use crate::protocol::response::Response;

pub fn write_response(resp: &Response) -> String {
    serde_json::to_string(resp).unwrap()
}

pub fn read_request(line: &str) -> Result<Request, String> {
    serde_json::from_str(line).map_err(|e| e.to_string())
}
