/// A single changed line in a local diff.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DiffLine {
    /// A line present only in the original.
    Removed(String),
    /// A line present only in the modified content.
    Added(String),
    /// A line unchanged between original and modified.
    Context(String),
}
