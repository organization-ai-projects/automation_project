use crate::transport::request::Request;
use std::path::PathBuf;

pub struct ValidateScreen;

impl ValidateScreen {
    pub fn request(scenario: PathBuf) -> Request {
        Request::Validate { scenario }
    }
}
