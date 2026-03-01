#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LayeringRules;

impl LayeringRules {
    pub fn evaluate(
        product_dir: &std::path::Path,
        product_name: &str,
        scope: crate::config::path_classification::PathClassification,
        mode: crate::config::enforcement_mode::EnforcementMode,
    ) -> Vec<crate::report::violation::Violation> {
        use crate::report::violation_code::ViolationCode;
        use crate::rules::rule_id::RuleId;
        use crate::scan::file_scanner::FileScanner;
        use crate::scan::rust_parser::RustParser;

        let mut out = Vec::new();
        let ui = product_dir.join("ui");
        if !ui.exists() {
            return out;
        }

        let pattern = format!("use {product_name}_backend::");
        let rs_files = FileScanner::gather_rs_files(&ui);
        for file in rs_files {
            let txt = std::fs::read_to_string(&file).unwrap_or_default();
            if txt.contains(&pattern) {
                out.push(make_violation(
                    RuleId::Layering,
                    ViolationCode::LayerUiImportsBackend,
                    scope,
                    &file,
                    "ui must not import backend internals",
                    mode,
                    true,
                    RustParser::first_line_of(&txt, &pattern),
                ));
            }
        }

        for bad_module in ["sim", "engine", "scheduler", "solver"] {
            for path in FileScanner::gather_named_entries(&ui, bad_module) {
                out.push(make_violation(
                    RuleId::Layering,
                    ViolationCode::LayerUiSuspectDomainLogic,
                    scope,
                    &path,
                    "ui contains suspicious domain logic module name",
                    mode,
                    false,
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
