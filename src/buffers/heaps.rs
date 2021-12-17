#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case,
         non_upper_case_globals, unused_assignments, unused_mut)]
#![register_tool(c2rust)]
#![feature(register_tool)]
pub type u8_0 = libc::c_uchar;
// 0x38000 bytes
#[no_mangle]
pub static mut gAudioHeap: [u8_0; 229376] = [0; 229376];
#[no_mangle]
pub static mut gSystemHeap: [u8_0; 1] = [0; 1];
