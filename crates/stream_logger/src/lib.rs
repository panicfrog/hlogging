use chrono::Local;
use hinterface::{LogHandler, LoggingLevel, Metadata};
use std::io::{stdout, Write};

#[allow(dead_code)]
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
    fn log(&self, level: &LoggingLevel, metadata: &Metadata, source: String, value: String) {
        let time = Local::now().format("%Y-%m-%dT%H:%M:%S%z").to_string();
        let l = format!(
            "{} {} {}: {} [{}] {}",
            time, level, self.label, metadata, source, value
        );
        stdout().write(l.as_bytes()).expect("write stdout error");
    }
}

#[cfg(test)]
mod tests {
    use crate::StreamLogger;
    use hinterface::{Logger, LoggingLevel, Metadata};

    #[test]
    fn it_works() {
        let stream_logger = StreamLogger::new("test");
        let logger = Logger::new(
            LoggingLevel::Notice,
            stream_logger.get_label().as_str(),
            stream_logger,
        );
        logger.debug(
            &Metadata::String("".to_string()),
            "source".to_string(),
            "debug log".to_string(),
        );
        logger.info(
            &Metadata::String("".to_string()),
            "source".to_string(),
            "info log".to_string(),
        );
        logger.warring(
            &Metadata::String("".to_string()),
            "source".to_string(),
            "warring log".to_string(),
        );
        logger.notice(
            &Metadata::String("".to_string()),
            "source".to_string(),
            "notice log".to_string(),
        );
        logger.error(
            &Metadata::String("".to_string()),
            "source".to_string(),
            "error log".to_string(),
        );
        logger.critical(
            &Metadata::String("".to_string()),
            "source".to_string(),
            "critical log".to_string(),
        );
    }
}