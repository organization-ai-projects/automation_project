use serde::{Deserialize, Serialize};
use crate::protocol::{Request, Response};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RequestMessage {
    pub id: u64,
    pub request: Request,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResponseMessage {
    pub id: u64,
    pub response: Response,
}
