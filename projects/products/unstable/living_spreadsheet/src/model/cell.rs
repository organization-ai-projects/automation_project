use crate::model::cell_id::CellId;
use crate::model::cell_value::CellValue;

pub struct Cell {
    pub id: CellId,
    pub formula: Option<String>,
    pub value: CellValue,
}

impl Cell {
    pub fn new(id: CellId) -> Self {
        Self { id, formula: None, value: CellValue::Empty }
    }

    pub fn with_value(id: CellId, value: CellValue) -> Self {
        Self { id, formula: None, value }
    }

    pub fn with_formula(id: CellId, formula: String) -> Self {
        Self { id, formula: Some(formula), value: CellValue::Empty }
    }
}
