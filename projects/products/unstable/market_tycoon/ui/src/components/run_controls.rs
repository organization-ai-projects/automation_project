//! projects/products/unstable/market_tycoon/ui/src/components/run_controls.rs
pub(crate) struct RunControls {
    seed: u64,
    ticks: u64,
}

impl RunControls {
    pub fn new(seed: u64, ticks: u64) -> Self {
        Self { seed, ticks }
    }

    pub fn seed(&self) -> u64 {
        self.seed
    }

    pub fn ticks(&self) -> u64 {
        self.ticks
    }
}
