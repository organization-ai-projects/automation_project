// projects/products/stable/varina/backend/src/tests/test_helpers.rs
use common_json::{to_value, Json};
use protocol::{Command, CommandType, Metadata, Payload, ProtocolId, run_request::RunRequest};
use crate::autopilot::{AutopilotMode, AutopilotPlan, AutopilotReport};
use crate::classified_changes::ClassifiedChanges;

/// Create a test metadata with default values for testing
pub fn test_metadata() -> Metadata {
    Metadata {
        request_id: ProtocolId::default(),
        job_id: None,
        product_id: None,
        client_id: None,
        timestamp_ms: None,
        schema_version: None,
    }
}

/// Builder for Command objects in tests
pub struct CommandBuilder {
    metadata: Metadata,
    command_type: CommandType,
    action: Option<String>,
    payload: Option<Payload>,
}

impl CommandBuilder {
    pub fn new() -> Self {
        Self {
            metadata: test_metadata(),
            command_type: CommandType::Execute,
            action: None,
            payload: None,
        }
    }

    pub fn action(mut self, action: &str) -> Self {
        self.action = Some(action.to_string());
        self
    }

    pub fn payload_with_type(mut self, payload_type: &str, payload_value: Json) -> Self {
        self.payload = Some(Payload {
            payload_type: Some(payload_type.to_string()),
            payload: Some(to_value(&payload_value).unwrap()),
        });
        self
    }

    pub fn build(self) -> Command {
        Command {
            metadata: self.metadata,
            command_type: self.command_type,
            action: self.action,
            payload: self.payload,
        }
    }
}

impl Default for CommandBuilder {
    fn default() -> Self {
        Self::new()
    }
}

/// Builder for RunRequest objects in tests
pub struct RunRequestBuilder {
    request_id: ProtocolId,
    repo_path: Option<String>,
}

impl RunRequestBuilder {
    pub fn new() -> Self {
        Self {
            request_id: ProtocolId::default(),
            repo_path: None,
        }
    }

    pub fn repo_path(mut self, path: &str) -> Self {
        self.repo_path = Some(path.to_string());
        self
    }

    pub fn build(self) -> RunRequest {
        RunRequest {
            request_id: self.request_id,
            repo_path: self.repo_path,
        }
    }
}

impl Default for RunRequestBuilder {
    fn default() -> Self {
        Self::new()
    }
}

/// Builder for AutopilotPlan objects in tests
pub struct AutopilotPlanBuilder {
    branch: String,
    will_stage: Vec<String>,
    will_commit: bool,
    commit_message: String,
    will_push: bool,
    notes: Vec<String>,
}

impl AutopilotPlanBuilder {
    pub fn new() -> Self {
        Self {
            branch: "main".to_string(),
            will_stage: vec![],
            will_commit: false,
            commit_message: String::new(),
            will_push: false,
            notes: vec![],
        }
    }

    pub fn branch(mut self, branch: &str) -> Self {
        self.branch = branch.to_string();
        self
    }

    pub fn will_stage(mut self, files: Vec<String>) -> Self {
        self.will_stage = files;
        self
    }

    pub fn will_commit(mut self, commit: bool) -> Self {
        self.will_commit = commit;
        self
    }

    pub fn commit_message(mut self, message: &str) -> Self {
        self.commit_message = message.to_string();
        self
    }

    pub fn will_push(mut self, push: bool) -> Self {
        self.will_push = push;
        self
    }

    pub fn notes(mut self, notes: Vec<String>) -> Self {
        self.notes = notes;
        self
    }

    pub fn build(self) -> AutopilotPlan {
        AutopilotPlan {
            branch: self.branch,
            will_stage: self.will_stage,
            will_commit: self.will_commit,
            commit_message: self.commit_message,
            will_push: self.will_push,
            notes: self.notes,
        }
    }
}

impl Default for AutopilotPlanBuilder {
    fn default() -> Self {
        Self::new()
    }
}

/// Builder for AutopilotReport objects in tests
pub struct AutopilotReportBuilder {
    mode: AutopilotMode,
    branch: String,
    detached_head: bool,
    changes: Vec<String>,
    classified: ClassifiedChanges,
    plan: AutopilotPlan,
    applied: bool,
    logs: Vec<String>,
}

impl AutopilotReportBuilder {
    pub fn new() -> Self {
        Self {
            mode: AutopilotMode::DryRun,
            branch: "main".to_string(),
            detached_head: false,
            changes: vec![],
            classified: ClassifiedChanges {
                blocked: vec![],
                relevant: vec![],
                unrelated: vec![],
            },
            plan: AutopilotPlan {
                branch: "main".to_string(),
                will_stage: vec![],
                will_commit: false,
                commit_message: String::new(),
                will_push: false,
                notes: vec![],
            },
            applied: false,
            logs: vec![],
        }
    }

    pub fn mode(mut self, mode: AutopilotMode) -> Self {
        self.mode = mode;
        self
    }

    pub fn branch(mut self, branch: &str) -> Self {
        self.branch = branch.to_string();
        self
    }

    pub fn classified(mut self, classified: ClassifiedChanges) -> Self {
        self.classified = classified;
        self
    }

    pub fn plan(mut self, plan: AutopilotPlan) -> Self {
        self.plan = plan;
        self
    }

    pub fn applied(mut self, applied: bool) -> Self {
        self.applied = applied;
        self
    }

    pub fn build(self) -> AutopilotReport {
        AutopilotReport {
            mode: self.mode,
            branch: self.branch,
            detached_head: self.detached_head,
            changes: self.changes,
            classified: self.classified,
            plan: self.plan,
            applied: self.applied,
            logs: self.logs,
        }
    }
}

impl Default for AutopilotReportBuilder {
    fn default() -> Self {
        Self::new()
    }
}

/// Assert that a response has an error with the expected code
pub fn assert_error_code(response: &protocol::CommandResponse, expected_code: i32) {
    assert!(
        response.error.is_some(),
        "Expected error but got none. Response: {:?}",
        response
    );
    let error = response.error.as_ref().unwrap();
    assert_eq!(
        error.code, expected_code,
        "Expected error code {} but got {}. Message: {}",
        expected_code, error.code, error.message
    );
}

/// Assert that a response has a specific HTTP status code
pub fn assert_status_code(response: &protocol::CommandResponse, expected_code: u16) {
    assert_eq!(
        response.status.code, expected_code,
        "Expected status code {} but got {}. Response: {:?}",
        expected_code, response.status.code, response
    );
}

/// Assert that an error message contains a specific substring
pub fn assert_error_contains(response: &protocol::CommandResponse, expected_substring: &str) {
    assert!(
        response.error.is_some(),
        "Expected error but got none"
    );
    let error = response.error.as_ref().unwrap();
    assert!(
        error.message.contains(expected_substring),
        "Expected error message to contain '{}' but got: '{}'",
        expected_substring, error.message
    );
}
