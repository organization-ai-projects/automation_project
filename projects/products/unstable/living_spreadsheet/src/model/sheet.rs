use std::collections::HashMap;
use crate::model::cell_id::CellId;
use crate::model::cell_value::CellValue;
use crate::model::cell::Cell;

pub struct Sheet {
    cells: HashMap<CellId, Cell>,
}

impl Sheet {
    pub fn new() -> Self {
        Self { cells: HashMap::new() }
    }

    pub fn set_value(&mut self, id: CellId, value: CellValue) {
        let cell = Cell::with_value(id.clone(), value);
        self.cells.insert(id, cell);
    }

    pub fn set_formula(&mut self, id: CellId, formula: String) {
        let cell = Cell::with_formula(id.clone(), formula);
        self.cells.insert(id, cell);
    }

    pub fn get(&self, id: &CellId) -> Option<&Cell> {
        self.cells.get(id)
    }

    pub fn get_value(&self, id: &CellId) -> CellValue {
        self.cells.get(id).map(|c| c.value.clone()).unwrap_or(CellValue::Empty)
    }

    pub fn cell_ids(&self) -> impl Iterator<Item = &CellId> {
        self.cells.keys()
    }

    pub fn cells(&self) -> impl Iterator<Item = &Cell> {
        self.cells.values()
    }

    /// Update only the computed value of an existing cell (keeps formula).
    pub fn update_value(&mut self, id: &CellId, value: CellValue) {
        if let Some(cell) = self.cells.get_mut(id) {
            cell.value = value;
        }
    }
}

impl Default for Sheet {
    fn default() -> Self {
        Self::new()
    }
}
