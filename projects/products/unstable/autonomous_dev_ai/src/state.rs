// projects/products/unstable/autonomous_dev_ai/src/state.rs

use serde::{Deserialize, Serialize};

/// Agent state machine states
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum AgentState {
    Idle,
    LoadConfig,
    LoadMemory,
    ReceiveGoal,
    ExploreRepository,
    GeneratePlan,
    ExecuteStep,
    Verify,
    EvaluateObjectives,
    PrCreation,
    ReviewFeedback,
    Done,
    Blocked,
    Failed,
}

impl AgentState {
    /// Check if this is a terminal state
    pub fn is_terminal(&self) -> bool {
        matches!(
            self,
            AgentState::Done | AgentState::Blocked | AgentState::Failed
        )
    }

    /// Get the next state after successful completion
    pub fn next_on_success(&self) -> Option<AgentState> {
        match self {
            AgentState::Idle => Some(AgentState::LoadConfig),
            AgentState::LoadConfig => Some(AgentState::LoadMemory),
            AgentState::LoadMemory => Some(AgentState::ReceiveGoal),
            AgentState::ReceiveGoal => Some(AgentState::ExploreRepository),
            AgentState::ExploreRepository => Some(AgentState::GeneratePlan),
            AgentState::GeneratePlan => Some(AgentState::ExecuteStep),
            AgentState::ExecuteStep => Some(AgentState::Verify),
            AgentState::Verify => Some(AgentState::EvaluateObjectives),
            AgentState::EvaluateObjectives => Some(AgentState::ExecuteStep), // Loop
            AgentState::PrCreation => Some(AgentState::ReviewFeedback),
            AgentState::ReviewFeedback => Some(AgentState::Done),
            AgentState::Done | AgentState::Blocked | AgentState::Failed => None,
        }
    }
}
