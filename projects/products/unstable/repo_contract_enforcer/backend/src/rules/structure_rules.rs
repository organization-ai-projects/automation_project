use crate::{config, reports, rules, scan};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StructureRules;

impl StructureRules {
    pub fn evaluate(
        product_dir: &std::path::Path,
        scope: config::path_classification::PathClassification,
        mode: config::enforcement_mode::EnforcementMode,
    ) -> Vec<reports::violation::Violation> {
        use reports::violation_code::ViolationCode;
        use rules::rule_id::RuleId;
        use scan::crate_scanner::CrateScanner;

        let mut out = Vec::new();
        let is_core_workspace = product_dir
            .file_name()
            .and_then(|s| s.to_str())
            .is_some_and(|name| name == "core");

        if !is_core_workspace {
            let backend = product_dir.join("backend");
            let ui = product_dir.join("ui");
            if !(backend.is_dir() && ui.is_dir()) {
                out.push(make_violation(
                    RuleId::Structure,
                    ViolationCode::StructMissingBackendOrUi,
                    (scope, mode),
                    product_dir,
                    "product must include backend/ and ui/ crates",
                    (true, None),
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
                    (scope, mode),
                    &product_dir.join(extra),
                    "only backend and ui crates are allowed",
                    (true, None),
                ));
            }
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
                (scope, mode),
                &root_cargo,
                &detail,
                (true, None),
            ));
        }

        if !product_dir.join("README.md").exists() {
            out.push(make_violation(
                RuleId::Structure,
                ViolationCode::StructMissingReadme,
                (scope, mode),
                product_dir,
                "product root must contain README.md",
                (true, None),
            ));
        }

        for bad in ["tooling", "tools", "devtools", "utils"] {
            if product_dir.join(bad).exists() {
                out.push(make_violation(
                    RuleId::Structure,
                    ViolationCode::StructForbiddenFolderName,
                    (scope, mode),
                    &product_dir.join(bad),
                    "forbidden folder name detected",
                    (true, None),
                ));
            }
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
