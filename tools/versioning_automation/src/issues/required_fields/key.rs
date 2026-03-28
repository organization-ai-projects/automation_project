//! tools/versioning_automation/src/issues/required_fields/key.rs
#[derive(Debug, Clone, Copy)]
pub(crate) enum Key {
    TitleRegex,
    RequiredSections,
    RequiredFields,
}

impl Key {
    fn base_name(self) -> &'static str {
        match self {
            Self::TitleRegex => "TITLE_REGEX",
            Self::RequiredSections => "REQUIRED_SECTIONS",
            Self::RequiredFields => "REQUIRED_FIELDS",
        }
    }

    pub(crate) fn contract_key_for_profile(profile: &str, base_key: Self) -> String {
        if profile == "review" {
            return format!("ISSUE_REVIEW_{}", base_key.base_name());
        }
        format!("ISSUE_{}", base_key.base_name())
    }
}
