// projects/products/code_agent_sandbox/src/logging/events.rs
use std::path::Path;

use common_json::Json;
use common_time::SystemClock;

use crate::{
    engine::EngineCtx,
    memory::{MemoryEvent, append_event},
};

pub fn log_event(
    mem_path: &Path,
    ctx: &EngineCtx,
    kind: &str,
    input: Option<String>,
    output: Option<String>,
    metadata: Option<Json>,
) {
    let event = MemoryEvent {
        ts: SystemClock::now_rfc3339(),
        run_id: ctx.run_id.clone(),
        kind: kind.into(),
        input,
        output,
        metadata,
    };
    if let Err(e) = append_event(mem_path, &event) {
        tracing::warn!("append_event failed: {:#}", e);
    }
}
