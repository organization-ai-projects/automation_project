pub struct StdoutWriter;

impl StdoutWriter {
    pub fn write_line(value: &str) {
        println!("{value}");
    }
}
