#[cfg(windows)] extern crate winapi;

use winapi::shared::windef::*;
//use winapi::shared::ntdef::*;
use winapi::um::winnt::*;
use winapi::shared::minwindef::*;
use winapi::um::winuser::*;
use winapi::um::processthreadsapi::*;
use winapi::um::memoryapi::*;
use std::ffi::CString;
use std::ptr::null_mut;

#[derive(Debug)]
enum MemeError {
    NotFound(String),
}

fn find_process(window_name: &str) -> Result<HANDLE, MemeError> {
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
        unsafe {
            let handle = OpenProcess(PROCESS_ALL_ACCESS, FALSE, proc_id);
            Ok(handle)
        }
    }
}

fn read_meme(proc_id: HANDLE, addr: usize, bytes: usize) -> Vec<u8> {
    let mut buf: Vec<u8> = vec![0; bytes];
    unsafe {
        assert_ne!(0, ReadProcessMemory(proc_id, addr as *const _, buf.as_mut_ptr() as *mut _, bytes, 0 as *mut usize));
        //let res = Vec::from_raw_parts(buf.as_mut_ptr() as *mut u8, bytes, bytes);
        buf
    }
}

fn main() {
    let proc_id = find_process("Untitled - Notepad").unwrap();
    println!("Proc ID is: {:?}", proc_id);
    let read = read_meme(proc_id, 0x875BDF9F00, 50);
    println!("Read: {:?}", String::from_utf8(read));
}
