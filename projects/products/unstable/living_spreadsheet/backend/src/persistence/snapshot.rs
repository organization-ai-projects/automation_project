use crate::diagnostics::error::SpreadsheetError;
use crate::model::cell_id::CellId;
use crate::model::cell_value::CellValue;
use crate::model::sheet::Sheet;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SnapshotCell {
    pub id: CellId,
    pub formula: Option<String>,
    pub value: CellValue,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Snapshot {
    pub cells: Vec<SnapshotCell>,
}

impl Snapshot {
    pub fn from_sheet(sheet: &Sheet) -> Self {
        let mut cells: Vec<SnapshotCell> = sheet
            .cells()
            .map(|c| SnapshotCell {
                id: c.id.clone(),
                formula: c.formula.clone(),
                value: c.value.clone(),
            })
            .collect();
        cells.sort_by(|a, b| a.id.cmp(&b.id));
        Self { cells }
    }

    pub fn to_json(&self) -> Result<String, SpreadsheetError> {
        serde_json::to_string(self).map_err(|e| SpreadsheetError::SerializationError(e.to_string()))
    }

    pub fn from_json(s: &str) -> Result<Self, SpreadsheetError> {
        serde_json::from_str(s).map_err(|e| SpreadsheetError::SerializationError(e.to_string()))
    }
}
