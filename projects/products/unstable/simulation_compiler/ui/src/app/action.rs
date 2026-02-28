// projects/products/unstable/simulation_compiler/ui/src/app/action.rs

#[derive(Debug, Clone)]
pub enum Action {
    LoadDsl { path: String },
    Compile,
    DryRun,
    GetReport,
    SetError(String),
    SetReport(String),
}
