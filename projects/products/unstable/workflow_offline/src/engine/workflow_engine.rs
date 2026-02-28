use crate::config::workflow_config::WorkflowConfig;
use crate::diagnostics::error::WorkflowError;
use crate::exec::command_exec::CommandExec;
use crate::exec::exec_result::ExecResult;
use crate::logging::run_report::{JobReport, RunReport};
use runtime_core::{DeterministicContext, Edge, EventLog, Graph, Node, RuntimeId, Seed};
use std::collections::HashMap;

/// Drives execution of a deterministic job DAG.
pub struct WorkflowEngine {
    config: WorkflowConfig,
    seed: u64,
    dry_run: bool,
}

impl WorkflowEngine {
    pub fn new(config: WorkflowConfig, seed: u64, dry_run: bool) -> Self {
        Self {
            config,
            seed,
            dry_run,
        }
    }

    /// Builds the runtime_core `Graph` from the workflow config.
    /// Returns `WorkflowError::DagError` on unknown dependency references.
    fn build_graph(&self) -> Result<(Graph, HashMap<RuntimeId, String>), WorkflowError> {
        // Assign stable RuntimeIds by sorted job position for determinism.
        let mut sorted_ids: Vec<&str> = self.config.jobs.iter().map(|j| j.id.as_str()).collect();
        sorted_ids.sort_unstable();

        let id_map: HashMap<&str, RuntimeId> = sorted_ids
            .iter()
            .enumerate()
            .map(|(i, &name)| (name, RuntimeId::new(i as u64)))
            .collect();

        let nodes: Vec<Node> = self
            .config
            .jobs
            .iter()
            .map(|j| Node::new(id_map[j.id.as_str()], j.id.clone()))
            .collect();

        let mut edges: Vec<Edge> = Vec::new();
        for job in &self.config.jobs {
            let to = id_map[job.id.as_str()];
            for dep in &job.deps {
                let from = *id_map.get(dep.as_str()).ok_or_else(|| {
                    WorkflowError::DagError(format!(
                        "job `{}` depends on unknown job `{dep}`",
                        job.id
                    ))
                })?;
                edges.push(Edge::new(from, to));
            }
        }

        // Build reverse map: RuntimeId -> job string id
        let rev_map: HashMap<RuntimeId, String> = id_map
            .into_iter()
            .map(|(name, rid)| (rid, name.to_string()))
            .collect();

        Ok((Graph::new(nodes, edges), rev_map))
    }

    /// Validates the DAG and returns the planned execution order as job string IDs.
    #[allow(dead_code)]
    pub fn planned_order(&self) -> Result<Vec<String>, WorkflowError> {
        let (graph, rev_map) = self.build_graph()?;
        let order = graph
            .topological_order()
            .map_err(|e| WorkflowError::DagError(e.to_string()))?;
        order
            .into_iter()
            .map(|rid| {
                rev_map
                    .get(&rid)
                    .cloned()
                    .ok_or_else(|| WorkflowError::Internal("missing id in reverse map".to_string()))
            })
            .collect()
    }

    /// Runs the workflow.
    ///
    /// In dry-run mode, validates the DAG and prints the planned order without
    /// executing any commands.  Returns an empty `RunReport` with `success = true`.
    ///
    /// In normal mode, executes each job in topological order, records events in
    /// a runtime_core `EventLog`, and returns a fully-populated `RunReport`.
    ///
    /// Stops at the first failing job and returns `WorkflowError::JobFailure`.
    pub fn run(&self) -> Result<RunReport, WorkflowError> {
        let (graph, rev_map) = self.build_graph()?;
        let order = graph
            .topological_order()
            .map_err(|e| WorkflowError::DagError(e.to_string()))?;

        if self.dry_run {
            println!("Planned execution order:");
            for rid in &order {
                if let Some(name) = rev_map.get(rid) {
                    println!("  {name}");
                }
            }
            return Ok(RunReport {
                workflow_name: self.config.name.clone(),
                seed: self.seed,
                jobs: vec![],
                success: true,
                event_log_json: String::new(),
            });
        }

        // Build a lookup: job_id_string -> JobConfig
        let job_lookup: HashMap<&str, &crate::config::job_config::JobConfig> = self
            .config
            .jobs
            .iter()
            .map(|j| (j.id.as_str(), j))
            .collect();

        let ctx = DeterministicContext::new(Seed::new(self.seed));
        let _ = ctx.seed(); // bind seed into context; ordering is topology-driven
        let mut event_log = EventLog::new();
        let mut job_reports: Vec<JobReport> = Vec::new();

        for (seq, rid) in order.iter().enumerate() {
            let job_name = rev_map
                .get(rid)
                .ok_or_else(|| WorkflowError::Internal("missing id in reverse map".to_string()))?;
            let job_cfg = job_lookup.get(job_name.as_str()).ok_or_else(|| {
                WorkflowError::Internal(format!("missing config for `{job_name}`"))
            })?;

            // Record event via runtime_core EventLog adapter.
            let runtime_job = runtime_core::Job::new(RuntimeId::new(seq as u64), *rid);
            event_log.record(&runtime_job);

            let result: ExecResult =
                CommandExec::new(&job_cfg.command, job_cfg.args.clone()).execute();

            let failed = !result.success();
            let exit_code = result.exit_code;
            job_reports.push(JobReport::new(job_name.clone(), result));

            if failed {
                return Err(WorkflowError::JobFailure {
                    job: job_name.clone(),
                    exit_code,
                });
            }
        }

        let event_log_json = String::from_utf8(
            event_log
                .serialize()
                .map_err(|e| WorkflowError::Internal(e.to_string()))?,
        )
        .map_err(|e| WorkflowError::Internal(e.to_string()))?;

        Ok(RunReport {
            workflow_name: self.config.name.clone(),
            seed: self.seed,
            jobs: job_reports,
            success: true,
            event_log_json,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::job_config::JobConfig;

    fn make_job(id: &str, deps: Vec<&str>) -> JobConfig {
        JobConfig {
            id: id.to_string(),
            command: "echo".to_string(),
            args: vec![id.to_string()],
            deps: deps.into_iter().map(|s| s.to_string()).collect(),
        }
    }

    fn make_engine(jobs: Vec<JobConfig>, seed: u64, dry_run: bool) -> WorkflowEngine {
        let config = WorkflowConfig {
            name: "test_workflow".to_string(),
            jobs,
        };
        WorkflowEngine::new(config, seed, dry_run)
    }

    #[test]
    fn dag_build_with_missing_dep_fails() {
        let engine = make_engine(vec![make_job("b", vec!["nonexistent"])], 0, false);
        assert!(matches!(
            engine.planned_order(),
            Err(WorkflowError::DagError(_))
        ));
    }

    #[test]
    fn dag_cycle_returns_dag_error() {
        let mut job_a = make_job("a", vec!["b"]);
        let mut job_b = make_job("b", vec!["a"]);
        // Manually fix up - make_job doesn't support cross-deps so override
        job_a.deps = vec!["b".to_string()];
        job_b.deps = vec!["a".to_string()];
        let engine = make_engine(vec![job_a, job_b], 0, false);
        assert!(matches!(
            engine.planned_order(),
            Err(WorkflowError::DagError(_))
        ));
    }

    #[test]
    fn deterministic_order_same_seed() {
        let jobs = vec![
            make_job("c", vec!["a"]),
            make_job("a", vec![]),
            make_job("b", vec!["a"]),
        ];
        let engine1 = make_engine(jobs.clone(), 42, false);
        let engine2 = make_engine(jobs, 42, false);
        let order1 = engine1.planned_order().unwrap();
        let order2 = engine2.planned_order().unwrap();
        assert_eq!(order1, order2);
        // "a" must appear before "b" and "c"
        let pos = |name: &str| order1.iter().position(|x| x == name).unwrap();
        assert!(pos("a") < pos("b"));
        assert!(pos("a") < pos("c"));
    }

    #[test]
    #[cfg(unix)]
    fn run_single_job_succeeds() {
        let engine = make_engine(vec![make_job("only", vec![])], 0, false);
        let report = engine.run().unwrap();
        assert!(report.success);
        assert_eq!(report.jobs.len(), 1);
        assert_eq!(report.jobs[0].job_id, "only");
    }

    #[test]
    #[cfg(unix)]
    fn failing_job_returns_job_failure() {
        let bad_job = JobConfig {
            id: "bad".to_string(),
            command: "false".to_string(),
            args: vec![],
            deps: vec![],
        };
        let engine = make_engine(vec![bad_job], 0, false);
        assert!(matches!(
            engine.run(),
            Err(WorkflowError::JobFailure { .. })
        ));
    }
}
