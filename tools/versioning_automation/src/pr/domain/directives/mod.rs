//! tools/versioning_automation/src/pr/domain/directives/mod.rs
mod directive_record;
mod directive_record_type;

#[cfg(test)]
mod tests;

pub(crate) use directive_record::DirectiveRecord;
pub(crate) use directive_record_type::DirectiveRecordType;
