use crate::tooling::ruleset::Ruleset;
use crate::tooling::tooling_error::ToolingError;

pub fn load_from_file(path: &str) -> Result<Ruleset, ToolingError> {
    let content = std::fs::read_to_string(path).map_err(ToolingError::from)?;
    let ruleset: Ruleset = common_json::from_json_str(&content).map_err(ToolingError::from)?;
    validate_ruleset(&ruleset)?;
    Ok(ruleset)
}

fn validate_ruleset(ruleset: &Ruleset) -> Result<(), ToolingError> {
    if ruleset.name.trim().is_empty() {
        return Err(ToolingError::Validation(
            "ruleset name must not be empty".to_string(),
        ));
    }
    if ruleset.rules.is_empty() {
        return Err(ToolingError::Validation(
            "ruleset must contain at least one rule".to_string(),
        ));
    }

    for (index, rule) in ruleset.rules.iter().enumerate() {
        if rule.name.trim().is_empty() {
            return Err(ToolingError::Validation(format!(
                "rule #{index} has an empty name"
            )));
        }
        if rule.pattern.trim().is_empty() {
            return Err(ToolingError::Validation(format!(
                "rule '{}' has an empty pattern",
                rule.name
            )));
        }
    }

    Ok(())
}
