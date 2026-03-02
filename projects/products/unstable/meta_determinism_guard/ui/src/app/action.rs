#[derive(Debug, Clone)]
pub enum Action {
    Scan { root: String, json: bool },
    Stability { cmd: String, runs: u32, json: bool },
    Report { json: bool },
}
