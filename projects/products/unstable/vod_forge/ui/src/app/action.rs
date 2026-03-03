use crate::app::analytics_view::AnalyticsView;
use crate::app::catalog_entry::CatalogEntry;
use crate::app::playback_view::PlaybackView;

pub enum Action {
    CatalogLoaded(Vec<CatalogEntry>),
    PlaybackUpdated(PlaybackView),
    AnalyticsLoaded(AnalyticsView),
    ErrorOccurred(String),
    Reset,
}
