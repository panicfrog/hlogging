use chrono::Local;
use hinterface::{LogHandler, LoggingLevel, Metadata};

pub struct StreamLogger {
    label: String,
}

impl StreamLogger {
    pub fn new(label: &str) -> Self {
        StreamLogger {
            label: label.to_string(),
        }
    }

    pub fn get_label(&self) -> String {
        self.label.clone()
    }
}

impl LogHandler for StreamLogger {
    fn log(&self, level: &LoggingLevel, metadata: Metadata, source: String, value: String) {
        let time = Local::now().format("%Y-%m-%dT%H:%M:%S%z").to_string();
        if source.is_empty() {
            println!("{} {} {}: {} {}", time, level, self.label, metadata, value);
        } else {
            println!(
                "{} {} {}: {} [{}] {}",
                time, level, self.label, metadata, source, value
            );
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::StreamLogger;
    use hinterface::{Logger, LoggingLevel, Metadata};
    use std::collections::HashMap;
    use std::fs::metadata;

    #[test]
    fn it_works() {}
}
