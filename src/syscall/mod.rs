pub mod fs;

use self::fs::*;
use core::slice;
use core::str;
use super::task::CURRENT_PROCESS;

pub fn to_str<'a>(ptr: usize, size: usize) -> &'a str {
    unsafe {
        let slice = slice::from_raw_parts(ptr as *const u8, size);
        str::from_utf8(slice).unwrap()
    }
}

const SYS_FOPEN: usize = 0x1;
const SYS_PRINT: usize = 0x2;
const UNDEFINED_SYSCALL: usize = 0xff;

#[allow(unused_variables)]
pub unsafe fn syscall(a: usize, b: usize, c: usize, d: usize, e: usize, f: usize) -> usize {
    let current_process = CURRENT_PROCESS.as_ref().unwrap();
    match a {
        SYS_FOPEN => {         
            let ptr = current_process.translate_data_address(b as u32);
            open(to_str(ptr as usize, c)) 
        },
        SYS_PRINT => {
            let ptr = current_process.translate_data_address(b as u32);
            let string = to_str(ptr as usize, c);
            print!("{}", string);
            0
        }
        _ => UNDEFINED_SYSCALL
    }
}