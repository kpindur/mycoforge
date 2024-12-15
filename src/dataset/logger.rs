use log::{Log, LevelFilter, Metadata, Record};
use std::fs::OpenOptions;
use std::io::Write;
use std::time::{SystemTime, UNIX_EPOCH};

pub struct SimpleLogger {
    file_path: Option<String>,
    console_level: LevelFilter
}

impl SimpleLogger {
    pub fn new(file_path: Option<String>, console_level: LevelFilter) -> Self { return Self { file_path, console_level }; }
}

impl Log for SimpleLogger {
    fn enabled(&self, metadata: &Metadata) -> bool {
        return metadata.level() <= self.console_level;
    }

    fn log(&self, record: &Record) {
        if !self.enabled(record.metadata()) { return; }

        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("Failed to get current time!")
            .as_secs();

        let log_entry = format!(
            "[{}] {} - {}\n",
            timestamp, record.level(), record.args()
        );

        println!("{}", log_entry);

        if let Some(path) = &self.file_path {
            if let Ok(mut file) = OpenOptions::new().append(true).create(true).open(path) {
                let _ = file.write_all(log_entry.as_bytes());
            }
        }
    }

    fn flush(&self) {}
}
