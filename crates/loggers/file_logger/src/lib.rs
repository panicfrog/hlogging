use chrono::Local;
use std::time::{Duration};
use hinterface::{LogHandler, LoggingLevel, Metadata};
use once_cell::sync::OnceCell;
use std::path::PathBuf;
use std::thread;
use std::io::{BufWriter};
use chacha20poly1305::aead::{AeadInPlace, NewAead, Aead};

use crossbeam::channel::{bounded, select, Sender, tick};

use std::io::Write;
use chacha20poly1305::{Key, ChaCha20Poly1305, Nonce};

static CROSSBEAM_SENDER: OnceCell<Sender<String>> = OnceCell::new();

pub struct FileLogger {
    directory: PathBuf,
    label: String,
}

impl FileLogger {
    pub fn new(label: &str, directory: PathBuf) -> Self {
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
    pub fn run(&self) {
        let name = format!("{}.log", Local::now().format("%Y%m%d"));
        let directory = self.get_directory();
        let file_path = self.get_directory().join(name);
        let (s, r) = bounded(600);
        match CROSSBEAM_SENDER.set(s) {
            Ok(_) => (),
            Err(e) => {
                dbg!("{:?}", e);
            }
        }

        thread::spawn(move || {
            std::fs::create_dir_all(&directory).expect("not creating directory");
            let file = std::fs::OpenOptions::new()
                .write(true)
                .create(true)
                .append(true)
                .open(&file_path)
                .expect("can't create log file");
            let mut buf_writer = BufWriter::new(file);
            let ticker = tick(Duration::from_millis(300));
            loop {
                select! {
                    recv(r) -> l => {
                       if let Ok(l) = l {
                          match buf_writer.write(l.as_bytes()) {
                              Ok(_) => (),
                              Err(e) => {
                                  dbg!("write error  {:?}", e);
                              }
                          };
                       }
                    },
                    recv(ticker) -> _ => {
                        if buf_writer.buffer().len() > 0 {
                            match buf_writer.flush() {
                                Ok(_) => (),
                                Err(e) => {
                                    dbg!("write error  {:?}", e);
                                }
                            }
                        }
                    }
                }
            }
        });
    }
}

impl LogHandler for FileLogger {
    fn log(&self, level: &LoggingLevel, metadata: &Metadata, source: &str, value: &str) {
        //TODO  检测文件是否超过限制
        let time = Local::now().format("%Y-%m-%dT%H:%M:%S%z");
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
        match CROSSBEAM_SENDER.get().expect("get sender error").send(l) {
            Ok(_) => (),
            Err(e) => {
                dbg!("{:?}", e);
            }
        };
    }
}

fn crypto() {
    let key = Key::from_slice(b"an example very very secret key."); // 32 bytes
    let cipher = ChaCha20Poly1305::new(key);
    let nonce = Nonce::from_slice(b"unique nonce"); // 12-bytes; unique per message

    let ciphertext = cipher.encrypt(nonce, b"plaintext message".as_ref())
        .expect("encryption failure!");

}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
