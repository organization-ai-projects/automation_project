/// The origin/source of an AST node.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Origin {
    Parser(&'static str),
    ProcMacro(&'static str),
    Ai(&'static str),
    Tool(&'static str),
}
