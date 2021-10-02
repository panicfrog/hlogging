use hinterface::{LogHandler, LoggingLevel, Metadata};
use chrono::Local;
use std::io::{stdout, Write};

#[allow(dead_code)]
pub struct StreamLogger {
    label: String
}

impl LogHandler for StreamLogger {
   fn log(&self, level: &LoggingLevel, metadata: &Metadata, source: String, value: String) {
        let time = Local::now().format("%Y-%m-%dT%H:%M:%S%z").to_string();
        let l = format!("{} {} {}: {} [{}] {}", time, level, self.label, metadata, source, value);
        stdout().write(l.as_bytes()).expect("write stdout error");
    }
}
