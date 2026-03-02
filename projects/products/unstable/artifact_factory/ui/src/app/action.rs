#[derive(Debug, Clone)]
pub enum Action {
    LoadInputs(Vec<String>),
    Analyze,
    RenderDocs,
    BuildBundle,
    GetBundle,
    Quit,
}
