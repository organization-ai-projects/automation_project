#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub enum Screen {
    #[default]
    Input,
    Graph,
    Render,
    Bundle,
}
