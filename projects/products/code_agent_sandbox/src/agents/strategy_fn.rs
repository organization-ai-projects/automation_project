// projects/products/code_agent_sandbox/src/agents/strategy_fn.rs
use ai::{AiBody, AiError, Task, TaskResult};

pub type StrategyFn = fn(&mut AiBody, &Task) -> Result<TaskResult, AiError>;
