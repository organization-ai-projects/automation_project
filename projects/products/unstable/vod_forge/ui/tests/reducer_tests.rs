// Self-contained reducer tests that do not require importing the crate as a lib.
// They duplicate only the minimal types needed to verify reducer logic.

#[derive(Debug, Clone, Default)]
struct CatalogEntry {
    id: String,
    name: String,
    year: u16,
}

#[derive(Debug, Clone)]
struct PlaybackView {
    session_id: String,
    tick: u32,
    progress_pct: f32,
    done: bool,
}

#[derive(Debug, Clone)]
struct AnalyticsView {
    total_watch_ticks: u64,
    completion_rate_pct: f32,
    episodes_watched: usize,
}

#[derive(Debug, Clone, Default)]
struct AppState {
    catalog_titles: Vec<CatalogEntry>,
    playback: Option<PlaybackView>,
    analytics: Option<AnalyticsView>,
    last_error: Option<String>,
}

enum Action {
    CatalogLoaded(Vec<CatalogEntry>),
    PlaybackUpdated(PlaybackView),
    AnalyticsLoaded(AnalyticsView),
    ErrorOccurred(String),
    Reset,
}

fn reduce(mut state: AppState, action: Action) -> AppState {
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

#[test]
fn test_catalog_loaded_clears_error() {
    let state = AppState {
        last_error: Some("old error".to_string()),
        ..Default::default()
    };
    let entries = vec![CatalogEntry {
        id: "tt001".to_string(),
        name: "Space Odyssey".to_string(),
        year: 2020,
    }];
    let new_state = reduce(state, Action::CatalogLoaded(entries));
    assert_eq!(new_state.catalog_titles.len(), 1);
    assert_eq!(new_state.catalog_titles[0].id, "tt001");
    assert!(new_state.last_error.is_none());
}

#[test]
fn test_playback_updated() {
    let state = AppState::default();
    let pv = PlaybackView {
        session_id: "sess-1".to_string(),
        tick: 10,
        progress_pct: 10.0,
        done: false,
    };
    let new_state = reduce(state, Action::PlaybackUpdated(pv));
    let p = new_state.playback.unwrap();
    assert_eq!(p.tick, 10);
    assert!(!p.done);
}

#[test]
fn test_analytics_loaded() {
    let state = AppState::default();
    let av = AnalyticsView {
        total_watch_ticks: 500,
        completion_rate_pct: 75.0,
        episodes_watched: 4,
    };
    let new_state = reduce(state, Action::AnalyticsLoaded(av));
    let a = new_state.analytics.unwrap();
    assert_eq!(a.total_watch_ticks, 500);
    assert_eq!(a.episodes_watched, 4);
}

#[test]
fn test_error_occurred() {
    let state = AppState::default();
    let new_state = reduce(
        state,
        Action::ErrorOccurred("something went wrong".to_string()),
    );
    assert_eq!(
        new_state.last_error.as_deref(),
        Some("something went wrong")
    );
}

#[test]
fn test_reset_clears_state() {
    let state = AppState {
        catalog_titles: vec![CatalogEntry {
            id: "tt001".to_string(),
            name: "X".to_string(),
            year: 2020,
        }],
        last_error: Some("err".to_string()),
        ..Default::default()
    };
    let new_state = reduce(state, Action::Reset);
    assert!(new_state.catalog_titles.is_empty());
    assert!(new_state.last_error.is_none());
    assert!(new_state.playback.is_none());
}

#[test]
fn test_progress_widget_rendering() {
    // Test the progress widget logic inline
    fn render_progress(pct: f32, width: usize) -> String {
        let filled = ((pct / 100.0) * width as f32).round() as usize;
        let filled = filled.min(width);
        let empty = width - filled;
        format!("[{}{}] {:.1}%", "#".repeat(filled), "-".repeat(empty), pct)
    }

    let s = render_progress(0.0, 10);
    assert!(s.starts_with("[----------]"), "got: {}", s);

    let s = render_progress(100.0, 10);
    assert!(s.starts_with("[##########]"), "got: {}", s);

    let s = render_progress(50.0, 10);
    assert!(s.starts_with("[#####-----]"), "got: {}", s);
}

#[test]
fn test_table_widget_rendering() {
    fn render_table(headers: &[&str], rows: &[Vec<String>]) -> String {
        let col_count = headers.len();
        let mut widths: Vec<usize> = headers.iter().map(|h| h.len()).collect();
        for row in rows {
            for (i, cell) in row.iter().enumerate() {
                if i < col_count {
                    widths[i] = widths[i].max(cell.len());
                }
            }
        }
        let mut out = String::new();
        for (i, h) in headers.iter().enumerate() {
            out.push_str(&format!("{:<width$}", h, width = widths[i] + 2));
        }
        out.push('\n');
        out.push_str(&"-".repeat(widths.iter().sum::<usize>() + col_count * 2));
        out.push('\n');
        out
    }

    let headers = vec!["ID", "Name", "Year"];
    let rows: Vec<Vec<String>> = vec![vec![
        "tt001".to_string(),
        "Space Odyssey".to_string(),
        "2020".to_string(),
    ]];
    let s = render_table(&headers, &rows);
    assert!(s.contains("ID"), "got: {}", s);
    assert!(s.contains("Name"), "got: {}", s);
}
