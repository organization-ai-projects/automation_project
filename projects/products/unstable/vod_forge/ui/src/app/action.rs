use crate::app::app_state::{AnalyticsView, CatalogEntry, PlaybackView};

pub enum Action {
    CatalogLoaded(Vec<CatalogEntry>),
    PlaybackUpdated(PlaybackView),
    AnalyticsLoaded(AnalyticsView),
    ErrorOccurred(String),
    Reset,
}
