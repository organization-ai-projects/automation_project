// projects/products/unstable/digital_pet/ui/src/screens/report_screen.rs
use crate::transport::ipc_client::RunReportDto;

pub struct ReportScreen {
    report: RunReportDto,
}

impl ReportScreen {
    pub fn new(report: RunReportDto) -> Self {
        Self { report }
    }

    pub fn render(&self) {
        println!("=== Run Report ===");
        println!("  Seed:           {}", self.report.seed);
        println!("  Final Species:  {}", self.report.final_species);
        println!("  Evolution Stage:{}", self.report.evolution_stage);
        println!("  Total Ticks:    {}", self.report.total_ticks);
        println!("  Care Mistakes:  {}", self.report.care_mistakes);
        println!("  Final Happiness:{}", self.report.final_happiness);
        println!("  Final HP:       {}", self.report.final_hp);
        println!("  Events:         {}", self.report.event_count);
        println!("  Run Hash:       {}", self.report.run_hash);
    }
}
