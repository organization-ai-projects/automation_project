// projects/libraries/versioning/src/document_builder.rs

use crate::revision_log::{ModificationCategory, RevisionLog};
use std::fmt::Write;

/// Builds documentation for revision history
pub struct DocumentBuilder {
    format_style: OutputFormat,
}

#[derive(Debug, Clone, Copy)]
pub enum OutputFormat {
    Markdown,
    PlainText,
}

impl DocumentBuilder {
    pub fn with_format(format_style: OutputFormat) -> Self {
        Self { format_style }
    }

    /// Generate formatted document from revision log
    pub fn generate_document(&self, log: &RevisionLog) -> String {
        match self.format_style {
            OutputFormat::Markdown => self.build_markdown(log),
            OutputFormat::PlainText => self.build_plain_text(log),
        }
    }

    fn build_markdown(&self, log: &RevisionLog) -> String {
        let mut output = String::new();

        writeln!(
            &mut output,
            "# Revision History: {}",
            log.get_project_title()
        )
        .unwrap();
        writeln!(&mut output).unwrap();

        for entry in log.get_entries() {
            writeln!(&mut output, "## Release {}", entry.get_release()).unwrap();
            writeln!(
                &mut output,
                "**Date**: {}",
                entry.get_timestamp().format("%Y-%m-%d")
            )
            .unwrap();
            writeln!(&mut output).unwrap();

            if !entry.get_contributors().is_empty() {
                write!(&mut output, "**Contributors**: ").unwrap();
                for (idx, contributor) in entry.get_contributors().iter().enumerate() {
                    if idx > 0 {
                        write!(&mut output, ", ").unwrap();
                    }
                    write!(&mut output, "{}", contributor).unwrap();
                }
                writeln!(&mut output).unwrap();
                writeln!(&mut output).unwrap();
            }

            // Group modifications by category
            let categories = [
                ModificationCategory::BreakingModification,
                ModificationCategory::SecurityUpdate,
                ModificationCategory::NewCapability,
                ModificationCategory::Enhancement,
                ModificationCategory::CorrectionApplied,
                ModificationCategory::DeprecationNotice,
            ];

            for category in &categories {
                let mods: Vec<_> = entry
                    .get_modifications()
                    .iter()
                    .filter(|m| m.get_category() == category)
                    .collect();

                if !mods.is_empty() {
                    writeln!(&mut output, "### {}", category.label()).unwrap();
                    for modification in mods {
                        writeln!(&mut output, "- {}", modification.get_description()).unwrap();
                    }
                    writeln!(&mut output).unwrap();
                }
            }

            writeln!(&mut output, "---").unwrap();
            writeln!(&mut output).unwrap();
        }

        output
    }

    fn build_plain_text(&self, log: &RevisionLog) -> String {
        let mut output = String::new();

        writeln!(
            &mut output,
            "REVISION HISTORY: {}",
            log.get_project_title().to_uppercase()
        )
        .unwrap();
        writeln!(&mut output, "{}", "=".repeat(60)).unwrap();
        writeln!(&mut output).unwrap();

        for entry in log.get_entries() {
            writeln!(&mut output, "Release {}", entry.get_release()).unwrap();
            writeln!(
                &mut output,
                "Date: {}",
                entry.get_timestamp().format("%Y-%m-%d")
            )
            .unwrap();

            if !entry.get_contributors().is_empty() {
                write!(&mut output, "Contributors: ").unwrap();
                for (idx, contributor) in entry.get_contributors().iter().enumerate() {
                    if idx > 0 {
                        write!(&mut output, ", ").unwrap();
                    }
                    write!(&mut output, "{}", contributor).unwrap();
                }
                writeln!(&mut output).unwrap();
            }

            writeln!(&mut output).unwrap();

            for modification in entry.get_modifications() {
                writeln!(
                    &mut output,
                    "  [{}] {}",
                    modification.get_category().label(),
                    modification.get_description()
                )
                .unwrap();
            }

            writeln!(&mut output).unwrap();
            writeln!(&mut output, "{}", "-".repeat(60)).unwrap();
            writeln!(&mut output).unwrap();
        }

        output
    }
}
