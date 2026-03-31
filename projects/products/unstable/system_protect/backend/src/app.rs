use crate::diagnostics::error::Error;
use crate::io::json_codec::JsonCodec;
use crate::moe_protect::moe_engine::MoeEngine;
use crate::protocol::request::Request;
use crate::protocol::request_payload::RequestPayload;
use crate::protocol::response::Response;
use crate::protocol::response_payload::ResponsePayload;

pub fn run() -> Result<(), Error> {
    let mut engine = MoeEngine::new();
    engine.register_default_experts();

    let stdin = std::io::stdin();
    let stdout = std::io::stdout();

    loop {
        let request: Request = match JsonCodec::read_request(&stdin) {
            Ok(req) => req,
            Err(Error::EndOfInput) => break,
            Err(e) => {
                let resp = Response::new(ResponsePayload::Error {
                    message: e.to_string(),
                });
                JsonCodec::write_response(&stdout, &resp)?;
                continue;
            }
        };

        let payload = match request.payload {
            RequestPayload::AnalyzeThreat { threat_event } => {
                match engine.analyze(threat_event) {
                    Ok(result) => ResponsePayload::ProtectionResult { result },
                    Err(e) => ResponsePayload::Error {
                        message: e.to_string(),
                    },
                }
            }
            RequestPayload::ListExperts => {
                let experts = engine.list_experts();
                ResponsePayload::ExpertList { experts }
            }
            RequestPayload::GetStatus => {
                let status = engine.status();
                ResponsePayload::Status { status }
            }
            RequestPayload::AddFirewallRule { rule } => {
                engine.add_firewall_rule(rule);
                ResponsePayload::Ok
            }
            RequestPayload::AddSignature { signature } => {
                engine.add_signature(signature);
                ResponsePayload::Ok
            }
            RequestPayload::Shutdown => {
                let resp = Response::new(ResponsePayload::Ok);
                JsonCodec::write_response(&stdout, &resp)?;
                break;
            }
        };

        let resp = Response::new(payload);
        JsonCodec::write_response(&stdout, &resp)?;
    }

    Ok(())
}
