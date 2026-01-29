// projects/products/code_agent_sandbox/src/engine/generate_utils.rs
use crate::sandbox_engine::{PATH_RIGHTS, Rights};

pub(crate) fn generate_globs(mask: Rights) -> Vec<String> {
    PATH_RIGHTS
        .iter()
        .filter(|r| (r.rights & mask) != 0)
        .map(|r| r.path.into())
        .collect()
}
