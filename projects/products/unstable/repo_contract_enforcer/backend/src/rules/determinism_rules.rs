#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DeterminismRules;

impl DeterminismRules {
    pub fn evaluate(
        product_dir: &std::path::Path,
        scope: crate::config::path_classification::PathClassification,
        mode: crate::config::enforcement_mode::EnforcementMode,
    ) -> Vec<crate::report::violation::Violation> {
        use crate::report::violation_code::ViolationCode;
        use crate::rules::rule_id::RuleId;
        use crate::scan::file_scanner::FileScanner;
        use crate::scan::rust_parser::RustParser;

        let mut out = Vec::new();
        let backend = product_dir.join("backend");
        if !backend.exists() {
            return out;
        }

        let rs_files = FileScanner::gather_rs_files(&backend);
        for file in rs_files {
            let txt = std::fs::read_to_string(&file).unwrap_or_default();
            if txt.contains("SystemTime") || txt.contains("Instant") {
                out.push(make_violation(
                    RuleId::Determinism,
                    ViolationCode::DetWallClockUsage,
                    scope,
                    &file,
                    "wall-clock API usage is forbidden",
                    mode,
                    true,
                    RustParser::first_line_of_any(&txt, &["SystemTime", "Instant"]),
                ));
            }
            if txt.contains("chrono") {
                out.push(make_violation(
                    RuleId::Determinism,
                    ViolationCode::DetForbiddenTimeDep,
                    scope,
                    &file,
                    "chrono usage is forbidden in backend core",
                    mode,
                    true,
                    RustParser::first_line_of(&txt, "chrono"),
                ));
            }

            if !file.to_string_lossy().contains("/protocol/") && txt.contains("println!") {
                out.push(make_violation(
                    RuleId::Determinism,
                    ViolationCode::DetStdoutUsage,
                    scope,
                    &file,
                    "println! outside protocol module is forbidden",
                    mode,
                    true,
                    RustParser::first_line_of(&txt, "println!"),
                ));
            }

            if txt.contains("rand::") || txt.contains("thread_rng(") || txt.contains("rand(") {
                out.push(make_violation(
                    RuleId::Determinism,
                    ViolationCode::DetNondeterministicRngHeuristic,
                    scope,
                    &file,
                    "possible nondeterministic RNG usage detected",
                    mode,
                    false,
                    RustParser::first_line_of_any(&txt, &["rand::", "thread_rng(", "rand("]),
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
