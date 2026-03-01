pub struct HumanPrinter;

impl HumanPrinter {
    pub fn print_report(value: &serde_json::Value) {
        if let Some(summary) = value.get("summary") {
            println!("repo_contract_enforcer summary:");
            println!(
                "stable errors={}, stable warnings={}, unstable errors={}, unstable warnings={}",
                summary
                    .get("stable_error_count")
                    .and_then(|v| v.as_u64())
                    .unwrap_or(0),
                summary
                    .get("stable_warning_count")
                    .and_then(|v| v.as_u64())
                    .unwrap_or(0),
                summary
                    .get("unstable_error_count")
                    .and_then(|v| v.as_u64())
                    .unwrap_or(0),
                summary
                    .get("unstable_warning_count")
                    .and_then(|v| v.as_u64())
                    .unwrap_or(0)
            );
        } else {
            println!("repo_contract_enforcer: no summary in report");
        }
    }
}
