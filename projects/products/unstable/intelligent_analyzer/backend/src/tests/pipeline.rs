use crate::config::AnalyzerConfig;
use crate::pipeline::run_pipeline;
use crate::report::AnalysisReport;

#[test]
fn pipeline_produces_result_for_simple_source() {
    let cfg = AnalyzerConfig::default();
    let source = "let x = 42;\nlet y = 10;\nprintln!(\"{}\", y);\n";
    let result = run_pipeline(&cfg, source);
    assert!(result.is_ok());
    let result = result.unwrap();
    assert!(!result.source_hash.is_empty());
}

#[test]
fn pipeline_with_analysis_disabled() {
    let cfg = AnalyzerConfig {
        enable_analysis: false,
        enable_linting: false,
        enable_neurosymbolic: false,
        confidence_threshold: 0.6,
    };
    let source = "let x = 42;\n";
    let result = run_pipeline(&cfg, source).unwrap();
    assert!(result.analysis_findings.is_empty());
    assert!(result.lint_findings.is_empty());
    assert!(result.insights.is_empty());
}

#[test]
fn report_from_pipeline_result() {
    let cfg = AnalyzerConfig {
        enable_analysis: true,
        enable_linting: true,
        enable_neurosymbolic: false,
        confidence_threshold: 0.6,
    };
    let source = "let unused = 1;\n";
    let result = run_pipeline(&cfg, source).unwrap();
    let report = AnalysisReport::from_result(&result);
    assert_eq!(report.total_findings, report.findings.len());
    assert_eq!(report.total_insights, report.insights.len());
}

#[test]
fn pipeline_deterministic_hash() {
    let cfg = AnalyzerConfig {
        enable_analysis: false,
        enable_linting: false,
        enable_neurosymbolic: false,
        confidence_threshold: 0.6,
    };
    let source = "fn main() {}\n";
    let r1 = run_pipeline(&cfg, source).unwrap();
    let r2 = run_pipeline(&cfg, source).unwrap();
    assert_eq!(r1.source_hash, r2.source_hash);
}

#[test]
fn default_config_enables_all() {
    let cfg = AnalyzerConfig::default();
    assert!(cfg.enable_analysis);
    assert!(cfg.enable_linting);
    assert!(cfg.enable_neurosymbolic);
}
