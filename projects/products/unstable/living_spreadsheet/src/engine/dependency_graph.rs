use std::collections::{HashMap, HashSet, VecDeque};
use crate::diagnostics::error::SpreadsheetError;
use crate::model::cell_id::CellId;

pub struct DependencyGraph {
    deps: HashMap<CellId, HashSet<CellId>>,
    rdeps: HashMap<CellId, HashSet<CellId>>,
}

impl DependencyGraph {
    pub fn new() -> Self {
        Self {
            deps: HashMap::new(),
            rdeps: HashMap::new(),
        }
    }

    pub fn set_deps(&mut self, cell: CellId, new_deps: Vec<CellId>) {
        // Remove old reverse deps
        if let Some(old_deps) = self.deps.get(&cell) {
            let old_deps: Vec<_> = old_deps.iter().cloned().collect();
            for dep in old_deps {
                if let Some(rdep_set) = self.rdeps.get_mut(&dep) {
                    rdep_set.remove(&cell);
                }
            }
        }

        let dep_set: HashSet<CellId> = new_deps.into_iter().collect();
        for dep in &dep_set {
            self.rdeps.entry(dep.clone()).or_default().insert(cell.clone());
        }
        self.deps.insert(cell, dep_set);
    }

    pub fn deps_of(&self, cell: &CellId) -> Vec<CellId> {
        let mut result = self.deps.get(cell).map(|s| s.iter().cloned().collect::<Vec<_>>()).unwrap_or_default();
        result.sort();
        result
    }

    /// Returns all cells transitively depending on `changed` (not including `changed` itself).
    pub fn affected(&self, changed: &CellId) -> Vec<CellId> {
        let mut visited = HashSet::new();
        let mut queue = VecDeque::new();
        if let Some(direct) = self.rdeps.get(changed) {
            for cell in direct {
                if visited.insert(cell.clone()) {
                    queue.push_back(cell.clone());
                }
            }
        }
        while let Some(cell) = queue.pop_front() {
            if let Some(next) = self.rdeps.get(&cell) {
                for c in next {
                    if visited.insert(c.clone()) {
                        queue.push_back(c.clone());
                    }
                }
            }
        }
        visited.into_iter().collect()
    }

    /// Kahn's algorithm to check for cycles.
    pub fn check_cycles(&self) -> Result<(), SpreadsheetError> {
        self.topo_order().map(|_| ())
    }

    /// Topological order of all cells that have deps entries (formula cells).
    pub fn topo_order(&self) -> Result<Vec<CellId>, SpreadsheetError> {
        // Build in-degree map for all known cells
        let mut all_cells: HashSet<CellId> = HashSet::new();
        for (cell, deps) in &self.deps {
            all_cells.insert(cell.clone());
            for d in deps {
                all_cells.insert(d.clone());
            }
        }

        let mut in_degree: HashMap<CellId, usize> = HashMap::new();
        for cell in &all_cells {
            in_degree.entry(cell.clone()).or_insert(0);
        }
        for (cell, deps) in &self.deps {
            // cell depends on deps => in_degree of cell increases per dep
            *in_degree.entry(cell.clone()).or_insert(0) += deps.len();
        }

        let mut queue: VecDeque<CellId> = in_degree.iter()
            .filter(|(_, deg)| **deg == 0)
            .map(|(c, _)| c.clone())
            .collect();

        // Sort for determinism
        let mut queue_vec: Vec<CellId> = queue.drain(..).collect();
        queue_vec.sort();
        let mut queue: VecDeque<CellId> = queue_vec.into_iter().collect();

        let mut order = Vec::new();

        while let Some(cell) = queue.pop_front() {
            order.push(cell.clone());
            // cells that depend on `cell`
            if let Some(rdep_set) = self.rdeps.get(&cell) {
                let mut next: Vec<CellId> = rdep_set.iter().cloned().collect();
                next.sort();
                for dependent in next {
                    let entry = in_degree.entry(dependent.clone()).or_insert(0);
                    if *entry > 0 {
                        *entry -= 1;
                    }
                    if *entry == 0 {
                        queue.push_back(dependent);
                    }
                }
            }
        }

        if order.len() < all_cells.len() {
            Err(SpreadsheetError::CycleDetected)
        } else {
            // Return only cells that have formula entries (have deps)
            Ok(order.into_iter().filter(|c| self.deps.contains_key(c)).collect())
        }
    }

    pub fn remove_cell(&mut self, cell: &CellId) {
        if let Some(old_deps) = self.deps.remove(cell) {
            for dep in old_deps {
                if let Some(rdep_set) = self.rdeps.get_mut(&dep) {
                    rdep_set.remove(cell);
                }
            }
        }
        // Note: do NOT remove cell from rdeps — other cells may still reference it
    }

    pub fn all_cells_with_deps(&self) -> Vec<CellId> {
        self.deps.keys().cloned().collect()
    }
}

impl Default for DependencyGraph {
    fn default() -> Self {
        Self::new()
    }
}
