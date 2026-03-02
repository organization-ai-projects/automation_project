use crate::{config, reports, rules, scan};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LayeringRules;

impl LayeringRules {
    pub fn evaluate(
        product_dir: &std::path::Path,
        product_name: &str,
        scope: config::path_classification::PathClassification,
        mode: config::enforcement_mode::EnforcementMode,
    ) -> Vec<reports::violation::Violation> {
        use reports::violation_code::ViolationCode;
        use rules::rule_id::RuleId;
        use scan::file_scanner::FileScanner;
        use scan::rust_parser::RustParser;

        let mut out = Vec::new();
        let ui = product_dir.join("ui");
        if !ui.exists() {
            return out;
        }

        let backend_crate_name = format!("{product_name}_backend");
        let rs_files = FileScanner::gather_rs_files(&ui);
        for file in rs_files {
            let txt = std::fs::read_to_string(&file).unwrap_or_default();
            if RustParser::imports_backend_crate(&txt, &backend_crate_name) {
                out.push(make_violation(
                    RuleId::Layering,
                    ViolationCode::LayerUiImportsBackend,
                    (scope, mode),
                    &file,
                    "ui must not import backend internals",
                    (true, None),
                ));
            }
        }

        for bad_module in ["sim", "engine", "scheduler", "solver"] {
            for path in FileScanner::gather_named_entries(&ui, bad_module) {
                out.push(make_violation(
                    RuleId::Layering,
                    ViolationCode::LayerUiSuspectDomainLogic,
                    (scope, mode),
                    &path,
                    "ui contains suspicious domain logic module name",
                    (false, None),
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
