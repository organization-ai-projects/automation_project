use crate::diagnostics::FuzzHarnessError;
use crate::replay::ReplayFile;
use crate::report::FuzzReport;
use crate::shrinker::ShrinkReport;
use std::path::Path;

pub(crate) struct JsonCodec;

impl JsonCodec {
    pub(crate) fn to_json_pretty(report: &FuzzReport) -> Result<String, FuzzHarnessError> {
        common_json::to_json_string_pretty(report)
            .map_err(|e| FuzzHarnessError::Json(e.to_string()))
    }

    pub(crate) fn save_replay_file(file: &ReplayFile, path: &Path) -> Result<(), FuzzHarnessError> {
        let json = common_json::to_json_string_pretty(file)
            .map_err(|e| FuzzHarnessError::Json(e.to_string()))?;
        std::fs::write(path, json)?;
        Ok(())
    }

    pub(crate) fn load_replay_file(path: &Path) -> Result<ReplayFile, FuzzHarnessError> {
        let data = std::fs::read_to_string(path)?;
        common_json::from_str(&data).map_err(|e| FuzzHarnessError::Json(e.to_string()))
    }

    pub(crate) fn save_shrink_report(
        report: &ShrinkReport,
        path: &Path,
    ) -> Result<(), FuzzHarnessError> {
        let json = common_json::to_json_string_pretty(report)
            .map_err(|e| FuzzHarnessError::Json(e.to_string()))?;
        std::fs::write(path, json)?;
        Ok(())
    }
}
