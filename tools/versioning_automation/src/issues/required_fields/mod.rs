mod contract_values;
mod gh_issue_payload;
mod key;
mod labels;
mod parser;
mod validation;

#[cfg(test)]
mod tests;

use std::process::Command;

use contract_values::ContractValues;
use gh_issue_payload::GhIssuePayload;
use key::ContractKey;
use parser::{body_has_section, extract_field_value, trim_whitespace};
use validation::ValidationEntry;

pub(crate) fn validate_title(
    title: &str,
    labels_raw: &str,
) -> Result<Vec<ValidationEntry>, String> {
    let profile = labels::profile_for_labels(labels_raw);
    let contract = ContractValues::load(profile)?;
    Ok(validate_title_with_contract(title, &contract))
}

pub(crate) fn validate_body(body: &str, labels_raw: &str) -> Result<Vec<ValidationEntry>, String> {
    let profile = labels::profile_for_labels(labels_raw);
    let contract = ContractValues::load(profile)?;
    Ok(validate_body_with_contract(body, &contract))
}

pub(crate) fn validate_content(
    title: &str,
    body: &str,
    labels_raw: &str,
) -> Result<Vec<ValidationEntry>, String> {
    let profile = labels::profile_for_labels(labels_raw);
    let contract = ContractValues::load(profile)?;

    let mut entries = validate_title_with_contract(title, &contract);
    entries.extend(validate_body_with_contract(body, &contract));
    Ok(entries)
}

pub(crate) fn non_compliance_reason_from_content(
    title: &str,
    body: &str,
    labels_raw: &str,
) -> Result<String, String> {
    if labels::labels_include(labels_raw, "issue-required-missing") {
        return Ok("label issue-required-missing is set on issue".to_string());
    }

    let validations = validate_content(title, body, labels_raw)?;
    if let Some(entry) = validations.first() {
        return Ok(entry.message.clone());
    }

    Ok(String::new())
}

pub(crate) fn fetch_non_compliance_reason(
    issue: &str,
    repo: Option<&str>,
) -> Result<String, String> {
    let mut cmd = Command::new("gh");
    cmd.arg("issue")
        .arg("view")
        .arg(issue)
        .arg("--json")
        .arg("labels,title,body");
    if let Some(repo_name) = repo.filter(|value| !value.trim().is_empty()) {
        cmd.arg("-R").arg(repo_name);
    }

    let output = cmd.output().map_err(|err| err.to_string())?;
    if !output.status.success() {
        return Ok(String::new());
    }

    let payload = String::from_utf8_lossy(&output.stdout).to_string();
    let parsed = common_json::from_json_str::<GhIssuePayload>(&payload)
        .map_err(|err| format!("failed to parse issue payload: {err}"))?;

    let labels_raw = parsed.join_labels();
    let title = parsed.title.unwrap_or_default();
    let body = parsed.body.unwrap_or_default();

    non_compliance_reason_from_content(&title, &body, &labels_raw)
}

fn validate_title_with_contract(title: &str, contract: &ContractValues) -> Vec<ValidationEntry> {
    if contract.title_regex.trim().is_empty() {
        return vec![ValidationEntry::new(
            "invalid_contract",
            "title".to_string(),
            format!("Missing contract key: {}", contract.title_regex_key),
        )];
    }

    let Ok(regex) = regex::Regex::new(&contract.title_regex) else {
        return vec![ValidationEntry::new(
            "invalid_contract",
            "title".to_string(),
            format!("Invalid title regex in contract: {}", contract.title_regex),
        )];
    };

    if regex.is_match(title) {
        Vec::new()
    } else {
        vec![ValidationEntry::new(
            "invalid_title",
            "title".to_string(),
            format!("Title must match regex: {}", contract.title_regex),
        )]
    }
}

fn validate_body_with_contract(body: &str, contract: &ContractValues) -> Vec<ValidationEntry> {
    let mut entries = Vec::new();

    for raw_section in contract.required_sections.lines() {
        let section = trim_whitespace(raw_section);
        if section.is_empty() {
            continue;
        }
        if !body_has_section(body, &section) {
            entries.push(ValidationEntry::new(
                "missing_section",
                section.clone(),
                format!("Missing required section: {section}"),
            ));
        }
    }

    for raw_rule in contract.required_fields.lines() {
        if raw_rule.trim().is_empty() {
            continue;
        }
        let mut parts = raw_rule.split('\t');
        let field_name = trim_whitespace(parts.next().unwrap_or_default());
        let field_regex = trim_whitespace(parts.next().unwrap_or_default());
        let field_help = trim_whitespace(parts.next().unwrap_or_default());

        if field_name.is_empty() || field_regex.is_empty() {
            continue;
        }

        let field_value = trim_whitespace(&extract_field_value(body, &field_name));
        if field_value.is_empty() {
            entries.push(ValidationEntry::new(
                "missing_field",
                field_name.clone(),
                format!("Missing required field: {field_name}:"),
            ));
            continue;
        }

        let Ok(regex) = regex::Regex::new(&field_regex) else {
            entries.push(ValidationEntry::new(
                "invalid_contract",
                field_name.clone(),
                format!("Invalid regex for field {field_name}: {field_regex}"),
            ));
            continue;
        };

        if !regex.is_match(&field_value) {
            entries.push(ValidationEntry::new(
                "invalid_field",
                field_name.clone(),
                format!("Invalid {field_name}: '{field_value}' (expected: {field_help})"),
            ));
        }
    }

    entries
}

pub(crate) fn contract_key_for_profile(profile: &str, base_key: ContractKey) -> String {
    key::contract_key_for_profile(profile, base_key)
}
