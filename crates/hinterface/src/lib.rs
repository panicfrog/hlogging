use std::collections::HashMap;
use std::fmt::{self, Display};
use std::sync::Arc;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

#[derive(PartialEq, Eq, PartialOrd, Debug)]
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
                    .join(", ");
                write!(fmt, "[{}]", s)
            }
            Metadata::Map { value } => {
                let v: String = value
                    .iter()
                    .map(|(k, v)| format!("{} : {}", k, v))
                    .collect::<Vec<String>>()
                    .join(",");
                write!(fmt, "{{{}}}", v)
            }
        }
    }
}

pub trait LogHandler: Send + Sync {
    fn log(&self, level: &LoggingLevel, metadata: Metadata, source: String, value: String);
}

// pub trait SizedHandler: Sized + LogHandler {}

#[allow(dead_code)]
pub struct Logger {
    level: LoggingLevel,
    label: String,
    handler: Arc<dyn LogHandler>,
}

impl Logger {
    pub fn get_label(&self) -> String {
        self.label.clone()
    }
}

impl Logger {
    pub fn new(level: LoggingLevel, label: &str, handler: Arc<dyn LogHandler>) -> Self {
        let h = handler.clone();
        Logger {
            level,
            label: label.to_string(),
            handler: h,
        }
    }

    pub fn log(&self, level: LoggingLevel, metadata: Metadata, source: String, content: String) {
        if self.level > level {
            return;
        }
        self.handler.log(&level, metadata, source, content);
    }
}
