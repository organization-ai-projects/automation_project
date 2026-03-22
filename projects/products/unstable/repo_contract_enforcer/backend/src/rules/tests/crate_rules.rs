//! projects/products/unstable/repo_contract_enforcer/backend/src/rules/tests/crate_rules.rs

use std::{
    env, fs,
    path::{self, PathBuf},
    time::{SystemTime, UNIX_EPOCH},
};

use crate::{
    config::{enforcement_mode::EnforcementMode, path_classification::PathClassification},
    reports::violation_code::ViolationCode,
    rules::crate_rules::CrateRules,
};
fn temp_product_root() -> PathBuf {
    let stamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("time before epoch")
        .as_nanos();
    let root = env::temp_dir().join(format!("repo_contract_enforcer_crate_rules_{stamp}"));
    fs::create_dir_all(&root).expect("create temp product root");
    root
}

fn write_minimal_bin_crate(crate_root: &path::Path, crate_name: &str) {
    fs::create_dir_all(crate_root.join("src")).expect("create crate src dir");
    fs::write(
        crate_root.join("Cargo.toml"),
        format!("[package]\nname = \"{crate_name}\"\nversion = \"0.1.0\"\nedition = \"2021\"\n"),
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
    fs::create_dir_all(request_rs.parent().expect("request parent")).expect("mkdir request parent");
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
    fs::create_dir_all(request_rs.parent().expect("request parent")).expect("mkdir request parent");
    fs::write(
        &request_rs,
        "pub struct Request;\n\nimpl Request {\n    pub fn is_valid(&self) -> bool { true }\n}\n",
    )
    .expect("write request.rs");

    let request_test = backend.join("src/protocol/tests/request.rs");
    fs::create_dir_all(request_test.parent().expect("request test parent"))
        .expect("mkdir request test parent");
    fs::write(&request_test, "#[test]\nfn request_contract() {}\n").expect("write request test");

    let violations = CrateRules::evaluate(
        &product_root,
        product_name,
        PathClassification::Stable,
        EnforcementMode::Strict,
    );

    assert!(!violations.iter().any(|v| {
        matches!(
            v.violation_code,
            ViolationCode::CrateMissingPairedTestFile | ViolationCode::CratePairedTestNotUnitStyle
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
    fs::create_dir_all(request_rs.parent().expect("request parent")).expect("mkdir request parent");
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
    fs::create_dir_all(record_rs.parent().expect("record parent")).expect("mkdir record parent");
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
    fs::create_dir_all(domain_mod.parent().expect("domain parent")).expect("create domain parent");
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
