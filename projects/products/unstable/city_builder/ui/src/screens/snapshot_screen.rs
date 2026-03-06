use crate::transport::request::Request;
use std::path::PathBuf;

pub struct SnapshotScreen;

impl SnapshotScreen {
    pub fn request(replay: PathBuf, at_tick: u64, out: PathBuf) -> Request {
        Request::Snapshot {
            replay,
            at_tick,
            out,
        }
    }
}
