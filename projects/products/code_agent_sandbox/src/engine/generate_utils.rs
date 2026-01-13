// projects/products/code_agent_sandbox/src/engine/generate_utils.rs
use crate::engine::{Rights, PATH_RIGHTS};

pub fn generate_globs(mask: Rights) -> Vec<String> {
    PATH_RIGHTS
        .iter()
        .filter(|r| (r.rights & mask) != 0)
        .map(|r| r.path.into())
        .collect()
}
