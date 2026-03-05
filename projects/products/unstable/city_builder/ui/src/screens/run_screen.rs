use crate::transport::request::Request;
use std::path::PathBuf;

pub struct RunScreen;

impl RunScreen {
    pub fn request(
        ticks: u64,
        seed: u64,
        scenario: PathBuf,
        out: PathBuf,
        replay_out: Option<PathBuf>,
    ) -> Request {
        Request::Run {
            ticks,
            seed,
            scenario,
            out,
            replay_out,
        }
    }
}
