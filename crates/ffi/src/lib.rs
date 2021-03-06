use file_logger::FileLogger;
pub use hinterface::{FilterPlugin, HandlerPlugin, LoggingLevel, Metadata};
use logger_system;
// use std::collections::HashMap;
use mmap_logger::MmapLogger;
use std::fs::OpenOptions;
use std::io::Write;
use std::path::PathBuf;
use std::sync::Arc;
use stream_logger::StreamLogger;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum WriteFileError {
    #[error("file not exists")]
    FileError,
    #[error("wrong permissions or other write error")]
    WriteError,
}

// pub enum Metadata {
//     String { value: String },
//     Array { value: Vec<Metadata> },
//     Map { value: HashMap<String, Metadata> },
// }

// fn convert_metadata(metadata: Metadata) -> hinterface::Metadata {
//     match metadata {
//         Metadata::String { value } => hinterface::Metadata::String { value },
//         Metadata::Array { value } => {
//             let _value = value
//                 .into_iter()
//                 .map(|v| convert_metadata(v))
//                 .collect::<Vec<hinterface::Metadata>>();
//             hinterface::Metadata::Array { value: _value }
//         }
//         Metadata::Map { value } => {
//             let _value = value
//                 .into_iter()
//                 .map(|(k, v)| (k, convert_metadata(v)))
//                 .collect::<HashMap<String, hinterface::Metadata>>();
//             hinterface::Metadata::Map { value: _value }
//         }
//     }
// }

pub fn write_file(filename: String, message: String) -> Result<(), WriteFileError> {
    let file = OpenOptions::new()
        .write(true)
        .create(true)
        .append(true)
        .open(&filename);
    match file {
        Ok(mut f) => match f.write(message.as_bytes()) {
            Ok(_) => Ok(()),
            Err(_e) => {
                dbg!("file: {} error: {:?}", &filename,);
                Err(WriteFileError::WriteError)
            }
        },
        Err(_e) => {
            dbg!("file: {} error: {:?}", &filename,);
            Err(WriteFileError::FileError)
        }
    }
}

pub enum HLoggingType {
    StdStream,
    FileLogger { directory: String },
    MmapLogger { directory: String },
}

// features of stream logger
pub fn configure(label: String, level: LoggingLevel, logger_type: HLoggingType) {
    match logger_type {
        HLoggingType::StdStream => {
            let stream_logger_handler = StreamLogger::new(label.as_str());
            logger_system::configure(label, level, Arc::new(stream_logger_handler))
        }
        HLoggingType::FileLogger { directory } => {
            let file_logger_handler = FileLogger::new(label.as_str(), PathBuf::from(directory));
            file_logger_handler.run();
            logger_system::configure(label, level, Arc::new(file_logger_handler));
        }
        HLoggingType::MmapLogger { directory } => {
            let mmap_logger_handler = MmapLogger::new(label.as_str(), PathBuf::from(directory));
            mmap_logger_handler.run();
            logger_system::configure(label, level, Arc::new(mmap_logger_handler));
        }
    }
}

pub fn debug(metadata: Metadata, message: String, source: Option<String>) {
    logger_system::debug(metadata, &message, source);
}
pub fn info(metadata: Metadata, message: String, source: Option<String>) {
    logger_system::info(metadata, &message, source);
}
pub fn notice(metadata: Metadata, message: String, source: Option<String>) {
    logger_system::notice(metadata, &message, source);
}
pub fn warring(metadata: Metadata, message: String, source: Option<String>) {
    logger_system::warring(metadata, &message, source);
}
pub fn error(metadata: Metadata, message: String, source: Option<String>) {
    logger_system::error(metadata, &message, source);
}
pub fn critical(metadata: Metadata, message: String, source: Option<String>) {
    logger_system::critical(metadata, &message, source);
}

include!(concat!(env!("OUT_DIR"), "/hlogging.uniffi.rs"));
