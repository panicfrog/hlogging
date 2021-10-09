use hinterface::LoggingLevel as Level;
use hinterface::{LogHandler, Logger, LoggingLevel, Metadata};
use once_cell::sync::OnceCell;
use std::sync::Arc;

static LOGGER: OnceCell<Arc<Logger>> = OnceCell::new();

pub fn configure(
    label: String,
    level: LoggingLevel,
    logger_handler: Arc<dyn LogHandler + Send + Sync>,
) {
    let logger = Logger::new(level, label.as_str(), logger_handler);
    LOGGER.set(Arc::new(logger)).unwrap_or_else(|_| {
        dbg!("configure logger error");
    });
}

fn get_logger() -> &'static Arc<Logger> {
    LOGGER.get().expect("need configure")
}

pub fn debug(metadata: Metadata, message: String, source: Option<String>) {
    let logger = get_logger();
    let source = if let Some(s) = source {
        s
    } else {
       "".to_string()
    };
    logger.clone().log(Level::Debug, metadata, source, message);
}
pub fn info(metadata: Metadata, message: String, source: Option<String>) {
    let logger = get_logger();
    let source = if let Some(s) = source {
        s
    } else {
       "" .to_string()
    };
    logger.clone().log(Level::Info, metadata, source, message);
}
pub fn notice(metadata: Metadata, message: String, source: Option<String>) {
    let logger = get_logger();
    let source = if let Some(s) = source {
        s
    } else {
        "" .to_string()
    };
    logger.log(Level::Notice, metadata, source, message);
}
pub fn warring(metadata: Metadata, message: String, source: Option<String>) {
    let logger = get_logger();
    let source = if let Some(s) = source {
        s
    } else {
        "" .to_string()
    };
    logger
        .clone()
        .log(Level::Warning, metadata, source, message);
}
pub fn error(metadata: Metadata, message: String, source: Option<String>) {
    let logger = get_logger();
    let source = if let Some(s) = source {
        s
    } else {
        "" .to_string()
    };
    logger.clone().log(Level::Error, metadata, source, message);
}
pub fn critical(metadata: Metadata, message: String, source: Option<String>) {
    let logger = get_logger();
    let source = if let Some(s) = source {
        s
    } else {
        "" .to_string()
    };
    logger
        .clone()
        .log(Level::Critical, metadata, source, message);
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
