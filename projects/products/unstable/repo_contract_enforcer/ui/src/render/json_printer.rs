pub struct JsonPrinter;

impl JsonPrinter {
    pub fn print_report(value: &serde_json::Value) -> anyhow::Result<()> {
        let txt = serde_json::to_string(value)?;
        println!("{txt}");
        Ok(())
    }
}
