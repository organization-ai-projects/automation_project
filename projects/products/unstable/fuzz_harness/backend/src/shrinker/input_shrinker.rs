use crate::diagnostics::FuzzHarnessError;
use crate::model::{FuzzInput, FuzzResult, FuzzTarget};
use crate::replay::ReplayFile;
use crate::shrinker::ShrinkReport;

pub(crate) struct InputShrinker;

impl InputShrinker {
    pub(crate) fn shrink(
        target: &dyn FuzzTarget,
        replay: &ReplayFile,
    ) -> Result<ShrinkReport, FuzzHarnessError> {
        let original_size = replay.input.data.len();
        let mut current = replay.input.data.clone();
        let mut steps: u64 = 0;
        let mut last_failure_msg = replay.failure_message.clone();

        // Phase 1: Try removing bytes from end to start
        let mut i = current.len();
        while i > 0 {
            i -= 1;
            if current.len() <= 1 {
                break;
            }
            let mut candidate = current.clone();
            candidate.remove(i);
            let input = FuzzInput {
                data: candidate.clone(),
                index: replay.input.index,
            };
            steps += 1;
            if let FuzzResult::Fail(msg) = target.execute(&input) {
                current = candidate;
                last_failure_msg = msg;
                if i > current.len() {
                    i = current.len();
                }
            }
        }

        // Phase 2: Try zeroing each byte
        for i in 0..current.len() {
            if current[i] == 0 {
                continue;
            }
            let mut candidate = current.clone();
            candidate[i] = 0;
            let input = FuzzInput {
                data: candidate.clone(),
                index: replay.input.index,
            };
            steps += 1;
            if let FuzzResult::Fail(msg) = target.execute(&input) {
                current = candidate;
                last_failure_msg = msg;
            }
        }

        // Phase 3: Try reducing each byte value via binary search
        for i in 0..current.len() {
            if current[i] == 0 {
                continue;
            }
            let mut low: u8 = 0;
            let mut high: u8 = current[i];
            while low < high {
                let mid = low + (high - low) / 2;
                if mid == current[i] {
                    break;
                }
                let mut candidate = current.clone();
                candidate[i] = mid;
                let input = FuzzInput {
                    data: candidate.clone(),
                    index: replay.input.index,
                };
                steps += 1;
                if let FuzzResult::Fail(msg) = target.execute(&input) {
                    current = candidate;
                    last_failure_msg = msg;
                    high = mid;
                } else {
                    low = mid + 1;
                }
            }
        }

        Ok(ShrinkReport {
            target_name: replay.target_name.clone(),
            original_size,
            shrunk_input: FuzzInput {
                data: current,
                index: replay.input.index,
            },
            shrink_steps: steps,
            failure_message: last_failure_msg,
        })
    }
}
