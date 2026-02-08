// projects/products/unstable/autonomous_dev_ai/src/lifecycle.rs

//! Agent lifecycle implementation

use crate::audit::AuditLogger;
use crate::config::AgentConfig;
use crate::error::{AgentError, AgentResult};
use crate::memory::MemoryGraph;
use crate::neural::NeuralLayer;
use crate::objectives::ObjectiveEvaluator;
use crate::state::AgentState;
use crate::symbolic::SymbolicController;
use crate::symbolic::planner::Plan;
use crate::symbolic::policy::PolicyEngine;
use crate::tools::{GitWrapper, RepoReader, TestRunner, ToolRegistry};

/// Agent lifecycle manager
pub struct LifecycleManager {
    pub state: AgentState,
    pub config: AgentConfig,
    pub memory: MemoryGraph,
    pub symbolic: SymbolicController,
    pub neural: NeuralLayer,
    pub policy: PolicyEngine,
    pub tools: ToolRegistry,
    pub audit: AuditLogger,
    pub iteration: usize,
    pub current_plan: Option<Plan>,
    pub current_step: usize,
}

impl LifecycleManager {
    pub fn new(config: AgentConfig, audit_log_path: &str) -> Self {
        let evaluator = ObjectiveEvaluator::new(config.objectives.clone());
        let symbolic = SymbolicController::new(
            evaluator,
            config.symbolic.strict_validation,
            config.symbolic.deterministic,
        );
        let neural = NeuralLayer::new(
            config.neural.enabled,
            config.neural.prefer_gpu,
            config.neural.cpu_fallback,
        );
        let policy = PolicyEngine::new();

        let mut tools = ToolRegistry::new();
        tools.register(Box::new(RepoReader));
        tools.register(Box::new(TestRunner));
        tools.register(Box::new(GitWrapper));

        Self {
            state: AgentState::Idle,
            config,
            memory: MemoryGraph::new(),
            symbolic,
            neural,
            policy,
            tools,
            audit: AuditLogger::new(audit_log_path),
            iteration: 0,
            current_plan: None,
            current_step: 0,
        }
    }

    /// Run the agent lifecycle
    pub fn run(&mut self, goal: &str) -> AgentResult<()> {
        self.transition_to(AgentState::LoadConfig)?;
        self.transition_to(AgentState::LoadMemory)?;
        self.transition_to(AgentState::ReceiveGoal)?;

        // Store the goal
        self.memory
            .metadata
            .insert("goal".to_string(), goal.to_string());

        // Main loop
        while !self.state.is_terminal() {
            match self.state {
                AgentState::ExploreRepository => self.explore_repository()?,
                AgentState::GeneratePlan => self.generate_plan()?,
                AgentState::ExecuteStep => self.execute_step()?,
                AgentState::Verify => self.verify_step()?,
                AgentState::EvaluateObjectives => self.evaluate_objectives()?,
                AgentState::PrCreation => self.create_pr()?,
                AgentState::ReviewFeedback => self.handle_review()?,
                _ => {
                    if let Some(next) = self.state.next_on_success() {
                        self.transition_to(next)?;
                    } else {
                        break;
                    }
                }
            }
        }

        Ok(())
    }

    /// Transition to a new state
    pub fn transition_to(&mut self, new_state: AgentState) -> AgentResult<()> {
        let old_state = format!("{:?}", self.state);
        let new_state_str = format!("{:?}", new_state);

        self.audit
            .log_state_transition(&old_state, &new_state_str)
            .map_err(|e| AgentError::State(e.to_string()))?;

        self.state = new_state;
        Ok(())
    }

    /// Explore repository
    fn explore_repository(&mut self) -> AgentResult<()> {
        tracing::info!("Exploring repository...");

        // Stub: would explore actual repo
        self.memory.add_explored_file("README.md".to_string());
        self.memory.add_explored_file("src/main.rs".to_string());

        self.transition_to(AgentState::GeneratePlan)
    }

    /// Generate execution plan
    fn generate_plan(&mut self) -> AgentResult<()> {
        tracing::info!("Generating plan...");

        let goal = self
            .memory
            .metadata
            .get("goal")
            .ok_or_else(|| AgentError::State("No goal set".to_string()))?;

        // Get neural suggestion if enabled
        let neural_suggestion = self.neural.propose_action(goal)?;

        if let Some(ref suggestion) = neural_suggestion {
            self.audit
                .log_neural_suggestion(&suggestion.action, suggestion.confidence)
                .map_err(|e| AgentError::State(e.to_string()))?;
        }

        // Create plan
        let mut plan = Plan::new(goal.clone());
        // Add some basic steps
        plan.add_step(crate::symbolic::planner::PlanStep {
            description: "Read repository structure".to_string(),
            tool: "read_file".to_string(),
            args: vec!["Cargo.toml".to_string()],
            verification: "file_exists".to_string(),
        });

        self.memory.add_plan(
            self.iteration,
            "Initial plan".to_string(),
            plan.steps.iter().map(|s| s.description.clone()).collect(),
        );

        self.current_plan = Some(plan);
        self.current_step = 0;

        self.transition_to(AgentState::ExecuteStep)
    }

    /// Execute current step
    fn execute_step(&mut self) -> AgentResult<()> {
        let plan = self
            .current_plan
            .as_ref()
            .ok_or_else(|| AgentError::State("No plan available".to_string()))?;

        if self.current_step >= plan.steps.len() {
            // All steps complete
            self.transition_to(AgentState::PrCreation)?;
            return Ok(());
        }

        let step = &plan.steps[self.current_step];
        tracing::info!("Executing step {}: {}", self.current_step, step.description);

        // Validate with policy
        if !self.policy.is_tool_allowed(&step.tool) {
            return Err(AgentError::PolicyViolation(format!(
                "Tool {} not allowed",
                step.tool
            )));
        }

        // Execute tool
        let tool = self
            .tools
            .get_tool(&step.tool)
            .ok_or_else(|| AgentError::Tool(format!("Tool {} not found", step.tool)))?;

        let result = tool.execute(&step.args)?;

        self.audit
            .log_tool_execution(&step.tool, &step.args, result.success)
            .map_err(|e| AgentError::State(e.to_string()))?;

        self.memory.add_decision(
            self.iteration,
            format!("Execute {}", step.tool),
            None,
            format!("Tool executed: {}", result.success),
        );

        self.current_step += 1;
        self.transition_to(AgentState::Verify)
    }

    /// Verify step execution
    fn verify_step(&mut self) -> AgentResult<()> {
        tracing::info!("Verifying step execution...");

        // Stub: would run actual verification
        // For now, assume success

        self.transition_to(AgentState::EvaluateObjectives)
    }

    /// Evaluate objectives
    fn evaluate_objectives(&mut self) -> AgentResult<()> {
        tracing::info!("Evaluating objectives...");

        // Stub scores
        let scores = vec![
            ("task_completion".to_string(), 0.5),
            ("policy_safety".to_string(), 1.0),
            ("tests_pass".to_string(), 1.0),
            ("minimal_diff".to_string(), 0.8),
            ("time_budget".to_string(), 0.7),
        ];

        let objective_scores = self.symbolic.evaluator.evaluate(&scores);
        let hard_satisfied = self
            .symbolic
            .evaluator
            .hard_objectives_satisfied(&objective_scores);

        self.audit
            .log_objective_evaluation(self.iteration, scores.clone())
            .map_err(|e| AgentError::State(e.to_string()))?;

        self.memory
            .add_objective_evaluation(self.iteration, scores, hard_satisfied);

        if !hard_satisfied {
            // Retry logic
            self.memory.add_failure(
                self.iteration,
                "Hard objectives not satisfied".to_string(),
                "Some objectives failed".to_string(),
                Some("Retry with adjusted approach".to_string()),
            );
            self.iteration += 1;

            // When hard objectives are not satisfied, always retry by generating a new plan.
            // This enforces the design that hard objectives cannot be violated on the path
            // to PR creation or completion.
            self.transition_to(AgentState::GeneratePlan)?;
        } else {
            self.transition_to(AgentState::ExecuteStep)?;
        }

        Ok(())
    }

    /// Create pull request
    fn create_pr(&mut self) -> AgentResult<()> {
        tracing::info!("Creating pull request...");

        let goal = self
            .memory
            .metadata
            .get("goal")
            .unwrap_or(&"No goal".to_string())
            .clone();

        let pr_body = format!(
            "## Goal\n{}\n\n## Plan\n{}\n\n## Iterations\n{}\n\n## Risk\nLow",
            goal,
            self.current_plan
                .as_ref()
                .map(|p| format!("{} steps", p.steps.len()))
                .unwrap_or_else(|| "No plan".to_string()),
            self.iteration
        );

        self.audit
            .log_symbolic_decision("create_pr", &pr_body)
            .map_err(|e| AgentError::State(e.to_string()))?;

        tracing::info!("PR would be created with body:\n{}", pr_body);

        self.transition_to(AgentState::ReviewFeedback)
    }

    /// Handle review feedback
    fn handle_review(&mut self) -> AgentResult<()> {
        tracing::info!("Handling review feedback...");

        // Stub: would handle actual review
        self.transition_to(AgentState::Done)
    }
}
