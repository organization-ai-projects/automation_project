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

            // Manifest convention is now blocking in strict mode for stable products.
            // It remains warning-only for unstable and relaxed modes.
            let transition_suffix = " (blocking in strict mode for stable products)";
            let metadata = product_dir.join("metadata.ron");
            if !metadata.exists() {
                out.push(make_violation(
                    RuleId::Structure,
                    ViolationCode::StructMissingProductMetadata,
                    (scope, mode),
                    &metadata,
                    &format!("product must include metadata.ron at root{transition_suffix}"),
                    (false, None),
                ));
            }

            for (crate_name, manifest_name) in [
                ("backend", "backend_manifest.ron"),
                ("ui", "ui_manifest.ron"),
            ] {
                let manifest = product_dir.join(crate_name).join(manifest_name);
                if !manifest.exists() {
                    out.push(make_violation(
                        RuleId::Structure,
                        ViolationCode::StructMissingCrateManifest,
                        (scope, mode),
                        &manifest,
                        &format!(
                            "crate '{crate_name}' must include {manifest_name}{transition_suffix}"
                        ),
                        (false, None),
                    ));
                }
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

    pub fn evaluate_shell_scripts(
        repo_root: &std::path::Path,
        mode: config::enforcement_mode::EnforcementMode,
    ) -> Vec<reports::violation::Violation> {
        use reports::violation_code::ViolationCode;
        use rules::rule_id::RuleId;

        let shell_root = repo_root.join("scripts/versioning/file_versioning/github");
        if !shell_root.is_dir() {
            return Vec::new();
        }

        let mut out = Vec::new();
        let mut files = Vec::new();
        collect_shell_files(&shell_root, &mut files);

        for file in files {
            let file_name = file
                .file_name()
                .and_then(|s| s.to_str())
                .unwrap_or_default();
            let content = std::fs::read_to_string(&file).unwrap_or_default();

            if file_name == "run.sh" && !content.contains("set -euo pipefail") {
                out.push(make_violation(
                    RuleId::Structure,
                    ViolationCode::StructShellRunMissingStrictMode,
                    (config::path_classification::PathClassification::Other, mode),
                    &file,
                    "run.sh must enable strict mode with `set -euo pipefail`",
                    (true, Some(1)),
                ));
            }

            if file_name == "load.sh" {
                for (idx, line) in content.lines().enumerate() {
                    if is_function_definition_line(line) {
                        out.push(make_violation(
                            RuleId::Structure,
                            ViolationCode::StructShellLoadHasFunctionDefinition,
                            (config::path_classification::PathClassification::Other, mode),
                            &file,
                            "load.sh must only aggregate module sources (no function definitions)",
                            (true, Some((idx + 1) as u32)),
                        ));
                        break;
                    }
                }
            }
        }

        out
    }
}

fn collect_shell_files(dir: &std::path::Path, out: &mut Vec<std::path::PathBuf>) {
    let read_dir = match std::fs::read_dir(dir) {
        Ok(rd) => rd,
        Err(_) => return,
    };

    for entry in read_dir.flatten() {
        let path = entry.path();
        if path.is_dir() {
            collect_shell_files(&path, out);
            continue;
        }
        if path.extension().and_then(|s| s.to_str()) == Some("sh") {
            out.push(path);
        }
    }
}

fn is_function_definition_line(line: &str) -> bool {
    let trimmed = line.trim_start();
    if trimmed.is_empty() || trimmed.starts_with('#') {
        return false;
    }

    let Some(paren_idx) = trimmed.find("()") else {
        return false;
    };
    let name = &trimmed[..paren_idx];
    if name.is_empty()
        || !name.chars().all(|c| c.is_ascii_alphanumeric() || c == '_')
        || !name
            .chars()
            .next()
            .is_some_and(|c| c.is_ascii_alphabetic() || c == '_')
    {
        return false;
    }

    let rest = trimmed[paren_idx + 2..].trim_start();
    rest.starts_with('{')
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

    if matches!(
        code,
        reports::violation_code::ViolationCode::StructMissingProductMetadata
            | reports::violation_code::ViolationCode::StructMissingCrateManifest
    ) {
        if scope == config::path_classification::PathClassification::Stable
            && mode == config::enforcement_mode::EnforcementMode::Strict
        {
            severity = config::severity::Severity::Error;
        } else {
            severity = config::severity::Severity::Warning;
        }
    }

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

#[cfg(test)]
mod tests {
    use super::StructureRules;
    use crate::config::enforcement_mode::EnforcementMode;
    use crate::reports::violation_code::ViolationCode;
    use std::fs;
    use std::time::{SystemTime, UNIX_EPOCH};

    fn temp_root(prefix: &str) -> std::path::PathBuf {
        let stamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("time before epoch")
            .as_nanos();
        let root = std::env::temp_dir().join(format!("repo_contract_enforcer_{prefix}_{stamp}"));
        fs::create_dir_all(&root).expect("create temp root");
        root
    }

    #[test]
    fn shell_run_requires_strict_mode() {
        let root = temp_root("shell_run_strict");
        let run_path =
            root.join("scripts/versioning/file_versioning/github/issues/auto_link/run.sh");
        fs::create_dir_all(run_path.parent().expect("run parent")).expect("create run parent");
        fs::write(
            &run_path,
            "#!/usr/bin/env bash\nsource \"./load.sh\"\nauto_link_run \"$@\"\n",
        )
        .expect("write run.sh");

        let violations = StructureRules::evaluate_shell_scripts(&root, EnforcementMode::Strict);
        assert!(violations.iter().any(|v| {
            v.violation_code == ViolationCode::StructShellRunMissingStrictMode
                && v.path.ends_with("run.sh")
        }));
    }

    #[test]
    fn shell_load_forbids_function_definitions() {
        let root = temp_root("shell_load_functions");
        let load_path = root.join("scripts/versioning/file_versioning/github/pr/common/load.sh");
        fs::create_dir_all(load_path.parent().expect("load parent")).expect("create load parent");
        fs::write(
            &load_path,
            "#!/usr/bin/env bash\nsource \"./a.sh\"\nhelper() {\n  echo ok\n}\n",
        )
        .expect("write load.sh");

        let violations = StructureRules::evaluate_shell_scripts(&root, EnforcementMode::Strict);
        assert!(violations.iter().any(|v| {
            v.violation_code == ViolationCode::StructShellLoadHasFunctionDefinition
                && v.path.ends_with("load.sh")
        }));
    }

    #[test]
    fn compliant_shell_layout_has_no_shell_structure_violations() {
        let root = temp_root("shell_layout_ok");
        let run_path = root.join("scripts/versioning/file_versioning/github/issues/manager/run.sh");
        let load_path = root.join("scripts/versioning/file_versioning/github/pr/common/load.sh");
        fs::create_dir_all(run_path.parent().expect("run parent")).expect("create run parent");
        fs::create_dir_all(load_path.parent().expect("load parent")).expect("create load parent");
        fs::write(
            &run_path,
            "#!/usr/bin/env bash\nset -euo pipefail\nsource \"./load.sh\"\nmain \"$@\"\n",
        )
        .expect("write compliant run.sh");
        fs::write(
            &load_path,
            "#!/usr/bin/env bash\nPR_COMMON_DIR=\"$(cd \"${BASH_SOURCE[0]%/*}\" && pwd)\"\nsource \"${PR_COMMON_DIR}/a.sh\"\n",
        )
        .expect("write compliant load.sh");

        let violations = StructureRules::evaluate_shell_scripts(&root, EnforcementMode::Strict);
        assert!(!violations.iter().any(|v| {
            matches!(
                v.violation_code,
                ViolationCode::StructShellRunMissingStrictMode
                    | ViolationCode::StructShellLoadHasFunctionDefinition
            )
        }));
    }
}
