use crate::app::analytics_view::AnalyticsView;
use crate::app::catalog_entry::CatalogEntry;
use crate::app::playback_view::PlaybackView;

#[derive(Debug, Clone, Default)]
pub struct AppState {
    pub catalog_titles: Vec<CatalogEntry>,
    pub playback: Option<PlaybackView>,
    pub analytics: Option<AnalyticsView>,
    pub last_error: Option<String>,
}
