use crate::{
    gh_cli,
    issues::required_fields::{
        ContractValues, GhIssuePayload, body_has_section, extract_field_value, labels,
        trim_whitespace,
    },
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct Validation {
    pub(crate) code: String,
    pub(crate) field: String,
    pub(crate) message: String,
}

impl Validation {
    pub(crate) fn new(code: &str, field: String, message: String) -> Self {
        Self {
            code: code.to_string(),
            field,
            message,
        }
    }

    pub(crate) fn as_pipe_line(&self) -> String {
        format!("{}|{}|{}", self.code, self.field, self.message)
    }

    pub(crate) fn validate_title(title: &str, labels_raw: &str) -> Result<Vec<Self>, String> {
        let profile = labels::profile_for_labels(labels_raw);
        let contract = ContractValues::load(profile)?;
        Ok(Self::validate_title_with_contract(title, &contract))
    }

    pub(crate) fn validate_body(body: &str, labels_raw: &str) -> Result<Vec<Self>, String> {
        let profile = labels::profile_for_labels(labels_raw);
        let contract = ContractValues::load(profile)?;
        Ok(Self::validate_body_with_contract(body, &contract))
    }

    pub(crate) fn validate_content(
        title: &str,
        body: &str,
        labels_raw: &str,
    ) -> Result<Vec<Self>, String> {
        let profile = labels::profile_for_labels(labels_raw);
        let contract = ContractValues::load(profile)?;

        let mut entries = Self::validate_title_with_contract(title, &contract);
        entries.extend(Self::validate_body_with_contract(body, &contract));
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

        let validations = Self::validate_content(title, body, labels_raw)?;
        if let Some(entry) = validations.first() {
            return Ok(entry.message.clone());
        }

        Ok(String::new())
    }

    pub(crate) fn fetch_non_compliance_reason(
        issue: &str,
        repo: Option<&str>,
    ) -> Result<String, String> {
        let mut args = vec![
            "issue".to_string(),
            "view".to_string(),
            issue.to_string(),
            "--json".to_string(),
            "labels,title,body".to_string(),
        ];
        if let Some(repo_name) = repo.filter(|value| !value.trim().is_empty()) {
            args.push("-R".to_string());
            args.push(repo_name.to_string());
        }

        let borrowed = args.iter().map(String::as_str).collect::<Vec<&str>>();
        let Ok(payload) = gh_cli::output_preserve(&borrowed) else {
            return Ok(String::new());
        };
        let parsed = common_json::from_json_str::<GhIssuePayload>(&payload)
            .map_err(|err| format!("failed to parse issue payload: {err}"))?;

        let labels_raw = parsed.join_labels();
        let title = parsed.title.unwrap_or_default();
        let body = parsed.body.unwrap_or_default();

        Self::non_compliance_reason_from_content(&title, &body, &labels_raw)
    }

    fn validate_title_with_contract(title: &str, contract: &ContractValues) -> Vec<Self> {
        if contract.title_regex.trim().is_empty() {
            return vec![Self::new(
                "invalid_contract",
                "title".to_string(),
                format!("Missing contract key: {}", contract.title_regex_key),
            )];
        }

        let Ok(regex) = regex::Regex::new(&contract.title_regex) else {
            return vec![Self::new(
                "invalid_contract",
                "title".to_string(),
                format!("Invalid title regex in contract: {}", contract.title_regex),
            )];
        };

        if regex.is_match(title) {
            Vec::new()
        } else {
            vec![Self::new(
                "invalid_title",
                "title".to_string(),
                format!("Title must match regex: {}", contract.title_regex),
            )]
        }
    }

    fn validate_body_with_contract(body: &str, contract: &ContractValues) -> Vec<Self> {
        let mut entries = Vec::new();

        for raw_section in contract.required_sections.lines() {
            let section = trim_whitespace(raw_section);
            if section.is_empty() {
                continue;
            }
            if !body_has_section(body, &section) {
                entries.push(Self::new(
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
                entries.push(Self::new(
                    "missing_field",
                    field_name.clone(),
                    format!("Missing required field: {field_name}:"),
                ));
                continue;
            }

            let Ok(regex) = regex::Regex::new(&field_regex) else {
                entries.push(Self::new(
                    "invalid_contract",
                    field_name.clone(),
                    format!("Invalid regex for field {field_name}: {field_regex}"),
                ));
                continue;
            };

            if !regex.is_match(&field_value) {
                entries.push(Self::new(
                    "invalid_field",
                    field_name.clone(),
                    format!("Invalid {field_name}: '{field_value}' (expected: {field_help})"),
                ));
            }
        }

        entries
    }
}
