use crate::diagnostics::error::PatchsmithError;
use super::dsl_op::DslOp;

pub struct DslParser;

impl DslParser {
    pub fn parse(input: &str) -> Result<Vec<DslOp>, PatchsmithError> {
        let ops: Vec<DslOp> = common_json::from_json_str(input)
            .map_err(|e| PatchsmithError::Parse(format!("DSL parse error: {e}")))?;
        if ops.is_empty() {
            return Err(PatchsmithError::Parse("empty DSL: no operations".into()));
        }
        for op in &ops {
            Self::validate(op)?;
        }
        Ok(ops)
    }

    fn validate(op: &DslOp) -> Result<(), PatchsmithError> {
        match op {
            DslOp::ReplaceRange { file, start, end, .. } => {
                if file.is_empty() {
                    return Err(PatchsmithError::Parse(
                        "ReplaceRange: file must not be empty".into(),
                    ));
                }
                if start > end {
                    return Err(PatchsmithError::Parse(format!(
                        "ReplaceRange: start ({start}) > end ({end})"
                    )));
                }
            }
            DslOp::ReplaceFirst { file, pattern, .. } => {
                if file.is_empty() {
                    return Err(PatchsmithError::Parse(
                        "ReplaceFirst: file must not be empty".into(),
                    ));
                }
                if pattern.is_empty() {
                    return Err(PatchsmithError::Parse(
                        "ReplaceFirst: pattern must not be empty".into(),
                    ));
                }
            }
            DslOp::InsertAfter { file, pattern, .. } => {
                if file.is_empty() {
                    return Err(PatchsmithError::Parse(
                        "InsertAfter: file must not be empty".into(),
                    ));
                }
                if pattern.is_empty() {
                    return Err(PatchsmithError::Parse(
                        "InsertAfter: pattern must not be empty".into(),
                    ));
                }
            }
            DslOp::DeleteRange { file, start, end } => {
                if file.is_empty() {
                    return Err(PatchsmithError::Parse(
                        "DeleteRange: file must not be empty".into(),
                    ));
                }
                if start > end {
                    return Err(PatchsmithError::Parse(format!(
                        "DeleteRange: start ({start}) > end ({end})"
                    )));
                }
            }
        }
        Ok(())
    }
}
