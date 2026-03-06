use crate::constraints::constraint::Constraint;
use crate::constraints::constraint_engine::ConstraintEngine;
use crate::determinism::deterministic_order::DeterministicOrder;
use crate::determinism::seed::Seed;
use crate::diagnostics::backend_error::BackendError;
use crate::dsl::parser::Parser;
use crate::events::event_log::EventLog;
use crate::events::model_event::ModelEvent;
use crate::io::canonical_json::to_canonical_string;
use crate::model::state::State;
use crate::protocol::request::Request;
use crate::protocol::request_payload::RequestPayload;
use crate::protocol::response::Response;
use crate::protocol::response_payload::ResponsePayload;
use crate::replay::replay_engine::ReplayEngine;
use crate::report::run_hash::RunHash;
use crate::report::run_report::RunReport;
use crate::snapshots::snapshot::Snapshot;
use crate::snapshots::snapshot_hash::SnapshotHash;
use crate::solve::solver::Solver;

#[derive(Debug, Default)]
pub struct BackendSession {
    pub model_source: Option<String>,
    pub state: State,
    pub constraints: Vec<Constraint>,
    pub event_log: EventLog,
    pub last_report: Option<RunReport>,
    pub replay_data: Option<String>,
    pub seed: Seed,
    pub step_count: u64,
    pub shutdown: bool,
}

impl BackendSession {
    pub fn should_shutdown(&self) -> bool {
        self.shutdown
    }

    pub fn handle(&mut self, request: Request) -> Response {
        let id = request.id;
        let payload = match self.handle_payload(request.payload) {
            Ok(payload) => payload,
            Err(error) => ResponsePayload::Error {
                message: error.to_string(),
            },
        };
        Response { id, payload }
    }

    fn handle_payload(&mut self, payload: RequestPayload) -> Result<ResponsePayload, BackendError> {
        match payload {
            RequestPayload::LoadModel { model } => {
                self.model_source = Some(model);
                self.state = State::default();
                self.constraints.clear();
                self.event_log = EventLog::default();
                self.step_count = 0;
                Ok(ResponsePayload::Ok)
            }
            RequestPayload::ValidateModel => {
                let source = self
                    .model_source
                    .as_deref()
                    .ok_or_else(|| BackendError::Validation("model not loaded".to_string()))?;
                let ast = Parser::parse(source)?;
                self.state = State::from_vars(&ast.vars);
                self.constraints = ast.constraints;
                Ok(ResponsePayload::Ok)
            }
            RequestPayload::NewRun { seed } => {
                self.seed = Seed::new(seed);
                self.reinitialize_state_from_model()?;
                self.event_log = EventLog::default();
                self.last_report = None;
                self.step_count = 0;
                Ok(ResponsePayload::Ok)
            }
            RequestPayload::Step => {
                self.apply_step()?;
                Ok(ResponsePayload::Ok)
            }
            RequestPayload::RunToEnd => {
                for step_index in 0..8_u64 {
                    self.apply_step()?;
                    if self.step_count < step_index + 1 {
                        return Err(BackendError::Engine(
                            "step counter did not advance monotonically".to_string(),
                        ));
                    }
                }
                self.last_report = Some(RunReport::from_state(
                    self.seed.value,
                    self.step_count,
                    &self.state,
                    &self.event_log,
                ));
                Ok(ResponsePayload::Ok)
            }
            RequestPayload::GetSnapshot => {
                let snapshot = Snapshot::from_state(self.step_count, &self.state, &self.event_log);
                let hash = SnapshotHash::compute(&snapshot)?;
                let state_json = to_canonical_string(&snapshot).map_err(BackendError::Codec)?;
                Ok(ResponsePayload::Snapshot { hash, state_json })
            }
            RequestPayload::GetReport => {
                let report = self.last_report.clone().unwrap_or_else(|| {
                    RunReport::from_state(
                        self.seed.value,
                        self.step_count,
                        &self.state,
                        &self.event_log,
                    )
                });
                let report_json = to_canonical_string(&report).map_err(BackendError::Codec)?;
                let run_hash = RunHash::compute(&report)?;
                self.last_report = Some(report);
                Ok(ResponsePayload::Report {
                    run_hash,
                    report_json,
                })
            }
            RequestPayload::SaveReplay => {
                let replay = ReplayEngine::build_replay(
                    self.seed.value,
                    self.step_count,
                    &self.event_log,
                    &self.state,
                )?;
                self.replay_data = Some(replay);
                Ok(ResponsePayload::Ok)
            }
            RequestPayload::GetReplay => {
                let replay = self
                    .replay_data
                    .clone()
                    .ok_or_else(|| BackendError::Replay("replay not available".to_string()))?;
                Ok(ResponsePayload::ReplayData { replay })
            }
            RequestPayload::LoadReplay { replay } => {
                self.replay_data = Some(replay);
                Ok(ResponsePayload::Ok)
            }
            RequestPayload::ReplayToEnd => {
                let replay = self
                    .replay_data
                    .as_deref()
                    .ok_or_else(|| BackendError::Replay("replay not loaded".to_string()))?;
                let (state, event_log, step_count) = ReplayEngine::replay_to_end(replay)?;
                self.state = state;
                self.event_log = event_log;
                self.step_count = step_count;
                self.last_report = Some(RunReport::from_state(
                    self.seed.value,
                    self.step_count,
                    &self.state,
                    &self.event_log,
                ));
                Ok(ResponsePayload::Ok)
            }
            RequestPayload::Shutdown => {
                self.shutdown = true;
                Ok(ResponsePayload::Ok)
            }
        }
    }

    fn reinitialize_state_from_model(&mut self) -> Result<(), BackendError> {
        let source = self
            .model_source
            .as_deref()
            .ok_or_else(|| BackendError::Validation("model not loaded".to_string()))?;
        let ast = Parser::parse(source)?;
        self.state = State::from_vars(&ast.vars);
        self.constraints = ast.constraints;
        Ok(())
    }

    fn apply_step(&mut self) -> Result<(), BackendError> {
        let transition = Solver::next_transition(self.step_count, self.seed.value, &self.state)?;
        let value_after = ConstraintEngine::apply(&mut self.state, &transition, &self.constraints)?;

        self.step_count += 1;
        let labels = DeterministicOrder::order(vec![
            format!("transition:{}", transition.transition_label()),
            format!("tick:{}", self.step_count),
            format!("var:{}", transition.target_var.0),
        ]);

        let event = ModelEvent::from_transition(self.step_count, &transition, value_after, labels);
        self.event_log.push(event);
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::BackendSession;
    use crate::protocol::request::Request;
    use crate::protocol::request_payload::RequestPayload;
    use crate::protocol::response_payload::ResponsePayload;

    fn request(payload: RequestPayload) -> Request {
        Request {
            id: Some("test".to_string()),
            payload,
        }
    }

    #[test]
    fn deterministic_run_produces_same_hash() {
        let model = "\nvar a int 1\nvar b int 2\nconstraint c1 a min -10\n".to_string();

        let mut session_1 = BackendSession::default();
        let mut session_2 = BackendSession::default();

        session_1.handle(request(RequestPayload::LoadModel {
            model: model.clone(),
        }));
        session_1.handle(request(RequestPayload::ValidateModel));
        session_1.handle(request(RequestPayload::NewRun { seed: 42 }));
        session_1.handle(request(RequestPayload::RunToEnd));

        session_2.handle(request(RequestPayload::LoadModel { model }));
        session_2.handle(request(RequestPayload::ValidateModel));
        session_2.handle(request(RequestPayload::NewRun { seed: 42 }));
        session_2.handle(request(RequestPayload::RunToEnd));

        let report_1 = session_1.handle(request(RequestPayload::GetReport));
        let report_2 = session_2.handle(request(RequestPayload::GetReport));

        let extracted_1 = extract_report(report_1.payload);
        let extracted_2 = extract_report(report_2.payload);
        assert!(extracted_1.is_some());
        assert!(extracted_2.is_some());
        let (hash_1, json_1) = extracted_1.expect("report should be present");
        let (hash_2, json_2) = extracted_2.expect("report should be present");

        assert_eq!(json_1, json_2);
        assert_eq!(hash_1, hash_2);
    }

    #[test]
    fn replay_roundtrip_preserves_report_hash() {
        let model = "\nvar x int 0\nvar y int 5\nconstraint c1 y min -100\n".to_string();
        let mut session = BackendSession::default();

        session.handle(request(RequestPayload::LoadModel { model }));
        session.handle(request(RequestPayload::ValidateModel));
        session.handle(request(RequestPayload::NewRun { seed: 5 }));
        session.handle(request(RequestPayload::RunToEnd));

        let original_report = session.handle(request(RequestPayload::GetReport));
        let extracted_original = extract_report(original_report.payload);
        assert!(extracted_original.is_some());
        let (original_hash, original_json) = extracted_original.expect("report should be present");

        session.handle(request(RequestPayload::SaveReplay));
        let replay_data = session
            .replay_data
            .clone()
            .expect("replay data should exist");
        session.handle(request(RequestPayload::LoadReplay {
            replay: replay_data,
        }));
        session.handle(request(RequestPayload::ReplayToEnd));

        let replay_report = session.handle(request(RequestPayload::GetReport));
        let extracted_replay = extract_report(replay_report.payload);
        assert!(extracted_replay.is_some());
        let (replay_hash, replay_json) = extracted_replay.expect("report should be present");

        assert_eq!(original_json, replay_json);
        assert_eq!(original_hash, replay_hash);
    }

    fn extract_report(payload: ResponsePayload) -> Option<(String, String)> {
        if let ResponsePayload::Report {
            run_hash,
            report_json,
        } = payload
        {
            Some((run_hash, report_json))
        } else {
            None
        }
    }
}
