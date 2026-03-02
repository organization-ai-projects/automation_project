use crate::determinism::logical_clock::LogicalClock;
use crate::determinism::seed::Seed;
use crate::diagnostics::error::RuntimeError;
use crate::event_log::event_log::EventLog;
use crate::graph::graph::Graph;
use crate::scheduler::scheduler::Scheduler;

/// Bundles a `Seed` and `LogicalClock` to drive deterministic execution.
/// Use `run` to execute the graph, recording every job into an `EventLog`.
pub struct DeterministicContext {
    seed: Seed,
    clock: LogicalClock,
}

impl DeterministicContext {
    pub fn new(seed: Seed) -> Self {
        Self {
            seed,
            clock: LogicalClock::new(),
        }
    }

    pub fn seed(&self) -> Seed {
        self.seed
    }

    pub fn clock(&self) -> &LogicalClock {
        &self.clock
    }

    /// Executes the graph deterministically and returns a populated `EventLog`.
    /// No `SystemTime` is used; ordering is driven solely by the graph topology
    /// and stable `RuntimeId` ordering.
    pub fn run(&mut self, graph: Graph) -> Result<EventLog, RuntimeError> {
        let scheduler = Scheduler::new(graph);
        let jobs = scheduler.schedule()?;
        let mut log = EventLog::new();
        for job in &jobs {
            self.clock.tick();
            log.record(job);
        }
        Ok(log)
    }
}
