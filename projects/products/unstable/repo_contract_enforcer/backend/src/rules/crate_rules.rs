#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CrateRules;

impl CrateRules {
    pub fn evaluate(
        product_dir: &std::path::Path,
        product_name: &str,
        scope: crate::config::path_classification::PathClassification,
        mode: crate::config::enforcement_mode::EnforcementMode,
    ) -> Vec<crate::report::violation::Violation> {
        use crate::report::violation_code::ViolationCode;
        use crate::rules::rule_id::RuleId;

        let mut out = Vec::new();
        let backend = product_dir.join("backend");
        let ui = product_dir.join("ui");

        for (crate_name, crate_path, expected_name) in [
            (
                "backend",
                backend.as_path(),
                format!("{product_name}_backend"),
            ),
            ("ui", ui.as_path(), format!("{product_name}_ui")),
        ] {
            let cargo = crate_path.join("Cargo.toml");
            let main_rs = crate_path.join("src/main.rs");
            if !main_rs.exists() {
                out.push(make_violation(
                    RuleId::Crate,
                    ViolationCode::CrateMissingMain,
                    scope,
                    &main_rs,
                    "crate must contain src/main.rs",
                    mode,
                    true,
                    None,
                ));
            }

            if cargo.exists() {
                let txt = std::fs::read_to_string(&cargo).unwrap_or_default();
                if txt.contains("[lib]") {
                    out.push(make_violation(
                        RuleId::Crate,
                        ViolationCode::CrateNotBinOnly,
                        scope,
                        &cargo,
                        "crate must be bin-only (no [lib])",
                        mode,
                        true,
                        None,
                    ));
                }

                if !txt.contains(&format!("name = \"{expected_name}\"")) {
                    out.push(make_violation(
                        RuleId::Naming,
                        ViolationCode::NameCrateMismatch,
                        scope,
                        &cargo,
                        &format!("{crate_name} crate name must be {expected_name}"),
                        mode,
                        true,
                        None,
                    ));
                }
            }
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
