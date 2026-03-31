pub struct CompanyReportView;

impl CompanyReportView {
    pub fn display(report_json: &str) {
        match common_json::from_str::<common_json::Json>(report_json) {
            Ok(json) => {
                if let Ok(companies) = json.get_field("companies") {
                    eprintln!("Companies: {companies}");
                }
            }
            Err(e) => {
                eprintln!("Failed to parse company report: {e}");
            }
        }
    }
}
