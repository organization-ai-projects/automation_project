#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NamingRules;

impl NamingRules {
    pub fn evaluate(
        product_dir: &std::path::Path,
        scope: crate::config::path_classification::PathClassification,
        mode: crate::config::enforcement_mode::EnforcementMode,
    ) -> Vec<crate::report::violation::Violation> {
        let mut out = Vec::new();
        let Some(dir_name) = product_dir.file_name().and_then(|n| n.to_str()) else {
            out.push(make_violation(
                crate::rules::rule_id::RuleId::Naming,
                crate::report::violation_code::ViolationCode::NameProductMismatch,
                scope,
                product_dir,
                "unable to resolve product directory name",
                mode,
                true,
                None,
            ));
            return out;
        };

        if dir_name.is_empty() {
            out.push(make_violation(
                crate::rules::rule_id::RuleId::Naming,
                crate::report::violation_code::ViolationCode::NameProductMismatch,
                scope,
                product_dir,
                "product directory name must not be empty",
                mode,
                true,
                None,
            ));
        }

        out
    }
}

fn make_violation(
    rule_id: crate::rules::rule_id::RuleId,
    code: crate::report::violation_code::ViolationCode,
    scope: crate::config::path_classification::PathClassification,
    path: &std::path::Path,
    message: &str,
    mode: crate::config::enforcement_mode::EnforcementMode,
    default_blocking: bool,
    line: Option<u32>,
) -> crate::report::violation::Violation {
    let mut severity = if default_blocking {
        crate::config::severity::Severity::Error
    } else {
        crate::config::severity::Severity::Warning
    };

    if mode == crate::config::enforcement_mode::EnforcementMode::Relaxed
        || scope == crate::config::path_classification::PathClassification::Unstable
    {
        severity = crate::config::severity::Severity::Warning;
    }

    crate::report::violation::Violation {
        rule_id,
        violation_code: code,
        severity,
        scope,
        path: path.to_string_lossy().to_string(),
        message: message.to_string(),
        line,
    }
}
