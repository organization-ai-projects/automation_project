pub struct LogWidget;

impl LogWidget {
    pub fn render(lines: &[String]) {
        for line in lines {
            println!("{line}");
        }
    }
}
