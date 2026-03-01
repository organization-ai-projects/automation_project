#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StructureRules;

impl StructureRules {
    pub fn evaluate(
        product_dir: &std::path::Path,
        scope: crate::config::path_classification::PathClassification,
        mode: crate::config::enforcement_mode::EnforcementMode,
    ) -> Vec<crate::report::violation::Violation> {
        use crate::report::violation_code::ViolationCode;
        use crate::rules::rule_id::RuleId;
        use crate::scan::crate_scanner::CrateScanner;

        let mut out = Vec::new();
        let backend = product_dir.join("backend");
        let ui = product_dir.join("ui");
        if !(backend.is_dir() && ui.is_dir()) {
            out.push(make_violation(
                RuleId::Structure,
                ViolationCode::StructMissingBackendOrUi,
                scope,
                product_dir,
                "product must include backend/ and ui/ crates",
                mode,
                true,
                None,
            ));
        }

        let child_crates = CrateScanner::discover_child_crates(product_dir);
        for extra in child_crates
            .iter()
            .filter(|name| name.as_str() != "backend" && name.as_str() != "ui")
        {
            out.push(make_violation(
                RuleId::Structure,
                ViolationCode::StructThirdCrateDetected,
                scope,
                &product_dir.join(extra),
                "only backend and ui crates are allowed",
                mode,
                true,
                None,
            ));
        }

        let root_cargo = product_dir.join("Cargo.toml");
        if root_cargo.exists() {
            let txt = std::fs::read_to_string(&root_cargo).unwrap_or_default();
            let members = CrateScanner::extract_workspace_members(&txt);
            let detail = if members.is_empty() {
                "nested root Cargo.toml is forbidden".to_string()
            } else {
                format!(
                    "nested workspace root Cargo.toml is forbidden (members: {})",
                    members.join(",")
                )
            };
            out.push(make_violation(
                RuleId::Structure,
                ViolationCode::StructThirdCrateDetected,
                scope,
                &root_cargo,
                &detail,
                mode,
                true,
                None,
            ));
        }

        if !product_dir.join("README.md").exists() {
            out.push(make_violation(
                RuleId::Structure,
                ViolationCode::StructMissingReadme,
                scope,
                product_dir,
                "product root must contain README.md",
                mode,
                true,
                None,
            ));
        }

        for bad in ["tooling", "tools", "devtools", "utils"] {
            if product_dir.join(bad).exists() {
                out.push(make_violation(
                    RuleId::Structure,
                    ViolationCode::StructForbiddenFolderName,
                    scope,
                    &product_dir.join(bad),
                    "forbidden folder name detected",
                    mode,
                    true,
                    None,
                ));
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
