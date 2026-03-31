//! projects/products/unstable/market_tycoon/ui/src/components/sim_report_view.rs
pub(crate) struct SimReportView;

use common_json::JsonAccess;

impl SimReportView {
    pub(crate) fn display(report_json: &str) {
        match common_json::from_str::<common_json::Json>(report_json) {
            Ok(json) => {
                if let Ok(hash) = json.get_field("run_hash") {
                    println!("Simulation Report\n================\nRun Hash: {:?}", hash);
                }
                if let Ok(profit) = json.get_field("net_profit") {
                    println!("Net Profit: {:?}", profit);
                }
                if let Ok(events) = json.get_field("event_count") {
                    println!("Event Count: {:?}", events);
                }
                println!("================\nEnd of Report");
            }
            Err(e) => {
                eprintln!("Failed to parse report: {e}");
            }
        }
    }
}
