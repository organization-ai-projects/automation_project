#[derive(Debug, Clone)]
pub struct RunResult {
    pub run_index: u32,
    pub stdout: String,
    pub hash: String,
}

#[derive(Debug, Clone)]
pub struct RunMatrix {
    pub results: Vec<RunResult>,
}

impl RunMatrix {
    pub fn new(results: Vec<RunResult>) -> Self {
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
