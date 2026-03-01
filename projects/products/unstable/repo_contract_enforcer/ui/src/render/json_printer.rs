pub struct JsonPrinter;

impl JsonPrinter {
    pub fn print_report(value: &serde_json::Value) -> anyhow::Result<()> {
        let txt = Self::render_report(value)?;
        println!("{txt}");
        Ok(())
    }

    pub fn render_report(value: &serde_json::Value) -> anyhow::Result<String> {
        Ok(serde_json::to_string(value)?)
    }
}

#[cfg(test)]
mod tests {
    use super::JsonPrinter;

    #[test]
    fn render_report_is_canonical_compact_json() {
        let report = serde_json::json!({
            "mode": "auto",
            "summary": {
                "stable_error_count": 0,
                "stable_warning_count": 1,
                "unstable_error_count": 0,
                "unstable_warning_count": 2
            }
        });
        let rendered = JsonPrinter::render_report(&report).expect("render report");
        assert_eq!(
            rendered,
            "{\"mode\":\"auto\",\"summary\":{\"stable_error_count\":0,\"stable_warning_count\":1,\"unstable_error_count\":0,\"unstable_warning_count\":2}}"
        );
    }
}
