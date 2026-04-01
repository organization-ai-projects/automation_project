use crate::diagnostics::FuzzHarnessError;
use crate::model::{FuzzResult, FuzzTarget};
use crate::replay::ReplayFile;

pub(crate) struct ReplayEngine;

impl ReplayEngine {
    pub(crate) fn replay(
        target: &dyn FuzzTarget,
        file: &ReplayFile,
    ) -> Result<FuzzResult, FuzzHarnessError> {
        let result = target.execute(&file.input);
        match &result {
            FuzzResult::Fail(_) => Ok(result),
            FuzzResult::Pass => Err(FuzzHarnessError::ReplayMismatch(
                "expected failure but target passed on replay".to_string(),
            )),
        }
    }
}
