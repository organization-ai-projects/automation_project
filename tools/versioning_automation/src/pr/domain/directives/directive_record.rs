use serde::Serialize;

use crate::pr::domain::directives::directive_record_type::DirectiveRecordType;

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize)]
pub(crate) struct DirectiveRecord {
    pub(crate) record_type: DirectiveRecordType,
    pub(crate) first: String,
    pub(crate) second: String,
}
