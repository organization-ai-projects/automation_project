mod manager;

#[cfg(test)]
mod tests;

pub(crate) use manager::{
    append_issue_compliance_note, evaluate_issue_compliance, validate_required_issue_fields,
};
