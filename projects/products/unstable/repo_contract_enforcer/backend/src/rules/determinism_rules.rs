use crate::{config, reports, rules, scan};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DeterminismRules;

impl DeterminismRules {
    pub fn evaluate(
        product_dir: &std::path::Path,
        scope: config::path_classification::PathClassification,
        mode: config::enforcement_mode::EnforcementMode,
    ) -> Vec<reports::violation::Violation> {
        use reports::violation_code::ViolationCode;
        use rules::rule_id::RuleId;
        use scan::file_scanner::FileScanner;
        use scan::rust_parser::RustParser;

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
                    (scope, mode),
                    &file,
                    "wall-clock API usage is forbidden",
                    (
                        true,
                        RustParser::first_line_of_any(&txt, &["SystemTime", "Instant"]),
                    ),
                ));
            }
            if txt.contains("chrono") {
                out.push(make_violation(
                    RuleId::Determinism,
                    ViolationCode::DetForbiddenTimeDep,
                    (scope, mode),
                    &file,
                    "chrono usage is forbidden in backend core",
                    (true, RustParser::first_line_of(&txt, "chrono")),
                ));
            }

            if !file.to_string_lossy().contains("/protocol/") && txt.contains("println!") {
                out.push(make_violation(
                    RuleId::Determinism,
                    ViolationCode::DetStdoutUsage,
                    (scope, mode),
                    &file,
                    "println! outside protocol module is forbidden",
                    (true, RustParser::first_line_of(&txt, "println!")),
                ));
            }
            if !file.to_string_lossy().contains("/protocol/")
                && (txt.contains("print!(")
                    || txt.contains("eprint!(")
                    || txt.contains("eprintln!("))
            {
                out.push(make_violation(
                    RuleId::Determinism,
                    ViolationCode::DetStdoutUsage,
                    (scope, mode),
                    &file,
                    "print/eprint macros outside protocol module are forbidden",
                    (
                        true,
                        RustParser::first_line_of_any(&txt, &["print!(", "eprint!(", "eprintln!("]),
                    ),
                ));
            }
            if !file.to_string_lossy().contains("/protocol/")
                && (txt.contains("std::io::stdout")
                    || txt.contains("std::io::stderr")
                    || txt.contains("io::stdout(")
                    || txt.contains("io::stderr("))
            {
                out.push(make_violation(
                    RuleId::Determinism,
                    ViolationCode::DetStdioUsage,
                    (scope, mode),
                    &file,
                    "direct stdout/stderr IO outside protocol module is forbidden",
                    (
                        true,
                        RustParser::first_line_of_any(
                            &txt,
                            &[
                                "std::io::stdout",
                                "std::io::stderr",
                                "io::stdout(",
                                "io::stderr(",
                            ],
                        ),
                    ),
                ));
            }

            if txt.contains("rand::") || txt.contains("thread_rng(") || txt.contains("rand(") {
                out.push(make_violation(
                    RuleId::Determinism,
                    ViolationCode::DetNondeterministicRngHeuristic,
                    (scope, mode),
                    &file,
                    "possible nondeterministic RNG usage detected",
                    (
                        false,
                        RustParser::first_line_of_any(&txt, &["rand::", "thread_rng(", "rand("]),
                    ),
                ));
            }
            if txt.contains(".unwrap()")
                || txt.contains(".expect(")
                || txt.contains("unwrap_unchecked(")
            {
                out.push(make_violation(
                    RuleId::Determinism,
                    ViolationCode::DetUnwrapRisk,
                    (scope, mode),
                    &file,
                    "unwrap/expect usage may panic at runtime",
                    (
                        true,
                        RustParser::first_line_of_any(
                            &txt,
                            &[".unwrap()", ".expect(", "unwrap_unchecked("],
                        ),
                    ),
                ));
            }
            if txt.contains("panic!(")
                || txt.contains("todo!(")
                || txt.contains("unimplemented!(")
                || txt.contains("unreachable!(")
            {
                out.push(make_violation(
                    RuleId::Determinism,
                    ViolationCode::DetPanicRisk,
                    (scope, mode),
                    &file,
                    "panic-like macros are forbidden in backend product code",
                    (
                        true,
                        RustParser::first_line_of_any(
                            &txt,
                            &["panic!(", "todo!(", "unimplemented!(", "unreachable!("],
                        ),
                    ),
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
