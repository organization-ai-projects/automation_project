#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub enum Screen {
    #[default]
    Editor,
    Run,
    Test,
    Transcript,
}
