use crate::{config, reports, rules};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NamingRules;

impl NamingRules {
    pub fn evaluate(
        product_dir: &std::path::Path,
        scope: config::path_classification::PathClassification,
        mode: config::enforcement_mode::EnforcementMode,
    ) -> Vec<reports::violation::Violation> {
        let mut out = Vec::new();
        let Some(dir_name) = product_dir.file_name().and_then(|n| n.to_str()) else {
            out.push(make_violation(
                rules::rule_id::RuleId::Naming,
                reports::violation_code::ViolationCode::NameProductMismatch,
                (scope, mode),
                product_dir,
                "unable to resolve product directory name",
                (true, None),
            ));
            return out;
        };

        if dir_name.is_empty() {
            out.push(make_violation(
                rules::rule_id::RuleId::Naming,
                reports::violation_code::ViolationCode::NameProductMismatch,
                (scope, mode),
                product_dir,
                "product directory name must not be empty",
                (true, None),
            ));
        }

        out
    }
}

fn make_violation(
    rule_id: rules::rule_id::RuleId,
    code: reports::violation_code::ViolationCode,
    context: (
        config::path_classification::PathClassification,
        config::enforcement_mode::EnforcementMode,
    ),
    path: &std::path::Path,
    message: &str,
    meta: (bool, Option<u32>),
) -> reports::violation::Violation {
    let (scope, mode) = context;
    let (default_blocking, line) = meta;
    let mut severity = if default_blocking {
        config::severity::Severity::Error
    } else {
        config::severity::Severity::Warning
    };

    if mode == config::enforcement_mode::EnforcementMode::Relaxed
        || scope == config::path_classification::PathClassification::Unstable
    {
        severity = config::severity::Severity::Warning;
    }

    reports::violation::Violation {
        rule_id,
        violation_code: code,
        severity,
        scope,
        path: path.to_string_lossy().to_string(),
        message: message.to_string(),
        line,
    }
}
