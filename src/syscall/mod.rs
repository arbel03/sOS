mod fs;
mod task;

pub use self::fs::*;
pub use self::task::*;
use core::{ slice, str };
use alloc::Vec;

pub fn to_str<'a>(ptr: usize, size: usize) -> &'a str {
    unsafe {
        let slice = slice::from_raw_parts(ptr as *const u8, size);
        return str::from_utf8_unchecked(slice);
    }
}

pub unsafe fn terminated_string<'a>(start: *const u8) -> &'a str {
    use core::{ str, slice, ptr };

    let mut length: isize = 0;
    loop {
        let current = start.offset(length);
        if ptr::read(current) == 0u8 {
            break;
        }
        length += 1;
    }
    return str::from_utf8_unchecked(slice::from_raw_parts(start, length as usize));
}

unsafe fn read_args<'a>(args: &[*const u8]) -> Vec<&'a str> {
    let mut arguments: Vec<&str> = Vec::with_capacity(args.len());
    for ptr in args.iter().cloned() {
        arguments.push(terminated_string(ptr));
    }
    return arguments;
}

const FS_OPEN: usize = 0x01;
const FS_PRINT: usize = 0x02;
const FS_READ: usize = 0x03;
const FS_STAT: usize = 0x04;
const IO_GETC: usize = 0x05;
const IO_DELC: usize = 0x06;
const PROC_EXECV: usize = 0x07;
const FS_DIR_NAME: usize = 0x08;
const PROC_INFO: usize = 0x09;
const PROC_SIZE: usize = 0x10;
const UNDEFINED_SYSCALL: usize = 0xff;

#[allow(unused_variables)]
pub unsafe fn syscall(a: usize, b: usize, c: usize, d: usize, e: usize, f: usize) -> usize {
    let current_process = ::task::get_current_process();
    match a {
        FS_OPEN => {         
            let ptr = current_process.get_load_information().translate_virtual_to_physical_address(b as *const u8);
            open(to_str(ptr as usize, c)) 
        },
        FS_PRINT => {
            let ptr = current_process.get_load_information().translate_virtual_to_physical_address(b as *const u8);
            let string = to_str(ptr as usize, c);
            print!("{}", string);
            0
        },
        FS_READ => {
            let ptr = current_process.get_load_information().translate_virtual_to_physical_address(c as *const u8);
            let slice = slice::from_raw_parts_mut(ptr as *mut u8, d);
            read(b, slice)
        },
        IO_GETC => {
            ::drivers::keyboard::getc() as usize
        },
        IO_DELC => {
            ::vga_buffer::WRITER.delete_char();
            0
        },
        PROC_EXECV => {
            let name_ptr = current_process.get_load_information().translate_virtual_to_physical_address(b as *const u8);
            let args_ptr = current_process.get_load_information().translate_virtual_to_physical_address(d as *const u8) as *const *const u8;
            let args_slice = slice::from_raw_parts(args_ptr, e);
            let args: Vec<*const u8> = args_slice.iter().cloned().map(|addr| current_process.get_load_information().translate_virtual_to_physical_address(addr)).collect();

            execv(to_str(name_ptr as usize, c), &read_args(&args))
        },
        FS_STAT => {
            let name_ptr = current_process.get_load_information().translate_virtual_to_physical_address(b as *const u8);
            let stat_ptr = current_process.get_load_information().translate_virtual_to_physical_address(d as *const u8);
            let child_node_count = e;
            stat(to_str(name_ptr as usize, c), stat_ptr as *mut u8, e)
        },
        FS_DIR_NAME => {
            let parent_folder_ptr = current_process.get_load_information().translate_virtual_to_physical_address(b as *const u8);
            let read_buffer_ptr = current_process.get_load_information().translate_virtual_to_physical_address(d as *const u8);
            let read_buffer = slice::from_raw_parts_mut(read_buffer_ptr as *mut u8, e);
            let child_node_count = f;
            read_dir_name(to_str(parent_folder_ptr as usize, c), read_buffer, child_node_count)
        },
        PROC_INFO => {
            let proc_info_ptr = current_process.get_load_information().translate_virtual_to_physical_address(b as *const u8);
            proc_info(proc_info_ptr as *mut u8, c)
        },
        PROC_SIZE => {
            proc_memory_size()
        },
        _ => UNDEFINED_SYSCALL
    }
}