use std::fs::OpenOptions;
use std::io::Write;
use thiserror::Error;
use std::fmt::Display;
use std::collections::HashMap;

#[derive(Error, Debug)]
pub enum WriteFileError {
    #[error("file not exists")]
    FileError,
    #[error("wrong permissions or other write error")]
    WriteError,
}

pub enum Metadata {
    String { value: String },
    Display { value: Box<dyn Display> },
    Array { value: Vec<Metadata> },
    Map { value: HashMap<String, Metadata> },
}

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

include!(concat!(env!("OUT_DIR"), "/hlogging.uniffi.rs"));
