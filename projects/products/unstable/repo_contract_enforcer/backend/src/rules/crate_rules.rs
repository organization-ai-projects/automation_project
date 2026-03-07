use crate::{config, reports, rules};

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
        use crate::scan::file_scanner::FileScanner;
        use crate::scan::rust_parser::RustParser;
        use reports::violation_code::ViolationCode;
        use rules::rule_id::RuleId;

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
                    let Some(source_content) =
                        read_text_or_emit_violation(&mut out, (scope, mode), &rs_file, "source")
                    else {
                        continue;
                    };

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
                        for finding in find_disallowed_items_in_main(&source_content) {
                            out.push(make_violation(
                                RuleId::Crate,
                                finding.code,
                                (scope, mode),
                                &rs_file,
                                finding.message,
                                (true, Some(finding.line)),
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

struct MainFileFinding {
    code: reports::violation_code::ViolationCode,
    line: u32,
    message: &'static str,
}

fn find_disallowed_items_in_main(content: &str) -> Vec<MainFileFinding> {
    use reports::violation_code::ViolationCode;

    // Heuristic line-based scanner: intentionally lightweight and not a full Rust parser.
    // It may miss or over-report some complex constructs; primary goal is guardrail enforcement.
    let mut out = Vec::new();
    for (idx, line) in content.lines().enumerate() {
        let trimmed = line.trim_start();
        if trimmed.starts_with("//") || trimmed.starts_with("#[") {
            continue;
        }
        let line_no = (idx + 1) as u32;

        if is_struct_line(trimmed) {
            out.push(MainFileFinding {
                code: ViolationCode::CrateBinaryMainContainsStruct,
                line: line_no,
                message: "binary main module must not define struct; move it to a dedicated module",
            });
            continue;
        }
        if is_enum_line(trimmed) {
            out.push(MainFileFinding {
                code: ViolationCode::CrateBinaryMainContainsEnum,
                line: line_no,
                message: "binary main module must not define enum; move it to a dedicated module",
            });
            continue;
        }
        if is_trait_line(trimmed) {
            out.push(MainFileFinding {
                code: ViolationCode::CrateBinaryMainContainsTrait,
                line: line_no,
                message: "binary main module must not define trait; move it to a dedicated module",
            });
            continue;
        }
        if trimmed.starts_with("impl ") {
            out.push(MainFileFinding {
                code: ViolationCode::CrateBinaryMainContainsImpl,
                line: line_no,
                message: "binary main module must not define impl blocks; move behavior to dedicated modules",
            });
            continue;
        }

        let has_fn = is_fn_line(trimmed);
        if has_fn {
            let is_main = trimmed.starts_with("fn main")
                || trimmed.starts_with("pub fn main")
                || trimmed.starts_with("async fn main")
                || trimmed.starts_with("pub async fn main")
                || trimmed.starts_with("unsafe fn main")
                || trimmed.starts_with("pub unsafe fn main")
                || trimmed.starts_with("async unsafe fn main")
                || trimmed.starts_with("pub async unsafe fn main");
            if !is_main {
                out.push(MainFileFinding {
                    code: ViolationCode::CrateBinaryMainContainsNonEntrypointFn,
                    line: line_no,
                    message: "binary main module must only expose the entrypoint function `main`; move helper functions to dedicated modules",
                });
            }
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
    use reports::violation_code::ViolationCode;
    use rules::rule_id::RuleId;

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

fn is_struct_line(trimmed: &str) -> bool {
    trimmed.starts_with("struct ")
        || trimmed.starts_with("pub struct ")
        || trimmed.starts_with("pub(crate) struct ")
        || trimmed.starts_with("pub(super) struct ")
        || (trimmed.starts_with("pub(in ") && trimmed.contains(") struct "))
}

fn is_enum_line(trimmed: &str) -> bool {
    trimmed.starts_with("enum ")
        || trimmed.starts_with("pub enum ")
        || trimmed.starts_with("pub(crate) enum ")
        || trimmed.starts_with("pub(super) enum ")
        || (trimmed.starts_with("pub(in ") && trimmed.contains(") enum "))
}

fn is_trait_line(trimmed: &str) -> bool {
    trimmed.starts_with("trait ")
        || trimmed.starts_with("pub trait ")
        || trimmed.starts_with("pub(crate) trait ")
        || trimmed.starts_with("pub(super) trait ")
        || (trimmed.starts_with("pub(in ") && trimmed.contains(") trait "))
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
}
