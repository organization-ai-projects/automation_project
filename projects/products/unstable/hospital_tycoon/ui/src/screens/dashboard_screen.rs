// projects/products/unstable/hospital_tycoon/ui/src/screens/dashboard_screen.rs
use crate::app::app_state::AppState;

pub struct DashboardScreen {
    state: AppState,
}

impl DashboardScreen {
    pub fn new(state: AppState) -> Self {
        Self { state }
    }

    pub fn render(&self) {
        println!(
            "=== Hospital Tycoon Dashboard (tick {}/{}) ===",
            self.state.current_tick, self.state.ticks
        );
        println!("  Seed:             {}", self.state.seed);
        println!("  Patients Treated: {}", self.state.patients_treated);
        println!("  Budget:           {}", self.state.final_budget);
        println!("  Reputation:       {}", self.state.reputation);
        if let Some(ref ev) = self.state.last_event {
            println!("  Last Event:       {}", ev);
        }
    }
}
