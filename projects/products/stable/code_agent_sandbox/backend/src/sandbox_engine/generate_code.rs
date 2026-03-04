// projects/products/code_agent_sandbox/src/engine/generate_code.rs
use crate::{
    actions::{ActionResult, LowLevelActionContext},
    normalization::normalize_extension,
};
use common_json::pjson;
use protocol::ProtocolId;

pub(crate) fn handle_generate_code(
    language: &str,
    code: &str,
    ctx: &LowLevelActionContext,
) -> Result<ActionResult, anyhow::Error> {
    let ai_ws = ctx.run_dir.join("ai_workspace");
    std::fs::create_dir_all(&ai_ws)?;

    if code.len() > ctx.config.max_write_bytes {
        return Err(anyhow::anyhow!(
            "Generated code exceeds max_write_bytes limit"
        ));
    }

    if !ai_ws.starts_with(ctx.run_dir) {
        return Err(anyhow::anyhow!("ai_workspace outside run_dir"));
    }

    let ext = normalize_extension(language);
    let file_path = ai_ws.join(format!("generated_{}.{}", ProtocolId::default(), ext));

    if !file_path.starts_with(ctx.run_dir) {
        return Err(anyhow::anyhow!("Attempted to write outside of run_dir"));
    }

    std::fs::write(&file_path, code)?;

    Ok(ActionResult::success(
        "CodeGenerated",
        "saved",
        Some(pjson!({
            "path": (file_path.to_string_lossy().to_string()),
            "language": (language.to_string()),
            "bytes": (code.len() as i64)
        })),
    ))
}
