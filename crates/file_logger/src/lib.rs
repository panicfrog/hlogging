use chrono::Local;
use hinterface::{LogHandler, LoggingLevel, Metadata};
use std::fs;
use std::fs::OpenOptions;
use std::io::Write;
use std::path::PathBuf;

pub struct FileLogger {
    file: PathBuf,
    label: String,
}

impl FileLogger {
    pub fn new(label: &str, directory: PathBuf) -> Self {
        let date = Local::now().format("%Y%m%d").to_string();
        fs::create_dir_all(&directory).expect("not creating directory");
        let file_path = directory.join(date);
        let _ = OpenOptions::new()
            .write(true)
            .create(true)
            .append(true)
            .open(&file_path)
            .expect("open file error");
        // TODO: hold file descriptor avoid open file every time
        FileLogger {
            label: label.to_string(),
            file: file_path,
        }
    }

    pub fn get_label(&self) -> String {
        self.label.clone()
    }
}

impl LogHandler for FileLogger {
    fn log(&self, level: &LoggingLevel, metadata: Metadata, source: String, value: String) {
        //TODO  检测文件是否超过限制
        let time = Local::now().format("%Y-%m-%dT%H:%M:%S%z").to_string();
        let l = if source.is_empty() {
            format!(
                "{} {} {}: {} {}\n",
                time, level, &self.label, metadata, value
            )
        } else {
            format!(
                "{} {} {}: {} [{}] {}\n",
                time, level, &self.label, metadata, source, value
            )
        };
        let mut file = OpenOptions::new()
            .write(true)
            .create(true)
            .append(true)
            .open(&self.file)
            .expect("open file error");
        match file.write(l.as_bytes()) {
            Ok(_) => (),
            Err(e) => {
                dbg!("{:?}", e);
            }
        };
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
