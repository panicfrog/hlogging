use std::ffi::CStr;
use std::os::raw::{c_int, c_char};
use std::fs::{OpenOptions};
use std::io::Write;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

#[no_mangle]
pub extern "C" fn write_log_file(path_str:*const c_char, message: *const c_char) -> c_int {
    let file_path = unsafe {
        assert!(!path_str.is_null());
        CStr::from_ptr(path_str)
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
        Err(e) => {
            dbg!("{:?}", e);
            1
        },
        Ok(mut f) => {
           match f.write(message.as_bytes()) {
               Err(e) => {
                   dbg!("{:?}", e);
                   2
               },
               Ok(_) => 0
           }
        }
    }
}