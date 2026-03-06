// projects/products/unstable/hospital_tycoon/ui/src/app/controller.rs
use crate::app::action::Action;
use crate::app::app_state::AppState;
use crate::app::reducer::Reducer;
use crate::diagnostics::app_error::AppError;
use crate::screens::dashboard_screen::DashboardScreen;
use crate::screens::hospital_screen::HospitalScreen;
use crate::screens::patient_screen::PatientScreen;
use crate::screens::report_screen::ReportScreen;
use crate::transport::ipc_client::IpcClient;
use crate::transport::run_report_dto::RunReportDto;
use crate::widgets::chart_widget::ChartWidget;
use crate::widgets::table_widget::TableWidget;

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
        Reducer::apply(state, &Action::Step(0));
        Ok(())
    }

    pub fn run_to_end(
        &mut self,
        client: &mut IpcClient,
        state: &mut AppState,
    ) -> Result<(), AppError> {
        Reducer::apply(state, &Action::RunToEnd);
        let snapshot = client.run_to_end()?;
        Reducer::apply(state, &Action::GetSnapshot);
        state.current_tick = snapshot.tick;
        state.final_budget = snapshot.budget_balance;
        state.reputation = snapshot.reputation_score;
        state.patients_treated = snapshot.patients_treated;
        state.run_hash = Some(snapshot.hash.clone());
        let hospital = HospitalScreen::new(0, 0, state.current_tick);
        hospital.render();
        let patients = PatientScreen::new(
            snapshot.patient_count,
            snapshot.patients_treated,
            state.current_tick,
        );
        patients.render();
        let dashboard = DashboardScreen::new(state.clone());
        dashboard.render();
        Ok(())
    }

    pub fn print_report(
        &mut self,
        client: &mut IpcClient,
        state: &mut AppState,
    ) -> Result<(), AppError> {
        Reducer::apply(state, &Action::GetReport);
        let report = client.get_report()?;
        state.final_budget = report.final_budget;
        state.reputation = report.final_reputation;
        state.patients_treated = report.patients_treated;
        state.run_hash = Some(report.run_hash.clone());
        let screen = ReportScreen::new(report);
        let summary = screen.summary_line();
        screen.render();
        println!("Summary: {}", summary);
        self.render_report_widgets(state);
        Ok(())
    }

    pub fn replay_to_end(
        &mut self,
        client: &mut IpcClient,
        state: &mut AppState,
    ) -> Result<RunReportDto, AppError> {
        Reducer::apply(state, &Action::ReplayToEnd);
        let report = client.replay_to_end()?;
        state.current_tick = report.total_ticks;
        state.final_budget = report.final_budget;
        state.reputation = report.final_reputation;
        state.patients_treated = report.patients_treated;
        state.run_hash = Some(report.run_hash.clone());
        Ok(report)
    }

    pub fn save_replay(&mut self, client: &mut IpcClient, path: &str) -> Result<(), AppError> {
        client.save_replay(path)?;
        Ok(())
    }

    fn render_report_widgets(&self, state: &AppState) {
        let mut table = TableWidget::new(vec![
            "Metric".to_string(),
            "Value".to_string(),
            "Tick".to_string(),
        ]);
        table.add_row(vec![
            "Patients Treated".to_string(),
            state.patients_treated.to_string(),
            state.current_tick.to_string(),
        ]);
        table.add_row(vec![
            "Budget".to_string(),
            state.final_budget.to_string(),
            state.current_tick.to_string(),
        ]);
        table.add_row(vec![
            "Reputation".to_string(),
            state.reputation.to_string(),
            state.current_tick.to_string(),
        ]);
        table.render();

        let reputation_chart = ChartWidget::new("Reputation", state.reputation as u64, 100);
        reputation_chart.render();
        let progress_chart = ChartWidget::new("Progress", state.current_tick, state.ticks.max(1));
        progress_chart.render();
    }
}
