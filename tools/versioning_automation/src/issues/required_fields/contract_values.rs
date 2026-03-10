use std::path::{Path, PathBuf};
use std::process::Command;

use crate::issues::required_fields::key::ContractKey;

#[derive(Debug, Clone)]
pub(crate) struct ContractValues {
    pub(crate) title_regex_key: String,
    pub(crate) title_regex: String,
    pub(crate) required_sections: String,
    pub(crate) required_fields: String,
}

impl ContractValues {
    pub(crate) fn load(profile: &str) -> Result<Self, String> {
        let title_regex_key = crate::issues::required_fields::contract_key_for_profile(
            profile,
            ContractKey::TitleRegex,
        );
        let required_sections_key = crate::issues::required_fields::contract_key_for_profile(
            profile,
            ContractKey::RequiredSections,
        );
        let required_fields_key = crate::issues::required_fields::contract_key_for_profile(
            profile,
            ContractKey::RequiredFields,
        );

        let path = contract_path();
        let values = shell_load_contract_values(
            &path,
            &title_regex_key,
            &required_sections_key,
            &required_fields_key,
        )?;

        Ok(Self {
            title_regex_key,
            title_regex: values.0,
            required_sections: values.1,
            required_fields: values.2,
        })
    }
}

fn contract_path() -> PathBuf {
    let mut resolved_root = String::new();

    if let Ok(output) = Command::new("git")
        .arg("rev-parse")
        .arg("--show-toplevel")
        .output()
        && output.status.success()
    {
        resolved_root = String::from_utf8_lossy(&output.stdout).trim().to_string();
    }

    if !resolved_root.is_empty() {
        let candidate = Path::new(&resolved_root).join(".github/issue_required_fields.conf");
        if candidate.exists() {
            return candidate;
        }
    }

    PathBuf::from(".github/issue_required_fields.conf")
}

fn shell_load_contract_values(
    contract_path: &Path,
    title_regex_key: &str,
    required_sections_key: &str,
    required_fields_key: &str,
) -> Result<(String, String, String), String> {
    if !contract_path.exists() {
        return Err(format!(
            "Missing issue contract file: {}",
            contract_path.display()
        ));
    }

    let script = r#"
source "$1" >/dev/null 2>&1 || exit 98
printf '%s\x1f%s\x1f%s' "${!2-}" "${!3-}" "${!4-}"
"#;

    let output = Command::new("bash")
        .arg("-c")
        .arg(script)
        .arg("bash")
        .arg(contract_path)
        .arg(title_regex_key)
        .arg(required_sections_key)
        .arg(required_fields_key)
        .output()
        .map_err(|err| format!("failed to load contract values: {err}"))?;

    if !output.status.success() {
        return Err("issue contract could not be loaded".to_string());
    }

    let stdout = String::from_utf8_lossy(&output.stdout).to_string();
    let mut parts = stdout.split('\u{1f}');

    let title_regex = parts.next().unwrap_or_default().to_string();
    let required_sections = parts.next().unwrap_or_default().to_string();
    let required_fields = parts.next().unwrap_or_default().to_string();

    Ok((title_regex, required_sections, required_fields))
}
