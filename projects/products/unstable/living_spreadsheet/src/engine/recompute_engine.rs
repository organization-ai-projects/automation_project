use std::collections::HashSet;
use runtime_core::public_api::{EventLog, Job, RuntimeId, Seed};
use crate::diagnostics::error::SpreadsheetError;
use crate::engine::dependency_graph::DependencyGraph;
use crate::explain::trace::{Trace, TraceStep};
use crate::formula::evaluator::Evaluator;
use crate::formula::parser::{extract_deps, Parser};
use crate::model::cell_id::CellId;
use crate::model::cell_value::CellValue;
use crate::model::sheet::Sheet;

pub struct RecomputeEngine {
    sheet: Sheet,
    dep_graph: DependencyGraph,
    event_log: EventLog,
    next_id: u64,
    #[allow(dead_code)]
    seed: Seed,
}

fn cell_runtime_id(cell: &CellId) -> RuntimeId {
    RuntimeId::new(cell.row as u64 * 10000 + cell.col as u64)
}

impl RecomputeEngine {
    pub fn new(seed: Seed) -> Self {
        Self {
            sheet: Sheet::new(),
            dep_graph: DependencyGraph::new(),
            event_log: EventLog::new(),
            next_id: 0,
            seed,
        }
    }

    pub fn sheet(&self) -> &Sheet {
        &self.sheet
    }

    pub fn sheet_mut(&mut self) -> &mut Sheet {
        &mut self.sheet
    }

    pub fn set_value(&mut self, id: CellId, value: CellValue) -> Result<(), SpreadsheetError> {
        self.dep_graph.remove_cell(&id);
        self.sheet.set_value(id, value);
        Ok(())
    }

    pub fn set_formula(&mut self, id: CellId, formula: String) -> Result<(), SpreadsheetError> {
        let expr = Parser::parse(&formula)?;
        let deps = extract_deps(&expr);
        self.dep_graph.set_deps(id.clone(), deps);
        self.sheet.set_formula(id, formula);
        self.dep_graph.check_cycles()?;
        Ok(())
    }

    pub fn recompute_from(&mut self, changed: &CellId) -> Result<Vec<CellId>, SpreadsheetError> {
        let affected: HashSet<CellId> = self.dep_graph.affected(changed).into_iter().collect();
        let topo = self.dep_graph.topo_order()?;
        let to_recompute: Vec<CellId> = topo.into_iter().filter(|c| affected.contains(c)).collect();

        for cell in &to_recompute {
            self.recompute_cell(cell)?;
        }
        Ok(to_recompute)
    }

    pub fn recompute_all(&mut self) -> Result<Vec<CellId>, SpreadsheetError> {
        let topo = self.dep_graph.topo_order()?;
        for cell in &topo {
            self.recompute_cell(cell)?;
        }
        Ok(topo)
    }

    fn recompute_cell(&mut self, id: &CellId) -> Result<(), SpreadsheetError> {
        let formula = match self.sheet.get(id) {
            Some(cell) => match &cell.formula {
                Some(f) => f.clone(),
                None => return Ok(()),
            },
            None => return Ok(()),
        };

        let expr = Parser::parse(&formula)?;
        let evaluator = Evaluator::new(&self.sheet);
        let value = evaluator.eval(&expr).unwrap_or_else(|e| CellValue::Error(e.to_string()));

        self.sheet.update_value(id, value);

        let job = Job::new(RuntimeId::new(self.next_id), cell_runtime_id(id));
        self.next_id += 1;
        self.event_log.record(&job);
        Ok(())
    }

    pub fn event_log(&self) -> &EventLog {
        &self.event_log
    }

    pub fn trace(&self, id: &CellId) -> Result<Trace, SpreadsheetError> {
        let cell = self.sheet.get(id).ok_or_else(|| {
            SpreadsheetError::UnknownCell(id.to_string())
        })?;

        let mut steps = Vec::new();

        // Gather dep steps recursively (simple DFS)
        let deps = self.dep_graph.deps_of(id);
        for dep in &deps {
            let dep_cell = self.sheet.get(dep);
            let step = TraceStep {
                cell: dep.clone(),
                formula: dep_cell.and_then(|c| c.formula.clone()),
                deps: self.dep_graph.deps_of(dep),
                result: self.sheet.get_value(dep),
            };
            steps.push(step);
        }

        // Target step
        steps.push(TraceStep {
            cell: id.clone(),
            formula: cell.formula.clone(),
            deps,
            result: cell.value.clone(),
        });

        Ok(Trace::new(id.clone(), steps))
    }

    pub fn get_value(&self, id: &CellId) -> CellValue {
        self.sheet.get_value(id)
    }
}
