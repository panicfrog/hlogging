use std::collections::HashMap;
use std::fmt::{self, Display};

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

#[derive(PartialEq, Eq, PartialOrd)]
pub enum LoggingLevel {
    Debug = 1,
    Info,
    Notice,
    Warning,
    Error,
    Critical,
}

impl std::fmt::Display for LoggingLevel {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        match self {
            LoggingLevel::Debug => write!(fmt, "DEBUG"),
            LoggingLevel::Info => write!(fmt, "INFO"),
            LoggingLevel::Notice => write!(fmt, "NOTICE"),
            LoggingLevel::Warning => write!(fmt, "WARNING"),
            LoggingLevel::Error => write!(fmt, "ERROR"),
            LoggingLevel::Critical => write!(fmt, "CRITICAL"),
        }
    }
}

pub enum Metadata {
    String { value: String },
    Display { value: Box<dyn Display> },
    Array { value: Vec<Metadata> },
    Map { value: HashMap<String, Metadata> },
}

impl std::fmt::Display for Metadata {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Metadata::String { value } => write!(fmt, "{}", value),
            Metadata::Display { value } => write!(fmt, "{}", value),
            Metadata::Array { value } => {
                let s: String = value
                    .iter()
                    .map(|d| d.to_string())
                    .collect::<Vec<_>>()
                    .join(",");
                write!(fmt, "[{}]", s)
            }
            Metadata::Map { value } => {
                let v: String = value
                    .iter()
                    .map(|(k, v)| format!("{},{}", k, v))
                    .collect::<Vec<String>>()
                    .join(",");
                write!(fmt, "{{ {} }}", v)
            }
        }
    }
}

pub trait LogHandler {
    fn log(&self, level: &LoggingLevel, metadata: &Metadata, source: String, value: String);
}

#[allow(dead_code)]
pub struct Logger<T: LogHandler + Sized> {
    level: LoggingLevel,
    label: String,
    handler: T,
}

impl<T> Logger<T>
where
    T: LogHandler + Sized,
{
    pub fn new(level: LoggingLevel, label: &str, handler: T) -> Self {
        Logger {
            level,
            label: label.to_string(),
            handler,
        }
    }

    fn log(&self, level: LoggingLevel, metadata: &Metadata, source: String, content: String) {
        if self.level > level {
            return;
        }
        self.handler.log(&level, metadata, source, content);
    }

    pub fn debug(&self, metadata: &Metadata, source: String, message: String) {
        self.log(LoggingLevel::Debug, metadata, source, message);
    }
    pub fn info(&self, metadata: &Metadata, source: String, message: String) {
        self.log(LoggingLevel::Info, metadata, source, message);
    }
    pub fn notice(&self, metadata: &Metadata, source: String, message: String) {
        self.log(LoggingLevel::Notice, metadata, source, message);
    }
    pub fn warring(&self, metadata: &Metadata, source: String, message: String) {
        self.log(LoggingLevel::Warning, metadata, source, message);
    }
    pub fn error(&self, metadata: &Metadata, source: String, message: String) {
        self.log(LoggingLevel::Error, metadata, source, message);
    }
    pub fn critical(&self, metadata: &Metadata, source: String, message: String) {
        self.log(LoggingLevel::Critical, metadata, source, message);
    }
}
