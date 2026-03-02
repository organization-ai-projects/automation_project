#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Mode {
    Auto,
    Strict,
    Relaxed,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Command {
    Check { root: String, mode: Mode },
    CheckProduct { path: String, mode: Mode },
}
