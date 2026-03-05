// projects/products/unstable/hospital_tycoon/ui/src/transport/outbound_request.rs
use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
#[serde(tag = "type")]
pub enum OutboundRequest {
    NewRun { seed: u64, ticks: u64 },
    RunToEnd,
    GetReport,
    SaveReplay { path: String },
    LoadReplay { path: String },
    ReplayToEnd,
}
