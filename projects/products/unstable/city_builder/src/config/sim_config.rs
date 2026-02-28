#[derive(Debug, Clone)]
pub struct SimConfig {
    pub grid_width: u32,
    pub grid_height: u32,
    pub seed: u64,
    pub total_ticks: u64,
}

impl SimConfig {
    pub fn next_rand(seed: &mut u64) -> u64 {
        *seed = seed.wrapping_mul(6364136223846793005).wrapping_add(1442695040888963407);
        *seed
    }
}
