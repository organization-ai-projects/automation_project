use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum VerifyKind {
    Fmt,
    Clippy,
    Test,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct VerifyStep {
    pub kind: VerifyKind,
    pub command: String,
    pub args: Vec<String>,
}

impl VerifyStep {
    pub fn new(kind: VerifyKind, command: impl Into<String>, args: Vec<String>) -> Self {
        Self {
            kind,
            command: command.into(),
            args,
        }
    }

    pub fn fmt() -> Self {
        Self::new(
            VerifyKind::Fmt,
            "cargo",
            vec!["fmt".to_string(), "--check".to_string()],
        )
    }

    pub fn clippy() -> Self {
        Self::new(VerifyKind::Clippy, "cargo", vec!["clippy".to_string()])
    }

    pub fn test() -> Self {
        Self::new(VerifyKind::Test, "cargo", vec!["test".to_string()])
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fmt_step_has_correct_args() {
        let step = VerifyStep::fmt();
        assert_eq!(step.command, "cargo");
        assert_eq!(step.args, vec!["fmt", "--check"]);
        assert_eq!(step.kind, VerifyKind::Fmt);
    }

    #[test]
    fn clippy_step_has_correct_args() {
        let step = VerifyStep::clippy();
        assert_eq!(step.args, vec!["clippy"]);
    }

    #[test]
    fn serializes_roundtrip() {
        let step = VerifyStep::test();
        let json = serde_json::to_string(&step).unwrap();
        let restored: VerifyStep = serde_json::from_str(&json).unwrap();
        assert_eq!(step, restored);
    }
}
