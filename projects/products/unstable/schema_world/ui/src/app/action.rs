use common_json::Json;

#[derive(Debug, Clone)]
pub enum Action {
    SchemaLoaded(Json),
    RecordInserted,
    SnapshotReady(String),
    ReportReady(Json),
    Error(String),
}
