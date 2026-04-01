use crate::canonical_json::CanonicalJson;
use crate::diagnostics::Error;
use crate::policy_result::PolicyResult;
use crate::query_result::QueryResult;
use crate::snapshot::Snapshot;

pub struct ReportGenerator;

impl ReportGenerator {
    pub fn generate_snapshot_report(snapshot: &Snapshot) -> Result<String, Error> {
        CanonicalJson::to_string(snapshot)
    }

    pub fn generate_query_report(result: &QueryResult) -> Result<String, Error> {
        CanonicalJson::to_string(result)
    }

    pub fn generate_policy_report(result: &PolicyResult) -> Result<String, Error> {
        CanonicalJson::to_string(result)
    }
}
