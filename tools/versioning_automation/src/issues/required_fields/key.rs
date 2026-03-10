#[derive(Debug, Clone, Copy)]
pub(crate) enum ContractKey {
    TitleRegex,
    RequiredSections,
    RequiredFields,
}

impl ContractKey {
    fn base_name(self) -> &'static str {
        match self {
            Self::TitleRegex => "TITLE_REGEX",
            Self::RequiredSections => "REQUIRED_SECTIONS",
            Self::RequiredFields => "REQUIRED_FIELDS",
        }
    }
}

pub(crate) fn contract_key_for_profile(profile: &str, base_key: ContractKey) -> String {
    if profile == "review" {
        return format!("ISSUE_REVIEW_{}", base_key.base_name());
    }
    format!("ISSUE_{}", base_key.base_name())
}
