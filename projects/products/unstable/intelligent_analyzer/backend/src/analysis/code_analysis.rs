use super::finding::Finding;
use super::finding_kind::FindingKind;
use super::scope_resolver::ScopeResolver;
use super::severity::Severity;
use super::symbol_extractor::SymbolExtractor;

/// Performs structural code analysis on source text.
pub struct CodeAnalysis;

impl CodeAnalysis {
    pub fn analyze(source: &str) -> Vec<Finding> {
        let mut findings = Vec::new();
        let symbols = SymbolExtractor::extract(source);
        let scopes = ScopeResolver::resolve(source);

        // Detect unused variables: defined in scope but never referenced later.
        for (name, def_line) in &symbols.definitions {
            if !symbols.references.iter().any(|(n, _)| n == name) {
                findings.push(Finding::new(
                    FindingKind::UnusedVariable,
                    Severity::Warning,
                    *def_line,
                    format!("variable `{name}` is defined but never used"),
                ));
            }
        }

        // Detect undefined symbols: referenced but never defined.
        for (name, ref_line) in &symbols.references {
            if !symbols.definitions.iter().any(|(n, _)| n == name) {
                findings.push(Finding::new(
                    FindingKind::UndefinedSymbol,
                    Severity::Error,
                    *ref_line,
                    format!("symbol `{name}` is used but never defined"),
                ));
            }
        }

        // Detect scope violations via mismatched braces.
        if !scopes.balanced {
            findings.push(Finding::new(
                FindingKind::ScopeViolation,
                Severity::Error,
                scopes.mismatch_line.unwrap_or(1),
                "mismatched braces detected".to_string(),
            ));
        }

        findings
    }
}
