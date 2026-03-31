use crate::diagnostics::error::BackendError;
use crate::dsl::parser::Parser;
use crate::execute::runner::Runner;
use crate::model::event_id::EventId;
use crate::model::machine::Machine;
use crate::model::state_id::StateId;
use crate::protocol::request::Request;
use crate::protocol::request_payload::RequestPayload;
use crate::protocol::response::Response;
use crate::protocol::response_payload::ResponsePayload;
use crate::replay::replay_engine::ReplayEngine;
use crate::replay::transcript::Transcript;
use crate::replay::transcript_codec::TranscriptCodec;
use crate::testing::exhaustive_tester::ExhaustiveTester;
use crate::testing::fuzz_tester::FuzzTester;
use crate::verify::verifier::Verifier;
use std::collections::BTreeMap;

#[derive(Debug, Default)]
pub struct BackendSession {
    pub machine: Option<Machine>,
    pub runner: Option<Runner>,
    pub last_transcript: Option<Transcript>,
    pub shutdown: bool,
}

impl BackendSession {
    pub fn should_shutdown(&self) -> bool {
        self.shutdown
    }

    pub fn handle(&mut self, request: Request) -> Response {
        let id = request.id;
        let payload = match self.handle_payload(request.payload) {
            Ok(p) => p,
            Err(e) => ResponsePayload::Error {
                message: e.to_string(),
            },
        };
        Response { id, payload }
    }

    fn handle_payload(&mut self, payload: RequestPayload) -> Result<ResponsePayload, BackendError> {
        match payload {
            RequestPayload::LoadMachine { machine: source } => {
                let ast = Parser::parse(&source)?;
                let machine = Self::ast_to_machine(&ast);
                self.machine = Some(machine);
                self.runner = None;
                self.last_transcript = None;
                Ok(ResponsePayload::Ok)
            }
            RequestPayload::Validate => {
                let machine = self.require_machine()?;
                let issues = Verifier::validate_machine(machine)?;
                if issues.is_empty() {
                    Ok(ResponsePayload::Ok)
                } else {
                    Ok(ResponsePayload::Error {
                        message: issues.join("; "),
                    })
                }
            }
            RequestPayload::Run { events } => {
                let machine = self.require_machine()?.clone();
                let event_ids: Vec<EventId> = events.iter().map(|e| EventId(e.clone())).collect();
                let transcript = ReplayEngine::build_transcript(&machine, &event_ids, None)?;
                self.last_transcript = Some(transcript);
                Ok(ResponsePayload::Ok)
            }
            RequestPayload::Step { event } => {
                let machine = self.require_machine()?.clone();
                if self.runner.is_none() {
                    self.runner = Some(Runner::new(machine));
                }
                let runner = self.runner.as_mut().unwrap();
                runner.step(&EventId(event))?;
                Ok(ResponsePayload::Ok)
            }
            RequestPayload::TestExhaustive => {
                let machine = self.require_machine()?.clone();
                let report = ExhaustiveTester::test(&machine)?;
                let report_json = common_json::to_string(&report)
                    .map_err(|e| BackendError::Codec(e.to_string()))?;
                Ok(ResponsePayload::TestReport { report_json })
            }
            RequestPayload::TestFuzz { seed, steps } => {
                let machine = self.require_machine()?.clone();
                let report = FuzzTester::test(&machine, seed, steps)?;
                let report_json = common_json::to_string(&report)
                    .map_err(|e| BackendError::Codec(e.to_string()))?;
                Ok(ResponsePayload::TestReport { report_json })
            }
            RequestPayload::GetTranscript => {
                let transcript = self
                    .last_transcript
                    .as_ref()
                    .ok_or_else(|| BackendError::Engine("no transcript available".to_string()))?;
                let transcript_json = TranscriptCodec::encode(transcript)?;
                Ok(ResponsePayload::Transcript { transcript_json })
            }
            RequestPayload::Shutdown => {
                self.shutdown = true;
                Ok(ResponsePayload::Ok)
            }
        }
    }

    fn require_machine(&self) -> Result<&Machine, BackendError> {
        self.machine
            .as_ref()
            .ok_or_else(|| BackendError::Engine("no machine loaded".to_string()))
    }

    fn ast_to_machine(ast: &crate::dsl::ast::Ast) -> Machine {
        let mut transitions_map: BTreeMap<String, Vec<crate::model::machine::Transition>> =
            BTreeMap::new();
        for (from, t) in &ast.transitions {
            let key = Machine::transition_key(from, &t.event);
            transitions_map.entry(key).or_default().push(t.clone());
        }
        Machine {
            id: ast.machine_id.clone().unwrap_or_else(|| crate::model::machine_id::MachineId("unnamed".to_string())),
            initial_state: ast.initial_state.clone().unwrap_or_else(|| StateId("initial".to_string())),
            states: ast.states.clone(),
            events: ast.events.clone(),
            transitions: transitions_map,
            variables: ast.variables.clone(),
        }
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

    const TOGGLE_MACHINE: &str = "\
machine toggle
state off
state on
event flip
initial off
transition off flip -> on
transition on flip -> off
";

    #[test]
    fn load_and_validate() {
        let mut session = BackendSession::default();
        let r = session.handle(request(RequestPayload::LoadMachine {
            machine: TOGGLE_MACHINE.to_string(),
        }));
        assert!(matches!(r.payload, ResponsePayload::Ok));
        let r = session.handle(request(RequestPayload::Validate));
        assert!(matches!(r.payload, ResponsePayload::Ok));
    }

    #[test]
    fn run_and_get_transcript() {
        let mut session = BackendSession::default();
        session.handle(request(RequestPayload::LoadMachine {
            machine: TOGGLE_MACHINE.to_string(),
        }));
        session.handle(request(RequestPayload::Run {
            events: vec!["flip".to_string(), "flip".to_string()],
        }));
        let r = session.handle(request(RequestPayload::GetTranscript));
        match r.payload {
            ResponsePayload::Transcript { transcript_json } => {
                assert!(!transcript_json.is_empty());
            }
            other => panic!("expected Transcript, got {other:?}"),
        }
    }

    #[test]
    fn deterministic_transcript() {
        let mut s1 = BackendSession::default();
        let mut s2 = BackendSession::default();
        let events = vec!["flip".to_string(), "flip".to_string(), "flip".to_string()];

        s1.handle(request(RequestPayload::LoadMachine { machine: TOGGLE_MACHINE.to_string() }));
        s2.handle(request(RequestPayload::LoadMachine { machine: TOGGLE_MACHINE.to_string() }));
        s1.handle(request(RequestPayload::Run { events: events.clone() }));
        s2.handle(request(RequestPayload::Run { events }));

        let t1 = s1.handle(request(RequestPayload::GetTranscript));
        let t2 = s2.handle(request(RequestPayload::GetTranscript));

        let json1 = extract_transcript(t1.payload);
        let json2 = extract_transcript(t2.payload);
        assert_eq!(json1, json2);
    }

    #[test]
    fn exhaustive_test_report() {
        let mut session = BackendSession::default();
        session.handle(request(RequestPayload::LoadMachine {
            machine: TOGGLE_MACHINE.to_string(),
        }));
        let r = session.handle(request(RequestPayload::TestExhaustive));
        match r.payload {
            ResponsePayload::TestReport { report_json } => {
                assert!(report_json.contains("exhaustive"));
            }
            other => panic!("expected TestReport, got {other:?}"),
        }
    }

    #[test]
    fn fuzz_test_deterministic() {
        let mut s1 = BackendSession::default();
        let mut s2 = BackendSession::default();
        s1.handle(request(RequestPayload::LoadMachine { machine: TOGGLE_MACHINE.to_string() }));
        s2.handle(request(RequestPayload::LoadMachine { machine: TOGGLE_MACHINE.to_string() }));
        let r1 = s1.handle(request(RequestPayload::TestFuzz { seed: 42, steps: 50 }));
        let r2 = s2.handle(request(RequestPayload::TestFuzz { seed: 42, steps: 50 }));
        let j1 = extract_test_report(r1.payload);
        let j2 = extract_test_report(r2.payload);
        let v1: crate::testing::test_report::TestReport = common_json::from_json_str(&j1).unwrap();
        let v2: crate::testing::test_report::TestReport = common_json::from_json_str(&j2).unwrap();
        assert_eq!(v1, v2);
    }

    fn extract_transcript(payload: ResponsePayload) -> String {
        match payload {
            ResponsePayload::Transcript { transcript_json } => transcript_json,
            other => panic!("expected Transcript, got {other:?}"),
        }
    }

    fn extract_test_report(payload: ResponsePayload) -> String {
        match payload {
            ResponsePayload::TestReport { report_json } => report_json,
            other => panic!("expected TestReport, got {other:?}"),
        }
    }
}
