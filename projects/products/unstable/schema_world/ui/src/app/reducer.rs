use crate::app::action::Action;
use crate::app::app_state::AppState;

pub fn reduce(state: &mut AppState, action: Action) {
    match action {
        Action::SchemaLoaded(schema) => state.schema = Some(schema),
        Action::RecordInserted => {}
        Action::SnapshotReady(hash) => state.snapshot_hash = Some(hash),
        Action::ReportReady(report) => state.report_json = Some(report),
        Action::Error(err) => state.last_error = Some(err),
    }
}
