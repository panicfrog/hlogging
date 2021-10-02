use std::ffi::CStr;
use std::fs::OpenOptions;
use std::io::Write;
use std::os::raw::{c_char, c_int};

#[no_mangle]
pub extern "C" fn write_log_file(file_path: *const c_char, message: *const c_char) -> c_int {
    let file_path = unsafe {
        assert!(!file_path.is_null());
        CStr::from_ptr(file_path)
    };
    let message = unsafe {
        assert!(!message.is_null());
        CStr::from_ptr(message)
    };

    let file_path = file_path.to_str().expect("path error");
    let message = message.to_str().expect("message error");

    let file = OpenOptions::new()
        .write(true)
        .create(true)
        .append(true)
        .open(file_path);
    match file {
        Ok(mut f) => match f.write(message.as_bytes()) {
            Ok(_) => 0,
            Err(_e) => {
                dbg!("file: {} error: {:?}", file_path,);
                2
            }
        },
        Err(_e) => {
            dbg!("file: {} error: {:?}", file_path,);
            1
        }
    }
}
