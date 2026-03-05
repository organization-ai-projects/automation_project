use crate::transport::request::Request;
use std::path::PathBuf;

pub struct ReplayScreen;

impl ReplayScreen {
    pub fn request(replay: PathBuf, out: PathBuf) -> Request {
        Request::Replay { replay, out }
    }
}
