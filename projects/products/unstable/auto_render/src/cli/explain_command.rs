use super::CliError;
use crate::plan::Plan;
use common_ron::read_ron;
use std::path::PathBuf;

pub struct ExplainCommand {
    pub plan_path: PathBuf,
}

impl ExplainCommand {
    pub fn run(&self) -> Result<(), CliError> {
        let plan: Plan = read_ron(&self.plan_path)
            .map_err(|e| CliError::Parse(format!("Failed to parse plan: {e}")))?;

        println!("Plan: {}", plan.metadata.plan_id.0);
        println!("Created: {}", plan.metadata.created_at);
        println!(
            "Planner: {} v{}",
            plan.metadata.planner_id, plan.metadata.planner_version
        );
        println!(
            "Schema: {}.{}.{}",
            plan.metadata.plan_schema_version.major,
            plan.metadata.plan_schema_version.minor,
            plan.metadata.plan_schema_version.patch,
        );
        println!("Explain: {}", plan.metadata.explain);
        println!("Actions ({}):", plan.actions.len());

        for (i, action) in plan.actions.iter().enumerate() {
            println!(
                "  [{}] {} ({:?}) requires {:?}",
                i + 1,
                action.action_id,
                action.action_type,
                action.capability_required
            );
            if !action.preconditions.is_empty() {
                println!("      pre: {}", action.preconditions[0].description);
            }
            if !action.postconditions.is_empty() {
                println!("      post: {}", action.postconditions[0].description);
            }
        }

        Ok(())
    }
}
