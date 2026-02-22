// projects/products/unstable/auto_manager_ai/src/artifact_contract.rs

use crate::domain::{ActionPlan, RunReport};

const EXPECTED_SCHEMA_VERSION: &str = "1";
const EXPECTED_PRODUCER: &str = "auto_manager_ai";

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ArtifactContractErrorCode {
    ActionPlanSchemaVersion,
    ActionPlanProducer,
    RunReportSchemaVersion,
    RunReportProducer,
}

impl ArtifactContractErrorCode {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::ActionPlanSchemaVersion => "ARTIFACT_CONTRACT_ACTION_PLAN_SCHEMA_VERSION_INVALID",
            Self::ActionPlanProducer => "ARTIFACT_CONTRACT_ACTION_PLAN_PRODUCER_INVALID",
            Self::RunReportSchemaVersion => "ARTIFACT_CONTRACT_RUN_REPORT_SCHEMA_VERSION_INVALID",
            Self::RunReportProducer => "ARTIFACT_CONTRACT_RUN_REPORT_PRODUCER_INVALID",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ArtifactContractError {
    pub code: ArtifactContractErrorCode,
    pub message: String,
}

impl ArtifactContractError {
    pub fn render(&self) -> String {
        format!("[{}] {}", self.code.as_str(), self.message)
    }
}

pub fn validate_action_plan_contract(plan: &ActionPlan) -> Result<(), ArtifactContractError> {
    if plan.schema_version != EXPECTED_SCHEMA_VERSION {
        return Err(ArtifactContractError {
            code: ArtifactContractErrorCode::ActionPlanSchemaVersion,
            message: format!(
                "Expected schema_version='{}', got '{}'",
                EXPECTED_SCHEMA_VERSION, plan.schema_version
            ),
        });
    }
    if plan.producer != EXPECTED_PRODUCER {
        return Err(ArtifactContractError {
            code: ArtifactContractErrorCode::ActionPlanProducer,
            message: format!(
                "Expected producer='{}', got '{}'",
                EXPECTED_PRODUCER, plan.producer
            ),
        });
    }
    Ok(())
}

pub fn validate_run_report_contract(report: &RunReport) -> Result<(), ArtifactContractError> {
    if report.schema_version != EXPECTED_SCHEMA_VERSION {
        return Err(ArtifactContractError {
            code: ArtifactContractErrorCode::RunReportSchemaVersion,
            message: format!(
                "Expected schema_version='{}', got '{}'",
                EXPECTED_SCHEMA_VERSION, report.schema_version
            ),
        });
    }
    if report.producer != EXPECTED_PRODUCER {
        return Err(ArtifactContractError {
            code: ArtifactContractErrorCode::RunReportProducer,
            message: format!(
                "Expected producer='{}', got '{}'",
                EXPECTED_PRODUCER, report.producer
            ),
        });
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::{
        artifact_contract::{validate_action_plan_contract, validate_run_report_contract},
        domain::{ActionPlan, RunReport},
    };

    #[test]
    fn validates_default_action_plan_contract() {
        let plan = ActionPlan::new("test".to_string());
        assert!(validate_action_plan_contract(&plan).is_ok());
    }

    #[test]
    fn validates_default_run_report_contract() {
        let report = RunReport::new("run_1".to_string());
        assert!(validate_run_report_contract(&report).is_ok());
    }
}
