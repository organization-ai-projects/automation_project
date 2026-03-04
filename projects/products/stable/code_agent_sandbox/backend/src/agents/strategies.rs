// projects/products/code_agent_sandbox/src/agents/strategies.rs
use ai::AiBody;

use crate::agents::StrategyFn;

pub(crate) const STRATEGIES: &[(&str, StrategyFn)] = &[
    ("auto", AiBody::solve),
    ("symbolicthenneural", AiBody::solve_symbolic_then_neural),
    (
        "neuralwithsymbolicvalidation",
        AiBody::solve_neural_with_validation,
    ),
    ("hybrid", AiBody::solve_hybrid),
];
