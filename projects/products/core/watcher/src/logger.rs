// projects/products/core/watcher/src/logger.rs
use log::LevelFilter;
use std::fs::OpenOptions;
use std::io::Write;
use std::sync::Mutex;

pub fn initialize_logger(log_file: &str, log_level: &str) {
    let level = match log_level {
        "debug" => LevelFilter::Debug,
        "info" => LevelFilter::Info,
        "warn" => LevelFilter::Warn,
        "error" => LevelFilter::Error,
        _ => LevelFilter::Info,
    };

    let file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(log_file)
        .unwrap();

    let _ = log::set_boxed_logger(Box::new(SimpleLogger {
        file: Mutex::new(file),
    }))
    .map(|()| log::set_max_level(level));
}

struct SimpleLogger {
    file: Mutex<std::fs::File>,
}

impl log::Log for SimpleLogger {
    fn enabled(&self, metadata: &log::Metadata) -> bool {
        metadata.level() <= log::max_level()
    }

    fn log(&self, record: &log::Record) {
        if self.enabled(record.metadata()) {
            let log_entry = format!("{} - {}\n", record.level(), record.args());
            if let Ok(mut file) = self.file.lock() {
                let _ = file.write_all(log_entry.as_bytes());
            }
        }
    }

    fn flush(&self) {}
}
