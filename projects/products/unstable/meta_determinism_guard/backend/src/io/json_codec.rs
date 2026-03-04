use crate::protocol::response::Response;
use anyhow::Result;

pub fn encode_response(response: &Response) -> Result<String> {
    Ok(common_json::to_string(response)?)
}

pub fn parse_request(line: &str) -> Result<crate::protocol::request::Request> {
    Ok(common_json::from_json_str(line)?)
}
