use crate::stability::stability_run_result::StabilityRunResult;

#[derive(Debug, Clone)]
pub struct StabilityRunMatrix {
    pub results: Vec<StabilityRunResult>,
}

impl StabilityRunMatrix {
    pub fn new(results: Vec<StabilityRunResult>) -> Self {
        Self { results }
    }

    pub fn all_hashes_equal(&self) -> bool {
        if self.results.is_empty() {
            return true;
        }
        let first = &self.results[0].hash;
        self.results.iter().all(|r| &r.hash == first)
    }

    pub fn sorted_hashes(&self) -> Vec<String> {
        let mut hashes: Vec<String> = self.results.iter().map(|r| r.hash.clone()).collect();
        hashes.sort();
        hashes
    }
}
