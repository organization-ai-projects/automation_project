// projects/products/unstable/universal_model_engine/backend/src/determinism/seed.rs
#[derive(Debug, Clone, Copy, Default)]
pub struct Seed {
    pub value: u64,
}

impl Seed {
    pub fn new(value: u64) -> Self {
        Self { value }
    }
}
