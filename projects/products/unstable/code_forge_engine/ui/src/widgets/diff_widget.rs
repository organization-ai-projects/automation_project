// projects/products/unstable/code_forge_engine/ui/src/widgets/diff_widget.rs
pub struct DiffWidget {
    pub before: String,
    pub after: String,
}

impl DiffWidget {
    pub fn new(before: impl Into<String>, after: impl Into<String>) -> Self {
        Self {
            before: before.into(),
            after: after.into(),
        }
    }

    pub fn has_changes(&self) -> bool {
        self.before != self.after
    }
}
