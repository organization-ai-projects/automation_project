use crate::report::{FailureRecord, RunHash};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) struct FuzzReport {
    pub(crate) target_name: String,
    pub(crate) seed: u64,
    pub(crate) iterations_run: u64,
    pub(crate) failure_count: u64,
    pub(crate) failures: Vec<FailureRecord>,
    pub(crate) run_hash: RunHash,
}

impl FuzzReport {
    pub(crate) fn compute_hash(&mut self) {
        let canonical = format!(
            "target:{},seed:{},iterations:{},failure_count:{},failures:[{}]",
            self.target_name,
            self.seed,
            self.iterations_run,
            self.failure_count,
            self.failures
                .iter()
                .map(|f| format!(
                    "{{idx:{},len:{},msg:{}}}",
                    f.input.index,
                    f.input.data.len(),
                    f.message
                ))
                .collect::<Vec<_>>()
                .join(",")
        );
        self.run_hash = RunHash::compute(canonical.as_bytes());
    }
}
