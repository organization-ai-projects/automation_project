use crate::{config, reports, rules};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CrateRules;

impl CrateRules {
    pub fn evaluate(
        product_dir: &std::path::Path,
        product_name: &str,
        scope: config::path_classification::PathClassification,
        mode: config::enforcement_mode::EnforcementMode,
    ) -> Vec<reports::violation::Violation> {
        use crate::scan::file_scanner::FileScanner;
        use crate::scan::rust_parser::RustParser;
        use reports::violation_code::ViolationCode;
        use rules::rule_id::RuleId;

        let mut out = Vec::new();
        let is_core_workspace = product_dir
            .file_name()
            .and_then(|s| s.to_str())
            .is_some_and(|name| name == "core");
        if is_core_workspace {
            return out;
        }

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
                    (scope, mode),
                    &main_rs,
                    "crate must contain src/main.rs",
                    (true, None),
                ));
            }

            if cargo.exists() {
                let txt = std::fs::read_to_string(&cargo).unwrap_or_default();
                if txt.contains("[lib]") {
                    out.push(make_violation(
                        RuleId::Crate,
                        ViolationCode::CrateNotBinOnly,
                        (scope, mode),
                        &cargo,
                        "crate must be bin-only (no [lib])",
                        (true, None),
                    ));
                }

                if !txt.contains(&format!("name = \"{expected_name}\"")) {
                    out.push(make_violation(
                        RuleId::Naming,
                        ViolationCode::NameCrateMismatch,
                        (scope, mode),
                        &cargo,
                        &format!("{crate_name} crate name must be {expected_name}"),
                        (true, None),
                    ));
                }
            }

            let src_dir = crate_path.join("src");
            if src_dir.exists() {
                for rs_file in FileScanner::gather_rs_files(&src_dir) {
                    let stem = rs_file
                        .file_stem()
                        .and_then(|s| s.to_str())
                        .unwrap_or_default();
                    if !should_enforce_primary_item_contract(&src_dir, &rs_file, stem) {
                        continue;
                    }

                    if let Some(expected_test_path) = expected_paired_test_path(&rs_file) {
                        if !expected_test_path.exists() {
                            out.push(make_violation(
                                RuleId::Crate,
                                ViolationCode::CrateMissingPairedTestFile,
                                (scope, mode),
                                &rs_file,
                                &format!(
                                    "missing paired test file; expected {}",
                                    expected_test_path.to_string_lossy()
                                ),
                                (true, None),
                            ));
                        }
                    }

                    let content = std::fs::read_to_string(&rs_file).unwrap_or_default();
                    if let Some(v) = RustParser::primary_item_contract_violation(&rs_file, &content)
                    {
                        out.push(make_violation(
                            RuleId::Crate,
                            ViolationCode::CratePrimaryItemContractViolation,
                            (scope, mode),
                            &rs_file,
                            &v.message,
                            (true, v.line),
                        ));
                    }
                }
            }
        }

        out
    }
}

fn should_enforce_primary_item_contract(
    src_dir: &std::path::Path,
    rs_file: &std::path::Path,
    stem: &str,
) -> bool {
    if matches!(stem, "main" | "mod" | "lib" | "public_api") {
        return false;
    }

    let rel = match rs_file.strip_prefix(src_dir) {
        Ok(path) => path,
        Err(_) => return true,
    };
    let mut comps = rel.components();
    !comps.any(|c| c.as_os_str() == "tests")
}

fn expected_paired_test_path(rs_file: &std::path::Path) -> Option<std::path::PathBuf> {
    let stem = rs_file
        .file_stem()
        .and_then(|s| s.to_str())
        .unwrap_or_default();
    if matches!(stem, "main" | "mod" | "lib" | "public_api") {
        return None;
    }

    let parent = rs_file.parent()?;
    Some(parent.join("tests").join(format!("{stem}.rs")))
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

#[cfg(test)]
mod tests {
    use super::CrateRules;
    use crate::config::enforcement_mode::EnforcementMode;
    use crate::config::path_classification::PathClassification;
    use crate::reports::violation_code::ViolationCode;
    use std::fs;
    use std::path::PathBuf;
    use std::time::{SystemTime, UNIX_EPOCH};

    fn temp_product_root() -> PathBuf {
        let stamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("time before epoch")
            .as_nanos();
        let root = std::env::temp_dir().join(format!("repo_contract_enforcer_crate_rules_{stamp}"));
        fs::create_dir_all(&root).expect("create temp product root");
        root
    }

    fn write_minimal_bin_crate(crate_root: &std::path::Path, crate_name: &str) {
        fs::create_dir_all(crate_root.join("src")).expect("create crate src dir");
        fs::write(
            crate_root.join("Cargo.toml"),
            format!(
                "[package]\nname = \"{crate_name}\"\nversion = \"0.1.0\"\nedition = \"2021\"\n"
            ),
        )
        .expect("write Cargo.toml");
        fs::write(crate_root.join("src/main.rs"), "fn main() {}\n").expect("write main.rs");
    }

    #[test]
    fn crate_requires_paired_test_file_for_rust_module() {
        let product_root = temp_product_root();
        let product_name = "project_alpha";

        let backend = product_root.join("backend");
        let ui = product_root.join("ui");
        write_minimal_bin_crate(&backend, "project_alpha_backend");
        write_minimal_bin_crate(&ui, "project_alpha_ui");

        let request_rs = backend.join("src/protocol/request.rs");
        fs::create_dir_all(request_rs.parent().expect("request parent"))
            .expect("mkdir request parent");
        fs::write(&request_rs, "pub struct Request;\n").expect("write request.rs");

        let violations = CrateRules::evaluate(
            &product_root,
            product_name,
            PathClassification::Stable,
            EnforcementMode::Strict,
        );

        assert!(violations.iter().any(|v| {
            v.violation_code == ViolationCode::CrateMissingPairedTestFile
                && v.path.ends_with("backend/src/protocol/request.rs")
        }));
    }

    #[test]
    fn crate_paired_test_file_contract_accepts_matching_tests_path() {
        let product_root = temp_product_root();
        let product_name = "project_beta";

        let backend = product_root.join("backend");
        let ui = product_root.join("ui");
        write_minimal_bin_crate(&backend, "project_beta_backend");
        write_minimal_bin_crate(&ui, "project_beta_ui");

        let request_rs = backend.join("src/protocol/request.rs");
        fs::create_dir_all(request_rs.parent().expect("request parent"))
            .expect("mkdir request parent");
        fs::write(&request_rs, "pub struct Request;\n").expect("write request.rs");

        let request_test = backend.join("src/protocol/tests/request.rs");
        fs::create_dir_all(request_test.parent().expect("request test parent"))
            .expect("mkdir request test parent");
        fs::write(&request_test, "#[test]\nfn request_contract() {}\n")
            .expect("write request test");

        let violations = CrateRules::evaluate(
            &product_root,
            product_name,
            PathClassification::Stable,
            EnforcementMode::Strict,
        );

        assert!(
            !violations
                .iter()
                .any(|v| v.violation_code == ViolationCode::CrateMissingPairedTestFile)
        );
    }
}
