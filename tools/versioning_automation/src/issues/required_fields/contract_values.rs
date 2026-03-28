//! tools/versioning_automation/src/issues/required_fields/contract_values.rs
use std::path::{Path, PathBuf};
use std::{collections::HashMap, fs};

use crate::git_cli;
use crate::issues::Key;

#[derive(Debug, Clone)]
pub(crate) struct ContractValues {
    pub(crate) title_regex_key: String,
    pub(crate) title_regex: String,
    pub(crate) required_sections: String,
    pub(crate) required_fields: String,
}

impl ContractValues {
    pub(crate) fn load(profile: &str) -> Result<Self, String> {
        let title_regex_key = Key::contract_key_for_profile(profile, Key::TitleRegex);
        let required_sections_key = Key::contract_key_for_profile(profile, Key::RequiredSections);
        let required_fields_key = Key::contract_key_for_profile(profile, Key::RequiredFields);

        let path = contract_path();
        let values = load_contract_values_from_file(
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
    if let Ok(root) = git_cli::output_trim(&["rev-parse", "--show-toplevel"]) {
        let candidate = Path::new(&root).join(".github/issue_required_fields.conf");
        if candidate.exists() {
            return candidate;
        }
    }

    if let Some(repo_root) = manifest_repo_root() {
        let candidate = repo_root.join(".github/issue_required_fields.conf");
        if candidate.exists() {
            return candidate;
        }
    }

    PathBuf::from(".github/issue_required_fields.conf")
}

fn manifest_repo_root() -> Option<PathBuf> {
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    manifest_dir.parent()?.parent().map(Path::to_path_buf)
}

fn load_contract_values_from_file(
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

    let source = fs::read_to_string(contract_path).map_err(|err| {
        format!(
            "failed to read issue contract file {}: {err}",
            contract_path.display()
        )
    })?;
    let values = parse_contract_assignments(&source);

    let title_regex = values.get(title_regex_key).cloned().unwrap_or_default();
    let required_sections = values
        .get(required_sections_key)
        .cloned()
        .unwrap_or_default();
    let required_fields = values.get(required_fields_key).cloned().unwrap_or_default();

    Ok((title_regex, required_sections, required_fields))
}

fn parse_contract_assignments(source: &str) -> HashMap<String, String> {
    let mut out = HashMap::new();
    for raw_line in source.lines() {
        let line = raw_line.trim();
        if line.is_empty() || line.starts_with('#') {
            continue;
        }

        let Some((key_raw, value_raw)) = line.split_once('=') else {
            continue;
        };
        let key = key_raw.trim();
        if key.is_empty() {
            continue;
        }
        let value = decode_contract_value(value_raw.trim());
        out.insert(key.to_string(), value);
    }
    out
}

fn decode_contract_value(raw: &str) -> String {
    if raw.starts_with("$'") && raw.ends_with('\'') && raw.len() >= 3 {
        return decode_ansi_c_quoted(&raw[2..raw.len() - 1]);
    }
    if raw.starts_with('\'') && raw.ends_with('\'') && raw.len() >= 2 {
        return raw[1..raw.len() - 1].to_string();
    }
    if raw.starts_with('"') && raw.ends_with('"') && raw.len() >= 2 {
        return raw[1..raw.len() - 1].to_string();
    }
    raw.to_string()
}

fn decode_ansi_c_quoted(input: &str) -> String {
    let mut out = String::with_capacity(input.len());
    let mut chars = input.chars().peekable();
    while let Some(ch) = chars.next() {
        if ch != '\\' {
            out.push(ch);
            continue;
        }
        let Some(next) = chars.next() else {
            out.push('\\');
            break;
        };
        match next {
            'n' => out.push('\n'),
            'r' => out.push('\r'),
            't' => out.push('\t'),
            '\\' => out.push('\\'),
            '\'' => out.push('\''),
            '"' => out.push('"'),
            'a' => out.push('\u{7}'),
            'b' => out.push('\u{8}'),
            'f' => out.push('\u{c}'),
            'v' => out.push('\u{b}'),
            other => {
                out.push('\\');
                out.push(other);
            }
        }
    }
    out
}
