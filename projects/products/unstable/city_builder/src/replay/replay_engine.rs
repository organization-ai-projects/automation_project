use crate::diagnostics::error::CityBuilderError;
use crate::report::sim_report::SimReport;

#[derive(Debug, Clone)]
pub struct ReplayEngine;

impl ReplayEngine {
    pub fn new() -> Self {
        Self
    }

    pub fn verify_reports(r1: &SimReport, r2: &SimReport) -> Result<(), CityBuilderError> {
        if r1.run_hash != r2.run_hash {
            return Err(CityBuilderError::ReplayMismatch(format!(
                "Run hashes differ: {} vs {}",
                r1.run_hash, r2.run_hash
            )));
        }
        for (t1, t2) in r1.tick_reports.iter().zip(r2.tick_reports.iter()) {
            if t1.snapshot_hash != t2.snapshot_hash {
                return Err(CityBuilderError::ReplayMismatch(format!(
                    "Tick {} snapshot hash mismatch: {} vs {}",
                    t1.tick, t1.snapshot_hash, t2.snapshot_hash
                )));
            }
        }
        Ok(())
    }
}

impl Default for ReplayEngine {
    fn default() -> Self {
        Self::new()
    }
}
