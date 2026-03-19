//! projects/products/stable/platform_ide/backend/src/app/tests/ide_config.rs
use crate::app::IdeConfig;

#[test]
fn ide_config_has_non_empty_platform_url() {
    let config = IdeConfig::from_env();
    assert!(!config.platform_url.trim().is_empty());
}
