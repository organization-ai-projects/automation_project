use std::path::Path;
use crate::diagnostics::tactics_grid_error::TacticsGridError;
use crate::report::battle_report::BattleReport;

pub struct JsonCodec;

impl JsonCodec {
    pub fn save_report(report: &BattleReport, path: &Path) -> Result<(), TacticsGridError> {
        let json = common_json::to_json_string_pretty(report)?;
        std::fs::write(path, json)?;
        Ok(())
    }
}
