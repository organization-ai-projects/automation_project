// projects/products/unstable/spec_to_runtime_compiler/ui/src/app/action.rs

#[derive(Debug, Clone)]
pub enum Action {
    LoadSpec { path: String },
    Compile,
    DryRun,
    GetReport,
    SetError(String),
    SetReport(String),
}
