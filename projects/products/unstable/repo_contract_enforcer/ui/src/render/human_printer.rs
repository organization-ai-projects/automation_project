pub struct HumanPrinter;

impl HumanPrinter {
    pub fn print_report(value: &serde_json::Value) {
        println!("{}", Self::render_report(value));
    }

    pub fn render_report(value: &serde_json::Value) -> String {
        if let Some(summary) = value.get("summary") {
            format!(
                "repo_contract_enforcer summary:\nstable errors={}, stable warnings={}, unstable errors={}, unstable warnings={}",
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
            )
        } else {
            "repo_contract_enforcer: no summary in report".to_string()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::HumanPrinter;

    #[test]
    fn render_report_is_stable_for_fixed_summary() {
        let report = serde_json::json!({
            "summary": {
                "stable_error_count": 1,
                "stable_warning_count": 2,
                "unstable_error_count": 3,
                "unstable_warning_count": 4
            }
        });
        let rendered = HumanPrinter::render_report(&report);
        assert_eq!(
            rendered,
            "repo_contract_enforcer summary:\nstable errors=1, stable warnings=2, unstable errors=3, unstable warnings=4"
        );
    }
}
