// projects/products/unstable/protocol_builder/ui/src/transport/payload.rs
use serde::{Deserialize, Serialize};

use super::request::Request;
use super::response::Response;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "direction", content = "data")]
pub enum Payload {
    Request(Request),
    Response(Response),
}
