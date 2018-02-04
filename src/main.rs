#[cfg(windows)] extern crate winapi;

use winapi::shared::windef::*;
use winapi::shared::minwindef::*;
use winapi::um::winuser::*;
use std::ffi::CString;
use std::ptr::null_mut;

#[derive(Debug)]
enum MemeError {
    NotFound(String),
}

fn find_process(window_name: &str) -> Result<u32, MemeError> {
    let hwnd = unsafe {
        let c_name = CString::new(window_name).unwrap();
        FindWindowA(null_mut(), c_name.as_ptr())
    };

    let hwnd_result = if hwnd == null_mut() {
        Err(MemeError::NotFound(format!("No window named {}", window_name)))
    } else {
        Ok(hwnd)
    }?;

    let proc_id = unsafe {
        let mut res = 0u32;
        let res_ptr = &mut res as *mut u32;
        GetWindowThreadProcessId(hwnd_result, res_ptr);
        res
    };

    if proc_id == 0 {
        Err(MemeError::NotFound("Cannot find process".to_string()))
    } else {
        Ok(proc_id)
    }
}

fn main() {
    println!("Proc ID is: {:?}", find_process("Untitled - Notepad"));
}
