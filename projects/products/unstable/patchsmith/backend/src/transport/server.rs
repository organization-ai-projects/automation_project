use std::io::{BufRead, Write};

use super::message::Message;
use super::payload::Payload;
use super::request::Request;
use super::response::Response;
use super::server_state::ServerState;
use crate::apply::applier::Applier;
use crate::dsl::dsl_parser::DslParser;
use crate::plan::plan_builder::PlanBuilder;
use crate::report::patch_report::PatchReport;
use crate::verify::verifier::Verifier;

fn handle_request(request: Request, state: &mut ServerState) -> Response {
    match request {
        Request::PlanDsl { dsl } => match DslParser::parse(&dsl) {
            Ok(ops) => match PlanBuilder::build(ops) {
                Ok(plan) => {
                    let json = common_json::to_string(&plan).unwrap_or_default();
                    state.current_plan = Some(plan);
                    Response::PlanReport { plan_json: json }
                }
                Err(e) => Response::Error {
                    message: e.to_string(),
                },
            },
            Err(e) => Response::Error {
                message: e.to_string(),
            },
        },
        Request::ApplyPlan { plan_json } => {
            match common_json::from_json_str::<crate::plan::patch_plan::PatchPlan>(&plan_json) {
                Ok(plan) => {
                    let json = common_json::to_string(&plan).unwrap_or_default();
                    state.current_plan = Some(plan);
                    Response::ApplyReport { report_json: json }
                }
                Err(e) => Response::Error {
                    message: format!("plan decode error: {e}"),
                },
            }
        }
        Request::VerifyPlan {
            plan_json,
            file_contents,
        } => match common_json::from_json_str::<crate::plan::patch_plan::PatchPlan>(&plan_json) {
            Ok(plan) => match Applier::apply(&plan, &file_contents) {
                Ok(result) => {
                    let verify = Verifier::verify(&plan, &result);
                    let report = PatchReport::from_verify(&verify, plan.ops.len());
                    let json = common_json::to_string(&report).unwrap_or_default();
                    Response::VerifyReport { report_json: json }
                }
                Err(e) => Response::Error {
                    message: e.to_string(),
                },
            },
            Err(e) => Response::Error {
                message: format!("plan decode error: {e}"),
            },
        },
        Request::Shutdown => Response::Ok,
    }
}

pub fn run() -> Result<(), Box<dyn std::error::Error>> {
    let stdin = std::io::stdin();
    let stdout = std::io::stdout();
    let mut stdout = stdout.lock();
    let mut state = ServerState { current_plan: None };

    for line in stdin.lock().lines() {
        let line = line?;
        let line = line.trim();
        if line.is_empty() {
            continue;
        }
        let mut should_shutdown = false;
        let response_message = match common_json::from_json_str::<Message>(line) {
            Ok(request_message) => {
                let response_payload = match request_message.payload {
                    Payload::Request(req) => {
                        if matches!(req, Request::Shutdown) {
                            should_shutdown = true;
                        }
                        handle_request(req, &mut state)
                    }
                    Payload::Response(_) => Response::Error {
                        message: "unexpected response payload from UI".to_string(),
                    },
                };
                Message {
                    id: request_message.id,
                    payload: Payload::Response(response_payload),
                }
            }
            Err(err) => Message {
                id: 0,
                payload: Payload::Response(Response::Error {
                    message: err.to_string(),
                }),
            },
        };
        let encoded = common_json::to_string(&response_message)?;
        writeln!(stdout, "{encoded}")?;
        stdout.flush()?;
        if should_shutdown {
            break;
        }
    }
    Ok(())
}
