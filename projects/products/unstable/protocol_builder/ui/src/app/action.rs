// projects/products/unstable/protocol_builder/ui/src/app/action.rs

#[derive(Debug, Clone)]
pub enum Action {
    SetSchemaPath(String),
    SetOutDir(String),
    SetReport {
        manifest_hash: String,
        report_json: String,
    },
    SetError(String),
    ClearError,
}
