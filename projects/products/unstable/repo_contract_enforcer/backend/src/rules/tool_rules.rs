use crate::scan::file_scanner::FileScanner;
use crate::scan::rust_parser::{MainItemViolationKind, RustParser};
use crate::{config, reports, rules};
use reports::violation_code::ViolationCode;
use rules::rule_id::RuleId;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ToolRules;

type RuleContext = (
    config::path_classification::PathClassification,
    config::enforcement_mode::EnforcementMode,
);
type ViolationMeta = (bool, Option<u32>);
const EXCLUDED_PRIMARY_STEMS: [&str; 4] = ["main", "mod", "lib", "public_api"];

impl ToolRules {
    pub fn evaluate(
        tool_dir: &std::path::Path,
        scope: config::path_classification::PathClassification,
        mode: config::enforcement_mode::EnforcementMode,
    ) -> Vec<reports::violation::Violation> {
        let mut out = Vec::new();

        let tool_name = tool_dir
            .file_name()
            .and_then(|s| s.to_str())
            .unwrap_or("unknown_tool");

        let cargo = tool_dir.join("Cargo.toml");
        let src_dir = tool_dir.join("src");
        let main_rs = src_dir.join("main.rs");
        let lib_rs = src_dir.join("lib.rs");

        if !main_rs.exists() {
            out.push(make_violation(
                RuleId::Crate,
                ViolationCode::CrateMissingMain,
                (scope, mode),
                &main_rs,
                "tool crate must contain src/main.rs",
                (true, None),
            ));
        }

        if lib_rs.exists() {
            out.push(make_violation(
                RuleId::Crate,
                ViolationCode::CrateNotBinOnly,
                (scope, mode),
                &lib_rs,
                "tool crate must be bin-only; src/lib.rs is forbidden",
                (true, None),
            ));
        }

        if cargo.exists() {
            if let Some(txt) =
                read_text_or_emit_violation(&mut out, (scope, mode), &cargo, "Cargo.toml")
            {
                if txt.contains("[lib]") {
                    out.push(make_violation(
                        RuleId::Crate,
                        ViolationCode::CrateNotBinOnly,
                        (scope, mode),
                        &cargo,
                        "tool crate must be bin-only (no [lib])",
                        (true, None),
                    ));
                }
                if txt.contains("[[bin]]") {
                    out.push(make_violation(
                        RuleId::Crate,
                        ViolationCode::CrateNotBinOnly,
                        (scope, mode),
                        &cargo,
                        "tool crate must be single-bin only (no [[bin]])",
                        (true, None),
                    ));
                }
                if !txt.contains(&format!("name = \"{tool_name}\"")) {
                    out.push(make_violation(
                        RuleId::Naming,
                        ViolationCode::NameCrateMismatch,
                        (scope, mode),
                        &cargo,
                        &format!("tool crate name must be {tool_name}"),
                        (true, None),
                    ));
                }
            }
        }

        let extra_bins_dir = src_dir.join("bin");
        if extra_bins_dir.exists() {
            out.push(make_violation(
                RuleId::Crate,
                ViolationCode::CrateNotBinOnly,
                (scope, mode),
                &extra_bins_dir,
                "tool crate must be single-bin only; src/bin is forbidden",
                (true, None),
            ));
        }

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
                    } else if expected_test_path.exists()
                        && let Some(paired_test_content) = read_text_or_emit_violation(
                            &mut out,
                            (scope, mode),
                            &expected_test_path,
                            "paired test source",
                        )
                        && !looks_like_unit_test_file(&paired_test_content)
                    {
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
    const TEST_MARKERS: [&str; 4] = ["#[test]", "#[tokio::test]", "#[rstest]", "#[test_case]"];
    TEST_MARKERS.iter().any(|marker| content.contains(marker))
}

fn rust_file_has_test_worthy_logic(content: &str) -> bool {
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

#[cfg(test)]
mod tests {
    use super::ToolRules;
    use crate::config::enforcement_mode::EnforcementMode;
    use crate::config::path_classification::PathClassification;
    use crate::reports::violation_code::ViolationCode;
    use std::fs;
    use std::path::PathBuf;
    use std::time::{SystemTime, UNIX_EPOCH};

    fn temp_tool_root() -> PathBuf {
        let stamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("time before epoch")
            .as_nanos();
        let root = std::env::temp_dir().join(format!("repo_contract_enforcer_tool_rules_{stamp}"));
        fs::create_dir_all(&root).expect("create temp tool root");
        root
    }

    fn write_minimal_tool(tool_root: &std::path::Path, tool_name: &str) {
        fs::create_dir_all(tool_root.join("src")).expect("create src dir");
        fs::write(
            tool_root.join("Cargo.toml"),
            format!("[package]\nname = \"{tool_name}\"\nversion = \"0.1.0\"\nedition = \"2021\"\n"),
        )
        .expect("write Cargo.toml");
        fs::write(tool_root.join("src/main.rs"), "fn main() {}\n").expect("write main.rs");
    }

    #[test]
    fn tool_detects_multiple_primary_items_in_single_file() {
        let tool_root = temp_tool_root();
        let tool_name = "multi_primary_tool";
        write_minimal_tool(&tool_root, tool_name);

        fs::write(
            tool_root.join("src/multi.rs"),
            "pub struct Alpha;\npub struct Beta;\n",
        )
        .expect("write multi.rs");

        let violations = ToolRules::evaluate(
            &tool_root,
            PathClassification::Tool,
            EnforcementMode::Strict,
        );

        assert!(violations.iter().any(|v| {
            v.violation_code == ViolationCode::CratePrimaryItemContractViolation
                && v.path.ends_with("src/multi.rs")
                && v.message
                    .contains("multiple primary struct/enum/trait declarations")
        }));
    }
}
