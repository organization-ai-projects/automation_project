#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CampaignScreen {
    pub seed: u64,
    pub days: u32,
    pub latest_status: String,
}

impl CampaignScreen {
    pub fn new(seed: u64, days: u32, latest_status: String) -> Self {
        Self {
            seed,
            days,
            latest_status,
        }
    }

    pub fn summary_line(&self) -> String {
        format!(
            "seed={} days={} status={}",
            self.seed, self.days, self.latest_status
        )
    }
}
