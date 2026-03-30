use std::collections::BTreeMap;

use crate::diagnostics::error::PatchsmithError;
use crate::dsl::dsl_op::DslOp;
use crate::plan::patch_plan::PatchPlan;

pub struct Applier;

/// The result of applying a plan. Contains modified file contents keyed by file path.
#[derive(Debug, Clone, PartialEq)]
pub struct ApplyResult {
    pub files: BTreeMap<String, String>,
}

impl Applier {
    /// Apply the plan to the given file contents.
    /// `file_contents` provides the initial content of each file referenced in the plan.
    /// Operations are applied sequentially (deterministic).
    pub fn apply(
        plan: &PatchPlan,
        file_contents: &BTreeMap<String, String>,
    ) -> Result<ApplyResult, PatchsmithError> {
        let mut files = file_contents.clone();
        for op in &plan.ops {
            Self::apply_op(op, &mut files)?;
        }
        Ok(ApplyResult { files })
    }

    fn apply_op(op: &DslOp, files: &mut BTreeMap<String, String>) -> Result<(), PatchsmithError> {
        match op {
            DslOp::ReplaceRange {
                file,
                start,
                end,
                text,
            } => {
                let start = *start as usize;
                let end = *end as usize;
                let content = files
                    .get(file)
                    .ok_or_else(|| PatchsmithError::Apply(format!("file not found: {file}")))?;
                if end > content.len() {
                    return Err(PatchsmithError::Apply(format!(
                        "ReplaceRange: end ({end}) exceeds file length ({})",
                        content.len()
                    )));
                }
                let mut result =
                    String::with_capacity(content.len() - (end - start) + text.len());
                result.push_str(&content[..start]);
                result.push_str(text);
                result.push_str(&content[end..]);
                files.insert(file.clone(), result);
            }
            DslOp::ReplaceFirst {
                file,
                pattern,
                text,
            } => {
                let content = files
                    .get(file)
                    .ok_or_else(|| PatchsmithError::Apply(format!("file not found: {file}")))?;
                if let Some(pos) = content.find(pattern.as_str()) {
                    let mut result =
                        String::with_capacity(content.len() - pattern.len() + text.len());
                    result.push_str(&content[..pos]);
                    result.push_str(text);
                    result.push_str(&content[pos + pattern.len()..]);
                    files.insert(file.clone(), result);
                } else {
                    return Err(PatchsmithError::Apply(format!(
                        "ReplaceFirst: pattern not found in {file}"
                    )));
                }
            }
            DslOp::InsertAfter {
                file,
                pattern,
                text,
            } => {
                let content = files
                    .get(file)
                    .ok_or_else(|| PatchsmithError::Apply(format!("file not found: {file}")))?;
                if let Some(pos) = content.find(pattern.as_str()) {
                    let insert_at = pos + pattern.len();
                    let mut result = String::with_capacity(content.len() + text.len());
                    result.push_str(&content[..insert_at]);
                    result.push_str(text);
                    result.push_str(&content[insert_at..]);
                    files.insert(file.clone(), result);
                } else {
                    return Err(PatchsmithError::Apply(format!(
                        "InsertAfter: pattern not found in {file}"
                    )));
                }
            }
            DslOp::DeleteRange { file, start, end } => {
                let start = *start as usize;
                let end = *end as usize;
                let content = files
                    .get(file)
                    .ok_or_else(|| PatchsmithError::Apply(format!("file not found: {file}")))?;
                if end > content.len() {
                    return Err(PatchsmithError::Apply(format!(
                        "DeleteRange: end ({end}) exceeds file length ({})",
                        content.len()
                    )));
                }
                let mut result = String::with_capacity(content.len() - (end - start));
                result.push_str(&content[..start]);
                result.push_str(&content[end..]);
                files.insert(file.clone(), result);
            }
        }
        Ok(())
    }
}
