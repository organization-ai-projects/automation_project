pub struct SimReportView;

impl SimReportView {
    pub fn display(report_json: &str) {
        match common_json::from_str::<common_json::Json>(report_json) {
            Ok(json) => {
                if let Ok(hash) = json.get_field("run_hash") {
                    eprintln!("Run Hash: {hash}");
                }
                if let Ok(profit) = json.get_field("net_profit") {
                    eprintln!("Net Profit: {profit}");
                }
                if let Ok(events) = json.get_field("event_count") {
                    eprintln!("Events: {events}");
                }
            }
            Err(e) => {
                eprintln!("Failed to parse report: {e}");
            }
        }
    }
}
