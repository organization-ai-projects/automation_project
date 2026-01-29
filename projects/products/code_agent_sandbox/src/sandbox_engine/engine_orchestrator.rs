// projects/products/code_agent_sandbox/src/engine/engine_orchestrator.rs
use anyhow::{Context, Result, anyhow};
use common_time::{SystemClock, timeout::with_timeout};

use crate::sandbox_engine::{
    EngineConfig, EngineInit, EnginePaths, Request, Response, initialize_engine, request,
};

/// The only official "out-of-domain" entry point.
pub(crate) async fn execute_request(
    req: Request,
    paths: &EnginePaths,
    config: EngineConfig,
) -> Result<Response> {
    let workspace_mode = req.workspace_mode;

    let mut init: EngineInit =
        initialize_engine(paths, &config, req.run_id.as_deref(), workspace_mode)
            .context("initialize_engine failed")?;

    let clock = SystemClock;

    with_timeout(
        async {
            let result = request::execute_with_init(req, &mut init, paths, &config).await;
            result.context("execute_with_init failed")
        },
        &clock,
        config.timeout,
    )
    .await
    .map_err(|_| anyhow!("Timeout exceeded while executing request"))?
}
