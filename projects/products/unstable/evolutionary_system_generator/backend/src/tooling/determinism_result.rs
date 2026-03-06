// projects/products/unstable/evolutionary_system_generator/backend/src/tooling/determinism_result.rs
#[derive(Debug)]
pub struct DeterminismResult {
    pub determinism_ok: bool,
    pub manifest_hash: Option<String>,
}
