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

    pub fn evaluate_library_dependencies(
        repo_root: &std::path::Path,
        mode: config::enforcement_mode::EnforcementMode,
    ) -> Vec<reports::violation::Violation> {
        use reports::violation_code::ViolationCode;
        use rules::rule_id::RuleId;

        let libraries_root = repo_root.join("projects/libraries");
        if !libraries_root.is_dir() {
            return Vec::new();
        }

        let cargo_files = collect_library_cargo_files(&libraries_root);
        if cargo_files.is_empty() {
            return Vec::new();
        }

        let mut crate_layer_by_name = std::collections::HashMap::new();
        let mut cargo_by_name = std::collections::HashMap::new();
        for cargo in &cargo_files {
            let content = std::fs::read_to_string(cargo).unwrap_or_default();
            let Some(crate_name) = parse_package_name(&content) else {
                continue;
            };
            let Some(layer_rank) = library_layer_rank_from_path(cargo) else {
                continue;
            };
            crate_layer_by_name.insert(crate_name.clone(), layer_rank);
            cargo_by_name.insert(crate_name, cargo.clone());
        }

        let mut out = Vec::new();
        for cargo in &cargo_files {
            if library_layer_rank_from_path(cargo).is_some() {
                let crate_root = cargo.parent().unwrap_or(repo_root);
                let src_dir = crate_root.join("src");
                let lib_rs = src_dir.join("lib.rs");
                let main_rs = src_dir.join("main.rs");

                if main_rs.exists() {
                    out.push(make_violation(
                        RuleId::Layering,
                        ViolationCode::LayerLibraryHasMainRs,
                        (config::path_classification::PathClassification::Other, mode),
                        &main_rs,
                        "library crates must not define src/main.rs (libraries are lib-only)",
                        (true, None),
                    ));
                }

                if !lib_rs.exists() {
                    out.push(make_violation(
                        RuleId::Layering,
                        ViolationCode::LayerLibraryMissingLibRs,
                        (config::path_classification::PathClassification::Other, mode),
                        &lib_rs,
                        "library crates must define src/lib.rs",
                        (true, None),
                    ));
                }
            }

            let content = std::fs::read_to_string(cargo).unwrap_or_default();
            let Some(crate_name) = parse_package_name(&content) else {
                continue;
            };
            let Some(self_rank) = crate_layer_by_name.get(&crate_name) else {
                continue;
            };

            let deps = parse_dependency_crate_names(&content);
            for dep in deps {
                let Some(dep_rank) = crate_layer_by_name.get(&dep) else {
                    continue;
                };
                if dep_rank > self_rank {
                    out.push(make_violation(
                        RuleId::Layering,
                        ViolationCode::LayerLibraryDependsOnHigherLayer,
                        (config::path_classification::PathClassification::Other, mode),
                        cargo,
                        &format!(
                            "library layering violation: '{crate_name}' depends on higher-layer crate '{dep}'"
                        ),
                        (true, None),
                    ));
                }
            }
        }

        out
    }
}

fn collect_library_cargo_files(root: &std::path::Path) -> Vec<std::path::PathBuf> {
    let mut out = Vec::new();
    let mut stack = vec![root.to_path_buf()];
    while let Some(dir) = stack.pop() {
        let read_dir = match std::fs::read_dir(dir) {
            Ok(rd) => rd,
            Err(_) => continue,
        };
        for entry in read_dir.flatten() {
            let path = entry.path();
            if path.is_dir() {
                stack.push(path);
                continue;
            }
            if path.file_name().and_then(|s| s.to_str()) == Some("Cargo.toml") {
                out.push(path);
            }
        }
    }
    out.sort();
    out
}

fn library_layer_rank_from_path(cargo_toml: &std::path::Path) -> Option<u8> {
    let txt = cargo_toml.to_string_lossy();
    if txt.contains("/projects/libraries/core/") {
        return Some(0);
    }
    if txt.contains("/projects/libraries/layers/domain/") {
        return Some(1);
    }
    if txt.contains("/projects/libraries/layers/orchestration/") {
        return Some(2);
    }
    None
}

fn parse_package_name(cargo_toml_content: &str) -> Option<String> {
    let mut in_package = false;
    for raw in cargo_toml_content.lines() {
        let line = raw.trim();
        if line.starts_with('[') {
            in_package = line == "[package]";
            continue;
        }
        if !in_package || !line.starts_with("name") {
            continue;
        }
        let (_, value) = line.split_once('=')?;
        let value = value.trim().trim_matches('"');
        if value.is_empty() {
            return None;
        }
        return Some(value.to_string());
    }
    None
}

fn parse_dependency_crate_names(cargo_toml_content: &str) -> Vec<String> {
    let mut out = Vec::new();
    let mut in_dependencies = false;
    for raw in cargo_toml_content.lines() {
        let line = raw.trim();
        if line.is_empty() || line.starts_with('#') {
            continue;
        }

        if line.starts_with('[') {
            // [dependencies], [dev-dependencies], [build-dependencies], [target.'...'.dependencies]
            in_dependencies = line.contains("dependencies");
            continue;
        }
        if !in_dependencies {
            continue;
        }
        let Some((name, _)) = line.split_once('=') else {
            continue;
        };
        let name = name.trim().trim_matches('"');
        if name.is_empty() {
            continue;
        }
        out.push(name.to_string());
    }
    out
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
    use super::LayeringRules;
    use crate::config::enforcement_mode::EnforcementMode;
    use crate::reports::violation_code::ViolationCode;
    use std::fs;
    use std::path::PathBuf;
    use std::time::{SystemTime, UNIX_EPOCH};

    fn temp_repo_root() -> PathBuf {
        let stamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("time before epoch")
            .as_nanos();
        let root = std::env::temp_dir().join(format!("repo_contract_enforcer_layering_{stamp}"));
        fs::create_dir_all(&root).expect("create temp repo root");
        root
    }

    fn write_library_cargo(repo_root: &std::path::Path, rel: &str, name: &str, deps: &[&str]) {
        let path = repo_root.join(rel).join("Cargo.toml");
        fs::create_dir_all(path.parent().expect("cargo parent")).expect("create cargo parent");
        let mut body = format!(
            "[package]\nname = \"{name}\"\nversion = \"0.1.0\"\nedition = \"2024\"\n\n[dependencies]\n"
        );
        for dep in deps {
            body.push_str(&format!("{dep} = {{ workspace = true }}\n"));
        }
        fs::write(path, body).expect("write Cargo.toml");
    }

    #[test]
    fn libraries_forbid_dependency_to_higher_layer() {
        let root = temp_repo_root();
        write_library_cargo(
            &root,
            "projects/libraries/core/foundation/core_a",
            "core_a",
            &["domain_a"],
        );
        write_library_cargo(
            &root,
            "projects/libraries/layers/domain/domain_a",
            "domain_a",
            &[],
        );

        let violations =
            LayeringRules::evaluate_library_dependencies(&root, EnforcementMode::Strict);
        assert!(violations.iter().any(|v| {
            v.violation_code == ViolationCode::LayerLibraryDependsOnHigherLayer
                && v.path
                    .ends_with("projects/libraries/core/foundation/core_a/Cargo.toml")
        }));
    }

    #[test]
    fn libraries_allow_dependency_to_same_or_lower_layer() {
        let root = temp_repo_root();
        write_library_cargo(
            &root,
            "projects/libraries/core/foundation/core_a",
            "core_a",
            &[],
        );
        write_library_cargo(
            &root,
            "projects/libraries/layers/domain/domain_a",
            "domain_a",
            &["core_a"],
        );
        write_library_cargo(
            &root,
            "projects/libraries/layers/orchestration/orch_a",
            "orch_a",
            &["domain_a", "core_a"],
        );

        let violations =
            LayeringRules::evaluate_library_dependencies(&root, EnforcementMode::Strict);
        assert!(
            !violations
                .iter()
                .any(|v| v.violation_code == ViolationCode::LayerLibraryDependsOnHigherLayer)
        );
    }

    #[test]
    fn libraries_forbid_main_rs_entrypoint() {
        let root = temp_repo_root();
        write_library_cargo(
            &root,
            "projects/libraries/layers/domain/domain_a",
            "domain_a",
            &[],
        );
        let src_dir = root.join("projects/libraries/layers/domain/domain_a/src");
        fs::create_dir_all(&src_dir).expect("create src");
        fs::write(src_dir.join("lib.rs"), "pub struct DomainA;\n").expect("write lib.rs");
        fs::write(src_dir.join("main.rs"), "fn main() {}\n").expect("write main.rs");

        let violations =
            LayeringRules::evaluate_library_dependencies(&root, EnforcementMode::Strict);
        assert!(violations.iter().any(|v| {
            v.violation_code == ViolationCode::LayerLibraryHasMainRs
                && v.path
                    .ends_with("projects/libraries/layers/domain/domain_a/src/main.rs")
        }));
    }

    #[test]
    fn libraries_require_lib_rs() {
        let root = temp_repo_root();
        write_library_cargo(
            &root,
            "projects/libraries/core/foundation/core_a",
            "core_a",
            &[],
        );
        let src_dir = root.join("projects/libraries/core/foundation/core_a/src");
        fs::create_dir_all(&src_dir).expect("create src");
        // intentionally missing src/lib.rs

        let violations =
            LayeringRules::evaluate_library_dependencies(&root, EnforcementMode::Strict);
        assert!(violations.iter().any(|v| {
            v.violation_code == ViolationCode::LayerLibraryMissingLibRs
                && v.path
                    .ends_with("projects/libraries/core/foundation/core_a/src/lib.rs")
        }));
    }
}
