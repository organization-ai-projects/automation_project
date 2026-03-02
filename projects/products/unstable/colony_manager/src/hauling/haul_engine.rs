use crate::hauling::haul_task::HaulTask;
use crate::model::colonist_id::ColonistId;
use crate::model::item_id::ItemId;
use std::collections::BTreeMap;

#[derive(Debug, Clone, Default)]
pub struct HaulEngine {
    pub tasks: BTreeMap<ItemId, HaulTask>,
}

impl HaulEngine {
    pub fn add_task(&mut self, task: HaulTask) {
        self.tasks.insert(task.item_id, task);
    }
    pub fn assign_tick(&mut self, available: &[ColonistId]) {
        let mut ai = available.iter();
        for task in self.tasks.values_mut() {
            if task.assigned_to.is_none() {
                task.assigned_to = ai.next().copied();
            }
        }
    }
}
