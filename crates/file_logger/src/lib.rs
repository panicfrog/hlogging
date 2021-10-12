use chrono::Local;
use hinterface::{LogHandler, LoggingLevel, Metadata};
use once_cell::sync::OnceCell;
use std::path::PathBuf;
use std::thread;
use tokio::fs;
use tokio::fs::OpenOptions;
use tokio::io::AsyncWriteExt;
use tokio::runtime::Builder;
use tokio::sync::mpsc;
use tokio::sync::mpsc::Sender;

static SENDER: OnceCell<Sender<String>> = OnceCell::new();

pub struct FileLogger {
    directory: PathBuf,
    label: String,
}

impl FileLogger {
    pub fn new(label: &str, directory: PathBuf) -> Self {
        // TODO: 记录directory
        FileLogger {
            label: label.to_string(),
            directory,
        }
    }
    pub fn get_directory(&self) -> PathBuf {
        self.directory.clone()
    }

    pub fn get_label(&self) -> String {
        self.label.clone()
    }
}

impl FileLogger {
    pub fn run(directory: PathBuf) {
        let date = Local::now().format("%Y%m%d").to_string();
        let directory = directory.clone();
        let file_path = directory.clone().join(date);
        let (tx, mut rx) = mpsc::channel(100);
        match SENDER.set(tx.clone()) {
            Ok(_) => (),
            Err(e) => {
                dbg!("{:?}", e);
            }
        };
        thread::spawn(move || {
            let rt = Builder::new_current_thread()
                .thread_name("hlogging_file_logger")
                .thread_stack_size(3 * 1024 * 1024)
                .build()
                .unwrap();
            rt.block_on(async {
                // TODO: 创建文件夹
                fs::create_dir_all(&directory)
                    .await
                    .expect("not creating directory");
                let mut file = OpenOptions::new()
                    .write(true)
                    .create(true)
                    .append(true)
                    .open(&file_path)
                    .await
                    .expect("can't create log file");
                while let Some(l) = rx.recv().await {
                    match file.write(l.as_bytes()).await {
                        Ok(_) => (),
                        Err(e) => {
                            dbg!("{:?}", e);
                        }
                    };
                }
            });
        });
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
        match SENDER.get().expect("").blocking_send(l) {
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
