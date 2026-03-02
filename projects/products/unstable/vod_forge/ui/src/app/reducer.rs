use crate::app::action::Action;
use crate::app::app_state::AppState;

pub fn reduce(mut state: AppState, action: Action) -> AppState {
    match action {
        Action::CatalogLoaded(titles) => {
            state.catalog_titles = titles;
            state.last_error = None;
        }
        Action::PlaybackUpdated(view) => {
            state.playback = Some(view);
            state.last_error = None;
        }
        Action::AnalyticsLoaded(view) => {
            state.analytics = Some(view);
            state.last_error = None;
        }
        Action::ErrorOccurred(msg) => {
            state.last_error = Some(msg);
        }
        Action::Reset => {
            state = AppState::default();
        }
    }
    state
}
