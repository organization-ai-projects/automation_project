// projects/products/code_agent_sandbox/src/engine/engine_orchestrator.rs
use anyhow::Result;

use crate::engine::{EngineConfig, EngineInit, EnginePaths, Request, Response, initialize_engine, request};

/// ✅ Seul point d’entrée officiel “hors domaine”.
pub fn execute_request(
    req: Request,
    paths: &EnginePaths,
    config: EngineConfig,
) -> Result<Response> {
    let workspace_mode = req.workspace_mode.clone();

    let mut init: EngineInit =
        initialize_engine(paths, &config, req.run_id.as_deref(), workspace_mode)?;

    // Domaine: exécution core
    request::execute_with_init(req, &mut init, paths, &config)
}
