// projects/libraries/core/foundation/common_time/src/fake_clock_state.rs
#[derive(Debug)]
pub(crate) struct FakeClockState {
    pub(crate) now: std::time::Instant,
}

impl FakeClockState {
    pub(crate) fn new() -> Self {
        Self {
            now: std::time::Instant::now(),
        }
    }
}
