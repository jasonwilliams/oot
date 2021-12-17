#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case,
         non_upper_case_globals, unused_assignments, unused_mut)]
#![register_tool(c2rust)]
#![feature(register_tool)]
pub type u16_0 = libc::c_ushort;
// 0x25800 bytes
#[no_mangle]
pub static mut gZBuffer: [[u16_0; 320]; 240] = [[0; 320]; 240];
