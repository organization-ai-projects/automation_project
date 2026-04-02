#[derive(Debug, Clone)]
pub enum Action {
    LoadMachine(String),
    Validate,
    Run(Vec<String>),
    Step(String),
    TestExhaustive,
    TestFuzz { seed: u64, steps: u64 },
    GetTranscript,
    Quit,
}
