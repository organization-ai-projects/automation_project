// projects/products/unstable/hospital_tycoon/ui/src/app/controller.rs
use crate::app::app_state::AppState;
use crate::diagnostics::error::AppError;
use crate::screens::report_screen::ReportScreen;
use crate::transport::ipc_client::IpcClient;

pub struct Controller;

impl Controller {
    pub fn new() -> Self {
        Self
    }

    pub fn init(
        &mut self,
        client: &mut IpcClient,
        state: &mut AppState,
        seed: u64,
        ticks: u64,
    ) -> Result<(), AppError> {
        client.new_run(seed, ticks)?;
        state.running = true;
        Ok(())
    }

    pub fn run_to_end(&mut self, client: &mut IpcClient, state: &mut AppState) -> Result<(), AppError> {
        client.run_to_end()?;
        state.current_tick = state.ticks;
        state.running = false;
        Ok(())
    }

    pub fn print_report(&mut self, client: &mut IpcClient) -> Result<(), AppError> {
        let report = client.get_report()?;
        let screen = ReportScreen::new(report);
        screen.render();
        Ok(())
    }
}
