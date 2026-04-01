use crate::diagnostics::FuzzHarnessError;
use crate::generator::InputGenerator;
use crate::model::{FuzzResult, FuzzTarget};
use crate::report::{FailureRecord, FuzzReport, RunHash};

pub(crate) struct FuzzRunner;

impl FuzzRunner {
    pub(crate) fn run(
        target: &dyn FuzzTarget,
        seed: u64,
        iterations: u64,
    ) -> Result<FuzzReport, FuzzHarnessError> {
        let mut generator = InputGenerator::new(seed);
        let mut failures: Vec<FailureRecord> = Vec::new();

        for _ in 0..iterations {
            let input = generator.next();
            let result = target.execute(&input);
            if let FuzzResult::Fail(msg) = result {
                failures.push(FailureRecord {
                    input,
                    message: msg,
                });
            }
        }

        let mut report = FuzzReport {
            target_name: target.name().to_string(),
            seed,
            iterations_run: iterations,
            failure_count: failures.len() as u64,
            failures,
            run_hash: RunHash(String::new()),
        };
        report.compute_hash();
        Ok(report)
    }
}
