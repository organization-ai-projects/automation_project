use crate::jobs::job_id::JobId;
use crate::jobs::job_queue::JobQueue;
use crate::model::colonist::Colonist;
use crate::model::colonist_id::ColonistId;
use std::collections::BTreeMap;

pub struct JobAssigner;

impl JobAssigner {
    pub fn assign(
        colonists: &BTreeMap<ColonistId, Colonist>,
        queue: &JobQueue,
    ) -> Vec<(ColonistId, JobId)> {
        let available: Vec<ColonistId> = {
            let mut v: Vec<ColonistId> = colonists
                .values()
                .filter(|c| c.assigned_job.is_none())
                .map(|c| c.id)
                .collect();
            v.sort();
            v
        };
        let unassigned = queue.unassigned_sorted();
        available.into_iter()
            .zip(unassigned.into_iter())
            .map(|(cid, job)| (cid, job.id))
            .collect()
    }
}
