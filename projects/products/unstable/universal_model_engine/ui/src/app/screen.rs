#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub enum Screen {
    #[default]
    Dsl,
    Run,
    Inspect,
    Replay,
    Report,
}
