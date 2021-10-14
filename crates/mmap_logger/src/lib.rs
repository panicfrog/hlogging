use bytes::BufMut;
use chrono::Local;
use crossbeam::channel::{bounded, select, Sender};
use hinterface::{LogHandler, LoggingLevel, Metadata};
use memmap2::MmapMut;
use once_cell::sync::OnceCell;
use std::path::PathBuf;
use std::thread;
use std::io::Write;

static MMAP_CROSSBEAM_SENDER: OnceCell<Sender<String>> = OnceCell::new();

pub struct MmapLogger {
    directory: PathBuf,
    label: String,
}

impl MmapLogger {
    pub fn new(label: &str, directory: PathBuf) -> Self {
        MmapLogger {
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

impl MmapLogger {
    pub fn run(&self) {
        let name = format!("{}.log", Local::now().format("%Y%m%d"));
        let directory = self.get_directory();
        let file_path = self.get_directory().join(name);
        let (s, r) = bounded(1000);
        match MMAP_CROSSBEAM_SENDER.set(s) {
            Ok(_) => (),
            Err(e) => {
                dbg!("{:?}", e);
            }
        }

        thread::spawn(move || {
            std::fs::create_dir_all(&directory).expect("not creating directory");
            let mut file = std::fs::OpenOptions::new()
                .read(true)
                .write(true)
                .create(true)
                .open(&file_path)
                .expect("can't create log file");
            let mut len = file.metadata().expect("get metadata error").len() as usize;
            file.set_len((len + 64*1024) as u64).unwrap();
            let mut mmap = unsafe { MmapMut::map_mut(&file).expect("mmap failed") };
            loop {
                select! {
                    recv(r) -> l => {
                       if let Ok(l) = l {
                       let l = l.as_bytes();
                       let start = len;
                       let end = start + l.len();
                       if end >= mmap.len() {
                          file.set_len((len + 64*1024) as u64).unwrap();
                          mmap = unsafe { MmapMut::map_mut(&file).expect("mmap failed") };
                       }
                        match (&mut mmap[start..end]).write(l) {
                            Ok(size) => len += size,
                            Err(e) => {
                                dbg!("{:?}", e);
                            }
                        };
                       }
                    },
                }
            }
        });
    }
}

impl LogHandler for MmapLogger {
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
        match MMAP_CROSSBEAM_SENDER
            .get()
            .expect("get sender error")
            .clone()
            .send(l)
        {
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
