#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub enum Screen {
    #[default]
    Contract,
    Preview,
    Generate,
    Report,
}
