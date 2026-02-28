use crate::model::cell_id::CellId;
use crate::model::cell_value::CellValue;

#[derive(Debug, Clone, PartialEq)]
pub struct TraceStep {
    pub cell: CellId,
    pub formula: Option<String>,
    pub deps: Vec<CellId>,
    pub result: CellValue,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Trace {
    pub target: CellId,
    pub steps: Vec<TraceStep>,
}

impl Trace {
    pub fn new(target: CellId, steps: Vec<TraceStep>) -> Self {
        Self { target, steps }
    }

    pub fn target_value(&self) -> Option<&CellValue> {
        self.steps.last().map(|s| &s.result)
    }
}
