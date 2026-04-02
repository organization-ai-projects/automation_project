//! projects/products/unstable/market_tycoon/ui/src/components/company_report_view.rs
pub(crate) struct CompanyReportView;

use common_json::JsonAccess;

impl CompanyReportView {
    pub(crate) fn display(report_json: &str) {
        match common_json::from_str::<common_json::Json>(report_json) {
            Ok(json) => {
                if let Ok(companies) = json.get_field("companies") {
                    eprintln!("Companies: {:?}", companies);
                }
                if let Some(object) = json.as_object() {
                    eprintln!("Additional Object Fields: {:?}", object);
                }
                if let Some(array) = json.as_array() {
                    eprintln!("Additional Array Fields: {:?}", array);
                }
            }
            Err(e) => {
                eprintln!("Failed to parse company report: {e}");
            }
        }
    }
}
