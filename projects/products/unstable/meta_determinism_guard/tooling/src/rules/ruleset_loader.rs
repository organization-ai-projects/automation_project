use anyhow::Result;
use crate::rules::ruleset::Ruleset;

pub fn load_from_file(path: &str) -> Result<Ruleset> {
    let content = std::fs::read_to_string(path)?;
    let ruleset: Ruleset = serde_json::from_str(&content)?;
    Ok(ruleset)
}
