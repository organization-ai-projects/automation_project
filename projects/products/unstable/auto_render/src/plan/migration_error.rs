use thiserror::Error;
use serde::{Deserialize, Serialize};

#[derive(Error, Debug, Clone, Serialize, Deserialize)]
pub enum MigrationError {
    #[error("Requires migration from {from} to {to}")]
    RequiresMigration { from: String, to: String },
    #[error("Incompatible schema versions")]
    Incompatible,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn migration_error_display() {
        let e = MigrationError::RequiresMigration {
            from: "1.0.0".to_string(),
            to: "2.0.0".to_string(),
        };
        assert!(e.to_string().contains("1.0.0"));
        assert!(e.to_string().contains("2.0.0"));
    }

    #[test]
    fn migration_error_incompatible_display() {
        let e = MigrationError::Incompatible;
        assert!(e.to_string().contains("Incompatible"));
    }

    #[test]
    fn migration_error_ron_roundtrip() {
        let e = MigrationError::RequiresMigration {
            from: "1.0.0".to_string(),
            to: "2.0.0".to_string(),
        };
        let s = ron::to_string(&e).expect("serialize");
        let d: MigrationError = ron::from_str(&s).expect("deserialize");
        assert!(d.to_string().contains("1.0.0"));
    }
}
