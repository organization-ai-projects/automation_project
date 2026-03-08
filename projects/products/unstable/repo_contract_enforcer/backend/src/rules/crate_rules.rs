use crate::scan::file_scanner::FileScanner;
use crate::scan::rust_parser::{MainItemViolationKind, RustParser};
use crate::{config, reports, rules};
use reports::violation_code::ViolationCode;
use rules::rule_id::RuleId;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CrateRules;

type RuleContext = (
    config::path_classification::PathClassification,
    config::enforcement_mode::EnforcementMode,
);
type ViolationMeta = (bool, Option<u32>);
const EXCLUDED_PRIMARY_STEMS: [&str; 4] = ["main", "mod", "lib", "public_api"];

impl CrateRules {
    pub fn evaluate(
        product_dir: &std::path::Path,
        product_name: &str,
        scope: config::path_classification::PathClassification,
        mode: config::enforcement_mode::EnforcementMode,
    ) -> Vec<reports::violation::Violation> {
        let mut out = Vec::new();
        // Internal convention: `/core` is an orchestrator workspace root and not a product.
        // Skip crate-level product checks there.
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
            let lib_rs = crate_path.join("src/lib.rs");
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
            if lib_rs.exists() {
                out.push(make_violation(
                    RuleId::Crate,
                    ViolationCode::CrateNotBinOnly,
                    (scope, mode),
                    &lib_rs,
                    "crate must be bin-only; src/lib.rs is forbidden",
                    (true, None),
                ));
            }

            if cargo.exists() {
                let Some(txt) =
                    read_text_or_emit_violation(&mut out, (scope, mode), &cargo, "Cargo.toml")
                else {
                    continue;
                };
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
                if txt.contains("[[bin]]") {
                    out.push(make_violation(
                        RuleId::Crate,
                        ViolationCode::CrateNotBinOnly,
                        (scope, mode),
                        &cargo,
                        "crate must be bin-only (no [[bin]])",
                        (true, None),
                    ));
                }
                if cargo_declares_dependency(&txt, "serde_json") {
                    out.push(make_violation(
                        RuleId::Crate,
                        ViolationCode::CrateForbiddenSerdeJsonDependency,
                        (scope, mode),
                        &cargo,
                        "products must not depend on serde_json; use common_json",
                        (true, None),
                    ));
                }
                if crate_name == "ui" && !cargo_declares_dependency(&txt, "dioxus") {
                    out.push(make_violation(
                        RuleId::Crate,
                        ViolationCode::CrateUiMissingDioxusDependency,
                        (scope, mode),
                        &cargo,
                        "ui crate must declare dioxus dependency",
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

            let extra_bins_dir = crate_path.join("src/bin");
            if extra_bins_dir.exists() {
                let bin_rs_files = FileScanner::gather_rs_files(&extra_bins_dir);
                if bin_rs_files.is_empty() {
                    out.push(make_violation(
                        RuleId::Crate,
                        ViolationCode::CrateNotBinOnly,
                        (scope, mode),
                        &extra_bins_dir,
                        "crate must be single-bin only; src/bin is forbidden",
                        (true, None),
                    ));
                } else {
                    for rs in bin_rs_files {
                        out.push(make_violation(
                            RuleId::Crate,
                            ViolationCode::CrateNotBinOnly,
                            (scope, mode),
                            &rs,
                            "crate must be single-bin only; src/bin targets are forbidden",
                            (true, None),
                        ));
                    }
                }
            }

            let src_dir = crate_path.join("src");
            if src_dir.exists() {
                for rs_file in FileScanner::gather_rs_files(&src_dir) {
                    let stem = rs_file
                        .file_stem()
                        .and_then(|s| s.to_str())
                        .unwrap_or_default();
                    let Some(source_content) =
                        read_text_or_emit_violation(&mut out, (scope, mode), &rs_file, "source")
                    else {
                        continue;
                    };
                    let is_test_file = is_file_under_tests_dir(&src_dir, &rs_file);

                    for line in RustParser::local_use_statement_lines(&source_content) {
                        out.push(make_violation(
                            RuleId::Crate,
                            ViolationCode::CrateContainsLocalUseStatement,
                            (scope, mode),
                            &rs_file,
                            "local `use` statements are forbidden; imports must be declared at module scope",
                            (true, Some(line)),
                        ));
                    }

                    if !is_test_file {
                        let allowed_inline_test_attr_lines = if stem == "mod" || stem == "main" {
                            allowed_inline_test_attr_lines_for_mod_rs(&source_content)
                        } else {
                            std::collections::HashSet::new()
                        };
                        for line in RustParser::inline_test_attribute_lines(&source_content) {
                            if allowed_inline_test_attr_lines.contains(&line) {
                                continue;
                            }
                            out.push(make_violation(
                                RuleId::Crate,
                                ViolationCode::CrateContainsInlineTestAttribute,
                                (scope, mode),
                                &rs_file,
                                "inline tests are forbidden in source files; move tests to nearest src/**/tests/*.rs",
                                (true, Some(line)),
                            ));
                        }
                    }

                    if stem == "main" {
                        for line in find_unscoped_pub_lines_in_main(&source_content) {
                            out.push(make_violation(
                                RuleId::Crate,
                                ViolationCode::CrateBinaryMainUnscopedPub,
                                (scope, mode),
                                &rs_file,
                                "unscoped `pub` in binary main module is discouraged; prefer private items or `pub(crate)`",
                                (false, Some(line)),
                            ));
                        }
                        for finding in RustParser::main_module_item_violations(&source_content) {
                            let (code, message) = match finding.kind {
                                MainItemViolationKind::Struct => (
                                    ViolationCode::CrateBinaryMainContainsStruct,
                                    "binary main module must not define struct; move it to a dedicated module",
                                ),
                                MainItemViolationKind::Enum => (
                                    ViolationCode::CrateBinaryMainContainsEnum,
                                    "binary main module must not define enum; move it to a dedicated module",
                                ),
                                MainItemViolationKind::Trait => (
                                    ViolationCode::CrateBinaryMainContainsTrait,
                                    "binary main module must not define trait; move it to a dedicated module",
                                ),
                                MainItemViolationKind::Impl => (
                                    ViolationCode::CrateBinaryMainContainsImpl,
                                    "binary main module must not define impl blocks; move behavior to dedicated modules",
                                ),
                                MainItemViolationKind::NonEntrypointFn => (
                                    ViolationCode::CrateBinaryMainContainsNonEntrypointFn,
                                    "binary main module must only expose the entrypoint function `main`; move helper functions to dedicated modules",
                                ),
                            };
                            out.push(make_violation(
                                RuleId::Crate,
                                code,
                                (scope, mode),
                                &rs_file,
                                message,
                                (true, finding.line),
                            ));
                        }
                    }

                    if !should_enforce_primary_item_contract(&src_dir, &rs_file, stem) {
                        continue;
                    }

                    if let Some(expected_test_path) = expected_paired_test_path(&rs_file) {
                        let requires_paired_test = rust_file_has_test_worthy_logic(&source_content);

                        if requires_paired_test && !expected_test_path.exists() {
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
                        } else if expected_test_path.exists() {
                            let Some(paired_test_content) = read_text_or_emit_violation(
                                &mut out,
                                (scope, mode),
                                &expected_test_path,
                                "paired test source",
                            ) else {
                                continue;
                            };
                            if !looks_like_unit_test_file(&paired_test_content) {
                                out.push(make_violation(
                                    RuleId::Crate,
                                    ViolationCode::CratePairedTestNotUnitStyle,
                                    (scope, mode),
                                    &expected_test_path,
                                    "paired test file must define at least one unit test (e.g. #[test], #[tokio::test], #[rstest])",
                                    (true, None),
                                ));
                            }
                        }
                    }

                    if let Some(v) =
                        RustParser::primary_item_contract_violation(&rs_file, &source_content)
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
    if is_excluded_primary_stem(stem) {
        return false;
    }

    let rel = match rs_file.strip_prefix(src_dir) {
        Ok(path) => path,
        Err(_) => return true,
    };
    let mut comps = rel.components();
    !comps.any(|c| c.as_os_str() == "tests")
}

fn is_file_under_tests_dir(src_dir: &std::path::Path, rs_file: &std::path::Path) -> bool {
    let rel = match rs_file.strip_prefix(src_dir) {
        Ok(path) => path,
        Err(_) => return false,
    };
    rel.components().any(|c| c.as_os_str() == "tests")
}

fn allowed_inline_test_attr_lines_for_mod_rs(content: &str) -> std::collections::HashSet<u32> {
    let mut allowed = std::collections::HashSet::new();
    let lines: Vec<&str> = content.lines().collect();
    for (idx, line) in lines.iter().enumerate() {
        if line.trim() != "#[cfg(test)]" {
            continue;
        }
        let mut next_idx = idx + 1;
        while next_idx < lines.len() && lines[next_idx].trim().is_empty() {
            next_idx += 1;
        }
        if next_idx >= lines.len() {
            continue;
        }
        let next = lines[next_idx].trim();
        if next == "mod tests;" || next == "pub mod tests;" {
            allowed.insert((idx + 1) as u32);
        }
    }
    allowed
}

fn expected_paired_test_path(rs_file: &std::path::Path) -> Option<std::path::PathBuf> {
    let stem = rs_file
        .file_stem()
        .and_then(|s| s.to_str())
        .unwrap_or_default();
    if is_excluded_primary_stem(stem) {
        return None;
    }

    let parent = rs_file.parent()?;
    Some(parent.join("tests").join(format!("{stem}.rs")))
}

fn looks_like_unit_test_file(content: &str) -> bool {
    // Heuristic guardrail: unit-style test files must expose at least one test attribute.
    // This prevents placeholder/integration-like files from satisfying the paired-test contract.
    const TEST_MARKERS: [&str; 4] = ["#[test]", "#[tokio::test]", "#[rstest]", "#[test_case]"];
    TEST_MARKERS.iter().any(|marker| content.contains(marker))
}

fn rust_file_has_test_worthy_logic(content: &str) -> bool {
    // Heuristic: require paired tests only when file contains behavioral logic.
    // Data-only type declarations (plain struct/enum/trait definitions) are exempt.
    content.lines().map(str::trim_start).any(is_logic_line)
}

fn find_unscoped_pub_lines_in_main(content: &str) -> Vec<u32> {
    let mut out = Vec::new();
    for (idx, line) in content.lines().enumerate() {
        let trimmed = line.trim_start();
        if trimmed.starts_with("//")
            || trimmed.starts_with("#[")
            || trimmed.starts_with("pub(crate)")
            || trimmed.starts_with("pub(super)")
            || trimmed.starts_with("pub(in ")
        {
            continue;
        }
        if trimmed.starts_with("pub ") {
            out.push((idx + 1) as u32);
        }
    }
    out
}

fn make_violation(
    rule_id: rules::rule_id::RuleId,
    code: reports::violation_code::ViolationCode,
    context: RuleContext,
    path: &std::path::Path,
    message: &str,
    meta: ViolationMeta,
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

fn read_text_or_emit_violation(
    out: &mut Vec<reports::violation::Violation>,
    context: RuleContext,
    path: &std::path::Path,
    kind: &str,
) -> Option<String> {
    match std::fs::read_to_string(path) {
        Ok(content) => Some(content),
        Err(err) => {
            out.push(make_violation(
                RuleId::Crate,
                ViolationCode::RuleFileReadError,
                context,
                path,
                &format!("failed to read {kind} file: {err}"),
                (true, None),
            ));
            None
        }
    }
}

fn is_excluded_primary_stem(stem: &str) -> bool {
    EXCLUDED_PRIMARY_STEMS.contains(&stem)
}

fn is_logic_line(trimmed: &str) -> bool {
    is_fn_line(trimmed) || trimmed.starts_with("impl ") || trimmed.starts_with("macro_rules!")
}

fn is_fn_line(trimmed: &str) -> bool {
    trimmed.starts_with("fn ")
        || trimmed.starts_with("pub fn ")
        || trimmed.starts_with("async fn ")
        || trimmed.starts_with("pub async fn ")
        || trimmed.starts_with("unsafe fn ")
        || trimmed.starts_with("pub unsafe fn ")
        || trimmed.starts_with("async unsafe fn ")
        || trimmed.starts_with("pub async unsafe fn ")
}

fn cargo_declares_dependency(cargo_toml_content: &str, crate_name: &str) -> bool {
    let mut in_dependencies = false;
    for raw in cargo_toml_content.lines() {
        let line = raw.trim();
        if line.is_empty() || line.starts_with('#') {
            continue;
        }
        if line.starts_with('[') {
            in_dependencies = line.contains("dependencies");
            continue;
        }
        if !in_dependencies {
            continue;
        }
        let Some((name, _)) = line.split_once('=') else {
            continue;
        };
        if name.trim().trim_matches('"') == crate_name {
            return true;
        }
    }
    false
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
    fn crate_requires_paired_test_file_for_logic_module() {
        let product_root = temp_product_root();
        let product_name = "project_alpha";

        let backend = product_root.join("backend");
        let ui = product_root.join("ui");
        write_minimal_bin_crate(&backend, "project_alpha_backend");
        write_minimal_bin_crate(&ui, "project_alpha_ui");

        let request_rs = backend.join("src/protocol/request.rs");
        fs::create_dir_all(request_rs.parent().expect("request parent"))
            .expect("mkdir request parent");
        fs::write(
            &request_rs,
            "pub struct Request;\n\nimpl Request {\n    pub fn is_valid(&self) -> bool { true }\n}\n",
        )
        .expect("write request.rs");

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
        fs::write(
            &request_rs,
            "pub struct Request;\n\nimpl Request {\n    pub fn is_valid(&self) -> bool { true }\n}\n",
        )
        .expect("write request.rs");

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

        assert!(!violations.iter().any(|v| {
            matches!(
                v.violation_code,
                ViolationCode::CrateMissingPairedTestFile
                    | ViolationCode::CratePairedTestNotUnitStyle
            )
        }));
    }

    #[test]
    fn crate_paired_test_file_requires_unit_test_marker() {
        let product_root = temp_product_root();
        let product_name = "project_gamma";

        let backend = product_root.join("backend");
        let ui = product_root.join("ui");
        write_minimal_bin_crate(&backend, "project_gamma_backend");
        write_minimal_bin_crate(&ui, "project_gamma_ui");

        let request_rs = backend.join("src/protocol/request.rs");
        fs::create_dir_all(request_rs.parent().expect("request parent"))
            .expect("mkdir request parent");
        fs::write(
            &request_rs,
            "pub struct Request;\n\nimpl Request {\n    pub fn is_valid(&self) -> bool { true }\n}\n",
        )
        .expect("write request.rs");

        let request_test = backend.join("src/protocol/tests/request.rs");
        fs::create_dir_all(request_test.parent().expect("request test parent"))
            .expect("mkdir request test parent");
        fs::write(
            &request_test,
            "pub fn helper_for_external_integration() -> bool { true }\n",
        )
        .expect("write request test");

        let violations = CrateRules::evaluate(
            &product_root,
            product_name,
            PathClassification::Stable,
            EnforcementMode::Strict,
        );

        assert!(violations.iter().any(|v| {
            v.violation_code == ViolationCode::CratePairedTestNotUnitStyle
                && v.path.ends_with("backend/src/protocol/tests/request.rs")
        }));
    }

    #[test]
    fn crate_data_only_module_does_not_require_paired_test_file() {
        let product_root = temp_product_root();
        let product_name = "project_delta";

        let backend = product_root.join("backend");
        let ui = product_root.join("ui");
        write_minimal_bin_crate(&backend, "project_delta_backend");
        write_minimal_bin_crate(&ui, "project_delta_ui");

        let record_rs = backend.join("src/store/account_record.rs");
        fs::create_dir_all(record_rs.parent().expect("record parent"))
            .expect("mkdir record parent");
        fs::write(
            &record_rs,
            "#[derive(Debug, Clone)]\npub struct AccountRecord {\n    pub id: String,\n}\n",
        )
        .expect("write account_record.rs");

        let violations = CrateRules::evaluate(
            &product_root,
            product_name,
            PathClassification::Stable,
            EnforcementMode::Strict,
        );

        assert!(!violations.iter().any(|v| {
            v.violation_code == ViolationCode::CrateMissingPairedTestFile
                && v.path.ends_with("backend/src/store/account_record.rs")
        }));
    }

    #[test]
    fn crate_detects_unscoped_pub_in_binary_main_module() {
        let product_root = temp_product_root();
        let product_name = "project_eps";

        let backend = product_root.join("backend");
        let ui = product_root.join("ui");
        write_minimal_bin_crate(&backend, "project_eps_backend");
        write_minimal_bin_crate(&ui, "project_eps_ui");

        fs::write(
            backend.join("src/main.rs"),
            "#![allow(dead_code)]\npub mod public_api;\nfn main() {}\n",
        )
        .expect("write backend main.rs");

        let violations = CrateRules::evaluate(
            &product_root,
            product_name,
            PathClassification::Stable,
            EnforcementMode::Strict,
        );

        assert!(violations.iter().any(|v| {
            v.violation_code == ViolationCode::CrateBinaryMainUnscopedPub
                && v.path.ends_with("backend/src/main.rs")
        }));
    }

    #[test]
    fn crate_detects_disallowed_items_in_binary_main_module() {
        let product_root = temp_product_root();
        let product_name = "project_zeta";

        let backend = product_root.join("backend");
        let ui = product_root.join("ui");
        write_minimal_bin_crate(&backend, "project_zeta_backend");
        write_minimal_bin_crate(&ui, "project_zeta_ui");

        fs::write(
            backend.join("src/main.rs"),
            "pub(crate) struct App;\nenum Mode { Fast }\ntrait Runner {}\nimpl App { fn run(&self) {} }\nfn helper() {}\nfn main() {}\n",
        )
        .expect("write backend main.rs");

        let violations = CrateRules::evaluate(
            &product_root,
            product_name,
            PathClassification::Stable,
            EnforcementMode::Strict,
        );

        assert!(
            violations
                .iter()
                .any(|v| v.violation_code == ViolationCode::CrateBinaryMainContainsStruct)
        );
        assert!(
            violations
                .iter()
                .any(|v| v.violation_code == ViolationCode::CrateBinaryMainContainsEnum)
        );
        assert!(
            violations
                .iter()
                .any(|v| v.violation_code == ViolationCode::CrateBinaryMainContainsTrait)
        );
        assert!(
            violations
                .iter()
                .any(|v| v.violation_code == ViolationCode::CrateBinaryMainContainsImpl)
        );
        assert!(
            violations
                .iter()
                .any(|v| v.violation_code == ViolationCode::CrateBinaryMainContainsNonEntrypointFn)
        );
    }

    #[test]
    fn crate_detects_local_use_statement_in_function_scope() {
        let product_root = temp_product_root();
        let product_name = "project_eta";

        let backend = product_root.join("backend");
        let ui = product_root.join("ui");
        write_minimal_bin_crate(&backend, "project_eta_backend");
        write_minimal_bin_crate(&ui, "project_eta_ui");

        fs::write(
            backend.join("src/service.rs"),
            "pub fn run() {\n    use std::collections::HashMap;\n    let _m: HashMap<String, String> = HashMap::new();\n}\n",
        )
        .expect("write service.rs");

        let violations = CrateRules::evaluate(
            &product_root,
            product_name,
            PathClassification::Stable,
            EnforcementMode::Strict,
        );

        assert!(violations.iter().any(|v| {
            v.violation_code == ViolationCode::CrateContainsLocalUseStatement
                && v.path.ends_with("backend/src/service.rs")
        }));
    }

    #[test]
    fn crate_detects_inline_test_attribute_outside_tests_dir() {
        let product_root = temp_product_root();
        let product_name = "project_theta";

        let backend = product_root.join("backend");
        let ui = product_root.join("ui");
        write_minimal_bin_crate(&backend, "project_theta_backend");
        write_minimal_bin_crate(&ui, "project_theta_ui");

        fs::write(
            backend.join("src/service.rs"),
            "#[cfg(test)]\nmod tests {\n    #[test]\n    fn smoke() {}\n}\n",
        )
        .expect("write service.rs");
        let service_test = backend.join("src/tests/service.rs");
        fs::create_dir_all(service_test.parent().expect("service test parent"))
            .expect("create service test parent");
        fs::write(&service_test, "#[test]\nfn allowed_in_tests_folder() {}\n")
            .expect("write tests/service.rs");

        let violations = CrateRules::evaluate(
            &product_root,
            product_name,
            PathClassification::Stable,
            EnforcementMode::Strict,
        );

        assert!(violations.iter().any(|v| {
            v.violation_code == ViolationCode::CrateContainsInlineTestAttribute
                && v.path.ends_with("backend/src/service.rs")
        }));
        assert!(!violations.iter().any(|v| {
            v.violation_code == ViolationCode::CrateContainsInlineTestAttribute
                && v.path.ends_with("backend/src/tests/service.rs")
        }));
    }

    #[test]
    fn crate_allows_cfg_test_mod_tests_declaration_in_mod_rs() {
        let product_root = temp_product_root();
        let product_name = "project_theta_bis";

        let backend = product_root.join("backend");
        let ui = product_root.join("ui");
        write_minimal_bin_crate(&backend, "project_theta_bis_backend");
        write_minimal_bin_crate(&ui, "project_theta_bis_ui");

        let domain_mod = backend.join("src/domain/mod.rs");
        fs::create_dir_all(domain_mod.parent().expect("domain parent"))
            .expect("create domain parent");
        fs::write(&domain_mod, "#[cfg(test)]\nmod tests;\n").expect("write domain/mod.rs");

        let domain_tests_mod = backend.join("src/domain/tests/mod.rs");
        fs::create_dir_all(domain_tests_mod.parent().expect("domain tests parent"))
            .expect("create domain tests parent");
        fs::write(&domain_tests_mod, "#[test]\nfn smoke() {}\n").expect("write domain tests");

        let violations = CrateRules::evaluate(
            &product_root,
            product_name,
            PathClassification::Stable,
            EnforcementMode::Strict,
        );

        assert!(!violations.iter().any(|v| {
            v.violation_code == ViolationCode::CrateContainsInlineTestAttribute
                && v.path.ends_with("backend/src/domain/mod.rs")
        }));
    }

    #[test]
    fn crate_allows_cfg_test_mod_tests_declaration_in_main_rs() {
        let product_root = temp_product_root();
        let product_name = "project_theta_ter";

        let backend = product_root.join("backend");
        let ui = product_root.join("ui");
        write_minimal_bin_crate(&backend, "project_theta_ter_backend");
        write_minimal_bin_crate(&ui, "project_theta_ter_ui");

        fs::write(
            backend.join("src/main.rs"),
            "#[cfg(test)]\nmod tests;\nfn main() {}\n",
        )
        .expect("write backend main.rs");

        let backend_tests_mod = backend.join("src/tests/mod.rs");
        fs::create_dir_all(backend_tests_mod.parent().expect("backend tests parent"))
            .expect("create backend tests parent");
        fs::write(
            &backend_tests_mod,
            "#![cfg(test)]\n#[test]\nfn smoke() {}\n",
        )
        .expect("write backend tests mod");

        let violations = CrateRules::evaluate(
            &product_root,
            product_name,
            PathClassification::Stable,
            EnforcementMode::Strict,
        );

        assert!(!violations.iter().any(|v| {
            v.violation_code == ViolationCode::CrateContainsInlineTestAttribute
                && v.path.ends_with("backend/src/main.rs")
        }));
    }

    #[test]
    fn crate_rejects_explicit_additional_bin_targets() {
        let product_root = temp_product_root();
        let product_name = "project_iota";

        let backend = product_root.join("backend");
        let ui = product_root.join("ui");
        write_minimal_bin_crate(&backend, "project_iota_backend");
        write_minimal_bin_crate(&ui, "project_iota_ui");

        fs::write(
            backend.join("Cargo.toml"),
            "[package]\nname = \"project_iota_backend\"\nversion = \"0.1.0\"\nedition = \"2021\"\n\n[[bin]]\nname = \"project_iota_backend\"\npath = \"src/main.rs\"\n\n[[bin]]\nname = \"extra\"\npath = \"src/bin/extra.rs\"\n",
        )
        .expect("write backend Cargo with [[bin]]");

        let violations = CrateRules::evaluate(
            &product_root,
            product_name,
            PathClassification::Stable,
            EnforcementMode::Strict,
        );

        assert!(violations.iter().any(|v| {
            v.violation_code == ViolationCode::CrateNotBinOnly
                && v.path.ends_with("backend/Cargo.toml")
                && v.message.contains("no [[bin]]")
        }));
    }

    #[test]
    fn crate_rejects_src_bin_targets() {
        let product_root = temp_product_root();
        let product_name = "project_kappa";

        let backend = product_root.join("backend");
        let ui = product_root.join("ui");
        write_minimal_bin_crate(&backend, "project_kappa_backend");
        write_minimal_bin_crate(&ui, "project_kappa_ui");

        let extra = backend.join("src/bin/extra.rs");
        fs::create_dir_all(extra.parent().expect("extra parent")).expect("mkdir extra bin parent");
        fs::write(&extra, "fn main() {}\n").expect("write extra bin");

        let violations = CrateRules::evaluate(
            &product_root,
            product_name,
            PathClassification::Stable,
            EnforcementMode::Strict,
        );

        assert!(violations.iter().any(|v| {
            v.violation_code == ViolationCode::CrateNotBinOnly
                && v.path.ends_with("backend/src/bin/extra.rs")
                && v.message.contains("src/bin targets are forbidden")
        }));
    }

    #[test]
    fn crate_rejects_serde_json_dependency_for_products() {
        let product_root = temp_product_root();
        let product_name = "project_lambda";

        let backend = product_root.join("backend");
        let ui = product_root.join("ui");
        write_minimal_bin_crate(&backend, "project_lambda_backend");
        write_minimal_bin_crate(&ui, "project_lambda_ui");

        fs::write(
            backend.join("Cargo.toml"),
            "[package]\nname = \"project_lambda_backend\"\nversion = \"0.1.0\"\nedition = \"2021\"\n\n[dependencies]\nserde_json = \"1\"\n",
        )
        .expect("write backend Cargo with serde_json");

        let violations = CrateRules::evaluate(
            &product_root,
            product_name,
            PathClassification::Stable,
            EnforcementMode::Strict,
        );

        assert!(violations.iter().any(|v| {
            v.violation_code == ViolationCode::CrateForbiddenSerdeJsonDependency
                && v.path.ends_with("backend/Cargo.toml")
        }));
    }

    #[test]
    fn crate_requires_dioxus_dependency_in_ui_crate() {
        let product_root = temp_product_root();
        let product_name = "project_mu";

        let backend = product_root.join("backend");
        let ui = product_root.join("ui");
        write_minimal_bin_crate(&backend, "project_mu_backend");
        write_minimal_bin_crate(&ui, "project_mu_ui");

        let violations = CrateRules::evaluate(
            &product_root,
            product_name,
            PathClassification::Stable,
            EnforcementMode::Strict,
        );

        assert!(violations.iter().any(|v| {
            v.violation_code == ViolationCode::CrateUiMissingDioxusDependency
                && v.path.ends_with("ui/Cargo.toml")
        }));
    }

    #[test]
    fn crate_accepts_dioxus_dependency_in_target_specific_section() {
        let product_root = temp_product_root();
        let product_name = "project_nu";

        let backend = product_root.join("backend");
        let ui = product_root.join("ui");
        write_minimal_bin_crate(&backend, "project_nu_backend");
        write_minimal_bin_crate(&ui, "project_nu_ui");

        fs::write(
            ui.join("Cargo.toml"),
            "[package]\nname = \"project_nu_ui\"\nversion = \"0.1.0\"\nedition = \"2021\"\n\n[target.'cfg(target_arch = \"wasm32\")'.dependencies]\ndioxus = { version = \"0.7.3\", features = [\"web\"] }\n",
        )
        .expect("write ui Cargo with target dioxus");

        let violations = CrateRules::evaluate(
            &product_root,
            product_name,
            PathClassification::Stable,
            EnforcementMode::Strict,
        );

        assert!(!violations.iter().any(|v| {
            v.violation_code == ViolationCode::CrateUiMissingDioxusDependency
                && v.path.ends_with("ui/Cargo.toml")
        }));
    }
}
