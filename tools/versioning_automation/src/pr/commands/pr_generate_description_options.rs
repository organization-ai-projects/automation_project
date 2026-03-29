//! tools/versioning_automation/src/pr/commands/pr_generate_description_options.rs
use crate::{
    errors_code::E_USAGE,
    pr::{generate_options::GenerateOptions, render::print_usage},
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct PrGenerateDescriptionOptions {
    pub(crate) passthrough: Vec<String>,
}

impl PrGenerateDescriptionOptions {
    pub(crate) fn run_generate_description(self) -> i32 {
        let parsed = match GenerateOptions::parse_generate_options(&self.passthrough) {
            Ok(value) => value,
            Err(msg) => {
                eprintln!("{msg}");
                return E_USAGE;
            }
        };

        if parsed.help {
            print_usage();
            return 0;
        }

        GenerateOptions::run_generate_flow(parsed)
    }
}
