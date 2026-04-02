#[derive(Debug, Clone, Default)]
pub struct AppState {
    pub current_screen: Screen,
    pub scenario_loaded: bool,
    pub run_active: bool,
    pub encounter_json: Option<String>,
    pub battle_json: Option<String>,
    pub snapshot_hash: Option<String>,
    pub snapshot_json: Option<String>,
    pub run_hash: Option<String>,
    pub report_json: Option<String>,
    pub replay_data: Option<String>,
    pub last_error: Option<String>,
}

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub enum Screen {
    #[default]
    Overworld,
    Encounter,
    Battle,
    Party,
    Report,
}
