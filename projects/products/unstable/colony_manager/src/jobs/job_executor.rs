use crate::jobs::job_id::JobId;
use crate::jobs::job_queue::JobQueue;
use crate::model::colonist::Colonist;
use crate::model::colonist_id::ColonistId;
use std::collections::BTreeMap;

pub struct JobExecutor;

impl JobExecutor {
    pub fn execute_tick(
        colonists: &mut BTreeMap<ColonistId, Colonist>,
        queue: &mut JobQueue,
    ) {
        let finished_jobs: Vec<JobId> = {
            let mut v = Vec::new();
            for job in queue.jobs.values_mut() {
                if let Some(cid) = job.assigned_to {
                    if let Some(colonist) = colonists.get(&cid) {
                        let work = (colonist.productivity * 1.0) as u32;
                        let work = work.max(1);
                        job.ticks_remaining = job.ticks_remaining.saturating_sub(work);
                    }
                    if job.ticks_remaining == 0 {
                        v.push(job.id);
                    }
                }
            }
            v
        };
        for jid in finished_jobs {
            if let Some(job) = queue.jobs.remove(&jid) {
                if let Some(cid) = job.assigned_to {
                    if let Some(c) = colonists.get_mut(&cid) {
                        c.assigned_job = None;
                    }
                }
            }
        }
    }
}
