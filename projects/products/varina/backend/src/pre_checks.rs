/// Niveau de checks avant commit/push.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PreChecks {
    None,
    FmtCheck,
    FmtCheckAndTests,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pre_checks_usage() {
        let check = PreChecks::None;
        assert_eq!(check, PreChecks::None);

        let check = PreChecks::FmtCheck;
        assert_eq!(check, PreChecks::FmtCheck);

        let check = PreChecks::FmtCheckAndTests;
        assert_eq!(check, PreChecks::FmtCheckAndTests);
    }
}
