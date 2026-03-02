// projects/products/unstable/hospital_tycoon/ui/src/screens/report_screen.rs
use crate::transport::ipc_client::RunReportDto;

pub struct ReportScreen {
    report: RunReportDto,
}

impl ReportScreen {
    pub fn new(report: RunReportDto) -> Self {
        Self { report }
    }

    pub fn render(&self) {
        println!("=== Hospital Tycoon Run Report ===");
        println!("  Seed:             {}", self.report.seed);
        println!("  Scenario:         {}", self.report.scenario_name);
        println!("  Total Ticks:      {}", self.report.total_ticks);
        println!("  Patients Treated: {}", self.report.patients_treated);
        println!("  Patients Died:    {}", self.report.patients_died);
        println!("  Final Budget:     {}", self.report.final_budget);
        println!("  Final Reputation: {}", self.report.final_reputation);
        println!("  Event Count:      {}", self.report.event_count);
        println!("  Run Hash:         {}", self.report.run_hash);
    }

    pub fn summary_line(&self) -> String {
        format!(
            "seed={} scenario={} treated={} budget={} hash={}",
            self.report.seed,
            self.report.scenario_name,
            self.report.patients_treated,
            self.report.final_budget,
            self.report.run_hash
        )
    }
}
