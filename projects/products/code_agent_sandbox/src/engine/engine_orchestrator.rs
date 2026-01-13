// projects/products/code_agent_sandbox/src/engine/engine_orchestrator.rs
use anyhow::{Result, anyhow};
use common_time::SystemClock;
use common_time::timeout::with_timeout;

use crate::engine::{
    EngineConfig, EngineInit, EnginePaths, Request, Response, initialize_engine, request,
};

/// ✅ Seul point d’entrée officiel “hors domaine”.
pub async fn execute_request(
    req: Request,
    paths: &EnginePaths,
    config: EngineConfig,
) -> Result<Response> {
    let workspace_mode = req.workspace_mode.clone();

    let mut init: EngineInit =
        initialize_engine(paths, &config, req.run_id.as_deref(), workspace_mode)?;

    // Domaine: exécution core
    let clock = SystemClock;
    match with_timeout(
        request::execute_with_init(req, &mut init, paths, &config),
        &clock,
        config.timeout,
    )
    .await
    {
        Ok(response) => response,
        Err(_) => Err(anyhow!("Timeout exceeded while executing request")),
    }
}
