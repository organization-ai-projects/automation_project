use crate::jobs::job::Job;
use crate::jobs::job_id::JobId;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct JobQueue {
    pub jobs: BTreeMap<JobId, Job>,
}

impl JobQueue {
    pub fn add(&mut self, job: Job) {
        self.jobs.insert(job.id, job);
    }
    pub fn remove(&mut self, id: &JobId) -> Option<Job> {
        self.jobs.remove(id)
    }
    pub fn unassigned_sorted(&self) -> Vec<&Job> {
        let mut jobs: Vec<&Job> = self
            .jobs
            .values()
            .filter(|j| j.assigned_to.is_none())
            .collect();
        jobs.sort_by(|a, b| b.priority.cmp(&a.priority).then(a.id.cmp(&b.id)));
        jobs
    }
}
